#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::set_page_parent::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn set_page_parent_clears_the_master_and_inverse_restores_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let mutation = LayoutMutation::SetPageParent(SetPageParent { id: "page-1".into(), parent_page_id: None });
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("parent clears");
    assert_eq!(next.pages[0].parent_page_id, None);
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert_eq!(restored.pages[0].parent_page_id.as_deref(), Some("parent-1"));
}
