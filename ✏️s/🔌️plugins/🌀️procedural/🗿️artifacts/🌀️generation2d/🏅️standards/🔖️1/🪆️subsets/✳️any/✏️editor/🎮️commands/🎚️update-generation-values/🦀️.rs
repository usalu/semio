//! 🧬️ Generation2d command — update generation values.

use crate::generation::handle_generation;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_os_flow::FlowEvalSession;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "update-generation-values")]
pub struct UpdateGenerationValues {
    pub generation_id: Option<String>,
    pub question_id: String,
    pub field_index: Option<u32>,
    pub value: semio_framework_value::DslValue,
}

pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    Ok(handle_generation("updateGenerationValues", Some(&payload.arguments()), doc, cfg).emit)
}


impl UpdateGenerationValues {
    /// 🧾️ Builds the shared generation transition arguments from a typed command.
    pub fn arguments(&self) -> semio_framework_value::DslValue {
        use semio_framework_value::DslValue;
        DslValue::object([
            ("generationId".into(), self.generation_id.clone().map_or(DslValue::Null, DslValue::String)),
            ("questionId".into(), DslValue::String(self.question_id.clone())),
            ("fieldIndex".into(), self.field_index.map_or(DslValue::Null, |index| DslValue::uint(u64::from(index)))),
            ("value".into(), self.value.clone()),
        ])
    }
}
