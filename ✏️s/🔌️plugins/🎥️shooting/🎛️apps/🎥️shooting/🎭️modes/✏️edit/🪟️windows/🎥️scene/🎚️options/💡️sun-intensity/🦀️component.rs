//! 💡️ Scene-window option — the sun intensity slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_sun_intensity`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.sun-intensity".into(),
        label: Some(labels.measure_sun_intensity.into()),
        value: snapshot.scene.sun.intensity,
        min: 0.0,
        max: 5.0,
        step: Some(0.1),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::apps::shooting::shooting_action("setSunIntensity", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::config::ShootingConfig;
    use crate::apps::shooting::terminology::shooting_play_labels;

    #[test]
    fn sun_intensity_measure_matches_the_fixture_default() {
        let snapshot = crate::artifacts::shooting::engine::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Slider { value, .. } => assert_eq!(value, 2.4),
            other => panic!("sun-intensity measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
