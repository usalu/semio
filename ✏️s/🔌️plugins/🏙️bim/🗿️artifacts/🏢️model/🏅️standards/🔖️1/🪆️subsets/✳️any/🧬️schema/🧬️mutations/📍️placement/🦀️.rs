//! 📍️ Shared reads of the opening and stair leaves. Openings: the length of a host, the width an opening resolves to and whether a
//! centre offset keeps it inside its host and clear of its neighbours. Stairs: the invariants every authored stair must hold. Columns: the authored rise between a base and a top constraint.
//! Nothing here is stored; lengths and widths are derived from authored parameters on every read.

use crate::standards::v1::subsets::any::schema::inferences::opening_frames::resolve_size;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{resolve, stacking, StoreyLevel};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::axis_length;
use crate::{ramp_construction_problem, stair_construction_problem, ModelDiff, ModelSnapshot, Opening, OpeningKind, Railing, Ramp, Stair, StairFlight, TopConstraint};
use protocol::{MutationOutcome, OutcomeCode};

const TOLERANCE: f64 = 1e-9;

/// 🔢️ Whether `value` is a finite length above zero.
pub fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

//#region 🔖️Openings
/// ⚠️ Why an opening kind cannot be used: a type it names is absent, or a void has no size.
pub enum KindIssue {
    Missing(String),
    Degenerate(String),
}

/// 📏️ Length of the axis of a wall or curtain wall hosting openings, none when `host` names neither.
pub fn host_length(base: &ModelSnapshot, host: &str) -> Option<f64> {
    base.walls.get(host).map(|wall| axis_length(&wall.axis)).or_else(|| base.curtain_walls.get(host).map(|wall| axis_length(&wall.axis)))
}

/// 📐️ Width the opening kind itself prescribes, none when the type it names is absent.
pub fn kind_width(base: &ModelSnapshot, kind: &OpeningKind) -> Option<f64> {
    match kind {
        OpeningKind::Window { window_type } => base.window_types.get(window_type).map(|row| row.width),
        OpeningKind::Door { door_type } => base.door_types.get(door_type).map(|row| row.width),
        OpeningKind::Void { width, .. } => Some(*width),
    }
}

/// 📐️ Width an opening resolves to: its own override, else the width of its kind.
pub fn width_of(base: &ModelSnapshot, opening: &Opening) -> Option<f64> {
    opening.width.or_else(|| kind_width(base, &opening.kind))
}

/// 🔎️ The reason `kind` cannot be used against `base`, none when it can.
pub fn kind_issue(base: &ModelSnapshot, kind: &OpeningKind) -> Option<KindIssue> {
    match kind {
        OpeningKind::Window { window_type } if !base.window_types.contains_key(window_type) => Some(KindIssue::Missing(format!("Window type \"{window_type}\" does not exist."))),
        OpeningKind::Door { door_type } if !base.door_types.contains_key(door_type) => Some(KindIssue::Missing(format!("Door type \"{door_type}\" does not exist."))),
        OpeningKind::Void { width, height } if !(positive(*width) && positive(*height)) => Some(KindIssue::Degenerate("A void needs a positive width and height.".to_string())),
        _ => None,
    }
}

/// 🚫️ The refusal of a [`KindIssue`] at `path`.
pub fn refuse_kind<P>(issue: KindIssue, path: P) -> MutationOutcome<ModelDiff>
where
    P: IntoIterator,
    P::Item: Into<String>,
{
    match issue {
        KindIssue::Missing(message) => MutationOutcome::refuse(OutcomeCode::TargetMissing, message, path),
        KindIssue::Degenerate(message) => MutationOutcome::refuse(OutcomeCode::Invariant, message, path),
    }
}

/// 📍️ Why an opening of `width` centred `offset` along a `length` host does not fit, none when it does. `skip` names an opening that
/// is being moved or resized and therefore does not collide with itself.
pub fn placement_issue(base: &ModelSnapshot, skip: Option<&str>, host: &str, length: f64, offset: f64, width: f64) -> Option<String> {
    let half = width / 2.0;
    if !offset.is_finite() || offset < half - TOLERANCE || offset > length - half + TOLERANCE {
        return Some(format!("An opening of {width} m centred {offset} m along its {length} m host does not fit inside it."));
    }
    base.openings.iter().filter(|(id, other)| other.host == host && Some(id.as_str()) != skip).find_map(|(id, other)| {
        let reach = (width + width_of(base, other)?) / 2.0;
        ((other.offset - offset).abs() < reach - TOLERANCE).then(|| format!("Opening \"{id}\" already occupies that stretch of the host."))
    })
}

