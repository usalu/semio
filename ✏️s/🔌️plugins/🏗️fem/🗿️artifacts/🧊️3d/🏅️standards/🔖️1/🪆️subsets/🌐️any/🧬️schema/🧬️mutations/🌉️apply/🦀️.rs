//! 🌉️ Central apply entry point: the only code of this artifact that turns a mutation's diff into the next snapshot; the schema and mutation leaves only build diffs.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

/// 🎬️ Applies one mutation to a projection through the central cold applier: `diff`, `apply_diff_cold_into` and the cold retirement of the diff.
pub fn apply_fem3d_mutation(snapshot: &mut Fem3dSnapshotValue, mutation: &Fem3dMutation) -> protocol::MutationApplyResult<()> {
    let (diff, _) = <Fem3dMutation as protocol::ColdMutation<Fem3dSnapshotValue>>::diff(mutation, snapshot).into_parts();
    let applied = protocol::apply_diff_cold_into(&diff, snapshot);
    protocol::MutationDiff::<Fem3dSnapshotValue>::retire_cold(diff);
    applied
}
