//! 🏗️ Architect play app commands — the program element shortcut: add and remove a room-shaped
//! element (removing one also clears every adjacency touching it).

pub mod add_element {
    use crate::apps::architect::catalog::default_element;
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-element")]
    pub struct AddElement {
        pub name: String,
    }

    pub fn handle(payload: &AddElement, _doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let element = default_element(payload.name.clone());
        let id = element.header.id.to_string();
        let mut next = cfg.snapshot.clone();
        next.selected_ids = vec![id];
        next.active_register = "elements".into();
        Ok(Emit {
            artifact_mutations: vec![ProgramMutation::CreateProgramElement(leaves::create_program_element::mutation::CreateProgramElement { program_element: element })],
            config_mutations: snapshot(next),
            ..Default::default()
        })
    }
}

pub mod remove_element {
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::{EntityId, ProgramSnapshot};
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-element")]
    pub struct RemoveElement {
        pub element_id: String,
    }

    pub fn handle(payload: &RemoveElement, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let element_id = &payload.element_id;
        let mut next = cfg.snapshot.clone();
        next.selected_ids.retain(|selected| selected != element_id);
        let mut operations = vec![ProgramMutation::DeleteProgramElement(leaves::delete_program_element::mutation::DeleteProgramElement { id: EntityId(element_id.clone()) })];
        for adjacency in program.adjacencies.iter().filter(|row| &row.element_a_id.0 == element_id || &row.element_b_id.0 == element_id) {
            operations.push(ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::mutation::DisconnectAdjacency { id: adjacency.header.id.clone() }));
        }
        Ok(Emit { artifact_mutations: operations, config_mutations: snapshot(next), ..Default::default() })
    }
}
