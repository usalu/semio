//! ✒️ `change-schema` — sets the playground document's `schema` metadata field. This artifact's
//! whole persistent snapshot is a single opaque `schema` string (a demonstrator stub with no other
//! structured content today) — see `📓️derivation-rules.md`'s metadata-only-mutation allowance for
//! a trivial snapshot.

use crate::artifacts::playground::standards::v1::subsets::any::schema::{
    mutations::PlaygroundMutation,
    snapshot::PlaygroundSnapshot,
};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSchema {
    pub new_schema: String,
}

impl protocol::MutationKind<PlaygroundSnapshot, PlaygroundMutation> for ChangeSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "playground", kind: "change-schema", record: "ChangedSchema" };

    async fn diff(&self, base: &PlaygroundSnapshot) -> protocol::MutationOutcome<<PlaygroundMutation as protocol::Mutation<PlaygroundSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &PlaygroundSnapshot) -> Vec<PlaygroundMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change playground schema to \"{}\"", self.new_schema)
    }
    async fn target(&self) -> Vec<String> {
        vec!["schema".into()]
    }
}
//#endregion 🔖️Payload
