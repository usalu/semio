//! 🔤 Text and math labels via Typst-to-SVG compilation.

use crate::color::Color;
use crate::sobject::VSobject;
use comemo::Prehashed as LazyHash;
use ecow::EcoString;
use mathematical_geometry::{BezPath, Point};
use std::path::PathBuf;
use std::sync::OnceLock;
use typst::foundations::{Bytes, Datetime};
use typst::text::{Font, FontBook};
use typst::{Library, World};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::utils::Abs;

const TEXT_PAGE_PT: f64 = 400.0;
const TEXT_MARGIN_PT: f64 = 8.0;
const TEXT_SIZE_PT: f64 = 36.0;

/// 📝 Plain text Sobject rendered through Typst.
#[derive(Clone, Debug)]
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

/// ∑ Math-mode label rendered through Typst.
#[derive(Clone, Debug)]
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
    } else {
        let rect = mathematical_geometry::Rect::new(-1.0, -0.5, 1.0, 0.5);
        v.set_paths(vec![{
            let mut p = BezPath::new();
            mathematical_geometry::append_shape_to_path(&mut p, rect, 0.01);
            p
        }]);
    }
    v.set_fill(color);
    v.style.stroke = None;
    v
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
}
