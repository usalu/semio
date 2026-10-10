//! 📄️ `sheet-layout`: the composed geometry of every authored sheet, derived from the model and never stored. A sheet is a piece of paper of a size and orientation; its viewports place authored views on it at a scale (see `view-linework` for the drawing
//! of a view); the layout answers where everything goes on the paper, in millimetres from the top left corner, x to the right and y downward:
//!
//! * the **frame**: the paper without its margins (20 mm at the binding edge, 10 mm elsewhere),
//! * per viewport the **window** (the crop of the viewport, else the bounds of the drawing with 5 mm of air, at the scale of the viewport) and the **map** from view metres to paper millimetres, so the drawing of the view lands scaled in its window,
//! * the **title block** in the bottom right corner of the frame (project, number, title, scale, drawn by, checked by, date, revision) with the text of every cell,
//! * the **revision table** above it, one row per revision of the sheet,
//! * the **findings**: a viewport that reaches beyond the frame, windows that overlap, a window that covers the title block or the revision table, a view that draws nothing.
//!
//! The graph is `View(id)` of every viewport of the sheet → `Sheet(id)`: editing one sheet recomputes that sheet only, editing a view recomputes the sheets that show it. The linework itself is not copied: a consumer maps
//! `view_linework[view]` through [`ViewMap`] into the [`PaperRect`] window.
//!
//! Related: <https://www.iso.org/standard/72482.html>, <https://en.wikipedia.org/wiki/ISO_216>.

use super::super::element_solids::{dep_object, dep_value};
use super::super::view_linework::ViewLinework;
use crate::{revisions_of, viewports_of, ModelSnapshot, ViewKind};
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

#[path = "🧾️title-block/🦀️.rs"]
pub mod title_block;
#[path = "🖼️windows/🦀️.rs"]
pub mod windows;

//#region 🔖️Values
/// ▭️ A rectangle on the paper in millimetres from the top left corner of the sheet, x to the right and y downward.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PaperRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl PaperRect {
    /// ➡️ The x of the right edge.
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    /// ⬇️ The y of the bottom edge.
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    /// ✖️ Whether the rectangles share area (touching edges do not count).
    pub fn overlaps(&self, other: &PaperRect) -> bool {
        const EPS: f64 = 1e-6;
        self.x < other.right() - EPS && other.x < self.right() - EPS && self.y < other.bottom() - EPS && other.y < self.bottom() - EPS
    }

    /// 📦️ Whether `other` lies inside this rectangle (touching edges count as inside, with a micrometre of tolerance).
    pub fn contains(&self, other: &PaperRect) -> bool {
        const EPS: f64 = 1e-6;
        other.x >= self.x - EPS && other.y >= self.y - EPS && other.right() <= self.right() + EPS && other.bottom() <= self.bottom() + EPS
    }
}

/// 🧭️ The map from the coordinates of a view (metres, y up: north in a plan, the elevation above the datum in a section or elevation) to the window of its viewport: the point `(min_x, max_y)` lands on the top left corner of the
/// window, `mm` paper millimetres per metre, y flips.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ViewMap {
    pub min_x: f64,
    pub max_y: f64,
    pub mm: f64,
}

impl ViewMap {
    /// 📍️ The paper position of the view point `(x, y)` in a window whose top left corner is `window`.
    pub fn point(&self, window: &PaperRect, x: f64, y: f64) -> (f64, f64) {
        (window.x + (x - self.min_x) * self.mm, window.y + (self.max_y - y) * self.mm)
    }
}

/// 🖼️ One viewport placed on the paper.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PlacedViewport {
    pub viewport: String,
    pub view: String,
    pub label: String,
    pub kind: ViewKind,
    pub scale: u32,
    pub window: PaperRect,
    pub map: ViewMap,
    pub empty: bool,
    pub cropped: bool,
}

/// 🏷️ What a cell of the title block holds.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum TitleField {
    Project,
    Number,
    Title,
    Scale,
    DrawnBy,
    CheckedBy,
    Date,
    Revision,
}

impl TitleField {
    /// 🔢️ Every field in the order the title block lists them.
    pub const ALL: [TitleField; 8] = [Self::Project, Self::Number, Self::Title, Self::Scale, Self::DrawnBy, Self::CheckedBy, Self::Date, Self::Revision];

    /// 🔤️ The stable slug of the field.
    pub fn slug(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Number => "number",
            Self::Title => "title",
            Self::Scale => "scale",
            Self::DrawnBy => "drawn-by",
            Self::CheckedBy => "checked-by",
            Self::Date => "date",
            Self::Revision => "revision",
        }
    }
}

/// 🧾️ One cell of the title block: the field, its rectangle and the text to print.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct TitleCell {
    pub field: TitleField,
    pub rect: PaperRect,
    pub value: String,
}

/// 🧾️ The title block: its rectangle and its cells.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct TitleBlock {
    pub rect: PaperRect,
    pub cells: Vec<TitleCell>,
}

