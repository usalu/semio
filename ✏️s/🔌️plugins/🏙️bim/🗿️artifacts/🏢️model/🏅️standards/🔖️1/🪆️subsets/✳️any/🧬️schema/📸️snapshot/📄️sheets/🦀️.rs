//! 📄️ Authored-sheet rules shared by the nine sheet leaves, the editor, the `sheet-layout` inference and the exports: the paper sizes, the conventions of a new sheet and viewport and why a sheet, a viewport or a revision row cannot be
//! written. Pure, total and read-only: a refusal names the field below the record and carries the message.
//! 📎 https://en.wikipedia.org/wiki/ISO_216

use super::{IsoSize, ModelSnapshot, Orientation, Paper, Point2, Sheet, SheetRevision, ViewCrop, ViewKind, Viewport};

//#region 🔖️Defaults
/// 📏️ The smallest scale denominator a viewport accepts (1:1).
pub const MIN_VIEWPORT_SCALE: u32 = 1;

/// 📏️ The largest scale denominator a viewport accepts (1:1000).
pub const MAX_VIEWPORT_SCALE: u32 = 1_000;

/// 📏️ The scale denominator of a new viewport.
pub const DEFAULT_VIEWPORT_SCALE: u32 = 100;

/// 📐️ The shortest side of a custom paper in millimetres.
pub const MIN_PAPER_SIDE: f64 = 50.0;

/// 📐️ The longest side of a custom paper in millimetres.
pub const MAX_PAPER_SIDE: f64 = 10_000.0;

/// 📐️ The standard sizes in the order they are offered, A4 first.
pub const ISO_SIZES: [IsoSize; 5] = [IsoSize::A4, IsoSize::A3, IsoSize::A2, IsoSize::A1, IsoSize::A0];

impl IsoSize {
    /// 📐️ The (long, short) sides in millimetres: 1189 by 841 for A0 down to 297 by 210 for A4.
    pub fn sides(self) -> (f64, f64) {
        match self {
            Self::A0 => (1189.0, 841.0),
            Self::A1 => (841.0, 594.0),
            Self::A2 => (594.0, 420.0),
            Self::A3 => (420.0, 297.0),
            Self::A4 => (297.0, 210.0),
        }
    }

    /// 🏷️ The name of the size.
    pub fn name(self) -> &'static str {
        match self {
            Self::A0 => "A0",
            Self::A1 => "A1",
            Self::A2 => "A2",
            Self::A3 => "A3",
            Self::A4 => "A4",
        }
    }

    /// 🔎️ The size a name spells, ignoring case.
    pub fn parse(text: &str) -> Option<Self> {
        ISO_SIZES.into_iter().find(|size| size.name().eq_ignore_ascii_case(text.trim()))
    }
}

impl Paper {
    /// 📄️ The standard paper of `size`.
    pub fn iso(size: IsoSize) -> Self {
        Self::Iso { size }
    }

    /// 📐️ The (long, short) sides in millimetres.
    pub fn sides(&self) -> (f64, f64) {
        match self {
            Self::Iso { size } => size.sides(),
            Self::Custom { width, height } => (width.max(*height), width.min(*height)),
        }
    }

    /// 🏷️ The name of the paper: `A3`, or `600 × 400` for a custom size.
    pub fn name(&self) -> String {
        match self {
            Self::Iso { size } => size.name().to_string(),
            Self::Custom { .. } => {
                let (long, short) = self.sides();
                format!("{long} × {short}")
            }
        }
    }
}

impl Orientation {
    /// 🔎️ The orientation a name spells, ignoring case.
    pub fn parse(text: &str) -> Option<Self> {
        [Self::Landscape, Self::Portrait].into_iter().find(|orientation| format!("{orientation:?}").eq_ignore_ascii_case(text.trim()))
    }
}

impl Sheet {
    /// 📄️ A sheet with the conventions of a new one: A3 landscape, no title block text.
    pub fn standard(number: &str, name: &str) -> Self {
        Self { number: number.into(), name: name.into(), paper: Paper::iso(IsoSize::A3), orientation: Orientation::Landscape, project: String::new(), drawn_by: String::new(), checked_by: String::new(), date: String::new(), revision: String::new(), scale_label: String::new() }
    }

    /// 📐️ The (width, height) of the paper in millimetres as it lies: the long side is the width for a landscape sheet and the height for a portrait one.
    pub fn size(&self) -> (f64, f64) {
        let (long, short) = self.paper.sides();
        match self.orientation {
            Orientation::Landscape => (long, short),
            Orientation::Portrait => (short, long),
        }
    }
}

impl Viewport {
    /// 🖼️ A viewport of `view` on `sheet` with the conventions of a new one: 1:100, no crop, titled by its view.
    pub fn standard(sheet: &str, view: &str, position: Point2) -> Self {
        Self { sheet: sheet.into(), view: view.into(), position, scale: DEFAULT_VIEWPORT_SCALE, crop: None, label: None }
    }
}

/// 🔎️ Whether a view of `kind` draws on paper: the plans, sections and elevations do, the cameras are rendered by the 3D window.
pub fn view_is_drawn(kind: ViewKind) -> bool {
    matches!(kind, ViewKind::Plan | ViewKind::CeilingPlan | ViewKind::Section | ViewKind::Elevation)
}
//#endregion 🔖️Defaults

//#region 🔖️Problems
/// 🚫️ Why a sheet, viewport or revision row cannot be written: the field below the record, the human message and whether a referenced record is absent (else a value breaks an invariant).
#[derive(Clone, Debug, PartialEq)]
pub struct SheetProblem {
    pub missing: bool,
    pub field: &'static str,
    pub message: String,
}

fn invariant(field: &'static str, message: impl Into<String>) -> Option<SheetProblem> {
    Some(SheetProblem { missing: false, field, message: message.into() })
}

fn missing(field: &'static str, message: impl Into<String>) -> Option<SheetProblem> {
    Some(SheetProblem { missing: true, field, message: message.into() })
}

/// 📅️ Whether `text` is empty or a calendar date as `year-month-day` (four digit year, two digit month 01 to 12 and day 01 to 31).
pub fn valid_date(text: &str) -> bool {
    if text.is_empty() {
        return true;
    }
    let bytes = text.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let digits = |range: std::ops::Range<usize>| bytes[range].iter().all(u8::is_ascii_digit);
    if !(digits(0..4) && digits(5..7) && digits(8..10)) {
        return false;
    }
    let number = |range: std::ops::Range<usize>| text[range].parse::<u32>().unwrap_or(0);
    (1..=12).contains(&number(5..7)) && (1..=31).contains(&number(8..10))
}

fn paper_problem(paper: &Paper) -> Option<SheetProblem> {
    let Paper::Custom { width, height } = paper else { return None };
    let sides = [*width, *height];
    if !sides.iter().all(|side| side.is_finite() && (MIN_PAPER_SIDE..=MAX_PAPER_SIDE).contains(side)) {
        return invariant("paper", format!("A custom paper side must be between {MIN_PAPER_SIDE} and {MAX_PAPER_SIDE} millimetres."));
    }
    None
}

/// 🚫️ Why `sheet` (to be stored under `id`) cannot be written, none when it can: the number and the name are filled, the number is unique within the project, a custom paper has two sides of a printable size and the date is
/// empty or `year-month-day`.
pub fn sheet_problem(base: &ModelSnapshot, id: &str, sheet: &Sheet) -> Option<SheetProblem> {
    if sheet.number.trim().is_empty() {
        return invariant("number", "A sheet number must not be blank.");
    }
    if base.sheets.iter().any(|(other, row)| other != id && row.number == sheet.number) {
        return invariant("number", format!("The project already has a sheet numbered \"{}\".", sheet.number));
    }
    if sheet.name.trim().is_empty() {
        return invariant("name", "A sheet name must not be blank.");
    }
    if let Some(problem) = paper_problem(&sheet.paper) {
        return Some(problem);
    }
    if !valid_date(&sheet.date) {
        return invariant("date", "A sheet date must be empty or a date as year-month-day (for example 2026-10-09).");
    }
    None
}

fn crop_problem(crop: &ViewCrop) -> Option<SheetProblem> {
    let corners = [crop.min.x, crop.min.y, crop.max.x, crop.max.y];
    if !corners.iter().all(|value| value.is_finite()) {
        return invariant("crop", "A crop rectangle needs finite corners.");
    }
    (!(crop.min.x < crop.max.x && crop.min.y < crop.max.y)).then(|| SheetProblem { missing: false, field: "crop", message: "A crop rectangle needs its minimum corner below and left of its maximum corner.".into() })
}

/// 🚫️ Why `viewport` (to be stored under `id`) cannot be written, none when it can: its sheet exists, its view exists and draws on paper, the scale is a whole number from 1 to 1000, the position is finite, the crop has a size and the label,
/// when given, is filled.
pub fn viewport_problem(base: &ModelSnapshot, _id: &str, viewport: &Viewport) -> Option<SheetProblem> {
    if !base.sheets.contains_key(&viewport.sheet) {
        return missing("sheet", format!("Sheet \"{}\" does not exist.", viewport.sheet));
    }
    let Some(view) = base.views.get(&viewport.view) else {
        return missing("view", format!("View \"{}\" does not exist.", viewport.view));
    };
    if !view_is_drawn(view.kind) {
        return invariant("view", format!("View \"{}\" is a camera and draws nothing on paper.", viewport.view));
    }
    if !(MIN_VIEWPORT_SCALE..=MAX_VIEWPORT_SCALE).contains(&viewport.scale) {
        return invariant("scale", format!("A viewport scale must be between {MIN_VIEWPORT_SCALE} and {MAX_VIEWPORT_SCALE}."));
    }
    if !(viewport.position.x.is_finite() && viewport.position.y.is_finite()) {
        return invariant("position", "A viewport position needs finite coordinates.");
    }
    if let Some(crop) = &viewport.crop {
        if let Some(problem) = crop_problem(crop) {
            return Some(problem);
        }
    }
    if viewport.label.as_ref().is_some_and(|label| label.trim().is_empty()) {
        return invariant("label", "A viewport label must not be blank.");
    }
    None
}

/// 🚫️ Why `revision` (to be stored under `id`) cannot be written, none when it can: its sheet exists, the mark is filled and unique within the sheet, the date is empty or `year-month-day` and the description is filled.
pub fn revision_problem(base: &ModelSnapshot, id: &str, revision: &SheetRevision) -> Option<SheetProblem> {
    if !base.sheets.contains_key(&revision.sheet) {
        return missing("sheet", format!("Sheet \"{}\" does not exist.", revision.sheet));
    }
    if revision.number.trim().is_empty() {
        return invariant("number", "A revision mark must not be blank.");
    }
    if base.sheet_revisions.iter().any(|(other, row)| other != id && row.sheet == revision.sheet && row.number == revision.number) {
        return invariant("number", format!("Sheet \"{}\" already has a revision \"{}\".", revision.sheet, revision.number));
    }
    if !valid_date(&revision.date) {
        return invariant("date", "A revision date must be empty or a date as year-month-day (for example 2026-10-09).");
    }
    if revision.description.trim().is_empty() {
        return invariant("description", "A revision description must not be blank.");
    }
    None
}
//#endregion 🔖️Problems

/// 🧾️ The revision rows of `sheet` in table order: by mark, then id.
pub fn revisions_of<'a>(base: &'a ModelSnapshot, sheet: &str) -> Vec<(&'a String, &'a SheetRevision)> {
    let mut rows: Vec<(&String, &SheetRevision)> = base.sheet_revisions.iter().filter(|(_, row)| row.sheet == sheet).collect();
    rows.sort_by(|a, b| (&a.1.number, a.0).cmp(&(&b.1.number, b.0)));
    rows
}

/// 🧾️ The viewports of `sheet` in drawing order: by id.
pub fn viewports_of<'a>(base: &'a ModelSnapshot, sheet: &str) -> Vec<(&'a String, &'a Viewport)> {
    base.viewports.iter().filter(|(_, row)| row.sheet == sheet).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
