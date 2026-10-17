use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::mutations::insert_page::InsertPage;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::remove_page::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemovePage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-page");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = testkit::snapshots(runtime_fixture_bytes("../../../🧪️tests/🧫️fixtures/🗑️remove-page/⬅️before.pdf"), runtime_fixture_bytes("../../../🧪️tests/🧫️fixtures/🗑️remove-page/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemovePage(RemovePage { index: 0 }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, _after) = testkit::snapshots(runtime_fixture_bytes("../../../🧪️tests/🧫️fixtures/🗑️remove-page/⬅️before.pdf"), runtime_fixture_bytes("../../../🧪️tests/🧫️fixtures/🗑️remove-page/➡️after.pdf"));
    for (offset, page) in base.pages.iter_mut().enumerate() {
        page.media_box[2] += offset as f64;
    }
    let mut first = base.pages[0].clone();
    first.media_box[2] += 100.0;
    let mut last = base.pages[0].clone();
    last.media_box[2] += 200.0;
    base.pages.insert(0, first);
    base.pages.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemovePage(RemovePage { index: 1 }), &base).await;
}

fn runtime_fixture_bytes(relative: &str) -> &'static [u8] {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_slice())
}
