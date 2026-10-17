use super::*;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Page".to_string()))])]);
    let page = ObjRef { num: 1, gen: 0 };
    let mutation = SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0], entry_index: None };
    let next = applied(&base, &PdfVtMutation::SetTrimBox(mutation.clone()));
    assert_eq!(support::page_box(&next, page, "TrimBox"), Some([1.0, 2.0, 300.0, 400.0]));
    assert_eq!(<SetTrimBox as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfVtMutation::RemoveTrimBox(RemoveTrimBox { page_index: 0 })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Page".to_string()))])]);
    assert_mutation_inverse_sum_law(&PdfVtMutation::SetTrimBox(SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0], entry_index: None }), &base).await;
}
