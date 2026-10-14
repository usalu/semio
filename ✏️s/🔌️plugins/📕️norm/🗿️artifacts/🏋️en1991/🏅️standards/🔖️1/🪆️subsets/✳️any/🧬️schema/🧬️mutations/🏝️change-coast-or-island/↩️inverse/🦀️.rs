//! Inverse for `change-coast-or-island`.
use crate::change_coast_or_island::ChangeCoastOrIsland;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeCoastOrIsland, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_coast_or_island::ChangeCoastOrIsland> {
    Ok((|| {
    vec![Op::from(ChangeCoastOrIsland { new_coast_or_island: base.coast_or_island })]

    })())
}
