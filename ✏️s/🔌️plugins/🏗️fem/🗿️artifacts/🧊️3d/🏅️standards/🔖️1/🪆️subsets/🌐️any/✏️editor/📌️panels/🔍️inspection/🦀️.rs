//! 🔍️ Fem3d play app panel — the inspection panel: real editable controls for whatever the
//! framework-owned `"fem3d"` selection resolves to (a node, element, solid, material, section,
//! support, load, load case or combination), or the document summary when nothing is selected.
//!
//! The body is a `ui::column` of `ui::section`s whose rows are `field(label) > control` — the shape
//! the React `Interpreter` renders as a labelled form row. A `Component::Tree` is NOT that shape:
//! its renderer maps sections to `TreeDataItem`s and drops every non-tree child, so a control nested
//! in a tree section reaches the host and is never drawn.
//!
//! Every control binds `Trigger::Change` to one `patch*` command with the argument map
//! `{field, id}`; the host merges the control's own value under `value`, so one flat
//! `{id, field, value}` payload covers every field of every entity kind and no per-field action has
//! to be declared. Read-only rows (ids, polygon sizes, analysis settings) are `field > ui::text` —
//! the analysis settings are edited from the results panel. Verbs (focus, delete, a pick row inside
//! a load case) are `ui::button`s bound with `Trigger::Activate`.

use crate::editor::fem3d::commands::patch_combination::{ADD_TERM_FIELD, REMOVE_TERM_FIELD, TERM_FIELD_PREFIX};
use crate::editor::fem3d::interaction::{
    fem3d_entity_kind, fem3d_load_owner, Fem3dInteractionSnapshot, FEM3D_GRANULARITY_COMBINATION, FEM3D_GRANULARITY_ELEMENT, FEM3D_GRANULARITY_LOAD, FEM3D_GRANULARITY_LOAD_CASE, FEM3D_GRANULARITY_MATERIAL, FEM3D_GRANULARITY_NODE,
    FEM3D_GRANULARITY_SECTION, FEM3D_GRANULARITY_SOLID, FEM3D_GRANULARITY_SUPPORT, FEM3D_INTERACTION_DOMAIN,
};
use crate::editor::fem3d::panels::artifact::dof_symbol;
use crate::editor::fem3d::terminology::Fem3dLabels;
use crate::editor::fem3d::{fem3d_action, ui_label};
use crate::Fem3dSnapshot;
use crate::{element_id, load_id, FemAxis, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind};
use semio_framework_plugin::{
    panel_page_rows, ActionId, BuiltNode, LocalizedLabel, PanelGroup, PanelRowBudget, PanelTabDefinition, PanelTabKind, PluginAssemblyError, Trigger, UiAssemblyResult, UiFixedList, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, INTERACTION_SELECT_ACTION_ID,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "fem3d.play.inspection";
const ROOT: &str = "fem3d-play-inspection";
/// 🔽️ Options one reference select offers before it stops listing — a document with more nodes than
/// this is re-pointed in the viewport, not from a dropdown, and `UiFixedList` admits 32 at most.
const SELECT_ITEMS_MAX: usize = 24;
/// 🧾️ Rows one nested listing (a case's loads, a combination's terms) materialises per render.
const LIST_ROWS_MAX: usize = 8;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Admission
fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "fem3d inspector admission failed")
}

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_error("ui.text"))
}

fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    ui_text(value).map(UiValue::Text)
}

fn ui_id<B: HasBase>(builder: B, id: impl AsRef<str>) -> UiAssemblyResult<B> {
    builder.try_id(id).map_err(|_| ui_error("ui.node.id"))
}

fn ui_build<B: Buildable>(builder: B) -> UiAssemblyResult<BuiltNode> {
    builder.try_build().map_err(|_| ui_error("ui.node.build"))
}

fn push(rows: &mut UiFixedList<BuiltNode>, node: UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<()> {
    rows.try_push(node?).map_err(|_| ui_error("ui.fixed-capacity"))
}
//#endregion 🔖️Admission

//#region 🔖️Layout
/// 🗂️ One collapsible group of form rows — `ui::section`, never a tree section.
fn section(id: &str, label: &str, rows: UiFixedList<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui_id(ui::section(ui_label(label)?), id)?.default_open(true);
    ui_build(builder.try_children(rows).map_err(|_| ui_error("ui.section.children"))?)
}

/// 📋️ The panel body: its sections stacked vertically.
fn column(sections: UiFixedList<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui_id(ui::column(), ROOT)?;
    ui_build(builder.try_children(sections).map_err(|_| ui_error("ui.column.children"))?)
}

/// 🧾️ One `field(label) > control` form row.
fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let row = ui_id(ui::field(ui_label(label)?), row_id)?;
    ui_build(row.try_child(control).map_err(|_| ui_error("ui.node.child"))?)
}

