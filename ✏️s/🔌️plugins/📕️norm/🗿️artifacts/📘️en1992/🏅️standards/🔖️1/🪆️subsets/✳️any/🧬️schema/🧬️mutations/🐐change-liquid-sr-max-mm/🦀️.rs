//! 🔧 `change-liquid-sr-max-mm` payload — changes the En1992 document's `liquid_s_r_max_mm` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_liquid_s_r_max_mm::ChangeLiquidSRMaxMm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLiquidSRMaxMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLiquidSRMaxMm {
    pub new_liquid_s_r_max_mm: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeLiquidSRMaxMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "liquid-sr-max-mm", kind: "change-liquid-sr-max-mm", record: "ChangedLiquidSRMaxMm" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change liquid s r max mm to {:?}", self.new_liquid_s_r_max_mm)
    }
}
//#endregion 🔖️ChangeLiquidSRMaxMm
