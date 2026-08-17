//! 🌞️ 🌞️ Procedural3d play app commands command — `set-sun-intensity`.

use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "sun-intensity")]
pub struct SetSunIntensity {
    pub value: f64}

pub fn handle(payload: &SetSunIntensity, _doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "setSunIntensity", Some(&json!({ "value": payload.value })));
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }]))
}
