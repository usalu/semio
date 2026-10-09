//! 🔺️ Diff constructor for `DeleteZone`: the zone leaves in one sparse diff together with its properties and classifications (see the shared
//! cascade), and every space that belonged to it is patched to no zone, so nothing is left dangling. An area scheme that still counts the zone is
//! refused as `mutation.target-referenced`: dropping the zone from its list would silently widen the rule to every zone.

use super::super::cascade;
use super::super::elements;
use super::DeleteZone;
use crate::{Assigned, Entry, KeyedDelta, ModelDiff, ModelSnapshot, SpacePatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteZone, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.zones.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Zone \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some((scheme, _)) = base.area_schemes.iter().find(|(_, row)| row.zones.contains(&payload.id)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Zone \"{}\" is still counted by area scheme \"{scheme}\".", payload.id), [payload.id.clone()]);
    }
    let removal = match cascade::closure(base, std::slice::from_ref(&payload.id)) {
        Ok(removal) => removal,
        Err(refusal) => return MutationOutcome::refuse(refusal.code, refusal.message, [refusal.target]),
    };
    let members = elements::zone_members(base, &payload.id);
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
