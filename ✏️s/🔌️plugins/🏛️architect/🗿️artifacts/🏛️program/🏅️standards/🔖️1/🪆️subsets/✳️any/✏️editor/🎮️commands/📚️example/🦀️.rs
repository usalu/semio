//! 📚️ Architect play app command — loading one of the app's declared example documents.

pub mod set_active_example {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::editor::architect::reset_document_effect;
    use crate::op::ProgramMutation;
    use crate::{sample_plugin, ProgramSnapshot};
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 🧬️ Whole-document replace has no `ProgramMutation` representative (banned outright by the
    /// taxonomy's forbidden vocabulary), so picking an example builds a `Effect::LoadDocument`
    /// through `reset_document_effect` — the same lane `importProgram`/`importRegistersCsv` use —
    /// and therefore lands outside undo history. An id this app does not publish is a no-op rather
    /// than a fault: the playground navbar dispatches whatever its combobox holds.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        if payload.example_id.is_empty() || payload.example_id == crate::examples::demo::ID {
            return Ok(Emit { effects: vec![reset_document_effect(&sample_plugin())], ..Default::default() });
        }
        Ok(Emit::default())
    }
}
