//! 🔧 `change-duct-test-pressure-pa` payload — changes the Din16798 document's `duct_test_pressure_pa` (duct test pressure).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDuctTestPressurePa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDuctTestPressurePa {
    pub new_duct_test_pressure_pa: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDuctTestPressurePa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "duct-test-pressure-pa", kind: "change-duct-test-pressure-pa", record: "ChangedDuctTestPressurePa" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_duct_test_pressure_pa::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_duct_test_pressure_pa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change duct test pressure to {}", self.new_duct_test_pressure_pa)
    }
}
//#endregion 🔖️ChangeDuctTestPressurePa