/// 📝️ One plain text node — a read-only value, or a truncated listing's continuation row.
fn text_node(id: &str, value: impl AsRef<str>) -> UiAssemblyResult<BuiltNode> {
    ui_build(ui_id(ui::text(ui_label(value)?), id)?)
}

/// 🔘️ One `Trigger::Activate` button — the verbs (focus, delete) and the pick rows of a nested listing.
fn action_button(id: &str, label: &str, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = action;
    let control = ui_id(ui::button(ui_label(label)?), id)?;
    let control = match args {
        Some(args) => control.try_on_with(Trigger::Activate, action, args).map_err(|_| ui_error("ui.control.binding"))?,
        None => control.try_on(Trigger::Activate, action).map_err(|_| ui_error("ui.control.binding"))?,
    };
    ui_build(control)
}

/// 🧾️ One nested listing's page: rows while both `LIST_ROWS_MAX` and the shared [`PanelRowBudget`]
/// last, closed by a text row naming what was left out.
fn paged_rows<T>(section_id: &str, entries: &[T], budget: &mut PanelRowBudget, mut row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    let mut placed = 0;
    for entry in entries {
        if placed == LIST_ROWS_MAX || !budget.spend() {
            break;
        }
        let node = match row(entry) {
            Ok(node) => node,
            Err(error) if error.code == "ui.fixed-capacity" => break,
            Err(error) => return Err(error),
        };
        if rows.try_push(node).is_err() {
            break;
        }
        placed += 1;
    }
    if placed < entries.len() {
        push(&mut rows, text_node(&format!("{section_id}.more"), format!("+{}", entries.len() - placed)))?;
    }
    Ok(rows)
}
//#endregion 🔖️Layout

