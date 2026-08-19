//! ☀️ Process 3d play app commands — the scene sun (config-only, ephemeral view state).

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleSun
pub mod toggle_sun {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-sun")]
    pub struct ToggleSun {}

    pub async fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let config = cfg.snapshot;
        Ok(Emit::config(vec![Process3dConfigMutation::SetSun { enabled: !config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }]))
    }
}
//#endregion 🔖️ToggleSun

//#region 🔖️SetSunAzimuth
pub mod set_sun_azimuth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-azimuth")]
    pub struct SetSunAzimuth {
        pub value: f64,
    }

    pub async fn handle(payload: &SetSunAzimuth, _doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let config = cfg.snapshot;
        Ok(Emit::config(vec![Process3dConfigMutation::SetSun { enabled: config.sun_enabled, azimuth: payload.value, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }]))
    }
}
//#endregion 🔖️SetSunAzimuth

//#region 🔖️SetSunElevation
pub mod set_sun_elevation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-elevation")]
    pub struct SetSunElevation {
        pub value: f64,
    }

    pub async fn handle(payload: &SetSunElevation, _doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let config = cfg.snapshot;
        Ok(Emit::config(vec![Process3dConfigMutation::SetSun { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: payload.value, intensity: config.sun_intensity, color: config.sun_color.clone() }]))
    }
}
//#endregion 🔖️SetSunElevation

//#region 🔖️SetSunIntensity
pub mod set_sun_intensity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-intensity")]
    pub struct SetSunIntensity {
        pub value: f64,
    }

    pub async fn handle(payload: &SetSunIntensity, _doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let config = cfg.snapshot;
        Ok(Emit::config(vec![Process3dConfigMutation::SetSun { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: payload.value, color: config.sun_color.clone() }]))
    }
}
//#endregion 🔖️SetSunIntensity
