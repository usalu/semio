//! ↩️ `introduce-subject` — undo is `retire-subject`, unless `base` already had this id (then
//! `create` was a no-op and there's nothing to undo).

use crate::retire_subject;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::introduce_subject::mutation::IntroduceSubject;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &IntroduceSubject, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_subject::mutation::RetireSubject> {
    Ok((|| {
    if base.dictionary.subjects.iter().any(|subject| subject.id == payload.subject.id) {
        return Vec::new();
    }
    vec![Op::from(retire_subject::mutation::RetireSubject { id: payload.subject.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
