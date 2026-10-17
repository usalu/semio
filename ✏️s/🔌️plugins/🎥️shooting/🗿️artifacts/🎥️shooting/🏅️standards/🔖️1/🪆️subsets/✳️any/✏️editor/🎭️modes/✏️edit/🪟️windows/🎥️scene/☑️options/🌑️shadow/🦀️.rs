//! 🌑️ Scene-window option — the shadow-enabled toggle.
//! Its command handler lives in `🎮️commands/☀️scene::set_shadow_enabled`.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::ShootingSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: "shooting.measure.shadow".into(),
        icon_id: "sun".into(),
        label: Some(labels.measure_shadow.into()),
        pressed: snapshot.scene.shadow.enabled,
        text: None,
        on_change: crate::editor::shooting::ui_helpers_and_io::shooting_window_action("setShadowEnabled", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests

//#endregion 🧪️Tests
