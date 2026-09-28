use super::*;
use crate::mutations::create_character_style::CreateCharacterStyle;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_character_style_sets_italic_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let created = LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()) }).diff(&base).diff().apply(&base).expect("create");
    let mutation = LayoutMutation::UpdateCharacterStyle(UpdateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), font_family: Some("Layout Sans".into()), font_size: Some(14.0), font_weight: Some(700), italic: Some(true), color: None, tracking: Some(10.0) });
    let next = mutation.diff(&created).diff().apply(&created).expect("update applies");
    let style = &next.character_styles[0];
    assert_eq!(style.italic, Some(true));
    assert_eq!(style.font_size, Some(14.0));
    let restored = mutation.inverse(&created)[0].diff(&next).diff().apply(&next).expect("inverse applies");
    assert_eq!(restored.character_styles[0].italic, None);
}
