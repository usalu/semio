use super::*;
use crate::mutations::create_character_style::CreateCharacterStyle;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn delete_character_style_removes_an_unused_style() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let created = LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()) }).diff(&base).diff().apply(&base).expect("create");
    let mutation = LayoutMutation::DeleteCharacterStyle(DeleteCharacterStyle { id: "character.emph".into() });
    let next = mutation.diff(&created).diff().apply(&created).expect("delete applies");
    assert!(next.character_styles.is_empty());
}
