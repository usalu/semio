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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
