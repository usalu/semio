//! ↩️ `insert-placement` inverse.

use crate::insert_placement::InsertPlacement;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_placement;

pub fn inverse<Op>(payload: &InsertPlacement, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_placement::RemovePlacement> {
    Ok((|| {
    vec![Op::from(remove_placement::RemovePlacement { id: payload.placement.id.clone() })]

    })())
}
