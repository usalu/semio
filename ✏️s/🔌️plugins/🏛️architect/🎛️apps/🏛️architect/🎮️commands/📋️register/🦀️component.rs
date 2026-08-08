//! 📋️ Architect play app commands — the register row lifecycle: which register is active, and
//! adding, removing and patching its rows.

pub mod select_register {
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::Program;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-register")]
    pub struct SelectRegister {
        pub register_id: String,
    }

    pub fn handle(payload: &SelectRegister, _doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let mut next = cfg.projection.clone();
        next.active_register = payload.register_id.clone();
        next.selected_ids.clear();
        Ok(Emit::config(snapshot(next)))
    }
}

pub mod add_register_item {
    use crate::apps::architect::catalog::add_register_item_operation;
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::engine::template::apply_template;
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::{EntityId, Program};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-register-item")]
    pub struct AddRegisterItem {
        pub register_id: String,
        pub name: String,
        pub template_id: Option<String>,
    }

    pub fn handle(payload: &AddRegisterItem, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.projection;
        if let Some(template_id) = &payload.template_id {
            let template_id = EntityId(template_id.clone());
            if let Some(template) = program.templates.iter().find(|row| row.header.id == template_id).cloned() {
                let mut scratch = program.clone();
                return Ok(Emit::mutations(apply_template(&mut scratch, &template)));
            }
        }
        let Some((operation, id)) = add_register_item_operation(program, &payload.register_id, &payload.name) else {
            return Ok(Emit::default());
        };
        let mut next = cfg.projection.clone();
        next.active_register = payload.register_id.clone();
        next.selected_ids = vec![id.to_string()];
        Ok(Emit { document_mutations: vec![operation], config_mutations: snapshot(next), ..Default::default() })
    }
}

pub mod remove_register_item {
    use crate::apps::architect::catalog::remove_register_item_operation;
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::{EntityId, Program};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-register-item")]
    pub struct RemoveRegisterItem {
        pub register_id: String,
        pub entity_id: String,
    }

    pub fn handle(payload: &RemoveRegisterItem, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.projection;
        let entity_id = EntityId(payload.entity_id.clone());
        let mut next = cfg.projection.clone();
        next.selected_ids.retain(|selected| selected != &entity_id.0);
        let mut operations = Vec::new();
        if let Some(operation) = remove_register_item_operation(&payload.register_id, entity_id.clone()) {
            operations.push(operation);
        }
        if payload.register_id == "elements" {
            for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id == entity_id || row.element_b_id == entity_id) {
                operations.push(ProgramMutation::ClearAdjacency { id: adjacency.header.id.clone() });
            }
        }
        Ok(Emit { document_mutations: operations, config_mutations: snapshot(next), ..Default::default() })
    }
}

pub mod patch_register_item {
    use crate::apps::architect::catalog::patch_register_item_operation;
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::{EntityId, Program};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-register-item")]
    pub struct PatchRegisterItem {
        pub register_id: String,
        pub entity_id: String,
        pub patch_json: String,
    }

    pub fn handle(payload: &PatchRegisterItem, _doc: &DocumentView<'_, Program>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let Ok(patch) = serde_json::from_str::<Value>(&payload.patch_json) else {
            return Ok(Emit::default());
        };
        match patch_register_item_operation(&payload.register_id, EntityId(payload.entity_id.clone()), patch) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
