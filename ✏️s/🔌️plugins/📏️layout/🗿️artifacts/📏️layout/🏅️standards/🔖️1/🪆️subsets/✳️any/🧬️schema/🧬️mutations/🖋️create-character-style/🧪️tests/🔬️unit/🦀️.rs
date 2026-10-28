use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::create_character_style::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn create_character_style_appends_one_and_inverse_removes_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let mutation = LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), index: None });
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("create applies");
    assert_eq!(next.character_styles.len(), 1);
    assert_eq!(next.character_styles[0].name.as_deref(), Some("Emphasis"));
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    assert!(restored.character_styles.is_empty());
}
