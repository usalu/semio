//! 🌑️ Scene-window option — the shadow-enabled toggle.
//! Its command handler lives in `🎮️commands/☀️scene::set_shadow_enabled`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingFixture;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(fixture: &ShootingFixture, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Toggle { id: "shooting.measure.shadow".into(), icon_id: "sun".into(), label: Some(labels.measure_shadow.into()), pressed: fixture.scene.shadow.enabled, text: None, on_change: crate::apps::shooting::shooting_action("setShadowEnabled", None) }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::config::ShootingConfig;
    use crate::apps::shooting::terminology::shooting_play_labels;

    #[test]
    fn shadow_measure_starts_pressed_by_default() {
        let fixture = crate::artifacts::shooting::engine::default_fixture();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&fixture, labels) {
            WindowMeasure::Toggle { pressed, .. } => assert!(pressed),
            other => panic!("shadow measure must be a toggle, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
