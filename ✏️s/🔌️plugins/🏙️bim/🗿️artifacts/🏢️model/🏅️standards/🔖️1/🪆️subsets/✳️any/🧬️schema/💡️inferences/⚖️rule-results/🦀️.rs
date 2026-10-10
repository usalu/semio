//! ⚖️ `rule-results`: what the authored numeric code checks (rules) find in the model. Everything here is derived from the runs, frames, rooms and zones of the model, nothing is stored.
//!
//! Definitions. Lengths in metres, areas in square metres, a slope as rise over run.
//! * a rule measures the members its scope covers: for the stair rules the stairs (`MaxRiser`: the riser height, `MinTread`: the tread depth, `MinStairWidth`: the clear width of the flights), for `MinDoorWidth` the width of the
//!   frame of the doors, for `MaxRampSlope` the slope of the ramps, for `MinClearHeight` and `MinCorridorWidth` the resolved spaces (the scope filter names their usage, compared without regard to case), for
//!   `MaxCompartmentArea` the net floor area of the zones (the scope filter names their category);
//! * the width of a corridor is the short side of the rectangle that has the area and the perimeter of its room (exact for a rectangular room, a good measure of a bent strip);
//! * a member violates a minimum rule when it measures less than the limit and a maximum rule when it measures more, with a tolerance of a nanometre; a member the model cannot measure (an unresolved room, a stair
//!   without risers) is not counted;
//! * the result lists the violations in element id order with the measured value, the limit and the storey of the member.
//!
//! The graph has one `Rule` node per rule; its parents are the nodes that hold the measures of its members (`StairRun`, `OpeningFrame`, `RampRun`, `Room`, `Zone`).
//!
//! Related: DIN 18065 (stairs), DIN 18040 (accessible building), ASR A1.8, the building codes of the federal states (MBO), <https://www.din.de>.

use super::super::element_solids::{dep_object, dep_value};
use super::super::opening_frames::OpeningFrame;
use super::super::ramp_runs::RampRun;
use super::super::spaces::{SpaceRoom, SpaceStatus, StoreyRooms};
use super::super::stair_runs::StairRun;
use super::super::zones::ZoneTotals;
use crate::{ModelSnapshot, OpeningKind, Rule, RuleKind};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

/// 🗺️ The snapshot collections the rules read, directly or through the runs, frames, rooms and zones they measure.
pub const READS: &[&str] = &["rules", "stairs", "ramps", "spaces", "zones", "openings", "walls", "wall_types", "curtain_walls", "curtain_wall_types", "window_types", "door_types", "columns", "column_types", "slabs", "slab_types", "ceilings", "ceiling_types", "materials", "storeys", "buildings", "sites"];

/// 📏️ How far past the limit a measure must be to violate: a nanometre.
pub const EPSILON: f64 = 1e-9;

//#region 🔖️Values
/// 🚫️ One member that breaks a rule: what it is, what it measures, the limit and the storey it stands on (empty for a zone).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RuleFinding {
    pub element: String,
    pub measured: f64,
    pub limit: f64,
    pub storey: String,
}

/// ⚖️ What one rule finds: how many members it measured and the violations among them.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RuleResult {
    pub checked: u32,
    pub violations: Vec<RuleFinding>,
}

impl RuleResult {
    /// ✅️ How many measured members keep the rule.
    pub fn passed(&self) -> usize {
        self.checked as usize - self.violations.len()
    }
}

/// 🧩️ One member of a rule: the id of the measured element (a stair, door, ramp, space or zone) and the storey it stands on.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Member {
    pub id: String,
    pub storey: Option<String>,
}

/// 📥️ The measures a rule reads from the parents of its node, by id.
#[derive(Default)]
pub struct Inputs<'a> {
    pub rooms: BTreeMap<&'a str, &'a StoreyRooms>,
    pub runs: BTreeMap<&'a str, &'a StairRun>,
    pub ramp_runs: BTreeMap<&'a str, &'a RampRun>,
    pub frames: BTreeMap<&'a str, &'a OpeningFrame>,
    pub zones: BTreeMap<&'a str, &'a ZoneTotals>,
}
//#endregion 🔖️Values

