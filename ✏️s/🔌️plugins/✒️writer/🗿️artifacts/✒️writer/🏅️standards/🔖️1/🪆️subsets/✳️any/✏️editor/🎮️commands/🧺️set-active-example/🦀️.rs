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

/// 📚️ The ONE id→document map this app has — `WriterPlayApp::dispatch`'s own `SetActiveExample`
/// arm calls it too, so the runtime route and this command leaf can never drift (they did: the arm
/// knew only `jack`/`dag.jack` while the published example id is `demo`, so every boot reset the pane
/// to an EMPTY document).
/// 📚️ `crate::examples::demo::ID` is the ONE example id this subset publishes
/// (`📚️examples/🎬️demo`, registered by `subsets::any::examples()`), and the playground navbar boots the
/// pane by dispatching exactly that id — an empty id means the same "load my default example".
/// `dag.jack` is the second authored asset beside it. An id this app does not publish resets to the
/// empty document rather than faulting: the navbar dispatches whatever its combobox holds.
pub fn document_for_example_id(example_id: &str) -> WriterSnapshot {
    match example_id {
        "" => jack_example_document(),
        id if id == crate::examples::demo::ID => jack_example_document(),
        "dag.jack" => dag_jack_example_document(),
        _ => empty_writer_snapshot(),
    }
}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Ok(Emit { effects: vec![reset_document_effect(&document_for_example_id(&payload.example_id))], ..Default::default() })
}
