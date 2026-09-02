//! 🌿 `change-alpha-s` payload — changes the En1997 document's `alpha_s` (shaft resistance factor alpha_s).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_alpha_s::ChangeAlphaS;

//#region 🔖️ChangeAlphaS
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
