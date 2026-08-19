//! 📐 `change-m-ed-knm` payload — changes the En1996 document's `m_ed_knm` (design bending moment M_Ed [kNm]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeMEdKnm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMEdKnm {
    pub new_m_ed_knm: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeMEdKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "m-ed-knm", kind: "change-m-ed-knm", record: "ChangedMEdKnm" };

    async fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_m_ed_knm::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_m_ed_knm::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change design bending moment M_Ed [kNm] to {}", self.new_m_ed_knm)
    }
}
//#endregion 🔖️ChangeMEdKnm
