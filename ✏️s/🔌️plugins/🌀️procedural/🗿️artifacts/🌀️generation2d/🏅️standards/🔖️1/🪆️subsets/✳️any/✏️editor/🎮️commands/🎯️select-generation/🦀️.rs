//! 🧬️ Generation2d command — select generation.

use crate::editor::generation2d::commands::generation::handle_generation;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "select-generation")]
pub struct SelectGeneration {
    pub id: Option<String>,
}

pub fn handle(payload: &SelectGeneration, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let args = dsl::DslValue::object([("id".into(), payload.id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null))]);
    Ok(handle_generation("selectGeneration", Some(&args), doc, cfg).emit)
}
