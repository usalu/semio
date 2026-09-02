//! 🐍 `change-m-ed-knm` payload — changes the En1999 document's `m_ed_knm` (design bending moment M_Ed [kNm]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_m_ed_knm::ChangeMEdKnm;

//#region 🔖️ChangeMEdKnm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMEdKnm {
    pub new_m_ed_knm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeMEdKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "m-ed-knm", kind: "change-m-ed-knm", record: "ChangedMEdKnm" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design bending moment M_Ed [kNm] to {}", self.new_m_ed_knm)
    }
}
//#endregion 🔖️ChangeMEdKnm
