//! 🔺️ Diff constructor for `MoveOpening`: a patch of the opening's centre offset and, when it is re-hosted, its host. The new host
//! (a wall or curtain wall) must exist and the opening must still fit inside its axis and clear of its neighbours.

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::placement::{host_length, placement_issue, width_of};
use crate::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::ceilings_roofs_openings::OpeningPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &MoveOpening, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(opening) = base.openings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Opening \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let host = payload.host.as_deref().unwrap_or(&opening.host);
    let Some(length) = host_length(base, host) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Host \"{host}\" is neither a wall nor a curtain wall."), ["host"]);
    };
    if host == opening.host && payload.offset == opening.offset {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Opening \"{}\" already sits there.", payload.id), [payload.id.clone()]);
    }
    let Some(width) = width_of(base, opening) else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Opening \"{}\" has no resolvable width.", payload.id), [payload.id.clone()]);
    };
    if let Some(message) = placement_issue(base, Some(&payload.id), host, length, payload.offset, width) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["offset"]);
    }
    let patch = OpeningPatch { host: (host != opening.host).then(|| host.to_string()), offset: (payload.offset != opening.offset).then_some(payload.offset), ..Default::default() };
    MutationOutcome::new(ModelDiff::openings(payload.id.clone(), Entry::Patched(patch)))
}
