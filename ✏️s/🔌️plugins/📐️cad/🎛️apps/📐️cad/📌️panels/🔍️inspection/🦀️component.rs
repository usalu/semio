//! 🔍️ CAD play app panel — the inspection panel: the field groups for whatever is selected
//! (object multi-selection, a primitive slot, a reference overlay, a node), or a schema summary.

use crate::apps::cad::terminology::{typology_label, CadLabels};
use crate::apps::cad::{cad_action, CadPlayView, TYPOLOGY_CATALOG};
use crate::artifacts::cad::{cad_all_objects, CadNode, CadObject, CadReference};
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_inspector_vec3_group, ActionDescriptor, Label, LocalizedLabel, PanelGroup, PanelTabDefinition,
    PanelTabKind, UiFieldNode, UiGroupNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const CAD_PLAY_BODY_PROPERTIES: &str = "cad.play.properties";
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
pub fn build_properties_panel(envelope: &CadPlayView, labels: &CadLabels, active_utility: Option<&str>) -> UiNode {
    if let (Some(object_id), Some(primitive_id)) = (envelope.runtime.selected_object_ids.first(), envelope.runtime.selected_primitive_id.as_deref()) {
        if let Some((object, _)) = cad_all_objects(&envelope.document).find(|(object, _)| object.id == *object_id) {
            let kind = envelope.runtime.selected_primitive_kind.as_deref().or_else(|| object.primitives.iter().find(|primitive| primitive.primitive_id == primitive_id).map(|primitive| primitive.kind.as_str())).unwrap_or("primitive");
            return ui_inspector_groups_to_tree(&[primitive_inspector_group(object, labels, primitive_id, kind)]);
        }
    }
    if !envelope.runtime.selected_object_ids.is_empty() {
        let selected: Vec<&CadObject> = envelope.runtime.selected_object_ids.iter().filter_map(|id| cad_all_objects(&envelope.document).find(|(object, _)| &object.id == id).map(|(object, _)| object)).collect();
        if !selected.is_empty() {
            return ui_inspector_groups_to_tree(&[object_inspector_group(&selected, labels)]);
        }
    }
    if let (Some(model_definition_id), Some(reference_id)) = (envelope.runtime.selected_reference_model_definition_id.as_deref(), envelope.runtime.selected_reference_id.as_deref()) {
        if let Some(reference) = envelope.document.references_by_model_definition_id.get(model_definition_id).and_then(|rows| rows.iter().find(|row| row.id == reference_id)) {
            return ui_inspector_groups_to_tree(&[reference_inspector_group(model_definition_id, reference, labels)]);
        }
    }
    if let Some(node_id) = envelope.runtime.selected_node_ids.first() {
        if let Some(node) = envelope.document.nodes.iter().find(|entry| &entry.id == node_id) {
            return ui_inspector_groups_to_tree(&[node_inspector_group(node, labels)]);
        }
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "cad-play-inspector.empty".into(),
        label: Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("cad-play-inspector.schema", labels.schema, &envelope.document.schema),
            ui_inspector_readonly_field("cad-play-inspector.utility", labels.utility, active_utility.unwrap_or(labels.none_placeholder.as_str())),
            ui_inspector_readonly_field("cad-play-inspector.objects", labels.objects, envelope.document.objects.len().to_string()),
        ],
    }])
}

/// @emoji 🌀️ Builds an editable 4-component quaternion group (`X`/`Y`/`Z`/`W` steppers) — orientation
/// fields have no shared helper (quaternions aren't `ui_inspector_vec3_group`'s 3-wide shape), so
/// this mirrors that helper's structure one component wider. The patch handler renormalizes after
/// any component edit so the result stays a valid unit quaternion.
pub fn inspector_quat_group(id: &str, label: impl Into<Label>, values: &[[f64; 4]], step: f64, axis_action: impl Fn(&str) -> ActionDescriptor) -> UiNode {
    // 🔤️ Axis symbols (X/Y/Z/W) are mathematical notation, not translatable UI chrome.
    let component = |index: usize, name: &str, label: &'static str| {
        let values: Vec<f64> = values.iter().map(|q| q[index]).collect();
        ui_inspector_stepper_field(format!("{id}.{name}"), Label::data(label), &values, step, axis_action(name))
    };
    UiNode::Group(UiGroupNode {
        id: id.into(),
        label: label.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![component(0, "x", "X"), component(1, "y", "Y"), component(2, "z", "Z"), component(3, "w", "W")],
        menu: None,
    })
}

pub fn object_inspector_group(objects: &[&CadObject], term_labels: &CadLabels) -> UiInspectorFieldGroup {
    let object_ids: Vec<String> = objects.iter().map(|object| object.id.clone()).collect();
    let labels: Vec<String> = objects.iter().map(|object| object.label.clone()).collect();
    let typologies: Vec<String> = objects.iter().map(|object| object.typology.clone()).collect();
    let hidden: Vec<bool> = objects.iter().map(|object| !object.visible).collect();
    let locked: Vec<bool> = objects.iter().map(|object| object.locked).collect();
    let origins: Vec<[f64; 3]> = objects.iter().map(|object| object.origin).collect();
    let scales: Vec<[f64; 3]> = objects.iter().map(|object| object.scale.unwrap_or([1.0, 1.0, 1.0])).collect();
    let orientations: Vec<[f64; 4]> = objects.iter().map(|object| object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])).collect();
    let label_mixed = ui_inspector_mixed_text(&labels);
    let typology_mixed = ui_inspector_mixed_text(&typologies);
    let hidden_mixed = ui_inspector_mixed_toggle(&hidden);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    UiInspectorFieldGroup {
        id: "cad-play-inspector.object".into(),
        label: if objects.len() == 1 { term_labels.object.into() } else { Label::data(format!("{} {}", objects.len(), term_labels.objects.as_str())) },
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.label".into(),
                label: term_labels.label.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "cad-play-inspector.object.label.input".into(),
                    input_kind: "text".into(),
                    value: label_mixed.value.clone(),
                    placeholder: label_mixed.placeholder.map(Label::data),
                    commit: None,
                    on_change: cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": "label" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    presence: UiPresence::default(),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.typology".into(),
                label: term_labels.typology.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "cad-play-inspector.object.typology.select".into(),
                    value: typology_mixed.value.clone(),
                    items: TYPOLOGY_CATALOG.iter().map(|entry| UiSelectItem { value: entry.typology.into(), label: Label::data(typology_label(entry.typology, term_labels)) }).collect(),
                    placeholder: typology_mixed.placeholder.map(Label::data),
                    on_change: cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": "typology" }))),
                    presence: UiPresence::default(),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.hidden".into(),
                label: term_labels.hidden.into(),
                child: Box::new(UiNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.hidden.toggle".into(),
                    icon_id: "eye-off".into(),
                    text: None,
                    on_change: cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": "hidden" }))),
                    presence: UiPresence::selected(hidden_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.locked".into(),
                label: term_labels.locked.into(),
                child: Box::new(UiNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.locked.toggle".into(),
                    icon_id: "lock".into(),
                    text: None,
                    on_change: cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": "locked" }))),
                    presence: UiPresence::selected(locked_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            {
                let object_ids = object_ids.clone();
                ui_inspector_vec3_group("cad-play-inspector.object.origin", term_labels.position, &origins, 0.1, move |axis| cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": format!("origin.{axis}") }))))
            },
            {
                let object_ids = object_ids.clone();
                ui_inspector_vec3_group("cad-play-inspector.object.scale", term_labels.scale, &scales, 0.1, move |axis| cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": format!("scale.{axis}") }))))
            },
            inspector_quat_group("cad-play-inspector.object.orientation", term_labels.rotation, &orientations, 0.01, |axis| cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": format!("orientation.{axis}") })))),
        ],
    }
}

pub fn primitive_inspector_group(object: &CadObject, labels: &CadLabels, primitive_id: &str, kind: &str) -> UiInspectorFieldGroup {
    let slot = object.primitives.iter().find(|primitive| primitive.primitive_id == primitive_id).map_or("primitive", |primitive| primitive.slot.as_str());
    UiInspectorFieldGroup {
        id: "cad-play-inspector.primitive".into(),
        label: labels.primitive.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("cad-play-inspector.primitive.object", labels.object, &object.label),
            ui_inspector_readonly_field("cad-play-inspector.primitive.slot", labels.slot, slot),
            ui_inspector_readonly_field("cad-play-inspector.primitive.kind", labels.kind, kind),
            ui_inspector_readonly_field("cad-play-inspector.primitive.id", labels.id, primitive_id),
        ],
    }
}

pub fn reference_inspector_group(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.reference".into(),
        label: labels.reference.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("cad-play-inspector.reference.id", labels.id, &reference.id),
            ui_inspector_readonly_field("cad-play-inspector.reference.source", labels.source, &reference.source_url),
            {
                let patch_cmd = |field: &str| cad_action("patchCadPlayReference", Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id, "field": field })));
                ui_inspector_stepper_field("cad-play-inspector.reference.widthWorld", labels.width_world, &[reference.width_world], 0.1, patch_cmd("widthWorld"))
            },
            {
                let patch_cmd = move |axis: &str| cad_action("patchCadPlayReference", Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id, "field": format!("origin.{axis}") })));
                ui_inspector_vec3_group("cad-play-inspector.reference.origin", labels.position, &[reference.origin], 0.1, patch_cmd)
            },
        ],
    }
}

