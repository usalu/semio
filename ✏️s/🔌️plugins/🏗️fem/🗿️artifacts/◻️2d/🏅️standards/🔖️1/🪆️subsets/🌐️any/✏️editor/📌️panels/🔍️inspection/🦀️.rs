//! 🔍️ Fem2d play app panel — the inspection panel: real editable controls for whatever the
//! framework-owned `"fem2d"` selection resolves to (a node, element, material, section, support,
//! region, load, load case or combination), or the document summary when nothing is selected.
//!
//! The body is a `Component::Tree` built through [`PanelTreeBuilder`]: each section holds
//! `treeItem` rows, and every editable control is the **child** of its row (`collectTreeItemControls`
//! in the React interpreter). Read-only facts use [`tree_item_desc`]; verbs and load picks use
//! [`tree_item_with_action`].
//!
//! Every control binds `Trigger::Change` to one `patch*` command with the argument map
//! `{field, id}`; the host merges the control's own value under `value`, so one flat
//! `{id, field, value}` payload covers every field of every entity kind and no per-field action has
//! to be declared. Read-only rows (ids, polygon sizes, analysis settings) are `field > ui::text` —
//! the analysis settings are edited from the results panel, which owns the `windowId`-tagged
//! `setAnalysisSettings` binding this panel has no window context for. Verbs (focus, delete, a pick
//! row inside a load case) are `ui::button`s bound with `Trigger::Activate`.
//!
//! 🪟️ The two listings that are as long as the document lets them be — a case's loads and a
//! combination's terms, plus the ids of a wide selection — are WINDOWED sections: the host names the
//! slice it is looking at and the section stamps the full extent beside it, so neither listing needs
//! a row cap nor a `+N` row standing in for the rest.

use crate::editor::fem2d::commands::patch_combination::{ADD_TERM_FIELD, TERM_FIELD_PREFIX};
use crate::editor::fem2d::interaction::{
    fem2d_entity_kind, fem2d_load_owner, Fem2dInteractionSnapshot, FEM2D_GRANULARITY_COMBINATION, FEM2D_GRANULARITY_ELEMENT, FEM2D_GRANULARITY_LOAD, FEM2D_GRANULARITY_LOAD_CASE, FEM2D_GRANULARITY_MATERIAL, FEM2D_GRANULARITY_NODE,
    FEM2D_GRANULARITY_REGION, FEM2D_GRANULARITY_SECTION, FEM2D_GRANULARITY_SUPPORT, FEM2D_INTERACTION_DOMAIN,
};
use crate::editor::fem2d::terminology::Fem2dLabels;
use crate::editor::fem2d::{fem2d_action, ui_label};
use crate::Fem2dSnapshot;
use crate::{element_id, load_id, FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind};
use semio_framework_plugin::{
    tree_item, tree_item_desc, tree_item_with_action, ActionId, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, Trigger, TreeWindows, UiAssemblyResult,
    UiFixedList, UiMapBuilder, UiText, UiValue, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, INTERACTION_SELECT_ACTION_ID,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "fem2d.play.inspection";
const ROOT: &str = "fem2d-play-inspection";
/// 🔽️ Options one reference select offers before it stops listing — a document with more nodes than
/// this is re-pointed in the viewport, not from a dropdown, and `UiFixedList` admits 32 at most.
const SELECT_ITEMS_MAX: usize = 24;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Admission
fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "fem2d inspector admission failed")
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
/// 🧾️ One labelled tree row whose inline control the interpreter mounts in `TreeDataItem.control`.
fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let row = ui_id(ui::tree_item(ui_label(label)?), row_id)?;
    ui_build(row.try_child(control).map_err(|_| ui_error("ui.node.child"))?)
}

