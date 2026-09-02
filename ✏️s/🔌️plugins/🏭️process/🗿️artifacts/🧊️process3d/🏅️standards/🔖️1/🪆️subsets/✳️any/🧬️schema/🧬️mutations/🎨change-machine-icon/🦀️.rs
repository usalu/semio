//! 🔧 `change-machine-icon` payload — changes an id-keyed [`WorkshopMachine`]'s `icon_id`.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, Workshop};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeMachineIcon
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeMachineIcon {
    pub id: String,
    pub new_icon_id: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ChangeMachineIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "machine", kind: "change-machine-icon", record: "ChangedMachineIcon" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change icon of machine \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeMachineIcon
