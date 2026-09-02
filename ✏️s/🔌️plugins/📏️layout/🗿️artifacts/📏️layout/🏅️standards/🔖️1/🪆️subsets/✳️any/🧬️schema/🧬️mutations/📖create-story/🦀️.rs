//! 📖 `create-story` — brings a new {@link TextStory} into existence in the id-keyed `stories`
//! collection.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, TextStory};
use crate::artifacts::layout::mutations::{LayoutMutation, delete_story};
use crate::artifacts::layout::schema::diff::LayoutStoriesDelta;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📖CreateStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateStory {
    pub story: TextStory,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "story", kind: "create-story", record: "CreatedStory" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_story(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_create_story(self, base)
    }
    async fn label(&self) -> String {
        format!("Create story \"{}\"", self.story.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.story.id.clone()]
    }
}
//#endregion 📖CreateStory


//#region 📖CreateStory
pub async fn diff_create_story(payload: &CreateStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.stories.iter().any(|story| story.id == payload.story.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A story with id \"{}\" already exists.", payload.story.id), [payload.story.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { added: vec![payload.story.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 📖CreateStory


//#region 📖CreateStory
pub async fn inverse_create_story(payload: &CreateStory, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeleteStory(delete_story::DeleteStory { id: payload.story.id.clone() })]
}
//#endregion 📖CreateStory
