#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::update_character_style::*;
use semio_s_artifact_layout_layout_mutations_content::create_character_style::CreateCharacterStyle;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_character_style_sets_italic_and_inverse_restores_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let created = protocol::apply_diff(LayoutMutation::CreateCharacterStyle(CreateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), index: None }).diff(&base).diff(), &base).expect("create");
    let mutation = LayoutMutation::UpdateCharacterStyle(UpdateCharacterStyle { id: "character.emph".into(), name: Some("Emphasis".into()), font_family: Some("Layout Sans".into()), font_size: Some(14.0), font_weight: Some(700), italic: Some(true), color: None, tracking: Some(10.0) });
    let next = protocol::apply_diff(mutation.diff(&created).diff(), &created).expect("update applies");
    let style = &next.character_styles[0];
    assert_eq!(style.italic, Some(true));
    assert_eq!(style.font_size, Some(14.0));
    let restored = protocol::apply_diff(mutation.inverse(&created).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    assert_eq!(restored.character_styles[0].italic, None);
}
