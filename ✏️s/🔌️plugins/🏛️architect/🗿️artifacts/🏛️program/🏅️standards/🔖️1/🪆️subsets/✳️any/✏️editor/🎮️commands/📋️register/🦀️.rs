//! 📋️ Architect play app commands — the register row lifecycle: which register is active, and
//! adding, removing and patching its rows.

pub mod select_register {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::ProgramSnapshot;
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "select-register")]
    pub struct SelectRegister {
        pub register_id: String,
    }

    pub fn handle(_payload: &SelectRegister, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}

pub mod add_register_item {
    use crate::editor::architect::behavior::apply_template;
    use crate::editor::architect::catalog::add_register_item_operation;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::{EntityId, ProgramSnapshot};
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "add-register-item")]
    pub struct AddRegisterItem {
        pub register_id: String,
        pub name: String,
        pub template_id: Option<String>,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new row used to also become
    /// the selection here — selection is framework-owned `InteractionState` now, only ever mutated by
    /// the framework's own injected `interactionSelect` handling, never by an app command's `Emit`
    /// (mirrors note's `add-block`). A named template must exist and the register must accept new rows;
    /// either miss is refused by name.
    pub fn handle(payload: &AddRegisterItem, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        if let Some(template_id) = payload.template_id.as_ref().filter(|id| !id.is_empty()) {
            let id = EntityId(template_id.clone());
            let template = program.templates.iter().find(|row| row.header.id == id).cloned().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("architect.template-missing"), format!("addRegisterItem found no template \"{template_id}\"")))?;
            let mut scratch = program.clone();
            return Ok(Emit::mutations(apply_template(&mut scratch, &template)));
        }
        let (operation, _id) = add_register_item_operation(program, &payload.register_id, &payload.name).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("architect.register-not-addable"), format!("addRegisterItem cannot add a row named \"{}\" to register \"{}\"", payload.name, payload.register_id)))?;
        Ok(Emit::mutations(vec![operation]))
    }
}

pub mod remove_register_item {
    use crate::editor::architect::catalog::remove_register_item_operation;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::editor::architect::catalog::find_register_for_entity;
    use crate::schema::mutations as leaves;
    use crate::{EntityId, ProgramSnapshot};
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "remove-register-item")]
    pub struct RemoveRegisterItem {
        pub register_id: String,
        pub entity_id: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the deleted id
    /// out of a config-owned selection — the framework owns pruning of the "program" domain's
    /// selection now (`validate_state`, run after every dispatch). The row must live in the named
    /// register; anything else is refused by name.
    pub fn handle(payload: &RemoveRegisterItem, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let entity_id = EntityId(payload.entity_id.clone());
        if find_register_for_entity(program, &entity_id) != Some(payload.register_id.as_str()) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("removeRegisterItem found no row \"{}\" in register \"{}\"", payload.entity_id, payload.register_id)));
        }
        let operation = remove_register_item_operation(&payload.register_id, entity_id.clone()).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("architect.register-not-removable"), format!("removeRegisterItem cannot remove rows from register \"{}\"", payload.register_id)))?;
        let mut operations = vec![operation];
        if payload.register_id == "elements" {
            for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id == entity_id || row.element_b_id == entity_id) {
                operations.push(ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::DisconnectAdjacency { id: adjacency.header.id.clone() }));
            }
        }
        Ok(Emit::mutations(operations))
    }
}

pub mod patch_register_item {
    use crate::editor::architect::catalog::patch_register_item_operation;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::op::ProgramMutation;
    use crate::{EntityId, ProgramSnapshot};
    use dsl::DslValue as Value;
    use dsl::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "patch-register-item")]
    pub struct PatchRegisterItem {
        pub register_id: String,
        pub entity_id: String,
        pub patch_json: String,
    }

    /// 🩹️ Merges a JSON object patch into one row of a patchable register; an unparsable patch, an
    /// unknown row or a register without patch support is refused by name.
    pub fn handle(payload: &PatchRegisterItem, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let patch = dsl::json::from_json_str::<Value>(&payload.patch_json).ok().filter(|value| matches!(value, Value::Object(_))).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("architect.patch-invalid"), format!("patchRegisterItem needs a JSON object patch, got {}", payload.patch_json)))?;
        let operation = patch_register_item_operation(doc.snapshot, &payload.register_id, &EntityId(payload.entity_id.clone()), &patch)
            .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("patchRegisterItem cannot apply {} to row \"{}\" of register \"{}\"", payload.patch_json, payload.entity_id, payload.register_id)))?;
        Ok(Emit::mutations(vec![operation]))
    }
}
