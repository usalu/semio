//! 📝 `edit-story` — splices a story's authored `content` body: `delete` characters from `offset` are replaced by `insert`.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutStoriesDelta, LayoutStoriesModification};
use crate::{LayoutDiff, LayoutSnapshot, TextStoryPatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📝EditStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct EditStory {
    pub id: String,
    /// 📍 Character offset the splice starts at.
    pub offset: usize,
    /// ✂️ Characters removed from `offset`.
    pub delete: usize,
    /// ✍️ Text put in their place.
    pub insert: String,
}

/// ✂️ The characters of `content` a splice replaces, or `None` when the range leaves the text.
fn spliced_range(content: &str, offset: usize, delete: usize) -> Option<(usize, usize)> {
    let length = content.chars().count();
    (offset.checked_add(delete)? <= length).then(|| {
        let start = content.char_indices().nth(offset).map_or(content.len(), |(at, _)| at);
        let end = content.char_indices().nth(offset + delete).map_or(content.len(), |(at, _)| at);
        (start, end)
    })
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
    let Some((start, end)) = spliced_range(&story.content, payload.offset, payload.delete) else {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("The splice leaves story \"{}\".", payload.id), [payload.id.clone()]);
    };
    if story.content[start..end] == payload.insert {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Story \"{}\" content is unchanged.", payload.id));
    }
    let content = format!("{}{}{}", &story.content[..start], payload.insert, &story.content[end..]);
    protocol::MutationOutcome::new(LayoutDiff {
        stories: Some(LayoutStoriesDelta { modified: vec![LayoutStoriesModification { id: payload.id.clone(), patch: TextStoryPatch { content: Some(content), style_runs: None } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 📝EditStory

//#region 📝EditStory
pub fn inverse_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else { return Vec::new() };
    let Some((start, end)) = spliced_range(&story.content, payload.offset, payload.delete) else { return Vec::new() };
    vec![LayoutMutation::EditStory(EditStory { id: payload.id.clone(), offset: payload.offset, delete: payload.insert.chars().count(), insert: story.content[start..end].to_string() })]

    })())
}
//#endregion 📝EditStory
