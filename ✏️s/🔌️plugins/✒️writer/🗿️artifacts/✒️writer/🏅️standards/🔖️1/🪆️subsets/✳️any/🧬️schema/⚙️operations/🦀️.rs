//! ⚙️ Writer mutation application, codec bridge, store laws, and behavior tests.

use crate::schema::mutations::WriterMutation;
#[cfg(test)]
use crate::schema::mutations::{ChangeLanguage, ChangeUri, EditText, RenameWriter};
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::{Mutation, MutationDiff};

//#region ⚙️Operations
/// 🧮️ Diff-first apply — matches every other migrated facet (`operation.diff(base).apply(base)`,
/// per wave 0's confirmation that `vcs::apply_mutation` is already diff-first under the hood).
pub fn apply_writer_mutation(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) -> protocol::MutationApplyResult<()> {
    let next = mutation.diff(snapshot).diff().apply(snapshot)?;

    *snapshot = next;
    Ok(())
}

pub fn inverse_writer_mutation(snapshot: &WriterSnapshot, mutation: &WriterMutation) -> Result<Vec<WriterMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(snapshot)?

    })
}

/// 🧮️ Applies `mutation` to `snapshot` and hands back the whole [`protocol::MutationOutcome`], the
/// diagnostics included. [`apply_writer_mutation`] answers `Result<(), _>` and drops the messages,
/// so a caller that has to distinguish an applied edit from an applied-with-`mutation.no-op`-warning
/// one — which is exactly what `edit-text`'s committed vector declares — cannot use it.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_writer_mutation_outcome(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) -> protocol::MutationOutcome<WriterDiff> {
    let outcome = <WriterMutation as Mutation<WriterSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `🦀️.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_writer_mutation_steps(mutation: &WriterMutation, base: &WriterSnapshot) -> Result<Vec<WriterMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(base)?

    })
}







//#endregion ⚙️Operations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
