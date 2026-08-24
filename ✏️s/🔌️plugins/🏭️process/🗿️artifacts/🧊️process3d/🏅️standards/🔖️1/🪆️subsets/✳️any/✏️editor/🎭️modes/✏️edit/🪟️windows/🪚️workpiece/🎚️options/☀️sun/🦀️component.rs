//! ☀️ Workpiece-window option — the scene sun measure group (toggle + azimuth/elevation/intensity),
//! built entirely by the shared SDK helper `world3d_sun_measures`. Its command handlers live in
//! `🎮️commands/☀️sun`.

use semio_framework_plugin::{world3d_sun_measures, ActionDescriptor, WindowMeasure, WorldSunConfig};

//#region 🔖️Measure
pub fn measure(sun: &WorldSunConfig) -> WindowMeasure {
    world3d_sun_measures("process3d", sun, |action, args| ActionDescriptor {
        controller_id: crate::editor::process3d::PROCESS_3D_PLAY_APP_ID.into(),
        action: action.into(),
        args: semio_framework::optional_json_to_dsl(args),
    })
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sun_off_yields_an_unpressed_toggle_child() {
        let sun = WorldSunConfig { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() };
        let group = measure(&sun);
        assert!(matches!(&group, WindowMeasure::Group { children, .. } if children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if !*pressed))));
    }

    #[semio_framework_async_macros::async_test]
    async fn sun_on_yields_a_pressed_toggle_child() {
        let sun = WorldSunConfig { enabled: true, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() };
        let group = measure(&sun);
        assert!(matches!(&group, WindowMeasure::Group { children, .. } if children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if *pressed))));
    }
}
//#endregion 🧪️Tests
