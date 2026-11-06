use super::*;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeRetirementRecipient,NativeDecodeWorkRecipient};
/// 🎟️ Finite lower-level work loan; job caller behavior is tested separately.
struct Loan;
impl NativeDecodeWorkRecipient for Loan {fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=10_000_000))}}
fn checked<T>(turn:semio_framework_value::RetainedWorkOutcome<T,store::NativeSnapshotFailure>,fuel:&mut u64)->Result<T,store::NativeSnapshotFailure>{*fuel=fuel.checked_sub(turn.work).expect("finite test work loan");turn.result}
fn failure_kind(error:store::NativeSnapshotFailure)->ValueRefusalKind{let kind=error.kind();drain(error);kind}
use semio_s_fem_node::volumetric::FemNodeValue;
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
fn source(count: usize) -> Fem3dSnapshotValue {
    let mut source = Fem3dSnapshotValue::default();
    source.nodes = (0..count).map(|i| FemNodeValue { id: format!("n{i:05}"), x: i as f64, y: 0., z: 0. }).collect();
    source
}
fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    let mut retirement_fuel=10_000_000u64;
    while !owner.terminal_is_empty() {
        let demand=owner.next_demand(4096).unwrap();
        assert!(retained_clone_grant_admits(grant(),demand));
        let step=owner.step(exact(demand)).unwrap();
        assert!(step.progress().fits(exact(demand)));
        retirement_fuel=retirement_fuel.checked_sub(owner.last_turn_work()).expect("finite retirement work loan");
    }
}
fn measured(count: usize) -> (usize,u64) {
    let source = source(count);
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut callback = |_| true;
    let mut native = NativeDecodeControl::new(256 * 1048576, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let (index,work,actual_work)=native.with_work_recipient(&mut Loan,|native| {
        let mut work_fuel=10_000_000;
        let mut owner=NativeSnapshotDecodeOwner::new(native,grant());
        let mut published=None;let turn=FemIdentityIndex::receive(&source,&mut published,&mut owner);let mut actual_work=turn.work;checked(turn,&mut work_fuel).unwrap();let index=published.take().unwrap();
        for rank in [0,count/2,count-1] {
            let mut found=None;let turn=index.lookup(&source,FemIdentityKind::Node,&format!("n{rank:05}"),&mut found,&mut owner);actual_work+=turn.work;
            checked(turn,&mut work_fuel).unwrap();assert_eq!(found.unwrap(),rank);
        }
        Ok::<_,ValueError>((index,owner.progress().copied_items,actual_work))
    }).unwrap();
    assert!(!native.has_retirement_owner());
    drain(index);
    (work,actual_work)
}
#[test]
fn sorted_identity_work_is_subquadratic_at_512_1024_and_10000_nodes() {
    let cases = cases();
    assert!(cases.namespace_duplicates_refuse && cases.missing_references_refuse && cases.cancellation_retains_original_custody);
    let small = measured(cases.node_counts[0]);
    let large = measured(cases.node_counts[1]);
    assert!(large.0 < cases.maximum_growth * small.0);
    assert!(large.1 < cases.maximum_growth as u64 * small.1);
    assert!(measured(cases.node_counts[2]).0 < cases.maximum_index_work);
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
    let error=native.with_work_recipient(&mut Loan,|native| {
        let mut work_fuel=10_000_000;
        let mut owner=NativeSnapshotDecodeOwner::new(native,grant());
        let mut published=None;let error=checked(FemIdentityIndex::receive(&source,&mut published,&mut owner),&mut work_fuel).err().unwrap();if let Some(value)=published.take(){drain(value)}Ok::<_,ValueError>(error)
    }).unwrap();
    assert_eq!(failure_kind(error),ValueRefusalKind::Canceled);
    enabled.set(true);
    native.with_work_recipient(&mut Loan, |native| native.with_retirement(grant(),|native| {
        let mut retirement_fuel=10_000_000u64;
        while native.has_retirement_owner(){let demand=native.retirement_demands(4096)?;assert!(retained_clone_grant_admits(grant(),demand));let offered=exact(demand);let turn=native.close_retirement_recipient(offered);retirement_fuel=retirement_fuel.checked_sub(turn.work).expect("finite retirement work loan");let step=turn.result?;assert!(step.progress().fits(offered));}
        Ok::<_,ValueError>(())
    })).unwrap();
    let index=native.with_work_recipient(&mut Loan,|native| {
        let mut work_fuel=10_000_000;
        let mut owner=NativeSnapshotDecodeOwner::new(native,grant());
        let mut published=None;checked(FemIdentityIndex::receive(&source,&mut published,&mut owner),&mut work_fuel).unwrap();let index=published.take().unwrap();
        assert_eq!(failure_kind({let mut found=None;checked(index.lookup(&source,FemIdentityKind::Node,"absent",&mut found,&mut owner),&mut work_fuel).unwrap_err()}),ValueRefusalKind::InvalidValue);
        Ok::<_,ValueError>(index)
    }).unwrap();
    drain(index);
    let mut duplicate=source;duplicate.nodes[1].id=duplicate.nodes[0].id.clone();
    let error=native.with_work_recipient(&mut Loan,|native| {
        let mut work_fuel=10_000_000;
        let mut owner=NativeSnapshotDecodeOwner::new(native,grant());
        let mut published=None;let error=checked(FemIdentityIndex::receive(&duplicate,&mut published,&mut owner),&mut work_fuel).err().unwrap();if let Some(value)=published.take(){drain(value)}Ok::<_,ValueError>(error)
    }).unwrap();
    assert_eq!(failure_kind(error),ValueRefusalKind::InvalidValue);
    native.with_work_recipient(&mut Loan, |native| native.with_retirement(grant(),|native| {
        let mut retirement_fuel=10_000_000u64;
        while native.has_retirement_owner(){let demand=native.retirement_demands(4096)?;assert!(retained_clone_grant_admits(grant(),demand));let offered=exact(demand);let turn=native.close_retirement_recipient(offered);retirement_fuel=retirement_fuel.checked_sub(turn.work).expect("finite retirement work loan");let step=turn.result?;assert!(step.progress().fits(offered));}
        Ok::<_,ValueError>(())
    })).unwrap();
}
