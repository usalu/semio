//! 📝 `edit-story` — replaces a story's authored `content` body.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutStoriesDelta, LayoutStoryPatchEntry};
use crate::{LayoutDiff, LayoutSnapshot, TextStoryPatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📝EditStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct EditStory {
    pub id: String,
    pub new_content: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for EditStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "story", kind: "edit-story", record: "EditedStory" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_edit_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_edit_story(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit story \"{}\"", self.id), &format!("Textfluss \"{}\" bearbeiten", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 📝EditStory

//#region 📝EditStory
pub fn diff_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if story.content == payload.new_content {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Story \"{}\" content is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        stories: Some(LayoutStoriesDelta { patched: vec![LayoutStoryPatchEntry { id: payload.id.clone(), patch: TextStoryPatch { content: Some(payload.new_content.clone()), style_runs: None } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 📝EditStory

//#region 📝EditStory
pub fn inverse_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.stories.iter().find(|story| story.id == payload.id) {
        Some(story) => vec![LayoutMutation::EditStory(EditStory { id: payload.id.clone(), new_content: story.content.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 📝EditStory
