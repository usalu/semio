//! 🔧 `change-step-origin` payload — sets an id-keyed [`ProcessStep`]'s `origin` provenance
//! (machine/capability the step was built from) to an explicit new value, or clears it.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::change_step_origin::ChangeStepOrigin;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, StepOrigin};
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeStepOrigin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStepOrigin {
    pub id: String,
    pub new_origin: Option<StepOrigin>,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ChangeStepOrigin {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "step", kind: "change-step-origin", record: "ChangedStepOrigin" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change origin of step \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeStepOrigin
