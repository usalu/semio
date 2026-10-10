//! 🧬️ BIM family windows: the family editor and the family preview. A family is shown as one `Table` surface: its name and category, its parameters (name, formula input, the value the inference resolved, the
//! issue the inference found) with the buttons that add one of every kind, and its solids (name, shape, volume, then one formula input per slot of the shape) with the buttons that add one of every shape.
//! The preview draws the visible solids of the same family on a `World3d` surface. Both windows follow the selection: the family (or the solid, or the parameter row) selected in the library or the outliner
//! is the one shown, the first family of the model when nothing is selected. Nothing here stores or computes a value: every cell is read from the `families` inference, every edit is the `editFamily` command.

#[path = "✏️edit/🦀️.rs"]
pub mod edit;
#[path = "🏷️vocabulary/🦀️.rs"]
pub mod vocabulary;

use crate::editor::bim::kit::bim_window_action;
use crate::editor::bim::terminology::BimLabels;
use crate::render::world::{mesh_data, overview_orbit, world_bounds};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidGroup};
use crate::standards::v1::subsets::any::schema::inferences::families::issues::message;
use crate::standards::v1::subsets::any::schema::inferences::families::{FamilyIssue, FamilySolidMesh, FamilyValue, IssueOwner};
use crate::{ModelInference, ModelSnapshot, SolidShape};
use semio_framework_plugin::DslValue;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TableCell;
use semio_framework_plugin::UiTreeActionPlacement;
use semio_framework_plugin::UiTreeItemAction;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::{Label, LocalizedLabel};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-edit-family";
pub const BODY_KEY: &str = "bim.edit.family";
const SURFACE_ID: &str = "bim.edit.family/table";
pub const VIEW_KIND_ID: &str = "bim-edit-family-view";
pub const VIEW_BODY_KEY: &str = "bim.edit.family-view";
const VIEW_SURFACE_ID: &str = "bim.edit.family-view/world";
//#endregion 🔖️Constants

//#region 🔖️Definition
fn definition_of(id: &str, body: &str, label: LocalizedLabel, surface_kind: SurfaceKind, icon: &str) -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: id.into(),
        label,
        body_key: body.into(),
        surface_kind,
        icon_id: icon.into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🧱️ The family editor, stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    definition_of(WINDOW_KIND_ID, BODY_KEY, BimLabels::localized(|labels| labels.window_family), SurfaceKind::Table, "shapes")
}

/// 🧱️ The family preview, stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn view_definition() -> WindowKindDefinition {
    definition_of(VIEW_KIND_ID, VIEW_BODY_KEY, BimLabels::localized(|labels| labels.window_family_view), SurfaceKind::World3d, "box")
}
//#endregion 🔖️Definition

//#region 🔖️Selection
/// 🎯️ The family the windows show: the first family in the selection (a family, one of its solids), else the first family of the model.
pub fn selected(snapshot: &ModelSnapshot, elements: &[String], library: &[String]) -> Option<String> {
    library.iter().chain(elements).find_map(|id| snapshot.families.contains_key(id).then(|| id.clone()).or_else(|| snapshot.family_solids.get(id).map(|row| row.family.clone()))).or_else(|| snapshot.families.keys().next().cloned())
}
//#endregion 🔖️Selection

//#region 🔖️Cells
fn text(value: impl Into<String>) -> TableCell {
    TableCell::Text { value: value.into() }
}

fn object<'a>(entries: impl IntoIterator<Item = (&'a str, &'a str)>) -> DslValue {
    DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), DslValue::String(value.to_string()))))
}

fn args(family: &str, part: &str, op: &str, key: &str) -> DslValue {
    object([("id", family), ("part", part), ("op", op), ("key", key)])
}

fn input(family: &str, part: &str, op: &str, key: &str, value: &str) -> TableCell {
    TableCell::EditableText { value: value.to_string(), action: bim_window_action("editFamily", Some(args(family, part, op, key))) }
}

fn button(icon: &str, label: &str, family: &str, part: &str, op: &str, key: &str, value: &str) -> UiTreeItemAction {
    let strings = [("id", family), ("part", part), ("op", op), ("key", key), ("value", value)];
    UiTreeItemAction { icon_id: icon.into(), label: Some(Label::data(label.to_string())), action: bim_window_action("editFamily", Some(object(strings))), placement: Some(UiTreeActionPlacement::Row), disabled: false, reason: None }
}

