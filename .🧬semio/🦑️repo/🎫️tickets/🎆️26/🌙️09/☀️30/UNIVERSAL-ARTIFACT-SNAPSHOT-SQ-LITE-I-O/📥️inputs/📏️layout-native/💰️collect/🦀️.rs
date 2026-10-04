//! 💰️ Held concrete aggregate ownership for the authored Layout domain rows.
use semio_framework_value::{DecodedValue, FromValue, ValueError};
use store::sqlite_snapshot::{transfer, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase};

pub(super) fn collect<T: FromValue>(rows: &[&SqliteRow], control: &mut SqliteSnapshotControl<'_>, mut construct: impl FnMut(&SqliteRow, &mut SqliteSnapshotControl<'_>) -> Result<T, ValueError>) -> Result<Vec<T>, ValueError> {
    let mut output = DecodedValue::new(transfer::reserve(rows.len(), control)?, |values: Vec<T>| {
        for value in values { T::retire_decoded(value); }
    });
    for (index, row) in rows.iter().enumerate() {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, rows.len())?;
        output.get_mut().push(construct(row, control)?);
    }
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, rows.len(), rows.len())?;
    Ok(output.take())
}
