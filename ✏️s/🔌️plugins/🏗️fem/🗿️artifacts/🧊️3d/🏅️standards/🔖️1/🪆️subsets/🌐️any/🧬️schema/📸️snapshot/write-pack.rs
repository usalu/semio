//! 📨️ Native FEM Pack publication shares the canonical original writer and keeps the actual persistent source borrowed.
use super::{Fem3dSnapshot, Fem3dSnapshotValue};
use semio_framework_dsl_record::BorrowedDslRecord;
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::mem::size_of;
use store::OriginalPackWriting;
impl Fem3dSnapshot {
    /// 📦️ Actual native fields stream into one final envelope buffer under original scratch, output and cancellation custody.
    pub fn encode_exchange_pack(&self, owner: &mut store::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, ValueError> {
        owner.receive_nested::<OriginalPackWriting, store::io_schema::IoPayload>(|slot, original| {
            let grant = original.remaining_grant();
            let bytes = size_of::<OriginalPackWriting>();
            if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < bytes {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "original FEM Pack grant cannot capture its writer header"));
            }
            original.native().checkpoint()?;
            original.native().begin_stage(0)?;
            *slot = Some(OriginalPackWriting::new("fem.fem3d", original.native().maximum_bytes())?);
            original.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
            original.drive_cursor(slot, |writer, native, grant| {
                let result = if writer.ready() {
                    let bytes = size_of::<store::io_schema::IoPayload>();
                    if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 {
                        return (Ok(None), Default::default());
                    }
                    writer.take(grant).map(|r| {
                        r.map_or((None, Default::default()), |(wire, mut p)| {
                            p.copied_bytes = bytes;
                            (Some(store::io_schema::IoPayload::Binary(wire)), p)
                        })
                    })
                } else {
                    writer.advance(self, Fem3dSnapshotValue::RECORD, native, grant).map(|p| (None, p))
                };
                match result {
                    Ok((out, p)) => (Ok(out), p),
                    Err(error) => {
                        let p = error.retained_progress();
                        (Err(error), p)
                    }
                }
            })
        })
    }
}
#[cfg(test)]
#[path = "write-pack-tests.rs"]
mod tests;
