//! 📄️ CAD play app panel — the document tree: every pane's objects (with their primitive children)
//! and reference overlays, plus the scene's nodes.

use crate::apps::cad::terminology::{typology_label, CadLabels};
use crate::apps::cad::{cad_action, cad_pane_suffix, cad_tree_item, CadPlayRuntime, CadPlayView};
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC};
use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadObject;
use crate::artifacts::cad::{CadPaneId, CadReference, CadSnapshot};
use semio_framework_plugin::{
    Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const CAD_PLAY_BODY_DOCUMENT: &str = "cad.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(CAD_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn object_tree_item(id_suffix: &str, object: &CadObject, labels: &CadLabels) -> UiTreeItemNode {
    let primitive_items: Vec<UiTreeItemNode> = object
        .primitives
        .iter()
        .map(|primitive| {
            let mut item = cad_tree_item(
                format!("cad-primitive:{id_suffix}:{}:{}", object.id, primitive.primitive_id),
                Label::data(format!("{}: {}", primitive.slot, primitive.primitive_id)),
                Some("hexagon"),
                cad_action(
                    "setPrimitiveSelection",
                    Some(json!({
                        "objectId": object.id,
                        "primitiveId": primitive.primitive_id,
                        "kind": primitive.kind,
                    })),
                ),
            );
            item.hover_action = Some(cad_action("worldHover", Some(json!({ "id": object.id }))));
            item.unhover_action = Some(cad_action("worldHover", None));
            item
        })
        .collect();
    let mut item = cad_tree_item(format!("cad-object:{id_suffix}:{}", object.id), Label::data(object.label.clone()), Some("box"), cad_action("worldSelect", Some(json!({ "ids": [object.id], "merge": "replace" }))));
    if !object.typology.is_empty() {
        item.description = Some(typology_label(&object.typology, labels).to_string());
    }
    item.hover_action = Some(cad_action("worldHover", Some(json!({ "id": object.id }))));
    item.unhover_action = Some(cad_action("worldHover", None));
    item.dimmed = Some(!object.visible);
    item.draggable = Some(!object.locked);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if object.visible { "eye-off" } else { "eye" }.into(),
            label: Some(if object.visible { labels.hide } else { labels.show }.into()),
            action: cad_action("patchObject", Some(json!({ "objectId": object.id, "field": "hidden", "value": object.visible }))),
            placement: Some(UiTreeActionPlacement::Row),
        },
        UiTreeItemAction {
            icon_id: if object.locked { "unlock" } else { "lock" }.into(),
            label: Some(if object.locked { labels.unlock } else { labels.lock }.into()),
            action: cad_action("patchObject", Some(json!({ "objectId": object.id, "field": "locked", "value": !object.locked }))),
            placement: Some(UiTreeActionPlacement::Row),
        },
        UiTreeItemAction { icon_id: "copy".into(), label: Some(labels.duplicate.into()), action: cad_action("duplicateObject", Some(json!({ "objectId": object.id }))), placement: Some(UiTreeActionPlacement::Menu) },
        UiTreeItemAction { icon_id: "trash-2".into(), label: Some(labels.delete.into()), action: cad_action("deleteObject", Some(json!({ "objectId": object.id }))), placement: Some(UiTreeActionPlacement::Menu) },
    ]);
    if !primitive_items.is_empty() {
        item.items = Some(primitive_items);
        item.default_open = Some(false);
    }
    item
}

pub fn reference_tree_item(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> UiTreeItemNode {
    let mut item = cad_tree_item(
        format!("cad-reference:{model_definition_id}:{}", reference.id),
        Label::data(reference.id.clone()),
        Some("image"),
        cad_action("setReferenceSelection", Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id }))),
    );
    item.description = Some(reference.source_url.clone());
    item.hover_action = Some(cad_action("referenceHover", Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id }))));
    item.unhover_action = Some(cad_action("referenceHover", None));
    item.dimmed = Some(reference.hidden);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if reference.hidden { "eye" } else { "eye-off" }.into(),
            label: Some(if reference.hidden { labels.show } else { labels.hide }.into()),
            action: cad_action(
                "patchCadPlayReference",
                Some(json!({
                    "modelDefinitionId": model_definition_id,
                    "referenceId": reference.id,
                    "field": "hidden",
                    "value": !reference.hidden,
                })),
            ),
            placement: Some(UiTreeActionPlacement::Row),
        },
        UiTreeItemAction {
            icon_id: if reference.locked { "unlock" } else { "lock" }.into(),
            label: Some(if reference.locked { labels.unlock } else { labels.lock }.into()),
            action: cad_action(
                "patchCadPlayReference",
                Some(json!({
                    "modelDefinitionId": model_definition_id,
                    "referenceId": reference.id,
                    "field": "locked",
                    "value": !reference.locked,
                })),
            ),
            placement: Some(UiTreeActionPlacement::Row),
        },
    ]);
    item
}

/// 🗂️ The `document.references_by_model_definition_id` lookup repeated once per pane in `build_document_tree`.
pub fn references_for<'a>(document: &'a CadSnapshot, model_definition_id: &str) -> &'a [CadReference] {
    document.references_by_model_definition_id.get(model_definition_id).map_or(&[][..], |rows| rows.as_slice())
}

pub fn document_tree_selected_ids(document: &CadSnapshot, runtime: &CadPlayRuntime) -> Option<Vec<String>> {
    if let (Some(model_definition_id), Some(reference_id)) = (runtime.selected_reference_model_definition_id.as_deref(), runtime.selected_reference_id.as_deref()) {
        return Some(vec![format!("cad-reference:{model_definition_id}:{reference_id}")]);
    }
    if let (Some(object_id), Some(primitive_id)) = (runtime.selected_object_ids.first(), runtime.selected_primitive_id.as_deref()) {
        if let Some(pane) = cad_find_object_pane(document, object_id) {
            return Some(vec![format!("cad-primitive:{}:{object_id}:{primitive_id}", cad_pane_suffix(pane))]);
        }
    }
    let selected: Vec<String> = runtime.selected_object_ids.iter().filter_map(|object_id| cad_find_object_pane(document, object_id).map(|pane| format!("cad-object:{}:{object_id}", cad_pane_suffix(pane)))).collect();
    if selected.is_empty() {
        None
    } else {
        Some(selected)
    }
}

pub fn document_tree_highlighted_ids(document: &CadSnapshot, runtime: &CadPlayRuntime) -> Option<Vec<String>> {
    let hovered = runtime.hovered_object_id.as_deref()?;
    if let Some(reference_id) = hovered.strip_prefix("reference:") {
        for pane in CadPaneId::all() {
            let model_definition_id = pane.model_definition_id();
            if document.references_by_model_definition_id.get(model_definition_id).is_some_and(|rows| rows.iter().any(|row| row.id == reference_id)) {
                return Some(vec![format!("cad-reference:{model_definition_id}:{reference_id}")]);
            }
        }
        return None;
    }
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `cad_find_object_pane` is
    // retired (no live per-pane object list on `CadSnapshot`, only composed model-child HANDLES).
    // Documented reduced-fidelity gap: hover highlighting no longer resolves an object's pane.
    let _ = (document, hovered);
    None
}

/// 🌳️ One pane's object section: namespaced by `id_suffix`, always expanded.
pub fn document_pane_section(label: impl Into<Label>, id_suffix: &str, objects: &[CadObject], labels: &CadLabels) -> (String, Option<Label>, bool, Vec<UiTreeItemNode>) {
    (format!("cad-play-document.{id_suffix}"), Some(label.into()), true, objects.iter().map(|object| object_tree_item(id_suffix, object, labels)).collect())
}

/// 🌳️ One pane's references section: collapsed by default, "(none)"-placeholder when empty.
pub fn artifact_references_section(document: &CadSnapshot, model_definition_id: &str, labels: &CadLabels) -> (String, Option<Label>, bool, Vec<UiTreeItemNode>) {
    (format!("cad-play-document.references.{model_definition_id}"), Some(labels.references.into()), false, references_for(document, model_definition_id).iter().map(|reference| reference_tree_item(model_definition_id, reference, labels)).collect())
}

