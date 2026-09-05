//! 🌞️ 🌞️ Generation3d play app commands command — `set-sun-elevation`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "sun-elevation")]
pub struct SetSunElevation {
    pub value: f64,
}

pub fn handle(payload: &SetSunElevation, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "setSunElevation", Some(&dsl::json!({ "value": payload.value })));
    Ok(Emit::config(vec![Generation3dConfigMutation::SetSun { json: dsl::json::to_json_string(&sun) }]))
}