//#region 🔖️Controls
/// 🎛️ The argument map every patch control carries: the edited field and the entity it addresses.
/// The control's own value arrives as `value` from the host, so it is deliberately absent here.
fn patch_args(field_name: &str, id: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    args.push("field".into(), ui_value_text(field_name)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    args.push("id".into(), ui_value_text(id)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    Ok(UiValue::Map(args.finish()))
}

fn bind<B: HasBase>(builder: B, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<B> {
    let (action, args) = fem3d_action(action, Some(patch_args(field_name, id)?))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| ui_error("ui.control.binding")),
        None => builder.try_on(Trigger::Change, action).map_err(|_| ui_error("ui.control.binding")),
    }
}

fn read_only_row(suffix: &str, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    control_row(&row_id, label, text_node(&format!("{row_id}.value"), value.to_string())?)
}

fn number_row(suffix: &str, label: &str, value: f64, step: f64, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::input(InputKind::Number).value(ui_text(format!("{value}"))?).step(step), format!("{row_id}.input"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn text_row(suffix: &str, label: &str, value: &str, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::input(InputKind::Text).value(ui_text(value)?), format!("{row_id}.input"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn slider_row(suffix: &str, label: &str, value: f64, bounds: (f64, f64, f64), action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let (minimum, maximum, step) = bounds;
    let control = ui_id(ui::slider(value).min(minimum).max(maximum).step(step), format!("{row_id}.slider"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn toggle_row(suffix: &str, label: &str, on: bool, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::toggle(on).text(ui_label(label)?), format!("{row_id}.toggle"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn select_row(suffix: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let mut control = ui_id(ui::select(ui_text(value)?), format!("{row_id}.select"))?;
    for (option, option_label) in options.into_iter().take(SELECT_ITEMS_MAX) {
        control = control.try_item(ui_text(option)?, ui_label(option_label)?).map_err(|_| ui_error("ui.select.item"))?;
    }
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}
//#endregion 🔖️Controls

//#region 🔖️Entities
fn node_options(doc: &Fem3dSnapshot) -> Vec<(String, String)> {
    doc.nodes.iter().map(|node| (node.id.clone(), node.id.clone())).collect()
}

fn element_options(doc: &Fem3dSnapshot) -> Vec<(String, String)> {
    doc.elements.iter().map(|element| (element_id(element).to_string(), element_id(element).to_string())).collect()
}

fn material_options(doc: &Fem3dSnapshot) -> Vec<(String, String)> {
    doc.materials.iter().map(|material| (material.id.clone(), material.name.clone())).collect()
}

fn section_options(doc: &Fem3dSnapshot) -> Vec<(String, String)> {
    doc.sections.iter().map(|section| (section.id.clone(), section.name.clone())).collect()
}

fn solid_options(doc: &Fem3dSnapshot) -> Vec<(String, String)> {
    doc.solids.iter().map(|solid| (solid.id.clone(), solid.name.clone())).collect()
}

fn axis_options() -> Vec<(String, String)> {
    FemAxis::ALL.iter().map(|axis| (axis.key().to_string(), axis.key().to_ascii_uppercase())).collect()
}
//#endregion 🔖️Entities

//#region 🔖️Sections
fn node_rows(node: &FemNode, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("node.id", labels.id.as_str(), &node.id))?;
    push(&mut rows, number_row("node.x", labels.x.as_str(), node.x, 0.1, "patchNode", "x", &node.id))?;
    push(&mut rows, number_row("node.y", labels.y.as_str(), node.y, 0.1, "patchNode", "y", &node.id))?;
    push(&mut rows, number_row("node.z", labels.z.as_str(), node.z, 0.1, "patchNode", "z", &node.id))?;
    Ok(rows)
}

fn element_rows(doc: &Fem3dSnapshot, element: &FemElement, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let (id, start, end, material_id, section_id, kind, roll) = match element {
        FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, "bar", None),
        FemElement::Frame { id, start, end, material_id, section_id, roll } => (id, start, end, material_id, section_id, "frame", Some(*roll)),
    };
    let kinds = vec![("bar".to_string(), labels.bar.as_str().to_string()), ("frame".to_string(), labels.frame.as_str().to_string())];
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("element.id", labels.id.as_str(), id))?;
    push(&mut rows, select_row("element.kind", labels.kind.as_str(), kind, kinds, "patchElement", "kind", id))?;
    push(&mut rows, select_row("element.start", labels.start.as_str(), start, node_options(doc), "patchElement", "start", id))?;
    push(&mut rows, select_row("element.end", labels.end.as_str(), end, node_options(doc), "patchElement", "end", id))?;
    push(&mut rows, select_row("element.material", labels.material.as_str(), material_id, material_options(doc), "patchElement", "materialId", id))?;
    push(&mut rows, select_row("element.section", labels.section.as_str(), section_id, section_options(doc), "patchElement", "sectionId", id))?;
    if let Some(roll) = roll {
        push(&mut rows, number_row("element.roll", labels.roll.as_str(), roll, 0.05, "patchElement", "roll", id))?;
    }
    Ok(rows)
}

fn material_rows(material: &FemMaterial, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("material.id", labels.id.as_str(), &material.id))?;
    push(&mut rows, text_row("material.name", labels.name.as_str(), &material.name, "patchMaterial", "name", &material.id))?;
    push(&mut rows, number_row("material.e", labels.youngs_modulus.as_str(), material.e, 1e9, "patchMaterial", "e", &material.id))?;
    push(&mut rows, number_row("material.g", labels.shear_modulus.as_str(), material.g, 1e9, "patchMaterial", "g", &material.id))?;
    push(&mut rows, slider_row("material.nu", labels.poisson_ratio.as_str(), material.nu, (0.0, 0.49, 0.01), "patchMaterial", "nu", &material.id))?;
    push(&mut rows, number_row("material.rho", labels.density.as_str(), material.rho, 10.0, "patchMaterial", "rho", &material.id))?;
    Ok(rows)
}

fn section_rows(section: &FemSection, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("section.id", labels.id.as_str(), &section.id))?;
    push(&mut rows, text_row("section.name", labels.name.as_str(), &section.name, "patchSection", "name", &section.id))?;
    push(&mut rows, number_row("section.area", labels.area.as_str(), section.area, 0.0001, "patchSection", "area", &section.id))?;
    push(&mut rows, number_row("section.iy", labels.second_moment_y.as_str(), section.iy, 0.000001, "patchSection", "iy", &section.id))?;
    push(&mut rows, number_row("section.iz", labels.second_moment_z.as_str(), section.iz, 0.000001, "patchSection", "iz", &section.id))?;
    push(&mut rows, number_row("section.j", labels.torsion_constant.as_str(), section.j, 0.000001, "patchSection", "j", &section.id))?;
    Ok(rows)
}

fn support_rows(doc: &Fem3dSnapshot, support: &FemSupport, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("support.id", labels.id.as_str(), &support.id))?;
    push(&mut rows, select_row("support.node", labels.node.as_str(), &support.node_id, node_options(doc), "patchSupport", "nodeId", &support.id))?;
    for (suffix, field_name, dof) in [("support.tx", "tx", FemDof::Tx), ("support.ty", "ty", FemDof::Ty), ("support.tz", "tz", FemDof::Tz), ("support.rx", "rx", FemDof::Rx), ("support.ry", "ry", FemDof::Ry), ("support.rz", "rz", FemDof::Rz)] {
        push(&mut rows, toggle_row(suffix, dof_symbol(dof), support.fixed.contains(&dof), "patchSupport", field_name, &support.id))?;
    }
    Ok(rows)
}

fn solid_rows(doc: &Fem3dSnapshot, solid: &FemSolid, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("solid.id", labels.id.as_str(), &solid.id))?;
    push(&mut rows, text_row("solid.name", labels.name.as_str(), &solid.name, "patchSolid", "name", &solid.id))?;
    push(&mut rows, select_row("solid.axis", labels.axis.as_str(), solid.axis.key(), axis_options(), "patchSolid", "axis", &solid.id))?;
    push(&mut rows, number_row("solid.base", labels.base.as_str(), solid.base_z, 0.1, "patchSolid", "baseZ", &solid.id))?;
    push(&mut rows, number_row("solid.height", labels.height.as_str(), solid.height, 0.1, "patchSolid", "height", &solid.id))?;
    push(&mut rows, number_row("solid.layers", labels.layers.as_str(), solid.layers as f64, 1.0, "patchSolid", "layers", &solid.id))?;
    push(&mut rows, number_row("solid.mesh-size", labels.mesh_size.as_str(), solid.mesh_size, 0.1, "patchSolid", "meshSize", &solid.id))?;
    push(&mut rows, select_row("solid.material", labels.material.as_str(), &solid.material_id, material_options(doc), "patchSolid", "materialId", &solid.id))?;
    push(&mut rows, read_only_row("solid.outline", labels.outline.as_str(), format!("{} {}", solid.outline.len(), labels.points.as_str())))?;
    push(&mut rows, read_only_row("solid.holes", labels.holes.as_str(), solid.holes.len()))?;
    Ok(rows)
}

fn load_rows(doc: &Fem3dSnapshot, load: &FemLoad, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = load_id(load);
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("load.id", labels.id.as_str(), id))?;
    let case = fem3d_load_owner(doc, id).and_then(|(case_id, _)| doc.load_cases.iter().find(|case| case.id == case_id));
    push(&mut rows, read_only_row("load.case", labels.load_case.as_str(), case.map_or_else(|| labels.none.as_str().to_string(), |case| case.name.clone())))?;
    match load {
        FemLoad::Nodal { node_id, dof, value, .. } => {
            let dofs = FemDof::ALL.iter().map(|entry| (dof_symbol(*entry).to_string(), dof_symbol(*entry).to_string())).collect();
            push(&mut rows, select_row("load.node", labels.node.as_str(), node_id, node_options(doc), "patchLoad", "nodeId", id))?;
            push(&mut rows, select_row("load.dof", labels.dof.as_str(), dof_symbol(*dof), dofs, "patchLoad", "dof", id))?;
            push(&mut rows, number_row("load.value", labels.value.as_str(), *value, 100.0, "patchLoad", "value", id))?;
        }
        FemLoad::MemberUdl { element_id: member, wx, wy, wz, .. } => {
            push(&mut rows, select_row("load.element", labels.element.as_str(), member, element_options(doc), "patchLoad", "elementId", id))?;
            push(&mut rows, number_row("load.wx", labels.wx.as_str(), *wx, 100.0, "patchLoad", "wx", id))?;
            push(&mut rows, number_row("load.wy", labels.wy.as_str(), *wy, 100.0, "patchLoad", "wy", id))?;
            push(&mut rows, number_row("load.wz", labels.wz.as_str(), *wz, 100.0, "patchLoad", "wz", id))?;
        }
        FemLoad::Area { solid_id, pressure, .. } => {
            push(&mut rows, select_row("load.solid", labels.solid.as_str(), solid_id, solid_options(doc), "patchLoad", "solidId", id))?;
            push(&mut rows, number_row("load.pressure", labels.pressure.as_str(), *pressure, 100.0, "patchLoad", "pressure", id))?;
        }
    }
    Ok(rows)
}

fn load_case_rows(case: &FemLoadCase, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("load-case.id", labels.id.as_str(), &case.id))?;
    push(&mut rows, text_row("load-case.name", labels.name.as_str(), &case.name, "patchLoadCase", "name", &case.id))?;
    push(&mut rows, toggle_row("load-case.self-weight", labels.self_weight.as_str(), case.self_weight, "patchLoadCase", "selfWeight", &case.id))?;
    Ok(rows)
}

/// 🕹️ A load button inside its case — activating it hands the load to the framework-owned `"fem3d"`
/// selection, the same `interactionSelect` a viewport pick or an artifact-tree row dispatches.
fn load_pick_row(load: &FemLoad, labels: &Fem3dLabels) -> UiAssemblyResult<BuiltNode> {
    let id = load_id(load);
    let (kind, detail) = match load {
        FemLoad::Nodal { node_id, dof, value, .. } => (labels.node.as_str(), format!("{node_id} {} {value}", dof_symbol(*dof))),
        FemLoad::MemberUdl { element_id: member, wx, wy, wz, .. } => (labels.element.as_str(), format!("{member} {wx} / {wy} / {wz}")),
        FemLoad::Area { solid_id, pressure, .. } => (labels.solid.as_str(), format!("{solid_id} {pressure}")),
    };
    action_button(id, &format!("{id} · {kind} · {detail}"), select_action(id)?)
}

fn select_action(id: &str) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    let target = protocol::InteractionTarget { granularity: FEM3D_GRANULARITY_LOAD.into(), id: id.into() };
    let targets = protocol::json::to_json_string(&protocol::DslValue::Array(vec![protocol::ToValue::to_value(&target)]));
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    for (key, value) in [("domainId", ui_value_text(FEM3D_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)] {
        args.push(key.into(), value).map_err(|_| ui_error("ui.value.map.entry"))?;
    }
    fem3d_action(INTERACTION_SELECT_ACTION_ID, Some(UiValue::Map(args.finish())))
}

fn combination_rows(combination: &FemCombination, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("combination.id", labels.id.as_str(), &combination.id))?;
    push(&mut rows, text_row("combination.name", labels.name.as_str(), &combination.name, "patchCombination", "name", &combination.id))?;
    Ok(rows)
}

