//! 🧬️ 🧬️ Generation3d play app commands command — `select-generation`.

use crate::editor::generation3d::commands::generation::generation_command_result;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "select-generation")]
pub struct SelectGeneration {
    pub id: String,
}

pub fn handle(payload: &SelectGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let args = dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone()))]);
    Ok(generation_command_result("selectGeneration", Some(&args), doc.snapshot, cfg.snapshot).map(|result| result.emit).unwrap_or_default())
}
