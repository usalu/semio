//! ↔️ Architect play app commands — the adjacency surface: patching an adjacency field, cycling or
//! setting a pair's kind, and filtering the matrix by kind.

pub mod set_adjacency_field {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::{EntityId, ProgramSnapshot};
    use crate::editor::architect::catalog::patch_register_item_operation;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
        use serde_json::Value;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-field")]
    pub struct SetAdjacencyField {
        pub entity_id: String,
        pub field: String,
        pub value_json: String,
    }

    pub async fn handle(payload: &SetAdjacencyField, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let Ok(value) = serde_json::from_str::<Value>(&payload.value_json) else {
            return Ok(Emit::default());
        };
        let mut patch = serde_json::Map::new();
        patch.insert(payload.field.clone(), value);
        match patch_register_item_operation(doc.snapshot, "adjacencies", EntityId(payload.entity_id.clone()), Value::Object(patch)) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}

pub mod set_adjacency_kind {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::{EntityId, ProgramSnapshot};
    use crate::editor::architect::catalog::{adjacency_kind_from_id, find_adjacency, new_adjacency, next_adjacency_kind};
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-kind")]
    pub struct SetAdjacencyKind {
        pub element_a_id: String,
        pub element_b_id: String,
        pub kind: Option<String>,
        pub cycle: bool,
    }

    pub async fn handle(payload: &SetAdjacencyKind, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
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
                Ok(Emit::mutations(vec![ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::mutation::ConnectAdjacency { adjacency })]))
            }
            None => {
                if let Some(row) = existing {
                    Ok(Emit::mutations(vec![ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::mutation::DisconnectAdjacency { id: row.header.id.clone() })]))
                } else {
                    Ok(Emit::default())
                }
            }
        }
    }
}

pub mod set_adjacency_filter {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::catalog::adjacency_kind_from_id;
    use crate::editor::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-filter")]
    pub struct SetAdjacencyFilter {
        pub kind: Option<String>,
    }

    pub async fn handle(payload: &SetAdjacencyFilter, _doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let mut next = cfg.snapshot.clone();
        next.adjacency_kind_filter = payload.kind.as_deref().and_then(adjacency_kind_from_id);
        Ok(Emit::config(snapshot(next)))
    }
}
