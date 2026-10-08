//! 🏔️ Normalized complete intrinsic map trees and exact independent child handles.
use crate::standards::v1::subsets::any::schema::snapshot::GisTerrainSnapshot;
use crate::schema::{ImportedMap,ImportedProperty};
use semio_framework_value::{DslValue,Number};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(error:impl std::fmt::Display)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())}

use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{FloatColumn,NativeEncodingBound}}};
#[path="📤️projection/🦀️.rs"]mod owned_projection;
#[path="📥️reconstruction/🦀️.rs"]mod owned_reconstruction;
const SCALAR:&[FloatColumn]=&[FloatColumn::Binary64(1)];
impl ArtifactSqliteSnapshot for GisTerrainSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io::IoPayload,ValueError>{
  if Self::SQLITE_SCHEMA.len()>control.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"terrain schema byte limit exceeded"))}
  let maximum_rows=control.limits().max_rows;crate::standards::v1::subsets::any::schema::snapshot::row_admission::snapshot(self,maximum_rows,||control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0))?;
  if let Some(map)=&self.imported_map{map.validate().map_err(invalid)?;}control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
  store::encode_sqlite_snapshot_record_native(encoding,"gis.gisterrain",crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::producer(),|native|crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::encode_record_controlled(self,native),control)
 }
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let maximum_rows=control.limits().max_rows;let maximum_value_bytes=control.limits().max_value_bytes;control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
  store::decode_sqlite_snapshot_record_native(payload,"gis.gisterrain",crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::producer(),|record,native|crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::reconstruct_record_controlled(record,native,maximum_rows,maximum_value_bytes),control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  let mut bound=NativeEncodingBound::new(control)?;bound.add(4096)?;
  if let Some(map)=&self.imported_map{map.validate().map_err(invalid)?;let mut pending=bound.allocate_frontier(0)?;for records in [&map.positions,&map.routes,&map.regions]{for value in records{bound.push_frontier(&mut pending,value)?;}}for property in &map.properties{bound.repeated(property.name.len(),6)?;bound.push_frontier(&mut pending,&property.value)?;}while let Some(value)=pending.pop(){bound.add(512)?;match value{DslValue::String(text)=>bound.repeated(text.len(),6)?,DslValue::Bytes(bytes)=>bound.repeated(bytes.len(),4)?,DslValue::Array(items)=>for value in items{bound.push_frontier(&mut pending,value)?;},DslValue::Object(members)=>for(name,value)in members{bound.repeated(name.len(),6)?;bound.push_frontier(&mut pending,value)?;},_=>()}}}
  if let Some(child)=&self.mesh{for text in [&child.child_id,&child.target.artifact_id,&child.target.dialect.artifact_kind,&child.target.dialect.standard,&child.target.dialect.subset]{bound.repeated(text.len(),6)?;}}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{owned_projection::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{owned_reconstruction::reconstruct(database,control)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.gis.gisterrain"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"GIS terrain does not own this semantic subset")))}let restored=Self::from_sqlite_database(database,control).map_err(store::io_schema::IoError::from_value_error)?;let maps=match(&self.imported_map,&restored.imported_map){(None,None)=>true,(Some(a),Some(b))=>a.same(b),_=>false};if self.exaggeration.to_bits()!=restored.exaggeration.to_bits()||!maps||self.mesh!=restored.mesh{return Err(store::io_schema::IoError::from_value_error(invalid("GIS terrain owned identity differs")))}Ok(store::io_schema::IoOutcome::clean(()))
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

