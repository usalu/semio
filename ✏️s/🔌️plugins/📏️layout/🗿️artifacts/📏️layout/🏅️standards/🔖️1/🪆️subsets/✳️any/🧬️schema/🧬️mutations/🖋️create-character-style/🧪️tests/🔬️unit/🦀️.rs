use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn create_character_style_appends_one_and_inverse_removes_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), index: None });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("create applies");
    assert_eq!(next.character_styles.len(), 1);
    assert_eq!(next.character_styles[0].name.as_deref(), Some("Emphasis"));
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    assert!(restored.character_styles.is_empty());
}