/// 📏️ The first opening hosted by `host` (in id order) whose cut rises above a host that is `height` metres tall, none when every hosted opening fits below it.
pub fn overflowing_opening(base: &ModelSnapshot, host: &str, height: f64) -> Option<String> {
    base.openings.iter().filter(|(_, opening)| opening.host == host).find_map(|(id, opening)| {
        let size = resolve_size(base, opening);
        (size.sill + size.height > height + TOLERANCE).then(|| id.clone())
    })
}
//#endregion 🔖️Openings

//#region 🔖️Stairs
/// ⚠️ A broken stair invariant: its outcome code, message and the field path below the stair record.
pub struct StairIssue {
    pub code: OutcomeCode,
    pub message: String,
    pub path: Vec<&'static str>,
}

fn invalid(field: &'static str, message: &str) -> Option<StairIssue> {
    Some(StairIssue { code: OutcomeCode::Invariant, message: message.to_string(), path: vec![field] })
}

fn flight_issue(flight: &StairFlight) -> Option<StairIssue> {
    let broken = match flight {
        StairFlight::Straight => false,
        StairFlight::LTurn { split, .. } => !positive(*split),
        StairFlight::UTurn { gap } => !(gap.is_finite() && *gap >= 0.0),
        StairFlight::Spiral { radius, sweep } => !positive(*radius) || !sweep.is_finite() || *sweep == 0.0,
    };
    broken.then(|| StairIssue { code: OutcomeCode::Invariant, message: "The flight of a stair needs finite, sensible dimensions.".to_string(), path: vec!["flight"] })
}

fn top_issue(base: &ModelSnapshot, stair: &Stair) -> Option<StairIssue> {
    match &stair.top {
        TopConstraint::Unconnected { height } if !positive(*height) => invalid("top", "An unconnected stair top needs a positive height."),
        TopConstraint::StoreyTop { offset } | TopConstraint::Unconnected { height: offset } if !offset.is_finite() => invalid("top", "A stair top offset must be finite."),
        TopConstraint::Roof { .. } | TopConstraint::Slab { .. } | TopConstraint::Ceiling { .. } => invalid("top", "Only a wall attaches its top to a roof, a slab or a ceiling."),
        TopConstraint::Storey { storey, offset } => {
            if !offset.is_finite() {
                return invalid("top", "A stair top offset must be finite.");
            }
            let own = base.storeys.get(&stair.storey).map(|row| &row.building);
            match base.storeys.get(storey) {
                None => Some(StairIssue { code: OutcomeCode::TargetMissing, message: format!("Storey \"{storey}\" does not exist."), path: vec!["top", "storey"] }),
                Some(row) if Some(&row.building) != own => Some(StairIssue { code: OutcomeCode::Invariant, message: format!("Storey \"{storey}\" belongs to another building."), path: vec!["top", "storey"] }),
                Some(_) => None,
            }
        }
        _ => None,
    }
}

/// 🔎️ The first broken invariant of `stair` against `base`, none when it holds. The stair's own storey is checked by the caller.
pub fn stair_issue(base: &ModelSnapshot, stair: &Stair) -> Option<StairIssue> {
    if !(stair.start.x.is_finite() && stair.start.y.is_finite()) {
        return invalid("start", "A stair start must be a finite point.");
    }
    if !stair.direction.is_finite() {
        return invalid("direction", "A stair direction must be a finite angle.");
    }
    if !positive(stair.width) {
        return invalid("width", "A stair width must be a positive length.");
    }
    if !positive(stair.max_riser) {
        return invalid("max_riser", "A stair maximum riser must be a positive length.");
    }
    if !positive(stair.min_tread) {
        return invalid("min_tread", "A stair minimum tread must be a positive length.");
    }
    flight_issue(&stair.flight).or_else(|| top_issue(base, stair)).or_else(|| stair_construction_problem(stair).and_then(|(field, message)| invalid(field, message)))
}

/// 🚫️ The refusal of a [`StairIssue`] below the record path `prefix`.
pub fn refuse_stair(issue: StairIssue, prefix: &[&str]) -> MutationOutcome<ModelDiff> {
    let path: Vec<String> = prefix.iter().chain(issue.path.iter()).map(|segment| segment.to_string()).collect();
    MutationOutcome::refuse(issue.code, issue.message, path)
}
//#endregion 🔖️Stairs

//#region 🔖️Ramps
fn ramp_top_issue(base: &ModelSnapshot, ramp: &Ramp) -> Option<StairIssue> {
    match &ramp.top {
        TopConstraint::Unconnected { height: offset } | TopConstraint::StoreyTop { offset } if !offset.is_finite() => invalid("top", "A ramp top offset must be finite."),
        TopConstraint::Roof { .. } | TopConstraint::Slab { .. } | TopConstraint::Ceiling { .. } => invalid("top", "Only a wall attaches its top to a roof, a slab or a ceiling."),
        TopConstraint::Storey { storey, offset } => {
            if !offset.is_finite() {
                return invalid("top", "A ramp top offset must be finite.");
            }
            let own = base.storeys.get(&ramp.storey).map(|row| &row.building);
            match base.storeys.get(storey) {
                None => Some(StairIssue { code: OutcomeCode::TargetMissing, message: format!("Storey \"{storey}\" does not exist."), path: vec!["top", "storey"] }),
                Some(row) if Some(&row.building) != own => Some(StairIssue { code: OutcomeCode::Invariant, message: format!("Storey \"{storey}\" belongs to another building."), path: vec!["top", "storey"] }),
                Some(_) => None,
            }
        }
        _ => None,
    }
}

