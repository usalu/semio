use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfPage, PdfSnapshot};
use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::mutations::remove_page::RemovePage;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::insert_page::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<InsertPage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "insert-page");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = testkit::snapshots(runtime_fixture_bytes("../../../🧪️tests/🧫️fixtures/📥️insert-page/⬅️before.pdf"), runtime_fixture_bytes("../../../🧪️tests/🧫️fixtures/📥️insert-page/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::InsertPage(InsertPage { index: 1, page: PdfPage::new(10.0, 20.0) }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = crate::standards::v1_7::subsets::base::io::export::text_document(&[(200.0, 100.0, "a"), (300.0, 100.0, "b"), (400.0, 100.0, "c")]);
    assert_mutation_inverse_sum_law(&PdfMutation::InsertPage(InsertPage { index: 1, page: PdfPage::new(10.0, 20.0) }), &base).await;
}

fn runtime_fixture_bytes(relative: &str) -> &'static [u8] {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_slice())
}
