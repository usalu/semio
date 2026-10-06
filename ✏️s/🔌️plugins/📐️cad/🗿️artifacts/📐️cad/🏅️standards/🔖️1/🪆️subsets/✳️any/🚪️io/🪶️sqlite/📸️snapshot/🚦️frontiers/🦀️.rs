//! 🚦️ Borrowed SQLite row admission and relational ordering.
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::SqliteRow;
use crate::standards::v1::subsets::any::schema::references::{invalid,frontiers::order};
pub(crate) fn keyed<'a>(rows: &'a [SqliteRow], columns: usize, parent: Option<i64>, control: &mut NativeDecodeControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(rows.len())?;
        let mut output = control.allocate_vec(rows.len())?;
        for row in rows {
            if row.rowid <= 0 || row.values.len() != columns || row.integer(0)? != row.rowid || parent.is_some_and(|parent| row.integer(1).ok() != Some(parent)) { return Err(invalid("CAD requires positive complete aliases and exact owning parents")); }
            output.push(row); control.step()?;
        }
        order(&mut output, |left, right, _| Ok(left.rowid.cmp(&right.rowid)), control)?;
        control.begin_stage(output.len().saturating_sub(1))?;
        for index in 1..output.len() {
            if output[index - 1].rowid == output[index].rowid { return Err(invalid("CAD SQL aliases must be unique")); }
            control.step()?;
        }
        Ok(output)
    })
}
pub(crate) fn by_identity<'a>(rows: &[&'a SqliteRow], identity: i64) -> Option<&'a SqliteRow> {
    rows.binary_search_by_key(&identity, |row| row.rowid).ok().map(|index| rows[index])
}
pub(crate) fn dense<'a>(rows: &[&'a SqliteRow], control: &mut NativeDecodeControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(rows.len())?;
        let mut slots = control.allocate_vec::<Option<&SqliteRow>>(rows.len())?;
        for _ in 0..rows.len() { slots.push(None); control.step()?; }
        control.begin_stage(rows.len())?;
        for row in rows {
            let ordinal = usize::try_from(row.integer(2)?).map_err(|_| invalid("CAD ordinal must be nonnegative and addressable"))?;
            let slot = slots.get_mut(ordinal).ok_or_else(|| invalid("CAD ordinal exceeds its owning collection"))?;
            if slot.replace(*row).is_some() { return Err(invalid("CAD ordinals must be unique and contiguous")); }
            control.step()?;
        }
        let mut output = control.allocate_vec(rows.len())?;
        control.begin_stage(rows.len())?;
        for slot in slots { output.push(slot.ok_or_else(|| invalid("CAD ordered relationship is incomplete"))?); control.step()?; }
        Ok(output)
    })
}
pub(crate) fn references<'a>(rows: &[&'a SqliteRow], groups: &[&SqliteRow], control: &mut NativeDecodeControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(rows.len())?;
        let mut output = control.allocate_vec(rows.len())?;
        for row in rows {
            if by_identity(groups, row.integer(1)?).is_none() { return Err(invalid("CAD reference has no owning map group")); }
            if row.integer(2)? < 0 { return Err(invalid("CAD reference ordinal must be nonnegative")); }
            output.push(*row); control.step()?;
        }
        order(&mut output, |left, right, _| {
            Ok(left.integer(1)?.cmp(&right.integer(1)?).then(left.integer(2)?.cmp(&right.integer(2)?)))
        }, control)?;
        control.begin_stage(output.len())?;
        let mut parent = None;
        let mut ordinal = 0i64;
        for row in &output {
            let next = row.integer(1)?;
            if parent != Some(next) { parent = Some(next); ordinal = 0; }
            if row.integer(2)? != ordinal { return Err(invalid("CAD reference ordinals must be unique and contiguous")); }
            ordinal = ordinal.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "CAD reference ordinal overflow"))?;
            control.step()?;
        }
        Ok(output)
    })
}
