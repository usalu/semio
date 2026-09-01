//! 🔧 `change-anchor-n-ed-kn` payload — changes the En1992 document's `anchor_n_ed_kn` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_anchor_n_ed_kn::ChangeAnchorNEdKn;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnchorNEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnchorNEdKn {
    pub new_anchor_n_ed_kn: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorNEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-n-ed-kn", kind: "change-anchor-n-ed-kn", record: "ChangedAnchorNEdKn" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change anchor n ed kn to {:?}", self.new_anchor_n_ed_kn)
    }
}
//#endregion 🔖️ChangeAnchorNEdKn
