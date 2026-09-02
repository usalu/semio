//! 🏞️ `change-bearing-d-rd-mm` payload — changes the En1998 document's `bearing_d_rd_mm` (bearing design displacement capacity D_Rd [mm]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_bearing_d_rd_mm::ChangeBearingDRdMm;

//#region 🔖️ChangeBearingDRdMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBearingDRdMm {
    pub new_bearing_d_rd_mm: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeBearingDRdMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bearing-d-rd-mm", kind: "change-bearing-d-rd-mm", record: "ChangedBearingDRdMm" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change bearing design displacement capacity D_Rd [mm] to {}", self.new_bearing_d_rd_mm)
    }
}
//#endregion 🔖️ChangeBearingDRdMm
