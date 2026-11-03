use crate::remove_weld_group::RemoveWeldGroup;
use crate::insert_weld_group;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveWeldGroup, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_weld_group::InsertWeldGroup> {
    Ok((|| {
    if payload.index >= base.weld_groups.len() { return Vec::new(); }
    vec![Op::from(insert_weld_group::InsertWeldGroup { index: Some(payload.index), weld_group: base.weld_groups[payload.index].clone() })]

    })())
}
