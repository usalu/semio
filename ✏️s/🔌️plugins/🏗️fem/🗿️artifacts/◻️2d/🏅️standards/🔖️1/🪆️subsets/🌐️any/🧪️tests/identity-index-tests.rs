use super::*;
use semio_framework_value::retirement::RetireOwned;
use super::*;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_s_fem_node::planar::FemNodeValue;
#[derive(semio_framework_value_derive::FromValue, RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct Cases {
    node_counts: Vec<usize>,
    maximum_growth: usize,
    maximum_index_work: usize,
    namespace_duplicates_refuse: bool,
    missing_references_refuse: bool,
    cancellation_retains_original_custody: bool,
}
fn cases() -> Cases {
    semio_framework_pack_json::from_json_str(include_str!("identity-index.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1048576, maximum_capacity_bytes: 256 * 1048576, maximum_release_bytes: 256 * 1048576, maximum_depth: 128 }
}
fn source(count: usize) -> Fem2dSnapshotValue {
    let mut source = Fem2dSnapshotValue::default();
    source.nodes = (0..count).map(|i| FemNodeValue { id: format!("n{i:05}"), x: i as f64, y: 0. }).collect();
    source
}
fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    while !owner.terminal_is_empty() {
        owner.step(grant()).unwrap();
    }
}
fn measured(count: usize) -> usize {
    let source = source(count);
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut callback = |_| true;
    let mut native = NativeDecodeControl::new(256 * 1048576, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let index = FemIdentityIndex::receive(&source, &mut owner).unwrap();
    for rank in [0, count / 2, count - 1] {
        assert_eq!(index.lookup(&source, FemIdentityKind::Node, &format!("n{rank:05}"), &mut owner).unwrap(), rank);
    }
    let work = owner.progress().copied_items;
    drop(owner);
    assert!(!native.has_retirement_owner());
    drain(index);
    work
}
#[test]
fn sorted_identity_work_is_subquadratic_at_512_1024_and_10000_nodes() {
    let cases = cases();
    assert!(cases.namespace_duplicates_refuse && cases.missing_references_refuse && cases.cancellation_retains_original_custody);
    let small = measured(cases.node_counts[0]);
    let large = measured(cases.node_counts[1]);
    assert!(large < cases.maximum_growth * small);
    assert!(measured(cases.node_counts[2]) < cases.maximum_index_work);
}
#[test]
fn sorting_cancellation_and_duplicate_or_absent_keys_keep_original_custody() {
    let source = source(1024);
    let calls = std::cell::Cell::new(0usize);
    let enabled = std::cell::Cell::new(false);
    let mut callback = |_| {
        let n = calls.get() + 1;
        calls.set(n);
        enabled.get() || n < 1500
    };
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(256 * 1048576, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let error = FemIdentityIndex::receive(&source, &mut NativeSnapshotDecodeOwner::new(&mut native, grant())).err().unwrap();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    enabled.set(true);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant()).unwrap();
    }
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let index = FemIdentityIndex::receive(&source, &mut owner).unwrap();
    assert_eq!(index.lookup(&source, FemIdentityKind::Node, "absent", &mut owner).unwrap_err().kind, ValueRefusalKind::InvalidValue);
    drop(owner);
    drain(index);
    let mut duplicate = source;
    duplicate.nodes[1].id = duplicate.nodes[0].id.clone();
    let error = FemIdentityIndex::receive(&duplicate, &mut NativeSnapshotDecodeOwner::new(&mut native, grant())).err().unwrap();
    assert_eq!(error.kind, ValueRefusalKind::InvalidValue);
    assert!(!native.has_retirement_owner());
}
