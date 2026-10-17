//! 📖️ Explicit PDF 1.7 document, COS, rendering and resource relations.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use store::sqlite_snapshot::{self, SqliteDatabase as Db, SqliteRow as RawRow, SqliteValue as V, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{Cell as C}};
use std::collections::{BTreeMap, BTreeSet};
#[path = "🔢️number/🦀️.rs"]
mod number;
use number::{Row,Projection};

#[path = "🪪️stream-roles/🦀️.rs"]
mod stream_roles;
#[path = "📦️artifact-reference/🦀️.rs"]
mod artifact_reference;

#[path = "🧩️cos/🦀️.rs"]
mod cos;
#[path = "🌈️color/🦀️.rs"]
mod color;
#[path = "🔤️font/🦀️.rs"]
mod font;
#[path = "🖋️content/🦀️.rs"]
mod content;
#[path = "🖼️resource/🦀️.rs"]
mod resource;
#[path = "📇️metadata/🦀️.rs"]
mod metadata;
#[path = "🎯️navigation/🦀️.rs"]
mod navigation;
#[path = "📌️annotation/🦀️.rs"]
mod annotation;
#[path = "📝️form/🦀️.rs"]
mod form;
#[path = "📄️document/🦀️.rs"]
mod document;
pub use document::{PdfConformanceProfile,validate_conformance_profile};
#[path = "🛂️admission/🦀️.rs"]
mod admission;

