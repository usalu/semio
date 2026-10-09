//! 📖️ The sheet set as one PDF 1.7 (`s.stdio.pdf@1.7/*`), one page per sheet, written through the typed document of the `s.stdio.pdf` artifact: the page is the paper in points, one base matrix maps paper millimetres (y down) onto it, so every
//! coordinate of the content stream is the paper millimetre of the sheet layout. Every viewport clips to its window and paints the linework of its view (filled regions, stroked lines with the pens of the SVG export, texts in Helvetica; arcs
//! become cubic Béziers); the frame, captions, title block and revision table are the same marks the SVG draws. [`SheetsPdf`] writes page by page (`DocumentStream`), so an export can yield between sheets and be cancelled.
//! 📎 https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf

use super::ink::{sheet_marks, Anchor, Mark, TitleLabels};
use crate::standards::v1::subsets::any::io::export::svg::drawing::lines_of;
use crate::standards::v1::subsets::any::io::export::svg::style::{annotated, paint_rank, style_class, LABEL_SIZE, STROKE_WIDTHS, TAG_SIZE};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanLinework, PlanStyle, PlanText, PlanVertex};
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::{PaperRect, PlacedViewport, SheetLayout};
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::{DocumentStream, EncodeOptions, PdfTextLayout};
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::{PdfFont, PdfInfo, PdfLineCap, PdfLineJoin, PdfOp, PdfPage, PdfSnapshot, PdfTextString};
use std::collections::BTreeMap;

#[path = "🌀️arcs/🦀️.rs"]
pub mod arcs;

/// 📏️ PDF points per paper millimetre (72 points to the inch).
pub const POINTS_PER_MM: f64 = 72.0 / 25.4;

const REGULAR: &str = "F1";
const BOLD: &str = "F2";
const HIDDEN_DASH: [f64; 2] = [1.5, 0.75];
const LINE_SPACING: f64 = 1.15;
const CENTRAL_SHIFT: f64 = 0.35;

fn font(id: &str, base: &str) -> PdfFont {
    PdfFont::standard(id, base)
}

/// 🔤️ The text as WinAnsi can show it: Latin-1 letters and symbols stay, the arrow, the arc and everything else outside Latin-1 become `->`, `~` and `?`.
pub fn win_ansi(text: &str) -> String {
    text.chars()
        .flat_map(|letter| match letter {
            '\u{20}'..='\u{7e}' | '\u{a0}'..='\u{ff}' => vec![letter],
            '→' => vec!['-', '>'],
            '⌒' => vec!['~'],
            '\n' | '\t' => vec![' '],
            _ => vec!['?'],
        })
        .collect()
}

struct Ink<'a> {
    ops: Vec<PdfOp>,
    regular: PdfTextLayout<'a>,
    bold: PdfTextLayout<'a>,
}

impl Ink<'_> {
    fn stroke_width(&mut self, width: f64) {
        self.ops.push(PdfOp::SetLineWidth { width });
    }

    fn rectangle(&mut self, rect: &PaperRect) {
        self.ops.push(PdfOp::Rectangle { x: rect.x, y: rect.y, width: rect.width, height: rect.height });
    }

    fn text(&mut self, at: (f64, f64), size: f64, bold: bool, anchor: Anchor, content: &str, rotation: f64) {
        let content = win_ansi(content);
        if content.trim().is_empty() {
            return;
        }
        let layout = if bold { &self.bold } else { &self.regular };
        let width = layout.width(&content).unwrap_or(content.chars().count() as f64 * 0.5) * size;
        let shift = match anchor {
            Anchor::Start => 0.0,
            Anchor::Middle => width / 2.0,
            Anchor::End => width,
        };
        let (sin, cos) = rotation.sin_cos();
        let face = if bold { BOLD } else { REGULAR };
        self.ops.extend([
            PdfOp::BeginText,
            PdfOp::SetFont { name: face.into(), size },
            PdfOp::SetTextMatrix { matrix: [cos, -sin, -sin, -cos, at.0 - shift * cos, at.1 + shift * sin] },
            PdfOp::ShowText { text: PdfTextString::text(content) },
            PdfOp::EndText,
        ]);
    }

    fn mark(&mut self, mark: &Mark) {
        match mark {
            Mark::Rect { rect, width, .. } => {
                self.stroke_width(*width);
                self.rectangle(rect);
                self.ops.push(PdfOp::Stroke);
            }
            Mark::Line { from, to, width, .. } => {
                self.stroke_width(*width);
                self.ops.extend([PdfOp::MoveTo { x: from.0, y: from.1 }, PdfOp::LineTo { x: to.0, y: to.1 }, PdfOp::Stroke]);
            }
            Mark::Text { class, x, y, size, bold, anchor, text } => {
                let gray = if matches!(*class, "cell-label" | "revision-heading") { 0.27 } else { 0.0 };
                self.ops.push(PdfOp::SetFillGray { gray });
                self.text((*x, *y), *size, *bold, *anchor, text, 0.0);
            }
        }
    }
}

