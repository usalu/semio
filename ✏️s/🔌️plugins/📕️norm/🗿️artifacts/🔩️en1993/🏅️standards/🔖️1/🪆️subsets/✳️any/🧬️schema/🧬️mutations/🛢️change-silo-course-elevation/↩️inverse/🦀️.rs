//! ↩️ `change-silo-course-elevation` inverse: restores the previous `course_elevation`.

use crate::change_silo_course_elevation::ChangeSiloCourseElevation;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloCourseElevation, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_course_elevation::ChangeSiloCourseElevation> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloCourseElevation { id: payload.id.clone(), new_course_elevation: row.course_elevation })],
        None => Vec::new(),
    })
}
