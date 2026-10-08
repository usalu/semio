//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

/// ▶️ Applies `mutation` to `snapshot` through its own diff — the artifact's single apply entry
/// point (mirrors dag's `apply_dag_mutation`/puzzle5d's `apply_puzzle5d_mutation`). A rejecting
/// diff carries an empty `VcsDiff`, so the snapshot is left untouched and `Ok(())` is still
/// returned; read [`protocol::MutationOutcome::messages`] to distinguish the two.
pub fn apply_vcs_mutation(snapshot: &mut VcsSnapshot, mutation: &VcsDemoMutation) -> protocol::MutationApplyResult<()> {
    let next = protocol::apply_diff(<VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(mutation, snapshot).diff(), snapshot)?;
    *snapshot = next;
    Ok(())
}

/// ▶️ [`apply_vcs_mutation`]'s reporting, non-async twin: applies `mutation` in place and returns
/// the diagnostic CODES it raised, in order. [`apply_vcs_mutation`] discards them and is `async`,
/// so neither the outcome-policy claim a committed `🎯️outcome/🔣️.json` makes nor a
/// synchronous test adapter can be served by it.
pub fn apply_vcs_mutation_reporting(snapshot: &mut VcsSnapshot, mutation: &VcsDemoMutation) -> Vec<String> {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), &*snapshot) {
        *snapshot = next;
    }
    outcome.messages().iter().map(|message| message.code.0.clone()).collect()
}
