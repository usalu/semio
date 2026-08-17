//! 🔧 `change-rho-l` payload — changes the En1992 document's `rho_l` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeRhoL
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRhoL {
    pub new_rho_l: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeRhoL {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "rho-l", kind: "change-rho-l", record: "ChangedRhoL" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        crate::artifacts::en1992::mutations::change_rho_l::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_rho_l::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change rho l to {:?}", self.new_rho_l)
    }
}
//#endregion 🔖️ChangeRhoL
