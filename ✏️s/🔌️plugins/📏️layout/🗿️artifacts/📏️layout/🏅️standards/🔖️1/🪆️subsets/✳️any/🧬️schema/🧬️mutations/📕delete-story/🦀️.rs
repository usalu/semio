//! 🗑️ `delete-story` — removes a {@link TextStory} by id; inverse recreates it via `create-story`.

use crate::mutations::{create_story, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::{LayoutStoriesDelta, LayoutStoryRemoval};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeleteStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteStory {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "story", kind: "delete-story", record: "DeletedStory" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_delete_story(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete story \"{}\"", self.id), &format!("Textfluss \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteStory

//#region 🗑️DeleteStory
pub fn diff_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.stories.iter().position(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { removed: vec![LayoutStoryRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteStory

//#region 🗑️DeleteStory
pub fn inverse_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.stories.iter().position(|story| story.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreateStory(create_story::CreateStory { story: base.stories[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🗑️DeleteStory
