//! 🧮️ Borrowed terrain and native Record trees count every normalized SQL owner row.
use super::GisTerrainSnapshot;
use crate::schema::ImportedMap;
use semio_framework_dsl_record::{FieldValue, RecordValue};
use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};
fn invalid(message: &'static str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
struct Rows { count: usize, maximum: usize }
impl Rows {
 fn add(&mut self, count: usize) -> Result<(), ValueError> { self.count = self.count.checked_add(count).filter(|count| *count <= self.maximum).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "terrain native complete owner row limit exceeded"))?; Ok(()) }
 fn tree(&mut self, value: &DslValue, depth: usize, checkpoint: &mut impl FnMut() -> Result<(), ValueError>) -> Result<(), ValueError> {
  checkpoint()?; if depth >= 64 { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "terrain native row census exceeds intrinsic depth limit")); }
  let branch = usize::from(matches!(value, DslValue::Bool(_) | DslValue::Number(_) | DslValue::String(_) | DslValue::Bytes(_))); self.add(2 + branch)?;
  match value { DslValue::Array(items) => for value in items { self.tree(value, depth + 1, checkpoint)?; }, DslValue::Object(members) => for (_, value) in members { self.tree(value, depth + 1, checkpoint)?; }, _ => () } Ok(())
 }
 fn feature(&mut self, value: &DslValue, checkpoint: &mut impl FnMut() -> Result<(), ValueError>) -> Result<(), ValueError> { if !matches!(value, DslValue::Object(_)) { return Err(invalid("terrain map feature requires complete object")); } self.tree(value, 0, checkpoint) }
 fn property(&mut self, name: &str, value: &DslValue, checkpoint: &mut impl FnMut() -> Result<(), ValueError>) -> Result<(), ValueError> { if matches!(name, "positions" | "routes" | "regions") { return Err(invalid("terrain map property uses reserved collection name")); } self.tree(value, 0, checkpoint) }
 fn map(&mut self, map: &ImportedMap, checkpoint: &mut impl FnMut() -> Result<(), ValueError>) -> Result<(), ValueError> { checkpoint()?; self.add(1)?; for records in [&map.positions, &map.routes, &map.regions] { for value in records { self.feature(value, checkpoint)?; } } for property in &map.properties { self.property(&property.name, &property.value, checkpoint)?; } Ok(()) }
 fn native_map(&mut self, map: &RecordValue, checkpoint: &mut impl FnMut() -> Result<(), ValueError>) -> Result<(), ValueError> {
  checkpoint()?; self.add(1)?; if map.fields.len() != 4 { return Err(invalid("terrain native map field census differs")); }
  for field in 0..3 { let Some(FieldValue::List(values)) = map.fields.get(&field) else { return Err(invalid("terrain native collection requires list")); }; for value in values { let FieldValue::Value(value) = value else { return Err(invalid("terrain native collection requires intrinsic value")); }; self.feature(value, checkpoint)?; } }
  let Some(FieldValue::List(properties)) = map.fields.get(&3) else { return Err(invalid("terrain native properties require list")); };
  for property in properties { let FieldValue::Record(property) = property else { return Err(invalid("terrain native property requires record")); }; if property.fields.len() != 2 { return Err(invalid("terrain native property field census differs")); } let Some(FieldValue::Text(name)) = property.fields.get(&0) else { return Err(invalid("terrain native property name requires text")); }; let Some(FieldValue::Value(value)) = property.fields.get(&1) else { return Err(invalid("terrain native property requires intrinsic value")); }; self.property(name, value, checkpoint)?; } Ok(())
 }
}
pub(super) fn snapshot(value: &GisTerrainSnapshot, maximum: usize, mut checkpoint: impl FnMut() -> Result<(), ValueError>) -> Result<usize, ValueError> { let mut rows = Rows { count: 0, maximum }; checkpoint()?; rows.add(2 + usize::from(value.mesh.is_some()))?; if let Some(map) = &value.imported_map { rows.map(map, &mut checkpoint)?; } Ok(rows.count) }
pub(super) fn native(value: &RecordValue, maximum: usize, mut checkpoint: impl FnMut() -> Result<(), ValueError>) -> Result<usize, ValueError> { let mut rows = Rows { count: 0, maximum }; checkpoint()?; let mesh = match value.fields.get(&2) { None | Some(FieldValue::Absent) => false, Some(FieldValue::Record(_)) => true, _ => return Err(invalid("terrain native optional mesh record differs")) }; rows.add(2 + usize::from(mesh))?; match value.fields.get(&1) { None | Some(FieldValue::Absent) => (), Some(FieldValue::Record(map)) => rows.native_map(map, &mut checkpoint)?, _ => return Err(invalid("terrain native optional map record differs")) } Ok(rows.count) }
