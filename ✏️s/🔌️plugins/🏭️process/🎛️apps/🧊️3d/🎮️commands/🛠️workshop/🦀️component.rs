//! 🛠️ Process 3d play app commands — workshop machine lifecycle (add / remove / update).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::engine::catalog_machine;
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot, WorkshopMachine, WorkshopMachinePatch};
use protocol::CollectionMutation;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
fn add_workshop_machine_operation(fixture: &Process3dSnapshot, machine: WorkshopMachine) -> Option<Process3dMutation> {
    if fixture.workshop.machines.iter().any(|existing| existing.id == machine.id) {
        return None;
    }
    let at = fixture.workshop.machines.len();
    Some(Process3dMutation::Machines { collection: CollectionMutation::Add { index: at, item: machine } })
}

fn remove_workshop_machine_operation(fixture: &Process3dSnapshot, id: &str) -> Option<Process3dMutation> {
    fixture.workshop.machines.iter().any(|machine| machine.id == id).then(|| Process3dMutation::Machines { collection: CollectionMutation::Remove { id: id.to_string() } })
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

    /// 🔧️ Programmatic full-machine edit, mirrors `update_step::UpdateStep`.
    pub fn handle(payload: &UpdateWorkshopMachine, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        if doc.snapshot.workshop.machines.iter().any(|existing| existing.id == payload.machine.id) {
            let patch = WorkshopMachinePatch { label: Some(payload.machine.label.clone()), icon_id: Some(payload.machine.icon_id.clone()), capabilities: Some(payload.machine.capabilities.clone()) };
            Ok(Emit::mutations(vec![Process3dMutation::Machines { collection: CollectionMutation::Patch { id: payload.machine.id.clone(), patch } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️UpdateWorkshopMachine
