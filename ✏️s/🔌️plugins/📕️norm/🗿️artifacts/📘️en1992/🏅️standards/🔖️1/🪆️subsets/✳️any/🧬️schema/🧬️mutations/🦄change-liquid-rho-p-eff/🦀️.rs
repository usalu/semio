//! 🔧 `change-liquid-rho-p-eff` payload — changes the En1992 document's `liquid_rho_p_eff` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_liquid_rho_p_eff::ChangeLiquidRhoPEff;

//#region 🔖️ChangeLiquidRhoPEff
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeLiquidRhoPEff {
    pub new_liquid_rho_p_eff: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeLiquidRhoPEff {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "liquid-rho-p-eff", kind: "change-liquid-rho-p-eff", record: "ChangedLiquidRhoPEff" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change liquid rho p eff to {:?}", self.new_liquid_rho_p_eff)
    }
}
//#endregion 🔖️ChangeLiquidRhoPEff
