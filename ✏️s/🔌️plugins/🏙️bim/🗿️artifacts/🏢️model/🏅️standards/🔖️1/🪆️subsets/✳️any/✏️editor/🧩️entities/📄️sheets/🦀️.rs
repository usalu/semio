//! 📄️ The sheet rows of the entity table: how an authored sheet, viewport and revision row read off the snapshot, how an edited value becomes a `set-*` mutation, what a new one is, and the pickers (orientation, sheets, drawable views) the
//! properties panel offers. The size of the paper, the scales and the window of a viewport are inferred rows of the `sheet-layout` inference.

use super::views::{pair_text, parse_crop};
use super::{number, parse_count, parse_point, parse_text, partial, point, Created, EntityKind, FieldRow, InferredRow};
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::io::export::sheets;
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::title_block::scale_text;
use crate::{view_is_drawn, Assigned, IsoSize, ModelMutation, ModelSnapshot, Orientation, Paper, Point2, Sheet, SheetRevision, Viewport, DEFAULT_VIEWPORT_SCALE, MAX_VIEWPORT_SCALE};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

//#region 🔖️Text
/// 📄️ The paper a text names: an ISO size (`A3`) or two sides in millimetres (`600 × 400`, `600 x 400`).
pub fn parse_paper(text: &str) -> Option<Paper> {
    if let Some(size) = IsoSize::parse(text) {
        return Some(Paper::iso(size));
    }
    let (first, second) = text.split_once('×').or_else(|| text.split_once(['x', 'X']))?;
    let sides = (first.trim().parse::<f64>().ok()?, second.trim().parse::<f64>().ok()?);
    (sides.0.is_finite() && sides.1.is_finite()).then_some(Paper::Custom { width: sides.0, height: sides.1 })
}

fn parse_orientation(text: &str) -> Option<Orientation> {
    Orientation::parse(text)
}

fn parse_label(text: &str) -> Option<Assigned<Option<String>>> {
    let text = text.trim();
    Some(Assigned::new((!text.is_empty()).then(|| text.to_string())))
}

fn crop_text(row: &Viewport) -> String {
    row.crop.map_or_else(String::new, |crop| pair_text(crop.min, crop.max))
}

fn window_text(width: f64, height: f64) -> String {
    format!("{} × {}", number((width * 100.0).round() / 100.0), number((height * 100.0).round() / 100.0))
}
//#endregion 🔖️Text

//#region 🔖️Choices
/// 🔄️ The orientations of a sheet, by their stored name and their localized label.
pub fn orientation_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    vec![("Landscape".to_string(), labels.choice_landscape.as_str().to_string()), ("Portrait".to_string(), labels.choice_portrait.as_str().to_string())]
}

/// 📄️ The sheets of the model by id and number and name, in print order.
pub fn sheet_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    sheets::ordered(snapshot).into_iter().filter_map(|id| snapshot.sheets.get(&id).map(|row| (id.clone(), format!("{} {}", row.number, row.name)))).collect()
}

/// 🖼️ The views that draw on paper by id and name.
pub fn view_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.views.iter().filter(|(_, view)| view_is_drawn(view.kind)).map(|(id, view)| (id.clone(), view.name.clone())).collect()
}
//#endregion 🔖️Choices

//#region 🔖️Fields
/// 🧾️ The authored parameters of a sheet.
pub static SHEET_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.sheets.get(id).map(|row| row.name.clone()), parse_text => set_sheet::SetSheet),
    field!("number", field_number, Text, |s, id| s.sheets.get(id).map(|row| row.number.clone()), parse_text => set_sheet::SetSheet),
    field!("paper", field_paper, Text, |s, id| s.sheets.get(id).map(|row| row.paper.name()), parse_paper => set_sheet::SetSheet),
    field!("orientation", field_orientation, Text, |s, id| s.sheets.get(id).map(|row| format!("{:?}", row.orientation)), choices: orientation_choices, parse_orientation => set_sheet::SetSheet),
    field!("project", field_project_line, Text, |s, id| s.sheets.get(id).map(|row| row.project.clone()), parse_text => set_sheet::SetSheet),
    field!("drawn_by", field_drawn_by, Text, |s, id| s.sheets.get(id).map(|row| row.drawn_by.clone()), parse_text => set_sheet::SetSheet),
    field!("checked_by", field_checked_by, Text, |s, id| s.sheets.get(id).map(|row| row.checked_by.clone()), parse_text => set_sheet::SetSheet),
    field!("date", field_date, Text, |s, id| s.sheets.get(id).map(|row| row.date.clone()), parse_text => set_sheet::SetSheet),
    field!("revision", field_revision_mark, Text, |s, id| s.sheets.get(id).map(|row| row.revision.clone()), parse_text => set_sheet::SetSheet),
    field!("scale_label", field_scale_label, Text, |s, id| s.sheets.get(id).map(|row| row.scale_label.clone()), parse_text => set_sheet::SetSheet),
];

/// 🧾️ The authored parameters of a viewport.
pub static VIEWPORT_FIELDS: &[FieldRow] = &[
    field!("sheet", field_sheet, Text, |s, id| s.viewports.get(id).map(|row| row.sheet.clone()), choices: sheet_choices, parse_text => set_viewport::SetViewport),
    field!("view", field_view, Text, |s, id| s.viewports.get(id).map(|row| row.view.clone()), choices: view_choices, parse_text => set_viewport::SetViewport),
    field!("position", field_position, Text, |s, id| s.viewports.get(id).map(|row| point(row.position.x, row.position.y)), parse_point => set_viewport::SetViewport),
    field!("scale", field_scale, Number, |s, id| s.viewports.get(id).map(|row| row.scale.to_string()), parse_count => set_viewport::SetViewport),
    field!("crop", field_crop, Text, |s, id| s.viewports.get(id).map(crop_text), |_, id, value| set!(set_viewport::SetViewport, id, "crop", &Assigned::new(parse_crop(value)?))),
    field!("label", field_label, Text, |s, id| s.viewports.get(id).map(|row| row.label.clone().unwrap_or_default()), |_, id, value| set!(set_viewport::SetViewport, id, "label", &parse_label(value)?)),
];

/// 🧾️ The authored parameters of a revision row.
pub static REVISION_FIELDS: &[FieldRow] = &[
    field!("sheet", field_sheet, Text, |s, id| s.sheet_revisions.get(id).map(|row| row.sheet.clone())),
    field!("number", field_mark, Text, |s, id| s.sheet_revisions.get(id).map(|row| row.number.clone()), parse_text => set_sheet_revision::SetSheetRevision),
    field!("date", field_date, Text, |s, id| s.sheet_revisions.get(id).map(|row| row.date.clone()), parse_text => set_sheet_revision::SetSheetRevision),
    field!("description", field_description, Text, |s, id| s.sheet_revisions.get(id).map(|row| row.description.clone()), parse_text => set_sheet_revision::SetSheetRevision),
    field!("author", field_author, Text, |s, id| s.sheet_revisions.get(id).map(|row| row.author.clone()), parse_text => set_sheet_revision::SetSheetRevision),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
/// 💡️ What a sheet is now, read off the `sheet-layout` inference: its size on the paper, its scales, how many viewports it holds and how many findings its layout raised.
pub static SHEET_INFERRED: &[InferredRow] = &[
    inferred!("size", field_sheet_size, |s, inference, id| inference.sheet_layouts.get(id).filter(|_| s.sheets.contains_key(id)).map(|layout| window_text(layout.width, layout.height))),
    inferred!("scales", field_scales, |s, inference, id| inference.sheet_layouts.get(id).filter(|_| s.sheets.contains_key(id)).map(|layout| scale_text(&layout.viewports.iter().map(|placed| placed.scale).collect::<Vec<_>>()))),
    inferred!("viewports", field_viewports, |s, inference, id| inference.sheet_layouts.get(id).filter(|_| s.sheets.contains_key(id)).map(|layout| layout.viewports.len().to_string())),
    inferred!("findings", field_findings, |s, inference, id| inference.sheet_layouts.get(id).filter(|_| s.sheets.contains_key(id)).map(|layout| layout.findings.len().to_string())),
];

/// 💡️ What a viewport is now, read off the `sheet-layout` inference: the size of its window on the paper in millimetres.
pub static VIEWPORT_INFERRED: &[InferredRow] = &[inferred!("window", field_window, |s, inference, id| s.viewports.get(id).and_then(|row| inference.sheet_layouts.get(&row.sheet)).and_then(|layout| layout.viewports.iter().find(|placed| placed.viewport == id)).map(|placed| window_text(placed.window.width, placed.window.height)))];
//#endregion 🔖️Inferred

//#region 🔖️Create
/// 🔢️ The next free sheet number: `A-101`, `A-102`, and so on.
pub fn next_number(snapshot: &ModelSnapshot) -> String {
    (101..).map(|ordinal| format!("A-{ordinal}")).find(|candidate| snapshot.sheets.values().all(|row| &row.number != candidate)).unwrap_or_else(|| "A-1".to_string())
}

/// 📄️ A new sheet: A3 landscape, the next free number, named by the caller, with the project line and the author of the project in its title block.
pub fn create_sheet(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let sheet = Sheet { project: snapshot.project.name.clone(), drawn_by: snapshot.project.author.clone(), ..Sheet::standard(&next_number(snapshot), name) };
    Ok(ModelMutation::CreateSheet(crate::mutations::create_sheet::CreateSheet { id: id.into(), sheet }))
}

fn container(snapshot: &ModelSnapshot, parent: &str) -> Result<String, &'static str> {
    snapshot.sheets.contains_key(parent).then(|| parent.to_string()).ok_or("bim.create.sheet-missing")
}

/// 🖼️ The scale a new viewport of a view takes: the scale of the view when a viewport accepts it, else 1:100.
pub fn scale_of(snapshot: &ModelSnapshot, view: &str) -> u32 {
    snapshot.views.get(view).map(|row| row.scale).filter(|scale| (1..=MAX_VIEWPORT_SCALE).contains(scale)).unwrap_or(DEFAULT_VIEWPORT_SCALE)
}

