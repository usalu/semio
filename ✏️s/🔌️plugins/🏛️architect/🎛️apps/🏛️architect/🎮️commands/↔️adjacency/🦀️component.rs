//! ↔️ Architect play app commands — the adjacency surface: patching an adjacency field, cycling or
//! setting a pair's kind, and filtering the matrix by kind.

pub mod set_adjacency_field {
    use crate::apps::architect::catalog::patch_register_item_operation;
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::{EntityId, Program};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-field")]
    pub struct SetAdjacencyField {
        pub entity_id: String,
        pub field: String,
        pub value_json: String,
    }

    pub fn handle(payload: &SetAdjacencyField, _doc: &DocumentView<'_, Program>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let Ok(value) = serde_json::from_str::<Value>(&payload.value_json) else {
            return Ok(Emit::default());
        };
        let mut patch = serde_json::Map::new();
        patch.insert(payload.field.clone(), value);
        match patch_register_item_operation("adjacencies", EntityId(payload.entity_id.clone()), Value::Object(patch)) {
            Some(operation) => Ok(Emit::operations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}

pub mod set_adjacency_kind {
    use crate::apps::architect::catalog::{adjacency_kind_from_id, find_adjacency, new_adjacency, next_adjacency_kind};
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::{EntityId, Program};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-kind")]
    pub struct SetAdjacencyKind {
        pub element_a_id: String,
        pub element_b_id: String,
        pub kind: Option<String>,
        pub cycle: bool,
    }

    pub fn handle(payload: &SetAdjacencyKind, doc: &DocumentView<'_, Program>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let program = doc.projection;
        let a = EntityId(payload.element_a_id.clone());
        let b = EntityId(payload.element_b_id.clone());
        let explicit = payload.kind.as_deref().and_then(adjacency_kind_from_id);
        let existing = find_adjacency(program, &a, &b);
        let next_kind = if payload.cycle { next_adjacency_kind(existing.map(|row| &row.kind)) } else { explicit.or_else(|| next_adjacency_kind(existing.map(|row| &row.kind))) };
        match next_kind {
            Some(kind) => {
                let adjacency = if let Some(row) = existing {
                    let mut updated = row.clone();
                    updated.kind = kind;
                    updated
                } else {
                    new_adjacency(program, &a, &b, kind)
                };
                Ok(Emit::operations(vec![ProgramOperation::SetAdjacency { adjacency }]))
            }
            None => {
                if let Some(row) = existing {
                    Ok(Emit::operations(vec![ProgramOperation::ClearAdjacency { id: row.header.id.clone() }]))
                } else {
                    Ok(Emit::default())
                }
            }
        }
    }
}

pub mod set_adjacency_filter {
    use crate::apps::architect::catalog::adjacency_kind_from_id;
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::Program;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-filter")]
    pub struct SetAdjacencyFilter {
        pub kind: Option<String>,
    }

    pub fn handle(payload: &SetAdjacencyFilter, _doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let mut next = cfg.projection.clone();
        next.adjacency_kind_filter = payload.kind.as_deref().and_then(adjacency_kind_from_id);
        Ok(Emit::config(snapshot(next)))
    }
}
