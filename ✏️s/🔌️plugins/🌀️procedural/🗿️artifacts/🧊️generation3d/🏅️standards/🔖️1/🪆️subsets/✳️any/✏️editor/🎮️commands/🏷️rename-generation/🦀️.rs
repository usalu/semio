//! 🧬️ 🧬️ Generation3d play app commands command — `rename-generation`.

use crate::editor::generation3d::commands::generation::generation_command_result;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "rename-generation")]
pub struct RenameGeneration {
    pub id: String,
    pub name: String,
}

pub fn handle(payload: &RenameGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(generation_command_result("renameGeneration", Some(&semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone())), ("name".to_string(), semio_framework_value::DslValue::String(payload.name.clone()))])), doc.snapshot, cfg.snapshot)
        .map(|result| result.emit)
        .unwrap_or_default())
}
