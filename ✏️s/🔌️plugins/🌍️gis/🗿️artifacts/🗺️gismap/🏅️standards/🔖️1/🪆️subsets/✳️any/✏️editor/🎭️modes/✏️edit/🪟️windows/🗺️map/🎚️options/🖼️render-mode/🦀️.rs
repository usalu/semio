//! 🖼️ Map window option — the raster/vector/combined render-mode select.

use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use crate::editor::gis2d::gis2d_window_action;
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Option
pub const GIS2D_RENDER_MODE_MEASURE_ID: &str = "gis2d-play-window.render-mode";

pub fn measure(cfg: &MapWindowConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: GIS2D_RENDER_MODE_MEASURE_ID.into(),
        label: Some(labels.render_mode.into()),
        value: cfg.render_mode.clone(),
        items: vec![
            MeasureSelectItem { id: "image".into(), value: "image".into(), label: labels.render_mode_image.into() },
            MeasureSelectItem { id: "vector".into(), value: "vector".into(), label: labels.render_mode_vector.into() },
            MeasureSelectItem { id: "combined".into(), value: "combined".into(), label: labels.render_mode_combined.into() },
        ],
        on_change: gis2d_window_action("setRenderMode", None),
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
