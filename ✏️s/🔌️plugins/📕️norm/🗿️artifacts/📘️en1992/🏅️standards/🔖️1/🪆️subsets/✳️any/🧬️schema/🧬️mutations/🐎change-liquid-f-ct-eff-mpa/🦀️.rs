//! 🔧 `change-liquid-f-ct-eff-mpa` payload — changes the En1992 document's `liquid_f_ct_eff_mpa` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_liquid_f_ct_eff_mpa::ChangeLiquidFCtEffMpa;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLiquidFCtEffMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLiquidFCtEffMpa {
    pub new_liquid_f_ct_eff_mpa: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeLiquidFCtEffMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "liquid-f-ct-eff-mpa", kind: "change-liquid-f-ct-eff-mpa", record: "ChangedLiquidFCtEffMpa" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change liquid f ct eff mpa to {:?}", self.new_liquid_f_ct_eff_mpa)
    }
}
//#endregion 🔖️ChangeLiquidFCtEffMpa
