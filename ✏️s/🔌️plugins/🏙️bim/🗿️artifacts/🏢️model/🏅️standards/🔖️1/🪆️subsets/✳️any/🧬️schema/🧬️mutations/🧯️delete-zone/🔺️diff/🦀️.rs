//! 🔺️ Diff constructor for `DeleteZone`: the zone leaves in one sparse diff together with its properties and classifications (see the shared
//! cascade), and every space that belonged to it is patched to no zone, so nothing is left dangling. An area scheme that still counts the zone is
//! refused as `mutation.target-referenced`: dropping the zone from its list would silently widen the rule to every zone.

use crate::standards::v1::subsets::any::schema::mutations::delete_zone::DeleteZone;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Assigned, Entry, KeyedDelta};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::SpacePatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteZone, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.zones.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Zone \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some((scheme, _)) = base.area_schemes.iter().find(|(_, row)| row.zones.contains(&payload.id)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Zone \"{}\" is still counted by area scheme \"{scheme}\".", payload.id), [payload.id.clone()]);
    }
    let removal = match semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::closure(base, std::slice::from_ref(&payload.id)) {
        Ok(removal) => removal,
        Err(refusal) => return MutationOutcome::refuse(refusal.code, refusal.message, [refusal.target]),
    };
    let members = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::zone_members(base, &payload.id);
    let cleared = members.len();
    let outcome = MutationOutcome::new(ModelDiff {
        spaces: (!members.is_empty()).then(|| KeyedDelta(members.into_iter().map(|space| (space, Entry::Patched(SpacePatch { zone: Some(Assigned::new(None)), ..Default::default() }))).collect())),
        ..removal.diff()
    });
    if cleared == 0 {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("Zone \"{}\" left {cleared} space(s) without a zone.", payload.id))
    }
}
