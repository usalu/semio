//! 🎯️ Scene-window option — "center model in viewport" toggle.
//! Its command handler lives in `🎮️commands/📦️asset` (config `center_model`, no dedicated command —
//! toggled directly through the `setCenterModel` view action, not modeled in this migration's command
//! surface split since it predates this measure; see the app's `🎭️modes` doc comment).

use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Toggle { id: "shooting.measure.center-model".into(), icon_id: "focus".into(), label: Some(labels.measure_center_model.into()), pressed: true, text: None, on_change: crate::editor::shooting::shooting_action("setCenterModel", None) }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::config::ShootingConfig;
    use crate::editor::shooting::terminology::shooting_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn center_model_toggle_starts_pressed() {
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(labels) {
            WindowMeasure::Toggle { pressed, .. } => assert!(pressed),
            other => panic!("center-model measure must be a toggle, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
