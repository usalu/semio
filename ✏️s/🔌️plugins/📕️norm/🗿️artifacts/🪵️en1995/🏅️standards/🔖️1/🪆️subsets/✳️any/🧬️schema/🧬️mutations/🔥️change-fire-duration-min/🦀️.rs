//! 🔧 `change-fire-duration-min` payload — changes the En1995 document's `fire_duration_min` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
//#region 🔖️ChangeFireDurationMin
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFireDurationMin {
    pub new_fire_duration_min: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeFireDurationMin {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fire-duration-min", kind: "change-fire-duration-min", record: "ChangedFireDurationMin" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fire duration min to {:?}", self.new_fire_duration_min)
    }
}
//#endregion 🔖️ChangeFireDurationMin
