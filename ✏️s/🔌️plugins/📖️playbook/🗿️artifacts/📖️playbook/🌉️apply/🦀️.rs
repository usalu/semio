//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::schema::mutations::PlaybookMutation;
use crate::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// ▶️ Applies `mutation` via its diff. External call site: `derived_construction`'s
/// `ArtifactBuilder::mutate` (`../🦀️.rs`).
pub fn apply_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> protocol::MutationApplyResult<PlaybookSnapshot> {
    protocol::apply_diff(protocol::Mutation::diff(mutation, snapshot).diff(), snapshot)
}

/// 🧮️ Applies `mutation` to `base` and hands back the whole `protocol::MutationOutcome`, the
/// diagnostics included — the shape an external conformance host needs, since a committed
/// `🎯️outcome` vector declares a status AND its diagnostic codes, and the plain apply wrapper
/// beside this one answers `Result<_, _>` and drops the messages.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_playbook_mutation_outcome(snapshot: &mut PlaybookSnapshot, mutation: &PlaybookMutation) -> protocol::MutationOutcome<PlaybookDiff> {
    let outcome = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), &*snapshot) {
        *snapshot = next;
    }
    outcome
}
