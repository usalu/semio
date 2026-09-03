//! 🗑️ `delete-story` — removes a {@link TextStory} by id; inverse recreates it via `create-story`.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::{LayoutMutation, create_story};
use crate::artifacts::layout::schema::diff::LayoutStoriesDelta;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeleteStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteStory {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "story", kind: "delete-story", record: "DeletedStory" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_story(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_delete_story(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete story \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteStory


//#region 🗑️DeleteStory
pub async fn diff_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.stories.iter().any(|story| story.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteStory


//#region 🗑️DeleteStory
pub async fn inverse_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.stories.iter().position(|story| story.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreateStory(create_story::CreateStory { story: base.stories[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteStory
