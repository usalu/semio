//! 🌞️ 🌞️ Procedural3d play app commands command — `set-sun-elevation`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
use serde_json::json;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "sun-elevation")]
pub struct SetSunElevation {
    pub value: f64,
}

pub fn handle(payload: &SetSunElevation, _doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "setSunElevation", Some(&json!({ "value": payload.value })));
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetSun { json: dsl::json::to_json_string(&sun) }]))
}
