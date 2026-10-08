//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::schema::mutations::FormMutation;
use crate::{FormsDiff, FormsSnapshot};
use protocol::Mutation;

/// ⚖️ Whole-document apply — a thin delegation to the derive-generated `Mutation::diff`+`apply`
/// (see file-level doc for why the free function itself stays, not its old hand-rolled match body).
pub fn apply_form_edit_mutation(spec: &FormsSnapshot, mutation: &FormMutation) -> protocol::MutationApplyResult<FormsSnapshot> {
    protocol::apply_diff(mutation.diff(spec).diff(), spec)
}

/// 🧮️ Applies `mutation` to `base` and hands back the whole `protocol::MutationOutcome`, the
/// diagnostics included — the shape an external conformance host needs, since a committed
/// `🎯️outcome` vector declares a status AND its diagnostic codes, and the plain apply wrapper
/// beside this one answers `Result<_, _>` and drops the messages.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_form_mutation_outcome(snapshot: &mut FormsSnapshot, mutation: &FormMutation) -> protocol::MutationOutcome<FormsDiff> {
    let outcome = <FormMutation as Mutation<FormsSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), &*snapshot) {
        *snapshot = next;
    }
    outcome
}
