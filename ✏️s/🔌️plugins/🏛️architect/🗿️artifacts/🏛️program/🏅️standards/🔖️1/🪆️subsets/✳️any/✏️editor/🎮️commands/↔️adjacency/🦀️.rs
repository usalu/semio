//! ↔️ Architect play app commands — the adjacency surface: patching an adjacency field, cycling or
//! setting a pair's kind, and filtering the matrix by kind.

pub mod set_adjacency_field {
    use crate::editor::architect::catalog::patch_register_item_operation;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::{EntityId, ProgramSnapshot};
    use dsl::DslValue as Value;
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-field")]
    pub struct SetAdjacencyField {
        pub entity_id: String,
        pub field: String,
        pub value_json: String,
    }

    /// ↔️ Sets one field of one adjacency row; an unparsable value, an unnamed field or an unknown
    /// adjacency is refused by name.
    pub fn handle(payload: &SetAdjacencyField, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let value = dsl::json::from_json_str::<Value>(&payload.value_json).map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("architect.adjacency-value-invalid"), format!("setAdjacencyField needs a JSON value, got {}", payload.value_json)))?;
        if payload.field.is_empty() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("architect.adjacency-field-missing"), "setAdjacencyField needs a field name"));
        }
        let patch = vec![(payload.field.clone(), value)];
        let operation = patch_register_item_operation(doc.snapshot, "adjacencies", &EntityId(payload.entity_id.clone()), &Value::Object(patch))
            .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("setAdjacencyField cannot set \"{}\" to {} on adjacency \"{}\"", payload.field, payload.value_json, payload.entity_id)))?;
        Ok(Emit::mutations(vec![operation]))
    }
}

pub mod set_adjacency_kind {
    use crate::editor::architect::catalog::{adjacency_kind_from_id, find_adjacency, new_adjacency, next_adjacency_kind};
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::schema::mutations as leaves;
    use crate::{EntityId, ProgramSnapshot};
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-kind")]
    pub struct SetAdjacencyKind {
        pub element_a_id: String,
        pub element_b_id: String,
        pub kind: Option<String>,
        pub cycle: bool,
    }

    pub fn handle(payload: &SetAdjacencyKind, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let a = EntityId(payload.element_a_id.clone());
        let b = EntityId(payload.element_b_id.clone());
        for (argument, id) in [("elementAId", &a), ("elementBId", &b)] {
            if !program.elements.iter().any(|element| element.header.id == *id) {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("setAdjacencyKind found no program element \"{id}\" for {argument}")));
            }
        }
        if a == b {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("architect.adjacency-self"), format!("setAdjacencyKind cannot relate element \"{a}\" to itself")));
        }
        let explicit = match payload.kind.as_deref().filter(|kind| !kind.is_empty()) {
            Some(kind) => Some(adjacency_kind_from_id(kind).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("architect.adjacency-kind-unknown"), format!("setAdjacencyKind has no adjacency kind \"{kind}\"")))?),
            None => None,
        };
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
                Ok(Emit::mutations(vec![ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::ConnectAdjacency { adjacency })]))
            }
            None => {
                if let Some(row) = existing {
                    Ok(Emit::mutations(vec![ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::DisconnectAdjacency { id: row.header.id.clone() })]))
                } else {
                    Ok(Emit::default())
                }
            }
        }
    }
}

pub mod set_adjacency_filter {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::ProgramSnapshot;
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-adjacency-filter")]
    pub struct SetAdjacencyFilter {
        pub kind: Option<String>,
    }

    pub fn handle(_payload: &SetAdjacencyFilter, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