/// 🛝️ The first broken invariant of `ramp` against `base`, none when it holds: the construction rules, a finite top and a top storey of the same building, a material that exists. The ramp's own storey is checked by the caller.
pub fn ramp_issue(base: &ModelSnapshot, ramp: &Ramp) -> Option<StairIssue> {
    if let Some((field, message)) = ramp_construction_problem(ramp) {
        return invalid(field, message);
    }
    if !base.materials.contains_key(&ramp.material) {
        return Some(StairIssue { code: OutcomeCode::TargetMissing, message: format!("Material \"{}\" does not exist.", ramp.material), path: vec!["material"] });
    }
    ramp_top_issue(base, ramp)
}
//#endregion 🔖️Ramps

//#region 🔖️RailingHosts
/// 🪝️ The first broken host rule of `railing` against `base`, none when it holds or the railing is not hosted: the host is a stair, a ramp or a slab of the same building, a slab edge exists and is straight, a stair or ramp has no edge index.
pub fn host_issue(base: &ModelSnapshot, railing: &Railing) -> Option<StairIssue> {
    let host = railing.host.as_ref()?;
    let building = |storey: &str| base.storeys.get(storey).map(|row| &row.building);
    let (storey, edges) = if let Some(stair) = base.stairs.get(&host.element) {
        (&stair.storey, None)
    } else if let Some(ramp) = base.ramps.get(&host.element) {
        (&ramp.storey, None)
    } else if let Some(slab) = base.slabs.get(&host.element) {
        (&slab.storey, Some(&slab.boundary))
    } else {
        return Some(StairIssue { code: OutcomeCode::TargetMissing, message: format!("Element \"{}\" is no stair, ramp or slab.", host.element), path: vec!["host", "element"] });
    };
    if building(storey).is_none() || building(storey) != building(&railing.storey) {
        return Some(StairIssue { code: OutcomeCode::Invariant, message: format!("Element \"{}\" stands in another building.", host.element), path: vec!["host", "element"] });
    }
    match edges {
        None if host.edge != 0 => invalid("host", "A stair or ramp host has no edge index; use side and inset."),
        Some(boundary) if host.edge as usize >= boundary.len() => Some(StairIssue { code: OutcomeCode::Invariant, message: format!("Slab \"{}\" has no edge {}.", host.element, host.edge), path: vec!["host", "edge"] }),
        Some(boundary) if boundary[host.edge as usize].bulge != 0.0 => Some(StairIssue { code: OutcomeCode::Invariant, message: format!("Edge {} of slab \"{}\" is curved; a railing follows straight edges only.", host.edge, host.element), path: vec!["host", "edge"] }),
        _ => None,
    }
}
//#endregion 🔖️RailingHosts

//#region 🔖️Rise
/// 🪜️ The elevation of storey `id` above its building datum, summed from stored heights along the stacking of its building: a pure read of
/// `base` that runs no inference engine, none when the storey is absent.
pub fn storey_elevation(base: &ModelSnapshot, id: &str) -> Option<f64> {
    let storey = base.storeys.get(id)?;
    let mut levels: std::collections::BTreeMap<String, StoreyLevel> = std::collections::BTreeMap::new();
    for (key, parent) in stacking(base, &storey.building) {
        let level = resolve(base, &key, parent.as_ref().and_then(|parent| levels.get(parent)));
        if key == id {
            return Some(level.elevation);
        }
        levels.insert(key, level);
    }
    None
}

/// 📏️ The authored rise between a base offset on storey `storey_id` and the top constraint `top`, none when the storey the rise is measured
/// from or the storey the top names is absent.
pub fn rise(base: &ModelSnapshot, storey_id: &str, base_offset: f64, top: &TopConstraint) -> Option<f64> {
    let storey = base.storeys.get(storey_id)?;
    match top {
        TopConstraint::Unconnected { height } => Some(*height),
        TopConstraint::StoreyTop { offset } => Some(storey.height + offset - base_offset),
        TopConstraint::Storey { storey: target, offset } if target == storey_id => Some(offset - base_offset),
        TopConstraint::Storey { storey: target, offset } => Some(storey_elevation(base, target)? + offset - storey_elevation(base, storey_id)? - base_offset),
        TopConstraint::Roof { .. } | TopConstraint::Slab { .. } | TopConstraint::Ceiling { .. } => None,
    }
}
//#endregion 🔖️Rise
