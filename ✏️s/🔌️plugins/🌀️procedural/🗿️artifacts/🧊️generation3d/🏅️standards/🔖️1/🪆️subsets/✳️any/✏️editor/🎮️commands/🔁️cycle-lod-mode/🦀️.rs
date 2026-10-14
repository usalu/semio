//! 🔁️ 🎚️ Generation3d play app commands command — `cycle-lod-mode`.

use semio_s_artifact_procedural_3d_editor_config::next_lod_mode;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔁️ The level-of-detail twin of `cycleShowMode` — same argument-free contract, same ladder
/// discipline (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "cycle-lod-mode")]
pub struct CycleLodMode {}

pub fn handle(_payload: &CycleLodMode, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation3dConfigMutation::SetLodMode(semio_s_artifact_procedural_3d_editor_config::SetLodMode { value: next_lod_mode(&cfg.snapshot.lod_mode) })]))
}
