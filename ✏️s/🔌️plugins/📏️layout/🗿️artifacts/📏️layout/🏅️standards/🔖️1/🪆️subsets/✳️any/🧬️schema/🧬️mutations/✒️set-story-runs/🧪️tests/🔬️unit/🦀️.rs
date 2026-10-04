use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn set_story_runs_applies_a_character_style_and_inverse_clears_it() {
    let mut base = crate::standards::v1::subsets::any::schema::default_document();
    base.character_styles.push(crate::CharacterStyle { id: "character-1".into(), name: Some("Emphasis".into()), font_family: None, font_size: Some(24.0), font_weight: None, italic: None, color: Some([1.0, 0.0, 0.0, 1.0]), tracking: None });
    let mutation = LayoutMutation::SetStoryRuns(SetStoryRuns { id: "story-1".into(), runs: vec![TextStyleRun { start: 0, end: 5, paragraph_style_id: None, character_style_id: Some("character-1".into()) }] });
    let next = mutation.diff(&base).diff().apply(&base).expect("runs apply");
    assert_eq!(next.stories[0].style_runs.len(), 1);
    assert_eq!(next.stories[0].style_runs[0].end, 5);
    let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff().apply(&next).expect("inverse");
    assert!(restored.stories[0].style_runs.is_empty());
}