fn integer<T: TryFrom<i64>>(row:Row<'_>, column: usize) -> Result<T,ValueError> { T::try_from(row.integer(column)?).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue,"PDF integer is outside its declared range")) }
fn boolean(row:Row<'_>, column: usize) -> Result<bool,ValueError> { match row.integer(column)? { 0 => Ok(false), 1 => Ok(true), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF boolean must be zero or one")) } }
fn optional_integer(row:Row<'_>, column: usize) -> Result<Option<i64>,ValueError> { if row.values.get(column) == Some(&V::Null) { Ok(None) } else { row.integer(column).map(Some) } }
fn optional_typed_integer<T: TryFrom<i64>>(row:Row<'_>, column: usize) -> Result<Option<T>,ValueError> { if row.values.get(column) == Some(&V::Null) { Ok(None) } else { integer(row, column).map(Some) } }
fn optional_real(row:Row<'_>, column: usize) -> Result<Option<f64>,ValueError> { if row.is_null(column)? { Ok(None) } else { row.real(column).map(Some) } }
fn optional_boolean(row:Row<'_>, column: usize) -> Result<Option<bool>,ValueError> { if row.values.get(column) == Some(&V::Null) { Ok(None) } else { boolean(row, column).map(Some) } }
fn real_cell(value: Option<f64>) -> C<'static> { value.map_or(C::Null, C::Real) }
fn integer_cell(value: Option<u32>) -> C<'static> { value.map_or(C::Null, |value| C::Integer(i64::from(value))) }
fn boolean_cell(value: Option<bool>) -> C<'static> { value.map_or(C::Null, |value| C::Integer(i64::from(value))) }
fn text_cell(value: &Option<String>) -> C<'_> { value.as_deref().map_or(C::Null, C::Text) }
fn optional_array<const N: usize>(row:Row<'_>, start: usize) -> Result<Option<[f64; N]>,ValueError> { let mut values = [0.0; N]; let mut present = 0; for (index, value) in values.iter_mut().enumerate() { if let Some(number) = optional_real(row, start + index)? { *value = number; present += 1; } } match present { 0 => Ok(None), count if count == N => Ok(Some(values)), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF optional array has partially absent components")) } }
fn array_cells<const N: usize>(value: &Option<[f64; N]>) -> [C<'static>; N] { value.map_or([C::Null; N], |value| value.map(C::Real)) }
fn write_sequence(out: &mut Projection<'_, '_>, table: &str, owner: i64, values: &[f64]) -> Result<(),ValueError> { for (ordinal, value) in values.iter().enumerate() { out.insert(table, &[C::Integer(owner), C::Integer(ordinal as i64), C::Real(*value)])?; } Ok(()) }
fn read_sequence(reader: &mut Reader<'_, '_, '_>, table: &'static str, owner: i64) -> Result<Vec<f64>,ValueError> { let mut values = Vec::new(); for row in reader.children(table, 1, 2, owner)? { let row = reader.take(table, row.rowid, 4)?; values.push(row.real(3)?); } Ok(values) }
fn null_except(row:Row<'_>, columns: std::ops::Range<usize>, present: &[usize]) -> Result<(),ValueError> { for column in columns { if !present.contains(&column) && !row.is_null(column)? { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF variant contains a property belonging to another kind")); } } Ok(()) }

struct Reader<'a, 'c, 'p> {
    db: &'a Db,
    rows: BTreeMap<(&'a str, i64), Row<'a>>,
    groups: BTreeMap<(&'a str, usize, usize, Option<(usize, &'a str)>), BTreeMap<i64, Vec<&'a RawRow>>>,
    used: BTreeSet<(&'a str, i64)>,
    control: &'c mut Control<'p>,
    total: usize,
}

impl<'a, 'c, 'p> Reader<'a, 'c, 'p> {
    fn new(db: &'a Db, control: &'c mut Control<'p>) -> Result<Self,ValueError> {
        control.check_database(db, Phase::ReconstructSnapshot)?;
        let mut reader = Self { db, rows: BTreeMap::new(), groups: BTreeMap::new(), used: BTreeSet::new(), control, total: 0 };
        for table in &db.tables { for row in &table.rows {
            if row.rowid <= 0 || row.integer(0)? != row.rowid || reader.rows.insert((&table.name, row.rowid), sqlite_snapshot::artifact::FloatRow::new(row,number::columns(&table.name))?).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF row identity must be positive and unique within its entity table")); }
            reader.total += 1;
            if reader.total % 256 == 0 { reader.control.checkpoint(Phase::ReconstructSnapshot, 0, reader.total)?; }
        } }
        Ok(reader)
    }
    fn take(&mut self, table: &'a str, key: i64, width: usize) -> Result<Row<'a>,ValueError> {
        let row = self.rows.get(&(table, key)).copied().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("missing {table} entity {key}")))?;
        if row.values.len() != width || !self.used.insert((table, key)) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{table} has an invalid row width, duplicate ownership or containment cycle"))); }
        if self.used.len() % 256 == 0 { self.control.checkpoint(Phase::ReconstructSnapshot, self.used.len(), self.total)?; }
        Ok(row)
    }
    fn children(&mut self, table: &'a str, parent_column: usize, ordinal_column: usize, parent: i64) -> Result<Vec<&'a RawRow>,ValueError> {
        self.children_with_role(table, parent_column, ordinal_column, parent, None)
    }
    fn children_with_role(&mut self, table: &'a str, parent_column: usize, ordinal_column: usize, parent: i64, role: Option<(usize, &'a str)>) -> Result<Vec<&'a RawRow>,ValueError> {
        let key = (table, parent_column, ordinal_column, role);
        if !self.groups.contains_key(&key) {
            let mut groups: BTreeMap<i64, Vec<&RawRow>> = BTreeMap::new();
            for (index, row) in self.db.table(table)?.rows.iter().enumerate() {
                if index % 256 == 0 { self.control.checkpoint(Phase::ReconstructSnapshot, self.used.len(), self.total)?; }
                if let Some((column, role)) = role { if row.text(column)? != role { continue; } }
                row.integer(ordinal_column)?;
                groups.entry(row.integer(parent_column)?).or_default().push(row);
            }
            for rows in groups.values_mut() {
                let mut comparisons = 0; let mut cancelled = None;
                rows.sort_by_key(|row| { comparisons += 1; if comparisons % 1024 == 0 && cancelled.is_none() { cancelled = self.control.checkpoint(Phase::ReconstructSnapshot, self.used.len(), self.total).err(); } row.integer(ordinal_column).unwrap_or(i64::MIN) });
                if let Some(error) = cancelled { return Err(error); }
                for (ordinal, row) in rows.iter().enumerate() { if row.integer(ordinal_column)? != ordinal as i64 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{table} order must be contiguous and unique"))); } }
            }
            self.groups.insert(key, groups);
        }
        Ok(self.groups.get_mut(&key).unwrap().remove(&parent).unwrap_or_default())
    }
    fn copy_text(&mut self,value:&str)->Result<String,ValueError>{sqlite_snapshot::artifact::Reconstruction::new(self.control)?.text(value)}
    fn copy_blob(&mut self,value:&[u8])->Result<Vec<u8>,ValueError>{sqlite_snapshot::artifact::Reconstruction::new(self.control)?.blob(value)}
    fn text(&mut self,row:Row<'_>,column:usize)->Result<String,ValueError>{self.copy_text(row.text(column)?)}
    fn optional_text(&mut self,row:Row<'_>,column:usize)->Result<Option<String>,ValueError>{row.optional_text(column)?.map(|value|self.copy_text(value)).transpose()}
    fn blob(&mut self,row:Row<'_>,column:usize)->Result<Vec<u8>,ValueError>{self.copy_blob(row.blob(column)?)}
    fn has(&self, table: &str, key: i64) -> bool { self.rows.contains_key(&(table, key)) }
    fn finish(self) -> Result<(),ValueError> {
        if self.used.len() != self.total { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF database contains orphaned or mismatched semantic entities")); }
        self.control.checkpoint(Phase::ReconstructSnapshot, self.total, self.total)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
