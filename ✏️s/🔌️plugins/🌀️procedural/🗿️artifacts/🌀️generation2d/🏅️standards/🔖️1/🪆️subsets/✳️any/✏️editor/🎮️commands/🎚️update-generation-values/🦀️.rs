//! 🧬️ Generation2d command — update generation values.

use crate::editor::generation2d::commands::generation::handle_generation;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "update-generation-values")]
pub struct UpdateGenerationValues {
    pub generation_id: Option<String>,
    pub question_id: String,
    pub value: dsl::DslValue,
}

pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let args = dsl::DslValue::object([
        ("generationId".into(), payload.generation_id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String)),
        ("questionId".into(), dsl::DslValue::String(payload.question_id.clone())),
        ("value".into(), payload.value.clone()),
    ]);
    Ok(handle_generation("updateGenerationValues", Some(&args), doc, cfg).emit)
}
