//! 🧬️ Generation2d command — select generation.

use crate::generation::handle_generation;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "select-generation")]
pub struct SelectGeneration {
    pub id: Option<String>,
}

pub fn handle(payload: &SelectGeneration, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let args = semio_framework_value::DslValue::object([("id".into(), payload.id.clone().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String))]);
    Ok(handle_generation("selectGeneration", Some(&args), doc, cfg).emit)
}
