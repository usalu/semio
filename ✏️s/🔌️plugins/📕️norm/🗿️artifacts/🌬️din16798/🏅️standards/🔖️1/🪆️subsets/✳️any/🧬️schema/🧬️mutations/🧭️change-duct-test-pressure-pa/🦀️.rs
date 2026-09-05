//! 🔧 `change-duct-test-pressure-pa` payload — changes the Din16798 document's `duct_test_pressure_pa` (duct test pressure).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeDuctTestPressurePa
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeDuctTestPressurePa {
    pub new_duct_test_pressure_pa: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDuctTestPressurePa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "duct-test-pressure-pa", kind: "change-duct-test-pressure-pa", record: "ChangedDuctTestPressurePa" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change duct test pressure to {}", self.new_duct_test_pressure_pa)
    }
}
//#endregion 🔖️ChangeDuctTestPressurePa