/// 🧾️ One row of the revision table.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RevisionRow {
    pub revision: String,
    pub mark: String,
    pub date: String,
    pub description: String,
    pub author: String,
    pub rect: PaperRect,
}

/// 🧾️ The revision table: its rectangle (header row included, no height without rows) and its rows.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RevisionTable {
    pub rect: PaperRect,
    pub rows: Vec<RevisionRow>,
}

/// ⚠️ What the layout of a sheet found wrong.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum SheetIssue {
    ViewportOutside,
    ViewportsOverlap,
    ViewportOverTitleBlock,
    ViewportEmpty,
}

impl SheetIssue {
    /// 🔤️ The stable slug of the issue.
    pub fn slug(self) -> &'static str {
        match self {
            Self::ViewportOutside => "viewport-outside",
            Self::ViewportsOverlap => "viewports-overlap",
            Self::ViewportOverTitleBlock => "viewport-over-title-block",
            Self::ViewportEmpty => "viewport-empty",
        }
    }

    /// 🌐️ The message in a locale (`en` or `de`) about the viewports `ids` (one or two); none for an unknown locale.
    pub fn message(self, locale: &str, ids: &[String]) -> Option<String> {
        let (first, second) = (ids.first().map_or("", String::as_str), ids.get(1).map_or("", String::as_str));
        Some(match (self, locale) {
            (Self::ViewportOutside, "en") => format!("Viewport {first} reaches beyond the frame of the sheet."),
            (Self::ViewportOutside, "de") => format!("Ansichtsfenster {first} ragt über den Rahmen des Blatts hinaus."),
            (Self::ViewportsOverlap, "en") => format!("Viewports {first} and {second} overlap."),
            (Self::ViewportsOverlap, "de") => format!("Ansichtsfenster {first} und {second} überlappen sich."),
            (Self::ViewportOverTitleBlock, "en") => format!("Viewport {first} covers the title block or the revision table."),
            (Self::ViewportOverTitleBlock, "de") => format!("Ansichtsfenster {first} verdeckt den Schriftkopf oder die Änderungstabelle."),
            (Self::ViewportEmpty, "en") => format!("Viewport {first} shows a view that draws nothing."),
            (Self::ViewportEmpty, "de") => format!("Ansichtsfenster {first} zeigt eine Ansicht ohne Zeichnung."),
            _ => return None,
        })
    }
}

/// ⚠️ One finding of a sheet: what it is and the viewports it names.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SheetFinding {
    pub issue: SheetIssue,
    pub viewports: Vec<String>,
}

impl SheetFinding {
    /// 🌐️ The message in a locale (`en` or `de`).
    pub fn text(&self, locale: &str) -> Option<String> {
        self.issue.message(locale, &self.viewports)
    }
}

/// 📄️ The layout of one sheet: its identity, its paper (the name, and the width and height as it lies), the frame, the placed viewports in id order, the title block, the revision table and the findings.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SheetLayout {
    pub sheet: String,
    pub number: String,
    pub name: String,
    pub paper: String,
    pub width: f64,
    pub height: f64,
    pub frame: PaperRect,
    pub viewports: Vec<PlacedViewport>,
    pub title_block: TitleBlock,
    pub revisions: RevisionTable,
    pub findings: Vec<SheetFinding>,
}
//#endregion 🔖️Values

//#region 🔖️Layout
fn findings_of(frame: &PaperRect, viewports: &[PlacedViewport], title: &PaperRect, revisions: &PaperRect) -> Vec<SheetFinding> {
    let mut found = Vec::new();
    for placed in viewports {
        let one = |issue| SheetFinding { issue, viewports: vec![placed.viewport.clone()] };
        if !frame.contains(&placed.window) {
            found.push(one(SheetIssue::ViewportOutside));
        }
        if placed.window.overlaps(title) || (revisions.height > 0.0 && placed.window.overlaps(revisions)) {
            found.push(one(SheetIssue::ViewportOverTitleBlock));
        }
        if placed.empty {
            found.push(one(SheetIssue::ViewportEmpty));
        }
    }
    for (index, first) in viewports.iter().enumerate() {
        for second in &viewports[index + 1..] {
            if first.window.overlaps(&second.window) {
                found.push(SheetFinding { issue: SheetIssue::ViewportsOverlap, viewports: vec![first.viewport.clone(), second.viewport.clone()] });
            }
        }
    }
    found
}

