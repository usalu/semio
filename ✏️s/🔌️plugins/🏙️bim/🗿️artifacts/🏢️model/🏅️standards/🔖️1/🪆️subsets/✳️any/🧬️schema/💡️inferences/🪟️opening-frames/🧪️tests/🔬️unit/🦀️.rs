use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelNode};
use crate::{Entry, ModelDiff, OpeningPatch, StoreyPatch, WallPatch};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const PLACED: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪟️opening-frames/🏡️placed/📸️snapshot/🔣️.json");
const INVALID: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪟️opening-frames/⚠️invalid/📸️snapshot/🔣️.json");
const PLACED_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪟️opening-frames/🏡️placed/💡️inference/🪟️opening-frames/🔣️.json");
const INVALID_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪟️opening-frames/⚠️invalid/💡️inference/🪟️opening-frames/🔣️.json");

fn placed() -> ModelSnapshot {
    from_json_str(PLACED, JsonMemberPolicy::Reject).expect("placed decodes")
}

fn invalid() -> ModelSnapshot {
    from_json_str(INVALID, JsonMemberPolicy::Reject).expect("invalid decodes")
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

fn vec3(value: &Vec3, x: f64, y: f64, z: f64) -> bool {
    close(value.x, x) && close(value.y, y) && close(value.z, z)
}

fn edited(snapshot: &ModelSnapshot, diff: &ModelDiff) -> ModelSnapshot {
    protocol::apply_diff(diff, snapshot).expect("the diff applies")
}

fn raise_storey(snapshot: &ModelSnapshot, id: &str, height: f64) -> ModelSnapshot {
    edited(snapshot, &ModelDiff::storeys(id, Entry::Patched(StoreyPatch { height: Some(height), ..Default::default() })))
}

fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 { x: a.y * b.z - a.z * b.y, y: a.z * b.x - a.x * b.z, z: a.x * b.y - a.y * b.x }
}

#[semio_framework_async_macros::async_test]
async fn a_window_on_a_line_host_resolves_type_size_sill_position_and_frame() {
    let frames = compute_opening_frames(&placed());
    let window = &frames["o-win-south"];
    assert!(close(window.width, 1.2) && close(window.height, 1.4) && close(window.sill, 0.9), "size and sill come from the window type");
    assert!(close(window.cut.s_min, 1.4) && close(window.cut.s_max, 2.6) && close(window.cut.z_min, 0.9) && close(window.cut.z_max, 2.3));
    assert!(close(window.reveal_depth, 0.3) && close(window.host_length, 8.0) && close(window.host_height, 3.0), "the reveal is the host thickness");
    assert!(close(window.point.x, 2.0) && close(window.point.y, 0.0));
    assert!(vec3(&window.local.origin, 2.0, 0.0, 0.9) && vec3(&window.local.x_axis, 1.0, 0.0, 0.0) && vec3(&window.local.y_axis, 0.0, 1.0, 0.0) && vec3(&window.local.z_axis, 0.0, 0.0, 1.0));
    assert!(window.valid && window.issues.is_empty() && window.overlaps.is_empty() && window.hand.is_none());
    let (sin, cos) = 0.5f64.sin_cos();
    assert!(vec3(&window.world.origin, 10.0 + 2.0 * cos, 5.0 + 2.0 * sin, 0.9 + 102.5), "building origin, rotation and the site plus building datum");
    assert!(vec3(&window.world.x_axis, cos, sin, 0.0) && vec3(&window.world.y_axis, -sin, cos, 0.0) && vec3(&window.world.z_axis, 0.0, 0.0, 1.0));
    assert_eq!(window.plan.len(), 1);
    assert_eq!(window.plan[0].role, PlanRole::Glazing);
    let PlanShape::Line { from, to } = window.plan[0].shape else { panic!("the glazing is a line") };
    assert!(close(from.x, 1.4) && close(to.x, 2.6) && close(from.y, 0.0) && close(to.y, 0.0));
}

#[semio_framework_async_macros::async_test]
async fn explicit_size_overrides_the_type_and_the_window_sill_adds_to_the_type_sill() {
    let wide = &compute_opening_frames(&placed())["o-win-south-wide"];
    assert!(close(wide.width, 2.0) && close(wide.height, 1.0) && close(wide.sill, 1.1));
    assert!(close(wide.cut.s_min, 4.5) && close(wide.cut.s_max, 6.5) && close(wide.cut.z_min, 1.1) && close(wide.cut.z_max, 2.1));
}

