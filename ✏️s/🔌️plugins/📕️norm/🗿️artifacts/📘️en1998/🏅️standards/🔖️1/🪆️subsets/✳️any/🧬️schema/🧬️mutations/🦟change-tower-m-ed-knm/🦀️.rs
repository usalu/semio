//! 🦟 `change-tower-m-ed-knm` payload — changes the En1998 document's `tower_m_ed_knm` (tower design moment M_Ed [kNm]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_tower_m_ed_knm::ChangeTowerMEdKnm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTowerMEdKnm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTowerMEdKnm {
    pub new_tower_m_ed_knm: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerMEdKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tower-m-ed-knm", kind: "change-tower-m-ed-knm", record: "ChangedTowerMEdKnm" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tower design moment M_Ed [kNm] to {}", self.new_tower_m_ed_knm)
    }
}
//#endregion 🔖️ChangeTowerMEdKnm
