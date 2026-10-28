use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
#[allow(unused_imports)] use protocol::{MutationKind, SemanticDescriptor};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_mutations_content::update_parent_page::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn update_parent_page_renames_and_inverse_restores_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let mutation = LayoutMutation::UpdateParentPage(UpdateParentPage { id: "parent-1".into(), name: "Cover master".into(), width: 420.0, height: 500.0 });
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("parent applies");
    assert_eq!(next.parent_pages[0].name, "Cover master");
    assert_eq!(next.parent_pages[0].width, 420.0);
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert_eq!(restored.parent_pages[0].name, "Master");
}