//#region 🔖️Members
/// 📐️ The short side of the rectangle that has `area` and `perimeter`: `(perimeter - sqrt(perimeter^2 - 16 area)) / 4`, the half perimeter when the room is rounder than any rectangle.
pub fn equivalent_width(area: f64, perimeter: f64) -> f64 {
    let discriminant = (perimeter * perimeter - 16.0 * area).max(0.0);
    ((perimeter - discriminant.sqrt()) / 4.0).max(0.0)
}

/// 🧩️ The members of `rule` in id order: the elements its kind measures that its scope covers (storeys, phases, element ids and, for spaces and zones, the filter).
pub fn members(snapshot: &ModelSnapshot, rule: &Rule) -> Vec<Member> {
    let scope = &rule.scope;
    let filtered = |text: &str| scope.filter.trim().is_empty() || text.trim().eq_ignore_ascii_case(scope.filter.trim());
    let mut rows: Vec<Member> = Vec::new();
    match rule.kind {
        RuleKind::MaxRiser | RuleKind::MinTread | RuleKind::MinStairWidth => {
            rows.extend(snapshot.stairs.iter().filter(|(id, row)| scope.covers(snapshot, id, Some(&row.storey))).map(|(id, row)| Member { id: id.clone(), storey: Some(row.storey.clone()) }));
        }
        RuleKind::MaxRampSlope => {
            rows.extend(snapshot.ramps.iter().filter(|(id, row)| scope.covers(snapshot, id, Some(&row.storey))).map(|(id, row)| Member { id: id.clone(), storey: Some(row.storey.clone()) }));
        }
        RuleKind::MinDoorWidth => {
            rows.extend(snapshot.openings.iter().filter(|(_, row)| matches!(row.kind, OpeningKind::Door { .. })).filter_map(|(id, _)| {
                let storey = crate::storey_of(snapshot, id)?;
                scope.covers(snapshot, id, Some(storey)).then(|| Member { id: id.clone(), storey: Some(storey.clone()) })
            }));
        }
        RuleKind::MinClearHeight | RuleKind::MinCorridorWidth => {
            rows.extend(snapshot.spaces.iter().filter(|(id, row)| filtered(&row.usage) && scope.covers(snapshot, id, Some(&row.storey))).map(|(id, row)| Member { id: id.clone(), storey: Some(row.storey.clone()) }));
        }
        RuleKind::MaxCompartmentArea => {
            rows.extend(snapshot.zones.iter().filter(|(id, row)| filtered(&row.category) && scope.covers(snapshot, id, None)).map(|(id, _)| Member { id: id.clone(), storey: None }));
        }
    }
    rows
}
//#endregion 🔖️Members

//#region 🔖️Results
fn resolved(room: &SpaceRoom) -> bool {
    matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit)
}

fn measure(rule: &Rule, member: &Member, inputs: &Inputs<'_>) -> Option<f64> {
    let id = member.id.as_str();
    match rule.kind {
        RuleKind::MaxRiser => inputs.runs.get(id).filter(|run| run.riser_count > 0).map(|run| run.riser_height),
        RuleKind::MinTread => inputs.runs.get(id).filter(|run| run.tread_count > 0).map(|run| run.tread),
        RuleKind::MinStairWidth => inputs.runs.get(id).filter(|run| run.riser_count > 0).map(|run| run.width),
        RuleKind::MinDoorWidth => inputs.frames.get(id).map(|frame| frame.width),
        RuleKind::MaxRampSlope => inputs.ramp_runs.get(id).filter(|run| run.length > 0.0).map(|run| run.slope),
        RuleKind::MaxCompartmentArea => inputs.zones.get(id).filter(|zone| zone.resolved > 0).map(|zone| zone.net_area),
        RuleKind::MinClearHeight | RuleKind::MinCorridorWidth => {
            let room = member.storey.as_deref().and_then(|storey| inputs.rooms.get(storey)).and_then(|rooms| rooms.get(id)).filter(|room| resolved(room))?;
            Some(if rule.kind == RuleKind::MinClearHeight { room.clear_height } else { equivalent_width(room.area, room.perimeter) })
        }
    }
}

/// ⚖️ The result of `rule` over its `members` and the `inputs` the parents hold.
pub fn result_of(rule: &Rule, members: &[Member], inputs: &Inputs<'_>) -> RuleResult {
    let mut result = RuleResult::default();
    for member in members {
        let Some(measured) = measure(rule, member, inputs) else { continue };
        result.checked += 1;
        let breaks = if rule.kind.is_maximum() { measured > rule.limit + EPSILON } else { measured < rule.limit - EPSILON };
        if breaks {
            result.violations.push(RuleFinding { element: member.id.clone(), measured, limit: rule.limit, storey: member.storey.clone().unwrap_or_default() });
        }
    }
    result
}
//#endregion 🔖️Results

