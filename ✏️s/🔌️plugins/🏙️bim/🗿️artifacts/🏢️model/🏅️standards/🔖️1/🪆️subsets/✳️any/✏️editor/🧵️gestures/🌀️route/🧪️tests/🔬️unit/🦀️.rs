use super::*;
use crate::editor::bim::entities::components::tests::furnished;
use crate::editor::bim::gestures::session::Shape;
use crate::editor::bim::gestures::tests::fixture::Rig;

fn created(step: &Step) -> &MepElement {
    match step.mutations.as_slice() {
        [ModelMutation::CreateMepElement(create)] => &create.mep,
        other => panic!("one create-mep-element expected, got {other:?}"),
    }
}

fn at(x: f64, y: f64, z: f64) -> Point3 {
    Point3 { x, y, z }
}

#[semio_framework_async_macros::async_test]
async fn clicks_collect_the_vertices_and_enter_writes_one_element_at_the_current_elevation() {
    let mut rig = Rig::plan(UTILITY, furnished());
    assert!(rig.down(2.0, 2.0).mutations.is_empty() && rig.down(6.0, 2.0).mutations.is_empty() && rig.down(6.0, 4.0).mutations.is_empty());
    let step = rig.finish();
    let element = created(&step);
    assert_eq!((element.storey.as_str(), element.system, element.shape.clone(), element.name.as_str()), ("st-ground", MepSystem::Supply, MepShape::Duct { width: 0.3, height: 0.2 }, "MEP element 1"));
    assert_eq!(element.path, vec![at(2.0, 2.0, 2.5), at(6.0, 2.0, 2.5), at(6.0, 4.0, 2.5)]);
    assert_eq!(rig.snapshot.mep_elements.len(), 1);
    assert!(rig.finish().mutations.is_empty(), "the route is over: Enter again writes nothing");
}

#[semio_framework_async_macros::async_test]
async fn a_double_click_ends_the_route_and_a_single_vertex_writes_nothing() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.down(2.0, 2.0);
    assert!(rig.finish().mutations.is_empty(), "one vertex is no element");
    rig.down(2.0, 2.0);
    rig.down(5.0, 2.0);
    rig.down(5.0, 2.0);
    assert_eq!(created(&rig.double(5.0, 2.0)).path, vec![at(2.0, 2.0, 2.5), at(5.0, 2.0, 2.5)], "the repeated click of a double click adds no vertex");
}

#[semio_framework_async_macros::async_test]
async fn a_click_at_the_same_plan_point_with_another_elevation_is_a_riser() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.down(2.0, 2.0);
    rig.down(5.0, 2.0);
    assert!(rig.typed("z 3.1").mutations.is_empty());
    rig.down(5.0, 2.0);
    let step = rig.finish();
    assert_eq!(created(&step).path, vec![at(2.0, 2.0, 2.5), at(5.0, 2.0, 2.5), at(5.0, 2.0, 3.1)]);
}

#[semio_framework_async_macros::async_test]
async fn backspace_takes_the_last_vertex_back_and_escape_drops_the_route_before_it_leaves_the_tool() {
    let mut rig = Rig::plan(UTILITY, furnished());
    assert!(!rig.key(GestureKey::Back).0, "with no vertex Backspace keeps its other meaning");
    rig.down(2.0, 2.0);
    rig.down(5.0, 2.0);
    rig.down(5.0, 5.0);
    assert!(rig.key(GestureKey::Back).0);
    assert_eq!(created(&rig.finish()).path, vec![at(2.0, 2.0, 2.5), at(5.0, 2.0, 2.5)]);
    rig.down(1.0, 1.0);
    assert_eq!(rig.escape().arm, None, "the first Escape drops the route");
    assert!(rig.finish().mutations.is_empty());
    assert_eq!(rig.escape().arm.as_deref(), Some("select"), "the second leaves the tool");
}

#[semio_framework_async_macros::async_test]
async fn the_keys_change_the_system_the_section_kind_and_the_elevation_of_the_vertices_to_come() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.down(1.0, 1.0);
    assert!(rig.key(GestureKey::Raise).0 && rig.key(GestureKey::Raise).0 && rig.key(GestureKey::Lower).0);
    assert!(rig.key(GestureKey::System).0 && rig.key(GestureKey::System).0 && rig.key(GestureKey::System).0);
    rig.down(4.0, 1.0);
    let element = created(&rig.finish()).clone();
    assert_eq!((element.system, element.shape.clone()), (MepSystem::DomesticWater, MepShape::Pipe { diameter: 0.1 }), "a water system brings a pipe until a section is chosen");
    assert_eq!(element.path, vec![at(1.0, 1.0, 2.5), at(4.0, 1.0, 2.6)], "the first vertex was set before the keys");
    rig.down(1.0, 2.0);
    assert!(rig.key(GestureKey::Next).0);
    rig.down(4.0, 2.0);
    assert!(matches!(created(&rig.finish()).shape, MepShape::Tray { .. }), "Tab turns the pipe into the next kind, a tray");
    rig.down(1.0, 3.0);
    assert!(rig.key(GestureKey::Previous).0 && rig.key(GestureKey::Previous).0);
    rig.down(4.0, 3.0);
    assert!(matches!(created(&rig.finish()).shape, MepShape::Duct { .. }));
    assert!(!rig.key(GestureKey::Turn(0.1)).0 && !rig.key(GestureKey::Mirror).0, "a route has nothing to turn or mirror");
}

