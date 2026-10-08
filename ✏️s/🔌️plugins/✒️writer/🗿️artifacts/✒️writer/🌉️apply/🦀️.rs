//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::schema::mutations::WriterMutation;
#[cfg(test)]
use crate::schema::mutations::{ChangeLanguage, ChangeUri, EditText, RenameWriter};
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::Mutation;

/// 🧮️ Diff-first apply — the mutation's diff folded through the central applier `protocol::apply_diff`.
pub fn apply_writer_mutation(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) -> protocol::MutationApplyResult<()> {
    *snapshot = protocol::apply_diff(mutation.diff(snapshot).diff(), snapshot)?;
    Ok(())
}

/// 🧮️ Applies `mutation` to `snapshot` and hands back the whole [`protocol::MutationOutcome`], the
/// diagnostics included. [`apply_writer_mutation`] answers `Result<(), _>` and drops the messages,
/// so a caller that has to distinguish an applied edit from an applied-with-`mutation.no-op`-warning
/// one — which is exactly what `edit-text`'s committed vector declares — cannot use it.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_writer_mutation_outcome(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) -> protocol::MutationOutcome<WriterDiff> {
    let outcome = <WriterMutation as Mutation<WriterSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), &*snapshot) {
        *snapshot = next;
    }
    outcome
}
