//! 🎯️ Scene-window option — "center model in viewport" toggle.
//! Its command handler lives in `🎮️commands/📦️asset` (config `center_model`, no dedicated command —
//! toggled directly through the `setCenterModel` view action, not modeled in this migration's command
//! surface split since it predates this measure; see the app's `🎭️modes` doc comment).

use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Toggle { id: "shooting.measure.center-model".into(), icon_id: "focus".into(), label: Some(labels.measure_center_model.into()), pressed: true, text: None, on_change: crate::editor::shooting::shooting_window_action("setCenterModel", None) }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
