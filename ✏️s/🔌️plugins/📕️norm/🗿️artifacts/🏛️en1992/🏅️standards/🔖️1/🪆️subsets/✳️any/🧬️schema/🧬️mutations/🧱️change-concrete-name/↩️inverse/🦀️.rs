//! ↩️ `change-concrete-name` inverse: restores the previous `name`.

use crate::change_concrete_name::ChangeConcreteName;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeConcreteName, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_concrete_name::ChangeConcreteName> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConcreteName, base: &En1992Snapshot) -> Option<ChangeConcreteName> {
    let r0 = base.concrete_grades.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangeConcreteName { grade_id: payload.grade_id.clone(), new_name: r0.name.clone() })
}
