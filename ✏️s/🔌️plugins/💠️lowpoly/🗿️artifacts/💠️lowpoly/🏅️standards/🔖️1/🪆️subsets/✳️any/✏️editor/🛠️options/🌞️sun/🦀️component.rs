//! 🌞️ Lowpoly play app — the world-3d sun window-chrome group, delegating to the framework's shared
//! `world3d_sun_measures` builder. Shared verbatim by both windows.

use crate::editor::lowpoly::config::{lowpoly_sun_config, LowpolyConfig};
use crate::editor::lowpoly::lowpoly_window_action;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, _labels: &LowpolyLabels) -> WindowMeasure {
    world3d_sun_measures("lowpoly", &lowpoly_sun_config(config), lowpoly_window_action)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn measure_builds_a_group() {
        let config = LowpolyConfig::default();
        assert!(matches!(measure(&config, semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>("en-US")), WindowMeasure::Group { .. }));
    }
}
//#endregion 🧪️Tests
