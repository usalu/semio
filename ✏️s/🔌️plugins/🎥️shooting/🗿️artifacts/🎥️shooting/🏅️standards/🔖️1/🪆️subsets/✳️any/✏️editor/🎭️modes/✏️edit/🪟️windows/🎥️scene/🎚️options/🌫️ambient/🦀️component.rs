//! 🌫️ Scene-window option — the ambient-intensity slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_ambient_intensity`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.ambient".into(),
        label: Some(labels.measure_ambient.into()),
        value: snapshot.scene.ambient.intensity,
        min: 0.0,
        max: 3.0,
        step: Some(0.05),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::shooting::shooting_action("setAmbientIntensity", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::config::ShootingConfig;
    use crate::editor::shooting::terminology::shooting_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn ambient_measure_matches_the_fixture_default() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Slider { value, .. } => assert_eq!(value, 1.15),
            other => panic!("ambient measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
