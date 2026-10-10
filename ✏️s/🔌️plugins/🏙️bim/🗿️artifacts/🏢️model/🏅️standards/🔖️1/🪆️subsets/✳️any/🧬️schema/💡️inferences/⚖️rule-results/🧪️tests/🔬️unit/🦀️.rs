//! 🧪️ The rules: the corridor width of a room, who a rule measures, who violates it and what a member the model cannot measure does.

use super::*;
use crate::{Point2, RuleScope, RuleSeverity};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the committed house decodes")
}

fn rule(kind: RuleKind, limit: f64) -> Rule {
    Rule { name: "Rule".into(), kind, limit, severity: RuleSeverity::Error, scope: RuleScope::all() }
}

fn room(status: SpaceStatus, area: f64, perimeter: f64, clear_height: f64) -> SpaceRoom {
    SpaceRoom { status, outline: Vec::new(), holes: Vec::new(), point: Point2 { x: 0.0, y: 0.0 }, area, perimeter, net_floor_area: area, floor_z: 0.0, clear_height, volume: area * clear_height, ceiling_slab: String::new(), ceiling: String::new(), bounding_walls: Vec::new() }
}

fn member(id: &str, storey: Option<&str>) -> Member {
    Member { id: id.into(), storey: storey.map(str::to_string) }
}

#[test]
fn the_width_of_a_room_is_the_short_side_of_the_rectangle_with_its_area_and_perimeter() {
    assert!((equivalent_width(12.0, 14.0) - 3.0).abs() < 1e-12, "a 4 by 3 room");
    assert!((equivalent_width(1.2 * 10.0, 2.0 * 11.2) - 1.2).abs() < 1e-12, "a 10 by 1.2 corridor");
    assert!((equivalent_width(9.0, 12.0) - 3.0).abs() < 1e-12, "a square");
    assert!((equivalent_width(10.0, 4.0 * 10.0_f64.sqrt()) - 10.0_f64.sqrt()).abs() < 1e-12);
    let round = equivalent_width(std::f64::consts::PI, 2.0 * std::f64::consts::PI);
    assert!((round - std::f64::consts::PI / 2.0).abs() < 1e-12, "a rounder room than any rectangle gets a quarter of its perimeter");
    assert_eq!(equivalent_width(0.0, 0.0), 0.0);
}

#[test]
fn a_rule_measures_the_members_its_kind_and_scope_name() {
    let model = house();
    let stairs: Vec<String> = members(&model, &rule(RuleKind::MaxRiser, 0.19)).into_iter().map(|member| member.id).collect();
    assert_eq!(stairs, model.stairs.keys().cloned().collect::<Vec<_>>());
    assert_eq!(members(&model, &rule(RuleKind::MinTread, 0.26)), members(&model, &rule(RuleKind::MaxRiser, 0.19)));
    let doors = members(&model, &rule(RuleKind::MinDoorWidth, 0.9));
    assert_eq!(doors.len(), model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Door { .. })).count());
    assert!(doors.windows(2).all(|pair| pair[0].id < pair[1].id));
    assert_eq!(members(&model, &rule(RuleKind::MinClearHeight, 2.4)).len(), model.spaces.len());
    assert!(members(&model, &rule(RuleKind::MaxCompartmentArea, 400.0)).iter().all(|member| member.storey.is_none()));
    let nowhere = Rule { scope: RuleScope { ids: vec!["st-nowhere".into()], ..RuleScope::all() }, ..rule(RuleKind::MinDoorWidth, 0.9) };
    assert!(members(&model, &nowhere).is_empty());
}

#[test]
fn the_scope_filter_names_the_usage_of_the_spaces_without_regard_to_case() {
    let mut model = house();
    let mut ids = model.spaces.keys().cloned();
    let (corridor, room_id) = (ids.next().expect("a space"), ids.next().expect("another space"));
    model.spaces.get_mut(&corridor).expect("space").usage = "Corridor".into();
    model.spaces.get_mut(&room_id).expect("space").usage = "Office".into();
    let rule = Rule { scope: RuleScope { filter: " corridor ".into(), ..RuleScope::all() }, ..rule(RuleKind::MinCorridorWidth, 1.2) };
    assert_eq!(members(&model, &rule).into_iter().map(|member| member.id).collect::<Vec<_>>(), [corridor]);
}

