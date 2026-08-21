//! 🔁 `replace-machine-capabilities` payload — whole-value swap of an id-keyed
//! [`WorkshopMachine`]'s `capabilities` list (large structured field, per
//! `📓️derivation-rules.md` rule 2).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Capability, Process3dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️ReplaceMachineCapabilities
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceMachineCapabilities {
    pub id: String,
    pub new_capabilities: Vec<Capability>,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ReplaceMachineCapabilities {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "machine", kind: "replace-machine-capabilities", record: "ReplacedMachineCapabilities" };

    async fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::replace_machine_capabilities::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::replace_machine_capabilities::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Replace capabilities of machine \"{}\"", self.id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ReplaceMachineCapabilities
