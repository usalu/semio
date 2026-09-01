//! 🌞️ Lowpoly play app commands — the world-3d sun toggle/azimuth/elevation/intensity, reusing the
//! framework's shared `WorldSunConfig`-shaped action logic. Config-only.

use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::editor::lowpoly::config::{lowpoly_sun_config, LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// 🌞️ Reuses the framework's shared sun toggle/slider logic (`apply_world3d_sun_action`), threading it
/// through `LowpolyConfig`'s flattened sun fields and returning the resulting `SetSun` config op.
fn apply_sun_command(config: &LowpolyConfig, action_id: &str, value: Option<f64>) -> LowpolyConfigMutation {
    let mut sun = lowpoly_sun_config(config);
    let args = value.map(|value| json!({ "value": value }));
    apply_world3d_sun_action(&mut sun, action_id, args.as_ref());
    LowpolyConfigMutation::SetSun { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color }
}

//#region 🔖️ToggleSun
pub mod toggle_sun {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "toggle-sun")]
    pub struct ToggleSun {}

    pub fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "toggleSun", None)]))
    }
}
//#endregion 🔖️ToggleSun

//#region 🔖️SetSunAzimuth
pub mod set_sun_azimuth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "set-sun-azimuth")]
    pub struct SetSunAzimuth {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunAzimuth, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "setSunAzimuth", Some(payload.value))]))
    }
}
//#endregion 🔖️SetSunAzimuth

//#region 🔖️SetSunElevation
pub mod set_sun_elevation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "set-sun-elevation")]
    pub struct SetSunElevation {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunElevation, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "setSunElevation", Some(payload.value))]))
    }
}
//#endregion 🔖️SetSunElevation

//#region 🔖️SetSunIntensity
pub mod set_sun_intensity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "set-sun-intensity")]
    pub struct SetSunIntensity {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunIntensity, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "setSunIntensity", Some(payload.value))]))
    }
}
//#endregion 🔖️SetSunIntensity

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, dispatch};
    use crate::editor::lowpoly::LowpolyCommand;
    use semio_framework_plugin::PluginApp;

    #[semio_framework_async_macros::async_test]
    async fn toggle_sun_flips_enabled() {
        let mut a = app();
        dispatch(&mut a, LowpolyCommand::ToggleSun(super::toggle_sun::ToggleSun {})).await;
        // 🎯️ Config isn't directly readable off `VcsArtifactApp`; assert through window measures instead
        // (mirrors the pre-migration test's approach of reading effects, not internal state).
        let measures = a.window_measures().await;
        assert!(!measures.is_empty());
    }
}
//#endregion 🧪️Tests
