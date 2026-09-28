use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_paragraph_style_changes_size_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
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
    let next = mutation.diff(&base).diff().apply(&base).expect("style applies");
    let updated = next.paragraph_styles.iter().find(|style| style.id == "paragraph.body").unwrap();
    assert_eq!(updated.font_size, 16.0);
    assert_eq!(updated.leading, 19.2);
    assert_eq!(updated.alignment, "center");
    assert_eq!(next.pages, base.pages);
    let restored = mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse applies");
    assert_eq!(restored.paragraph_styles, base.paragraph_styles);
}
