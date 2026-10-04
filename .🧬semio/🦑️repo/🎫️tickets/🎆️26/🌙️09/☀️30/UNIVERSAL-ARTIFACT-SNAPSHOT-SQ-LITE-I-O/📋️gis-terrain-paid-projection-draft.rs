//! 🏔️ Held exact GIS projection with paid borrowed frontier and stack unsigned formatting.
use super::{GisTerrainSnapshot, DslValue, Number, ValueError, invalid, SCALAR};
use store::sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl, artifact::{Projection, Cell, insert_ieee754, insert_key_ieee754}};
enum Parent<'a> { Collection(&'static str, usize), Property(usize, &'a str), Array(i64, usize), Member(i64, usize, &'a str) }
fn decimal(mut value: u64, buffer: &mut [u8; 20]) -> Result<&str, ValueError> { let mut start = buffer.len(); loop { start -= 1; buffer[start] = b'0' + (value % 10) as u8; value /= 10; if value == 0 { break; } } std::str::from_utf8(&buffer[start..]).map_err(invalid) }
pub(super) fn project(snapshot: &GisTerrainSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
 let mut projection = Projection::new(<GisTerrainSnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA, control)?;
 projection.insert("gis_terrain_document", &[])?; insert_ieee754(&mut projection, "gis_terrain_parameters", &[Cell::Real(snapshot.exaggeration)], SCALAR)?;
 if let Some(child) = &snapshot.mesh { projection.insert("gis_terrain_mesh_child", &[Cell::Text(&child.child_id), Cell::Text(&child.target.artifact_id), Cell::Text(&child.target.dialect.artifact_kind), Cell::Text(&child.target.dialect.standard), Cell::Text(&child.target.dialect.subset)])?; }
 if let Some(map) = &snapshot.imported_map {
  map.validate().map_err(invalid)?; projection.insert("gis_terrain_imported_map", &[])?;
  let mut pending = projection.allocate_frontier(0)?;
  for (table, records) in [("gis_terrain_position", &map.positions), ("gis_terrain_route", &map.routes), ("gis_terrain_region", &map.regions)] { for (ordinal, value) in records.iter().enumerate().rev() { projection.push_frontier(&mut pending, (value, Parent::Collection(table, ordinal), 0usize))?; } }
  for (ordinal, property) in map.properties.iter().enumerate().rev() { projection.push_frontier(&mut pending, (&property.value, Parent::Property(ordinal, &property.name), 0usize))?; }
  while let Some((value, parent, depth)) = pending.pop() {
   projection.checkpoint()?; if depth >= 64 { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::DepthLimit, "terrain intrinsic projection exceeds depth limit")); }
   let kind = match value { DslValue::Null => "null", DslValue::Bool(_) => "boolean", DslValue::Number(Number::UInt(_)) => "unsigned", DslValue::Number(Number::Int(_)) => "signed", DslValue::Number(Number::Float(_)) => "float", DslValue::String(_) => "text", DslValue::Bytes(_) => "bytes", DslValue::Array(_) => "array", DslValue::Object(_) => "object" };
   let id = projection.insert("gis_terrain_value", &[Cell::Text(kind)])?;
   match parent {
    Parent::Collection(table, ordinal) => { projection.insert(table, &[Cell::Integer(1), Cell::Integer(i64::try_from(ordinal).map_err(invalid)?), Cell::Integer(id)])?; },
    Parent::Property(ordinal, name) => { projection.insert("gis_terrain_property", &[Cell::Integer(1), Cell::Integer(i64::try_from(ordinal).map_err(invalid)?), Cell::Text(name), Cell::Integer(id)])?; },
    Parent::Array(parent, ordinal) => { projection.insert("gis_terrain_array", &[Cell::Integer(parent), Cell::Integer(i64::try_from(ordinal).map_err(invalid)?), Cell::Integer(id)])?; },
    Parent::Member(parent, ordinal, name) => { projection.insert("gis_terrain_member", &[Cell::Integer(parent), Cell::Integer(i64::try_from(ordinal).map_err(invalid)?), Cell::Text(name), Cell::Integer(id)])?; },
   }
   match value {
    DslValue::Null => (), DslValue::Bool(value) => projection.insert_key("gis_terrain_boolean", id, &[Cell::Integer(i64::from(*value))])?,
    DslValue::Number(Number::UInt(value)) => { let mut buffer = [0u8; 20]; projection.insert_key("gis_terrain_unsigned", id, &[Cell::Text(decimal(*value, &mut buffer)?)])?; },
    DslValue::Number(Number::Int(value)) => projection.insert_key("gis_terrain_signed", id, &[Cell::Integer(*value)])?,
    DslValue::Number(Number::Float(value)) => insert_key_ieee754(&mut projection, "gis_terrain_float", id, &[Cell::Real(*value)], SCALAR)?,
    DslValue::String(value) => projection.insert_key("gis_terrain_text", id, &[Cell::Text(value)])?, DslValue::Bytes(value) => projection.insert_key("gis_terrain_bytes", id, &[Cell::Blob(value)])?,
    DslValue::Array(items) => for (ordinal, value) in items.iter().enumerate().rev() { projection.push_frontier(&mut pending, (value, Parent::Array(id, ordinal), depth + 1))?; },
    DslValue::Object(members) => for (ordinal, (name, value)) in members.iter().enumerate().rev() { projection.push_frontier(&mut pending, (value, Parent::Member(id, ordinal, name), depth + 1))?; },
   }
  }
 }
 projection.finish()
}
