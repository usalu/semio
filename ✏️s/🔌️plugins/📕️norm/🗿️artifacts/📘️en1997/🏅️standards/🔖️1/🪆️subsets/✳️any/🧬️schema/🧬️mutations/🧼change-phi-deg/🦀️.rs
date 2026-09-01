//! 🧼 `change-phi-deg` payload — changes the En1997 document's `phi_deg` (friction angle phi [deg]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_phi_deg::ChangePhiDeg;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangePhiDeg
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangePhiDeg {
    pub new_phi_deg: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangePhiDeg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "phi-deg", kind: "change-phi-deg", record: "ChangedPhiDeg" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change friction angle phi [deg] to {}", self.new_phi_deg)
    }
}
//#endregion 🔖️ChangePhiDeg