/// 📄️ The layout of the sheet `id` from the drawings of its views (by view id) its parents inferred. A viewport whose view is missing is not placed; a sheet that is not there has an empty layout.
pub fn layout_of(snapshot: &ModelSnapshot, id: &str, drawings: &BTreeMap<&str, &ViewLinework>) -> SheetLayout {
    let Some(sheet) = snapshot.sheets.get(id) else { return SheetLayout::default() };
    let (width, height) = sheet.size();
    let frame = windows::frame_of(width, height);
    let viewports: Vec<PlacedViewport> = viewports_of(snapshot, id)
        .into_iter()
        .filter_map(|(viewport_id, viewport)| snapshot.views.get(&viewport.view).map(|view| windows::place(viewport_id, viewport, view, drawings.get(viewport.view.as_str()).copied())))
        .collect();
    let rows = revisions_of(snapshot, id);
    let scales: Vec<u32> = viewports.iter().map(|placed| placed.scale).collect();
    let title = title_block::title_block(&frame, sheet, &scales, rows.last().map(|(_, row)| row.number.as_str()));
    let revisions = title_block::revision_table(&title.rect, &rows);
    let findings = findings_of(&frame, &viewports, &title.rect, &revisions.rect);
    SheetLayout { sheet: id.to_string(), number: sheet.number.clone(), name: sheet.name.clone(), paper: sheet.paper.name(), width, height, frame, viewports, title_block: title, revisions, findings }
}
//#endregion 🔖️Layout

//#region 🔖️Dependency
/// 🔑️ Everything the layout of the sheet `id` reads of the snapshot besides the drawings of its parents: the sheet, its viewports, its revision rows and the kind and name of every view they show (the label falls back to the name; the
/// drawing of a view does not depend on its name, so renaming a view recomputes the sheets that print it and no linework).
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(sheet) = snapshot.sheets.get(id) else { return DslValue::Null };
    let viewports = viewports_of(snapshot, id);
    let views: BTreeSet<&String> = viewports.iter().map(|(_, viewport)| &viewport.view).collect();
    dep_object([
        ("sheet", dep_value(sheet)),
        ("viewports", DslValue::object(viewports.iter().map(|(viewport_id, viewport)| ((*viewport_id).clone(), dep_value(*viewport))))),
        ("revisions", DslValue::object(revisions_of(snapshot, id).into_iter().map(|(row_id, row)| (row_id.clone(), dep_value(row))))),
        (
            "views",
            DslValue::object(views.into_iter().map(|view| (view.clone(), snapshot.views.get(view).map_or(DslValue::Null, |row| dep_object([("kind", dep_value(&row.kind)), ("name", dep_value(&row.name))]))))),
        ),
    ])
}

/// 📖️ The snapshot collections the layout of a sheet reads.
pub const READS: &[&str] = &["sheets", "viewports", "sheet_revisions", "views"];
//#endregion 🔖️Dependency

//#region 🔖️Metrics
#[derive(value_derive::ToValue)]
struct ViewportMetrics {
    scale: u32,
    mm: f64,
    window: Vec<f64>,
    cropped: bool,
    empty: bool,
}

#[derive(value_derive::ToValue)]
struct SheetMetrics {
    paper: String,
    size: Vec<f64>,
    frame: Vec<f64>,
    title: Vec<f64>,
    title_text: BTreeMap<String, String>,
    revisions: Vec<f64>,
    revision_rows: Vec<String>,
    viewports: BTreeMap<String, ViewportMetrics>,
    findings: Vec<String>,
}

fn rect_of(rect: &PaperRect) -> Vec<f64> {
    vec![rect.x, rect.y, rect.width, rect.height]
}

/// 📏️ The canonical JSON table `sheet → { paper, size, frame, title, title_text, revisions, revision_rows, viewports: id → { scale, mm, window, cropped, empty }, findings: ["<slug>|<ids>"] }` the sheet oracle compares; rectangles are `[x, y, width, height]` in paper millimetres.
pub fn metrics_json(layouts: &BTreeMap<String, SheetLayout>) -> String {
    let table: BTreeMap<String, SheetMetrics> = layouts
        .iter()
        .map(|(id, layout)| {
            let viewports = layout.viewports.iter().map(|placed| (placed.viewport.clone(), ViewportMetrics { scale: placed.scale, mm: placed.map.mm, window: rect_of(&placed.window), cropped: placed.cropped, empty: placed.empty })).collect();
            let title_text = layout.title_block.cells.iter().map(|cell| (cell.field.slug().to_string(), cell.value.clone())).collect();
            let revision_rows = layout.revisions.rows.iter().map(|row| format!("{}|{}|{}|{}", row.mark, row.date, row.description, row.author)).collect();
            let mut findings: Vec<String> = layout.findings.iter().map(|finding| format!("{}|{}", finding.issue.slug(), finding.viewports.join(","))).collect();
            findings.sort();
            let metrics = SheetMetrics { paper: layout.paper.clone(), size: vec![layout.width, layout.height], frame: rect_of(&layout.frame), title: rect_of(&layout.title_block.rect), title_text, revisions: rect_of(&layout.revisions.rect), revision_rows, viewports, findings };
            (id.clone(), metrics)
        })
        .collect();
    semio_framework_pack_json::to_json_string(&table)
}
//#endregion 🔖️Metrics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
