//! 🖼️ Generic icon encoding resolver for board nodes (typst, emoji, raster, inline SVG).

use base64::Engine as _;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use typst::foundations::{Bytes, Datetime};
use typst::layout::{Abs, PagedDocument};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::Font;
use typst::utils::LazyHash;
use typst::Library;
use typst::LibraryExt;
use typst::World;

/// 🔍 Optional lookup for domain-themed SVG icons (e.g. puzzle metabolism table).
pub type ThemedSvgLookup = fn(&str) -> Option<&'static str>;

#[derive(Debug)]
pub enum BoardResolvedIcon {
    None,
    SvgThemed(String),
    SvgPlain(String),
    RasterRgba8 { rgba: Arc<[u8]>, w: u32, h: u32 },
}

struct RgbaImage {
    data: Arc<[u8]>,
    w: u32,
    h: u32,
}

fn decode_raster_icon_bytes(t: &str) -> Option<RgbaImage> {
    let s = t.trim().strip_prefix("image:").unwrap_or(t.trim()).trim();
    let rest = s
        .strip_prefix("data:image/png;base64,")
        .or_else(|| s.strip_prefix("data:image/jpeg;base64,"))
        .or_else(|| s.strip_prefix("data:image/jpg;base64,"))?;
    let raw = base64::engine::general_purpose::STANDARD.decode(rest.trim()).ok()?;
    let img = image::load_from_memory(&raw).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    if w == 0 || h == 0 {
        return None;
    }
    Some(RgbaImage {
        data: Arc::from(rgba.into_raw().into_boxed_slice()),
        w,
        h,
    })
}

fn typst_asset_font_list() -> Vec<Font> {
    let mut out = Vec::new();
    for bytes in typst_assets::fonts() {
        let blob = Bytes::new(bytes);
        let mut idx = 0u32;
        loop {
            if let Some(f) = Font::new(blob.clone(), idx) {
                out.push(f);
                idx = idx.saturating_add(1);
            } else {
                break;
            }
        }
    }
    out
}

fn typst_asset_font_list_plus_noto_color_emoji() -> Vec<Font> {
    let mut out = typst_asset_font_list();
    let emoji_blob = Bytes::new(crate::icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF);
    let mut idx = 0u32;
    loop {
        if let Some(f) = Font::new(emoji_blob.clone(), idx) {
            out.push(f);
            idx = idx.saturating_add(1);
        } else {
            break;
        }
    }
    out
}

fn board_typst_compile_markup_to_svg(markup: &str, fonts: &'static [Font], book: &'static LazyHash<typst::text::FontBook>) -> Option<String> {
    static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
    static MAIN: OnceLock<FileId> = OnceLock::new();
    let library = LIB.get_or_init(|| LazyHash::new(Library::default()));
    let main = *MAIN.get_or_init(|| FileId::new(None, VirtualPath::new("/board.typ")));
    let source = Source::new(main, markup.to_string());
    struct BoardTypstWorld<'a> {
        library: &'static LazyHash<Library>,
        book: &'static LazyHash<typst::text::FontBook>,
        main: FileId,
        source: Source,
        fonts: &'a [Font],
    }
    impl World for BoardTypstWorld<'_> {
        fn library(&self) -> &LazyHash<Library> {
            self.library
        }
        fn book(&self) -> &LazyHash<typst::text::FontBook> {
            self.book
        }
        fn main(&self) -> FileId {
            self.main
        }
        fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
            if id == self.main {
                Ok(self.source.clone())
            } else {
                Err(typst::diag::FileError::NotFound(PathBuf::from("board.typ")))
            }
        }
        fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
            Err(typst::diag::FileError::NotFound(PathBuf::from("board.bin")))
        }
        fn font(&self, index: usize) -> Option<Font> {
            self.fonts.get(index).cloned()
        }
        fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
            None
        }
    }
    let w = BoardTypstWorld { library, book, main, source, fonts };
    let warned = typst::compile::<PagedDocument>(&w);
    let doc = warned.output.ok()?;
    if doc.pages.is_empty() {
        return None;
    }
    Some(typst_svg::svg_merged(&doc, Abs::pt(3.0)))
}

static TYPST_ASSET_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
static TYPST_ASSET_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();
static TYPST_ICON_EMOJI_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
static TYPST_ICON_EMOJI_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();

pub fn board_typst_markup_to_svg(markup: &str) -> Option<String> {
    let fonts = TYPST_ASSET_FONTS.get_or_init(typst_asset_font_list);
    let book = TYPST_ASSET_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
    board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
}

fn board_typst_markup_to_svg_for_icon_emoji(markup: &str) -> Option<String> {
    let fonts = TYPST_ICON_EMOJI_FONTS.get_or_init(typst_asset_font_list_plus_noto_color_emoji);
    let book = TYPST_ICON_EMOJI_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
    board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
}

fn resolve_inline_svg(encoded: &str) -> Option<String> {
    let t = encoded.trim();
    if t.is_empty() {
        return None;
    }
    let lower = t.to_ascii_lowercase();
    if lower.starts_with("<?xml") || lower.contains("<svg") {
        return Some(t.to_string());
    }
    None
}

/// @emoji 🔍 Resolves an icon encoding to paintable content; `themed_lookup` marks SVG as themed when present.
pub fn board_resolve_icon_kind(encoded: &str, themed_lookup: ThemedSvgLookup) -> BoardResolvedIcon {
    let t = encoded.trim();
    if t.is_empty() {
        return BoardResolvedIcon::None;
    }
    if let Some(src) = t.strip_prefix("typst:") {
        let src = src.trim();
        if src.is_empty() {
            return BoardResolvedIcon::None;
        }
        let wrapped = format!("#set page(width: 96pt, height: 96pt, margin: 3pt, fill: none)\n{src}");
        return match board_typst_markup_to_svg(&wrapped) {
            Some(s) => BoardResolvedIcon::SvgPlain(s),
            None => BoardResolvedIcon::None,
        };
    }
    if let Some(em) = t.strip_prefix("emoji:") {
        let em = em.trim();
        if em.is_empty() {
            return BoardResolvedIcon::None;
        }
        let wrapped = format!(
            "#set page(width: 88pt, height: 88pt, margin: 2pt, fill: none)\n#set align(center + horizon)\n#set text(size: 44pt, font: \"Noto Color Emoji\")\n{em}"
        );
        return match board_typst_markup_to_svg_for_icon_emoji(&wrapped) {
            Some(s) => BoardResolvedIcon::SvgPlain(s),
            None => BoardResolvedIcon::None,
        };
    }
    if let Some(img) = decode_raster_icon_bytes(t) {
        return BoardResolvedIcon::RasterRgba8 { rgba: img.data, w: img.w, h: img.h };
    }
    if let Some(svg) = resolve_inline_svg(t) {
        if themed_lookup(t).is_some() {
            return BoardResolvedIcon::SvgThemed(svg);
        }
        return BoardResolvedIcon::SvgPlain(svg);
    }
    BoardResolvedIcon::None
}
