#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::{create_character_style, update_character_style};
#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::delete_character_style::*;
use semio_s_artifact_layout_layout_mutations_content::create_character_style::CreateCharacterStyle;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn delete_character_style_removes_an_unused_style() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let created = protocol::apply_diff_cold(LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), index: None }).diff(&base).diff(), &base).expect("create");
    let mutation = LayoutMutation::DeleteCharacterStyle(DeleteCharacterStyle { id: "character.emph".into() });
    let next = protocol::apply_diff_cold(mutation.diff(&created).diff(), &created).expect("delete applies");
    assert!(next.character_styles.is_empty());
}
