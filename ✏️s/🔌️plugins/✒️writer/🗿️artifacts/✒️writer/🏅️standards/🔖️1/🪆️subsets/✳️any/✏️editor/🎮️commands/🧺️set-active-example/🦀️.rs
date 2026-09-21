//! ✍️ ✍️ Writer play app commands command — `set-active-example`.

use crate::document_dsl::{dag_jack_example_document, jack_example_document};
use crate::editor::writer::reset_document_effect;
use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

use crate::schema::empty_writer_snapshot;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ `crate::examples::demo::ID` is the ONE example id this subset publishes
/// (`📚️examples/🎬️demo`, registered by `subsets::any::examples()`), and the playground navbar boots the
/// pane by dispatching exactly that id — an empty id means the same "load my default example".
/// `dag.jack` is the second authored asset beside it. An id this app does not publish resets to the
/// empty document rather than faulting: the navbar dispatches whatever its combobox holds.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    let document = match payload.example_id.as_str() {
        "" => jack_example_document(),
        id if id == crate::examples::demo::ID => jack_example_document(),
        "dag.jack" => dag_jack_example_document(),
        _ => empty_writer_snapshot(),
    };
    Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })
}
