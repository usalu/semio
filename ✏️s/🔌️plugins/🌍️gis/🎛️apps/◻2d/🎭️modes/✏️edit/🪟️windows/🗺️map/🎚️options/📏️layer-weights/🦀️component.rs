//! 📏️ Map window option — the per-layer stroke-weight slider group.
//!
//! 🧭️ Single owner of the "which layers currently expose a weight slider, and at what value" rule —
//! reused verbatim by the inspector panel's matching slider fields.

use crate::apps::gis2d::config::Gis2dConfig;
use crate::apps::gis2d::gis2d_action;
use crate::apps::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use framework_surface_tiled_map::{clamp_map_layer_weight, gis_map_layer_weight_slider_ids_json};
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

//#region 🔖️Vocabulary
pub const GIS2D_LAYER_WEIGHTS_MEASURE_ID: &str = "gis2d-play-window.layer-weights";

/// 📏️ `(layer_id, label, weight)` for every layer the current LOD/render mode exposes a weight
/// slider for; a layer with no explicit entry sits at `1.0`.
pub fn layer_weight_entries(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> Vec<(String, String, f64)> {
    let ids: Vec<String> = serde_json::from_str(&gis_map_layer_weight_slider_ids_json(&cfg.lod_mode, &cfg.render_mode)).unwrap_or_default();
    ids.into_iter()
        .map(|layer_id| {
            let value = cfg.layer_stroke_scale.get(&layer_id).copied().map_or(1.0, clamp_map_layer_weight);
            let label = gis2d_layer_label(&layer_id, labels).to_string();
            (layer_id, label, value)
        })
        .collect()
}
//#endregion 🔖️Vocabulary

//#region 🔖️Option
pub fn measure(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    let children: Vec<WindowMeasure> = layer_weight_entries(cfg, labels)
        .into_iter()
        .map(|(layer_id, label, value)| WindowMeasure::Slider {
            id: format!("gis2d-play-window.weight.{layer_id}"),
            label: Some(format!("{label} {}", labels.weight_suffix.as_str())),
            value,
            min: 0.25,
            max: 3.0,
            step: Some(0.05),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: gis2d_action("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),
            waiting: None,
        })
        .collect();
    WindowMeasure::Group {
        id: GIS2D_LAYER_WEIGHTS_MEASURE_ID.into(),
        label: labels.layer_weights_group.into(),
        default_open: Some(false),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children,
        active_utility_id: None,
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::terminology::gis2d_labels;

    #[test]
    fn weight_entries_default_to_one_and_honour_explicit_overrides() {
        let mut config = Gis2dConfig::default();
        let labels = gis2d_labels(&config);
        let defaults = layer_weight_entries(&config, labels);
        assert!(defaults.iter().all(|(_, _, value)| *value == 1.0));
        let Some((first_id, _, _)) = defaults.first().cloned() else { return };
        config.layer_stroke_scale.insert(first_id.clone(), 2.0);
        let overridden = layer_weight_entries(&config, gis2d_labels(&config));
        assert_eq!(overridden.iter().find(|(id, _, _)| id == &first_id).map(|(_, _, value)| *value), Some(2.0));
    }

    #[test]
    fn the_group_is_collapsed_by_default_and_mirrors_the_entry_list() {
        let config = Gis2dConfig::default();
        let labels = gis2d_labels(&config);
        let WindowMeasure::Group { children, default_open, .. } = measure(&config, labels) else {
            panic!("layer weights is a group measure");
        };
        assert_eq!(default_open, Some(false));
        assert_eq!(children.len(), layer_weight_entries(&config, labels).len());
    }
}
//#endregion 🧪️Tests
