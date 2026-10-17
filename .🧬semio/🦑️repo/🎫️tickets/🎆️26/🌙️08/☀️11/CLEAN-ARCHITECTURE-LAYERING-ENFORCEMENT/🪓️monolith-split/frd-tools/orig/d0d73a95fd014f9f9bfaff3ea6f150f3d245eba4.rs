use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::remove_image::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveImage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-image");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = testkit::snapshots(runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🌫️remove-image/⬅️before.pdf"), runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🌫️remove-image/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveImage(RemoveImage { id: testkit::removed(&before.images, &after.images).id }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = testkit::snapshots(runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🌫️remove-image/⬅️before.pdf"), runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🌫️remove-image/➡️after.pdf"));
    let target = testkit::removed(&base.images, &after.images);
    let mut first = target.clone();
    first.id = "first".to_string();
    let mut last = target.clone();
    last.id = "last".to_string();
    base.images.insert(0, first);
    base.images.push(last);
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveImage(RemoveImage { id: target.id.clone() }), &base).await;
}

fn runtime_fixture_bytes(relative: &str) -> &'static [u8] {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_slice())
}
