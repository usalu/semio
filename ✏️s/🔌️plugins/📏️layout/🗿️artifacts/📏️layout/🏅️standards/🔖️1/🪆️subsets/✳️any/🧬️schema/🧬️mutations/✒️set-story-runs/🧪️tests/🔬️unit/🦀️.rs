use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_model::TextStyleRun;
#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::set_story_runs::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn set_story_runs_applies_a_character_style_and_inverse_clears_it() {
    let mut base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    base.character_styles.push(semio_s_artifact_layout_layout_model::CharacterStyle { id: "character-1".into(), name: Some("Emphasis".into()), font_family: None, font_size: Some(24.0), font_weight: None, italic: None, color: Some([1.0, 0.0, 0.0, 1.0]), tracking: None });
    let mutation = LayoutMutation::SetStoryRuns(SetStoryRuns { id: "story-1".into(), runs: vec![TextStyleRun { start: 0, end: 5, paragraph_style_id: None, character_style_id: Some("character-1".into()) }] });
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("runs apply");
    assert_eq!(next.stories[0].style_runs.len(), 1);
    assert_eq!(next.stories[0].style_runs[0].end, 5);
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert!(restored.stories[0].style_runs.is_empty());
}