//#region 🔖️Dependency
/// 🔑️ What the `Rule` node of `id` reads of the snapshot besides the measures of its parents: the rule and its members with their storeys.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(rule) = snapshot.rules.get(id) else { return DslValue::Null };
    let rows = DslValue::object(members(snapshot, rule).into_iter().map(|member| (member.id.clone(), dep_value(&member.storey))));
    dep_object([("rule", dep_value(rule)), ("members", rows)])
}
//#endregion 🔖️Dependency

//#region 🔖️Tables
fn number(value: f64) -> DslValue {
    DslValue::float(value)
}

/// ⚖️ The table the third-party oracle recomputes, `{ <rule id>: { checked, violations: [ { element, measured, limit, storey } ] } }`.
pub fn table_json(results: &BTreeMap<String, RuleResult>) -> String {
    let rules = results.iter().map(|(id, result)| {
        let violations = result.violations.iter().map(|finding| {
            DslValue::object([
                ("element".to_string(), DslValue::String(finding.element.clone())),
                ("measured".to_string(), number(finding.measured)),
                ("limit".to_string(), number(finding.limit)),
                ("storey".to_string(), DslValue::String(finding.storey.clone())),
            ])
        });
        (id.clone(), DslValue::object([("checked".to_string(), DslValue::int(i64::from(result.checked))), ("violations".to_string(), DslValue::Array(violations.collect()))]))
    });
    semio_framework_pack_json::to_json_string(&DslValue::object(rules))
}

/// 📏️ The measures the rules read, `{ stairs, frames, ramps, zones, rooms }`: the committed input of the rules oracle, written by the blessing test from the inferred runs, door frames, ramp runs, zone totals and rooms
/// (the outlines of the rooms are what the oracle measures with shapely).
pub fn measures_json(snapshot: &ModelSnapshot, inferred: &crate::ModelInference) -> String {
    let by_id = |rows: Vec<(String, DslValue)>| DslValue::object(rows);
    let stairs = by_id(
        inferred
            .stair_runs
            .iter()
            .map(|(id, run)| (id.clone(), DslValue::object([("riser_height".to_string(), number(run.riser_height)), ("tread".to_string(), number(run.tread)), ("width".to_string(), number(run.width)), ("riser_count".to_string(), DslValue::int(i64::from(run.riser_count))), ("tread_count".to_string(), DslValue::int(i64::from(run.tread_count)))])))
            .collect(),
    );
    let frames = by_id(snapshot.openings.iter().filter(|(_, opening)| matches!(opening.kind, OpeningKind::Door { .. })).filter_map(|(id, _)| inferred.opening_frames.get(id).map(|frame| (id.clone(), number(frame.width)))).collect());
    let ramps = by_id(inferred.ramp_runs.iter().map(|(id, run)| (id.clone(), DslValue::object([("slope".to_string(), number(run.slope)), ("length".to_string(), number(run.length))]))).collect());
    let zones = by_id(inferred.zone_totals.iter().map(|(id, zone)| (id.clone(), DslValue::object([("net_area".to_string(), number(zone.net_area)), ("resolved".to_string(), DslValue::int(i64::from(zone.resolved)))]))).collect());
    let loop_value = |vertices: &[crate::Vertex]| DslValue::Array(vertices.iter().map(|vertex| semio_framework_value::ToValue::to_value(vertex)).collect());
    let rooms = by_id(
        inferred
            .spaces
            .iter()
            .map(|(id, room)| {
                let row = DslValue::object([
                    ("status".to_string(), DslValue::String(format!("{:?}", room.status))),
                    ("clear_height".to_string(), number(room.clear_height)),
                    ("outline".to_string(), loop_value(&room.outline)),
                    ("holes".to_string(), DslValue::Array(room.holes.iter().map(|hole| loop_value(hole)).collect())),
                ]);
                (id.clone(), row)
            })
            .collect(),
    );
    semio_framework_pack_json::to_json_string(&DslValue::object([("stairs".to_string(), stairs), ("frames".to_string(), frames), ("ramps".to_string(), ramps), ("zones".to_string(), zones), ("rooms".to_string(), rooms)]))
}
//#endregion 🔖️Tables

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
