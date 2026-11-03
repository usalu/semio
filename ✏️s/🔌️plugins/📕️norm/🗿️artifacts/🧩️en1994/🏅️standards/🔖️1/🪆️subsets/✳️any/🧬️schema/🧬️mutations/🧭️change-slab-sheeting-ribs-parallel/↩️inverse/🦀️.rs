//! ↩️ Inverse for `change-slab-sheeting-ribs-parallel`.
use crate::change_slab_sheeting_ribs_parallel::ChangeSlabSheetingRibsParallel;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabSheetingRibsParallel, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_sheeting_ribs_parallel::ChangeSlabSheetingRibsParallel> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabSheetingRibsParallel { id: payload.id.clone(), new_ribs_parallel_to_beam: slab.sheeting.ribs_parallel_to_beam })])
}
