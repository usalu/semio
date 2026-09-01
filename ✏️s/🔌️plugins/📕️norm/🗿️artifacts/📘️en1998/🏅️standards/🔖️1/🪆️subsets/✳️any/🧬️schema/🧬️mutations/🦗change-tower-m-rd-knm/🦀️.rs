//! 🦗 `change-tower-m-rd-knm` payload — changes the En1998 document's `tower_m_rd_knm` (tower moment resistance M_Rd [kNm]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_tower_m_rd_knm::ChangeTowerMRdKnm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTowerMRdKnm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTowerMRdKnm {
    pub new_tower_m_rd_knm: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerMRdKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tower-m-rd-knm", kind: "change-tower-m-rd-knm", record: "ChangedTowerMRdKnm" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tower moment resistance M_Rd [kNm] to {}", self.new_tower_m_rd_knm)
    }
}
//#endregion 🔖️ChangeTowerMRdKnm
