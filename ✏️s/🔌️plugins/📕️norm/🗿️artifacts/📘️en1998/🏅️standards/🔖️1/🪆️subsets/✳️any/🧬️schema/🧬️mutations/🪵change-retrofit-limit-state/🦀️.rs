//! 🪵 `change-retrofit-limit-state` payload — changes the En1998 document's `retrofit_limit_state` (retrofit limit state).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_retrofit_limit_state::ChangeRetrofitLimitState;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeRetrofitLimitState
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRetrofitLimitState {
    pub new_retrofit_limit_state: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeRetrofitLimitState {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "retrofit-limit-state", kind: "change-retrofit-limit-state", record: "ChangedRetrofitLimitState" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change retrofit limit state to \"{}\"", self.new_retrofit_limit_state)
    }
}
//#endregion 🔖️ChangeRetrofitLimitState
