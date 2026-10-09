use super::*;
use crate::{Building, Storey};

fn base() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.buildings.insert("b".into(), Building { site: "s".into(), name: "House".into(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 });
    snapshot.buildings.insert("c".into(), Building { site: "s".into(), name: "Annex".into(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 });
    snapshot.storeys.insert("g".into(), Storey { building: "b".into(), name: "Ground".into(), level: 0, height: 3.0, cut_height: Some(1.5) });
    snapshot.storeys.insert("a".into(), Storey { building: "c".into(), name: "Annex ground".into(), level: 0, height: 3.0, cut_height: None });
    snapshot
}

fn plane() -> ViewPlane {
    ViewPlane { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 10.0, y: 0.0 } }
}

fn camera() -> ViewCamera {
    ViewCamera { target: Point2 { x: 0.0, y: 0.0 }, target_height: 1.0, azimuth: 0.5, pitch: 0.4, distance: 20.0 }
}

fn problem(view: &View) -> Option<(bool, &'static str)> {
    view_problem(&base(), "v", view).map(|row| (row.missing, row.field))
}

#[semio_framework_async_macros::async_test]
async fn every_kind_with_its_own_fields_is_writable() {
    assert_eq!(problem(&View::of_storey("b", "Ground plan", ViewKind::Plan, "g")), None);
    assert_eq!(problem(&View::of_storey("b", "Ground ceiling", ViewKind::CeilingPlan, "g")), None);
    assert_eq!(problem(&View::through("b", "A", ViewKind::Section, plane())), None);
    assert_eq!(problem(&View::through("b", "South", ViewKind::Elevation, plane())), None);
    assert_eq!(problem(&View { camera: Some(camera()), ..View::standard("b", "Iso", ViewKind::Orthographic) }), None);
    assert_eq!(problem(&View { camera: Some(camera()), ..View::standard("b", "Eye", ViewKind::Perspective) }), None);
}

#[semio_framework_async_macros::async_test]
async fn a_kind_owns_exactly_its_fields() {
    assert_eq!(problem(&View::standard("b", "Plan", ViewKind::Plan)), Some((false, "storey")));
    assert_eq!(problem(&View { plane: Some(plane()), ..View::of_storey("b", "Plan", ViewKind::Plan, "g") }), Some((false, "plane")));
    assert_eq!(problem(&View { storey: Some("g".into()), ..View::through("b", "A", ViewKind::Section, plane()) }), Some((false, "storey")));
    assert_eq!(problem(&View::standard("b", "A", ViewKind::Section)), Some((false, "plane")));
    assert_eq!(problem(&View::standard("b", "Iso", ViewKind::Orthographic)), Some((false, "camera")));
    assert_eq!(problem(&View { camera: Some(camera()), ..View::through("b", "A", ViewKind::Elevation, plane()) }), Some((false, "camera")));
    assert_eq!(problem(&View { cut_height: Some(1.0), ..View::through("b", "A", ViewKind::Elevation, plane()) }), Some((false, "cut_height")));
    assert_eq!(problem(&View { crop: Some(ViewCrop { min: Point2 { x: 0.0, y: 0.0 }, max: Point2 { x: 1.0, y: 1.0 } }), camera: Some(camera()), ..View::standard("b", "Iso", ViewKind::Perspective) }), Some((false, "crop")));
}

#[semio_framework_async_macros::async_test]
async fn references_must_exist_and_agree() {
    assert_eq!(problem(&View::of_storey("x", "Plan", ViewKind::Plan, "g")), Some((true, "building")));
    assert_eq!(problem(&View::of_storey("b", "Plan", ViewKind::Plan, "nope")), Some((true, "storey")));
    assert_eq!(problem(&View::of_storey("b", "Plan", ViewKind::Plan, "a")), Some((false, "storey")));
}

