use super::*;
use crate::editor::bim::entities::components::tests::furnished;
use crate::editor::bim::gestures::session::{Modifiers, Shape};
use crate::editor::bim::gestures::tests::fixture::Rig;
use crate::Point2;
use std::f64::consts::{FRAC_PI_2, PI};

fn created(step: &Step) -> &Component {
    match step.mutations.as_slice() {
        [ModelMutation::CreateComponent(create)] => &create.component,
        other => panic!("one create-component expected, got {other:?}"),
    }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn one_click_places_the_first_family_free_standing_and_the_tool_goes_on_placing() {
    let mut rig = Rig::plan(UTILITY, furnished());
    let first = rig.down(2.0, 2.0);
    let component = created(&first);
    assert_eq!((component.family.as_str(), component.storey.as_str(), component.name.as_str()), ("fam-table", "st-ground", "Table 1"));
    assert_eq!((component.position, component.elevation, component.rotation, component.mirrored, component.host.clone(), component.system), (Point2 { x: 2.0, y: 2.0 }, 0.0, 0.0, false, None, None));
    assert_eq!(created(&rig.down(5.0, 3.0)).name, "Table 2");
    assert_eq!(rig.snapshot.components.len(), 2, "the model holds both instances");
}

#[semio_framework_async_macros::async_test]
async fn moving_shows_the_ghost_of_the_family_and_never_writes() {
    let mut rig = Rig::plan(UTILITY, furnished());
    let before = rig.snapshot.clone();
    assert!(rig.mv(2.0, 2.0).mutations.is_empty());
    assert_eq!(rig.snapshot, before);
    assert!(rig.shows(Shape::Path) && rig.shows(Shape::Label) && rig.shows(Shape::Snap));
    let ring = rig.preview.marks.iter().find(|mark| mark.closed).expect("the footprint ring");
    let xs: Vec<f64> = ring.corners().iter().map(|corner| corner[0]).collect();
    let width = xs.iter().cloned().fold(f64::MIN, f64::max) - xs.iter().cloned().fold(f64::MAX, f64::min);
    assert!(close(width, 1.61), "the ghost of the table is as wide as its solids (the legs overhang the top by 5 mm), got {width}");
    assert!(rig.preview.marks.iter().any(|mark| mark.shape == Shape::Label && mark.text.starts_with("Table")), "the label names the family");
}

#[semio_framework_async_macros::async_test]
async fn a_plumbing_family_clings_to_the_wall_beside_the_pointer_and_ctrl_places_it_free() {
    let snapshot = furnished();
    let thickness = thickness_of(&snapshot, &snapshot.walls["w-south"]);
    let (x, y) = (3.0, thickness / 2.0 + 0.03);
    let mut rig = Rig::plan(UTILITY, snapshot.clone());
    rig.library = vec!["fam-basin".into()];
    let component = created(&rig.down(x, y)).clone();
    assert_eq!((component.family.as_str(), component.host.as_deref(), component.position), ("fam-basin", Some("w-south"), Point2 { x, y }), "the host is the wall and the position stays where the pointer was");
    let mut rig = Rig::plan(UTILITY, snapshot);
    rig.library = vec!["fam-basin".into()];
    let pointer = rig.pointer(x, y, Modifiers { ctrl: true, ..Modifiers::default() });
    let step = rig.send(ToolEvent::Down(pointer));
    assert_eq!(created(&step).host, None, "Ctrl places it free-standing");
    let mut rig = Rig::plan(UTILITY, furnished());
    assert_eq!(created(&rig.down(x, y)).host, None, "a table is no wall-mounted family");
}

#[semio_framework_async_macros::async_test]
async fn the_ghost_of_a_wall_mounted_family_stands_on_the_face_with_its_back_to_the_wall() {
    let snapshot = furnished();
    let thickness = thickness_of(&snapshot, &snapshot.walls["w-south"]);
    let mut rig = Rig::plan(UTILITY, snapshot);
    rig.library = vec!["fam-basin".into()];
    rig.mv(3.0, thickness / 2.0 + 0.03);
    let ring = rig.preview.marks.iter().find(|mark| mark.closed).expect("ring");
    let ys: Vec<f64> = ring.corners().iter().map(|corner| corner[1]).collect();
    let (low, high) = (ys.iter().cloned().fold(f64::MAX, f64::min), ys.iter().cloned().fold(f64::MIN, f64::max));
    assert!(close(low, thickness / 2.0), "the back of the basin lies on the north face of the wall, got {low}");
    assert!(close(high - low, 0.4), "the basin stands 0.4 m out of the wall, got {}", high - low);
    rig.mv(3.0, -(thickness / 2.0 + 0.03));
    let ring = rig.preview.marks.iter().find(|mark| mark.closed).expect("ring");
    let high = ring.corners().iter().map(|corner| corner[1]).fold(f64::MIN, f64::max);
    assert!(close(high, -thickness / 2.0), "on the other side it turns round and its back lies on the south face, got {high}");
}

#[test]
fn the_wall_fit_puts_the_origin_on_the_face_of_the_side_and_the_depth_axis_away_from_the_wall() {
    let axis = Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 6.0, y: 0.0 } };
    let (origin, yaw) = fit(&axis, 0.3, [2.0, 0.5]);
    assert!(close(origin[0], 2.0) && close(origin[1], 0.15) && close(yaw, 0.0), "left of the travel: {origin:?} {yaw}");
    let (origin, yaw) = fit(&axis, 0.3, [2.0, -0.5]);
    assert!(close(origin[0], 2.0) && close(origin[1], -0.15) && close(yaw.abs(), PI), "right of the travel: {origin:?} {yaw}");
    let (origin, yaw) = fit(&axis, 0.3, [2.0, 0.0]);
    assert!(close(origin[1], 0.15) && close(yaw, 0.0), "a point on the axis takes the left face");
    let vertical = Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 0.0, y: 6.0 } };
    let (origin, yaw) = fit(&vertical, 0.2, [0.4, 3.0]);
    assert!(close(origin[0], 0.1) && close(origin[1], 3.0) && close(yaw, -FRAC_PI_2), "right of a north-going wall is east, the depth axis turns to +x: {origin:?} {yaw}");
}

