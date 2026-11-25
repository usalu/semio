use crate::insert_fastener_position::InsertFastenerPosition;
use crate::remove_fastener_position;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertFastenerPosition, base: &En1995Snapshot) -> Option<crate::remove_fastener_position::RemoveFastenerPosition> {
    Some(remove_fastener_position::RemoveFastenerPosition { fastener_id: payload.fastener_id.clone(), id: payload.position.id.clone() })
}
