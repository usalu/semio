//! 🌞️ Lowpoly play app — the world-3d sun window-chrome group, delegating to the framework's shared
//! `world3d_sun_measures` builder. Shared verbatim by both windows.

use crate::apps::lowpoly::config::{lowpoly_sun_config, LowpolyConfig};
use crate::apps::lowpoly::lowpoly_action;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use semio_framework_plugin::{world3d_sun_measures, WindowMeasure};

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, _labels: &LowpolyLabels) -> WindowMeasure {
    world3d_sun_measures("lowpoly", &lowpoly_sun_config(config), lowpoly_action)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_builds_a_group() {
        let config = LowpolyConfig::default();
        assert!(matches!(measure(&config, &LowpolyLabels::NATIVE_EN), WindowMeasure::Group { .. }));
    }
}
//#endregion 🧪️Tests
