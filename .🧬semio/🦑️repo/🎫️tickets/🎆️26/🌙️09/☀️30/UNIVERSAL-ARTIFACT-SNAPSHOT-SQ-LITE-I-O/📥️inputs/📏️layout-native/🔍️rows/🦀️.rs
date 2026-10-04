//! 🔍️ Held direct Layout row ownership and nullable parent frontier proposal.
use store::sqlite_snapshot::{transfer, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue};
use semio_framework_value::{ValueError, ValueRefusalKind};

fn invalid(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
fn push<T>(values: &mut Vec<T>, value: T, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    if values.len() == values.capacity() {
        let target = values.capacity().max(1).checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "Layout index capacity overflow"))?;
        let bytes = target.checked_mul(size_of::<T>()).filter(|bytes| *bytes <= isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "Layout index extent overflow"))?;
        control.admit_allocation_bytes(bytes)?;
        values.try_reserve_exact(target - values.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "Layout index backing allocation failed"))?;
    }
    values.push(value);
    Ok(())
}

struct ParentIndex { column: usize, ordinal: usize, positions: Vec<usize> }
struct Table<'a> { name: &'static str, width: usize, rows: Vec<&'a SqliteRow>, used: Vec<bool>, parents: Vec<ParentIndex> }
pub(super) struct Rows<'a> { tables: Vec<Table<'a>> }

impl<'a> Rows<'a> {
    pub(super) fn new(database: &'a SqliteDatabase, declared: &[(&'static str, usize)], control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        let mut tables = transfer::reserve(declared.len(), control)?;
        for &(name, width) in declared {
            let source = &database.table(name)?.rows;
            let mut rows = transfer::reserve(source.len(), control)?;
            let mut used = transfer::reserve(source.len(), control)?;
            for (index, row) in source.iter().enumerate() {
                if row.rowid <= 0 || row.values.len() != width || row.integer(0)? != row.rowid { return Err(invalid("Layout row identity or fields")); }
                rows.push(row);
                used.push(false);
                if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, source.len())?; }
            }
            transfer::heap_sort(&mut rows, SqliteSnapshotPhase::ReconstructSnapshot, control, |left, right, _| Ok(left.rowid.cmp(&right.rowid)))?;
            for index in 1..rows.len() {
                if rows[index - 1].rowid == rows[index].rowid { return Err(invalid("Layout row aliases must be unique")); }
                if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, rows.len())?; }
            }
            tables.push(Table { name, width, rows, used, parents: Vec::new() });
        }
        Ok(Self { tables })
    }

    fn table(&self, name: &str) -> Result<usize, ValueError> {
        self.tables.iter().position(|table| table.name == name).ok_or_else(|| invalid("unknown Layout table"))
    }

    pub(super) fn contains(&self, name: &str, identity: i64) -> Result<bool, ValueError> {
        let table = &self.tables[self.table(name)?];
        Ok(table.rows.binary_search_by_key(&identity, |row| row.rowid).is_ok_and(|position| !table.used[position]))
    }

    pub(super) fn take(&mut self, name: &str, identity: i64) -> Result<&'a SqliteRow, ValueError> {
        let index = self.table(name)?;
        let table = &mut self.tables[index];
        let position = table.rows.binary_search_by_key(&identity, |row| row.rowid).map_err(|_| invalid("dangling Layout relation"))?;
        if std::mem::replace(&mut table.used[position], true) { return Err(invalid("multiply owned Layout relation")); }
        Ok(table.rows[position])
    }

    pub(super) fn list(&mut self, name: &str, parent: i64, column: usize, ordinal: usize, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
        let index = self.table(name)?;
        let table = &mut self.tables[index];
        if column == 0 || column >= table.width || ordinal == 0 || ordinal >= table.width || column == ordinal { return Err(invalid("Layout parent and ordinal columns must be distinct fields")); }
        let selected = match table.parents.iter().position(|entry| entry.column == column && entry.ordinal == ordinal) {
            Some(index) => index,
            None => {
                let mut count = 0usize;
                for (index, row) in table.rows.iter().enumerate() {
                    if row.values[column] != SqliteValue::Null { row.integer(column)?; count = count.checked_add(1).ok_or_else(|| invalid("Layout parent count overflow"))?; }
                    if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, table.rows.len())?; }
                }
                let mut positions = transfer::reserve(count, control)?;
                for (index, row) in table.rows.iter().enumerate() {
                    if row.values[column] != SqliteValue::Null {
                        if row.integer(ordinal)? < 0 { return Err(invalid("Layout ordinal must be nonnegative")); }
                        positions.push(index);
                    }
                    if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, table.rows.len())?; }
                }
                transfer::heap_sort(&mut positions, SqliteSnapshotPhase::ReconstructSnapshot, control, |left, right, _| {
                    let left = table.rows[*left]; let right = table.rows[*right];
                    Ok(left.integer(column)?.cmp(&right.integer(column)?).then(left.integer(ordinal)?.cmp(&right.integer(ordinal)?)))
                })?;
                push(&mut table.parents, ParentIndex { column, ordinal, positions }, control)?;
                table.parents.len() - 1
            }
        };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 0)?;
        let positions = &table.parents[selected].positions;
        let start = positions.partition_point(|position| table.rows[*position].integer(column).expect("validated parent") < parent);
        let end = positions.partition_point(|position| table.rows[*position].integer(column).expect("validated parent") <= parent);
        let mut output = transfer::reserve(end - start, control)?;
        for (ordinal_value, position) in positions[start..end].iter().enumerate() {
            let row = table.rows[*position];
            if row.integer(ordinal)? != i64::try_from(ordinal_value).map_err(|_| invalid("Layout ordinal exceeds SQL width"))? { return Err(invalid("Layout relationship order must be contiguous and unique")); }
            if std::mem::replace(&mut table.used[*position], true) { return Err(invalid("multiply owned Layout relation")); }
            output.push(row);
            if ordinal_value % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, ordinal_value, end - start)?; }
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, end - start, end - start)?;
        Ok(output)
    }

    pub(super) fn finish(self, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        for table in self.tables {
            for (index, used) in table.used.iter().enumerate() {
                if !used { return Err(invalid("orphan Layout rows")); }
                if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, table.used.len())?; }
            }
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)
    }
}
