//! 🧮️ 🧮️ Generation2d play app commands command — `set-eval-outputs`.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-eval-outputs")]
pub struct SetEvalOutputs {
    pub outputs_json: String,
}

pub fn handle(payload: &SetEvalOutputs, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    session.set_eval_json(std::sync::Arc::new(payload.outputs_json.clone()), doc.retained_grant()?).map_err(|(error, _)| Fault::from(error.to_string()))?;
    Ok(Emit::default())
}

//#region 🧪️Tests
//#endregion 🧪️Tests