#[test]
fn the_frame_turns_after_the_mirror_and_moves_to_its_origin() {
    let frame = Frame { origin: [1.0, 2.0], yaw: FRAC_PI_2, mirrored: false };
    let [x, y] = frame.world([1.0, 0.5]);
    assert!(close(x, 0.5) && close(y, 3.0), "a quarter turn takes local (1, 0.5) to (-0.5, 1) before the move: {x} {y}");
    let [x, y] = Frame { mirrored: true, ..frame }.world([1.0, 0.5]);
    assert!(close(x, 0.5) && close(y, 1.0), "the mirror flips local x first: {x} {y}");
}

#[semio_framework_async_macros::async_test]
async fn the_keys_turn_mirror_raise_and_cycle_what_the_next_click_writes() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.mv(2.0, 2.0);
    for _ in 0..3 {
        assert!(rig.key(GestureKey::Turn(PI / 12.0)).0);
    }
    assert!(rig.key(GestureKey::Mirror).0 && rig.key(GestureKey::Raise).0 && rig.key(GestureKey::Raise).0);
    let component = created(&rig.down(2.0, 2.0)).clone();
    assert!(close(component.rotation, PI / 4.0) && component.mirrored && close(component.elevation, 0.2), "{component:?}");
    assert!(rig.key(GestureKey::Turn(-FRAC_PI_2)).0);
    assert!(close(created(&rig.down(5.0, 2.0)).rotation, 7.0 * PI / 4.0), "a turn back wraps below zero");
    assert!(rig.key(GestureKey::Next).0);
    assert_eq!(created(&rig.down(6.0, 4.0)).family, "fam-basin", "Tab moves to the next family");
    assert!(rig.key(GestureKey::Previous).0 && rig.key(GestureKey::Previous).0);
    assert_eq!(created(&rig.down(6.5, 4.0)).family, "fam-broken", "and back past the first one wraps round");
    assert!(!rig.key(GestureKey::Back).0, "Backspace is no key of this tool");
}

#[semio_framework_async_macros::async_test]
async fn twenty_four_turns_of_fifteen_degrees_are_back_at_zero_without_noise() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.mv(2.0, 2.0);
    for _ in 0..24 {
        rig.key(GestureKey::Turn(PI / 12.0));
    }
    assert_eq!(created(&rig.down(2.0, 2.0)).rotation, 0.0);
    assert_eq!(normalized(-FRAC_PI_2), 3.0 * FRAC_PI_2);
}

#[semio_framework_async_macros::async_test]
async fn the_system_key_cycles_through_the_nine_systems_and_back_to_a_plain_component() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.mv(2.0, 2.0);
    let mut systems = Vec::new();
    for position in 0..10 {
        assert!(rig.key(GestureKey::System).0);
        systems.push(created(&rig.down(2.0 + position as f64 * 0.5, 2.0)).system);
    }
    let expected: Vec<Option<MepSystem>> = SYSTEMS.iter().copied().map(Some).chain([None]).collect();
    assert_eq!(systems, expected);
}

#[semio_framework_async_macros::async_test]
async fn the_options_can_be_typed_and_a_bad_option_is_refused_without_touching_the_tool() {
    let mut rig = Rig::plan(UTILITY, furnished());
    for line in ["z 0.8", "rot 45", "mirror", "sys water", "fam basin"] {
        let step = rig.typed(line);
        assert!(step.refused.is_none() && step.mutations.is_empty(), "{line}: {step:?}");
    }
    let step = rig.typed("2, 2");
    let component = created(&step);
    assert_eq!((component.family.as_str(), component.elevation, component.mirrored, component.system, component.position), ("fam-basin", 0.8, true, Some(MepSystem::DomesticWater), Point2 { x: 2.0, y: 2.0 }));
    assert!(close(component.rotation, PI / 4.0));
    for line in ["z high", "rot x", "sys steam", "fam nothing", "z"] {
        assert_eq!(rig.typed(line).refused, Some(INVALID), "{line}");
    }
    assert_eq!(rig.typed("z 30cm").refused, None);
    assert_eq!(created(&rig.typed("4, 4")).elevation, 0.3);
}

#[test]
fn a_length_takes_its_unit_and_a_system_word_names_the_system_or_none() {
    assert_eq!((length("0.3"), length("300mm"), length(" 30 cm "), length("2m"), length("-1.5")), (Some(0.3), Some(0.3), Some(0.3), Some(2.0), Some(-1.5)));
    assert_eq!((length("x"), length(""), length("inf")), (None, None, None));
    assert_eq!((system_word("none"), system_word("Gas"), system_word("water"), system_word("steam")), (Some(None), Some(Some(MepSystem::Gas)), Some(Some(MepSystem::DomesticWater)), None));
    assert_eq!((next_system(None), next_system(Some(MepSystem::Lighting))), (Some(MepSystem::Supply), None));
}

