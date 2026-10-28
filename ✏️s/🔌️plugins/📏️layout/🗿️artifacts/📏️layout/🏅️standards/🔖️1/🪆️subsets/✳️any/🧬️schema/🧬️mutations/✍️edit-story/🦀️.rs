//! 📝 `edit-story` — splices a story's authored `content` body: `delete` characters from `offset` are replaced by `insert`.

use semio_s_artifact_layout_layout_diff::LayoutStoriesDelta;
use semio_s_artifact_layout_layout_diff::LayoutStoriesModification;
use semio_s_artifact_layout_layout_diff::{LayoutDiff, TextStoryPatch, TextStyleRunsDelta, TextStyleRunRow};
use semio_s_artifact_layout_layout_model::story_splice::{byte_range, splice};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📝EditStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
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

impl<Op> MutationKind<LayoutSnapshot, Op> for EditStory where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::edit_story::EditStory> + From<crate::set_story_runs::SetStoryRuns> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "story", kind: "edit-story", record: "EditedStory" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit story \"{}\"", self.id), &format!("Textfluss \"{}\" bearbeiten", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for EditStory where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::edit_story::EditStory> + From<crate::set_story_runs::SetStoryRuns> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_edit_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_edit_story(self, base)?
    
    })
}
}
//#endregion 📝EditStory

//#region 📝EditStory
pub fn diff_edit_story(payload: &EditStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some((start, end)) = byte_range(&story.content, payload.offset, payload.delete) else {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("The splice leaves story \"{}\".", payload.id), [payload.id.clone()]);
    };
    if story.content[start..end] == payload.insert {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Story \"{}\" content is unchanged.", payload.id));
    }
    let Some(next) = splice(story, payload.offset, payload.delete, &payload.insert) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The spliced style ranges exceed the text address space.", [payload.id.clone()]);
    };
    let style_runs = (story.style_runs != next.style_runs).then(|| TextStyleRunsDelta {
        len: Some(next.style_runs.len()),
        rows: next.style_runs.iter().enumerate().map(|(index, run)| TextStyleRunRow { index, run: run.clone() }).collect(),
    });
    protocol::MutationOutcome::new(LayoutDiff {
        stories: Some(LayoutStoriesDelta { modified: vec![LayoutStoriesModification { id: payload.id.clone(), patch: TextStoryPatch { content: Some(next.content), style_runs } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 📝EditStory

//#region 📝EditStory
pub fn inverse_edit_story<Op: From<crate::edit_story::EditStory> + From<crate::set_story_runs::SetStoryRuns>>(payload: &EditStory, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_edit_story(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()); }
    Ok((|| {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else { return Vec::new() };
    let Some((start, end)) = byte_range(&story.content, payload.offset, payload.delete) else { return Vec::new() };
    let mut inverse = Vec::new();
    if !story.style_runs.is_empty() { inverse.push(Op::from(crate::set_story_runs::SetStoryRuns { id: story.id.clone(), runs: story.style_runs.clone() })); }
    inverse.push(Op::from(EditStory { id: payload.id.clone(), offset: payload.offset, delete: payload.insert.chars().count(), insert: story.content[start..end].to_string() }));
    inverse

    })())
}
//#endregion 📝EditStory