pub fn build_document_tree(envelope: &CadPlayView, labels: &CadLabels) -> UiNode {
    let node_items: Vec<UiTreeItemNode> =
        envelope.document.nodes.iter().map(|node| cad_tree_item(format!("cad-node:{}", node.id), Label::data(node.label.clone()), Some("git-branch"), cad_action("setNodeSelection", Some(json!({ "nodeIds": [node.id] }))))).collect();

    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `CadSnapshot`'s inline
    // per-pane object lists are gone (composed `s.stdio.semio.model` CHILD documents now, unresolved
    // at this render boundary — see `🔖️Composition` in `🏪️store/🦀️component.rs`). Each pane
    // section renders empty until a resolved-child-content render path exists; documented gap.
    let no_objects: Vec<CadObject> = Vec::new();
    let (shape_id, shape_label, shape_open, shape_items) = document_pane_section(labels.pane_shape, "shape", &no_objects, labels);
    let (shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_SHAPE, labels);
    let (building_id, building_label, building_open, building_items) = document_pane_section(labels.pane_building, "building", &no_objects, labels);
    let (building_refs_id, building_refs_label, building_refs_open, building_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_BUILDING, labels);
    let (energy_id, energy_label, energy_open, energy_items) = document_pane_section(labels.pane_energy, "energy", &no_objects, labels);
    let (energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_ENERGY, labels);
    let (structure_id, structure_label, structure_open, structure_items) = document_pane_section(labels.pane_structure_classic, "structure-classic", &no_objects, labels);
    let (structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, labels);

    let mut builder = PanelTreeBuilder::new("cad-play-document")
        .section(shape_id, shape_label, shape_open, shape_items)
        .section_or_placeholder(shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items, labels.none_placeholder)
        .section(building_id, building_label, building_open, building_items)
        .section_or_placeholder(building_refs_id, building_refs_label, building_refs_open, building_refs_items, labels.none_placeholder)
        .section(energy_id, energy_label, energy_open, energy_items)
        .section_or_placeholder(energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items, labels.none_placeholder)
        .section(structure_id, structure_label, structure_open, structure_items)
        .section_or_placeholder(structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items, labels.none_placeholder)
        .section("cad-play-document.nodes", Some(labels.nodes.into()), true, node_items);
    if let Some(ids) = document_tree_selected_ids(&envelope.document, &envelope.runtime) {
        builder = builder.selected(ids);
    }
    if let Some(ids) = document_tree_highlighted_ids(&envelope.document, &envelope.runtime) {
        builder = builder.highlighted(ids);
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::cad::testkit::*;
    use crate::apps::cad::config::CadConfig;
    use crate::apps::cad::{CadPlayApp, CadPlayRuntime};
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_scene};
    use semio_framework_plugin::{ArtifactView, PluginApp, SelectionSet, UiNode, ViewModel};

    #[test]
    fn document_lists_objects_and_nodes() {
        let mut app = new_app();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-object:"));
        assert!(json.contains("cad-node:"));
    }

    #[test]
    fn document_tree_shows_name_with_kind_as_secondary_label() {
        let app = CadPlayApp::default();
        let mut scene = default_document();
        scene.objects[0].label = "U2".into();
        scene.objects[0].typology = "building.building.beam".into();
        let history = empty_history();
        let doc = ArtifactView { snapshot: &scene, history: &history };
        let node = render_direct(&app, CAD_PLAY_BODY_DOCUMENT, &doc, &CadConfig::default());
        let UiNode::Tree(tree) = node else {
            panic!("document body should render a tree");
        };
        let object_item = tree.sections.iter().flat_map(|section| section.items.iter()).find(|item| item.id.contains("cad-object:") && item.label.as_str() == "U2").expect("named object tree item");
        assert_eq!(object_item.description.as_deref(), Some("Beam"));

        let de_node = render_direct(&app, CAD_PLAY_BODY_DOCUMENT, &doc, &CadConfig { locale: "de".into(), ..CadConfig::default() });
        let UiNode::Tree(de_tree) = de_node else {
            panic!("document body should render a tree");
        };
        let de_object_item = de_tree.sections.iter().flat_map(|section| section.items.iter()).find(|item| item.id.contains("cad-object:") && item.label.as_str() == "U2").expect("named object tree item in German");
        assert_eq!(de_object_item.description.as_deref(), Some("Träger"));
    }

    #[test]
    fn document_tree_includes_primitive_children() {
        let mut app = new_app();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-primitive:"));
        assert!(json.contains("hoverAction"));
    }

    #[test]
    fn document_tree_reflects_viewport_selection() {
        let scene = forest_play_scene();
        let object_id = scene.objects.iter().find(|object| object.visible).expect("visible shape object").id.clone();
        let runtime = CadPlayRuntime { selected_object_ids: SelectionSet::from(vec![object_id.clone()]), hovered_object_id: Some(object_id.clone()), ..CadPlayRuntime::default() };
        let selected = document_tree_selected_ids(&scene, &runtime).expect("selected");
        assert!(selected.iter().any(|id| id.contains(&object_id) && id.starts_with("cad-object:shape:")));
        let highlighted = document_tree_highlighted_ids(&scene, &runtime).expect("highlighted");
        assert!(highlighted.iter().any(|id| id.contains(&object_id) && id.starts_with("cad-object:shape:")));
    }

    #[test]
    fn cad_labels_translate_document_tree_panes_in_german() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = ArtifactView { snapshot: &scene, history: &history };
        let config = CadConfig { locale: "de".into(), ..CadConfig::default() };
        let node = render_direct(&app, CAD_PLAY_BODY_DOCUMENT, &doc, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Form\""));
        assert!(json.contains("Gebäude"));
        assert!(json.contains("Energie"));
        assert!(json.contains("Tragwerk Klassisch"));
        assert!(json.contains("Referenzen"));
        assert!(json.contains("\"Knoten\""));
        assert!(!json.contains("\"Shape\""));
        assert!(!json.contains("Struktur Klassisch"));
    }
}
//#endregion 🧪️Tests
