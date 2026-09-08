//! 🗑️ `delete-story` — removes a {@link TextStory} by id; inverse recreates it via `create-story`.


use crate::{LayoutDiff, LayoutSnapshot};
use crate::mutations::{LayoutMutation, create_story};
use crate::schema::diff::LayoutStoriesDelta;
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
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_delete_story(self, base)
    }
    fn label(&self) -> String {
        format!("Delete story \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteStory


//#region 🗑️DeleteStory
pub fn diff_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.stories.iter().any(|story| story.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteStory


//#region 🗑️DeleteStory
pub fn inverse_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.stories.iter().position(|story| story.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreateStory(create_story::CreateStory { story: base.stories[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteStory
