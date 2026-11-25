use crate::change_floor_member::ChangeFloorMember;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFloorMember, base: &En1995Snapshot) -> Option<crate::change_floor_member::ChangeFloorMember> {
    match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => Some(ChangeFloorMember { floor_id: payload.floor_id.clone(), new_value: item.member_id.clone() }),
        None => None,
    }
}
