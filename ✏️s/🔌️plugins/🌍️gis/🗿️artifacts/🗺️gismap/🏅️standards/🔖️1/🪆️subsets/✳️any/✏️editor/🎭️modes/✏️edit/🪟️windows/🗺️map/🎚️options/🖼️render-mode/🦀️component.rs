//! 🖼️ Map window option — the raster/vector/combined render-mode select.

use crate::editor::gis2d::config::Gis2dConfig;
use crate::editor::gis2d::gis2d_action;
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Option
pub const GIS2D_RENDER_MODE_MEASURE_ID: &str = "gis2d-play-window.render-mode";

pub async fn measure(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: GIS2D_RENDER_MODE_MEASURE_ID.into(),
        label: Some(labels.render_mode.into()),
        value: cfg.render_mode.clone(),
        items: vec![
            MeasureSelectItem { id: "image".into(), value: "image".into(), label: labels.render_mode_image.into() },
            MeasureSelectItem { id: "vector".into(), value: "vector".into(), label: labels.render_mode_vector.into() },
            MeasureSelectItem { id: "combined".into(), value: "combined".into(), label: labels.render_mode_combined.into() },
        ],
        on_change: gis2d_action("setRenderMode", None),
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::terminology::gis2d_labels;

    #[semio_framework_async_macros::async_test]
    async fn the_measure_mirrors_the_config_value_and_offers_all_three_modes() {
        let config = Gis2dConfig::default();
        let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&config)) else {
            panic!("render mode is a select measure");
        };
        assert_eq!(value, "combined");
        assert_eq!(items.len(), 3);
    }
}
//#endregion 🧪️Tests
