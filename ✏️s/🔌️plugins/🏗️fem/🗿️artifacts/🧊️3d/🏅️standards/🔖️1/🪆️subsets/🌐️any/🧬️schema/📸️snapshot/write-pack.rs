//! 📨️ Native FEM Pack publication shares the canonical original writer and borrows the persistent source.
use super::{Fem3dSnapshot, Fem3dSnapshotValue};
use semio_framework_dsl_record::BorrowedDslRecord;
use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use std::mem::size_of;
use store::{OriginalPackWriting,NativeSnapshotFailure,native_snapshot_turn};
impl Fem3dSnapshot {
    /// 📦️ Streams actual native fields under original scratch, output and cancellation custody.
    pub fn encode_exchange_pack(&self,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<store::io_schema::IoPayload,NativeSnapshotFailure>{
        owner.receive_nested::<OriginalPackWriting,store::io_schema::IoPayload,store::NativeSnapshotFailure>(|slot,original|native_snapshot_turn(|work|{
            let grant=original.remaining_grant();
            let bytes=size_of::<OriginalPackWriting>();
            if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original FEM Pack grant cannot capture its writer header").into())}
            original.native().checkpoint()?;
            original.native().begin_stage(0)?;
            *slot=Some(OriginalPackWriting::new("fem.fem3d",original.native().maximum_bytes())?);
            work.performed();
            original.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            work.take(original.drive_cursor(slot,|writer,native,grant|{
                let result=if writer.ready(){
                    let bytes=size_of::<store::io_schema::IoPayload>();
                    if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return RetainedWorkOutcome{work:0,result:Ok((None,Default::default()))}}
                    writer.take(grant).map(|r|r.map_or((None,Default::default()),|(wire,mut p)|{p.copied_bytes=bytes;(Some(store::io_schema::IoPayload::Binary(wire)),p)}))
                }else{writer.advance(self,Fem3dSnapshotValue::RECORD,native,grant).map(|p|(None,p))};
                RetainedWorkOutcome{work:writer.last_turn_work(),result}
            }))
        }))
    }
}
