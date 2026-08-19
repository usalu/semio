//! 🌉 `change-span-m` — sets the En 1994 beam span [m] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSpanM {
    pub new_span_m: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeSpanM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "span-m", kind: "change-span-m", record: "ChangedSpanM" };

    async fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change span to {}", self.new_span_m)
    }
}
//#endregion 🔖️Payload
