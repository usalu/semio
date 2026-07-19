//! 🔤 Text and math labels via Typst-to-SVG compilation.

use crate::color::Color;
use crate::sobject::{Sobject, VSobject};
use ecow::EcoString;
use mathematical_geometry::{append_shape_to_path, BezPath, PathEl, Point, Rect};
use std::path::PathBuf;
use std::sync::OnceLock;
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::layout::Abs;
use typst::utils::LazyHash;
use typst::LibraryExt;
use typst::{Library, World};

const TEXT_PAGE_PT: f64 = 400.0;
const TEXT_MARGIN_PT: f64 = 8.0;
const TEXT_SIZE_PT: f64 = 36.0;

/// 📝 Plain text Sobject rendered through Typst.
#[derive(Clone)]
pub struct Text {
    pub inner: VSobject,
    pub content: EcoString,
    pub font_size: f64,
}

impl Text {
    pub fn new(content: impl Into<EcoString>, color: Color) -> Self {
        let content = content.into();
        let svg = typst_markup_to_svg(&wrap_text(&content, TEXT_SIZE_PT)).unwrap_or_default();
        let mut inner = svg_to_vobject(&svg, color);
        inner.set_name(content.to_string());
        Self {
            inner,
            content,
            font_size: TEXT_SIZE_PT,
        }
    }

    pub fn as_sobject(&self) -> &VSobject {
        &self.inner
    }

    pub fn as_sobject_mut(&mut self) -> &mut VSobject {
        &mut self.inner
    }
}

fn format_decimal(value: f64, decimals: u32) -> String {
    format!("{value:.prec$}", prec = decimals as usize)
}

/// 🔢 Decimal number label with interpolatable value.
#[derive(Clone)]
pub struct DecimalNumber {
    pub value: f64,
    pub inner: Text,
    pub decimals: u32,
}

impl DecimalNumber {
    pub fn new(value: f64, decimals: u32, color: Color) -> Self {
        let inner = Text::new(format_decimal(value, decimals), color);
        Self {
            value,
            inner,
            decimals,
        }
    }

    pub fn lerp_value(&mut self, target: f64, t: f64, color: Color) {
        let t = t.clamp(0.0, 1.0);
        self.value = self.value + (target - self.value) * t;
        self.inner = Text::new(format_decimal(self.value, self.decimals), color);
    }

    pub fn as_sobject(&self) -> &VSobject {
        &self.inner.inner
    }
}

/// 🔢 Integer label wrapper.
#[derive(Clone)]
pub struct Integer {
    pub value: i64,
    pub inner: Text,
}

impl Integer {
    pub fn new(value: i64, color: Color) -> Self {
        Self {
            value,
            inner: Text::new(value.to_string(), color),
        }
    }

    pub fn as_sobject(&self) -> &VSobject {
        &self.inner.inner
    }
}

/// 📄 Multi-line paragraph wrapper.
#[derive(Clone)]
pub struct Paragraph {
    pub lines: Vec<EcoString>,
    pub inner: Text,
}

impl Paragraph {
    pub fn new(lines: Vec<impl Into<EcoString>>, color: Color) -> Self {
        let lines: Vec<EcoString> = lines.into_iter().map(Into::into).collect();
        let body = lines.iter().map(|l| l.as_str()).collect::<Vec<_>>().join("\n");
        Self {
            lines,
            inner: Text::new(body, color),
        }
    }

    pub fn as_sobject(&self) -> &VSobject {
        &self.inner.inner
    }
}

/// 💻 Monospace code block wrapper.
#[derive(Clone)]
pub struct Code {
    pub source: EcoString,
    pub inner: Text,
}

