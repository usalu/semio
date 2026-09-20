//! 📚️ VCS play app command — loading one of the app's declared example documents.

pub mod set_active_example {
    use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
    use crate::editor::vcs::vcs_example_document_effect;
    use crate::op::VcsDemoMutation;
    use crate::VcsSnapshot;
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 🧬️ Whole-document replace has no `VcsDemoMutation` representative, so picking an example
    /// builds an `Effect::LoadDocument` and therefore lands outside undo history. An id this app
    /// does not publish is a no-op rather than a fault: the playground navbar dispatches whatever
    /// its combobox holds, including an empty string before the catalogue resolves.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        if payload.example_id.is_empty() || payload.example_id == crate::examples::demo::ID {
            return Ok(Emit { effects: vec![vcs_example_document_effect()], ..Default::default() });
        }
        Ok(Emit::default())
    }
}