/// 🔘️ One activatable tree row — focus, delete, and load-case pick rows.
fn action_button(id: &str, label: &str, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<BuiltNode> {
    tree_item_with_action(id, ui_label(label)?, None, action)
}

//#endregion 🔖️Layout

//#region 🔖️Controls
/// 🎛️ The argument map every patch control carries: the edited field and the entity it addresses.
/// `UiMapBuilder` sorts keys on admission; `field` < `id` is listed in that order for readability; the
/// control's own value arrives as `value` from the host, so it is deliberately absent here.
fn patch_args(field_name: &str, id: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    args.push("field".into(), ui_value_text(field_name)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    args.push("id".into(), ui_value_text(id)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    Ok(UiValue::Map(args.finish()))
}

fn bind<B: HasBase>(builder: B, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<B> {
    let (action, args) = fem2d_action(action, Some(patch_args(field_name, id)?))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| ui_error("ui.control.binding")),
        None => builder.try_on(Trigger::Change, action).map_err(|_| ui_error("ui.control.binding")),
    }
}

fn read_only_row(suffix: &str, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<BuiltNode> {
    tree_item_desc(format!("{ROOT}.{suffix}"), ui_label(label)?, Some(value.to_string()))
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
fn node_options(doc: &Fem2dSnapshot) -> Vec<(String, String)> {
    doc.nodes.iter().map(|node| (node.id.clone(), node.id.clone())).collect()
}

fn element_options(doc: &Fem2dSnapshot) -> Vec<(String, String)> {
    doc.elements.iter().map(|element| (element_id(element).to_string(), element_id(element).to_string())).collect()
}

fn material_options(doc: &Fem2dSnapshot) -> Vec<(String, String)> {
    doc.materials.iter().map(|material| (material.id.clone(), material.name.clone())).collect()
}

fn section_options(doc: &Fem2dSnapshot) -> Vec<(String, String)> {
    doc.sections.iter().map(|section| (section.id.clone(), section.name.clone())).collect()
}

fn region_options(doc: &Fem2dSnapshot) -> Vec<(String, String)> {
    doc.regions.iter().map(|region| (region.id.clone(), region.name.clone())).collect()
}

fn dof_key(dof: FemDof) -> &'static str {
    match dof {
        FemDof::Tx => "Tx",
        FemDof::Ty => "Ty",
        FemDof::Tz => "Tz",
        FemDof::Rx => "Rx",
        FemDof::Ry => "Ry",
        FemDof::Rz => "Rz",
    }
}
//#endregion 🔖️Entities

//#region 🔖️Sections
fn node_rows(node: &FemNode, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("node.id", labels.id.as_str(), &node.id))?;
    push(&mut rows, number_row("node.x", labels.x.as_str(), node.x, 0.1, "patchNode", "x", &node.id))?;
    push(&mut rows, number_row("node.y", labels.y.as_str(), node.y, 0.1, "patchNode", "y", &node.id))?;
    Ok(rows)
}

fn element_rows(doc: &Fem2dSnapshot, element: &FemElement, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let (id, start, end, material_id, section_id, kind) = match element {
        FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, "bar"),
        FemElement::Beam { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, "beam"),
    };
    let kinds = vec![("bar".to_string(), labels.bar.as_str().to_string()), ("beam".to_string(), labels.beam.as_str().to_string())];
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("element.id", labels.id.as_str(), id))?;
    push(&mut rows, select_row("element.kind", labels.kind.as_str(), kind, kinds, "patchElement", "kind", id))?;
    push(&mut rows, select_row("element.start", labels.start.as_str(), start, node_options(doc), "patchElement", "start", id))?;
    push(&mut rows, select_row("element.end", labels.end.as_str(), end, node_options(doc), "patchElement", "end", id))?;
    push(&mut rows, select_row("element.material", labels.material.as_str(), material_id, material_options(doc), "patchElement", "materialId", id))?;
    push(&mut rows, select_row("element.section", labels.section.as_str(), section_id, section_options(doc), "patchElement", "sectionId", id))?;
    Ok(rows)
}

fn material_rows(material: &FemMaterial, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("material.id", labels.id.as_str(), &material.id))?;
    push(&mut rows, text_row("material.name", labels.name.as_str(), &material.name, "patchMaterial", "name", &material.id))?;
    push(&mut rows, number_row("material.e", labels.youngs_modulus.as_str(), material.e, 1e9, "patchMaterial", "e", &material.id))?;
    push(&mut rows, slider_row("material.nu", labels.poisson_ratio.as_str(), material.nu, (0.0, 0.49, 0.01), "patchMaterial", "nu", &material.id))?;
    push(&mut rows, number_row("material.rho", labels.density.as_str(), material.rho, 10.0, "patchMaterial", "rho", &material.id))?;
    Ok(rows)
}

