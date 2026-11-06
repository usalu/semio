//! 🧪️ Actual persistent FEM Pack publication uses the shared original codec and final wire owner.
use super::*;
/// 🪙️ Finite lower-level encoding work loan.
struct EncodeLoan { limit:u64, posted:u64 }
impl semio_framework_value::native_encoding::NativeEncodeWorkRecipient for EncodeLoan {
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.limit))}
 fn committed(&mut self,actual:u64)->Result<(),semio_framework_value::ValueError>{self.posted=self.posted.checked_add(actual).filter(|total|*total<=self.limit).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"finite encoder test loan exhausted"))?;Ok(())}
}

#[test]
fn actual_fem_native_pack_publication_matches_the_canonical_reference_document() {
    let source = Fem2dSnapshot::default();
    let expected = pack::snapshot::ArtifactPack::encode_pack(&source);
    let mut callback = |_| true;
    let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut native = semio_framework_value::NativeEncodeControl::new(128 * 1048576, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 2_000_000, maximum_copy_bytes: 128 * 1048576, maximum_capacity_bytes: 128 * 1048576, maximum_release_bytes: 128 * 1048576, maximum_depth: 128 };
    let payload=native.with_work_recipient(&mut EncodeLoan { limit:2_000_000,posted:0 }, |native| {
        let mut original=store::NativeSnapshotEncodeOwner::new(native,grant);
        let mut published=None;let turn=source.encode_exchange_pack(&mut published,&mut original);
        assert!(turn.work<=2_000_000);
        turn.result.unwrap();let payload=published.take().unwrap();
        assert!(original.progress().copied_items>0);
        Ok::<_,semio_framework_value::ValueError>(payload)
    }).unwrap();
    let store::io_schema::IoPayload::Binary(wire)=&payload else {panic!("binary native Pack")};
    assert_eq!(wire,&expected);
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(payload).ok().unwrap();
    let mut retirement_fuel=2_000_000u64;
    while !close.terminal_is_empty() {
        let demand=close.next_demand(4096).unwrap();
        assert!(retained_clone_grant_admits(grant,demand));
        let offered=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let step=close.step(offered).unwrap();
        assert!(step.progress().fits(offered));
        retirement_fuel=retirement_fuel.checked_sub(close.last_turn_work()).expect("finite retirement work loan");
    }
}
