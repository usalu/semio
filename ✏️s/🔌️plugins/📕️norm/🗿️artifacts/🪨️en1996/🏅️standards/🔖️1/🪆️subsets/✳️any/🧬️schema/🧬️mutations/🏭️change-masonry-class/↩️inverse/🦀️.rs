use crate::change_masonry_class::ChangeMasonryClass;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(_payload: &ChangeMasonryClass, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_masonry_class::ChangeMasonryClass> {
    Ok((|| {
    vec![Op::from(ChangeMasonryClass { new_masonry_class: base.masonry_class })]

    })())
}
