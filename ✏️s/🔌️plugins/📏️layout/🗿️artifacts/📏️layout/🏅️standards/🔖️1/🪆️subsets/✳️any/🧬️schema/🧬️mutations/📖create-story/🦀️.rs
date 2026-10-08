//! 📖 `create-story` — brings a new {@link TextStory} into existence in the id-keyed `stories`
//! collection.

use crate::mutations::{delete_story, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::{insertion_order, LayoutStoriesDelta};
use crate::{LayoutDiff, LayoutSnapshot, TextStory};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📖CreateStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateStory {
    pub story: TextStory,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "story", kind: "create-story", record: "CreatedStory" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_create_story(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create story \"{}\"", self.story.id), &format!("Textfluss \"{}\" erstellen", self.story.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.story.id.clone()]
    }
}
//#endregion 📖CreateStory

//#region 📖CreateStory
pub fn diff_create_story(payload: &CreateStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.stories.iter().any(|story| story.id == payload.story.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A story with id \"{}\" already exists.", payload.story.id), [payload.story.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { added: vec![payload.story.clone()], reordered: insertion_order(base.stories.iter().map(|story| story.id.as_str()), &payload.story.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 📖CreateStory

//#region 📖CreateStory
pub fn inverse_create_story(payload: &CreateStory, _base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![LayoutMutation::DeleteStory(delete_story::DeleteStory { id: payload.story.id.clone() })]

    })())
}
//#endregion 📖CreateStory
