//! 🌉️ Central apply entry point: the only code of this artifact that turns a mutation's diff into the next snapshot; the schema and mutation leaves only build diffs.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

/// 🎬️ Applies one mutation to a projection through the central applier (`vcs::apply_mutation` runs `diff`, `apply_diff_cold` and the cold retirement).
pub fn apply_fem2d_mutation(snapshot: &mut Fem2dSnapshotValue, mutation: &Fem2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(snapshot, mutation)?;
    *snapshot = next;
    Ok(())
}
