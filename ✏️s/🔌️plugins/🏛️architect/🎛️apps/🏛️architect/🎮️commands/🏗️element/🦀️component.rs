//! 🏗️ Architect play app commands — the program element shortcut: add and remove a room-shaped
//! element (removing one also clears every adjacency touching it).

pub mod add_element {
    use crate::apps::architect::catalog::default_element;
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::Program;
    use protocol::CollectionOperation;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-element")]
    pub struct AddElement {
        pub name: String,
    }

    pub fn handle(payload: &AddElement, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let program = doc.projection;
        let element = default_element(payload.name.clone());
        let id = element.header.id.to_string();
        let mut next = cfg.projection.clone();
        next.selected_ids = vec![id];
        next.active_register = "elements".into();
        Ok(Emit {
            document_operations: vec![ProgramOperation::Elements(CollectionOperation::Add { index: program.elements.len(), item: element })],
            config_operations: snapshot(next),
            ..Default::default()
        })
    }
}

pub mod remove_element {
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::{EntityId, Program};
    use protocol::CollectionOperation;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-element")]
    pub struct RemoveElement {
        pub element_id: String,
    }

    pub fn handle(payload: &RemoveElement, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let program = doc.projection;
        let element_id = &payload.element_id;
        let mut next = cfg.projection.clone();
        next.selected_ids.retain(|selected| selected != element_id);
        let mut operations = vec![ProgramOperation::Elements(CollectionOperation::Remove { id: EntityId(element_id.clone()) })];
        for adjacency in program.adjacencies.iter().filter(|row| &row.element_a_id.0 == element_id || &row.element_b_id.0 == element_id) {
            operations.push(ProgramOperation::ClearAdjacency { id: adjacency.header.id.clone() });
        }
        Ok(Emit { document_operations: operations, config_operations: snapshot(next), ..Default::default() })
    }
}
