//! 🌫️ Scene-window option — the ambient-intensity slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_ambient_intensity`.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingFixture;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(fixture: &ShootingFixture, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.ambient".into(),
        label: Some(labels.measure_ambient.into()),
        value: fixture.scene.ambient.intensity,
        min: 0.0,
        max: 3.0,
        step: Some(0.05),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::apps::shooting::shooting_action("setAmbientIntensity", None),
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
    fn ambient_measure_matches_the_fixture_default() {
        let fixture = crate::artifacts::shooting::engine::default_fixture();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&fixture, labels) {
            WindowMeasure::Slider { value, .. } => assert_eq!(value, 1.15),
            other => panic!("ambient measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
