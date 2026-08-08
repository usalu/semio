//! 🗂️ Architect play app command — entity selection. Config-only: emits `config_mutations`, never
//! document operations.

pub mod set_selection {
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let mut next = cfg.snapshot.clone();
        next.selected_ids = payload.ids.clone();
        Ok(Emit::config(snapshot(next)))
    }
}
