//! 🔺️ Diff constructor for `SetSite`: a sparse site patch of exactly the provided fields. Latitude lies in -90..90 and longitude
//! in -180..180 degrees, elevation and true north are finite, a boundary is empty (cleared) or a loop of at least three finite
//! points enclosing an area. A patch that restates the current values is a `mutation.no-op`. Nothing downstream is written: the
//! absolute elevation of every storey on the site follows the site elevation by inference (`storey-levels`).
//! Whole-list ruling: the boundary is ONE closed plot polygon, so a provided boundary replaces the boundary as one field.

use super::SetSite;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch, Point2};
use protocol::{MutationOutcome, OutcomeCode};

fn enclosed(points: &[Point2]) -> bool {
    let area: f64 = points.iter().zip(points.iter().cycle().skip(1)).map(|(from, to)| from.x * to.y - to.x * from.y).sum();
    points.len() >= 3 && points.iter().all(|point| point.x.is_finite() && point.y.is_finite()) && area.abs() > 0.0
}

pub fn diff(payload: &SetSite, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(site) = base.sites.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Site \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.latitude.is_some_and(|value| !(-90.0..=90.0).contains(&value)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A latitude lies within -90 and 90 degrees.", ["latitude"]);
    }
    if payload.longitude.is_some_and(|value| !(-180.0..=180.0).contains(&value)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A longitude lies within -180 and 180 degrees.", ["longitude"]);
    }
    if payload.elevation.is_some_and(|value| !value.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A site elevation must be a finite length.", ["elevation"]);
    }
    if payload.true_north.is_some_and(|value| !value.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A true north angle must be finite.", ["true_north"]);
    }
    if payload.boundary.as_deref().is_some_and(|points| !points.is_empty() && !enclosed(points)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A site boundary is empty or a loop of at least three points enclosing an area.", ["boundary"]);
    }
    let patch = payload.patch().minimal(site);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Site \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::sites(payload.id.clone(), Entry::Patched(patch)))
}
