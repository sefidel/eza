use ansi_term::Style;

use crate::fs::File;
use crate::info::filetype::FileExtensions;
use lazy_static::lazy_static;
use std::collections::HashMap;


pub trait FileIcon {
    fn icon_file(&self, file: &File<'_>) -> Option<char>;
}


#[derive(Copy, Clone)]
pub enum Icons {
    Audio,
    Image,
    Video,
    Compressed,
}

impl Icons {
    pub fn value(self) -> char {
        match self {
            Self::Audio  => '\u{f001}', // 
            Self::Image  => '\u{f1c5}', // 
            Self::Video  => '\u{f03d}', // 
            Self::Compressed => '\u{f410}', // 
        }
    }
}


/// Converts the style used to paint a file name into the style that should be
/// used to paint an icon.
///
/// - The background colour should be preferred to the foreground colour, as
///   if one is set, it’s the more “obvious” colour choice.
/// - If neither is set, just use the default style.
/// - Attributes such as bold or underline should not be used to paint the
///   icon, as they can make it look weird.
pub fn iconify_style(style: Style) -> Style {
    style.background.or(style.foreground)
         .map(Style::from)
         .unwrap_or_default()
}



lazy_static! {
    static ref MAP_BY_NAME: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert(".Trash", '\u{f1f8}'); // 
        m.insert(".atom", '\u{e764}'); // 
        m.insert(".bashprofile", '\u{e615}'); // 
        m.insert(".bashrc", '\u{f489}'); // 
        m.insert(".git", '\u{f1d3}'); // 
        m.insert(".gitattributes", '\u{f1d3}'); // 
        m.insert(".gitconfig", '\u{f1d3}'); // 
        m.insert(".github", '\u{f408}'); // 
        m.insert(".gitignore", '\u{f1d3}'); // 
        m.insert(".gitignore_global", '\u{f1d3}'); // 
        m.insert(".gitmodules", '\u{f1d3}'); // 
        m.insert(".rvm", '\u{e21e}'); // 
        m.insert(".vimrc", '\u{e7c5}'); // 
        m.insert(".vscode", '\u{e70c}'); // 
        m.insert(".zshrc", '\u{f489}'); // 
        m.insert(".emacs", '\u{e632}'); // 
        m.insert("LICENSE", '\u{f02d}'); // 
        m.insert("Cargo.lock", '\u{e7a8}'); // 
        m.insert("bin", '\u{e5fc}'); // 
        m.insert("config", '\u{e5fc}'); // 
        m.insert("docker-compose.yml", '\u{f308}'); // 
        m.insert("Dockerfile", '\u{f308}'); // 
        m.insert("Earthfile", '\u{f0ac}'); // 
        m.insert("ds_store", '\u{f179}'); // 
        m.insert("gitignore_global", '\u{f1d3}'); // 
        m.insert("gitlab-ci.yml", '\u{f296}'); // 
        m.insert("go.mod", '\u{e626}'); // 
        m.insert("go.sum", '\u{e626}'); // 
        m.insert("gradle", '\u{e256}'); // 
        m.insert("gruntfile.coffee", '\u{e611}'); // 
        m.insert("gruntfile.js", '\u{e611}'); // 
        m.insert("gruntfile.ls", '\u{e611}'); // 
        m.insert("gulpfile.coffee", '\u{e610}'); // 
        m.insert("gulpfile.js", '\u{e610}'); // 
        m.insert("gulpfile.ls", '\u{e610}'); // 
        m.insert("hidden", '\u{f023}'); // 
        m.insert("include", '\u{e5fc}'); // 
        m.insert("lib", '\u{f121}'); // 
        m.insert("localized", '\u{f179}'); // 
        m.insert("Makefile", '\u{f489}'); // 
        m.insert("node_modules", '\u{e718}'); // 
        m.insert("npmignore", '\u{e71e}'); // 
        m.insert("PKGBUILD", '\u{f303}'); // 
        m.insert("rubydoc", '\u{e73b}'); // 
        m.insert("yarn.lock", '\u{e718}'); // 
        m.insert("Vagrantfile", '\u{2371}'); //⍱

        m
    };
}

