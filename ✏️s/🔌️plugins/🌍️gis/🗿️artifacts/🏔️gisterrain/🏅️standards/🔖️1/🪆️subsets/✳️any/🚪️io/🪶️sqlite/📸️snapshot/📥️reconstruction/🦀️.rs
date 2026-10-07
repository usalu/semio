//! 🏔️ Complete GIS reconstruction pays actual indexes, output containers and iterative frames.
use super::{GisTerrainSnapshot, ImportedMap, ImportedProperty, SCALAR};
use semio_framework_value::{DslValue, NativeDecodeControl, Number, ValueError, ValueRefusalKind};
use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, artifact::FloatRow};
use std::{cmp::Ordering, ops::Range};

const TABLES: [&str; 17] = ["gis_terrain_document", "gis_terrain_parameters", "gis_terrain_mesh_child", "gis_terrain_imported_map", "gis_terrain_value", "gis_terrain_position", "gis_terrain_route", "gis_terrain_region", "gis_terrain_property", "gis_terrain_array", "gis_terrain_member", "gis_terrain_boolean", "gis_terrain_unsigned", "gis_terrain_signed", "gis_terrain_float", "gis_terrain_text", "gis_terrain_bytes"];
const VALUE: usize = 4;
const PROPERTY: usize = 8;
const ARRAY: usize = 9;
const MEMBER: usize = 10;
fn invalid(message: &'static str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
fn overflow() -> ValueError { ValueError::new(ValueRefusalKind::OwnershipLimit, "terrain relational ownership overflow") }

struct Owner<'control, 'callback> { native: &'control mut NativeDecodeControl<'callback>, bytes: usize, maximum: usize }
impl Owner<'_, '_> {
 fn checkpoint(&mut self) -> Result<(), ValueError> { self.native.step() }
 fn semantic(&mut self, count: usize) -> Result<(), ValueError> { self.bytes = self.bytes.checked_add(count).filter(|bytes| *bytes <= self.maximum).ok_or_else(overflow)?; Ok(()) }
 fn scalar(&mut self) -> Result<(), ValueError> { self.semantic(8) }
 fn text(&mut self, text: &str) -> Result<String, ValueError> { self.semantic(text.len())?; self.native.copy_text(text) }
 fn bytes(&mut self, bytes: &[u8]) -> Result<Vec<u8>, ValueError> { self.semantic(bytes.len())?; self.native.copy_bytes(bytes) }
 fn allocate<T>(&mut self, count: usize) -> Result<Vec<T>, ValueError> { self.native.allocate_vec(count) }
 fn push<T>(&mut self, values: &mut Vec<T>, value: T) -> Result<(), ValueError> {
  if values.len() == values.capacity() { let count = values.capacity().max(1).checked_mul(2).ok_or_else(overflow)?; let mut next = self.allocate(count)?; for value in values.drain(..) { next.push(value); self.checkpoint()?; } *values = next; }
  values.push(value); Ok(())
 }
}

fn sort<T>(values: &mut [T], owner: &mut Owner<'_, '_>, compare: impl Fn(&T, &T) -> Ordering) -> Result<(), ValueError> {
 fn sift<T>(values: &mut [T], mut root: usize, end: usize, owner: &mut Owner<'_, '_>, compare: &impl Fn(&T, &T) -> Ordering) -> Result<(), ValueError> {
  while root < end / 2 { owner.checkpoint()?; let mut child = root * 2 + 1; if child + 1 < end && compare(&values[child], &values[child + 1]).is_lt() { child += 1; } if !compare(&values[root], &values[child]).is_lt() { break; } values.swap(root, child); root = child; } Ok(())
 }
 let total = values.len();
 for root in (0..total / 2).rev() { sift(values, root, total, owner, &compare)?; }
 for end in (1..total).rev() { owner.checkpoint()?; values.swap(0, end); sift(values, 0, end, owner, &compare)?; } Ok(())
}

struct Entry<'a> { table: usize, row: &'a SqliteRow, consumed: bool }
#[derive(Clone, Copy)]
struct Relation { table: usize, parent: i64, ordinal: usize, entry: usize }
struct Rows<'a> { entries: Vec<Entry<'a>>, relations: Vec<Relation>, tables: [Range<usize>; 17] }
impl<'a> Rows<'a> {
 fn new(database: &'a SqliteDatabase, owner: &mut Owner<'_, '_>) -> Result<Self, ValueError> {
  let total = database.tables.iter().try_fold(0usize, |total, table| total.checked_add(table.rows.len()).ok_or_else(overflow))?;
  let mut entries = owner.allocate(total)?;
  for table in &database.tables { let index = TABLES.iter().position(|name| table.name.eq_ignore_ascii_case(name)).ok_or_else(|| invalid("unknown GIS terrain table"))?; for row in &table.rows { owner.checkpoint()?; if row.integer(0)? != row.rowid { return Err(invalid("GIS terrain row identity differs")); } entries.push(Entry { table: index, row, consumed: false }); } }
  sort(&mut entries, owner, |a, b| (a.table, a.row.rowid).cmp(&(b.table, b.row.rowid)))?;
  let mut tables = std::array::from_fn(|_| 0..0);
  for (index, entry) in entries.iter().enumerate() { owner.checkpoint()?; if index > 0 && entries[index - 1].table == entry.table && entries[index - 1].row.rowid == entry.row.rowid { return Err(invalid("GIS terrain duplicate SQL identity")); } if index == 0 || entries[index - 1].table != entry.table { tables[entry.table].start = index; } tables[entry.table].end = index + 1; }
  let count = (5..=10).try_fold(0usize, |total, table| total.checked_add(tables[table].len()).ok_or_else(overflow))?;
  let mut relations = owner.allocate(count)?;
  for (entry, row) in entries.iter().enumerate() { if !(5..=10).contains(&row.table) { continue; } owner.checkpoint()?; let ordinal = usize::try_from(row.row.integer(2)?).map_err(|_| invalid("GIS terrain ordinals must be dense"))?; relations.push(Relation { table: row.table, parent: row.row.integer(1)?, ordinal, entry }); }
  sort(&mut relations, owner, |a, b| (a.table, a.parent, a.ordinal).cmp(&(b.table, b.parent, b.ordinal)))?;
  let mut ordinal = 0usize;
  for (index, relation) in relations.iter().enumerate() { owner.checkpoint()?; if index == 0 || (relations[index - 1].table, relations[index - 1].parent) != (relation.table, relation.parent) { ordinal = 0; } if relation.ordinal != ordinal { return Err(invalid("GIS terrain ordinals must be dense")); } ordinal = ordinal.checked_add(1).ok_or_else(overflow)?; }
  Ok(Self { entries, relations, tables })
 }
 fn take_entry(&mut self, index: usize, owner: &mut Owner<'_, '_>) -> Result<&'a SqliteRow, ValueError> { owner.checkpoint()?; let entry = &mut self.entries[index]; if entry.consumed { return Err(invalid("dangling or multiply owned GIS terrain relation")); } entry.consumed = true; Ok(entry.row) }
 fn take(&mut self, table: usize, id: i64, owner: &mut Owner<'_, '_>) -> Result<&'a SqliteRow, ValueError> {
  let mut range = self.tables[table].clone(); while range.start < range.end { owner.checkpoint()?; let middle = range.start + range.len() / 2; match self.entries[middle].row.rowid.cmp(&id) { Ordering::Less => range.start = middle + 1, Ordering::Greater => range.end = middle, Ordering::Equal => return self.take_entry(middle, owner) } } Err(invalid("dangling or multiply owned GIS terrain relation"))
 }
 fn count(&self, table: usize, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> { let mut count = 0usize; for index in self.tables[table].clone() { owner.checkpoint()?; count += usize::from(!self.entries[index].consumed); } Ok(count) }
 fn one(&mut self, table: usize, owner: &mut Owner<'_, '_>) -> Result<&'a SqliteRow, ValueError> { if self.tables[table].len() != 1 { return Err(invalid("GIS terrain requires one owner")); } self.take_entry(self.tables[table].start, owner) }
 fn relation_range(&self, table: usize, parent: i64, owner: &mut Owner<'_, '_>) -> Result<Range<usize>, ValueError> {
  let mut lower = 0usize; let mut upper = self.relations.len(); while lower < upper { owner.checkpoint()?; let middle = lower + (upper - lower) / 2; let relation = self.relations[middle]; if (relation.table, relation.parent) < (table, parent) { lower = middle + 1; } else { upper = middle; } } let start = lower; upper = self.relations.len(); while lower < upper { owner.checkpoint()?; let middle = lower + (upper - lower) / 2; let relation = self.relations[middle]; if (relation.table, relation.parent) <= (table, parent) { lower = middle + 1; } else { upper = middle; } } Ok(start..lower)
 }
 fn relation(&mut self, index: usize, owner: &mut Owner<'_, '_>) -> Result<&'a SqliteRow, ValueError> { self.take_entry(self.relations[index].entry, owner) }
 fn finish(self, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> { for entry in self.entries { owner.checkpoint()?; if !entry.consumed { return Err(invalid("orphan GIS terrain entities")); } } Ok(()) }
}

enum Container { Array(Vec<DslValue>), Object { members: Vec<(String, DslValue)>, pending_name: Option<String> } }
struct Frame { remaining: Range<usize>, output: Container }
impl Frame {
 fn next(&mut self, rows: &mut Rows<'_>, owner: &mut Owner<'_, '_>) -> Result<Option<i64>, ValueError> { let Some(index) = self.remaining.next() else { return Ok(None); }; let row = rows.relation(index, owner)?; match &mut self.output { Container::Array(_) => Ok(Some(row.integer(3)?)), Container::Object { pending_name, .. } => { *pending_name = Some(owner.text(row.text(3)?)?); Ok(Some(row.integer(4)?)) } } }
 fn attach(&mut self, value: DslValue) -> Result<(), ValueError> { match &mut self.output { Container::Array(values) => values.push(value), Container::Object { members, pending_name } => members.push((pending_name.take().ok_or_else(|| invalid("GIS terrain member attachment differs"))?, value)) } Ok(()) }
 fn finish(self) -> DslValue { match self.output { Container::Array(values) => DslValue::Array(values), Container::Object { members, .. } => DslValue::Object(members) } }
}

fn intrinsic(rows: &mut Rows<'_>, mut id: i64, owner: &mut Owner<'_, '_>) -> Result<DslValue, ValueError> {
 let mut frames = owner.allocate::<Frame>(0)?;
 loop {
  if frames.len() >= 64 { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "terrain intrinsic reconstruction exceeds depth limit")); }
  let root = rows.take(VALUE, id, owner)?;
  let mut value = match root.text(1)? {
   "null" => DslValue::Null,
   "boolean" => { owner.scalar()?; let value = rows.take(11, id, owner)?.integer(1)?; if !(0..=1).contains(&value) { return Err(invalid("intrinsic boolean differs")); } DslValue::Bool(value != 0) },
   "unsigned" => { owner.scalar()?; let decimal = rows.take(12, id, owner)?.text(1)?; if decimal.is_empty() || decimal.len() > 20 || decimal.len() > 1 && decimal.starts_with('0') || !decimal.bytes().all(|byte| byte.is_ascii_digit()) { return Err(invalid("intrinsic unsigned64 canonical decimal")); } DslValue::Number(Number::UInt(decimal.parse::<u64>().map_err(|_| invalid("intrinsic unsigned64 overflow"))?)) },
   "signed" => { owner.scalar()?; DslValue::Number(Number::Int(rows.take(13, id, owner)?.integer(1)?)) },
   "float" => { owner.scalar()?; DslValue::Number(Number::Float(FloatRow::new(rows.take(14, id, owner)?, SCALAR)?.real(1)?)) },
   "text" => { let row = rows.take(15, id, owner)?; DslValue::String(owner.text(row.text(1)?)?) },
   "bytes" => { let row = rows.take(16, id, owner)?; DslValue::Bytes(owner.bytes(row.blob(1)?)?) },
   kind @ ("array" | "object") => { let remaining = rows.relation_range(if kind == "array" { ARRAY } else { MEMBER }, id, owner)?; let output = if kind == "array" { Container::Array(owner.allocate(remaining.len())?) } else { Container::Object { members: owner.allocate(remaining.len())?, pending_name: None } }; let mut frame = Frame { remaining, output }; if let Some(child) = frame.next(rows, owner)? { owner.push(&mut frames, frame)?; id = child; continue; } frame.finish() },
   _ => return Err(invalid("unknown intrinsic kind")),
  };
  loop { let Some(parent) = frames.last_mut() else { return Ok(value); }; parent.attach(value)?; if let Some(child) = parent.next(rows, owner)? { id = child; break; } value = frames.pop().ok_or_else(|| invalid("terrain reconstruction frontier differs"))?.finish(); }
 }
}

