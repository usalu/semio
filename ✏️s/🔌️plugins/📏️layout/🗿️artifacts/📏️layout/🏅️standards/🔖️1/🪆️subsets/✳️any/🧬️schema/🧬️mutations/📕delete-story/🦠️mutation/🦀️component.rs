//! 🗑️ `delete-story` — removes a {@link TextStory} by id; inverse recreates it via `create-story`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🗑️DeleteStory
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteStory {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "story", kind: "delete-story", record: "DeletedStory" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_delete_story(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_delete_story(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete story \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteStory
