use super::*;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::catalog_object()]);
    let mutation = SetMarkInfo { marked: true, entry_index: None };
    let next = applied(&base, &PdfUaMutation::SetMarkInfo(mutation.clone()));
    assert_eq!(support::catalog_flag(&next, "MarkInfo", "Marked"), Some(true));
    assert_eq!(<SetMarkInfo as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::RemoveMarkInfo(RemoveMarkInfo {})]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfUaMutation::SetMarkInfo(SetMarkInfo { marked: true, entry_index: None }), &base).await;
}
