use crate::insert_weld_group::InsertWeldGroup;
use crate::remove_weld_group;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertWeldGroup, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_weld_group::RemoveWeldGroup> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.weld_groups.len());
    vec![Op::from(remove_weld_group::RemoveWeldGroup { index: at })]

    })())
}
