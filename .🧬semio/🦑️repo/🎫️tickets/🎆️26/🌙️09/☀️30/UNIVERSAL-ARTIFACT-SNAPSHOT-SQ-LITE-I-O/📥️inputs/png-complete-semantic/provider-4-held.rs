//! 🚦️ Controlled logical PNG Record construction keeps raw publication validation in the file engine.
use super::PngSnapshot;
use crate::store;
use semio_framework_value::FromValue;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError};
pub(super) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<PngSnapshot,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,"stdio.png",PngSnapshot::__dsl_spec_producer(),|record,native|{super::sqlite::admit_record(record,limits,native)?;PngSnapshot::__dsl_from_record_controlled(record,native)},control)}
pub(super) fn encode(snapshot:&PngSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{super::sqlite::admit(snapshot,SqliteSnapshotPhase::EncodeNative,control)?;store::encode_sqlite_snapshot_record_native(encoding,"stdio.png",PngSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control)}
pub(super) fn preflight(snapshot:&PngSnapshot,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{super::sqlite::preflight(snapshot,control)}
