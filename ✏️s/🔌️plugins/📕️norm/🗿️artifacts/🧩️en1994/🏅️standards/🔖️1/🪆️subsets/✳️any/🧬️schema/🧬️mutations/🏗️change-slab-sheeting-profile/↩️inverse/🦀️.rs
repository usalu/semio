//! ↩️ Inverse for `change-slab-sheeting-profile`.
use crate::change_slab_sheeting_profile::ChangeSlabSheetingProfile;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabSheetingProfile, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_sheeting_profile::ChangeSlabSheetingProfile> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabSheetingProfile { id: payload.id.clone(), new_profile: slab.sheeting.profile })])
}