#[semio_framework_async_macros::async_test]
async fn flipping_the_facing_turns_the_frame_half_a_turn_and_stays_right_handed() {
    let frames = compute_opening_frames(&placed());
    let (plain, flipped) = (&frames["o-win-south"], &frames["o-win-south-wide"]);
    assert!(vec3(&flipped.local.x_axis, -1.0, 0.0, 0.0) && vec3(&flipped.local.y_axis, 0.0, -1.0, 0.0));
    assert!(close(flipped.local.origin.y, plain.local.origin.y), "the origin stays on the host location line");
    for frame in frames.values().filter(|frame| frame.valid) {
        let normal = cross(&frame.local.x_axis, &frame.local.y_axis);
        assert!(vec3(&normal, 0.0, 0.0, 1.0), "x cross y is up");
        let world = cross(&frame.world.x_axis, &frame.world.y_axis);
        assert!(vec3(&world, 0.0, 0.0, 1.0));
    }
}

#[semio_framework_async_macros::async_test]
async fn a_door_sits_on_the_floor_and_swings_with_its_hand() {
    let frames = compute_opening_frames(&placed());
    let door = &frames["o-door-south"];
    assert!(close(door.sill, 0.0) && close(door.cut.z_min, 0.0) && close(door.cut.z_max, 2.1) && close(door.cut.s_min, 6.55) && close(door.cut.s_max, 7.45));
    assert_eq!(door.hand, Some(Swing::Left));
    assert_eq!(door.plan.len(), 2);
    let PlanShape::Line { from, to } = door.plan[0].shape else { panic!("the open leaf is a line") };
    assert!(close(from.x, 7.45) && close(from.y, 0.15) && close(to.x, 7.45) && close(to.y, 1.05), "a Left door hinges at +x on the +y face and opens towards +y");
    let PlanShape::Arc { centre, radius, start_angle, sweep } = door.plan[1].shape else { panic!("the swing is an arc") };
    assert!(close(centre.x, 7.45) && close(centre.y, 0.15) && close(radius, 0.9) && close(start_angle.abs(), std::f64::consts::PI) && close(sweep, -FRAC_PI_2));
    assert_eq!((door.plan[0].role, door.plan[1].role), (PlanRole::Leaf, PlanRole::Swing));
    let flipped_hand = &frames["o-door-east"];
    assert_eq!(flipped_hand.hand, Some(Swing::Left), "a Right door type with flip_hand swings Left");
    assert_eq!(flipped_hand.plan.len(), 4, "a double door has two leaves");
    let ends: Vec<(f64, f64)> = flipped_hand.plan.iter().filter_map(|stroke| if let PlanShape::Line { to, .. } = stroke.shape { Some((to.x, to.y)) } else { None }).collect();
    assert!(close(ends[0].0, 6.95) && close(ends[0].1, 3.9) && close(ends[1].0, 6.95) && close(ends[1].1, 2.1), "the east wall left normal is -x and its body lies on that side, so the leaves hinge on the face 0.15 m away and open towards -x");
    let world = &flipped_hand.world;
    let mapped = world.affine().apply_point([1.0, 2.0, 3.0]);
    assert!(close(mapped[0], world.origin.x + world.x_axis.x + 2.0 * world.y_axis.x) && close(mapped[1], world.origin.y + world.x_axis.y + 2.0 * world.y_axis.y) && close(mapped[2], world.origin.z + 3.0), "the frame affine maps opening-local points into the world");
}

