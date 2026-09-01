//! 🏖️ `change-period-ratio` payload — changes the En1998 document's `period_ratio` (period ratio).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_period_ratio::ChangePeriodRatio;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangePeriodRatio
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangePeriodRatio {
    pub new_period_ratio: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangePeriodRatio {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "period-ratio", kind: "change-period-ratio", record: "ChangedPeriodRatio" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change period ratio to {}", self.new_period_ratio)
    }
}
//#endregion 🔖️ChangePeriodRatio
