use super::InsertSiloShell;
use crate::diff::En1993SiloList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertSiloShell, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.silo_shells.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.silo_shell.clone());
    protocol::MutationOutcome::new(En1993Diff { silo_shells: Some(En1993SiloList { values }), ..Default::default() })
}