#[semio_framework_async_macros::async_test]
async fn an_arc_host_places_the_opening_on_the_circle_with_the_tangent_frame() {
    let frames = compute_opening_frames(&placed());
    let (centre, radius, start) = ((4.0f64, 3.0f64), 5.0f64, 3.0f64.atan2(4.0));
    let window = &frames["o-win-arc"];
    let angle = start + 4.0 / radius;
    assert!(close(window.point.x, centre.0 + radius * angle.cos()) && close(window.point.y, centre.1 + radius * angle.sin()));
    assert!(close(window.local.x_axis.x, -angle.sin()) && close(window.local.x_axis.y, angle.cos()), "x is the tangent of travel");
    let towards_centre = (centre.0 - window.point.x) * window.local.y_axis.x + (centre.1 - window.point.y) * window.local.y_axis.y;
    assert!(towards_centre > 0.0, "the left normal of a counter-clockwise arc points to its centre");
    assert!(close(window.host_length, 5.0 * 4.0 * 0.5f64.atan()), "the host length is the arc length");
    assert!(close(window.cut.s_min, 3.4) && close(window.cut.s_max, 4.6), "the cut is an arc-length interval");
    let door = &frames["o-door-arc"];
    assert!(door.valid, "{:?}", door.issues);
    let angle = start + 7.5 / radius;
    assert!(close(door.local.x_axis.x, angle.sin()) && close(door.local.x_axis.y, -angle.cos()), "a flipped facing reverses the tangent");
    assert_eq!(door.hand, Some(Swing::Right), "a Left door type with flip_hand swings Right");
}

#[semio_framework_async_macros::async_test]
async fn walls_above_stand_on_the_storey_stack_and_a_curtain_wall_host_uses_its_mullion_depth() {
    let frames = compute_opening_frames(&placed());
    let first = &frames["o-void-first"];
    assert!(close(first.local.origin.z, 3.0 + 0.5) && close(first.host_height, 2.8) && close(first.reveal_depth, 0.15));
    assert!(first.plan.is_empty(), "a void draws nothing");
    let curtain = &frames["o-void-curtain"];
    assert!(close(curtain.reveal_depth, 0.12) && close(curtain.host_length, 6.0) && close(curtain.host_height, 3.0) && curtain.valid);
    assert!(close(curtain.point.x, 3.0) && close(curtain.point.y, 10.0));
    let east = &frames["o-door-east"];
    assert!(close(east.local.origin.z, 0.1) && close(east.host_height, 2.4), "base offset lifts the host base");
}

#[semio_framework_async_macros::async_test]
async fn every_placement_problem_is_reported_by_name() {
    let frames = compute_opening_frames(&invalid());
    let issues = |id: &str| frames[id].issues.clone();
    assert_eq!(issues("o-outside"), vec![OpeningIssue::OutsideHostExtent]);
    assert_eq!(issues("o-above"), vec![OpeningIssue::AboveHostTop]);
    assert_eq!(issues("o-below"), vec![OpeningIssue::BelowHostBase]);
    assert_eq!(issues("o-orphan"), vec![OpeningIssue::HostMissing]);
    assert_eq!(issues("o-no-type"), vec![OpeningIssue::TypeMissing, OpeningIssue::NonPositiveSize]);
    assert_eq!(issues("o-zero"), vec![OpeningIssue::NonPositiveSize]);
    assert_eq!(issues("o-overlap-a"), vec![OpeningIssue::OverlapsSibling]);
    assert_eq!(frames["o-overlap-a"].overlaps, vec!["o-overlap-b".to_string()]);
    assert_eq!(frames["o-overlap-b"].overlaps, vec!["o-overlap-a".to_string()]);
    assert!(frames.values().all(|frame| !frame.valid), "every case of the invalid model is invalid");
    assert!(frames["o-orphan"].plan.is_empty() && frames["o-orphan"].local == Frame::default(), "an orphan has no placement");
}

#[semio_framework_async_macros::async_test]
async fn a_lower_storey_height_makes_a_window_cross_the_wall_top_and_a_higher_one_heals_it() {
    let snapshot = placed();
    assert!(compute_opening_frames(&snapshot)["o-win-south"].valid);
    let lowered = compute_opening_frames(&raise_storey(&snapshot, "st-ground", 2.2));
    assert!(close(lowered["o-win-south"].host_height, 2.2));
    assert_eq!(lowered["o-win-south"].issues, vec![OpeningIssue::AboveHostTop], "the window top 2.3 m exceeds the 2.2 m wall");
    assert!(lowered["o-door-south"].valid, "the 2.1 m door still fits");
    let healed = compute_opening_frames(&raise_storey(&snapshot, "st-ground", 3.4));
    assert!(healed["o-win-south"].valid && close(healed["o-win-south"].host_height, 3.4));
    let before = compute_opening_frames(&snapshot);
    assert!(close(healed["o-void-first"].local.origin.z - before["o-void-first"].local.origin.z, 0.4), "openings on the storey above are lifted with it");
    assert_eq!(healed["o-win-south"].cut, before["o-win-south"].cut, "the cut in the host development does not move");
}