fn row(id: &str, cells: Vec<(&str, TableCell)>) -> DslValue {
    let mut entries = vec![("id".to_string(), DslValue::String(id.to_string()))];
    entries.extend(cells.into_iter().map(|(column, cell)| (column.to_string(), semio_framework_value::ToValue::to_value(&cell))));
    DslValue::object(entries)
}

fn column_value(id: &str, label: &str) -> DslValue {
    DslValue::object([("id".to_string(), DslValue::String(id.into())), ("label".to_string(), DslValue::String(label.into())), ("sortable".to_string(), DslValue::Bool(false))])
}

fn table(columns: Vec<DslValue>, rows: Vec<DslValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let scene = semio_framework_plugin::TableScene::base(semio_framework_pack_json::to_json_string(&DslValue::Array(columns)), semio_framework_pack_json::to_json_string(&DslValue::Array(rows)));
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Table, &scene)
}
//#endregion 🔖️Cells

//#region 🔖️Rows
fn issue_text(issues: &[&FamilyIssue], labels: &BimLabels) -> String {
    issues.iter().map(|issue| message(issue, labels.locale().as_str())).collect::<Vec<_>>().join(" · ")
}

fn issues_of<'a>(value: &'a FamilyValue, owner: IssueOwner, subject: &str, field: Option<&str>) -> Vec<&'a FamilyIssue> {
    value.issues.iter().filter(|issue| issue.owner == owner && issue.subject == subject && field.is_none_or(|field| issue.field == field)).collect()
}

fn volume_text(solid: Option<&FamilySolidMesh>) -> String {
    solid.map_or_else(String::new, |solid| format!("{:.6} m³", solid.volume))
}

fn family_rows(snapshot: &ModelSnapshot, id: &str, value: &FamilyValue, labels: &BimLabels) -> Vec<DslValue> {
    let Some(family) = snapshot.families.get(id) else { return Vec::new() };
    let general: Vec<&FamilyIssue> = value.issues.iter().filter(|issue| issue.owner == IssueOwner::Family).collect();
    let categories = edit::CATEGORIES.iter().filter(|category| **category != family.category).map(|category| button("tag", &vocabulary::category_label(labels, *category), id, "family", "category", "", &format!("{category:?}"))).collect();
    vec![row("family", vec![("part", text(labels.fam_section_family.as_str())), ("item", text(labels.column_name.as_str())), ("formula", input(id, "family", "rename", "", &family.name)), ("value", text(vocabulary::category_label(labels, family.category))), ("issue", text(issue_text(&general, labels))), ("actions", TableCell::Buttons { buttons: categories })])]
}

fn parameter_rows(snapshot: &ModelSnapshot, id: &str, value: &FamilyValue, labels: &BimLabels) -> Vec<DslValue> {
    let mut rows: Vec<DslValue> = Vec::new();
    for parameter in crate::mutations::family_rules::parameters_in_order(snapshot, id) {
        let resolved = value.parameters.get(&parameter.name);
        let shown = resolved.and_then(|row| row.value.as_ref()).map_or_else(String::new, |found| vocabulary::value_text(labels, found));
        let next = edit::KINDS[(edit::KINDS.iter().position(|kind| *kind == parameter.kind).unwrap_or(0) + 1) % edit::KINDS.len()];
        let buttons = vec![
            button("refresh-cw", &BimLabels::named(labels.fam_act_kind_named, &vocabulary::kind_label(labels, next)), id, "parameter", "kind", &parameter.name, &format!("{next:?}")),
            button("trash", &BimLabels::named(labels.act_delete_named, &parameter.name), id, "parameter", "remove", &parameter.name, ""),
        ];
        rows.push(row(
            &format!("parameter:{}", parameter.name),
            vec![
                ("part", text(labels.fam_section_parameters.as_str())),
                ("item", text(format!("{} · {}", parameter.name, vocabulary::kind_label(labels, parameter.kind)))),
                ("formula", input(id, "parameter", "formula", &parameter.name, &parameter.value)),
                ("value", text(shown)),
                ("issue", text(issue_text(&issues_of(value, IssueOwner::Parameter, &parameter.name, None), labels))),
                ("actions", TableCell::Buttons { buttons }),
            ],
        ));
    }
    let adds = edit::KINDS.iter().map(|kind| button("plus", &BimLabels::named(labels.fam_act_add_parameter_named, &vocabulary::kind_label(labels, *kind)), id, "parameter", "add", &format!("{kind:?}"), "")).collect();
    rows.push(row("parameter:add", vec![("part", text(labels.fam_section_parameters.as_str())), ("item", text(labels.fam_add_parameter.as_str())), ("formula", text("")), ("value", text("")), ("issue", text("")), ("actions", TableCell::Buttons { buttons: adds })]));
    rows
}

