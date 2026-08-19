//! 🗑️ `delete-machine` payload — removes an id-keyed [`WorkshopMachine`] from the document's
//! workshop.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteMachine
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMachine {
    pub id: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for DeleteMachine {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "machine", kind: "delete-machine", record: "DeletedMachine" };

    async fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::delete_machine::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::delete_machine::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Delete machine \"{}\"", self.id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteMachine
