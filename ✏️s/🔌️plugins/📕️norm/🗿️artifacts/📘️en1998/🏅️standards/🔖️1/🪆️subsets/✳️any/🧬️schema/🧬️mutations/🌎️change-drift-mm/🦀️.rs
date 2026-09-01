//! 🌎️ `change-drift-mm` payload — changes the En1998 document's `drift_mm` (interstorey drift [mm]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_drift_mm::ChangeDriftMm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDriftMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDriftMm {
    pub new_drift_mm: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeDriftMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "drift-mm", kind: "change-drift-mm", record: "ChangedDriftMm" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change interstorey drift [mm] to {}", self.new_drift_mm)
    }
}
//#endregion 🔖️ChangeDriftMm
