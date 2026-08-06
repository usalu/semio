//! 🛠️ Process 3d play app commands — workshop machine lifecycle (add / remove / update).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::artifacts::process3d::engine::catalog_machine;
use crate::artifacts::process3d::{op::Process3dOperation, Process3dDocument, WorkshopMachine, WorkshopMachinePatch};
use protocol::CollectionOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
fn add_workshop_machine_operation(fixture: &Process3dDocument, machine: WorkshopMachine) -> Option<Process3dOperation> {
    if fixture.workshop.machines.iter().any(|existing| existing.id == machine.id) {
        return None;
    }
    let at = fixture.workshop.machines.len();
    Some(Process3dOperation::Machines { collection: CollectionOperation::Add { index: at, item: machine } })
}

fn remove_workshop_machine_operation(fixture: &Process3dDocument, id: &str) -> Option<Process3dOperation> {
    fixture.workshop.machines.iter().any(|machine| machine.id == id).then(|| Process3dOperation::Machines { collection: CollectionOperation::Remove { id: id.to_string() } })
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

    pub fn handle(payload: &AddWorkshopMachine, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        match catalog_machine(&payload.catalog_id, &payload.machine_id) {
            Some(machine) => match add_workshop_machine_operation(fixture, machine) {
                Some(operation) => {
                    let selected = format!("machine:{}", payload.machine_id);
                    Ok(Emit { document_operations: vec![operation], config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(selected) }], ..Default::default() })
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

    pub fn handle(payload: &RemoveWorkshopMachine, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        match remove_workshop_machine_operation(fixture, &payload.id) {
            Some(operation) => {
                let mut config_operations = Vec::new();
                if config.selected_id.as_deref() == Some(format!("machine:{}", payload.id).as_str()) {
                    config_operations.push(Process3dConfigOperation::SetSelectedId { value: None });
                }
                Ok(Emit { document_operations: vec![operation], config_operations, ..Default::default() })
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
    pub fn handle(payload: &UpdateWorkshopMachine, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        if doc.projection.workshop.machines.iter().any(|existing| existing.id == payload.machine.id) {
            let patch = WorkshopMachinePatch { label: Some(payload.machine.label.clone()), icon_id: Some(payload.machine.icon_id.clone()), capabilities: Some(payload.machine.capabilities.clone()) };
            Ok(Emit::operations(vec![Process3dOperation::Machines { collection: CollectionOperation::Patch { id: payload.machine.id.clone(), patch } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️UpdateWorkshopMachine
