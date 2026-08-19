//! 🔧 `change-hr-savings-reference-kwh` payload — changes the Din16798 document's `hr_savings_reference_kwh` (heat recovery savings reference).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHrSavingsReferenceKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHrSavingsReferenceKwh {
    pub new_hr_savings_reference_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHrSavingsReferenceKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hr-savings-reference-kwh", kind: "change-hr-savings-reference-kwh", record: "ChangedHrSavingsReferenceKwh" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_hr_savings_reference_kwh::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_hr_savings_reference_kwh::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change heat recovery savings reference to {}", self.new_hr_savings_reference_kwh)
    }
}
//#endregion 🔖️ChangeHrSavingsReferenceKwh