#[semio_framework_async_macros::async_test]
async fn the_library_selection_chooses_the_family_and_escape_leaves_the_tool() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.library = vec!["fam-broken".into()];
    assert_eq!(created(&rig.down(2.0, 2.0)).family, "fam-broken");
    rig.library = vec!["fam-hea".into()];
    assert_eq!(created(&rig.down(4.0, 2.0)).family, "fam-broken", "a profile is never placed: the earlier choice stays");
    let escape = rig.escape();
    assert_eq!(escape.arm.as_deref(), Some("select"));
    assert!(escape.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn without_a_family_or_a_storey_the_click_is_refused() {
    let mut bare = furnished();
    bare.families.retain(|_, family| family.category == crate::FamilyCategory::Profile);
    let mut rig = Rig::plan(UTILITY, bare);
    assert_eq!(rig.down(2.0, 2.0).refused, Some(FAMILY_MISSING));
    assert!(rig.snapshot.components.is_empty());
    let mut rig = Rig::on(UTILITY, furnished(), crate::editor::bim::gestures::session::Surface::Plan { storey: "nowhere".into() });
    assert_eq!(rig.down(2.0, 2.0).refused, Some(STOREY_MISSING));
}

#[semio_framework_async_macros::async_test]
async fn a_terminal_shows_its_connector_cross_in_the_ghost() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.mv(2.0, 2.0);
    let plain = rig.preview.marks.iter().filter(|mark| mark.shape == Shape::Path).count();
    rig.key(GestureKey::System);
    let terminal = rig.preview.marks.iter().filter(|mark| mark.shape == Shape::Path).count();
    assert_eq!(terminal, plain + 2, "two strokes make the cross");
}

const GESTURE_CASES: &str = include_str!("../../../../../🧫️fixtures/🛠️gestures/🔣️.json");

fn cases(key: &str) -> Vec<serde_json::Value> {
    let all: serde_json::Value = serde_json::from_str(GESTURE_CASES).expect("the committed cases");
    all[key].as_array().unwrap_or_else(|| panic!("cases {key}")).clone()
}

fn pair(value: &serde_json::Value) -> [f64; 2] {
    [value[0].as_f64().expect("x"), value[1].as_f64().expect("y")]
}

#[test]
fn the_wall_fits_the_oracle_wrote_replay_through_the_fit_to_the_same_origin_and_depth_axis() {
    let all = cases("component_fits");
    assert!(all.len() >= 6);
    for case in all {
        let axis = Axis::Line { start: point2(pair(&case["axis"]["start"])), end: point2(pair(&case["axis"]["end"])) };
        let (origin, yaw) = fit(&axis, case["thickness"].as_f64().expect("thickness"), pair(&case["point"]));
        let (expected, depth) = (pair(&case["origin"]), pair(&case["depth"]));
        assert!(close(origin[0], expected[0]) && close(origin[1], expected[1]), "origin of {case}: {origin:?}");
        assert!(close(-yaw.sin(), depth[0]) && close(yaw.cos(), depth[1]), "the local +y axis of {case} points along {depth:?}, yaw {yaw}");
    }
}

#[test]
fn the_frames_the_oracle_wrote_replay_through_the_frame_to_the_same_plan_points() {
    for case in cases("component_frames") {
        let frame = Frame { origin: pair(&case["origin"]), yaw: case["yaw"].as_f64().expect("yaw").to_radians(), mirrored: case["mirrored"].as_bool().expect("mirrored") };
        for (local, world) in case["local"].as_array().expect("local").iter().zip(case["world"].as_array().expect("world")) {
            let (found, expected) = (frame.world(pair(local)), pair(world));
            assert!(close(found[0], expected[0]) && close(found[1], expected[1]), "{case}: {local} -> {found:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_turn_runs_the_oracle_summed_replay_through_the_keys_to_the_same_rotation() {
    for case in cases("turns") {
        let mut rig = Rig::plan(UTILITY, furnished());
        rig.mv(2.0, 2.0);
        for step in case["steps"].as_array().expect("steps") {
            assert!(rig.key(GestureKey::Turn(step.as_f64().expect("degrees").to_radians())).0);
        }
        let rotation = created(&rig.down(2.0, 2.0)).rotation.to_degrees();
        let expected = case["degrees"].as_f64().expect("degrees");
        assert!(close(rotation, expected) || close((rotation - expected).abs(), 360.0), "{case}: {rotation}");
    }
}
