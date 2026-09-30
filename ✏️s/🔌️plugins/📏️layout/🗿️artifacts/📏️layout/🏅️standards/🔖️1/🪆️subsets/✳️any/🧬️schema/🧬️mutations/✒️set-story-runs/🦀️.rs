//! ✒️ `set-story-runs` — replaces the style runs on one story.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutStoriesDelta, LayoutStoryPatchEntry};
use crate::{LayoutDiff, LayoutSnapshot, TextStoryPatch, TextStyleRun};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStoryRuns {
    pub id: String,
    pub runs: Vec<TextStyleRun>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for SetStoryRuns {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "story-runs", kind: "set-story-runs", record: "SetStoryRuns" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_story_runs(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> { inverse_set_story_runs(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Set style runs on story \"{}\"", self.id), &format!("Formate von Textfluss \"{}\" setzen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

fn run_ok(story_len: usize, content: &str, run: &TextStyleRun, base: &LayoutSnapshot) -> bool {
    run.start <= run.end
        && run.end <= story_len
        && content.is_char_boundary(run.start)
        && content.is_char_boundary(run.end)
        && run.paragraph_style_id.as_ref().map(|id| base.paragraph_styles.iter().any(|style| style.id == *id)).unwrap_or(true)
        && run.character_style_id.as_ref().map(|id| base.character_styles.iter().any(|style| style.id == *id)).unwrap_or(true)
}

pub fn diff_set_story_runs(payload: &SetStoryRuns, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.runs.iter().any(|run| !run_ok(story.content.len(), &story.content, run, base)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A style run stays inside the story, on character boundaries, and names a style that exists.", std::iter::empty::<String>());
    }
    if story.style_runs == payload.runs {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Story style runs are already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        stories: Some(LayoutStoriesDelta { patched: vec![LayoutStoryPatchEntry { id: payload.id.clone(), patch: TextStoryPatch { content: None, style_runs: Some(payload.runs.clone()) } }], ..Default::default() }),
        ..Default::default()
    })
}

pub fn inverse_set_story_runs(payload: &SetStoryRuns, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(story) = base.stories.iter().find(|story| story.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::SetStoryRuns(SetStoryRuns { id: story.id.clone(), runs: story.style_runs.clone() })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
