//! 🔧 `change-liquid-rho-p-eff` payload — changes the En1992 document's `liquid_rho_p_eff` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLiquidRhoPEff
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLiquidRhoPEff {
    pub new_liquid_rho_p_eff: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeLiquidRhoPEff {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "liquid-rho-p-eff", kind: "change-liquid-rho-p-eff", record: "ChangedLiquidRhoPEff" };

    async fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        crate::artifacts::en1992::mutations::change_liquid_rho_p_eff::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_liquid_rho_p_eff::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change liquid rho p eff to {:?}", self.new_liquid_rho_p_eff)
    }
}
//#endregion 🔖️ChangeLiquidRhoPEff
