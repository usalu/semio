use crate::change_fastener_position_x::ChangeFastenerPositionX;
use crate::change_fastener_position_x;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerPositionX, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_position_x::ChangeFastenerPositionX> {
    let Some(parent) = base.fasteners.iter().find(|row| row.id == payload.fastener_id) else { return Ok(Vec::new()); };
    let Some(row) = parent.positions.iter().find(|row| row.id == payload.position_id) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeFastenerPositionX { fastener_id: payload.fastener_id.clone(), position_id: payload.position_id.clone(), new_value: row.x_m })])
}