impl Code {
    pub fn new(source: impl Into<EcoString>, color: Color) -> Self {
        let source = source.into();
        let wrapped = format!(
            "#set page(width: {TEXT_PAGE_PT}pt, height: {TEXT_PAGE_PT}pt, margin: {TEXT_MARGIN_PT}pt, fill: none)\n#set text(size: {TEXT_SIZE_PT}pt, font: \"Courier New\")\n`{source}`"
        );
        let svg = typst_markup_to_svg(&wrapped).unwrap_or_default();
        let mut inner_v = svg_to_vobject(&svg, color);
        inner_v.set_name(source.to_string());
        Self {
            source: source.clone(),
            inner: Text {
                inner: inner_v,
                content: source,
                font_size: TEXT_SIZE_PT,
            },
        }
    }

    pub fn as_sobject(&self) -> &VSobject {
        &self.inner.inner
    }
}

/// ∑ Math-mode label rendered through Typst.
#[derive(Clone)]
pub struct MathText {
    pub inner: VSobject,
    pub latex: EcoString,
}

impl MathText {
    pub fn new(expr: impl Into<EcoString>, color: Color) -> Self {
        let latex = expr.into();
        let wrapped = format!(
            "#set page(width: {}pt, height: {}pt, margin: {}pt, fill: none)\n#set text(size: {}pt)\n$ {latex} $",
            TEXT_PAGE_PT, TEXT_PAGE_PT, TEXT_MARGIN_PT, TEXT_SIZE_PT
        );
        let svg = typst_markup_to_svg(&wrapped).unwrap_or_default();
        let mut inner = svg_to_vobject(&svg, color);
        inner.set_name(latex.to_string());
        Self { inner, latex }
    }

    pub fn as_sobject(&self) -> &VSobject {
        &self.inner
    }
}

fn wrap_text(text: &str, size: f64) -> String {
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
    format!(
        "#set page(width: {TEXT_PAGE_PT}pt, height: {TEXT_PAGE_PT}pt, margin: {TEXT_MARGIN_PT}pt, fill: none)\n#set align(center + horizon)\n#set text(size: {size}pt)\n\"{escaped}\""
    )
}

fn svg_to_vobject(svg: &str, color: Color) -> VSobject {
    let mut v = VSobject::new();
    if svg.is_empty() {
        v.set_paths(vec![BezPath::new()]);
        v.set_fill(color);
        v.style.stroke = None;
        return v;
    }
    let options = usvg::Options::default();
    if let Ok(tree) = usvg::Tree::from_str(svg, &options) {
        let height = tree.size().height() as f64;
        let scale = if height > 1e-9 { 2.0 / height } else { 0.01 };
        let offset_y = height * scale;
        let mut paths = Vec::new();
        for child in tree.root().children() {
            collect_svg_paths(child, scale, offset_y, &mut paths);
        }
        if paths.is_empty() {
            paths.push(fallback_text_rect());
        }
        v.set_paths(paths);
    } else {
        v.set_paths(vec![fallback_text_rect()]);
    }
    v.set_fill(color);
    v.style.stroke = None;
    v
}

fn fallback_text_rect() -> BezPath {
    let rect = Rect::new(-1.0, -0.5, 1.0, 0.5);
    let mut p = BezPath::new();
    append_shape_to_path(&mut p, &rect, 0.01);
    p
}

fn map_svg_point(x: f32, y: f32, scale: f64, offset_y: f64) -> Point {
    Point::new(x as f64 * scale, offset_y - y as f64 * scale)
}

fn collect_svg_paths(node: &usvg::Node, scale: f64, offset_y: f64, out: &mut Vec<BezPath>) {
    match node {
        usvg::Node::Group(group) => {
            for child in group.children() {
                collect_svg_paths(child, scale, offset_y, out);
            }
        }
        usvg::Node::Path(path) => {
            let mut p = BezPath::new();
            for segment in path.data().segments() {
                match segment {
                    usvg::tiny_skia_path::PathSegment::MoveTo(pt) => {
                        p.move_to(map_svg_point(pt.x, pt.y, scale, offset_y));
                    }
                    usvg::tiny_skia_path::PathSegment::LineTo(pt) => {
                        p.line_to(map_svg_point(pt.x, pt.y, scale, offset_y));
                    }
                    usvg::tiny_skia_path::PathSegment::QuadTo(c, pt) => {
                        p.quad_to(
                            map_svg_point(c.x, c.y, scale, offset_y),
                            map_svg_point(pt.x, pt.y, scale, offset_y),
                        );
                    }
                    usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, pt) => {
                        p.curve_to(
                            map_svg_point(c1.x, c1.y, scale, offset_y),
                            map_svg_point(c2.x, c2.y, scale, offset_y),
                            map_svg_point(pt.x, pt.y, scale, offset_y),
                        );
                    }
                    usvg::tiny_skia_path::PathSegment::Close => p.close_path(),
                }
            }
            if !p.elements().is_empty() {
                out.push(p);
            }
        }
        _ => {}
    }
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

