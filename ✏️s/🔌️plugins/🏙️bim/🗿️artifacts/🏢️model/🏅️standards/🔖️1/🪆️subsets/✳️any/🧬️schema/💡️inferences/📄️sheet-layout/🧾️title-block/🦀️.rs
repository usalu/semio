//! 🧾️ The title block and the revision table of a sheet: the cells of a three row title block (project and number, title and scale, drawn by, checked by, date and revision) in the bottom right corner of the frame, and the revision table that
//! grows upward from it. Cells carry the text to print and the rectangle to print it in; the heading of a cell is the localized name of its [`TitleField`] and belongs to whoever draws the sheet.
//! 📎 https://www.iso.org/standard/72482.html (ISO 7200, data fields in title blocks)

use super::{PaperRect, RevisionRow, RevisionTable, TitleBlock, TitleCell, TitleField};
use crate::{Sheet, SheetRevision};

/// 📏️ The width of the title block and the revision table on a roomy sheet; a narrow frame shrinks both to its width.
pub const TITLE_BLOCK_WIDTH: f64 = 180.0;

/// 📏️ The height of one of the three rows of the title block.
pub const ROW_HEIGHT: f64 = 16.0;

/// 📏️ The height of the header and of every row of the revision table.
pub const REVISION_ROW_HEIGHT: f64 = 6.0;

/// 📏️ The shares of the width of the revision table of its four columns: mark, date, description, author (they add up to [`TITLE_BLOCK_WIDTH`]).
pub const REVISION_COLUMNS: [f64; 4] = [14.0, 26.0, 110.0, 30.0];

/// 🔢️ The offsets of the five column edges of a revision table `width` wide, from its left edge.
pub fn column_edges(width: f64) -> [f64; 5] {
    let total: f64 = REVISION_COLUMNS.iter().sum();
    let mut edges = [0.0; 5];
    for (index, share) in REVISION_COLUMNS.iter().enumerate() {
        edges[index + 1] = edges[index] + width * share / total;
    }
    edges
}

/// 📏️ The scales of a sheet as one line: the distinct denominators from the largest scale to the smallest, `1:50, 1:100`; none when the sheet shows nothing.
pub fn scale_text(scales: &[u32]) -> String {
    let mut distinct: Vec<u32> = scales.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    distinct.into_iter().map(|scale| format!("1:{scale}")).collect::<Vec<_>>().join(", ")
}

/// 🧾️ The title block of `sheet` in the bottom right corner of `frame`. An empty scale line prints the scales of the viewports, an empty revision mark the mark of the last row of the revision table.
pub fn title_block(frame: &PaperRect, sheet: &Sheet, scales: &[u32], last_mark: Option<&str>) -> TitleBlock {
    let width = TITLE_BLOCK_WIDTH.min(frame.width);
    let row = ROW_HEIGHT.min(frame.height / 3.0);
    let rect = PaperRect { x: frame.right() - width, y: frame.bottom() - 3.0 * row, width, height: 3.0 * row };
    let (split, quarter) = (width * 110.0 / 180.0, width / 4.0);
    let cell = |field: TitleField, x: f64, line: u8, span: f64, value: String| TitleCell { field, rect: PaperRect { x: rect.x + x, y: rect.y + f64::from(line) * row, width: span, height: row }, value };
    let scale = if sheet.scale_label.is_empty() { scale_text(scales) } else { sheet.scale_label.clone() };
    let revision = if sheet.revision.is_empty() { last_mark.unwrap_or_default().to_string() } else { sheet.revision.clone() };
    let cells = vec![
        cell(TitleField::Project, 0.0, 0, split, sheet.project.clone()),
        cell(TitleField::Number, split, 0, width - split, sheet.number.clone()),
        cell(TitleField::Title, 0.0, 1, split, sheet.name.clone()),
        cell(TitleField::Scale, split, 1, width - split, scale),
        cell(TitleField::DrawnBy, 0.0, 2, quarter, sheet.drawn_by.clone()),
        cell(TitleField::CheckedBy, quarter, 2, quarter, sheet.checked_by.clone()),
        cell(TitleField::Date, 2.0 * quarter, 2, quarter, sheet.date.clone()),
        cell(TitleField::Revision, 3.0 * quarter, 2, width - 3.0 * quarter, revision),
    ];
    TitleBlock { rect, cells }
}

/// 🧾️ The revision table of the rows `rows` (in table order) directly above the title block `title`: a header row, then one row per revision from the top down. Without rows the table is empty and has no height.
pub fn revision_table(title: &PaperRect, rows: &[(&String, &SheetRevision)]) -> RevisionTable {
    let height = if rows.is_empty() { 0.0 } else { (rows.len() + 1) as f64 * REVISION_ROW_HEIGHT };
    let rect = PaperRect { x: title.x, y: title.y - height, width: title.width, height };
    let rows = rows
        .iter()
        .enumerate()
        .map(|(index, (id, row))| RevisionRow {
            revision: (*id).clone(),
            mark: row.number.clone(),
            date: row.date.clone(),
            description: row.description.clone(),
            author: row.author.clone(),
            rect: PaperRect { x: rect.x, y: rect.y + (index + 1) as f64 * REVISION_ROW_HEIGHT, width: rect.width, height: REVISION_ROW_HEIGHT },
        })
        .collect();
    RevisionTable { rect, rows }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
