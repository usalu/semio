//! 🔍️ CAD play app panel — the inspection panel: the field groups for whatever is selected
//! (object multi-selection, a primitive slot, a reference overlay, a node), or a schema summary.

#[cfg(test)]
use crate::editor::cad::terminology::typology_label;
use crate::editor::cad::terminology::CadLabels;
#[cfg(test)]
use crate::editor::cad::TYPOLOGY_CATALOG;
use crate::editor::cad::{CadPlayView, CAD_PLAY_APP_ID};
#[cfg(test)]
use crate::standards::v1::subsets::any::io::geometry_import::CadObject;
use crate::{CadNode, CadReference};
use protocol::DslValue;
use semio_framework_plugin::{
    tree_item, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_inspector_vec3_group, ActionDescriptor, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiFieldNode, UiGroupNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
#[cfg(test)]
use semio_framework_plugin::{ui_inspector_mixed_text, ui_inspector_mixed_toggle, UiSelectItem, UiSelectNode};

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
fn cad_action(action: &str, args: Option<DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: CAD_PLAY_APP_ID.into(), action: action.into(), args }
}

/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the object/primitive inspector
/// branches below used to scan `CadSnapshot`'s inline object list (`cad_all_objects`), which no
/// longer exists — object data lives inside composed `s.stdio.semio.model` CHILD documents,
/// unresolved at this render boundary (see `🔖️Composition` in `🏪️store/🦀️.rs`).
/// Documented reduced-fidelity gap: those two branches fall through to the reference/node/summary
/// panel until a resolved-child-content render path exists.
pub fn build_properties_panel(envelope: &CadPlayView, labels: &CadLabels, active_utility: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let rows = crate::editor::cad::ui_node_list([
        tree_item("cad-play-inspector.schema", crate::editor::cad::ui_label(format!("{}: {}", labels.schema.as_str(), envelope.document.schema))?),
        tree_item("cad-play-inspector.utility", crate::editor::cad::ui_label(format!("{}: {}", labels.utility.as_str(), active_utility.unwrap_or(labels.none_placeholder.as_str())))?),
        tree_item("cad-play-inspector.objects", crate::editor::cad::ui_label(format!("{}: 0", labels.objects.as_str()))?),
    ])?;
    PanelTreeBuilder::new("cad-play-inspector")?.section("cad-play-inspector.summary", Some(crate::editor::cad::ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
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

#[cfg(test)]
fn patch_selection_args(object_ids: &[String], field: String) -> DslValue {
    DslValue::object([("objectIds".to_string(), DslValue::Array(object_ids.iter().map(|id| DslValue::String(id.clone())).collect())), ("field".to_string(), DslValue::String(field))])
}

#[cfg(test)]
pub(crate) fn object_inspector_group(objects: &[&CadObject], term_labels: &CadLabels) -> UiInspectorFieldGroup {
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
                    on_change: cad_action("patchSelection", Some(patch_selection_args(&object_ids, "label".to_string()))),
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
                    on_change: cad_action("patchSelection", Some(patch_selection_args(&object_ids, "typology".to_string()))),
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
                    on_change: cad_action("patchSelection", Some(patch_selection_args(&object_ids, "hidden".to_string()))),
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
                    on_change: cad_action("patchSelection", Some(patch_selection_args(&object_ids, "locked".to_string()))),
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
                ui_inspector_vec3_group("cad-play-inspector.object.origin", term_labels.position, &origins, 0.1, move |axis| cad_action("patchSelection", Some(patch_selection_args(&object_ids, format!("origin.{axis}")))))
            },
            {
                let object_ids = object_ids.clone();
                ui_inspector_vec3_group("cad-play-inspector.object.scale", term_labels.scale, &scales, 0.1, move |axis| cad_action("patchSelection", Some(patch_selection_args(&object_ids, format!("scale.{axis}")))))
            },
            inspector_quat_group("cad-play-inspector.object.orientation", term_labels.rotation, &orientations, 0.01, |axis| cad_action("patchSelection", Some(patch_selection_args(&object_ids, format!("orientation.{axis}"))))),
        ],
    }
}

#[cfg(test)]
pub(crate) fn primitive_inspector_group(object: &CadObject, labels: &CadLabels, primitive_id: &str, kind: &str) -> UiInspectorFieldGroup {
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
                let patch_cmd = |field: &str| {
                    cad_action(
                        "patchCadPlayReference",
                        Some(DslValue::object([
                            ("modelDefinitionId".to_string(), DslValue::String(model_definition_id.to_string())),
                            ("referenceId".to_string(), DslValue::String(reference.id.clone())),
                            ("field".to_string(), DslValue::String(field.to_string())),
                        ])),
                    )
                };
                ui_inspector_stepper_field("cad-play-inspector.reference.widthWorld", labels.width_world, &[reference.width_world], 0.1, patch_cmd("widthWorld"))
            },
            {
                let patch_cmd = move |axis: &str| {
                    cad_action(
                        "patchCadPlayReference",
                        Some(DslValue::object([
                            ("modelDefinitionId".to_string(), DslValue::String(model_definition_id.to_string())),
                            ("referenceId".to_string(), DslValue::String(reference.id.clone())),
                            ("field".to_string(), DslValue::String(format!("origin.{axis}"))),
                        ])),
                    )
                };
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
                    on_change: cad_action("renameNode", Some(DslValue::object([("nodeId".to_string(), DslValue::String(node.id.clone()))]))),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
