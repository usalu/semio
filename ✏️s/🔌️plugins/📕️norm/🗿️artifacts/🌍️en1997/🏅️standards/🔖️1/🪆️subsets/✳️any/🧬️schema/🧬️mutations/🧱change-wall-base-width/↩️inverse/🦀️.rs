use crate::change_wall_base_width::ChangeWallBaseWidth;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangeWallBaseWidth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_base_width::ChangeWallBaseWidth> {
    Ok((|| {
    let w = base.retaining_walls.iter().find(|f| f.id == payload.id).map(|f| f.base_width).unwrap_or(payload.new_base_width);
    vec![Op::from(ChangeWallBaseWidth { id: payload.id.clone(), new_base_width: w })]

    })())
}
