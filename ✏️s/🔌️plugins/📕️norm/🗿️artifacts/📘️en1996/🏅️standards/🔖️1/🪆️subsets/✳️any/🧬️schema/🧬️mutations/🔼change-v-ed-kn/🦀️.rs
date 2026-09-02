//! 🔼 `change-v-ed-kn` payload — changes the En1996 document's `v_ed_kn` (design shear force V_Ed [kN]).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_v_ed_kn::ChangeVEdKn;

//#region 🔖️ChangeVEdKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeVEdKn {
    pub new_v_ed_kn: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeVEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "v-ed-kn", kind: "change-v-ed-kn", record: "ChangedVEdKn" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design shear force V_Ed [kN] to {}", self.new_v_ed_kn)
    }
}
//#endregion 🔖️ChangeVEdKn