#[semio_framework_async_macros::async_test]
async fn names_are_filled_and_unique_per_building() {
    assert_eq!(problem(&View::of_storey("b", " ", ViewKind::Plan, "g")), Some((false, "name")));
    let mut snapshot = base();
    snapshot.views.insert("w".into(), View::of_storey("b", "Ground plan", ViewKind::Plan, "g"));
    let twin = View::through("b", "Ground plan", ViewKind::Section, plane());
    assert_eq!(view_problem(&snapshot, "v", &twin).map(|row| row.field), Some("name"));
    assert_eq!(view_problem(&snapshot, "w", &View::of_storey("b", "Ground plan", ViewKind::Plan, "g")), None, "a view does not clash with itself");
    assert_eq!(view_problem(&snapshot, "v", &View::through("c", "Ground plan", ViewKind::Section, plane())), None, "other buildings may reuse the name");
}

#[semio_framework_async_macros::async_test]
async fn numbers_are_positive_finite_and_ordered() {
    let section = || View::through("b", "A", ViewKind::Section, plane());
    assert_eq!(problem(&View { depth: 0.0, ..section() }), Some((false, "depth")));
    assert_eq!(problem(&View { depth: f64::NAN, ..section() }), Some((false, "depth")));
    assert_eq!(problem(&View { scale: 0, ..section() }), Some((false, "scale")));
    assert_eq!(problem(&View { scale: MAX_VIEW_SCALE + 1, ..section() }), Some((false, "scale")));
    assert_eq!(problem(&View { cut_height: Some(0.0), ..View::of_storey("b", "P", ViewKind::Plan, "g") }), Some((false, "cut_height")));
    assert_eq!(problem(&View { crop: Some(ViewCrop { min: Point2 { x: 2.0, y: 0.0 }, max: Point2 { x: 1.0, y: 1.0 } }), ..section() }), Some((false, "crop")));
    assert_eq!(problem(&View { hidden: vec![ViewCategory::Beams, ViewCategory::Walls], ..section() }), Some((false, "hidden")));
    assert_eq!(problem(&View { hidden: vec![ViewCategory::Walls, ViewCategory::Walls], ..section() }), Some((false, "hidden")));
    assert_eq!(problem(&View { hidden: vec![ViewCategory::Walls, ViewCategory::Beams], ..section() }), None);
    assert_eq!(problem(&View::through("b", "A", ViewKind::Section, ViewPlane { start: Point2 { x: 1.0, y: 1.0 }, end: Point2 { x: 1.0, y: 1.0 } })), Some((false, "plane")));
    assert_eq!(problem(&View { camera: Some(ViewCamera { pitch: FRAC_PI_2, ..camera() }), ..View::standard("b", "Iso", ViewKind::Orthographic) }), Some((false, "camera")));
    assert_eq!(problem(&View { camera: Some(ViewCamera { distance: 0.0, ..camera() }), ..View::standard("b", "Iso", ViewKind::Orthographic) }), Some((false, "camera")));
}

#[semio_framework_async_macros::async_test]
async fn the_cut_height_falls_back_from_the_view_to_the_storey_to_the_convention() {
    let snapshot = base();
    let plan = View::of_storey("b", "P", ViewKind::Plan, "g");
    assert_eq!(view_cut_height(&snapshot, &plan), Some(1.5), "the storey's cut height");
    assert_eq!(view_cut_height(&snapshot, &View { cut_height: Some(0.9), ..plan.clone() }), Some(0.9), "the view's own cut height");
    assert_eq!(view_cut_height(&snapshot, &View::of_storey("c", "P", ViewKind::Plan, "a")), Some(DEFAULT_CUT_HEIGHT), "the plan convention");
    assert_eq!(view_cut_height(&snapshot, &View::of_storey("b", "C", ViewKind::CeilingPlan, "g")), Some(DEFAULT_CEILING_CUT_HEIGHT), "the ceiling convention ignores the storey");
    assert_eq!(view_cut_height(&snapshot, &View::through("b", "A", ViewKind::Section, plane())), None);
}

#[semio_framework_async_macros::async_test]
async fn the_phase_filter_lets_one_phase_through() {
    let view = View::standard("b", "P", ViewKind::Plan);
    assert!(view.shows_phase(Phase::New) && view.shows_phase(Phase::Existing));
    let filtered = View { phase: Some(Phase::New), ..view };
    assert!(filtered.shows_phase(Phase::New) && !filtered.shows_phase(Phase::Existing));
}
