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
    /// 🎚️ The press this value belongs to, when the Form control it came from is CONTINUOUS (a
    /// dragged slider, a held spinner). Every value of one press folds into ONE undoable edit under
    /// this identity; absent means a discrete edit of its own
    /// (`📓️slider-preview-update-2026-09-15.md`).
    pub gesture: Option<String>,
}

pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let generation_id = payload.generation_id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String);
    let args = dsl::DslValue::object([("generationId".to_string(), generation_id), ("questionId".to_string(), dsl::DslValue::String(payload.question_id.clone())), ("value".to_string(), payload.value.clone())]);
    let mut emit = generation_command_result("updateGenerationValues", Some(&args), doc.snapshot, cfg.snapshot).map(|result| result.emit).unwrap_or_default();
    if let Some(key) = crate::editor::generation3d::commands::patch_flow_widgets::patch_coalesce_key(payload.gesture.as_deref()) {
        emit.coalesce_key = Some(key);
    }
    Ok(emit)
}