fn section_rows(section: &FemSection, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("section.id", labels.id.as_str(), &section.id))?;
    push(&mut rows, text_row("section.name", labels.name.as_str(), &section.name, "patchSection", "name", &section.id))?;
    push(&mut rows, number_row("section.area", labels.area.as_str(), section.area, 0.0001, "patchSection", "area", &section.id))?;
    push(&mut rows, number_row("section.iy", labels.second_moment_of_area.as_str(), section.iy, 0.000001, "patchSection", "iy", &section.id))?;
    Ok(rows)
}

fn support_rows(doc: &Fem2dSnapshot, support: &FemSupport, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("support.id", labels.id.as_str(), &support.id))?;
    push(&mut rows, select_row("support.node", labels.node.as_str(), &support.node_id, node_options(doc), "patchSupport", "nodeId", &support.id))?;
    for (suffix, label, field_name, dof) in [("support.tx", labels.tx.as_str(), "tx", FemDof::Tx), ("support.ty", labels.ty.as_str(), "ty", FemDof::Ty), ("support.rz", labels.rz.as_str(), "rz", FemDof::Rz)] {
        push(&mut rows, toggle_row(suffix, label, support.fixed.contains(&dof), "patchSupport", field_name, &support.id))?;
    }
    Ok(rows)
}

fn region_rows(doc: &Fem2dSnapshot, region: &FemRegion, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("region.id", labels.id.as_str(), &region.id))?;
    push(&mut rows, text_row("region.name", labels.name.as_str(), &region.name, "patchRegion", "name", &region.id))?;
    push(&mut rows, slider_row("region.thickness", labels.thickness.as_str(), region.thickness, (0.01, 2.0, 0.01), "patchRegion", "thickness", &region.id))?;
    push(&mut rows, number_row("region.mesh-size", labels.mesh_size.as_str(), region.mesh_size, 0.1, "patchRegion", "meshSize", &region.id))?;
    push(&mut rows, select_row("region.material", labels.material.as_str(), &region.material_id, material_options(doc), "patchRegion", "materialId", &region.id))?;
    push(&mut rows, read_only_row("region.outline", labels.outline.as_str(), format!("{} {}", region.outline.len(), labels.points.as_str())))?;
    push(&mut rows, read_only_row("region.holes", labels.holes.as_str(), region.holes.len()))?;
    Ok(rows)
}

fn load_rows(doc: &Fem2dSnapshot, load: &FemLoad, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = load_id(load);
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("load.id", labels.id.as_str(), id))?;
    let case = fem2d_load_owner(doc, id).and_then(|(case_id, _)| doc.load_cases.iter().find(|case| case.id == case_id));
    push(&mut rows, read_only_row("load.case", labels.load_case.as_str(), case.map_or_else(|| labels.none.as_str().to_string(), |case| case.name.clone())))?;
    match load {
        FemLoad::Nodal { node_id, dof, value, .. } => {
            let dofs = [FemDof::Tx, FemDof::Ty, FemDof::Rz].iter().map(|entry| (dof_key(*entry).to_string(), dof_key(*entry).to_string())).collect();
            push(&mut rows, select_row("load.node", labels.node.as_str(), node_id, node_options(doc), "patchLoad", "nodeId", id))?;
            push(&mut rows, select_row("load.dof", labels.dof.as_str(), dof_key(*dof), dofs, "patchLoad", "dof", id))?;
            push(&mut rows, number_row("load.value", labels.value.as_str(), *value, 100.0, "patchLoad", "value", id))?;
        }
        FemLoad::MemberUdl { element_id: member, wx, wy, .. } => {
            push(&mut rows, select_row("load.element", labels.element.as_str(), member, element_options(doc), "patchLoad", "elementId", id))?;
            push(&mut rows, number_row("load.wx", labels.wx.as_str(), *wx, 100.0, "patchLoad", "wx", id))?;
            push(&mut rows, number_row("load.wy", labels.wy.as_str(), *wy, 100.0, "patchLoad", "wy", id))?;
        }
        FemLoad::Area { region_id, pressure, .. } => {
            push(&mut rows, select_row("load.region", labels.region.as_str(), region_id, region_options(doc), "patchLoad", "regionId", id))?;
            push(&mut rows, number_row("load.pressure", labels.pressure.as_str(), *pressure, 100.0, "patchLoad", "pressure", id))?;
        }
    }
    Ok(rows)
}

