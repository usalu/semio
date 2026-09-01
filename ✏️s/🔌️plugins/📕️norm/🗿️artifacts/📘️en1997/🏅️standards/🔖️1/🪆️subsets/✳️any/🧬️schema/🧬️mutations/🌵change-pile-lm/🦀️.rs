//! 🌵 `change-pile-lm` payload — changes the En1997 document's `pile_l_m` (pile length [m]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_pile_l_m::ChangePileLM;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangePileLM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangePileLM {
    pub new_pile_l_m: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangePileLM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pile-lm", kind: "change-pile-lm", record: "ChangedPileLM" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change pile length [m] to {}", self.new_pile_l_m)
    }
}
//#endregion 🔖️ChangePileLM