/// 🖼️ The view a new viewport of `sheet` shows: a drawable view not on the sheet yet, else the first drawable view.
pub fn next_view(snapshot: &ModelSnapshot, sheet: &str) -> Option<String> {
    let placed: Vec<&String> = snapshot.viewports.values().filter(|row| row.sheet == sheet).map(|row| &row.view).collect();
    let mut drawable = snapshot.views.iter().filter(|(_, view)| view_is_drawn(view.kind)).map(|(id, _)| id);
    let first = drawable.clone().next().cloned();
    drawable.find(|id| !placed.contains(id)).cloned().or(first)
}

/// 🖼️ A new viewport of the sheet `parent`: the next drawable view at its scale, staggered by ten millimetres from the viewports already there.
pub fn create_viewport(snapshot: &ModelSnapshot, id: &str, parent: &str, _name: &str) -> Created {
    let sheet = container(snapshot, parent)?;
    let view = next_view(snapshot, &sheet).ok_or("bim.create.view-missing")?;
    let count = snapshot.viewports.values().filter(|row| row.sheet == sheet).count() as f64;
    let viewport = Viewport { scale: scale_of(snapshot, &view), ..Viewport::standard(&sheet, &view, Point2 { x: 30.0 + 10.0 * count, y: 30.0 + 10.0 * count }) };
    Ok(ModelMutation::CreateViewport(crate::mutations::create_viewport::CreateViewport { id: id.into(), viewport }))
}

/// 🧾️ The next revision mark of a sheet: the letter after the highest letter mark, `A` for the first.
pub fn next_mark(snapshot: &ModelSnapshot, sheet: &str) -> String {
    let marks: Vec<&String> = snapshot.sheet_revisions.values().filter(|row| row.sheet == sheet).map(|row| &row.number).collect();
    ('A'..='Z').map(String::from).find(|candidate| !marks.contains(&candidate)).unwrap_or_else(|| (marks.len() + 1).to_string())
}

/// 🧾️ A new revision row of the sheet `parent`: the next mark, described by the caller, signed by the author of the project.
pub fn create_revision(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let sheet = container(snapshot, parent)?;
    let revision = SheetRevision { number: next_mark(snapshot, &sheet), date: String::new(), description: name.into(), author: snapshot.project.author.clone(), sheet };
    Ok(ModelMutation::CreateSheetRevision(crate::mutations::create_sheet_revision::CreateSheetRevision { id: id.into(), sheet_revision: revision }))
}
//#endregion 🔖️Create

//#region 🔖️Kinds
/// 📄️ The entity row of sheets.
pub const SHEET: EntityKind = EntityKind {
    kind: "sheet",
    icon: "file-text",
    library: false,
    label: |labels| labels.kind_sheet,
    group: |labels| labels.group_sheets,
    ids: |snapshot| sheets::ordered(snapshot),
    name: |snapshot, id| snapshot.sheets.get(id).map(|row| row.name.clone()),
    parent: |_, _| None,
    delete: delete!(delete_sheet::DeleteSheet),
    rename: renaming!(set_sheet::SetSheet),
    create: Some(create_sheet),
    fields: SHEET_FIELDS,
    inferred: SHEET_INFERRED,
};

/// 🖼️ The entity row of viewports: named by their label, else by the name of their view.
pub const VIEWPORT: EntityKind = EntityKind {
    kind: "viewport",
    icon: "frame",
    library: false,
    label: |labels| labels.kind_viewport,
    group: |labels| labels.group_viewports,
    ids: |snapshot| snapshot.viewports.keys().cloned().collect(),
    name: |snapshot, id| snapshot.viewports.get(id).map(|row| row.label.clone().unwrap_or_else(|| snapshot.views.get(&row.view).map_or_else(|| row.view.clone(), |view| view.name.clone()))),
    parent: |snapshot, id| snapshot.viewports.get(id).map(|row| row.sheet.clone()),
    delete: delete!(delete_viewport::DeleteViewport),
    rename: None,
    create: Some(create_viewport),
    fields: VIEWPORT_FIELDS,
    inferred: VIEWPORT_INFERRED,
};

/// 🧾️ The entity row of revision rows: named by their mark and their description.
pub const REVISION: EntityKind = EntityKind {
    kind: "sheet-revision",
    icon: "history",
    library: false,
    label: |labels| labels.kind_sheet_revision,
    group: |labels| labels.group_sheet_revisions,
    ids: |snapshot| snapshot.sheet_revisions.keys().cloned().collect(),
    name: |snapshot, id| snapshot.sheet_revisions.get(id).map(|row| format!("{} {}", row.number, row.description)),
    parent: |snapshot, id| snapshot.sheet_revisions.get(id).map(|row| row.sheet.clone()),
    delete: delete!(delete_sheet_revision::DeleteSheetRevision),
    rename: None,
    create: Some(create_revision),
    fields: REVISION_FIELDS,
    inferred: &[],
};
//#endregion 🔖️Kinds

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