/// ⚖️ One `term:<caseId>` factor input, labelled with the referenced case's own name so the row
/// reads as "Dead Load 1.35" rather than as an opaque id.
fn term_row(doc: &Fem3dSnapshot, combination: &FemCombination, case_id: &str, factor: f64) -> UiAssemblyResult<BuiltNode> {
    let name = doc.load_cases.iter().find(|case| case.id == case_id).map_or_else(|| case_id.to_string(), |case| case.name.clone());
    number_row(&format!("combination.term.{case_id}"), &name, factor, 0.05, "patchCombination", &format!("{TERM_FIELD_PREFIX}{case_id}"), &combination.id)
}

/// ➕️ The select that appends a term: its options are the cases this combination does NOT already
/// superpose, and the chosen id rides the control's own `value`.
fn add_term_row(doc: &Fem3dSnapshot, combination: &FemCombination, labels: &Fem3dLabels) -> UiAssemblyResult<Option<BuiltNode>> {
    let options: Vec<(String, String)> = doc.load_cases.iter().filter(|case| !combination.terms.contains_key(&case.id)).map(|case| (case.id.clone(), case.name.clone())).collect();
    if options.is_empty() {
        return Ok(None);
    }
    select_row("combination.add-term", labels.add_term.as_str(), "", options, "patchCombination", ADD_TERM_FIELD, &combination.id).map(Some)
}

