//! 📨️ Native FEM Pack publication retains the writer, wire and caller output independently.
use super::{Fem2dSnapshot,Fem2dSnapshotValue};
use semio_framework_dsl_record::{BorrowedDslRecord,receiving::NativeSnapshotEffectProgress};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
const WRITER_ACTION_WORK: u64 = 1 << 16;
use std::mem::size_of;
use store::{OriginalPackWriting,NativeSnapshotFailure,native_snapshot_turn};
#[derive(semio_framework_value::RetireOwned)]
struct PackFrame{writer:Option<OriginalPackWriting>,wire:Option<Vec<u8>>,progress:NativeSnapshotEffectProgress,unwind:Option<NativeSnapshotFailure>}
impl Fem2dSnapshot {
 /// 📦️ Streams native fields and retains each owner through final payload publication.
 pub fn encode_exchange_pack(&self,output:&mut Option<store::io_schema::IoPayload>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  owner.receive_nested::<PackFrame,store::io_schema::IoPayload,NativeSnapshotFailure>(output,&mut |slot,output,birth,birth_unwind,original|native_snapshot_turn(|work|{
   let bytes=size_of::<PackFrame>();let grant=original.remaining_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original FEM Pack grant cannot capture its writer frame").into())}
   original.native().begin_stage(0)?;let maximum=original.native().maximum_bytes();let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
   let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{let writer=OriginalPackWriting::new("fem.fem2d",maximum)?;*slot=Some(PackFrame{writer:Some(writer),wire:None,progress:Default::default(),unwind:None});*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});work.take(turn)?;birth.reset_completed()?;
   let frame=slot.as_mut().unwrap();work.take(original.drive_cursor(&mut frame.writer,&mut frame.wire,&mut |writer,wire,_,_,native,wallet|{
    let body=wallet.remaining_grant();
    let demand=if writer.ready(){RetirementDemand{depth:1,..Default::default()}}else{match writer.demand(body.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error.into())}}};
    if !retained_clone_grant_admits(body,demand){return RetainedWorkOutcome{work:0,result:Ok(())}}
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};
    match native.with_effect(grant,WRITER_ACTION_WORK,|native,effect|{
     let result=if writer.ready(){writer.take(effect).map(|result|result.map_or(Default::default(),|(value,progress)|{*wire=Some(value);progress}))}else{writer.advance(self,Fem2dSnapshotValue::RECORD,native,effect)};
     (result,effect.performed_work())
    }){
     Ok(Some((result,work)))=>RetainedWorkOutcome{work,result:result.and_then(|progress|wallet.record_progress(progress)).map_err(NativeSnapshotFailure::from)},
     Ok(None)=>RetainedWorkOutcome{work:0,result:Ok(())},
     Err(error)=>RetainedWorkOutcome{work:0,result:Err(error.into())},
    }
   }))?;
   if frame.wire.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"FEM Pack wire was not retained").into())}
   let bytes=size_of::<store::io_schema::IoPayload>();let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};
   let turn=original.effect(demand,&mut frame.progress,&mut frame.unwind,&mut |performed,physical|{*output=Some(store::io_schema::IoPayload::Binary(frame.wire.take().unwrap()));*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});work.take(turn)?;frame.progress.reset_completed()?;Ok(())
  }))
 }
}