fn path(ops: &mut Vec<PdfOp>, vertices: &[PlanVertex], closed: bool, map: &dyn Fn(f64, f64) -> (f64, f64)) {
    let Some(first) = vertices.first() else { return };
    let start = map(first.x, first.y);
    ops.push(PdfOp::MoveTo { x: start.0, y: start.1 });
    let segments = if closed { vertices.len() } else { vertices.len() - 1 };
    for index in 0..segments {
        let (from, to) = (&vertices[index], &vertices[(index + 1) % vertices.len()]);
        if from.bulge.abs() > 1e-12 {
            for (first_control, second_control, end) in arcs::arc_cubics((from.x, from.y), (to.x, to.y), from.bulge) {
                let (a, b, c) = (map(first_control.0, first_control.1), map(second_control.0, second_control.1), map(end.0, end.1));
                ops.push(PdfOp::CurveTo { x1: a.0, y1: a.1, x2: b.0, y2: b.1, x3: c.0, y3: c.1 });
            }
        } else if index + 1 < vertices.len() {
            let end = map(to.x, to.y);
            ops.push(PdfOp::LineTo { x: end.0, y: end.1 });
        }
    }
    if closed {
        ops.push(PdfOp::ClosePath);
    }
}

fn pen(style: PlanStyle) -> f64 {
    STROKE_WIDTHS.iter().find(|(class, _)| *class == style_class(style)).map_or(0.25, |(_, width)| *width)
}

fn show_text(ink: &mut Ink<'_>, item: &PlanText, placed: &PlacedViewport) {
    let (x, y) = placed.map.point(&placed.window, item.x, item.y);
    let notation = annotated(item.kind);
    let centred = notation || matches!(item.kind, PlanKind::SpaceTag | PlanKind::GridLabel);
    let size = if notation && item.height > 0.0 {
        item.height * placed.map.mm
    } else if item.kind == PlanKind::GridLabel {
        LABEL_SIZE
    } else {
        TAG_SIZE
    };
    let lines = lines_of(item);
    let step = TAG_SIZE * LINE_SPACING;
    let lift = -((lines.len() - 1) as f64) * step / 2.0;
    let central = if item.kind == PlanKind::GridLabel { CENTRAL_SHIFT * size } else { 0.0 };
    let (sin, cos) = item.rotation.sin_cos();
    ink.ops.push(PdfOp::SetFillGray { gray: 0.0 });
    for (index, line) in lines.iter().enumerate() {
        let offset = lift + index as f64 * step + central;
        ink.text((x + offset * sin, y + offset * cos), size, false, if centred { Anchor::Middle } else { Anchor::Start }, line, item.rotation);
    }
}

fn meter<'a>(face: &'a PdfFont, resources: &NativePdfArtifactResources) -> Result<PdfTextLayout<'a>, String> {
    PdfTextLayout::new(face, 1.0, resources).map_err(|error| error.to_string())
}

fn drawing(ink: &mut Ink<'_>, lines: &PlanLinework, placed: &PlacedViewport) {
    let map = |x: f64, y: f64| placed.map.point(&placed.window, x, y);
    ink.ops.extend([PdfOp::SetLineCap { cap: PdfLineCap::Round }, PdfOp::SetLineJoin { join: PdfLineJoin::Round }, PdfOp::SetStrokeGray { gray: 0.0 }]);
    let mut regions: Vec<_> = lines.regions.iter().collect();
    regions.sort_by_key(|region| paint_rank(region.style));
    for region in regions {
        let fill = match region.style {
            PlanStyle::Cut => Some(0.1),
            PlanStyle::Projection => Some(0.9),
            _ => None,
        };
        ink.stroke_width(pen(region.style));
        if let Some(gray) = fill {
            ink.ops.push(PdfOp::SetFillGray { gray });
        }
        for ring in std::iter::once(&region.outer).chain(region.holes.iter()) {
            path(&mut ink.ops, ring, true, &map);
        }
        ink.ops.push(if fill.is_some() { PdfOp::FillStrokeEvenOdd } else { PdfOp::Stroke });
    }
    let mut strokes: Vec<_> = lines.polylines.iter().filter(|line| !annotated(line.kind)).collect();
    strokes.sort_by_key(|line| paint_rank(line.style));
    let mut dashed = false;
    for line in strokes.into_iter().chain(lines.polylines.iter().filter(|line| annotated(line.kind))) {
        let hidden = line.style == PlanStyle::Hidden;
        if hidden != dashed {
            ink.ops.push(PdfOp::SetDash { array: if hidden { HIDDEN_DASH.to_vec() } else { Vec::new() }, phase: 0.0 });
            dashed = hidden;
        }
        ink.stroke_width(pen(line.style));
        path(&mut ink.ops, &line.vertices, line.closed, &map);
        ink.ops.push(PdfOp::Stroke);
    }
    if dashed {
        ink.ops.push(PdfOp::SetDash { array: Vec::new(), phase: 0.0 });
    }
    for text in lines.texts.iter().filter(|text| !annotated(text.kind)).chain(lines.texts.iter().filter(|text| annotated(text.kind))) {
        show_text(ink, text, placed);
    }
}

