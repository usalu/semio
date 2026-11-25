use crate::remove_fastener_position::RemoveFastenerPosition;
use crate::insert_fastener_position;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveFastenerPosition, base: &En1995Snapshot) -> Option<crate::insert_fastener_position::InsertFastenerPosition> {
    let Some(parent) = base.fasteners.iter().find(|row| row.id == payload.fastener_id) else { return None; };
    let Some(at) = parent.positions.iter().position(|row| row.id == payload.id) else { return None; };
    Some(insert_fastener_position::InsertFastenerPosition { fastener_id: payload.fastener_id.clone(), index: at, position: parent.positions[at].clone() })
}