fn typst_compile_markup_to_svg(markup: &str, fonts: &'static [Font], book: &'static LazyHash<FontBook>) -> Option<String> {
    static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
    static MAIN: OnceLock<FileId> = OnceLock::new();
    let library = LIB.get_or_init(|| LazyHash::new(Library::default()));
    let main = *MAIN.get_or_init(|| FileId::new(None, VirtualPath::new("/animate.typ")));
    let source = Source::new(main, markup.to_string());
    struct AnimateTypstWorld<'a> {
        library: &'static LazyHash<Library>,
        book: &'static LazyHash<FontBook>,
        main: FileId,
        source: Source,
        fonts: &'a [Font],
    }
    impl World for AnimateTypstWorld<'_> {
        fn library(&self) -> &LazyHash<Library> {
            self.library
        }
        fn book(&self) -> &LazyHash<FontBook> {
            self.book
        }
        fn main(&self) -> FileId {
            self.main
        }
        fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
            if id == self.main {
                Ok(self.source.clone())
            } else {
                Err(typst::diag::FileError::NotFound(PathBuf::from("animate.typ")))
            }
        }
        fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
            Err(typst::diag::FileError::NotFound(PathBuf::from("animate.bin")))
        }
        fn font(&self, index: usize) -> Option<Font> {
            self.fonts.get(index).cloned()
        }
        fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
            None
        }
    }
    let w = AnimateTypstWorld {
        library,
        book,
        main,
        source,
        fonts,
    };
    let warned = typst::compile::<PagedDocument>(&w);
    let doc = warned.output.ok()?;
    if doc.pages.is_empty() {
        return None;
    }
    Some(typst_svg::svg_merged(&doc, Abs::pt(4.0)))
}

static TYPST_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
static TYPST_BOOK: OnceLock<LazyHash<FontBook>> = OnceLock::new();

/// 🖨️ Compile Typst markup to merged SVG.
pub fn typst_markup_to_svg(markup: &str) -> Option<String> {
    let fonts = TYPST_FONTS.get_or_init(typst_asset_font_list);
    let book = TYPST_BOOK.get_or_init(|| LazyHash::new(FontBook::from_fonts(fonts.iter())));
    typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typst_plain_text_compiles() {
        let svg = typst_markup_to_svg(&wrap_text("hello", 24.0));
        assert!(svg.is_some());
        assert!(svg.unwrap().contains("svg"));
    }

    #[test]
    fn math_text_builds_vobject() {
        let m = MathText::new("x^2", Color::WHITE);
        assert!(!m.latex.is_empty());
    }

    #[test]
    fn decimal_number_lerps() {
        let mut d = DecimalNumber::new(0.0, 2, Color::WHITE);
        d.lerp_value(10.0, 0.5, Color::WHITE);
        assert!((d.value - 5.0).abs() < 1e-9);
    }

    #[test]
    fn text_wrappers_build() {
        let i = Integer::new(42, Color::WHITE);
        assert_eq!(i.value, 42);
        let p = Paragraph::new(vec!["line one", "line two"], Color::WHITE);
        assert_eq!(p.lines.len(), 2);
        let c = Code::new("fn main() {}", Color::WHITE);
        assert!(!c.source.is_empty());
    }
}
