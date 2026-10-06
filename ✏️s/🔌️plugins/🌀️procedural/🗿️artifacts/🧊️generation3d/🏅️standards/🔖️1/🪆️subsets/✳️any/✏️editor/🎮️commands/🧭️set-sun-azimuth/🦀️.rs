//! 🌞️ 🌞️ Generation3d play app commands command — `set-sun-azimuth`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "sun-azimuth")]
pub struct SetSunAzimuth {
    pub value: f64,
}

pub fn handle(payload: &SetSunAzimuth, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "setSunAzimuth", Some(&semio_framework_pack_json::json!({ "value": payload.value })));
    Ok(Emit::config(vec![Generation3dConfigMutation::SetSun(crate::editor::generation3d::config::SetSun { json: semio_framework_pack_json::to_json_string(&sun) })]))
}