/// 📖️ The page of one sheet: the paper in points, the frame, every viewport clipped to its window with the linework of its view in `drawings` (by view id), the captions, the title block and the revision table.
pub fn pdf_page(layout: &SheetLayout, drawings: &BTreeMap<&str, &ViewLinework>, labels: &TitleLabels) -> Result<PdfPage, String> {
    let (regular, bold) = (font(REGULAR, "Helvetica"), font(BOLD, "Helvetica-Bold"));
    let resources = NativePdfArtifactResources::default();
    let mut ink = Ink { ops: Vec::new(), regular: meter(&regular, &resources)?, bold: meter(&bold, &resources)? };
    let height = layout.height * POINTS_PER_MM;
    ink.ops.extend([PdfOp::Save, PdfOp::Transform { matrix: [POINTS_PER_MM, 0.0, 0.0, -POINTS_PER_MM, 0.0, height] }, PdfOp::SetStrokeGray { gray: 0.0 }, PdfOp::SetFillGray { gray: 0.0 }]);
    let marks = sheet_marks(layout, labels);
    marks.frame.iter().for_each(|mark| ink.mark(mark));
    for placed in &layout.viewports {
        ink.ops.push(PdfOp::Save);
        ink.rectangle(&placed.window);
        ink.ops.extend([PdfOp::Clip, PdfOp::EndPath]);
        if let Some(view) = drawings.get(placed.view.as_str()) {
            drawing(&mut ink, &view.lines, placed);
        }
        ink.ops.push(PdfOp::Restore);
    }
    marks.captions.iter().chain(&marks.title_block).chain(&marks.revisions).for_each(|mark| ink.mark(mark));
    ink.ops.push(PdfOp::Restore);
    let mut page = PdfPage::new(layout.width * POINTS_PER_MM, height);
    page.content = ink.ops;
    Ok(page)
}

/// 📖️ A PDF written page by page: [`SheetsPdf::begin`] lowers the document (fonts, info) and yields the header, every [`SheetsPdf::page`] appends the bytes of one sheet, [`SheetsPdf::finish`] closes the file.
pub struct SheetsPdf {
    stream: DocumentStream,
    bytes: Vec<u8>,
}

impl SheetsPdf {
    /// 🏁 Starts the document of `expected_pages` sheets titled `title`.
    pub fn begin(title: &str, expected_pages: usize) -> Result<Self, String> {
        let mut snapshot = PdfSnapshot::default();
        snapshot.fonts = vec![font(REGULAR, "Helvetica"), font(BOLD, "Helvetica-Bold")];
        snapshot.info = PdfInfo { title: Some(win_ansi(title)), creator: Some("semio BIM".into()), producer: Some("s.stdio.pdf 1.7".into()), ..PdfInfo::default() };
        let (stream, bytes) = DocumentStream::begin(&snapshot, expected_pages, EncodeOptions::export()).map_err(|error| error.to_string())?;
        Ok(Self { stream, bytes })
    }

    /// 📄 Appends the page of one sheet.
    pub fn page(&mut self, layout: &SheetLayout, drawings: &BTreeMap<&str, &ViewLinework>, labels: &TitleLabels) -> Result<(), String> {
        let page = pdf_page(layout, drawings, labels)?;
        let bytes = self.stream.page(&page).map_err(|error| error.to_string())?;
        self.bytes.extend(bytes);
        Ok(())
    }

    /// 🏁 Closes the document and returns the file.
    pub fn finish(mut self) -> Result<Vec<u8>, String> {
        let tail = self.stream.finish().map_err(|error| error.to_string())?;
        self.bytes.extend(tail);
        Ok(self.bytes)
    }
}

/// 📖️ The PDF of the sheets `layouts` in order, one page each.
pub fn sheets_pdf(title: &str, layouts: &[&SheetLayout], drawings: &BTreeMap<&str, &ViewLinework>, labels: &TitleLabels) -> Result<Vec<u8>, String> {
    let mut pdf = SheetsPdf::begin(title, layouts.len())?;
    for layout in layouts {
        pdf.page(layout, drawings, labels)?;
    }
    pdf.finish()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
