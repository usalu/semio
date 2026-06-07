//! 🖼️ Generic icon encoding resolver for board nodes (url, shortcode, typst, emoji, raster, inline SVG, catalog, text).

use base64::Engine as _;
use serde::{Deserialize, Serialize};
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

mod icon_shortcodes {
    include!(concat!(env!("OUT_DIR"), "/icon_shortcode_match.rs"));
}

/// 🔍 Optional lookup for domain-themed SVG icons (e.g. puzzle metabolism table).
pub type ThemedSvgLookup = fn(&str) -> Option<&'static str>;

// #region 🏷️IconUnion

/// @emoji 🖼 Canonical structured icon payload shared across canvases and UI chrome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Icon {
    Url { url: String },
    Shortcode { code: String },
    Data { data: String },
    Emoji { emoji: String },
    Typst { src: String },
    Text { text: String },
    Svg { svg: String },
    Catalog { key: String },
}

fn strip_legacy_image_data_prefix(raw: &str) -> &str {
    raw.trim()
        .strip_prefix("image:")
        .map(str::trim)
        .unwrap_or(raw.trim())
}

fn is_raster_data_url_payload(s: &str) -> bool {
    let u = strip_legacy_image_data_prefix(s).to_ascii_lowercase();
    u.starts_with("data:image/png;base64,")
        || u.starts_with("data:image/jpeg;base64,")
        || u.starts_with("data:image/jpg;base64,")
        || u.starts_with("data:image/webp;base64,")
        || u.starts_with("data:image/gif;base64,")
}

fn is_svg_data_url_payload(s: &str) -> bool {
    strip_legacy_image_data_prefix(s)
        .to_ascii_lowercase()
        .starts_with("data:image/svg+xml")
}

fn looks_like_shortcode_token(t: &str) -> bool {
    t.len() >= 3
        && t.starts_with(':')
        && t.ends_with(':')
        && t[1..t.len() - 1].chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '+' | '-'))
}

fn shortcode_inner(t: &str) -> Option<&str> {
    if !looks_like_shortcode_token(t) {
        return None;
    }
    Some(&t[1..t.len() - 1])
}

fn looks_like_ascii_catalogish_stem(s: &str) -> bool {
    let t = s.trim();
    if t.is_empty() || !t.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')) {
        return false;
    }
    matches!(t.chars().next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && (t.contains('.') || t.contains('_') || t.contains('-') || t.len() > 48)
}

fn is_extended_pictographic_char(c: char) -> bool {
    let cp = c as u32;
    matches!(cp, 0x1F1E6..=0x1F1FF | 0x1F300..=0x1FAFF | 0x2600..=0x27BF | 0x2300..=0x23FF)
        || matches!(c, '©' | '®' | '™' | '☺' | '☻' | '♥' | '♦' | '♣' | '♠' | '✓' | '✔' | '✕' | '✖' | '✗' | '✘')
        || c == '\u{FE0F}'
        || c == '\u{200D}'
}

fn looks_like_bare_emoji(s: &str) -> bool {
    let t = s.trim();
    !t.is_empty() && t.chars().any(is_extended_pictographic_char)
}

