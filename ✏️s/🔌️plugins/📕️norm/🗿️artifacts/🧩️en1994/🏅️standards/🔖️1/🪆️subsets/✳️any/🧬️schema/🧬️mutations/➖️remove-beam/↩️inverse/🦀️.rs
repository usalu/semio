//! ↩️ Inverse for `remove-beam`: the beam back at its position, then its stud rows back at theirs in ascending order.
use crate::remove_beam::RemoveBeam;
use crate::insert_beam::InsertBeam;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_mut_stud_rows::insert_stud::InsertStud;

pub fn inverse<Op>(payload: &RemoveBeam, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<InsertBeam> + From<InsertStud> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let mut restore = vec![Op::from(InsertBeam { index, beam: base.beams[index].clone() })];
    restore.extend(super::diff::owned_studs(payload, base).into_iter().map(|at| Op::from(InsertStud { index: at, item: base.studs[at].clone() })));
    Ok(restore)
}
