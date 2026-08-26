//! 🔧 `change-cooling-reference-kwh` payload — changes the Din16798 document's `cooling_reference_kwh` (cooling energy reference).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeCoolingReferenceKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeCoolingReferenceKwh {
    pub new_cooling_reference_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCoolingReferenceKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cooling-reference-kwh", kind: "change-cooling-reference-kwh", record: "ChangedCoolingReferenceKwh" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_cooling_reference_kwh::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_cooling_reference_kwh::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cooling energy reference to {}", self.new_cooling_reference_kwh)
    }
}
//#endregion 🔖️ChangeCoolingReferenceKwh
