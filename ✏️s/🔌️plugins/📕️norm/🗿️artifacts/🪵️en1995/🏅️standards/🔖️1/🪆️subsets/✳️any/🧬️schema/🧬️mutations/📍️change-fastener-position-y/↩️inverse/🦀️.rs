use crate::change_fastener_position_y::ChangeFastenerPositionY;
use crate::change_fastener_position_y;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerPositionY, base: &En1995Snapshot) -> Option<crate::change_fastener_position_y::ChangeFastenerPositionY> {
    let Some(parent) = base.fasteners.iter().find(|row| row.id == payload.fastener_id) else { return None; };
    let Some(row) = parent.positions.iter().find(|row| row.id == payload.position_id) else { return None; };
    Some(ChangeFastenerPositionY { fastener_id: payload.fastener_id.clone(), position_id: payload.position_id.clone(), new_value: row.y_m })
}
