//! 🧽️ Lowpoly play app — the eraser utility's size/opacity/hardness sliders. Utility-tagged
//! (`active_utility_id: Some("eraser")`) so `partition_window_measures` surfaces this group in the
//! Utility Options rail only while the eraser utility is active. Shared verbatim by both windows.

use semio_s_artifact_lowpoly_lowpoly_config::LowpolyConfig;
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::LowpolyLabels;
use semio_s_artifact_lowpoly_lowpoly_view::utility_params_value;
use semio_framework_plugin::WindowMeasure;

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    semio_s_artifact_lowpoly_lowpoly_kit::paint_utility_params_group("eraser", &utility_params_value(config), labels)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
