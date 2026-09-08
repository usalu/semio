//! 🔁 `replace-machine-capabilities` payload — whole-value swap of an id-keyed
//! [`WorkshopMachine`]'s `capabilities` list (large structured field, per
//! `📓️derivation-rules.md` rule 2).

use crate::diff::Process3dDiff;
use crate::mutations::Process3dMutation;
use crate::{Capability, Process3dSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ReplaceMachineCapabilities
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReplaceMachineCapabilities {
    pub id: String,
    pub new_capabilities: Vec<Capability>,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ReplaceMachineCapabilities {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "machine", kind: "replace-machine-capabilities", record: "ReplacedMachineCapabilities" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace capabilities of machine \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ReplaceMachineCapabilities
