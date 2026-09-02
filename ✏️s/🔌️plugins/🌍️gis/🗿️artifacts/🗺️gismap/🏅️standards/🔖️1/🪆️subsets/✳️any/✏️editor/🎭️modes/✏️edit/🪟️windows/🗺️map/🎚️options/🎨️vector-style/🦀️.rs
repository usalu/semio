//! 🎨️ Map window option — the colored / figure-ground / inverted-figure vector-style select.

use crate::editor::gis2d::config::Gis2dConfig;
use crate::editor::gis2d::gis2d_window_action;
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Option
pub const GIS2D_VECTOR_STYLE_MEASURE_ID: &str = "gis2d-play-window.vector-style";

pub fn measure(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: GIS2D_VECTOR_STYLE_MEASURE_ID.into(),
        label: Some(labels.vector_style.into()),
        value: cfg.vector_style.clone(),
        items: vec![
            MeasureSelectItem { id: "colored".into(), value: "colored".into(), label: labels.vector_style_colored.into() },
            MeasureSelectItem { id: "figureGround".into(), value: "figureGround".into(), label: labels.vector_style_figure_ground.into() },
            MeasureSelectItem { id: "invertedFigure".into(), value: "invertedFigure".into(), label: labels.vector_style_inverted_figure.into() },
        ],
        on_change: gis2d_window_action("setVectorStyle", None),
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::terminology::gis2d_labels;

    #[semio_framework_async_macros::async_test]
    async fn the_measure_mirrors_the_config_value_and_offers_all_three_styles() {
        let config = Gis2dConfig::default();
        let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&config)) else {
            panic!("vector style is a select measure");
        };
        assert_eq!(value, "colored");
        assert_eq!(items.len(), 3);
    }
}
//#endregion 🧪️Tests
