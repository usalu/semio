//! 🔍️ CAD play app panel — the inspection panel: the field groups for whatever is selected
//! (object multi-selection, a primitive slot, a reference overlay, a node), or a schema summary.

use crate::editor::cad::terminology::{typology_label, CadLabels};
use crate::editor::cad::modes::edit;
use crate::editor::cad::{cad_pane_suffix, ui_label, ui_value_map, ui_value_text, CadPlayView};
use crate::standards::v1::subsets::any::io::geometry_import::CadObject;
use crate::standards::v1::subsets::any::schema::inferences::object_scale_json;
use crate::{CadNode, CadPaneId};
use semio_framework_plugin::{
    tree_item, tree_item_desc, tree_item_with_action, ui_node_list, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult, UiFixedList, UiValue,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const CAD_PLAY_BODY_PROPERTIES: &str = "cad.play.properties";
/// 🪟️ The windowed section the selected object ids are listed in — the node key the host reports its
/// open/scroll window for.
pub const IDS_SECTION: &str = "cad-play-inspector.ids";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(CAD_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render

const ROOT: &str = "cad-play-inspector";

fn push(fields: &mut UiFixedList<BuiltNode>, node: UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<()> {
    fields.try_push(node?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "cad inspector field admission failed"))
}

/// 🧾️ One read-only `label` / `value` row.
fn read_only(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<()> {
    push(fields, read_only_row(id, label, value))
}

/// 🧾️ The bare `label` / `value` row node, for the windowed sections that collect their entries
/// first and materialise only the slice the host asked for.
fn read_only_row(id: &str, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<BuiltNode> {
    tree_item_desc(format!("{ROOT}.{id}"), ui_label(label)?, Some(value.to_string()))
}

fn vec3(value: [f64; 3]) -> String {
    format!("{}, {}, {}", value[0], value[1], value[2])
}

fn vec4(value: [f64; 4]) -> String {
    format!("{}, {}, {}, {}", value[0], value[1], value[2], value[3])
}

/// 🔎️ Every selected object across the four panes, with the pane it lives in — the `"cad"` domain's
/// object ids are unique across panes, so the first pane owning an id is its home.
pub(crate) fn selected_objects(envelope: &CadPlayView) -> Vec<(CadPaneId, CadObject)> {
    let mut selected = Vec::new();
    for pane in CadPaneId::all() {
        let Some(scene) = edit::cad_pane_working_scene(&envelope.document, pane) else { continue };
        let (objects, _) = edit::cad_pane_working_objects(&scene, pane);
        for id in &envelope.interaction.ids {
            if let Some(object) = objects.iter().find(|object| &object.id == id) {
                selected.push((pane, object.clone()));
            }
        }
    }
    selected
}

/// 🔍️ The selected object's field group (the first selected object carries the fields, the ids row
/// lists every selected id), or `None` when the `"cad"` selection resolves to no object in this
/// document.
fn selected_object_section(envelope: &CadPlayView, labels: &CadLabels, windows: &TreeWindows<'_>) -> Option<UiAssemblyResult<BuiltNode>> {
    let selected = selected_objects(envelope);
    let (pane, object) = selected.first()?;
    let build = || -> UiAssemblyResult<BuiltNode> {
        // 🪟️ The ids block grows with the selection, so it is a windowed section of its OWN, keyed by
        // the RAW object id — an offset window must never renumber a row key, and the object's own
        // fields must not be pushed out of the window by a wide selection (they are an author-fixed
        // group of nine rows, paid for by `TREE_WINDOW_FIXED_NODE_HEADROOM`).
        let ids: Vec<String> = selected.iter().map(|(_, selected)| selected.id.clone()).collect();
        let mut fields = UiFixedList::default();
        read_only(&mut fields, "object.label", labels.label.as_str(), &object.label)?;
        read_only(&mut fields, "object.typology", labels.typology.as_str(), typology_label(&object.typology, labels))?;
        read_only(&mut fields, "object.pane", labels.slot.as_str(), cad_pane_suffix(*pane))?;
        read_only(&mut fields, "object.origin", labels.position.as_str(), vec3(object.origin))?;
        read_only(&mut fields, "object.orientation", labels.rotation.as_str(), vec4(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])))?;
        read_only(&mut fields, "object.scale", labels.scale.as_str(), vec3(object_scale_json(object)))?;
        read_only(&mut fields, "object.primitives", labels.primitive.as_str(), object.primitives.iter().map(|primitive| format!("{} ({})", primitive.slot, primitive.kind)).collect::<Vec<_>>().join(", "))?;
        read_only(&mut fields, "object.hidden", labels.hidden.as_str(), !object.visible)?;
        read_only(&mut fields, "object.locked", labels.locked.as_str(), object.locked)?;
        let title = if selected.len() == 1 { labels.object.as_str().to_string() } else { format!("{} {}", selected.len(), labels.objects.as_str()) };
        PanelTreeBuilder::new(ROOT)?
            .window_section(windows, IDS_SECTION, Some(ui_label(labels.id.as_str())?), true, &ids, |id| read_only_row(&format!("ids.{id}"), labels.id.as_str(), id))?
            .section(format!("{ROOT}.object"), Some(ui_label(&title)?), true, fields)?
            .build()
    };
    Some(build())
}

/// 🔍️ The selected object's fields when the `"cad"` domain selects one, else the document summary
/// (schema, active utility, object count across the four panes).
/// 🩹️ One `patchCadPlayReference` row — a bounded edit (a boolean `value` or a numeric `delta`) on
/// one field of the selected reference, dispatched through the real `ChangeReference*`/
/// `MoveReference` mutation path.
fn reference_patch_row(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, model_definition_id: &str, reference_id: &str, field: &str, value: Option<bool>, delta: Option<f64>) -> UiAssemblyResult<()> {
    // 🔑️ `UiMapBuilder::push` admits keys in strictly ascending order only.
    let mut entries: Vec<(&'static str, UiValue)> = Vec::new();
    if let Some(delta) = delta {
        entries.push(("delta", UiValue::Number(delta)));
    }
    entries.push(("field", ui_value_text(field)?));
    entries.push(("modelDefinitionId", ui_value_text(model_definition_id)?));
    entries.push(("referenceId", ui_value_text(reference_id)?));
    if let Some(value) = value {
        entries.push(("value", ui_value_text(if value { "true" } else { "false" })?));
    }
    let action = crate::editor::cad::cad_action("patchCadPlayReference", Some(ui_value_map(entries)?))?;
    push(fields, tree_item_with_action(format!("{ROOT}.{id}"), ui_label(label)?, None, action))
}

/// 🖼️ The app-owned reference-overlay selection (`CadPlayRuntime::selected_reference_*`): the
/// reference's fields plus bounded edit rows.
fn selected_reference_section(envelope: &CadPlayView, labels: &CadLabels) -> Option<UiAssemblyResult<BuiltNode>> {
    let model_definition_id = envelope.runtime.selected_reference_model_definition_id.as_deref()?;
    let reference_id = envelope.runtime.selected_reference_id.as_deref()?;
    let reference = envelope.document.references_by_model_definition_id.get(model_definition_id)?.iter().find(|reference| reference.id == reference_id)?;
    let build = || -> UiAssemblyResult<BuiltNode> {
        let mut fields = UiFixedList::default();
        read_only(&mut fields, "reference.id", labels.id.as_str(), &reference.id)?;
        read_only(&mut fields, "reference.source", labels.source.as_str(), &reference.source_url)?;
        read_only(&mut fields, "reference.pane", labels.slot.as_str(), model_definition_id)?;
        read_only(&mut fields, "reference.width", labels.width_world.as_str(), reference.width_world)?;
        read_only(&mut fields, "reference.origin", labels.position.as_str(), vec3(reference.origin))?;
        reference_patch_row(&mut fields, "reference.hidden", if reference.hidden { labels.show.as_str() } else { labels.hide.as_str() }, model_definition_id, reference_id, "hidden", Some(!reference.hidden), None)?;
        reference_patch_row(&mut fields, "reference.locked", if reference.locked { labels.unlock.as_str() } else { labels.lock.as_str() }, model_definition_id, reference_id, "locked", Some(!reference.locked), None)?;
        reference_patch_row(&mut fields, "reference.width.grow", &format!("{} +1", labels.width_world.as_str()), model_definition_id, reference_id, "widthWorld", None, Some(1.0))?;
        reference_patch_row(&mut fields, "reference.width.shrink", &format!("{} −1", labels.width_world.as_str()), model_definition_id, reference_id, "widthWorld", None, Some(-1.0))?;
        for (axis, sign, delta) in [("x", "+", 1.0), ("x", "−", -1.0), ("y", "+", 1.0), ("y", "−", -1.0)] {
            reference_patch_row(&mut fields, &format!("reference.origin.{axis}.{}", if delta > 0.0 { "plus" } else { "minus" }), &format!("{} {} {sign}1", labels.position.as_str(), axis.to_ascii_uppercase()), model_definition_id, reference_id, &format!("origin.{axis}"), None, Some(delta))?;
        }
        PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.reference"), Some(ui_label(labels.reference.as_str())?), true, fields)?.build()
    };
    Some(build())
}

/// 🌿️ The document-tree node selection (`CadPlayRuntime::selected_node_ids`): read-only rows.
fn selected_node_section(envelope: &CadPlayView, labels: &CadLabels) -> Option<UiAssemblyResult<BuiltNode>> {
    let first = envelope.runtime.selected_node_ids.first()?;
    let node: &CadNode = envelope.document.nodes.iter().find(|node| &node.id == first)?;
    let build = || -> UiAssemblyResult<BuiltNode> {
        let mut fields = UiFixedList::default();
        read_only(&mut fields, "node.id", labels.id.as_str(), &node.id)?;
        read_only(&mut fields, "node.label", labels.label.as_str(), &node.label)?;
        PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.node"), Some(ui_label(labels.node.as_str())?), true, fields)?.build()
    };
    Some(build())
}

pub fn build_properties_panel(envelope: &CadPlayView, labels: &CadLabels, active_utility: Option<&str>, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    if let Some(section) = selected_object_section(envelope, labels, windows).or_else(|| selected_reference_section(envelope, labels)).or_else(|| selected_node_section(envelope, labels)) {
        return section;
    }
    let objects = CadPaneId::all().into_iter().filter_map(|pane| edit::cad_pane_working_scene(&envelope.document, pane).map(|scene| edit::cad_pane_working_objects(&scene, pane).0.len())).sum::<usize>();
    let rows = ui_node_list([
        tree_item("cad-play-inspector.schema", ui_label(format!("{}: {}", labels.schema.as_str(), envelope.document.schema))?),
        tree_item("cad-play-inspector.utility", ui_label(format!("{}: {}", labels.utility.as_str(), active_utility.unwrap_or(labels.none_placeholder.as_str())))?),
        tree_item("cad-play-inspector.objects", ui_label(format!("{}: {objects}", labels.objects.as_str()))?),
    ])?;
    PanelTreeBuilder::new("cad-play-inspector")?.section("cad-play-inspector.summary", Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
}

//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
