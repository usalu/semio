//! ↩️ `change-bar-layer-aggregate-size` inverse: restores the previous `aggregate_size`.

use crate::change_bar_layer_aggregate_size::ChangeBarLayerAggregateSize;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLayerAggregateSize, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_layer_aggregate_size::ChangeBarLayerAggregateSize> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarLayerAggregateSize, base: &En1992Snapshot) -> Option<ChangeBarLayerAggregateSize> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.longitudinal.iter().find(|row| row.id == payload.layer_id)?;
    Some(ChangeBarLayerAggregateSize { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_aggregate_size: r1.aggregate_size })
}
