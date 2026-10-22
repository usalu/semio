#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::update_paragraph_style::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn update_paragraph_style_changes_size_and_inverse_restores_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let style = base.paragraph_styles.iter().find(|style| style.id == "paragraph.body").unwrap();
    let mutation = LayoutMutation::UpdateParagraphStyle(UpdateParagraphStyle {
        id: style.id.clone(),
        name: style.name.clone(),
        font_family: style.font_family.clone(),
        font_size: 16.0,
        font_weight: style.font_weight,
        leading: 19.2,
        tracking: style.tracking,
        alignment: "center".into(),
    });
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("style applies");
    let updated = next.paragraph_styles.iter().find(|style| style.id == "paragraph.body").unwrap();
    assert_eq!(updated.font_size, 16.0);
    assert_eq!(updated.leading, 19.2);
    assert_eq!(updated.alignment, "center");
    assert_eq!(next.pages, base.pages);
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    assert_eq!(restored.paragraph_styles, base.paragraph_styles);
}
