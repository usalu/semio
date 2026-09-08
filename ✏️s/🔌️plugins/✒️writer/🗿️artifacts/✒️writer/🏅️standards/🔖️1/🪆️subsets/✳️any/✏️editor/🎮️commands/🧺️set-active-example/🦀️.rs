//! ✍️ ✍️ Writer play app commands command — `set-active-example`.

use crate::dsl::{dag_jack_example_document, jack_example_document};
use crate::op::WriterMutation;
use crate::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::editor::writer::reset_document_effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::schema::empty_writer_snapshot;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
