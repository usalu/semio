//! 🧭️ Scene-window option — the sun azimuth slider.
//! Its command handler lives in `🎮️commands/☀️scene::set_sun_azimuth`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "shooting.measure.sun-azimuth".into(),
        label: Some(labels.measure_sun_azimuth.into()),
        value: snapshot.scene.sun.azimuth,
        min: 0.0,
        max: 360.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::shooting::shooting_action("setSunAzimuth", None),
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
    async fn sun_azimuth_measure_spans_a_full_turn() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let labels = shooting_play_labels(&ShootingConfig::default());
        match measure(&snapshot, labels) {
            WindowMeasure::Slider { min, max, .. } => assert_eq!((min, max), (0.0, 360.0)),
            other => panic!("sun-azimuth measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
