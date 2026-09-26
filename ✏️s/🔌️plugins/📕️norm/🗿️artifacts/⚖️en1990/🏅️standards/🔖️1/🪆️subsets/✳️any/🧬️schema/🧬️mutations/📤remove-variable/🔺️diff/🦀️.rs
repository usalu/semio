use super::RemoveVariable;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &RemoveVariable, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.variables.len() {
        return MutationOutcome::fatal("mutation.invariant", format!("variables index out of range"), Vec::<String>::new());
    }
    let mut next = base.variables.clone();
    next.remove(payload.index);
    MutationOutcome::new(En1990Diff { variables: Some(next), ..En1990Diff::default() })
}
