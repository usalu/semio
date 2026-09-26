use super::RemoveSeismic;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &RemoveSeismic, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.seismics.len() {
        return MutationOutcome::fatal("mutation.invariant", format!("seismics index out of range"), Vec::<String>::new());
    }
    let mut next = base.seismics.clone();
    next.remove(payload.index);
    MutationOutcome::new(En1990Diff { seismics: Some(next), ..En1990Diff::default() })
}