#[semio_framework_async_macros::async_test]
async fn moving_and_curving_the_host_axis_re_derives_every_frame_along_it() {
    let snapshot = placed();
    let axis = |axis: Axis| ModelDiff::walls("w-south", Entry::Patched(WallPatch { axis: Some(axis), ..Default::default() }));
    let moved = compute_opening_frames(&edited(&snapshot, &axis(Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 0.0, y: 8.0 } })));
    assert!(close(moved["o-win-south"].point.x, 0.0) && close(moved["o-win-south"].point.y, 2.0), "the window follows the rotated axis at the same arc length");
    assert!(vec3(&moved["o-win-south"].local.x_axis, 0.0, 1.0, 0.0) && vec3(&moved["o-win-south"].local.y_axis, -1.0, 0.0, 0.0));
    assert_eq!(moved["o-win-south"].cut, compute_opening_frames(&snapshot)["o-win-south"].cut);
    let shortened = compute_opening_frames(&edited(&snapshot, &axis(Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 6.0, y: 0.0 } })));
    assert_eq!(shortened["o-door-south"].issues, vec![OpeningIssue::OutsideHostExtent], "a door beyond the new end leaves the host extent");
    let curved = compute_opening_frames(&edited(&snapshot, &axis(Axis::Arc { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 8.0, y: 0.0 }, bulge: 0.5 })));
    let window = &curved["o-win-south"];
    assert!(close(window.host_length, 5.0 * 4.0 * 0.5f64.atan()));
    assert!(!close(window.local.x_axis.y, 0.0), "on a curved host the tangent turns");
    assert!(close(window.local.x_axis.x.hypot(window.local.x_axis.y), 1.0));
}

