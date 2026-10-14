//! 🌞️ 🌞️ Generation3d play app commands command — `toggle-sun`.

use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "toggle-sun")]
pub struct ToggleSun {}

pub fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "toggleSun", None);
    Ok(Emit::config(vec![Generation3dConfigMutation::SetSun(semio_s_artifact_procedural_3d_editor_config::SetSun { json: semio_framework_pack_json::to_json_string(&sun) })]))
}
