//! ✒️ `set-story-runs` — replaces the style runs on one story.

#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::{LayoutStoriesDelta, TextStyleRunRow, TextStyleRunsDelta};
use semio_s_artifact_layout_layout_diff::LayoutStoriesModification;
use semio_s_artifact_layout_layout_diff::{LayoutDiff, TextStoryPatch};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::TextStyleRun;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStoryRuns {
    pub id: String,
    pub runs: Vec<TextStyleRun>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for SetStoryRuns where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::set_story_runs::SetStoryRuns> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "story-runs", kind: "set-story-runs", record: "SetStoryRuns" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Set style runs on story \"{}\"", self.id), &format!("Formate von Textfluss \"{}\" setzen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for SetStoryRuns where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::set_story_runs::SetStoryRuns> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_story_runs(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ inverse_set_story_runs(self, base)? 
    })
}
}

pub fn diff_set_story_runs(payload: &SetStoryRuns, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Err(message) = semio_s_artifact_layout_layout_model::validation::validate_style_runs(base, &story.content, &payload.runs) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, std::iter::empty::<String>());
    }
    if story.style_runs == payload.runs {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Story style runs are already set to that value.");
    }
    let rows: Vec<TextStyleRunRow> = payload.runs.iter().enumerate().filter(|(index, run)| story.style_runs.get(*index) != Some(*run)).map(|(index, run)| TextStyleRunRow { index, run: run.clone() }).collect();
    let len = (payload.runs.len() != story.style_runs.len()).then_some(payload.runs.len());
    protocol::MutationOutcome::new(LayoutDiff {
        stories: Some(LayoutStoriesDelta { modified: vec![LayoutStoriesModification { id: payload.id.clone(), patch: TextStoryPatch { content: None, style_runs: Some(TextStyleRunsDelta { len, rows }) } }], ..Default::default() }),
        ..Default::default()
    })
}

pub fn inverse_set_story_runs<Op: From<crate::set_story_runs::SetStoryRuns>>(payload: &SetStoryRuns, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else { return Vec::new() };
    vec![Op::from(SetStoryRuns { id: story.id.clone(), runs: story.style_runs.clone() })]

    })())
}

