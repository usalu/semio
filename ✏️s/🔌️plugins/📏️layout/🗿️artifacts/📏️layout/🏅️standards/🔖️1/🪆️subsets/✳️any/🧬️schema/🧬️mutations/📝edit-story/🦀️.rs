//! 📝 `edit-story` — replaces a story's authored `content` body.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, TextStoryPatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutStoriesDelta, LayoutStoryPatchEntry};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📝EditStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct EditStory {
    pub id: String,
    pub new_content: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for EditStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "story", kind: "edit-story", record: "EditedStory" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_edit_story(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_edit_story(self, base)
    }
    async fn label(&self) -> String {
        format!("Edit story \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 📝EditStory


//#region 📝EditStory
pub async fn diff_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if story.content == payload.new_content {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Story \"{}\" content is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        stories: Some(LayoutStoriesDelta { patched: vec![LayoutStoryPatchEntry { id: payload.id.clone(), patch: TextStoryPatch { content: Some(payload.new_content.clone()) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 📝EditStory


//#region 📝EditStory
pub async fn inverse_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.stories.iter().find(|story| story.id == payload.id) {
        Some(story) => vec![LayoutMutation::EditStory(EditStory { id: payload.id.clone(), new_content: story.content.clone() })],
        None => Vec::new(),
    }
}
//#endregion 📝EditStory