pub fn icon_for_file(file: &File<'_>) -> char {
    let extensions = Box::new(FileExtensions);

    if let Some(icon) = MAP_BY_NAME.get(file.name.as_str()) { *icon }
    else if file.points_to_directory() {
        match file.name.as_str() {
            "bin"           => '\u{e5fc}', // 
            ".git"          => '\u{f1d3}', // 
            ".idea"         => '\u{e7b5}', // 
            _               => '\u{f413}'  // 
        }
    }
    else if let Some(icon) = extensions.icon_file(file) { icon }
    else if let Some(ext) = file.ext.as_ref() {
        match ext.as_str() {
            "7z"            => '\u{f410}', // 
            "a"             => '\u{f17c}', // 
            "acf"           => '\u{f1b6}', // 
            "ai"            => '\u{e7b4}', // 
            "android"       => '\u{e70e}', // 
            "apk"           => '\u{e70e}', // 
            "apple"         => '\u{f179}', // 
            "asm"           => '\u{e637}', // 
            "avi"           => '\u{f03d}', // 
            "avif"          => '\u{f1c5}', // 
            "avro"          => '\u{e60b}', // 
            "awk"           => '\u{f489}', // 
            "bash"          => '\u{f489}', // 
            "bash_history"  => '\u{f489}', // 
            "bash_profile"  => '\u{f489}', // 
            "bashrc"        => '\u{f489}', // 
            "bat"           => '\u{ebc4}', //  
            "bats"          => '\u{f489}', // 
            "bib"           => '\u{e69b}', // 
            "bin"           => '\u{eae8}', // 
            "bmp"           => '\u{f1c5}', // 
            "bst"           => '\u{e69b}', // 
            "bz"            => '\u{f410}', // 
            "bz2"           => '\u{f410}', // 
            "c"             => '\u{e61e}', // 
            "c++"           => '\u{e61d}', // 
            "cab"           => '\u{e70f}', // 
            "cc"            => '\u{e61d}', // 
            "cert"          => '\u{eafa}', // 
            "cfg"           => '\u{e615}', // 
            "cjs"           => '\u{e74e}', // 
            "class"         => '\u{e256}', // 
            "clj"           => '\u{e768}', // 
            "cljs"          => '\u{e76a}', // 
            "cls"           => '\u{e69b}', // 
            "cmd"           => '\u{e70f}', // 
            "coffee"        => '\u{f0f4}', // 
            "conf"          => '\u{e615}', // 
            "config"        => '\u{e615}', // 
            "cp"            => '\u{e61d}', // 
            "cpio"          => '\u{f410}', // 
            "cpp"           => '\u{e61d}', // 
            "crt"           => '\u{eafa}', // 
            "cs"            => '\u{f031b}', // 󰌛
            "csh"           => '\u{f489}', // 
            "cshtml"        => '\u{f1fa}', // 
            "csproj"        => '\u{f031b}', // 󰌛
            "css"           => '\u{e749}', // 
            "csv"           => '\u{f1c3}', // 
            "csx"           => '\u{f031b}', // 󰌛
            "cts"           => '\u{e628}', // 
            "cu"            => '\u{e64b}', // 
            "cxx"           => '\u{e61d}', // 
            "d"             => '\u{e7af}', // 
            "dart"          => '\u{e798}', // 
            "db"            => '\u{f1c0}', // 
            "deb"           => '\u{e77d}', // 
            "desktop"       => '\u{ebd1}', // 
            "diff"          => '\u{f440}', // 
            "djvu"          => '\u{f02d}', // 
            "dll"           => '\u{e70f}', // 
            "doc"           => '\u{f1c2}', // 
            "docx"          => '\u{f1c2}', // 
            "drawio"        => '\u{ebba}', // 
            "ds_store"      => '\u{f179}', // 
            "DS_store"      => '\u{f179}', // 
            "dump"          => '\u{f1c0}', // 
            "ebook"         => '\u{e28b}', // 
            "ebuild"        => '\u{f30d}', // 
            "editorconfig"  => '\u{e615}', // 
            "ejs"           => '\u{e618}', // 
            "el"            => '\u{e632}', // 
            "elm"           => '\u{e62c}', // 
            "eml"           => '\u{f003}', //  
            "env"           => '\u{f462}', // 
            "eot"           => '\u{f031}', // 
            "epub"          => '\u{e28a}', // 
            "erb"           => '\u{e73b}', // 
            "erl"           => '\u{e7b1}', // 
            "ex"            => '\u{e62d}', // 
            "exe"           => '\u{f17a}', // 
            "exs"           => '\u{e62d}', // 
            "fish"          => '\u{f489}', // 
            "flac"          => '\u{f001}', // 
            "flv"           => '\u{f03d}', // 
            "font"          => '\u{f031}', // 
            "fs"            => '\u{e7a7}', // 
            "fsi"           => '\u{e7a7}', // 
            "fsx"           => '\u{e7a7}', // 
            "gdoc"          => '\u{f1c2}', // 
            "gem"           => '\u{e21e}', // 
            "gemfile"       => '\u{e21e}', // 
            "gemspec"       => '\u{e21e}', // 
            "gform"         => '\u{f298}', // 
            "gif"           => '\u{f1c5}', // 
            "git"           => '\u{f1d3}', // 
            "gitattributes" => '\u{f1d3}', // 
            "gitignore"     => '\u{f1d3}', // 
            "gitmodules"    => '\u{f1d3}', // 
            "go"            => '\u{e626}', // 
            "gpg"           => '\u{e60a}', // 
            "gradle"        => '\u{e256}', // 
            "groovy"        => '\u{e775}', // 
            "gsheet"        => '\u{f1c3}', // 
            "gslides"       => '\u{f1c4}', // 
            "guardfile"     => '\u{e21e}', // 
            "gz"            => '\u{f410}', // 
            "h"             => '\u{f0fd}', // 
            "hbs"           => '\u{e60f}', // 
            "hpp"           => '\u{f0fd}', // 
            "hs"            => '\u{e777}', // 
            "htm"           => '\u{f13b}', // 
            "html"          => '\u{f13b}', // 
            "hxx"           => '\u{f0fd}', // 
            "ical"          => '\u{eab0}', // 
            "icalendar"     => '\u{eab0}', // 
            "ico"           => '\u{f1c5}', // 
            "ics"           => '\u{eab0}', // 
            "ifb"           => '\u{eab0}', // 
            "image"         => '\u{f1c5}', // 
            "img"           => '\u{e271}', // 
            "iml"           => '\u{e7b5}', // 
            "ini"           => '\u{f17a}', // 
            "ipynb"         => '\u{e678}', // 
            "iso"           => '\u{e271}', // 
            "j2c"           => '\u{f1c5}', // 
            "j2k"           => '\u{f1c5}', // 
            "jad"           => '\u{e256}', // 
            "jar"           => '\u{e256}', // 
            "java"          => '\u{e256}', // 
            "jfi"           => '\u{f1c5}', // 
            "jfif"          => '\u{f1c5}', // 
            "jif"           => '\u{f1c5}', // 
            "jl"            => '\u{e624}', // 
            "jmd"           => '\u{f48a}', // 
            "jp2"           => '\u{f1c5}', // 
            "jpe"           => '\u{f1c5}', // 
            "jpeg"          => '\u{f1c5}', // 
            "jpg"           => '\u{f1c5}', // 
            "jpx"           => '\u{f1c5}', // 
            "js"            => '\u{e74e}', // 
            "json"          => '\u{e60b}', // 
            "jsx"           => '\u{e7ba}', // 
            "jxl"           => '\u{f1c5}', // 
            "kdb"           => '\u{f23e}', // 
            "kdbx"          => '\u{f23e}', // 
            "key"           => '\u{eb11}', // 
            "ko"            => '\u{f17c}', // 
            "ksh"           => '\u{f489}', // 
            "latex"         => '\u{e69b}', // 
            "less"          => '\u{e758}', // 
            "lhs"           => '\u{e777}', // 
            "license"       => '\u{f02d}', // 
            "localized"     => '\u{f179}', // 
            "lock"          => '\u{f023}', // 
            "log"           => '\u{f18d}', // 
            "lua"           => '\u{e620}', // 
            "lz"            => '\u{f410}', // 
            "lz4"           => '\u{f410}', // 
            "lzh"           => '\u{f410}', // 
            "lzma"          => '\u{f410}', // 
            "lzo"           => '\u{f410}', // 
            "m"             => '\u{e61e}', // 
            "ml"            => '\u{e67a}', // 
            "mli"           => '\u{e67a}', // 
            "mll"           => '\u{e67a}', // 
            "mly"           => '\u{e67a}', // 
            "mm"            => '\u{e61d}', // 
            "m4a"           => '\u{f001}', // 
            "magnet"        => '\u{f076}', // 
            "markdown"      => '\u{f48a}', // 
            "md"            => '\u{f48a}', // 
            "mjs"           => '\u{e74e}', // 
            "mk"            => '\u{f489}', // 
            "mkd"           => '\u{f48a}', // 
            "mkv"           => '\u{f03d}', // 
            "mobi"          => '\u{e28b}', // 
            "mov"           => '\u{f03d}', // 
            "mp2"           => '\u{f001}', // 
            "mp3"           => '\u{f001}', // 
            "mp4"           => '\u{f03d}', // 
            "msi"           => '\u{e70f}', // 
            "mts"           => '\u{e628}', // 
            "mustache"      => '\u{e60f}', // 
            "nix"           => '\u{f313}', // 
            "node"          => '\u{f0399}', // 󰎙
            "npmignore"     => '\u{e71e}', // 
            "o"             => '\u{eae8}', // 
            "odp"           => '\u{f1c4}', // 
            "ods"           => '\u{f1c3}', // 
            "odt"           => '\u{f1c2}', // 
            "ogg"           => '\u{f001}', // 
            "ogv"           => '\u{f03d}', // 
            "org"           => '\u{e633}', // 
            "otf"           => '\u{f031}', // 
            "out"           => '\u{eb2c}', // 
            "part"          => '\u{f43a}', // 
            "patch"         => '\u{f440}', // 
            "pdf"           => '\u{f1c1}', // 
            "pem"           => '\u{eb11}', // 
            "php"           => '\u{e73d}', // 
            "pl"            => '\u{e769}', // 
            "plx"           => '\u{e769}', // 
            "pm"            => '\u{e769}', // 
            "png"           => '\u{f1c5}', // 
            "pod"           => '\u{e769}', // 
            "ppt"           => '\u{f1c4}', // 
            "pptx"          => '\u{f1c4}', // 
            "procfile"      => '\u{e21e}', // 
            "properties"    => '\u{e60b}', // 
            "ps1"           => '\u{ebc7}', //  
            "psd"           => '\u{e7b8}', // 
            "psd1"          => '\u{ebc7}', //  
            "psm1"          => '\u{ebc7}', //  
            "pxm"           => '\u{f1c5}', // 
            "py"            => '\u{e606}', // 
            "pyc"           => '\u{e606}', // 
            "qcow2"         => '\u{e271}', // 
            "r"             => '\u{f25d}', // 
            "rakefile"      => '\u{e21e}', // 
            "rar"           => '\u{f410}', // 
            "razor"         => '\u{f1fa}', // 
            "rb"            => '\u{e21e}', // 
            "rdata"         => '\u{f25d}', // 
            "rdb"           => '\u{e76d}', // 
            "rdoc"          => '\u{f48a}', // 
            "rds"           => '\u{f25d}', // 
            "readme"        => '\u{f48a}', // 
            "rlib"          => '\u{e7a8}', // 
            "rmd"           => '\u{f48a}', // 
            "rmeta"         => '\u{e7a8}', // 
            "rpm"           => '\u{e7bb}', // 
            "rs"            => '\u{e7a8}', // 
            "rspec"         => '\u{e21e}', // 
            "rspec_parallel"=> '\u{e21e}', // 
            "rspec_status"  => '\u{e21e}', // 
            "rss"           => '\u{f09e}', // 
            "rst"           => '\u{f15c}', // 
            "rtf"           => '\u{f0219}', // 󰈙
            "ru"            => '\u{e21e}', // 
            "rubydoc"       => '\u{e73b}', // 
            "s"             => '\u{e637}', // 
            "sass"          => '\u{e603}', // 
            "scala"         => '\u{e737}', // 
            "scss"          => '\u{e749}', // 
            "service"       => '\u{eba2}', // 
            "sh"            => '\u{f489}', // 
            "shell"         => '\u{f489}', // 
            "slim"          => '\u{e73b}', // 
            "sln"           => '\u{e70c}', // 
            "so"            => '\u{f17c}', // 
            "sql"           => '\u{f1c0}', // 
            "sqlite3"       => '\u{e7c4}', // 
            "sty"           => '\u{e69b}', // 
            "styl"          => '\u{e600}', // 
            "stylus"        => '\u{e600}', // 
            "svg"           => '\u{f1c5}', // 
            "swift"         => '\u{e755}', // 
            "t"             => '\u{e769}', // 
            "tar"           => '\u{f410}', // 
            "taz"           => '\u{f410}', // 
            "tbz"           => '\u{f410}', // 
            "tbz2"          => '\u{f410}', // 
            "tex"           => '\u{e69b}', // 
            "tgz"           => '\u{f410}', // 
            "tiff"          => '\u{f1c5}', // 
            "tlz"           => '\u{f410}', // 
            "toml"          => '\u{e615}', // 
            "torrent"       => '\u{e275}', // 
            "ts"            => '\u{e628}', // 
            "tsv"           => '\u{f1c3}', // 
            "tsx"           => '\u{e7ba}', // 
            "ttf"           => '\u{f031}', // 
            "twig"          => '\u{e61c}', // 
            "txt"           => '\u{f15c}', // 
            "txz"           => '\u{f410}', // 
            "tz"            => '\u{f410}', // 
            "tzo"           => '\u{f410}', // 
            "unity"         => '\u{e721}', // 
            "unity3d"       => '\u{e721}', // 
            "vdi"           => '\u{e271}', // 
            "vhd"           => '\u{e271}', // 
            "video"         => '\u{f03d}', // 
            "vim"           => '\u{e7c5}', // 
            "vmdk"          => '\u{e271}', // 
            "vue"           => '\u{f0844}', // 󰡄
            "war"           => '\u{e256}', // 
            "wav"           => '\u{f001}', // 
            "webm"          => '\u{f03d}', // 
            "webp"          => '\u{f1c5}', // 
            "windows"       => '\u{f17a}', // 
            "woff"          => '\u{f031}', // 
            "woff2"         => '\u{f031}', // 
            "xhtml"         => '\u{f13b}', // 
            "xls"           => '\u{f1c3}', // 
            "xlsm"          => '\u{f1c3}', // 
            "xlsx"          => '\u{f1c3}', // 
            "xml"           => '\u{f05c0}', // 󰗀
            "xul"           => '\u{f05c0}', // 󰗀
            "xz"            => '\u{f410}', // 
            "yaml"          => '\u{f481}', // 
            "yml"           => '\u{f481}', // 
            "zig"           => '\u{21af}', // ↯
            "zip"           => '\u{f410}', // 
            "zsh"           => '\u{f489}', // 
            "zsh-theme"     => '\u{f489}', // 
            "zshrc"         => '\u{f489}', // 
            "zst"           => '\u{f410}', // 
            "svelte"        => '\u{e697}', // 
            _               => '\u{f15b}'  // 
        }
    }
    else {
        '\u{f016}'
    }
}
