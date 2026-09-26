use super::RemoveSiloShell;
use crate::diff::En1993SiloList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveSiloShell, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.silo_shells.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("silo-shell index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.silo_shells.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { silo_shells: Some(En1993SiloList { values }), ..Default::default() })
}
