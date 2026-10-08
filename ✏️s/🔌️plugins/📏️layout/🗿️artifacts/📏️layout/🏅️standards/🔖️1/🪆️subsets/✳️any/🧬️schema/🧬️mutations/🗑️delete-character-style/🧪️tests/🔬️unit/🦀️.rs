use super::*;
use crate::mutations::create_character_style::CreateCharacterStyle;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn delete_character_style_removes_an_unused_style() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let created = protocol::apply_diff(LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), index: None }).diff(&base).diff(), &base).expect("create");
    let mutation = LayoutMutation::DeleteCharacterStyle(DeleteCharacterStyle { id: "character.emph".into() });
    let next = protocol::apply_diff(mutation.diff(&created).diff(), &created).expect("delete applies");
    assert!(next.character_styles.is_empty());
}