/// ➖️ The select that drops a term: its options are the cases this combination superposes.
fn remove_term_row(doc: &Fem3dSnapshot, combination: &FemCombination, labels: &Fem3dLabels) -> UiAssemblyResult<Option<BuiltNode>> {
    let options: Vec<(String, String)> = combination.terms.keys().map(|case_id| (case_id.clone(), doc.load_cases.iter().find(|case| &case.id == case_id).map_or_else(|| case_id.clone(), |case| case.name.clone()))).collect();
    if options.is_empty() {
        return Ok(None);
    }
    select_row("combination.remove-term", &format!("{} {}", labels.delete.as_str(), labels.term.as_str()), "", options, "patchCombination", REMOVE_TERM_FIELD, &combination.id).map(Some)
}
//#endregion 🔖️Sections

//#region 🔖️Render
/// 📋️ The document summary shown when the `"fem3d"` domain selects nothing: what this document is
/// and how much of it there is, plus the analysis settings read-only.
fn summary(doc: &Fem3dSnapshot, labels: &Fem3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut counts = UiFixedList::default();
    push(&mut counts, read_only_row("summary.schema", labels.schema.as_str(), crate::FEM_3D_SCHEMA))?;
    for (suffix, label, count) in [
        ("summary.nodes", labels.nodes.as_str(), doc.nodes.len()),
        ("summary.elements", labels.elements.as_str(), doc.elements.len()),
        ("summary.solids", labels.solids.as_str(), doc.solids.len()),
        ("summary.materials", labels.materials.as_str(), doc.materials.len()),
        ("summary.sections", labels.sections.as_str(), doc.sections.len()),
        ("summary.supports", labels.supports.as_str(), doc.supports.len()),
        ("summary.load-cases", labels.load_cases.as_str(), doc.load_cases.len()),
        ("summary.combinations", labels.combinations.as_str(), doc.combinations.len()),
    ] {
        push(&mut counts, read_only_row(suffix, label, count))?;
    }
    let mut analysis = UiFixedList::default();
    push(&mut analysis, read_only_row("summary.modal-count", labels.modal_count.as_str(), doc.analysis.modal_count))?;
    push(&mut analysis, read_only_row("summary.buckling-count", labels.buckling_count.as_str(), doc.analysis.buckling_count))?;
    push(&mut analysis, read_only_row("summary.deformation-scale", labels.deformation_scale.as_str(), doc.analysis.deformation_scale))?;
    let mut sections = UiFixedList::default();
    push(&mut sections, section(&format!("{ROOT}.summary"), labels.summary.as_str(), counts))?;
    push(&mut sections, section(&format!("{ROOT}.analysis"), labels.analysis.as_str(), analysis))?;
    column(sections)
}

