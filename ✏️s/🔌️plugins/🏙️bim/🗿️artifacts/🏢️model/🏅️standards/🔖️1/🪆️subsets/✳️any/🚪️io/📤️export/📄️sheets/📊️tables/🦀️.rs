//! 📊️ The tables the sheet export oracles compare: what an SVG file of a sheet and the PDF of the set must show, derived from the `sheet-layout` and the marks the writers draw (never read back from the committed files).
//! The lxml + shapely oracle measures the SVG files and the pypdf oracle the PDF; both reproduce these tables from the files alone.
//! 📎 https://www.w3.org/TR/SVG11/masking.html#ClippingPaths

use super::ink::{sheet_marks, Mark, TitleLabels};
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::{PaperRect, SheetLayout};
use std::collections::BTreeMap;

#[derive(value_derive::ToValue)]
struct SvgViewport {
    scale: u32,
    window: Vec<f64>,
}

#[derive(value_derive::ToValue)]
struct SvgSheet {
    paper: String,
    size: Vec<f64>,
    viewports: BTreeMap<String, SvgViewport>,
    title_texts: Vec<String>,
    revision_texts: Vec<String>,
}

#[derive(value_derive::ToValue)]
struct PdfPageTable {
    width: f64,
    height: f64,
    has_number: bool,
    has_name: bool,
}

#[derive(value_derive::ToValue)]
struct PdfSet {
    pages: Vec<PdfPageTable>,
}

fn window(rect: &PaperRect) -> Vec<f64> {
    vec![rect.x, rect.y, rect.width, rect.height]
}

fn runs(marks: &[Mark]) -> Vec<String> {
    let mut texts: Vec<String> = marks.iter().filter_map(|mark| if let Mark::Text { text, .. } = mark { Some(text.clone()).filter(|text| !text.is_empty()) } else { None }).collect();
    texts.sort();
    texts
}

fn tenth(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

/// 🎨️ The canonical JSON table `sheet → { paper, size, viewports: id → { scale, window }, title_texts, revision_texts }` of the SVG files of `layouts`: the paper in millimetres, the clip window of every viewport and the sorted text runs of the title block and the revision table.
pub fn svg_table_json(layouts: &[&SheetLayout], labels: &TitleLabels) -> String {
    let table: BTreeMap<String, SvgSheet> = layouts
        .iter()
        .map(|layout| {
            let marks = sheet_marks(layout, labels);
            let viewports = layout.viewports.iter().map(|placed| (placed.viewport.clone(), SvgViewport { scale: placed.scale, window: window(&placed.window) })).collect();
            (layout.sheet.clone(), SvgSheet { paper: layout.paper.clone(), size: vec![layout.width, layout.height], viewports, title_texts: runs(&marks.title_block), revision_texts: runs(&marks.revisions) })
        })
        .collect();
    semio_framework_pack_json::to_json_string(&table)
}

/// 📖️ The canonical JSON table `{ pages: [{ width, height, has_number, has_name }] }` of the PDF of `layouts` in print order: the size of every page in millimetres to a tenth and whether its text shows the number and the title of the sheet.
pub fn pdf_table_json(layouts: &[&SheetLayout]) -> String {
    let pages = layouts.iter().map(|layout| PdfPageTable { width: tenth(layout.width), height: tenth(layout.height), has_number: !layout.number.is_empty(), has_name: !layout.name.is_empty() }).collect();
    semio_framework_pack_json::to_json_string(&PdfSet { pages })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
