use super::*;
use crate::create_character_style::CreateCharacterStyle;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn delete_character_style_removes_an_unused_style() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let created = protocol::apply_diff(LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), index: None }).diff(&base).diff(), &base).expect("create");
    let mutation = LayoutMutation::DeleteCharacterStyle(DeleteCharacterStyle { id: "character.emph".into() });
    let next = protocol::apply_diff(mutation.diff(&created).diff(), &created).expect("delete applies");
    assert!(next.character_styles.is_empty());
}
