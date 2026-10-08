//! 🌉️ Central apply entry point: the only code of this artifact that turns a mutation's diff into the next snapshot; the schema and mutation leaves only build diffs.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;

/// 🎬️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation` runs `diff`, `apply_diff` and the cold retirement).
pub fn apply_fem2d_mutation(snapshot: &mut Fem2dSnapshot, mutation: &Fem2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(snapshot, mutation)?;
    *snapshot = next;
    Ok(())
}
