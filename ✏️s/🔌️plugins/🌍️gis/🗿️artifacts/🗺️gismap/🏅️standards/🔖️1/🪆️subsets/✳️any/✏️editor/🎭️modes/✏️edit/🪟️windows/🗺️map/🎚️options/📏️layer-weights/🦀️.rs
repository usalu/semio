//! 📏️ Map window option — the per-layer stroke-weight slider group.
//!
//! 🧭️ Single owner of the "which layers currently expose a weight slider, and at what value" rule —
//! reused verbatim by the inspector panel's matching slider fields.

use crate::editor::gis2d::config::Gis2dConfig;
use crate::editor::gis2d::gis2d_window_action;
use crate::editor::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use semio_framework_plugin::WindowMeasure;
use semio_framework_surface::tiled_map::{clamp_map_layer_weight, gis_map_layer_weight_slider_ids_json};
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
            on_change: gis2d_window_action("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
