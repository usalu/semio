//! 🌞️ Procedural3d play app commands — the 3D preview sun toggle/azimuth/elevation/intensity display
//! options (config-only; never document operations).

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️ToggleSun
pub mod toggle_sun {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-sun")]
    pub struct ToggleSun {}

    pub fn handle(_payload: &ToggleSun, _doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let mut sun = cfg.snapshot.sun();
        apply_world3d_sun_action(&mut sun, "toggleSun", None);
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }]))
    }
}
//#endregion 🔖️ToggleSun

//#region 🔖️SetSunAzimuth
pub mod set_sun_azimuth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-azimuth")]
    pub struct SetSunAzimuth {
        pub value: f64}

    pub fn handle(payload: &SetSunAzimuth, _doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let mut sun = cfg.snapshot.sun();
        apply_world3d_sun_action(&mut sun, "setSunAzimuth", Some(&json!({ "value": payload.value })));
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }]))
    }
}
//#endregion 🔖️SetSunAzimuth

//#region 🔖️SetSunElevation
pub mod set_sun_elevation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-elevation")]
    pub struct SetSunElevation {
        pub value: f64}

    pub fn handle(payload: &SetSunElevation, _doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let mut sun = cfg.snapshot.sun();
        apply_world3d_sun_action(&mut sun, "setSunElevation", Some(&json!({ "value": payload.value })));
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }]))
    }
}
//#endregion 🔖️SetSunElevation

//#region 🔖️SetSunIntensity
pub mod set_sun_intensity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-intensity")]
    pub struct SetSunIntensity {
        pub value: f64}

    pub fn handle(payload: &SetSunIntensity, _doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let mut sun = cfg.snapshot.sun();
        apply_world3d_sun_action(&mut sun, "setSunIntensity", Some(&json!({ "value": payload.value })));
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }]))
    }
}
//#endregion 🔖️SetSunIntensity

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn toggle_sun_never_mutates_the_document() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural3dCommand::ToggleSun(toggle_sun::ToggleSun {}));
        assert_eq!(app.snapshot().expect("snapshot"), before, "toggleSun must not mutate the document");
    }
}
//#endregion 🧪️Tests
