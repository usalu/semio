//! 🌑️ Scene-window option — the shadow-enabled toggle.
//! Its command handler lives in `🎮️commands/☀️scene::set_shadow_enabled`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Toggle { id: "shooting.measure.shadow".into(), icon_id: "sun".into(), label: Some(labels.measure_shadow.into()), pressed: snapshot.scene.shadow.enabled, text: None, on_change: crate::editor::shooting::shooting_action("setShadowEnabled", None) }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::config::ShootingConfig;
    use crate::editor::shooting::terminology::shooting_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn shadow_measure_starts_pressed_by_default() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Toggle { pressed, .. } => assert!(pressed),
            other => panic!("shadow measure must be a toggle, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
