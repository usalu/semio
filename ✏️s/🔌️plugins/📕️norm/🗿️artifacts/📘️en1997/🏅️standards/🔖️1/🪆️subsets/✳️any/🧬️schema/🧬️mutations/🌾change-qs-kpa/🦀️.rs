//! 🌾 `change-qs-kpa` payload — changes the En1997 document's `q_s_kpa` (shaft resistance q_s [kPa]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_q_s_kpa::ChangeQSKpa;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeQSKpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeQSKpa {
    pub new_q_s_kpa: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeQSKpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "qs-kpa", kind: "change-qs-kpa", record: "ChangedQSKpa" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change shaft resistance q_s [kPa] to {}", self.new_q_s_kpa)
    }
}
//#endregion 🔖️ChangeQSKpa
