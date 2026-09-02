//! 🔧 `change-anchor-f-yk-mpa` payload — changes the En1992 document's `anchor_f_yk_mpa` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_anchor_f_yk_mpa::ChangeAnchorFYkMpa;

//#region 🔖️ChangeAnchorFYkMpa
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAnchorFYkMpa {
    pub new_anchor_f_yk_mpa: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorFYkMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-f-yk-mpa", kind: "change-anchor-f-yk-mpa", record: "ChangedAnchorFYkMpa" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change anchor f yk mpa to {:?}", self.new_anchor_f_yk_mpa)
    }
}
//#endregion 🔖️ChangeAnchorFYkMpa
