//! 🛠️ Process 3d play app commands — workshop machine lifecycle (add / remove / update).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::engine::catalog_machine;
use crate::artifacts::process3d::mutations::change_machine_icon::mutation::ChangeMachineIcon;
use crate::artifacts::process3d::mutations::create_machine::mutation::CreateMachine;
use crate::artifacts::process3d::mutations::delete_machine::mutation::DeleteMachine;
use crate::artifacts::process3d::mutations::rename_machine::mutation::RenameMachine;
use crate::artifacts::process3d::mutations::replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot, WorkshopMachine};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
fn add_workshop_machine_operation(fixture: &Process3dSnapshot, machine: WorkshopMachine) -> Option<Process3dMutation> {
    if fixture.workshop.machines.iter().any(|existing| existing.id == machine.id) {
        return None;
    }
    let at = fixture.workshop.machines.len();
    Some(Process3dMutation::CreateMachine(CreateMachine { index: at, machine }))
}

fn remove_workshop_machine_operation(fixture: &Process3dSnapshot, id: &str) -> Option<Process3dMutation> {
    fixture.workshop.machines.iter().any(|machine| machine.id == id).then(|| Process3dMutation::DeleteMachine(DeleteMachine { id: id.to_string() }))
}
//#endregion 🔖️Helpers

//#region 🔖️AddWorkshopMachine
pub mod add_workshop_machine {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-workshop-machine")]
    pub struct AddWorkshopMachine {
        pub catalog_id: String,
        pub machine_id: String,
    }

    pub fn handle(payload: &AddWorkshopMachine, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        match catalog_machine(&payload.catalog_id, &payload.machine_id) {
            Some(machine) => match add_workshop_machine_operation(fixture, machine) {
                Some(operation) => {
                    let selected = format!("machine:{}", payload.machine_id);
                    Ok(Emit { artifact_mutations: vec![operation], config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: Some(selected) }], ..Default::default() })
                }
                None => Ok(Emit::default()),
            },
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️AddWorkshopMachine

//#region 🔖️RemoveWorkshopMachine
pub mod remove_workshop_machine {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-workshop-machine")]
    pub struct RemoveWorkshopMachine {
        pub id: String,
    }

    pub fn handle(payload: &RemoveWorkshopMachine, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        match remove_workshop_machine_operation(fixture, &payload.id) {
            Some(operation) => {
                let mut config_mutations = Vec::new();
                if config.selected_id.as_deref() == Some(format!("machine:{}", payload.id).as_str()) {
                    config_mutations.push(Process3dConfigMutation::SetSelectedId { value: None });
                }
                Ok(Emit { artifact_mutations: vec![operation], config_mutations, ..Default::default() })
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RemoveWorkshopMachine

//#region 🔖️UpdateWorkshopMachine
pub mod update_workshop_machine {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "update-workshop-machine")]
    pub struct UpdateWorkshopMachine {
        #[dsl(block)]
        pub machine: WorkshopMachine,
    }

    /// 🔧️ Programmatic full-machine edit, mirrors `update_step::UpdateStep` — the machine's fields
    /// each carry their own semantic mutation now (`RenameMachine`/`ChangeMachineIcon`/
    /// `ReplaceMachineCapabilities`), so this diffs `payload.machine` against the current entity and
    /// emits one targeted mutation per field that actually changed.
    pub fn handle(payload: &UpdateWorkshopMachine, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let Some(existing) = doc.snapshot.workshop.machines.iter().find(|existing| existing.id == payload.machine.id) else {
            return Ok(Emit::default());
        };
        let mut operations = Vec::new();
        if existing.label != payload.machine.label {
            operations.push(Process3dMutation::RenameMachine(RenameMachine { id: payload.machine.id.clone(), new_label: payload.machine.label.clone() }));
        }
        if existing.icon_id != payload.machine.icon_id {
            operations.push(Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: payload.machine.id.clone(), new_icon_id: payload.machine.icon_id.clone() }));
        }
        if existing.capabilities != payload.machine.capabilities {
            operations.push(Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: payload.machine.id.clone(), new_capabilities: payload.machine.capabilities.clone() }));
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️UpdateWorkshopMachine
