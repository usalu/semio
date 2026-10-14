//! ↩️ `retire-subject` — undo re-`create`s the subject from BASE state, at its original index;
//! missing id ⇒ `Vec::new()`.

use crate::introduce_subject;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::retire_subject::mutation::RetireSubject;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RetireSubject, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_subject::mutation::IntroduceSubject> {
    Ok((|| {
    let Some(position) = base.dictionary.subjects.iter().position(|subject| subject.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_subject::mutation::IntroduceSubject { subject: base.dictionary.subjects[position].clone(), index: Some(position) })]

    })())
}
//#endregion 🔖️Inverse
