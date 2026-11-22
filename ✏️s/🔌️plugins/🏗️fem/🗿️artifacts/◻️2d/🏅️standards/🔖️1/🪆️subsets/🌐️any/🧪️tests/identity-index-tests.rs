use super::*;
use semio_framework_value::RetainedCloneGrant;
fn checked<T>(turn:semio_framework_value::RetainedWorkOutcome<T,store::NativeSnapshotFailure>,fuel:&mut u128)->Result<T,store::NativeSnapshotFailure>{*fuel=fuel.checked_sub(turn.work).expect("finite test work loan");turn.result}
fn failure_kind(error:store::NativeSnapshotFailure)->ValueRefusalKind{let kind=error.kind();drain(error);kind}
use semio_s_fem_node::planar::FemNodeValue;
#[derive(semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    semio_s_fem_receiving::testkit::retire(value)
}
fn measured(count: usize) -> (usize,u128) {
    let source = source(count);
    let mut fuel = 10_000_000;
    let (index, mut work) = original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
        let mut published = None;
        checked(FemIdentityIndex::receive(&source, &mut published, owner), &mut fuel).unwrap();
        (published.unwrap(), owner.progress().copied_items)
    });
    for rank in [0, count / 2, count - 1] {
        work += original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
            let mut found = None;
            checked(index.lookup(&source, FemIdentityKind::Node, &format!("n{rank:05}"), &mut found, owner), &mut fuel).unwrap();
            assert_eq!(found, Some(rank));
            owner.progress().copied_items
        });
    }
    drain(index);
    (work, 10_000_000 - fuel)
}
#[test]
fn sorted_identity_work_is_subquadratic_at_512_1024_and_10000_nodes() {
    let cases = cases();
    assert!(cases.namespace_duplicates_refuse && cases.missing_references_refuse && cases.cancellation_retains_original_custody);
    let small = measured(cases.node_counts[0]);
    let large = measured(cases.node_counts[1]);
    assert!(large.0 < cases.maximum_growth * small.0);
    assert!(large.1 < cases.maximum_growth as u128 * small.1);
    assert!(measured(cases.node_counts[2]).0 < cases.maximum_index_work);
}
#[test]
fn sorting_cancellation_and_duplicate_or_absent_keys_keep_original_custody() {
    let source = source(1024);
    let calls = std::cell::Cell::new(0usize);
    let mut callback = |_| {
        let n = calls.get() + 1;
        calls.set(n);
        n < 1500
    };
    let error = original_owners::with_decoding_granted(grant(), &mut callback, |owner| {
        let mut fuel = 10_000_000;
        let mut published = None;
        let error = checked(FemIdentityIndex::receive(&source, &mut published, owner), &mut fuel).unwrap_err();
        assert!(published.is_none());
        error
    });
    assert_eq!(failure_kind(error),ValueRefusalKind::Canceled);
    assert!(calls.get() >= 1500);
    let index = original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
        let mut fuel = 10_000_000;
        let mut published = None;
        checked(FemIdentityIndex::receive(&source, &mut published, owner), &mut fuel).unwrap();
        published.unwrap()
    });
    original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
        let mut fuel = 10_000_000;
        let mut found = None;
        let error = checked(index.lookup(&source, FemIdentityKind::Node, "absent", &mut found, owner), &mut fuel).unwrap_err();
        assert_eq!(failure_kind(error), ValueRefusalKind::InvalidValue);
        assert!(found.is_none());
    });
    drain(index);
    let mut duplicate=source;duplicate.nodes[1].id=duplicate.nodes[0].id.clone();
    let error = original_owners::with_decoding_granted(grant(), &mut |_| true, |owner| {
        let mut fuel = 10_000_000;
        let mut published = None;
        let error = checked(FemIdentityIndex::receive(&duplicate, &mut published, owner), &mut fuel).unwrap_err();
        assert!(published.is_none());
        error
    });
    assert_eq!(failure_kind(error),ValueRefusalKind::InvalidValue);
}
