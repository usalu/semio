//! 🗂️ Architect play app command — entity selection. Config-only: emits `config_operations`, never
//! document operations.

pub mod set_selection {
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::Program;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let mut next = cfg.projection.clone();
        next.selected_ids = payload.ids.clone();
        Ok(Emit::config(snapshot(next)))
    }
}
