//! 🌿 `change-alpha-s` payload — changes the En1997 document's `alpha_s` (shaft resistance factor alpha_s).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_alpha_s::ChangeAlphaS;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAlphaS
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAlphaS {
    pub new_alpha_s: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeAlphaS {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "alpha-s", kind: "change-alpha-s", record: "ChangedAlphaS" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change shaft resistance factor alpha_s to {}", self.new_alpha_s)
    }
}
//#endregion 🔖️ChangeAlphaS