fn solid_rows(snapshot: &ModelSnapshot, id: &str, value: &FamilyValue, labels: &BimLabels) -> Vec<DslValue> {
    let mut rows: Vec<DslValue> = Vec::new();
    for (solid_id, solid) in snapshot.family_solids.iter().filter(|(_, row)| row.family == id) {
        let mesh = value.solids.get(solid_id);
        let token = edit::shape_token(&solid.shape);
        let mut buttons = vec![button("trash", &BimLabels::named(labels.act_delete_named, &solid.name), id, "solid", "remove", solid_id, "")];
        if let SolidShape::Revolution { axis, .. } = &solid.shape {
            let next = edit::AXES[(edit::AXES.iter().position(|known| known == axis).unwrap_or(0) + 1) % edit::AXES.len()];
            buttons.push(button("rotate-3d", &BimLabels::named(labels.fam_act_axis_named, vocabulary::axis_label(next)), id, "solid", "axis", solid_id, vocabulary::axis_label(next)));
        }
        let slots = crate::standards::v1::subsets::any::schema::authored::formula::solid_slots(solid);
        let whole: Vec<&FamilyIssue> = value.issues.iter().filter(|issue| issue.owner == IssueOwner::Solid && issue.subject == *solid_id && !slots.iter().any(|(field, _)| *field == issue.field)).collect();
        let hidden = mesh.is_some_and(|mesh| !mesh.visible);
        let state = format!("{} · {}{}", vocabulary::shape_label(labels, token), volume_text(mesh), if hidden { format!(" · {}", labels.fam_hidden.as_str()) } else { String::new() });
        rows.push(row(
            &format!("solid:{solid_id}"),
            vec![
                ("part", text(labels.fam_section_solids.as_str())),
                ("item", input(id, "solid", "name", solid_id, &solid.name)),
                ("formula", text("")),
                ("value", text(state)),
                ("issue", text(issue_text(&whole, labels))),
                ("actions", TableCell::Buttons { buttons }),
            ],
        ));
        for (field, formula) in slots {
            let issues = issues_of(value, IssueOwner::Solid, solid_id, Some(&field));
            rows.push(row(&format!("slot:{solid_id}#{field}"), vec![("part", text("")), ("item", text(field.clone())), ("formula", input(id, "solid", "slot", &format!("{solid_id}#{field}"), formula)), ("value", text("")), ("issue", text(issue_text(&issues, labels))), ("actions", TableCell::Buttons { buttons: Vec::new() })]));
        }
    }
    let adds = edit::SHAPES.iter().map(|token| button("plus", &BimLabels::named(labels.fam_act_add_solid_named, &vocabulary::shape_label(labels, token)), id, "solid", "add", token, "")).collect();
    rows.push(row("solid:add", vec![("part", text(labels.fam_section_solids.as_str())), ("item", text(labels.fam_add_solid.as_str())), ("formula", text("")), ("value", text("")), ("issue", text("")), ("actions", TableCell::Buttons { buttons: adds })]));
    rows
}

/// 🧾️ The rows of the family editor for family `id`: the family, its parameters, its solids; each with the formula input, the inferred value and the inferred issue.
pub fn rows(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, labels: &BimLabels) -> Vec<DslValue> {
    let value = inference.families.get(id).cloned().unwrap_or_default();
    [family_rows(snapshot, id, &value, labels), parameter_rows(snapshot, id, &value, labels), solid_rows(snapshot, id, &value, labels)].concat()
}

/// 📋️ The columns of the family editor.
pub fn columns(labels: &BimLabels) -> Vec<DslValue> {
    vec![
        column_value("part", labels.fam_part.as_str()),
        column_value("item", labels.fam_item.as_str()),
        column_value("formula", labels.fam_formula.as_str()),
        column_value("value", labels.fam_value.as_str()),
        column_value("issue", labels.fam_issue.as_str()),
        column_value("actions", labels.sch_actions.as_str()),
    ]
}

