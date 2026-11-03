//! ↩️ Inverse for `change-column-wall-thickness-m`.
use crate::change_column_wall_thickness_m::ChangeColumnWallThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnWallThicknessM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_wall_thickness_m::ChangeColumnWallThicknessM> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnWallThicknessM { id: payload.id.clone(), new_wall_thickness_m: column.wall_thickness_m })])
}
