//! 🔧 `change-step-enabled` payload — sets an id-keyed [`ProcessStep`]'s `enabled` flag to an
//! explicit new value.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeStepEnabled
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStepEnabled {
    pub id: String,
    pub new_enabled: bool,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ChangeStepEnabled {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "step", kind: "change-step-enabled", record: "ChangedStepEnabled" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::change_step_enabled::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::change_step_enabled::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        if self.new_enabled { format!("Enable step \"{}\"", self.id) } else { format!("Disable step \"{}\"", self.id) }
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeStepEnabled
