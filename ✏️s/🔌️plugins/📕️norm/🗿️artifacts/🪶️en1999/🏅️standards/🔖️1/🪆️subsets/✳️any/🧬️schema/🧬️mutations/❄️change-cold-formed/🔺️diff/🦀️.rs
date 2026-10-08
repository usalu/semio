//! 🔺️ `change-cold-formed` diff.

use crate::mutations::change_cold_formed::ChangeColdFormed;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999ColdFormedRows};

pub fn diff(payload: &ChangeColdFormed, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.cold_formed == &payload.cold_formed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.cold_formed.iter().enumerate().find(|(at, row)| payload.cold_formed[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Cold-formed sheet id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::setting(&base.cold_formed, &payload.cold_formed)), ..Default::default() })
}
