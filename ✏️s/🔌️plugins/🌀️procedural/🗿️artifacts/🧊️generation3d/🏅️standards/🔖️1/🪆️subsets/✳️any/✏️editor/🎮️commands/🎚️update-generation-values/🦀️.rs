//! 🧬️ Generation3d command — `update-generation-values`: the ABSOLUTE answer one question takes in a generation. A
//! continuous form control (slider, held spinner) rides the framework scrub machine (design §13.1): every tick re-derives
//! this one absolute leaf and the release commits ONE edit, so the handler never reads a gesture.

use crate::editor::generation3d::commands::generation::generation_command_result;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "update-generation-values")]
pub struct UpdateGenerationValues {
    pub generation_id: Option<String>,
    pub question_id: String,
    pub value: semio_framework_value::DslValue,
}

pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let generation_id = payload.generation_id.clone().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String);
    let args = semio_framework_value::DslValue::object([("generationId".to_string(), generation_id), ("questionId".to_string(), semio_framework_value::DslValue::String(payload.question_id.clone())), ("value".to_string(), payload.value.clone())]);
    Ok(generation_command_result("updateGenerationValues", Some(&args), doc.snapshot, cfg.snapshot).map(|result| result.emit).unwrap_or_default())
}
