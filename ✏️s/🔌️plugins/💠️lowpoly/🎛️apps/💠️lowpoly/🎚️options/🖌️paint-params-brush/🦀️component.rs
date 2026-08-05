//! 🖌️ Lowpoly play app — the brush utility's size/opacity/hardness sliders. Utility-tagged
//! (`active_utility_id: Some("brush")`) so `partition_window_measures` surfaces this group in the
//! Utility Options rail only while the brush utility is active. Shared verbatim by both windows.

use crate::apps::lowpoly::config::LowpolyConfig;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use crate::apps::lowpoly::view::utility_params_value;
use semio_framework_plugin::WindowMeasure;

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    crate::apps::lowpoly::paint_utility_params_group("brush", &utility_params_value(config), labels)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_is_tagged_for_the_brush_utility() {
        let m = measure(&LowpolyConfig::default(), &LowpolyLabels::NATIVE_EN);
        match m {
            WindowMeasure::Group { active_utility_id, .. } => assert_eq!(active_utility_id, Some("brush".to_string())),
            other => panic!("expected Group, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
