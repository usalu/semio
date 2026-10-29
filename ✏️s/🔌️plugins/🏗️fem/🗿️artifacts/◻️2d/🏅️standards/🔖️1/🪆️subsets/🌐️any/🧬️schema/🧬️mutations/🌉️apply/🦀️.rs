//! 🌉️ Central apply entry point: the only code of this artifact that turns a mutation's diff into the next snapshot; the schema and mutation leaves only build diffs.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

/// 🎬️ Applies one mutation to a projection through the central cold applier: `diff`, `apply_diff_cold_into` and the cold retirement of the diff.
pub fn apply_fem2d_mutation(snapshot: &mut Fem2dSnapshotValue, mutation: &Fem2dMutation) -> protocol::MutationApplyResult<()> {
    let (diff, _) = <Fem2dMutation as protocol::ColdMutation<Fem2dSnapshotValue>>::diff(mutation, snapshot).into_parts();
    let applied = protocol::apply_diff_cold_into(&diff, snapshot);
    protocol::MutationDiff::<Fem2dSnapshotValue>::retire_cold(diff);
    applied
}
