//! 🧬️ Generation2d command — rename generation.

use crate::editor::generation2d::commands::generation::handle_generation;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "rename-generation")]
pub struct RenameGeneration {
    pub id: String,
    pub name: String,
}

pub fn handle(payload: &RenameGeneration, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let args = dsl::DslValue::object([("id".into(), dsl::DslValue::String(payload.id.clone())), ("name".into(), dsl::DslValue::String(payload.name.clone()))]);
    Ok(handle_generation("renameGeneration", Some(&args), doc, cfg).emit)
}