/// 🧬️ Renders the family editor: the table of the selected family, or the hint to select one when the model has none.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, elements: &[String], library: &[String], labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(family) = selected(snapshot, elements, library) else {
        let hint = row("none", vec![("part", text(labels.fam_none.as_str())), ("item", text("")), ("formula", text("")), ("value", text("")), ("issue", text("")), ("actions", TableCell::Buttons { buttons: Vec::new() })]);
        return table(columns(labels), vec![hint]);
    };
    table(columns(labels), rows(snapshot, inference, &family, labels))
}
//#endregion 🔖️Rows

//#region 🔖️Preview
fn array(values: &[f64]) -> DslValue {
    DslValue::Array(values.iter().map(|value| DslValue::float(*value)).collect())
}

/// 🧱️ The element solid a family solid mesh draws as: one body group of its material.
pub fn element_solid(mesh: &FamilySolidMesh) -> ElementSolid {
    ElementSolid {
        groups: vec![SolidGroup { part: "body".into(), material: mesh.material.clone(), layer: 0 }],
        positions: mesh.positions.clone(),
        normals: mesh.normals.clone(),
        indices: mesh.indices.clone(),
        face_groups: vec![0; mesh.indices.len() / 3],
        bounds: mesh.bounds,
        volume: mesh.volume,
        area: mesh.area,
        ..ElementSolid::default()
    }
}

/// 🧊️ The visible solids of a family value as the element solids the world renderer draws, by solid id.
pub fn drawn(value: &FamilyValue) -> Vec<(String, ElementSolid)> {
    value.visible().filter(|(_, mesh)| !mesh.indices.is_empty()).map(|(id, mesh)| (id.clone(), element_solid(mesh))).collect()
}

/// 🎬️ The world scene of the family preview: the visible solids of family `id` as meshes and identity instances, framed once per family.
pub fn view_scene(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str) -> semio_framework_plugin::World3dScene {
    let solids = inference.families.get(id).map(drawn).unwrap_or_default();
    let meshes: Vec<DslValue> = solids.iter().map(|(solid_id, solid)| DslValue::object([("id".to_string(), DslValue::String(solid_id.clone())), ("data".to_string(), semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::from(mesh_data(snapshot, solid))))])).collect();
    let instances: Vec<DslValue> = solids
        .iter()
        .map(|(solid_id, _)| {
            DslValue::object([
                ("id".to_string(), DslValue::String(solid_id.clone())),
                ("meshId".to_string(), DslValue::String(solid_id.clone())),
                ("position".to_string(), array(&[0.0, 0.0, 0.0])),
                ("rotation".to_string(), array(&[0.0, 0.0, 0.0, 1.0])),
                ("scale".to_string(), array(&[1.0, 1.0, 1.0])),
                ("label".to_string(), DslValue::String(snapshot.family_solids.get(solid_id).map_or_else(|| solid_id.clone(), |row| row.name.clone()))),
                ("selected".to_string(), DslValue::Bool(false)),
                ("hovered".to_string(), DslValue::Bool(false)),
            ])
        })
        .collect();
    let camera = overview_orbit(world_bounds(solids.iter().map(|(_, solid)| solid)));
    let camera_json = semio_framework_plugin::world3d_camera_projection_json(camera.position, camera.target, camera.up, camera.zoom, &semio_framework_plugin::WorldProjectionConfig::default());
    let mut scene = semio_framework_plugin::world3d_scene(camera_json, semio_framework_pack_json::to_json_string(&meshes), semio_framework_pack_json::to_json_string(&instances), semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default());
    scene.fit_json = Some(semio_framework_pack_json::to_json_string(&DslValue::object([("enabled".to_string(), DslValue::Bool(true)), ("revision".to_string(), DslValue::float(f64::from(crate::render::plan::framing_revision(id)))), ("padding".to_string(), DslValue::float(1.25))])));
    scene
}

/// 🧊️ Renders the family preview: the selected family in 3D, an empty scene when the model has none.
pub fn view_render(snapshot: &ModelSnapshot, inference: &ModelInference, elements: &[String], library: &[String]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let id = selected(snapshot, elements, library).unwrap_or_default();
    semio_framework_plugin::scene_surface(VIEW_SURFACE_ID, semio_framework_ui_contract::SurfaceKind::World3d, &view_scene(snapshot, inference, &id))
}
//#endregion 🔖️Preview

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
