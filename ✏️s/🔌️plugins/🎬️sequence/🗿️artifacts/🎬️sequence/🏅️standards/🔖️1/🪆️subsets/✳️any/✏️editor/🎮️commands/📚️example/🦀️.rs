//! 📚️ Sequence play app commands — load one of this subset's committed examples.

use crate::editor::sequence::reset_sequence_document_effect;
use crate::mutations::SequenceMutation;
use crate::SequenceSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 🧬️ The whole-document replacement this verb publishes. A composed-child document has no
    /// whole-snapshot mutation representative, so it lands as an `Effect::LoadDocument` outside undo
    /// history — the `🧊️process3d`/`🕸️dag` shape. The document is the subset's own committed example
    /// asset, parsed through this subset's own codec, so what loads is exactly what
    /// `📚️examples/🎬️demo` declares rather than a second, hand-built copy that could drift from it.
    /// An id this app does not publish is an empty emit rather than a fault: the playground navbar
    /// dispatches whatever its combobox holds, including ids belonging to other apps.
    ///
    /// 🪨️ Free of any `ArtifactView`: the retained work answers this verb before a working scene
    /// exists, which is precisely what loading an example is there to make possible.
    pub fn emit(example_id: &str) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        if !example_id.is_empty() && example_id != crate::examples::demo::ID {
            return Ok(Emit::default());
        }
        let document = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT)
            .map_err(|error| Fault::from(format!("sequence example {} is not parsable: {error:?}", crate::examples::demo::ID)))?;
        Ok(Emit { effects: vec![reset_sequence_document_effect(&document)], ..Default::default() })
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        emit(&payload.example_id)
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
