//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use super::{DeflateSnapshot,DeflateLevelHint};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,ValueError,ValueRefusalKind,artifact::RowWriter}};
#[path="💰️backing/🦀️.rs"] mod backing;
impl ArtifactSqliteSnapshot for DeflateSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{super::sqlite_native::decode(payload,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{super::sqlite_native::encode(self,encoding,control)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{super::sqlite_native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}

/// 🧮️ Checks the actual authored relational metadata without allocating its SQL recipe.
pub(super)fn admit_layout(limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 const WIDTHS:[(&str,usize);2]=[("deflate_document",6),("deflate_payload_byte",4)];
 let mut schema=0usize;let mut count=0usize;for statement in DeflateSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).filter(|statement|!statement.is_empty()){let(name,width)=WIDTHS.get(count).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Deflate authored table extent changed"))?;if *width>limits.max_columns{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Deflate columns exceed caller limit"))}schema=schema.checked_add(statement.len()).and_then(|value|value.checked_add(name.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate schema extent overflow"))?;count+=1;}
 if count!=WIDTHS.len(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Deflate authored table extent changed"))}if schema>limits.max_schema_bytes||DeflateSnapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes||count>limits.max_tables||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Deflate relational metadata exceeds caller limit"))}Ok(())
}
/// 🫳️ Visits all actual document and payload cells through the original semantic control.
pub(super)fn admit_values(snapshot:&DeflateSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{admit_layout(control.limits())?;let mut output=RowWriter::borrowed(control,phase)?;backing::visit_rows(snapshot,&mut output)}
/// 🫧️ Bounds measured RFC1950 expansion before the final payload allocation.
pub(super)fn native_payload_capacity(limits:SqliteDatabaseLimits)->Result<usize,ValueError>{admit_layout(limits)?;let header=32usize.checked_add(crate::STDIO_DEFLATE_DOCUMENT_SCHEMA.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate document cell extent overflow"))?;let payload=limits.max_value_bytes.checked_sub(header).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate document values exceed caller limit"))?;Ok(payload/32)}