fn load_case_rows(case: &FemLoadCase, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("load-case.id", labels.id.as_str(), &case.id))?;
    push(&mut rows, text_row("load-case.name", labels.name.as_str(), &case.name, "patchLoadCase", "name", &case.id))?;
    push(&mut rows, toggle_row("load-case.self-weight", labels.self_weight.as_str(), case.self_weight, "patchLoadCase", "selfWeight", &case.id))?;
    Ok(rows)
}

/// 🕹️ A load button inside its case — activating it hands the load to the framework-owned `"fem2d"`
/// selection, the same `interactionSelect` a viewport pick or an artifact-tree row dispatches, so
/// the inspector re-renders on that load instead of inventing a second selection channel. Its node
/// id is the load's own id, which is what a scripted driver and the DOM address it by.
fn load_pick_row(load: &FemLoad, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    let id = load_id(load);
    let (kind, detail) = match load {
        FemLoad::Nodal { node_id, dof, value, .. } => (labels.node.as_str(), format!("{node_id} {} {value}", dof_key(*dof))),
        FemLoad::MemberUdl { element_id: member, wx, wy, .. } => (labels.element.as_str(), format!("{member} {wx} / {wy}")),
        FemLoad::Area { region_id, pressure, .. } => (labels.region.as_str(), format!("{region_id} {pressure}")),
    };
    action_button(id, &format!("{id} · {kind} · {detail}"), select_action(id)?)
}

fn select_action(id: &str) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    let target = protocol::InteractionTarget { granularity: FEM2D_GRANULARITY_LOAD.into(), id: id.into() };
    let targets = protocol::json::to_json_string(&protocol::DslValue::Array(vec![protocol::ToValue::to_value(&target)]));
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    for (key, value) in [("domainId", ui_value_text(FEM2D_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)] {
        args.push(key.into(), value).map_err(|_| ui_error("ui.value.map.entry"))?;
    }
    fem2d_action(INTERACTION_SELECT_ACTION_ID, Some(UiValue::Map(args.finish())))
}

/// ⚖️ The combination's own fields, and the term verb beside them: the `Terms` section is a WINDOW
/// over the terms alone, so a control that is not a term cannot live inside it without claiming a
/// slot in that window's extent.
fn combination_rows(doc: &Fem2dSnapshot, combination: &FemCombination, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("combination.id", labels.id.as_str(), &combination.id))?;
    push(&mut rows, text_row("combination.name", labels.name.as_str(), &combination.name, "patchCombination", "name", &combination.id))?;
    if let Some(row) = add_term_row(doc, combination, labels)? {
        rows.try_push(row).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    Ok(rows)
}

/// ⚖️ One `term:<caseId>` factor input, labelled with the referenced case's own name so the row
/// reads as "Dead Load 1.35" rather than as an opaque id.
fn term_row(doc: &Fem2dSnapshot, combination: &FemCombination, term: &FemCombinationTerm) -> UiAssemblyResult<BuiltNode> {
    let name = doc.load_cases.iter().find(|case| case.id == term.case_id).map_or_else(|| term.case_id.clone(), |case| case.name.clone());
    number_row(&format!("combination.term.{}", term.case_id), &name, term.factor, 0.05, "patchCombination", &format!("{TERM_FIELD_PREFIX}{}", term.case_id), &combination.id)
}

