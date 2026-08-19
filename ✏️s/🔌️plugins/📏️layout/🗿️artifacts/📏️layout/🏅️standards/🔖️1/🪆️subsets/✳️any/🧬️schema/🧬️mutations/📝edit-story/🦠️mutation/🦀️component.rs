//! 📝 `edit-story` — replaces a story's authored `content` body.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 📝EditStory
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditStory {
    pub id: String,
    pub new_content: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for EditStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "story", kind: "edit-story", record: "EditedStory" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_edit_story(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_edit_story(self, base)
    }
    async fn label(&self) -> String {
        format!("Edit story \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 📝EditStory
