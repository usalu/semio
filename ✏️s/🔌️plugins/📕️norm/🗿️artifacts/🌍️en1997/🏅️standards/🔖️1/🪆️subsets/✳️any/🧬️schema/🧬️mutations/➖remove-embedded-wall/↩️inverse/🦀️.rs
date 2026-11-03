use crate::remove_embedded_wall::RemoveEmbeddedWall;
use crate::insert_embedded_wall::InsertEmbeddedWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveEmbeddedWall, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_embedded_wall::InsertEmbeddedWall> {
    Ok((|| {
    match base.embedded_walls.get(payload.index).cloned() {
        Some(embedded_wall) => vec![Op::from(InsertEmbeddedWall { index: payload.index, embedded_wall })],
        None => Vec::new(),
    }

    })())
}
