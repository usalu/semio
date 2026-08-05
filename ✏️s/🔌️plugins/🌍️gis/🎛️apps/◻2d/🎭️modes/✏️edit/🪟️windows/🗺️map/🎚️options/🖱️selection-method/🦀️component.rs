//! 🖱️ Map window option — the rectangle/lasso marquee-method select.

use crate::apps::gis2d::config::Gis2dConfig;
use crate::apps::gis2d::gis2d_action;
use crate::apps::gis2d::terminology::Gis2dPlayLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Option
pub const GIS2D_SELECTION_METHOD_MEASURE_ID: &str = "gis2d-play-window.selection-method";

pub fn measure(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: GIS2D_SELECTION_METHOD_MEASURE_ID.into(),
        label: Some(labels.selection_method.into()),
        value: cfg.selection_method.clone(),
        items: vec![
            MeasureSelectItem { id: "rectangle".into(), value: "rectangle".into(), label: labels.selection_method_rectangle.into() },
            MeasureSelectItem { id: "lasso".into(), value: "lasso".into(), label: labels.selection_method_lasso.into() },
        ],
        on_change: gis2d_action("setSelectionMethod", None),
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::terminology::gis2d_labels;

    #[test]
    fn the_measure_mirrors_the_config_value_and_offers_both_methods() {
        let config = Gis2dConfig::default();
        let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&config)) else {
            panic!("selection method is a select measure");
        };
        assert_eq!(value, "rectangle");
        assert_eq!(items.len(), 2);
    }
}
//#endregion 🧪️Tests
