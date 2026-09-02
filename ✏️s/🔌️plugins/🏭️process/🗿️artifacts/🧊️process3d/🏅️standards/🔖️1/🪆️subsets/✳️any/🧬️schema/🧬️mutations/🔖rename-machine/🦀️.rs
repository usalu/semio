//! 🏷️ `rename-machine` payload — changes an id-keyed [`WorkshopMachine`]'s `label`.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RenameMachine
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameMachine {
    pub id: String,
    pub new_label: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for RenameMachine {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "machine", kind: "rename-machine", record: "RenamedMachine" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename machine to \"{}\"", self.new_label)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️RenameMachine
