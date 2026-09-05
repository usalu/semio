//! 🐷 `change-wall-soil-gamma-kn-m3` payload — changes the En1998 document's `wall_soil_gamma_kn_m3` (wall backfill unit weight [kN/m3]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
//#region 🔖️ChangeWallSoilGammaKnM3
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeWallSoilGammaKnM3 {
    pub new_wall_soil_gamma_kn_m3: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeWallSoilGammaKnM3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "wall-soil-gamma-kn-m3", kind: "change-wall-soil-gamma-kn-m3", record: "ChangedWallSoilGammaKnM3" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change wall backfill unit weight [kN/m3] to {}", self.new_wall_soil_gamma_kn_m3)
    }
}
//#endregion 🔖️ChangeWallSoilGammaKnM3
