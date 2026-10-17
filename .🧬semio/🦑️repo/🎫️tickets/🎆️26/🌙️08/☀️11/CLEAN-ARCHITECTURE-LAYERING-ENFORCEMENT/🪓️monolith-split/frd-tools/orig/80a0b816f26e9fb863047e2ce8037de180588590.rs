use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::insert_object::InsertObject;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::remove_object::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveObject as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-object");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, _after) = testkit::snapshots(runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🧹️remove-object/⬅️before.pdf"), runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🧹️remove-object/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveObject(RemoveObject { id: before.objects.last().expect("the seed holds objects").id, admitted_stream_roles: None }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (base, _after) = testkit::snapshots(runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🧹️remove-object/⬅️before.pdf"), runtime_fixture_bytes("../../../../../../../🧪️tests/🧫️fixtures/🧹️remove-object/➡️after.pdf"));
    let id = support::next_object_id(&base);
    let base = crate::standards::v1_7::subsets::base::schema::testkit::applied(&base, &PdfMutation::InsertObject(crate::standards::v1_7::subsets::base::schema::mutations::InsertObject { id, value: PdfObject::Int(7), index: Some(1), admitted_stream_roles: None }));
    assert_mutation_inverse_sum_law(&PdfMutation::RemoveObject(RemoveObject { id, admitted_stream_roles: None }), &base).await;
}

fn runtime_fixture_bytes(relative: &str) -> &'static [u8] {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_slice())
}
