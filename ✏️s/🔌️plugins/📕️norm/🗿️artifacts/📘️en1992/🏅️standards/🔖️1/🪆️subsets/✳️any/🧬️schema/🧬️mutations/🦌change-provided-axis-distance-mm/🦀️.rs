//! 🔧 `change-provided-axis-distance-mm` payload — changes the En1992 document's `provided_axis_distance_mm` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_provided_axis_distance_mm::ChangeProvidedAxisDistanceMm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeProvidedAxisDistanceMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeProvidedAxisDistanceMm {
    pub new_provided_axis_distance_mm: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeProvidedAxisDistanceMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "provided-axis-distance-mm", kind: "change-provided-axis-distance-mm", record: "ChangedProvidedAxisDistanceMm" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change provided axis distance mm to {:?}", self.new_provided_axis_distance_mm)
    }
}
//#endregion 🔖️ChangeProvidedAxisDistanceMm
