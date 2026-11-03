use crate::change_fastener_load_angle::ChangeFastenerLoadAngle;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerLoadAngle, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_load_angle::ChangeFastenerLoadAngle> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerLoadAngle { fastener_id: payload.fastener_id.clone(), new_value: item.load_angle_rad })],
        None => Vec::new(),
    })
}