#[semio_framework_async_macros::async_test]
async fn the_section_and_the_system_can_be_typed_with_their_units() {
    let mut rig = Rig::plan(UTILITY, furnished());
    for line in ["duct 400mm x 250mm", "sys exhaust", "z 3"] {
        assert!(rig.typed(line).refused.is_none(), "{line}");
    }
    rig.down(0.0, 0.0);
    rig.down(2.0, 0.0);
    let element = created(&rig.finish()).clone();
    assert_eq!((element.system, element.shape), (MepSystem::Exhaust, MepShape::Duct { width: 0.4, height: 0.25 }));
    assert_eq!(element.path[0].z, 3.0);
    rig.typed("sys gas");
    rig.down(0.0, 1.0);
    rig.down(2.0, 1.0);
    assert_eq!(created(&rig.finish()).shape, MepShape::Duct { width: 0.4, height: 0.25 }, "a chosen section stays when the system changes");
    assert!(rig.typed("pipe 25mm").refused.is_none() && rig.typed("tray 0.3 x 0.06").refused.is_none());
    for line in ["pipe", "pipe 0", "duct 0.3", "duct a x b", "sys steam", "z high", "tray 1 x -1"] {
        assert_eq!(rig.typed(line).refused, Some(INVALID), "{line}");
    }
    let step = rig.typed("1, 1");
    assert!(step.refused.is_none(), "a point line still clicks");
}

#[semio_framework_async_macros::async_test]
async fn the_preview_shows_the_run_the_rubber_band_the_options_and_the_length() {
    let mut rig = Rig::plan(UTILITY, furnished());
    rig.mv(1.0, 1.0);
    assert!(rig.shows(Shape::Label) && rig.shows(Shape::Snap) && !rig.shows(Shape::Path));
    rig.down(1.0, 1.0);
    rig.down(4.0, 1.0);
    rig.mv(4.0, 5.0);
    assert!(rig.shows(Shape::Path) && rig.shows(Shape::Dot));
    let labels: Vec<&str> = rig.preview.marks.iter().filter(|mark| mark.shape == Shape::Label).map(|mark| mark.text.as_str()).collect();
    assert!(labels.contains(&"3.00 m"), "the length of the run so far: {labels:?}");
    assert!(labels.iter().any(|text| text.starts_with("duct 300 × 200") && text.contains("Supply air") && text.ends_with("z 2.50 m")), "{labels:?}");
}

#[semio_framework_async_macros::async_test]
async fn moving_never_writes_and_a_lost_pointer_drops_the_route() {
    let mut rig = Rig::plan(UTILITY, furnished());
    let before = rig.snapshot.clone();
    rig.mv(1.0, 1.0);
    rig.down(1.0, 1.0);
    rig.mv(3.0, 3.0);
    assert_eq!(rig.snapshot, before);
    rig.send(ToolEvent::Lost);
    assert!(rig.finish().mutations.is_empty());
}

const GESTURE_CASES: &str = include_str!("../../../../../🧫️fixtures/🛠️gestures/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_routes_the_oracle_measured_replay_through_the_clicks_to_the_same_path_and_length() {
    let all: serde_json::Value = serde_json::from_str(GESTURE_CASES).expect("the committed cases");
    let routes = all["routes"].as_array().expect("routes");
    assert!(routes.len() >= 3);
    for case in routes {
        let mut rig = Rig::plan(UTILITY, furnished());
        let points: Vec<[f64; 3]> = case["points"].as_array().expect("points").iter().map(|p| [p[0].as_f64().expect("x"), p[1].as_f64().expect("y"), p[2].as_f64().expect("z")]).collect();
        for point in &points {
            assert!(rig.typed(&format!("z {}", point[2])).refused.is_none());
            rig.down(point[0], point[1]);
        }
        let last = points.last().expect("a point");
        rig.mv(last[0], last[1]);
        let expected = format!("{:.2} m", case["length"].as_f64().expect("length"));
        assert!(rig.preview.marks.iter().any(|mark| mark.shape == Shape::Label && mark.text == expected), "the run measures {expected}: {:?}", rig.preview.marks.iter().map(|mark| mark.text.clone()).collect::<Vec<_>>());
        let path = created(&rig.finish()).path.clone();
        assert_eq!(path, points.iter().map(|p| at(p[0], p[1], p[2])).collect::<Vec<_>>(), "{case}");
    }
}