/// 🎯️ The selected entity's verbs — focus (entities with viewport geometry) and delete — as
/// `Trigger::Activate` buttons; they live in one group rather than on every row so a page of rows
/// costs one argument map each and the arena keeps credit for the panels rendered beside it.
fn action_rows(id: &str, kind: &str, labels: &Fem3dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    if matches!(kind, FEM3D_GRANULARITY_NODE | FEM3D_GRANULARITY_ELEMENT | FEM3D_GRANULARITY_SOLID | FEM3D_GRANULARITY_SUPPORT | FEM3D_GRANULARITY_LOAD) {
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
        args.push("id".into(), ui_value_text(id)?).map_err(|_| ui_error("ui.value.map.entry"))?;
        push(&mut rows, action_button(&format!("{ROOT}.actions.focus"), labels.focus.as_str(), fem3d_action("focusEntity", Some(UiValue::Map(args.finish())))?))?;
    }
    let mut ids = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| ui_error("ui.value.list"))?;
    ids.push(ui_value_text(id)?).map_err(|_| ui_error("ui.value.list.item"))?;
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    args.push("ids".into(), UiValue::List(ids.finish())).map_err(|_| ui_error("ui.value.map.entry"))?;
    push(&mut rows, action_button(&format!("{ROOT}.actions.delete"), labels.delete.as_str(), fem3d_action("removeSelection", Some(UiValue::Map(args.finish())))?))?;
    Ok(rows)
}

/// 🔢️ One text row per selected id — the fields below belong to the FIRST of them, which is the one
/// every viewport pick leaves at the head of the selection.
fn selection_rows(interaction: &Fem3dInteractionSnapshot) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    for (index, id) in interaction.selected_ids.iter().take(LIST_ROWS_MAX).enumerate() {
        push(&mut rows, text_node(&format!("{ROOT}.selection.{index}"), id))?;
    }
    Ok(rows)
}

