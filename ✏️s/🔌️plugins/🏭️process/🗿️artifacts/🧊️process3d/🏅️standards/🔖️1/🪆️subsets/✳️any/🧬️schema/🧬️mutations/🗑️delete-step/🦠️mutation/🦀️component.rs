//! 🗑️ `delete-step` payload — removes an id-keyed [`ProcessStep`] from the document's ordered
//! timeline.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteStep
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStep {
    pub id: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for DeleteStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    fn diff(&self, base: &Process3dSnapshot) -> Process3dDiff {
        crate::artifacts::process3d::mutations::delete_step::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::delete_step::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete step \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteStep
