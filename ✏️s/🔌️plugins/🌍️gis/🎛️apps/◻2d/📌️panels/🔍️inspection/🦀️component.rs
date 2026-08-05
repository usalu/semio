//! 🔍️ GIS 2D play app panel — the inspector: map-view settings plus the selected layer's fields.

use crate::apps::gis2d::config::{layer_visible, Gis2dConfig};
use crate::apps::gis2d::modes::edit::windows::map::options::{layer_weights, lod_mode};
use crate::apps::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use crate::apps::gis2d::{gis2d_action, GIS_MAP_LAYER_IDS};
use crate::artifacts::gismap::GIS_MAP_SCHEMA;
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_mixed_toggle, ui_inspector_readonly_field, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode,
    UiSliderNode, UiToggleNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const GIS2D_PLAY_BODY_INSPECTION: &str = "gis2d.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(GIS2D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📏️ The inspector's stroke-weight sliders — the same `(layer, label, weight)` rows the map window's
/// `🎚️options/📏️layer-weights` group is built from, rendered as inspector fields.
fn layer_weight_slider_fields(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> Vec<UiNode> {
    layer_weights::layer_weight_entries(cfg, labels)
        .into_iter()
        .map(|(layer_id, label, value)| {
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("gis2d-play-inspector.weight.{layer_id}"),
                label: Label::data(format!("{label} {}", labels.weight_suffix.as_str())),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: format!("gis2d-play-inspector.weight.{layer_id}.slider"),
                    value,
                    min: 0.25,
                    max: 3.0,
                    step: 0.05,
                    on_change: gis2d_action("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            })
        })
        .collect()
}

fn map_view_field_group(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiInspectorFieldGroup {
    let lod_items: Vec<UiSelectItem> = lod_mode::lod_select_entries(labels).into_iter().map(|(value, label)| UiSelectItem { value, label: Label::data(label) }).collect();
    let selection: Value = serde_json::from_str(&cfg.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let selected_count = selection.get("positions").and_then(|value| value.as_array()).map_or(0, Vec::len) + selection.get("routes").and_then(|value| value.as_array()).map_or(0, Vec::len);
    let mut fields = vec![
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.render-mode".into(),
            label: labels.render_mode.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.render-mode.select".into(),
                value: cfg.render_mode.clone(),
                items: vec![
                    UiSelectItem { value: "image".into(), label: labels.render_mode_image.into() },
                    UiSelectItem { value: "vector".into(), label: labels.render_mode_vector.into() },
                    UiSelectItem { value: "combined".into(), label: labels.render_mode_combined.into() },
                ],
                placeholder: None,
                on_change: gis2d_action("setRenderMode", None),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            menu: None,
        }),
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.vector-style".into(),
            label: labels.vector_style.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.vector-style.select".into(),
                value: cfg.vector_style.clone(),
                items: vec![
                    UiSelectItem { value: "colored".into(), label: labels.vector_style_colored.into() },
                    UiSelectItem { value: "figureGround".into(), label: labels.vector_style_figure_ground.into() },
                    UiSelectItem { value: "invertedFigure".into(), label: labels.vector_style_inverted_figure.into() },
                ],
                placeholder: None,
                on_change: gis2d_action("setVectorStyle", None),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            menu: None,
        }),
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.lod-mode".into(),
            label: labels.lod_mode.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.lod-mode.select".into(),
                value: cfg.lod_mode.clone(),
                items: lod_items,
                placeholder: None,
                on_change: gis2d_action("setLodMode", None),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            menu: None,
        }),
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.selection-method".into(),
            label: labels.selection_method.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.selection-method.select".into(),
                value: cfg.selection_method.clone(),
                items: vec![UiSelectItem { value: "rectangle".into(), label: labels.selection_method_rectangle.into() }, UiSelectItem { value: "lasso".into(), label: labels.selection_method_lasso.into() }],
                placeholder: None,
                on_change: gis2d_action("setSelectionMethod", None),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            menu: None,
        }),
        ui_inspector_readonly_field("gis2d-play-inspector.feature-selection", labels.selected_features, selected_count.to_string()),
    ];
    fields.extend(layer_weight_slider_fields(cfg, labels));
    UiInspectorFieldGroup { presence: UiPresence::default(), id: "gis2d-play-inspector.map-view".into(), label: labels.map_view.into(), default_open: Some(true), fields }
}

pub fn render(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let map_view_group = map_view_field_group(cfg, labels);
    if cfg.selected_ids.is_empty() {
        let visible_count = GIS_MAP_LAYER_IDS.iter().filter(|(id, _, _)| layer_visible(cfg, id)).count();
        return ui_inspector_groups_to_tree(&[
            map_view_group,
            UiInspectorFieldGroup {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.summary".into(),
                label: labels.map_layer.into(),
                default_open: Some(true),
                fields: vec![
                    ui_inspector_readonly_field("gis2d-play-inspector.schema", labels.schema, GIS_MAP_SCHEMA.to_string()),
                    ui_inspector_readonly_field("gis2d-play-inspector.visible-count", labels.layers_visible, format!("{visible_count}/{}", GIS_MAP_LAYER_IDS.len())),
                ],
            },
        ]);
    }
    let layer_id = &cfg.selected_ids[0];
    let label = gis2d_layer_label(layer_id, labels);
    let visible = layer_visible(cfg, layer_id);
    let mixed = ui_inspector_mixed_toggle(&[visible]);
    ui_inspector_groups_to_tree(&[
        map_view_group,
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.layer".into(),
            label: labels.map_layer.into(),
            default_open: Some(true),
            fields: vec![
                ui_inspector_readonly_field("gis2d-play-inspector.id", labels.field_id, layer_id.clone()),
                ui_inspector_readonly_field("gis2d-play-inspector.label", labels.field_label, label.to_string()),
                UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
                    id: "gis2d-play-inspector.visible".into(),
                    label: labels.field_visible.into(),
                    child: Box::new(UiNode::Toggle(UiToggleNode {
                        id: "gis2d-play-inspector.visible.toggle".into(),
                        icon_id: "eye".into(),
                        presence: UiPresence::selected(mixed.uniform && mixed.pressed),
                        text: None,
                        on_change: gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": layer_id }))),
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }),
            ],
        },
    ])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::commands::selection::set_selection;
    use crate::apps::gis2d::testkit::{app, dispatch, render as render_body};
    use crate::apps::gis2d::Gis2dCommand;

    #[test]
    fn the_empty_selection_inspector_summarises_the_schema_and_visible_count() {
        let mut app = app();
        let json = render_body(&mut app, GIS2D_PLAY_BODY_INSPECTION);
        assert!(json.contains(GIS_MAP_SCHEMA));
        assert!(json.contains(&format!("{}/{}", GIS_MAP_LAYER_IDS.len(), GIS_MAP_LAYER_IDS.len())));
    }

    #[test]
    fn selecting_a_layer_swaps_the_summary_group_for_that_layers_fields() {
        let mut app = app();
        dispatch(&mut app, Gis2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["roads".into()] }));
        let json = render_body(&mut app, GIS2D_PLAY_BODY_INSPECTION);
        assert!(json.contains("gis2d-play-inspector.visible.toggle"));
        assert!(!json.contains("gis2d-play-inspector.visible-count"));
    }

    #[test]
    fn the_definition_binds_the_framework_inspection_tab_to_this_body() {
        let definition = definition();
        assert!(matches!(definition.group, PanelGroup::Details));
        assert_eq!(definition.body_key.as_deref(), Some(GIS2D_PLAY_BODY_INSPECTION));
    }
}
//#endregion 🧪️Tests