fn snapshot(database: &SqliteDatabase, owner: &mut Owner<'_, '_>) -> Result<GisTerrainSnapshot, ValueError> {
 let mut rows = Rows::new(database, owner)?;
 let document = rows.one(0, owner)?; let id = document.rowid;
 let parameters = rows.take(1, id, owner)?; if rows.count(1, owner)? != 0 { return Err(invalid("terrain parameter ownership differs")); } let exaggeration = FloatRow::new(parameters, SCALAR)?.real(1)?; owner.scalar()?;
 let mesh = if rows.count(2, owner)? > 0 { let row = rows.take(2, id, owner)?; Some(store::ArtifactChild::new(owner.text(row.text(1)?)?, semio_framework_artifact_reference::ArtifactRef { artifact_id: owner.text(row.text(2)?)?, dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: owner.text(row.text(3)?)?, standard: owner.text(row.text(4)?)?, subset: owner.text(row.text(5)?)? } })) } else { None };
 let imported_map = if rows.count(3, owner)? > 0 {
  rows.take(3, id, owner)?; let mut map = ImportedMap::default();
  for (table, target) in [(5, &mut map.positions), (6, &mut map.routes), (7, &mut map.regions)] { let relations = rows.relation_range(table, id, owner)?; *target = owner.allocate(relations.len())?; for index in relations { let child = rows.relation(index, owner)?.integer(3)?; let value = intrinsic(&mut rows, child, owner)?; if !matches!(value, DslValue::Object(_)) { return Err(invalid("imported feature requires intrinsic object")); } target.push(value); } }
  let relations = rows.relation_range(PROPERTY, id, owner)?; map.properties = owner.allocate(relations.len())?;
  for index in relations { let row = rows.relation(index, owner)?; let name = row.text(3)?; if matches!(name, "positions" | "routes" | "regions") { return Err(invalid("reserved imported map property")); } map.properties.push(ImportedProperty { name: owner.text(name)?, value: intrinsic(&mut rows, row.integer(4)?, owner)? }); }
  Some(map)
 } else { None };
 rows.finish(owner)?; owner.native.checkpoint()?; Ok(GisTerrainSnapshot { exaggeration, imported_map, mesh })
}

pub(super) fn reconstruct(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<GisTerrainSnapshot, ValueError> {
 store::sqlite_snapshot::validate_sqlite_database_schema_controlled(database, <GisTerrainSnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA, SqliteSnapshotPhase::ReconstructSnapshot, control)?;
 control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
 let maximum = control.reconstruction_remaining_bytes()?; let mut semantic = 0usize;
 let result = control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot, |remaining, progress| {
  let mut callback = |event: semio_framework_value::native_decoding::NativeDecodeProgress| progress(event.completed, event.total);
  let mut native = NativeDecodeControl::new(remaining, &mut callback);
  let result = native.begin_stage(0).and_then(|()| { let mut owner = Owner { native: &mut native, bytes: 0, maximum }; let result = snapshot(database, &mut owner); semantic = owner.bytes; result });
  (result, native.owned_bytes())
 });
 control.admit_reconstruction_bytes(semantic)?; result?
}
