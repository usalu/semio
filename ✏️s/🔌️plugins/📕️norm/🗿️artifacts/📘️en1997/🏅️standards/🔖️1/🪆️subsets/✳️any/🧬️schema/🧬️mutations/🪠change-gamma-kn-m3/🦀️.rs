//! 🪠 `change-gamma-kn-m3` payload — changes the En1997 document's `gamma_kn_m3` (soil unit weight [kN/m3]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_gamma_kn_m3::ChangeGammaKnM3;

//#region 🔖️ChangeGammaKnM3
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeGammaKnM3 {
    pub new_gamma_kn_m3: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeGammaKnM3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "gamma-kn-m3", kind: "change-gamma-kn-m3", record: "ChangedGammaKnM3" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change soil unit weight [kN/m3] to {}", self.new_gamma_kn_m3)
    }
}
//#endregion 🔖️ChangeGammaKnM3
