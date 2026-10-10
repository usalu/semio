//! ✒️ The ink of a sheet that does not come from a view: the frame, the captions of the viewports, the title block and the revision table as neutral marks (rectangles, lines and texts in paper millimetres, y downward). The SVG and the PDF writers
//! draw the same marks, so the two files show the same sheet. The headings of the cells and of the revision columns come from the caller in the language it prints in.
//! 📎 https://www.iso.org/standard/72482.html

use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::title_block::column_edges;
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::{PaperRect, SheetLayout, TitleField};

/// ↔️ Where a text stands on its x: it starts at, is centred on or ends at its position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    Start,
    Middle,
    End,
}

/// ✒️ One mark of ink in paper millimetres: a stroked rectangle or line of a width, or a text of a size whose baseline starts, is centred or ends at `x, y`.
#[derive(Clone, Debug, PartialEq)]
pub enum Mark {
    Rect { class: &'static str, rect: PaperRect, width: f64 },
    Line { class: &'static str, from: (f64, f64), to: (f64, f64), width: f64 },
    Text { class: &'static str, x: f64, y: f64, size: f64, bold: bool, anchor: Anchor, text: String },
}

/// 🗣️ The headings a sheet prints, in the language of the export: the name of every title block cell and the four revision columns.
#[derive(Clone, Debug, PartialEq)]
pub struct TitleLabels {
    pub project: String,
    pub number: String,
    pub title: String,
    pub scale: String,
    pub drawn_by: String,
    pub checked_by: String,
    pub date: String,
    pub revision: String,
    pub revision_mark: String,
    pub revision_date: String,
    pub revision_description: String,
    pub revision_author: String,
}

impl TitleLabels {
    /// 🇬🇧 The English headings, the language of the serializer of the io mechanism, which has no locale to ask.
    pub fn english() -> Self {
        let text = |value: &str| value.to_string();
        Self { project: text("Project"), number: text("Sheet no."), title: text("Title"), scale: text("Scale"), drawn_by: text("Drawn by"), checked_by: text("Checked by"), date: text("Date"), revision: text("Revision"), revision_mark: text("Mark"), revision_date: text("Date"), revision_description: text("Description"), revision_author: text("By") }
    }

    /// 🏷️ The heading of a title block cell.
    pub fn field(&self, field: TitleField) -> &str {
        match field {
            TitleField::Project => &self.project,
            TitleField::Number => &self.number,
            TitleField::Title => &self.title,
            TitleField::Scale => &self.scale,
            TitleField::DrawnBy => &self.drawn_by,
            TitleField::CheckedBy => &self.checked_by,
            TitleField::Date => &self.date,
            TitleField::Revision => &self.revision,
        }
    }

    /// 🏷️ The headings of the four revision columns.
    pub fn columns(&self) -> [&str; 4] {
        [&self.revision_mark, &self.revision_date, &self.revision_description, &self.revision_author]
    }
}

/// ✒️ The marks of a sheet that are not drawings, grouped the way the writers nest them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SheetMarks {
    pub frame: Vec<Mark>,
    pub captions: Vec<Mark>,
    pub title_block: Vec<Mark>,
    pub revisions: Vec<Mark>,
}

/// 📏️ The stroke width of the frame.
pub const FRAME_WIDTH: f64 = 0.7;

/// 📏️ The stroke widths of the outline of the title block and the revision table, and of their inner lines.
pub const OUTLINE_WIDTH: f64 = 0.5;
pub const GRID_WIDTH: f64 = 0.18;

/// 📏️ The distance from the cell edge to its text and the sizes of headings and values at the full row height of 16 millimetres.
const INSET: f64 = 1.5;
const HEADING_SIZE: f64 = 2.0;
const REVISION_SIZE: f64 = 2.8;
const CAPTION_SIZE: f64 = 3.5;
const CAPTION_GAP: f64 = 5.0;
const FULL_ROW: f64 = 16.0;

fn value_size(field: TitleField) -> (f64, bool) {
    match field {
        TitleField::Title | TitleField::Number => (5.0, true),
        TitleField::Project => (4.0, false),
        _ => (3.5, false),
    }
}

fn caption_text(label: &str, scale: u32) -> String {
    format!("{label}   1:{scale}")
}

/// ✒️ The marks of `layout`: the frame, a caption under every window, the title block with the text of its cells and the revision table. `labels` are the headings in the language of the export.
pub fn sheet_marks(layout: &SheetLayout, labels: &TitleLabels) -> SheetMarks {
    let frame = vec![Mark::Rect { class: "frame", rect: layout.frame, width: FRAME_WIDTH }];
    let captions = layout
        .viewports
        .iter()
        .map(|placed| Mark::Text { class: "caption", x: placed.window.x, y: placed.window.bottom() + CAPTION_GAP, size: CAPTION_SIZE, bold: true, anchor: Anchor::Start, text: caption_text(&placed.label, placed.scale) })
        .collect();
    let mut title_block = vec![Mark::Rect { class: "title-block", rect: layout.title_block.rect, width: OUTLINE_WIDTH }];
    for cell in &layout.title_block.cells {
        let k = (cell.rect.height / FULL_ROW).min(1.0);
        let (size, bold) = value_size(cell.field);
        title_block.push(Mark::Rect { class: "title-cell", rect: cell.rect, width: GRID_WIDTH });
        title_block.push(Mark::Text { class: "cell-label", x: cell.rect.x + INSET, y: cell.rect.y + 3.2 * k, size: HEADING_SIZE * k, bold: false, anchor: Anchor::Start, text: labels.field(cell.field).to_string() });
        title_block.push(Mark::Text { class: "cell-value", x: cell.rect.x + INSET, y: cell.rect.bottom() - 4.5 * k, size: size * k, bold, anchor: Anchor::Start, text: cell.value.clone() });
    }
    let mut revisions = Vec::new();
    let table = &layout.revisions;
    if !table.rows.is_empty() {
        let edges = column_edges(table.rect.width);
        revisions.push(Mark::Rect { class: "revision-table", rect: table.rect, width: OUTLINE_WIDTH });
        for (index, heading) in labels.columns().into_iter().enumerate() {
            revisions.push(Mark::Text { class: "revision-heading", x: table.rect.x + edges[index] + INSET, y: table.rect.y + 4.2, size: REVISION_SIZE, bold: true, anchor: Anchor::Start, text: heading.to_string() });
        }
        for edge in &edges[1..4] {
            revisions.push(Mark::Line { class: "revision-column", from: (table.rect.x + edge, table.rect.y), to: (table.rect.x + edge, table.rect.bottom()), width: GRID_WIDTH });
        }
        for row in &table.rows {
            revisions.push(Mark::Rect { class: "revision-row", rect: row.rect, width: GRID_WIDTH });
            for (index, text) in [&row.mark, &row.date, &row.description, &row.author].into_iter().enumerate() {
                revisions.push(Mark::Text { class: "revision-text", x: row.rect.x + edges[index] + INSET, y: row.rect.y + 4.2, size: REVISION_SIZE, bold: false, anchor: Anchor::Start, text: text.clone() });
            }
        }
    }
    SheetMarks { frame, captions, title_block, revisions }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