#[semio_framework_async_macros::async_test]
async fn a_moved_opening_slides_along_its_host_and_overlap_follows_the_offsets() {
    let snapshot = placed();
    let slide = |id: &str, offset: f64| ModelDiff::openings(id, Entry::Patched(OpeningPatch { offset: Some(offset), ..Default::default() }));
    let slid = compute_opening_frames(&edited(&snapshot, &slide("o-win-south", 5.0)));
    assert!(close(slid["o-win-south"].point.x, 5.0) && close(slid["o-win-south"].cut.s_min, 4.4));
    assert_eq!(slid["o-win-south"].overlaps, vec!["o-win-south-wide".to_string()], "the windows now share s and z");
    assert_eq!(slid["o-win-south-wide"].overlaps, vec!["o-win-south".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn the_host_extent_agrees_with_the_wall_layout_of_the_same_wall() {
    let snapshot = placed();
    let layouts = crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout(&snapshot);
    let frames = compute_opening_frames(&snapshot);
    for (opening, wall) in [("o-win-south", "w-south"), ("o-door-east", "w-east"), ("o-win-arc", "w-arc"), ("o-void-first", "w-first")] {
        assert!(close(frames[opening].host_height, layouts[wall].height), "{wall} height");
        assert!(close(frames[opening].host_length, layouts[wall].length), "{wall} length");
        assert!(close(frames[opening].reveal_depth, layouts[wall].thickness), "{wall} thickness");
        assert!(close(frames[opening].local.origin.z - frames[opening].sill, layouts[wall].base_z), "{wall} base");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_field_is_deterministic_and_empty_for_an_empty_model() {
    let snapshot = placed();
    assert_eq!(compute_opening_frames(&snapshot), compute_opening_frames(&snapshot));
    assert!(compute_opening_frames(&ModelSnapshot::default()).is_empty());
    let frames = compute_opening_frames(&snapshot);
    assert_eq!(frames.keys().collect::<Vec<_>>(), snapshot.openings.keys().collect::<Vec<_>>(), "one frame per opening");
}

#[semio_framework_async_macros::async_test]
async fn the_faces_follow_the_location_line_and_the_facing() {
    let frames = compute_opening_frames(&placed());
    let faces = |id: &str| (frames[id].face_front, frames[id].face_back);
    assert!(close(faces("o-door-east").0, 0.15) && close(faces("o-door-east").1, 0.0), "an Exterior wall extends to the left of its axis only");
    assert!(close(faces("o-win-south").0, 0.15) && close(faces("o-win-south").1, 0.15), "a Center wall extends half its thickness to each side");
    assert!(close(frames["o-door-east"].reveal_depth, 0.15) && frames.values().filter(|frame| frame.valid).all(|frame| close(frame.face_front + frame.face_back, frame.reveal_depth)));
    let mirrored = edited(&placed(), &ModelDiff::openings("o-door-east", Entry::Patched(OpeningPatch { flip_facing: Some(true), ..Default::default() })));
    let flipped = &compute_opening_frames(&mirrored)["o-door-east"];
    assert!(close(flipped.face_front, 0.0) && close(flipped.face_back, 0.15), "flipping the facing swaps the sides");
    assert!(vec3(&flipped.local.y_axis, 1.0, 0.0, 0.0));
}

fn agrees(path: &str, expected: &serde_json::Value, actual: &serde_json::Value) {
    match (expected, actual) {
        (serde_json::Value::Object(want), serde_json::Value::Object(got)) => {
            for (name, value) in want {
                agrees(&format!("{path}.{name}"), value, got.get(name).unwrap_or_else(|| panic!("{path}.{name} is missing from the subject")));
            }
        }
        (serde_json::Value::Array(want), serde_json::Value::Array(got)) => {
            assert_eq!(want.len(), got.len(), "{path}: same length");
            want.iter().zip(got).enumerate().for_each(|(index, (left, right))| agrees(&format!("{path}[{index}]"), left, right));
        }
        (serde_json::Value::Number(want), serde_json::Value::Number(got)) => {
            let (want, got) = (want.as_f64().expect("number"), got.as_f64().expect("number"));
            assert!((want - got).abs() <= 1e-9 * want.abs().max(1.0), "{path}: oracle {want}, subject {got}");
        }
        _ => assert_eq!(expected, actual, "{path}"),
    }
}

fn reproduces(snapshot: &ModelSnapshot, table: &str) {
    let expected: serde_json::Value = serde_json::from_str(table).expect("JSON table");
    let produced: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&compute_opening_frames(snapshot))).expect("JSON table");
    assert_eq!(expected.as_object().expect("table").keys().collect::<Vec<_>>(), produced.as_object().expect("table").keys().collect::<Vec<_>>(), "same ids");
    agrees("frames", &expected, &produced);
}

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_third_party_oracle_tables() {
    reproduces(&placed(), PLACED_TABLE);
    reproduces(&invalid(), INVALID_TABLE);
}

#[semio_framework_async_macros::async_test]
async fn an_opening_depends_on_its_host_and_the_cuts_of_its_siblings_and_a_host_on_its_layout() {
    let snapshot = placed();
    let steps = plan::build(&snapshot, kinds::closure(kinds::FRAMES));
    let parents = |key: ModelNode| steps.iter().find(|step| step.key == key).map(|step| step.parents.clone()).expect("planned");
    let frame = parents(ModelNode::OpeningFrame("o-win-south".into()));
    assert_eq!(frame[..2], [ModelNode::Host("w-south".into()), ModelNode::Cut("o-win-south".into())]);
    let siblings: Vec<&str> = snapshot.openings.iter().filter(|(id, opening)| opening.host == "w-south" && id.as_str() != "o-win-south").map(|(id, _)| id.as_str()).collect();
    assert_eq!(frame[2..].iter().map(|node| if let ModelNode::Cut(id) = node { id.as_str() } else { "?" }).collect::<Vec<_>>(), siblings, "every sibling on the host is a parent, in id order");
    assert_eq!(parents(ModelNode::Host("w-south".into())), vec![ModelNode::Storey("st-ground".into()), ModelNode::WallLayout("w-south".into())]);
    assert_eq!(parents(ModelNode::Host("cw-1".into())), vec![ModelNode::Storey("st-ground".into()), ModelNode::CurtainLayout("cw-1".into())]);
    let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).expect("planned");
    for step in &steps {
        for parent in &step.parents {
            assert!(position(parent) < position(&step.key), "parents come first");
        }
    }
    let orphan = plan::build(&invalid(), kinds::closure(kinds::FRAMES));
    assert!(!orphan.iter().find(|step| step.key == ModelNode::OpeningFrame("o-orphan".into())).expect("planned").parents.iter().any(|parent| matches!(parent, ModelNode::Host(_))), "an orphan has no host parent");
    assert!(orphan.iter().all(|step| step.key != ModelNode::Host("w-ghost".into())));
}

