//! 🧬️ Generation2d command — rename generation.

use crate::generation::handle_generation;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "rename-generation")]
pub struct RenameGeneration {
    pub id: String,
    pub name: String,
}

pub fn handle(payload: &RenameGeneration, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let args = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String(payload.id.clone())), ("name".into(), semio_framework_value::DslValue::String(payload.name.clone()))]);
    Ok(handle_generation("renameGeneration", Some(&args), doc, cfg).emit)
}