#[test]
fn a_minimum_is_broken_below_the_limit_and_a_maximum_above_it() {
    let mut runs = StairRun { riser_count: 16, tread_count: 15, riser_height: 0.2, tread: 0.25, width: 1.0, ..StairRun::default() };
    let mut inputs = Inputs::default();
    runs.riser_height = 0.2;
    inputs.runs.insert("stair-1", &runs);
    let stair = [member("stair-1", Some("st-ground"))];
    let riser = result_of(&rule(RuleKind::MaxRiser, 0.19), &stair, &inputs);
    assert_eq!((riser.checked, riser.violations.len()), (1, 1));
    assert_eq!(riser.violations[0], RuleFinding { element: "stair-1".into(), measured: 0.2, limit: 0.19, storey: "st-ground".into() });
    assert_eq!(result_of(&rule(RuleKind::MaxRiser, 0.2), &stair, &inputs).violations.len(), 0, "the limit itself is allowed");
    assert_eq!(result_of(&rule(RuleKind::MinTread, 0.26), &stair, &inputs).violations.len(), 1);
    assert_eq!(result_of(&rule(RuleKind::MinTread, 0.25), &stair, &inputs).violations.len(), 0);
    assert_eq!(result_of(&rule(RuleKind::MinStairWidth, 1.1), &stair, &inputs).violations.len(), 1);
    assert_eq!(result_of(&rule(RuleKind::MinStairWidth, 0.8), &stair, &inputs).passed(), 1);
}

#[test]
fn doors_ramps_zones_and_rooms_are_measured_on_their_own_values() {
    let frame = OpeningFrame { width: 0.8, ..OpeningFrame::default() };
    let ramp = RampRun { length: 6.0, slope: 0.09, ..RampRun::default() };
    let zone = ZoneTotals { resolved: 3, net_area: 450.0, ..ZoneTotals::default() };
    let rooms: StoreyRooms = [("s-1".to_string(), room(SpaceStatus::Inferred, 12.0, 14.0, 2.3)), ("s-2".to_string(), room(SpaceStatus::NotEnclosed, 0.0, 0.0, 0.0))].into();
    let mut inputs = Inputs::default();
    inputs.frames.insert("d-1", &frame);
    inputs.ramp_runs.insert("r-1", &ramp);
    inputs.zones.insert("z-1", &zone);
    inputs.rooms.insert("st-ground", &rooms);
    assert_eq!(result_of(&rule(RuleKind::MinDoorWidth, 0.9), &[member("d-1", Some("st-ground"))], &inputs).violations.len(), 1);
    assert_eq!(result_of(&rule(RuleKind::MaxRampSlope, 0.0833), &[member("r-1", Some("st-ground"))], &inputs).violations.len(), 1);
    assert_eq!(result_of(&rule(RuleKind::MaxRampSlope, 0.1), &[member("r-1", Some("st-ground"))], &inputs).violations.len(), 0);
    assert_eq!(result_of(&rule(RuleKind::MaxCompartmentArea, 400.0), &[member("z-1", None)], &inputs).violations.len(), 1);
    let spaces = [member("s-1", Some("st-ground")), member("s-2", Some("st-ground"))];
    let height = result_of(&rule(RuleKind::MinClearHeight, 2.4), &spaces, &inputs);
    assert_eq!((height.checked, height.violations.len()), (1, 1), "an unresolved room is not measured");
    assert!((height.violations[0].measured - 2.3).abs() < 1e-12);
    let width = result_of(&rule(RuleKind::MinCorridorWidth, 3.5), &spaces, &inputs);
    assert!((width.violations[0].measured - 3.0).abs() < 1e-9 && width.violations[0].element == "s-1");
}

#[test]
fn a_member_the_model_cannot_measure_is_not_counted() {
    let flat = StairRun::default();
    let mut inputs = Inputs::default();
    inputs.runs.insert("stair-1", &flat);
    let result = result_of(&rule(RuleKind::MaxRiser, 0.19), &[member("stair-1", None), member("stair-9", None)], &inputs);
    assert_eq!(result, RuleResult::default());
}

#[test]
fn the_dependency_names_the_rule_and_its_members_and_nothing_else_of_the_model() {
    let mut model = house();
    model.rules.insert("r-1".into(), rule(RuleKind::MinDoorWidth, 0.9));
    let before = dependency(&model, "r-1");
    model.project.name = "Renamed".into();
    assert_eq!(dependency(&model, "r-1"), before, "a project rename does not touch a rule");
    model.rules.get_mut("r-1").expect("rule").limit = 0.8;
    assert_ne!(dependency(&model, "r-1"), before);
    assert_eq!(dependency(&model, "r-9"), DslValue::Null);
}
