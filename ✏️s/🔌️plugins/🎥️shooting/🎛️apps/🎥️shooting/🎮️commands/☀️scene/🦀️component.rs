//! ☀️ Shooting play app commands — scene-lighting setters (sun, ambient, material, shadow). All real,
//! undoable document mutations via `ShootingMutation::PatchScene`.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingSnapshot, ShootingScenePatch};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSunAzimuth
pub mod set_sun_azimuth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-azimuth")]
    pub struct SetSunAzimuth {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunAzimuth, _doc: &DocumentView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::PatchScene { patch: ShootingScenePatch { sun_azimuth: Some(payload.value), ..Default::default() } }]))
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

    pub fn handle(payload: &SetSunElevation, _doc: &DocumentView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::PatchScene { patch: ShootingScenePatch { sun_elevation: Some(payload.value), ..Default::default() } }]))
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

    pub fn handle(payload: &SetSunIntensity, _doc: &DocumentView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::PatchScene { patch: ShootingScenePatch { sun_intensity: Some(payload.value), ..Default::default() } }]))
    }
}
//#endregion 🔖️SetSunIntensity

//#region 🔖️SetAmbientIntensity
pub mod set_ambient_intensity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "ambient-intensity")]
    pub struct SetAmbientIntensity {
        pub value: f64,
    }

    pub fn handle(payload: &SetAmbientIntensity, _doc: &DocumentView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::PatchScene { patch: ShootingScenePatch { ambient_intensity: Some(payload.value), ..Default::default() } }]))
    }
}
//#endregion 🔖️SetAmbientIntensity

//#region 🔖️SetMaterialRoughness
pub mod set_material_roughness {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "material-roughness")]
    pub struct SetMaterialRoughness {
        pub value: f64,
    }

    pub fn handle(payload: &SetMaterialRoughness, _doc: &DocumentView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::PatchScene { patch: ShootingScenePatch { material_roughness: Some(payload.value), ..Default::default() } }]))
    }
}
//#endregion 🔖️SetMaterialRoughness

//#region 🔖️SetShadowEnabled
pub mod set_shadow_enabled {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "shadow-enabled")]
    pub struct SetShadowEnabled {
        pub value: bool,
    }

    pub fn handle(payload: &SetShadowEnabled, _doc: &DocumentView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::PatchScene { patch: ShootingScenePatch { shadow_enabled: Some(payload.value), ..Default::default() } }]))
    }
}
//#endregion 🔖️SetShadowEnabled

//#region 🔖️ToggleSun
pub mod toggle_sun {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-sun")]
    pub struct ToggleSun {
        pub value: bool,
    }

    pub fn handle(payload: &ToggleSun, _doc: &DocumentView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::PatchScene { patch: ShootingScenePatch { sun_enabled: Some(payload.value), ..Default::default() } }]))
    }
}
//#endregion 🔖️ToggleSun

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn scene_setters_mutate_lighting() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }));
        dispatch(&mut app, ShootingCommand::SetShadowEnabled(set_shadow_enabled::SetShadowEnabled { value: false }));
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.scene.sun.azimuth, 90.0);
        assert!(!snapshot.scene.shadow.enabled);
    }

    #[test]
    fn toggle_sun_round_trips_through_ops_and_defaults_off() {
        let mut app = shooting_app();
        assert!(!app.snapshot().expect("snapshot").scene.sun.enabled);
        dispatch(&mut app, ShootingCommand::ToggleSun(toggle_sun::ToggleSun { value: true }));
        assert!(app.snapshot().expect("snapshot").scene.sun.enabled);
    }
}
//#endregion 🧪️Tests
