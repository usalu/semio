//! 🗑️ `delete-machine` payload — removes an id-keyed [`WorkshopMachine`] from the document's
//! workshop.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::create_machine::CreateMachine;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DeleteMachine
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteMachine {
    pub id: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for DeleteMachine {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "machine", kind: "delete-machine", record: "DeletedMachine" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete machine \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteMachine
