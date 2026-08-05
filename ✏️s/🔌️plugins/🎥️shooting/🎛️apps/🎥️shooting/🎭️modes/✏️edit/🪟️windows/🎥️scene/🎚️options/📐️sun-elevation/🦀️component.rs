//! 📐️ Scene-window option — the sun elevation slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_sun_elevation`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingFixture;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(fixture: &ShootingFixture, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.sun-elevation".into(),
        label: Some(labels.measure_sun_elevation.into()),
        value: fixture.scene.sun.elevation,
        min: -10.0,
        max: 90.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::apps::shooting::shooting_action("setSunElevation", None),
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
    fn sun_elevation_measure_allows_below_horizon() {
        let fixture = crate::artifacts::shooting::engine::default_fixture();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&fixture, labels) {
            WindowMeasure::Slider { min, max, .. } => assert_eq!((min, max), (-10.0, 90.0)),
            other => panic!("sun-elevation measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
