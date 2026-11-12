//! 🧮️ 🧮️ Generation2d play app commands command — `set-eval-outputs`.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_os_flow::FlowEvalSession;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-eval-outputs")]
pub struct SetEvalOutputs {
    pub outputs_json: String,
}

/// 🧮️ Publishing an evaluation replaces the session's shared original under the turn's original effect, which this marks-free fallback does not own, so it refuses.
pub fn handle(_payload: &SetEvalOutputs, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    Err(Fault::from("generation2d-set-eval-outputs-requires-retained-route"))
}

//#region 🧪️Tests
//#endregion 🧪️Tests
