//! 🔧 `change-duct-leakage-m3-sm2` payload — changes the Din16798 document's `duct_leakage_m3_s_m2` (duct leakage rate).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeDuctLeakageM3SM2
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeDuctLeakageM3SM2 {
    pub new_duct_leakage_m3_s_m2: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDuctLeakageM3SM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "duct-leakage-m3-sm2", kind: "change-duct-leakage-m3-sm2", record: "ChangedDuctLeakageM3SM2" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change duct leakage rate to {}", self.new_duct_leakage_m3_s_m2)
    }
}
//#endregion 🔖️ChangeDuctLeakageM3SM2
