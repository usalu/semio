//! 🌳 `change-pile-base-area-m2` payload — changes the En1997 document's `pile_base_area_m2` (pile base area [m2]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_pile_base_area_m2::ChangePileBaseAreaM2;

//#region 🔖️ChangePileBaseAreaM2
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePileBaseAreaM2 {
    pub new_pile_base_area_m2: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangePileBaseAreaM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pile-base-area-m2", kind: "change-pile-base-area-m2", record: "ChangedPileBaseAreaM2" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change pile base area [m2] to {}", self.new_pile_base_area_m2)
    }
}
//#endregion 🔖️ChangePileBaseAreaM2
