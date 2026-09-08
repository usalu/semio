//! 🖌️ Lowpoly play app — the brush utility's size/opacity/hardness sliders. Utility-tagged
//! (`active_utility_id: Some("brush")`) so `partition_window_measures` surfaces this group in the
//! Utility Options rail only while the brush utility is active. Shared verbatim by both windows.

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::utility_params_value;
use semio_framework_plugin::WindowMeasure;

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    crate::editor::lowpoly::paint_utility_params_group("brush", &utility_params_value(config), labels)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
