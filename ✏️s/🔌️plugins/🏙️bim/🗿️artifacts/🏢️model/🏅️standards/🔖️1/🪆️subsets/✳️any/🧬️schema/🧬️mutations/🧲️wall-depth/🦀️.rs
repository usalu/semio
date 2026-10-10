//! 🧗️ Shared rules of the wall depth leaves: the invariants of a wall sweep, the attach of a wall top to a roof, slab or ceiling and of a wall base to a slab (target exists, same building, finite offset, no loop of
//! references), and the authored reveal of an opening. Pure reads of the authored snapshot: no inference runs inside a diff, so the check of a loop walks the references of the records, never an inferred surface.

use crate::mutations::wall_geometry::{material_flaw, profile_flaw, Flaw, EPS};
use crate::standards::v1::subsets::any::schema::authored::profile::{profile_extents, profile_polygon};
use crate::standards::v1::subsets::any::schema::authored::references::{edges, find_cycle, top_target};
use crate::{ModelSnapshot, TopConstraint, WallSweep};
use protocol::OutcomeCode;

fn invariant(path: &[&str], message: &str) -> Option<Flaw> {
    Some(Flaw::new(OutcomeCode::Invariant, path, message))
}

//#region 🔖️Sweeps
/// 🧷️ The first reason a whole wall sweep record is invalid against `base`, with paths relative to the record: the host wall exists, the profile has positive dimensions, the height above the base is not negative, the inset is
/// not negative and leaves some of the profile showing, and the material exists.
pub fn sweep_flaw(base: &ModelSnapshot, sweep: &WallSweep) -> Option<Flaw> {
    if !base.walls.contains_key(&sweep.host) {
        return Some(Flaw::new(OutcomeCode::TargetMissing, &["host"], format!("Wall \"{}\" does not exist.", sweep.host)));
    }
    if profile_flaw(&sweep.profile).is_some() {
        return invariant(&["profile"], "A sweep profile needs positive dimensions.");
    }
    if !(sweep.height.is_finite() && sweep.height >= 0.0) {
        return invariant(&["height"], "The height of a sweep above the wall base must not be negative.");
    }
    let (across, _) = profile_extents(&profile_polygon(&sweep.profile));
    if !(sweep.inset.is_finite() && sweep.inset >= 0.0 && sweep.inset < across - EPS) {
        return invariant(&["inset"], "The inset of a sweep must not be negative and must leave part of the profile showing.");
    }
    material_flaw(base, "material", &sweep.material)
}
//#endregion 🔖️Sweeps

//#region 🔖️Attach
fn same_building(base: &ModelSnapshot, left: &str, right: &str) -> bool {
    base.storeys.get(left).zip(base.storeys.get(right)).is_some_and(|(a, b)| a.building == b.building)
}

fn target_flaw(base: &ModelSnapshot, storey: &str, path: &[&str], noun: &str, id: &str, found: Option<&String>) -> Option<Flaw> {
    match found {
        None => Some(Flaw::new(OutcomeCode::TargetMissing, path, format!("{noun} \"{id}\" does not exist."))),
        Some(target) if !same_building(base, storey, target) => invariant(path, &format!("{noun} \"{id}\" belongs to another building.")),
        Some(_) => None,
    }
}

/// 🔗️ The first reason an attach of the wall `wall` (a new wall when `None`) standing on `storey` is invalid: the roof, slab or ceiling of a top attach and the slab of a base attach must exist in the same building, the offset must be
/// finite and the chain of references must not loop. A top that is no attach and an absent base slab are not judged here. Paths are relative to the wall record (`top/roof`, `base_slab`).
pub fn attach_flaw(base: &ModelSnapshot, wall: Option<&str>, storey: &str, top: &TopConstraint, base_slab: Option<&str>) -> Option<Flaw> {
    let top_flaw = match top {
        TopConstraint::Roof { roof, offset } => target_flaw(base, storey, &["top", "roof"], "Roof", roof, base.roofs.get(roof).map(|row| &row.storey)).or_else(|| (!offset.is_finite()).then(|| Flaw::new(OutcomeCode::Invariant, &["top", "offset"], "A top offset must be finite."))),
        TopConstraint::Slab { slab, offset } => target_flaw(base, storey, &["top", "slab"], "Slab", slab, base.slabs.get(slab).map(|row| &row.storey)).or_else(|| (!offset.is_finite()).then(|| Flaw::new(OutcomeCode::Invariant, &["top", "offset"], "A top offset must be finite."))),
        TopConstraint::Ceiling { ceiling, offset } => target_flaw(base, storey, &["top", "ceiling"], "Ceiling", ceiling, base.ceilings.get(ceiling).map(|row| &row.storey)).or_else(|| (!offset.is_finite()).then(|| Flaw::new(OutcomeCode::Invariant, &["top", "offset"], "A top offset must be finite."))),
        _ => None,
    };
    if top_flaw.is_some() {
        return top_flaw;
    }
    if let Some(slab) = base_slab {
        if let Some(flaw) = target_flaw(base, storey, &["base_slab"], "Slab", slab, base.slabs.get(slab).map(|row| &row.storey)) {
            return Some(flaw);
        }
    }
    let id = wall?;
    let proposed: Vec<String> = top_target(top).map(|(target, _)| target.to_string()).into_iter().chain(base_slab.map(str::to_string)).collect();
    let authored = edges(base);
    let graph = |node: &str| if node == id { proposed.clone() } else { authored(node) };
    find_cycle(id, &graph).and_then(|_| invariant(&["top"], "The attach references of the wall loop."))
}
//#endregion 🔖️Attach

//#region 🔖️Reveal
/// 🪟️ The first reason an authored reveal is invalid: a negative or non-finite depth, a material that does not exist. Paths are relative to the opening record.
pub fn reveal_flaw(base: &ModelSnapshot, depth: Option<f64>, material: Option<&str>) -> Option<Flaw> {
    if depth.is_some_and(|depth| !(depth.is_finite() && depth >= 0.0)) {
        return invariant(&["reveal_depth"], "A reveal depth must not be negative.");
    }
    material.and_then(|material| material_flaw(base, "reveal_material", material))
}
//#endregion 🔖️Reveal

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
