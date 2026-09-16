//! 📄️ CAD play app panel — the document tree: every pane's objects (with their primitive children)
//! and reference overlays, plus the scene's nodes.

use crate::editor::cad::modes::edit;
use crate::editor::cad::terminology::{typology_label, CadLabels};
use crate::editor::cad::{cad_action, cad_tree_item, cad_tree_item_static, ui_label, ui_node_list, ui_value_bool, ui_value_list, ui_value_map, ui_value_number, ui_value_text, CadPlayRuntime, CadPlayView, CAD_INTERACTION_DOMAIN};
use crate::standards::v1::subsets::any::io::geometry_import::CadObject;
use crate::standards::v1::subsets::any::schema::inferences::{CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC};
use crate::{CadPaneId, CadReference, CadSnapshot};
use std::collections::BTreeMap;
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, BuiltNode, Label as UiLabel, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    panel_continuation_row, panel_page_rows, LabelText, LocalizedLabel, PanelGroup, PanelRowBudget, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID,
};

//#region 🔖️Constants
pub const CAD_PLAY_BODY_ARTIFACT: &str = "cad.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(CAD_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ A tree-row pick in the `"cad"` domain — the same `interactionSelect` `World3dHost` dispatches
/// for a viewport pick, so a row click and a mesh click land in the one framework-owned selection.
fn select_object_action(object_id: &str) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<UiValue>)> {
    let target = protocol::InteractionTarget { granularity: edit::CAD_WORLD_PICK_GRANULARITY.into(), id: object_id.into() };
    let targets = protocol::json::to_json_string(&protocol::DslValue::Array(vec![protocol::ToValue::to_value(&target)]));
    let args = ui_value_map([("domainId", ui_value_text(CAD_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)])?;
    semio_framework_plugin::ActionFactory::new(crate::editor::cad::CAD_PLAY_CONTROLLER_ID).action(INTERACTION_SELECT_ACTION_ID, Some(args))
}

/// 🌳️ One object's row. Its id is the object's RAW id — the tree is bound to the `"cad"` domain, so
/// the framework marks the selected/hovered rows from the domain state by that id (a composite
/// `cad-object:…` id would never match). Primitive children keep their composite ids: they are not
/// domain targets at `object` granularity.
///
/// 🪙️ The row authors ONE argument map (its pick). The former hide/lock/duplicate/delete row actions
/// dispatched `patchObject`/`duplicateObject`/`deleteObject`, whose handlers are documented no-ops
/// until the composed-child dispatch seam lands (`🎮️commands/🧱️object`), and their four extra maps
/// per row are what pushed a 24-object document past the one-page `UiValue` argument arena
/// (`ui.fixed-capacity: cad UI map admission failed` on every later refresh).
pub(crate) fn object_tree_item(id_suffix: &str, object: &CadObject, labels: &CadLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let primitive_items = ui_node_list(object.primitives.iter().map(|primitive| {
        cad_tree_item_static(format!("cad-primitive:{id_suffix}:{}:{}", object.id, primitive.primitive_id), format!("{}: {}", primitive.slot, primitive.primitive_id), Some("hexagon"))
    }))?;
    let mut item = cad_tree_item(object.id.clone(), &object.label, Some("box"), select_object_action(&object.id)?)?;
    for primitive in primitive_items {
        item.children.try_push(primitive).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad primitive child admission failed"))?;
    }
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        if !object.typology.is_empty() {
            props.description = Some(UiText::try_from_string(typology_label(&object.typology, labels).to_string()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad object description admission failed"))?);
        }
        props.dimmed = Some(!object.visible);
        props.draggable = Some(!object.locked);
        props.default_open = Some(false);
    }
    Ok(item)
}

pub fn reference_tree_item(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let select_args = ui_value_map([("modelDefinitionId", ui_value_text(model_definition_id)?), ("referenceId", ui_value_text(&reference.id)?)])?;
    let mut item = cad_tree_item(format!("cad-reference:{model_definition_id}:{}", reference.id), &reference.id, Some("image"), cad_action("setReferenceSelection", Some(select_args))?)?;
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `UiTreeItemNode` no longer carries
    // `hoverAction`/`unhoverAction` — no generic tree-hover mechanism replaces it for a non-
    // `interaction_domain`-bound tree; `referenceHover` stays reachable from the World3d surface only.
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.description = Some(UiText::try_from_str(&reference.source_url).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad reference description admission failed"))?);
        props.dimmed = Some(reference.hidden);
        let specs = [
            (if reference.hidden { "eye" } else { "eye-off" }, if reference.hidden { labels.show } else { labels.hide }, "hidden", !reference.hidden),
            (if reference.locked { "unlock" } else { "lock" }, if reference.locked { labels.unlock } else { labels.lock }, "locked", !reference.locked),
        ];
        let mut row_actions = UiFixedList::default();
        for (icon, label, field, value) in specs {
            // 🔑️ `UiMapBuilder::push` admits keys in strictly ascending order only — `field` before
            // `modelDefinitionId` before `referenceId` before `value`.
            let args = ui_value_map([("field", ui_value_text(field)?), ("modelDefinitionId", ui_value_text(model_definition_id)?), ("referenceId", ui_value_text(&reference.id)?), ("value", ui_value_bool(value))])?;
            let (action, args) = cad_action("patchCadPlayReference", Some(args))?;
            let row_action = RowAction {
                icon: UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad reference row action icon admission failed"))?,
                label: Some(ui_label(label.as_str())?),
                action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
                placement: RowActionPlacement::Row,
            };
            row_actions.try_push(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad reference row action admission failed"))?;
        }
        props.row_actions = row_actions;
    }
    Ok(item)
}

/// 🗂️ The `document.references_by_model_definition_id` lookup repeated once per pane in `build_document_tree`.
pub fn references_for<'a>(document: &'a CadSnapshot, model_definition_id: &str) -> &'a [CadReference] {
    document.references_by_model_definition_id.get(model_definition_id).map_or(&[][..], |rows| rows.as_slice())
}

/// 🕹️ Only the app-owned reference-overlay selection is resolved here — mesh object selection is
/// the framework-owned `"cad"` domain the built tree is bound to (`PanelTreeBuilder::interaction_domain`),
/// which marks object rows by their raw ids itself.
pub fn document_tree_selected_ids(_document: &CadSnapshot, runtime: &CadPlayRuntime) -> semio_framework_plugin::UiAssemblyResult<Option<UiFixedList<String>>> {
    if let (Some(model_definition_id), Some(reference_id)) = (runtime.selected_reference_model_definition_id.as_deref(), runtime.selected_reference_id.as_deref()) {
        let mut ids = UiFixedList::default();
        ids.try_push(format!("cad-reference:{model_definition_id}:{reference_id}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad selected id admission failed"))?;
        return Ok(Some(ids));
    }
    Ok(None)
}

pub fn document_tree_highlighted_ids(document: &CadSnapshot, runtime: &CadPlayRuntime) -> semio_framework_plugin::UiAssemblyResult<Option<UiFixedList<String>>> {
    let Some(hovered) = runtime.hovered_reference_id.as_deref() else {
        return Ok(None);
    };
    for pane in CadPaneId::all() {
        let model_definition_id = pane.model_definition_id();
        if document.references_by_model_definition_id.get(model_definition_id).is_some_and(|rows| rows.iter().any(|row| row.id == hovered)) {
            let mut ids = UiFixedList::default();
            ids.try_push(format!("cad-reference:{model_definition_id}:{hovered}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad highlighted id admission failed"))?;
            return Ok(Some(ids));
        }
    }
    Ok(None)
}

/// 🧾️ Sections this tree assembles, in order — four pane object sections, their references, the nodes.
const SECTIONS: usize = 9;

/// 📄️ Rows one section shows before it closes with a continuation — the tree holds nine sections on
/// one shared argument-arena page, so a section that spent the whole page would leave its siblings
/// with none.
pub const CAD_SECTION_ROWS: usize = 6;

/// 📄️ The page a section is parked on, clamped to the last page its own entry count actually has —
/// a stale cursor (the document shrank under it) reads as the last real page, never an empty section.
pub fn section_page(pages: &BTreeMap<String, u32>, section_id: &str, entries: usize) -> usize {
    if entries == 0 {
        return 0;
    }
    let last = (entries - 1) / CAD_SECTION_ROWS;
    (pages.get(section_id).copied().unwrap_or(0) as usize).min(last)
}

/// ➕️ The continuation row closing a truncated section. When a next page exists it carries a
/// `setPanelPage` argument map and is therefore clickable; when the argument arena has no credit left
/// for that map it degrades to the SDK's plain `+N` row rather than vanishing — a section showing
/// neither its rows nor the `+N` that explains why reads as an empty document.
pub fn continuation_row(section_id: &str, omitted: usize, next_page: Option<u32>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let Some(page) = next_page else {
        return panel_continuation_row(section_id, omitted);
    };
    let args = ui_value_map([("page", ui_value_number(f64::from(page))), ("section", ui_value_text(section_id)?)]);
    let built = args.and_then(|args| cad_action("setPanelPage", Some(args))).and_then(|action| cad_tree_item(format!("{section_id}.more"), format!("+{omitted}"), Some("more-horizontal"), action));
    match built {
        Ok(item) => Ok(item),
        Err(_) => panel_continuation_row(section_id, omitted),
    }
}

/// 🗂️ `semio_framework_plugin::paged_panel_section` with a cursor: the section starts at its
/// `setPanelPage` page instead of always at entry zero, and its continuation row advances that page.
/// Everything else (the shared [`PanelRowBudget`], the capacity-refusal fallback) is the SDK's.
pub fn paged_section_from<T>(
    section_id: &str,
    entries: &[T],
    pages: &BTreeMap<String, u32>,
    budget: &mut PanelRowBudget,
    mut row: impl FnMut(&T, &mut PanelRowBudget) -> semio_framework_plugin::UiAssemblyResult<BuiltNode>,
) -> semio_framework_plugin::UiAssemblyResult<UiFixedList<BuiltNode>> {
    let page = section_page(pages, section_id, entries.len());
    let offset = page.saturating_mul(CAD_SECTION_ROWS).min(entries.len());
    let slice = &entries[offset..];
    let mut items = UiFixedList::<BuiltNode>::default();
    let quota = slice.len().min(CAD_SECTION_ROWS);
    let truncated = slice.len() > CAD_SECTION_ROWS;
    let mut placed = 0;
    for entry in slice {
        if placed == CAD_SECTION_ROWS || (truncated && budget.remaining() <= 1) || !budget.spend() {
            break;
        }
        placed += 1;
        match budget.nested(quota - placed, |nested| row(entry, nested)) {
            Ok(node) => {
                if items.try_push(node).is_err() {
                    placed -= 1;
                    break;
                }
            }
            Err(error) if error.code == "ui.fixed-capacity" => {
                placed -= 1;
                break;
            }
            Err(error) => return Err(error),
        }
    }
    if placed < slice.len() && budget.spend() {
        if let Ok(more) = continuation_row(section_id, slice.len() - placed, Some(page as u32 + 1)) {
            let _ = items.try_push(more);
        }
    }
    Ok(items)
}

/// 🌳️ One pane's object section: namespaced by `id_suffix`, always expanded, paged against `budget`
/// from its own `setPanelPage` cursor so a document past one page closes with a clickable `+N`
/// continuation row instead of failing an admission mid-row.
pub(crate) fn document_pane_section(
    label: LabelText,
    id_suffix: &str,
    objects: &[CadObject],
    labels: &CadLabels,
    pages: &BTreeMap<String, u32>,
    budget: &mut PanelRowBudget,
) -> semio_framework_plugin::UiAssemblyResult<(String, Option<UiLabel>, bool, UiFixedList<BuiltNode>)> {
    let section_id = format!("cad-play-document.{id_suffix}");
    let items = paged_section_from(&section_id, objects, pages, budget, |object, _| object_tree_item(id_suffix, object, labels))?;
    Ok((section_id, Some(ui_label(label.as_str())?), true, items))
}

/// 🌳️ One pane's references section: collapsed by default, "(none)"-placeholder when empty.
pub fn artifact_references_section(
    document: &CadSnapshot,
    model_definition_id: &str,
    labels: &CadLabels,
    pages: &BTreeMap<String, u32>,
    budget: &mut PanelRowBudget,
) -> semio_framework_plugin::UiAssemblyResult<(String, Option<UiLabel>, bool, UiFixedList<BuiltNode>)> {
    let section_id = format!("cad-play-document.references.{model_definition_id}");
    let items = paged_section_from(&section_id, references_for(document, model_definition_id), pages, budget, |reference, _| reference_tree_item(model_definition_id, reference, labels))?;
    Ok((section_id, Some(ui_label(labels.references.as_str())?), false, items))
}

pub fn build_document_tree(envelope: &CadPlayView, labels: &CadLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    // 🪆️ Each pane's objects come from its composed `s.stdio.semio.model` child's local owner — the
    // same resolver the world scenes read (`edit::cad_pane_working_scene`); a handle with no local
    // owner lists nothing rather than a fabricated placeholder.
    let scenes: Vec<_> = CadPaneId::all().into_iter().map(|pane| (pane, edit::cad_pane_working_scene(&envelope.document, pane))).collect();
    let objects_of = |wanted: CadPaneId| -> &[CadObject] { scenes.iter().find(|(pane, _)| *pane == wanted).and_then(|(pane, scene)| scene.as_deref().map(|scene| edit::cad_pane_working_objects(scene, *pane).0)).unwrap_or(&[]) };
    // 🪙️ One interactive-row page for the whole tree: every section reserves the rows the sections
    // after it still need, exactly like puzzle 3d's document tree.
    let pages = &envelope.runtime.panel_pages;
    let budget = &mut PanelRowBudget::new(panel_page_rows());
    let (shape_id, shape_label, shape_open, shape_items) = budget.nested(SECTIONS - 1, |share| document_pane_section(labels.pane_shape, "shape", objects_of(CadPaneId::Shape), labels, pages, share))?;
    let (shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items) = budget.nested(SECTIONS - 2, |share| artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_SHAPE, labels, pages, share))?;
    let (building_id, building_label, building_open, building_items) = budget.nested(SECTIONS - 3, |share| document_pane_section(labels.pane_building, "building", objects_of(CadPaneId::Building), labels, pages, share))?;
    let (building_refs_id, building_refs_label, building_refs_open, building_refs_items) = budget.nested(SECTIONS - 4, |share| artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_BUILDING, labels, pages, share))?;
    let (energy_id, energy_label, energy_open, energy_items) = budget.nested(SECTIONS - 5, |share| document_pane_section(labels.pane_energy, "energy", objects_of(CadPaneId::Energy), labels, pages, share))?;
    let (energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items) = budget.nested(SECTIONS - 6, |share| artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_ENERGY, labels, pages, share))?;
    let (structure_id, structure_label, structure_open, structure_items) =
        budget.nested(SECTIONS - 7, |share| document_pane_section(labels.pane_structure_classic, "structure-classic", objects_of(CadPaneId::StructureClassic), labels, pages, share))?;
    let (structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items) =
        budget.nested(SECTIONS - 8, |share| artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, labels, pages, share))?;
    let node_items = paged_section_from("cad-play-document.nodes", &envelope.document.nodes, pages, budget, |node, _| {
        let node_ids = ui_value_list([ui_value_text(&node.id)?])?;
        let args = ui_value_map([("nodeIds", node_ids)])?;
        cad_tree_item(format!("cad-node:{}", node.id), &node.label, Some("git-branch"), cad_action("setNodeSelection", Some(args))?)
    })?;

    let mut builder = PanelTreeBuilder::new("cad-play-document")?
        .section(shape_id, shape_label, shape_open, shape_items)?
        .section_or_placeholder(shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items, ui_label(labels.none_placeholder.as_str())?)?
        .section(building_id, building_label, building_open, building_items)?
        .section_or_placeholder(building_refs_id, building_refs_label, building_refs_open, building_refs_items, ui_label(labels.none_placeholder.as_str())?)?
        .section(energy_id, energy_label, energy_open, energy_items)?
        .section_or_placeholder(energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items, ui_label(labels.none_placeholder.as_str())?)?
        .section(structure_id, structure_label, structure_open, structure_items)?
        .section_or_placeholder(structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items, ui_label(labels.none_placeholder.as_str())?)?
        .section("cad-play-document.nodes", Some(ui_label(labels.nodes.as_str())?), true, node_items)?
        .interaction_domain(CAD_INTERACTION_DOMAIN)?;
    if let Some(ids) = document_tree_selected_ids(&envelope.document, &envelope.runtime)? {
        builder = builder.selected(ids)?;
    }
    if let Some(ids) = document_tree_highlighted_ids(&envelope.document, &envelope.runtime)? {
        builder = builder.highlighted(ids)?;
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
