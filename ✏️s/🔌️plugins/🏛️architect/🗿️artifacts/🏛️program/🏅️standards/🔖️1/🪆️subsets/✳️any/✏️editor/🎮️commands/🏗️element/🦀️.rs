//! 🏗️ Architect play app commands — the program element shortcut: add and remove a room-shaped
//! element (removing one also clears every adjacency touching it).

pub mod add_element {
    use dsl::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::catalog::default_element;
    use crate::editor::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "add-element")]
    pub struct AddElement {
        pub name: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new element used to also
    /// become the selection here — selection is framework-owned `InteractionState` now, only ever
    /// mutated by the framework's own injected `interactionSelect` handling, never by an app
    /// command's `Emit` (mirrors note's `add-block`).
    pub async fn handle(payload: &AddElement, _doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let element = default_element(payload.name.clone());
        let mut next = cfg.snapshot.clone();
        next.active_register = "elements".into();
        Ok(Emit { artifact_mutations: vec![ProgramMutation::CreateProgramElement(leaves::create_program_element::CreateProgramElement { program_element: element })], config_mutations: snapshot(next), ..Default::default() })
    }
}

pub mod remove_element {
    use dsl::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::{EntityId, ProgramSnapshot};
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "remove-element")]
    pub struct RemoveElement {
        pub element_id: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the deleted id
    /// out of a config-owned selection — the framework owns pruning of the "program" domain's
    /// selection now (`validate_state`, run after every dispatch).
    pub async fn handle(payload: &RemoveElement, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let element_id = &payload.element_id;
        let mut operations = vec![ProgramMutation::DeleteProgramElement(leaves::delete_program_element::DeleteProgramElement { id: EntityId(element_id.clone()) })];
        for adjacency in program.adjacencies.iter().filter(|row| &row.element_a_id.0 == element_id || &row.element_b_id.0 == element_id) {
            operations.push(ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::DisconnectAdjacency { id: adjacency.header.id.clone() }));
        }
        Ok(Emit::mutations(operations))
    }
}