/// ➕️ The select that appends a term: its options are the cases this combination does NOT already
/// superpose, and the chosen id rides the control's own `value`, which is why the bound field is the
/// constant `addTerm` rather than a per-case `term:<caseId>`.
fn add_term_row(doc: &Fem2dSnapshot, combination: &FemCombination, labels: &Fem2dLabels) -> UiAssemblyResult<Option<BuiltNode>> {
    let options: Vec<(String, String)> = doc.load_cases.iter().filter(|case| !combination.terms.iter().any(|term| term.case_id == case.id)).map(|case| (case.id.clone(), case.name.clone())).collect();
    if options.is_empty() {
        return Ok(None);
    }
    select_row("combination.add-term", labels.add_term.as_str(), "", options, "patchCombination", ADD_TERM_FIELD, &combination.id).map(Some)
}
//#endregion 🔖️Sections

//#region 🔖️Render
/// 📋️ The document summary shown when the `"fem2d"` domain selects nothing: what this document is
/// and how much of it there is, plus the analysis settings read-only.
fn summary(doc: &Fem2dSnapshot, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut counts = UiFixedList::default();
    push(&mut counts, read_only_row("summary.schema", labels.schema.as_str(), crate::FEM_2D_SCHEMA))?;
    for (suffix, label, count) in [
        ("summary.nodes", labels.nodes.as_str(), doc.nodes.len()),
        ("summary.elements", labels.elements.as_str(), doc.elements.len()),
        ("summary.regions", labels.regions.as_str(), doc.regions.len()),
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
    PanelTreeBuilder::new(ROOT)?
        .section(format!("{ROOT}.summary"), Some(ui_label(labels.summary.as_str())?), true, counts)?
        .section(format!("{ROOT}.analysis"), Some(ui_label(labels.analysis.as_str())?), true, analysis)?
        .build()
}

/// 🎯️ The selected entity's verbs — focus (entities with viewport geometry) and delete — as
/// `Trigger::Activate` buttons; they live in one group rather than on every row so a page of rows
/// costs one argument map each and the arena keeps credit for the panels rendered beside it.
fn action_rows(id: &str, kind: &str, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    if matches!(kind, FEM2D_GRANULARITY_NODE | FEM2D_GRANULARITY_ELEMENT | FEM2D_GRANULARITY_REGION | FEM2D_GRANULARITY_SUPPORT | FEM2D_GRANULARITY_LOAD) {
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
        args.push("id".into(), ui_value_text(id)?).map_err(|_| ui_error("ui.value.map.entry"))?;
        push(&mut rows, action_button(&format!("{ROOT}.actions.focus"), labels.focus.as_str(), fem2d_action("focusEntity", Some(UiValue::Map(args.finish())))?))?;
    }
    let mut ids = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| ui_error("ui.value.list"))?;
    ids.push(ui_value_text(id)?).map_err(|_| ui_error("ui.value.list.item"))?;
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    args.push("ids".into(), UiValue::List(ids.finish())).map_err(|_| ui_error("ui.value.map.entry"))?;
    push(&mut rows, action_button(&format!("{ROOT}.actions.delete"), labels.delete.as_str(), fem2d_action("removeSelection", Some(UiValue::Map(args.finish())))?))?;
    Ok(rows)
}

/// 🔢️ One text row per selected id, keyed by the id itself so a window over a wide selection names
/// the same row at any offset — the fields below belong to the FIRST of them, which is the one every
/// viewport pick leaves at the head of the selection.
fn selection_row(id: &str) -> UiAssemblyResult<BuiltNode> {
    tree_item(format!("{ROOT}.selection.{id}"), ui_label(id)?)
}

