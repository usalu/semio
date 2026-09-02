//! 🛁 `change-n-pile-ed-kn` payload — changes the En1997 document's `n_pile_ed_kn` (design pile axial load N_Ed [kN]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_n_pile_ed_kn::ChangeNPileEdKn;

//#region 🔖️ChangeNPileEdKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeNPileEdKn {
    pub new_n_pile_ed_kn: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeNPileEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-pile-ed-kn", kind: "change-n-pile-ed-kn", record: "ChangedNPileEdKn" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design pile axial load N_Ed [kN] to {}", self.new_n_pile_ed_kn)
    }
}
//#endregion 🔖️ChangeNPileEdKn
