use crate::change_bar_layer_count::ChangeBarLayerCount;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLayerCount, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_layer_count::ChangeBarLayerCount> {
    Ok((|| {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(layer) = m.longitudinal.iter().find(|l| l.id == payload.layer_id) else { return vec![]; };
    vec![Op::from(ChangeBarLayerCount { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_count: layer.count })]

    })())
}