pub fn node_inspector_group(node: &CadNode, labels: &CadLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.node".into(),
        label: labels.node.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.node.label".into(),
                label: labels.label.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "cad-play-inspector.node.label.input".into(),
                    input_kind: "text".into(),
                    value: node.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_action("renameNode", Some(json!({ "nodeId": node.id }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    presence: UiPresence::default(),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            ui_inspector_readonly_field("cad-play-inspector.node.kind", labels.kind, &node.kind),
        ],
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::cad::testkit::*;
    use crate::apps::cad::config::CadConfig;
    use crate::apps::cad::terminology::cad_labels;
    use crate::apps::cad::{make_object_for_typology, CadPlayRuntime};
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::default_document;
    use crate::artifacts::cad::CadPaneId;
    use semio_framework_plugin::SelectionSet;
    fn selected_box_panel(config: &CadConfig) -> String {
        let runtime = CadPlayRuntime { selected_object_ids: SelectionSet::from(vec!["object-box-1".into()]), ..CadPlayRuntime::default() };
        let panel = build_properties_panel(&view(default_document(), runtime), cad_labels(config), None);
        serde_json::to_string(&panel).unwrap()
    }



    #[test]
    fn multi_selection_inspector_shows_mixed_values() {
        let mut scene = default_document();
        let second = make_object_for_typology("spatial.shape.primitive.box", 1, CadPaneId::Shape);
        let second_id = second.id.clone();
        scene.objects.push(second);
        scene.objects[0].label = "Alpha".into();
        scene.objects[1].label = "Beta".into();
        scene.objects[0].orientation = Some([0.0, 0.0, 0.0, 1.0]);
        scene.objects[1].orientation = Some([0.0, 0.707, 0.0, 0.707]);
        let runtime = CadPlayRuntime { selected_object_ids: SelectionSet::from(vec!["object-box-1".into(), second_id]), ..CadPlayRuntime::default() };
        let panel = build_properties_panel(&view(scene, runtime), cad_labels(&CadConfig::default()), None);
        let json = serde_json::to_string(&panel).unwrap();
        assert!(json.contains("Mixed"));
        assert!(json.contains("cad-play-inspector.object.orientation"));
    }

    #[test]
    fn cad_labels_resolve_native_by_default() {
        let json = selected_box_panel(&CadConfig::default());
        assert!(json.contains("\"Object\""));
        assert!(!json.contains("Building component"));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_in_english() {
        let config = CadConfig { terminology: "reuse".into(), locale: "en".into(), ..CadConfig::default() };
        let json = selected_box_panel(&config);
        assert!(json.contains("Building component"));
        assert!(!json.contains("\"Object\""));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_in_german() {
        let config = CadConfig { terminology: "reuse".into(), locale: "de".into(), ..CadConfig::default() };
        assert!(selected_box_panel(&config).contains("Baukomponente"));
    }

    #[test]
    fn cad_labels_resolve_native_terminology_in_german() {
        let config = CadConfig { terminology: "native".into(), locale: "de".into(), ..CadConfig::default() };
        assert!(selected_box_panel(&config).contains("\"Objekt\""));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_for_primitive() {
        let runtime = CadPlayRuntime { selected_object_ids: SelectionSet::from(vec!["object-box-1".into()]), selected_primitive_id: Some("box-solid".into()), ..CadPlayRuntime::default() };
        let config = CadConfig { terminology: "reuse".into(), locale: "de".into(), ..CadConfig::default() };
        let panel = build_properties_panel(&view(default_document(), runtime), cad_labels(&config), None);
        assert!(serde_json::to_string(&panel).unwrap().contains("Bauteil"));
    }
}
//#endregion 🧪️Tests
