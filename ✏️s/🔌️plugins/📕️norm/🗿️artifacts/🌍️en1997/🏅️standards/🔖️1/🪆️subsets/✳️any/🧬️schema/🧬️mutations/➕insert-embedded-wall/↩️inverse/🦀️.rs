use crate::insert_embedded_wall::InsertEmbeddedWall;
use crate::remove_embedded_wall::RemoveEmbeddedWall;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertEmbeddedWall, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_embedded_wall::RemoveEmbeddedWall> {
    Ok((|| {
    let at = payload.index.min(base.embedded_walls.len());
    vec![Op::from(RemoveEmbeddedWall { index: at })]

    })())
}