/// 🔍️ The panel body: the first resolvable selected entity's editable fields, or the document
/// summary. Every group title is the entity's own noun, so the panel reads as "Node" / "Load Case"
/// rather than as a generic "Properties".
pub fn render(doc: &Fem2dSnapshot, interaction: &Fem2dInteractionSnapshot, labels: &Fem2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let Some((id, kind)) = interaction.selected_ids.iter().find_map(|id| fem2d_entity_kind(doc, id).map(|kind| (id.as_str(), kind))) else {
        return summary(doc, labels);
    };
    let mut builder = PanelTreeBuilder::new(ROOT)?;
    if interaction.selected_ids.len() > 1 {
        let title = format!("{} {}", interaction.selected_ids.len(), labels.selected.as_str());
        builder = builder.window_section(windows, &format!("{ROOT}.selection"), Some(ui_label(&title)?), true, &interaction.selected_ids, |id| selection_row(id))?;
    }
    builder = match kind {
        FEM2D_GRANULARITY_NODE => {
            let node = doc.nodes.iter().find(|node| node.id == id).ok_or_else(|| ui_error("ui.document"))?;
            builder.section(format!("{ROOT}.node"), Some(ui_label(labels.node.as_str())?), true, node_rows(node, labels)?)?
        }
        FEM2D_GRANULARITY_ELEMENT => {
            let element = doc.elements.iter().find(|element| element_id(element) == id).ok_or_else(|| ui_error("ui.document"))?;
            builder.section(format!("{ROOT}.element"), Some(ui_label(labels.element.as_str())?), true, element_rows(doc, element, labels)?)?
        }
        FEM2D_GRANULARITY_MATERIAL => {
            let material = doc.materials.iter().find(|material| material.id == id).ok_or_else(|| ui_error("ui.document"))?;
            builder.section(format!("{ROOT}.material"), Some(ui_label(labels.material.as_str())?), true, material_rows(material, labels)?)?
        }
        FEM2D_GRANULARITY_SECTION => {
            let section_record = doc.sections.iter().find(|section| section.id == id).ok_or_else(|| ui_error("ui.document"))?;
            builder.section(format!("{ROOT}.section"), Some(ui_label(labels.section.as_str())?), true, section_rows(section_record, labels)?)?
        }
        FEM2D_GRANULARITY_SUPPORT => {
            let support = doc.supports.iter().find(|support| support.id == id).ok_or_else(|| ui_error("ui.document"))?;
            builder.section(format!("{ROOT}.support"), Some(ui_label(labels.support.as_str())?), true, support_rows(doc, support, labels)?)?
        }
        FEM2D_GRANULARITY_REGION => {
            let region = doc.regions.iter().find(|region| region.id == id).ok_or_else(|| ui_error("ui.document"))?;
            builder.section(format!("{ROOT}.region"), Some(ui_label(labels.region.as_str())?), true, region_rows(doc, region, labels)?)?
        }
        FEM2D_GRANULARITY_LOAD => {
            let load = doc.load_cases.iter().flat_map(|case| case.loads.iter()).find(|load| load_id(load) == id).ok_or_else(|| ui_error("ui.document"))?;
            builder.section(format!("{ROOT}.load"), Some(ui_label(labels.load.as_str())?), true, load_rows(doc, load, labels)?)?
        }
        FEM2D_GRANULARITY_LOAD_CASE => {
            let case = doc.load_cases.iter().find(|case| case.id == id).ok_or_else(|| ui_error("ui.document"))?;
            builder
                .section(format!("{ROOT}.load-case"), Some(ui_label(labels.load_case.as_str())?), true, load_case_rows(case, labels)?)?
                .window_section(windows, &format!("{ROOT}.load-case.loads"), Some(ui_label(labels.loads.as_str())?), true, &case.loads, |load| load_pick_row(load, labels))?
        }
        FEM2D_GRANULARITY_COMBINATION => {
            let combination = doc.combinations.iter().find(|combination| combination.id == id).ok_or_else(|| ui_error("ui.document"))?;
            builder
                .section(format!("{ROOT}.combination"), Some(ui_label(labels.combination.as_str())?), true, combination_rows(doc, combination, labels)?)?
                .window_section(windows, &format!("{ROOT}.combination.terms"), Some(ui_label(labels.terms.as_str())?), true, &combination.terms, |term| term_row(doc, combination, term))?
        }
        _ => return summary(doc, labels),
    };
    builder.section(format!("{ROOT}.actions"), Some(ui_label(labels.actions.as_str())?), true, action_rows(id, kind, labels)?)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
