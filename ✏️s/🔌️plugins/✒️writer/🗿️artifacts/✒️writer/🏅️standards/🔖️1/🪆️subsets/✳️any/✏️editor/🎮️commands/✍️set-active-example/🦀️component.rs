//! ✍️ ✍️ Writer play app commands command — `set-active-example`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::editor::writer::reset_document_effect;
use crate::artifacts::writer::dsl::{dag_jack_example_document, jack_example_document};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

use crate::artifacts::writer::schema::empty_writer_snapshot;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let document = match payload.example_id.as_str() {
        "jack" => jack_example_document(),
        "dag.jack" => dag_jack_example_document(),
        _ => empty_writer_snapshot(),
    };
    Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })
}
