//! 🧬️ 🧬️ Generation3d play app commands command — `update-generation-values`.

use crate::editor::generation3d::commands::generation::generation_command_result;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️AddGeneration
//#endregion 🔖️AddGeneration

//#region 🔖️RemoveGeneration
//#endregion 🔖️RemoveGeneration

//#region 🔖️RenameGeneration
//#endregion 🔖️RenameGeneration

//#region 🔖️UpdateGenerationValues
//#endregion 🔖️UpdateGenerationValues

//#region 🔖️SelectGeneration
//#endregion 🔖️SelectGeneration

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "update-generation-values")]
pub struct UpdateGenerationValues {
    pub generation_id: Option<String>,
    pub question_id: String,
    pub value: dsl::DslValue,
}

pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);
    let args = dsl::DslValue::object([("generationId".to_string(), generation_id), ("questionId".to_string(), dsl::DslValue::String(payload.question_id.clone())), ("value".to_string(), payload.value.clone())]);
    Ok(generation_command_result("updateGenerationValues", Some(&args), doc.snapshot, cfg.snapshot).map(|result| result.emit).unwrap_or_default())
}