/// 🔍️ The panel body: the first resolvable selected entity's editable fields, or the document
/// summary. Every group title is the entity's own noun, so the panel reads as "Node" / "Load Case"
/// rather than as a generic "Properties".
pub fn render(doc: &Fem3dSnapshot, interaction: &Fem3dInteractionSnapshot, labels: &Fem3dLabels) -> UiAssemblyResult<BuiltNode> {
    let Some((id, kind)) = interaction.selected_ids.iter().find_map(|id| fem3d_entity_kind(doc, id).map(|kind| (id.as_str(), kind))) else {
        return summary(doc, labels);
    };
    let mut sections = UiFixedList::default();
    if interaction.selected_ids.len() > 1 {
        let title = format!("{} {}", interaction.selected_ids.len(), labels.selected.as_str());
        push(&mut sections, section(&format!("{ROOT}.selection"), &title, selection_rows(interaction)?))?;
    }
    let mut budget = PanelRowBudget::new(panel_page_rows());
    match kind {
        FEM3D_GRANULARITY_NODE => {
            let node = doc.nodes.iter().find(|node| node.id == id).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.node"), labels.node.as_str(), node_rows(node, labels)?))?;
        }
        FEM3D_GRANULARITY_ELEMENT => {
            let element = doc.elements.iter().find(|element| element_id(element) == id).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.element"), labels.element.as_str(), element_rows(doc, element, labels)?))?;
        }
        FEM3D_GRANULARITY_SOLID => {
            let solid = doc.solids.iter().find(|solid| solid.id == id).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.solid"), labels.solid.as_str(), solid_rows(doc, solid, labels)?))?;
        }
        FEM3D_GRANULARITY_MATERIAL => {
            let material = doc.materials.iter().find(|material| material.id == id).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.material"), labels.material.as_str(), material_rows(material, labels)?))?;
        }
        FEM3D_GRANULARITY_SECTION => {
            let section_record = doc.sections.iter().find(|section| section.id == id).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.section"), labels.section.as_str(), section_rows(section_record, labels)?))?;
        }
        FEM3D_GRANULARITY_SUPPORT => {
            let support = doc.supports.iter().find(|support| support.id == id).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.support"), labels.support.as_str(), support_rows(doc, support, labels)?))?;
        }
        FEM3D_GRANULARITY_LOAD => {
            let load = doc.load_cases.iter().flat_map(|case| case.loads.iter()).find(|load| load_id(load) == id).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.load"), labels.load.as_str(), load_rows(doc, load, labels)?))?;
        }
        FEM3D_GRANULARITY_LOAD_CASE => {
            let case = doc.load_cases.iter().find(|case| case.id == id).ok_or_else(|| ui_error("ui.document"))?;
            let loads = paged_rows(&format!("{ROOT}.load-case.loads"), &case.loads, &mut budget, |load| load_pick_row(load, labels))?;
            push(&mut sections, section(&format!("{ROOT}.load-case"), labels.load_case.as_str(), load_case_rows(case, labels)?))?;
            push(&mut sections, section(&format!("{ROOT}.load-case.loads"), labels.loads.as_str(), loads))?;
        }
        FEM3D_GRANULARITY_COMBINATION => {
            let combination = doc.combinations.iter().find(|combination| combination.id == id).ok_or_else(|| ui_error("ui.document"))?;
            let entries: Vec<(String, f64)> = combination.terms.iter().map(|(case_id, factor)| (case_id.clone(), *factor)).collect();
            let mut terms = paged_rows(&format!("{ROOT}.combination.terms"), &entries, &mut budget, |(case_id, factor)| term_row(doc, combination, case_id, *factor))?;
            if let Some(row) = add_term_row(doc, combination, labels)? {
                terms.try_push(row).map_err(|_| ui_error("ui.fixed-capacity"))?;
            }
            if let Some(row) = remove_term_row(doc, combination, labels)? {
                terms.try_push(row).map_err(|_| ui_error("ui.fixed-capacity"))?;
            }
            push(&mut sections, section(&format!("{ROOT}.combination"), labels.combination.as_str(), combination_rows(combination, labels)?))?;
            push(&mut sections, section(&format!("{ROOT}.combination.terms"), labels.terms.as_str(), terms))?;
        }
        _ => return summary(doc, labels),
    }
    push(&mut sections, section(&format!("{ROOT}.actions"), labels.actions.as_str(), action_rows(id, kind, labels)?))?;
    column(sections)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
