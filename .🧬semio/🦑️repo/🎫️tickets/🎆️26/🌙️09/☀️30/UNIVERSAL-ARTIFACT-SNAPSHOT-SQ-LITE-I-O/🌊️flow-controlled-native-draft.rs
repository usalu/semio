use crate::FlowSnapshot;
use store::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotControl};

pub(crate) fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<FlowSnapshot, String> {
    control.check_rows(2)?;
    store::decode_sqlite_snapshot_record_native(payload, <FlowSnapshot as store::ArtifactDsl>::envelope_id(), FlowSnapshot::__dsl_spec_producer(), FlowSnapshot::__dsl_from_record_controlled, control)
}

pub(crate) fn encode_sqlite_snapshot_native(snapshot: &FlowSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, String> {
    control.check_rows(2)?;
    store::encode_sqlite_snapshot_record_native(encoding, <FlowSnapshot as store::ArtifactDsl>::envelope_id(), FlowSnapshot::__dsl_spec_producer(), |native| snapshot.__dsl_to_record_controlled(native), control)
}
