//! 🧬️ Generation2d command — remove generation.

use crate::editor::generation2d::commands::generation::handle_generation;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-generation")]
pub struct RemoveGeneration {
    pub id: String,
}

pub fn handle(payload: &RemoveGeneration, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let args = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String(payload.id.clone()))]);
    Ok(handle_generation("removeGeneration", Some(&args), doc, cfg).emit)
}
