//! 🔺️ Diff constructor for `DeleteLeader`: the leader leaves in one sparse diff (see the shared cascade); the elements it names stay untouched.

use super::super::cascade;
use super::DeleteLeader;
use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteLeader, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.leaders.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Leader \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Leader", Some(&payload.id))
}
