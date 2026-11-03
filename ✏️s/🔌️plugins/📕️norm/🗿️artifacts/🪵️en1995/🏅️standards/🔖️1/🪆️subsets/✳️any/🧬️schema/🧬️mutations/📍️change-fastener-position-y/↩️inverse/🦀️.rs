use crate::change_fastener_position_y::ChangeFastenerPositionY;
use crate::change_fastener_position_y;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerPositionY, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_position_y::ChangeFastenerPositionY> {
    let Some(parent) = base.fasteners.iter().find(|row| row.id == payload.fastener_id) else { return Ok(Vec::new()); };
    let Some(row) = parent.positions.iter().find(|row| row.id == payload.position_id) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeFastenerPositionY { fastener_id: payload.fastener_id.clone(), position_id: payload.position_id.clone(), new_value: row.y_m })])
}
