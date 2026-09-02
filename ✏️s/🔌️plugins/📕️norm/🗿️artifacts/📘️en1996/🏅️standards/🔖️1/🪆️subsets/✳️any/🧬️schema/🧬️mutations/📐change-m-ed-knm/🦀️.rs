//! 📐 `change-m-ed-knm` payload — changes the En1996 document's `m_ed_knm` (design bending moment M_Ed [kNm]).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_m_ed_knm::ChangeMEdKnm;

//#region 🔖️ChangeMEdKnm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMEdKnm {
    pub new_m_ed_knm: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeMEdKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "m-ed-knm", kind: "change-m-ed-knm", record: "ChangedMEdKnm" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design bending moment M_Ed [kNm] to {}", self.new_m_ed_knm)
    }
}
//#endregion 🔖️ChangeMEdKnm
