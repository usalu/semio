//! 🌉️ Central apply entry point: the only code of this artifact that turns a mutation's diff into the next snapshot; the schema and mutation leaves only build diffs.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::Fem3dSnapshot;

/// 🎬️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation` runs `diff`, `apply_diff` and the cold retirement).
pub fn apply_fem3d_mutation(snapshot: &mut Fem3dSnapshot, mutation: &Fem3dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(snapshot, mutation)?;
    *snapshot = next;
    Ok(())
}
