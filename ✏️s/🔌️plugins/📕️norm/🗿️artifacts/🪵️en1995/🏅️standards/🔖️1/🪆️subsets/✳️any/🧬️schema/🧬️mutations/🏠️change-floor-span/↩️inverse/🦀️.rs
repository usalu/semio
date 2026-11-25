use crate::change_floor_span::ChangeFloorSpan;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorSpan, base: &En1995Snapshot) -> Option<crate::change_floor_span::ChangeFloorSpan> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorSpan { floor_id: payload.floor_id.clone(), new_value: item.span_m }),
        None => None,
    }
}