fn looks_like_bare_url(s: &str) -> bool {
    let lower = s.trim().to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
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

/// @emoji 🔤 Decodes a canonical icon string into a structured {@link Icon}.
pub fn decode_icon(encoded: &str) -> Option<Icon> {
    let t = encoded.trim();
    if t.is_empty() {
        return None;
    }
    if let Some(url) = t.strip_prefix("url:") {
        let url = url.trim();
        return (!url.is_empty()).then(|| Icon::Url { url: url.to_string() });
    }
    if looks_like_bare_url(t) {
        return Some(Icon::Url { url: t.to_string() });
    }
    if let Some(code) = shortcode_inner(t) {
        return Some(Icon::Shortcode { code: code.to_string() });
    }
    if let Some(src) = t.strip_prefix("typst:") {
        let src = src.trim();
        return (!src.is_empty()).then(|| Icon::Typst { src: src.to_string() });
    }
    if t.starts_with('$') {
        return Some(Icon::Typst { src: t.to_string() });
    }
    if let Some(em) = t.strip_prefix("emoji:") {
        let em = em.trim();
        return (!em.is_empty()).then(|| Icon::Emoji { emoji: em.to_string() });
    }
    if let Some(text) = t.strip_prefix("text:") {
        let text = text.trim();
        return (!text.is_empty()).then(|| Icon::Text { text: text.to_string() });
    }
    if is_raster_data_url_payload(t) || is_svg_data_url_payload(t) || t.to_ascii_lowercase().starts_with("data:") {
        return Some(Icon::Data { data: t.to_string() });
    }
    if let Some(svg) = resolve_inline_svg(t) {
        return Some(Icon::Svg { svg });
    }
    if looks_like_ascii_catalogish_stem(t) {
        return Some(Icon::Catalog { key: t.to_string() });
    }
    if looks_like_bare_emoji(t) {
        return Some(Icon::Emoji { emoji: t.to_string() });
    }
    if t.chars().count() <= 16 {
        return Some(Icon::Text { text: t.to_string() });
    }
    Some(Icon::Catalog { key: t.to_string() })
}

/// @emoji 🔤 Encodes a structured {@link Icon} into the canonical wire string.
pub fn encode_icon(icon: &Icon) -> String {
    match icon {
        Icon::Url { url } => {
            let u = url.trim();
            if u.to_ascii_lowercase().starts_with("http://") || u.to_ascii_lowercase().starts_with("https://") {
                format!("url:{u}")
            } else {
                format!("url:{u}")
            }
        }
        Icon::Shortcode { code } => format!(":{code}:"),
        Icon::Data { data } => data.trim().to_string(),
        Icon::Emoji { emoji } => format!("emoji:{}", emoji.trim()),
        Icon::Typst { src } => {
            let s = src.trim();
            if s.starts_with('$') {
                s.to_string()
            } else {
                format!("typst:{s}")
            }
        }
        Icon::Text { text } => format!("text:{}", text.trim()),
        Icon::Svg { svg } => svg.trim().to_string(),
        Icon::Catalog { key } => key.trim().to_string(),
    }
}

// #endregion 🏷️IconUnion

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

fn decode_data_url_svg(s: &str) -> Option<String> {
    let t = strip_legacy_image_data_prefix(s).trim();
    let lower = t.to_ascii_lowercase();
    if lower.starts_with("data:image/svg+xml;base64,") {
        let rest = t.split_once(',').map(|(_, b)| b.trim())?;
        let raw = base64::engine::general_purpose::STANDARD.decode(rest).ok()?;
        return String::from_utf8(raw).ok();
    }
    if lower.starts_with("data:image/svg+xml,") {
        let rest = t.split_once(',').map(|(_, b)| b.trim())?;
        return Some(percent_decode_utf8(rest));
    }
    None
}

fn percent_decode_utf8(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex_nibble(bytes[i + 1]), hex_nibble(bytes[i + 2])) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn decode_raster_icon_bytes(t: &str) -> Option<RgbaImage> {
    let s = strip_legacy_image_data_prefix(t).trim();
    let rest = s
        .strip_prefix("data:image/png;base64,")
        .or_else(|| s.strip_prefix("data:image/jpeg;base64,"))
        .or_else(|| s.strip_prefix("data:image/jpg;base64,"))
        .or_else(|| s.strip_prefix("data:image/webp;base64,"))
        .or_else(|| s.strip_prefix("data:image/gif;base64,"))?;
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

fn board_typst_markup_to_svg_for_icon_text(markup: &str) -> Option<String> {
    let fonts = TYPST_ASSET_FONTS.get_or_init(typst_asset_font_list);
    let book = TYPST_ASSET_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
    board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
}

fn resolve_typst_src(src: &str) -> BoardResolvedIcon {
    let src = src.trim();
    if src.is_empty() {
        return BoardResolvedIcon::None;
    }
    let wrapped = format!("#set page(width: 96pt, height: 96pt, margin: 3pt, fill: none)\n{src}");
    match board_typst_markup_to_svg(&wrapped) {
        Some(s) => BoardResolvedIcon::SvgPlain(s),
        None => BoardResolvedIcon::None,
    }
}

fn resolve_emoji_body(em: &str) -> BoardResolvedIcon {
    let em = em.trim();
    if em.is_empty() {
        return BoardResolvedIcon::None;
    }
    let wrapped = format!(
        "#set page(width: 88pt, height: 88pt, margin: 2pt, fill: none)\n#set align(center + horizon)\n#set text(size: 44pt, font: \"Noto Color Emoji\")\n{em}"
    );
    match board_typst_markup_to_svg_for_icon_emoji(&wrapped) {
        Some(s) => BoardResolvedIcon::SvgPlain(s),
        None => BoardResolvedIcon::None,
    }
}

fn resolve_text_body(text: &str) -> BoardResolvedIcon {
    let text = text.trim();
    if text.is_empty() {
        return BoardResolvedIcon::None;
    }
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
    let wrapped = format!(
        "#set page(width: 96pt, height: 96pt, margin: 3pt, fill: none)\n#set align(center + horizon)\n#set text(size: 28pt)\n\"{escaped}\""
    );
    match board_typst_markup_to_svg_for_icon_text(&wrapped) {
        Some(s) => BoardResolvedIcon::SvgPlain(s),
        None => BoardResolvedIcon::None,
    }
}

fn resolve_icon_data(data: &str) -> BoardResolvedIcon {
    if let Some(svg) = decode_data_url_svg(data) {
        return BoardResolvedIcon::SvgPlain(svg);
    }
    if let Some(img) = decode_raster_icon_bytes(data) {
        return BoardResolvedIcon::RasterRgba8 { rgba: img.data, w: img.w, h: img.h };
    }
    BoardResolvedIcon::None
}

fn resolve_structured_icon(icon: &Icon, themed_lookup: ThemedSvgLookup) -> BoardResolvedIcon {
    match icon {
        Icon::Url { .. } => BoardResolvedIcon::None,
        Icon::Shortcode { code } => icon_shortcodes::icon_shortcode_to_emoji(code)
            .map(|em| resolve_emoji_body(em))
            .unwrap_or(BoardResolvedIcon::None),
        Icon::Data { data } => resolve_icon_data(data),
        Icon::Emoji { emoji } => resolve_emoji_body(emoji),
        Icon::Typst { src } => resolve_typst_src(src),
        Icon::Text { text } => resolve_text_body(text),
        Icon::Svg { svg } => {
            if themed_lookup(svg).is_some() {
                BoardResolvedIcon::SvgThemed(svg.clone())
            } else {
                BoardResolvedIcon::SvgPlain(svg.clone())
            }
        }
        Icon::Catalog { key } => themed_lookup(key)
            .map(|svg| BoardResolvedIcon::SvgThemed(svg.to_string()))
            .unwrap_or(BoardResolvedIcon::None),
    }
}

/// @emoji 🔍 Resolves an icon encoding to paintable content; `themed_lookup` marks SVG as themed when present.
pub fn board_resolve_icon_kind(encoded: &str, themed_lookup: ThemedSvgLookup) -> BoardResolvedIcon {
    let Some(icon) = decode_icon(encoded) else {
        return BoardResolvedIcon::None;
    };
    resolve_structured_icon(&icon, themed_lookup)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(s: &str) {
        let icon = decode_icon(s).expect("decode");
        let encoded = encode_icon(&icon);
        let again = decode_icon(&encoded).expect("re-decode");
        assert_eq!(icon, again, "round-trip failed for {s} -> {encoded}");
    }

    #[test]
    fn icon_codec_round_trips_all_kinds() {
        round_trip("url:https://example.com/icon.png");
        round_trip(":smile:");
        round_trip("data:image/png;base64,iVBORw0KGgo=");
        round_trip("emoji:☺");
        round_trip("typst:$x^2$");
        round_trip("text:Hi");
        round_trip(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#);
        round_trip("capsule_J");
    }

    #[test]
    fn icon_codec_decodes_catalog_stem() {
        assert!(matches!(decode_icon("capsule_J"), Some(Icon::Catalog { .. })));
    }

    #[test]
    fn icon_codec_resolves_shortcode_to_svg() {
        let r = board_resolve_icon_kind(":smile:", |_| None);
        match r {
            BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_resolves_text_to_svg() {
        let r = board_resolve_icon_kind("text:Hi", |_| None);
        match r {
            BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_url_returns_none_for_sync_resolver() {
        assert!(matches!(board_resolve_icon_kind("url:https://example.com/x.png", |_| None), BoardResolvedIcon::None));
    }
}
