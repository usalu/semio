//! ☀️ Scene-window option — the sun-enabled toggle.
//! Its command handler lives in `🎮️commands/☀️scene::toggle_sun`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Toggle { id: "shooting.measure.sun-enabled".into(), icon_id: "sun".into(), label: Some(labels.measure_sun.into()), pressed: snapshot.scene.sun.enabled, text: None, on_change: crate::apps::shooting::shooting_action("toggleSun", None) }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::terminology::shooting_play_labels;
    use crate::apps::shooting::config::ShootingConfig;

    #[test]
    fn sun_enabled_measure_mirrors_the_fixture_default_off() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Toggle { pressed, .. } => assert!(!pressed),
            other => panic!("sun-enabled measure must be a toggle, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
