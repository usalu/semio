use super::*;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanKind;
use crate::{Point2, View, ViewKind};

fn demo() -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

fn through_the_house() -> BimSectionWindowConfig {
    BimSectionWindowConfig { view: "v-section-a".into(), ..BimSectionWindowConfig::default() }
}

#[semio_framework_async_macros::async_test]
async fn a_section_view_through_the_demo_cuts_the_two_side_walls() {
    let (_, inference) = demo();
    let drawing = &inference.view_linework["v-section-a"];
    let elements: std::collections::BTreeSet<&str> = drawing.lines.regions.iter().filter(|region| region.kind == PlanKind::SectionCut).map(|region| region.element.as_str()).collect();
    assert_eq!(elements, std::collections::BTreeSet::from(["w-east", "w-west"]));
}

#[semio_framework_async_macros::async_test]
async fn the_cut_poche_has_one_region_per_wall_layer_spanning_the_wall_height_and_thickness() {
    let (_, inference) = demo();
    let east: Vec<_> = inference.view_linework["v-section-a"].lines.regions.iter().filter(|region| region.kind == PlanKind::SectionCut && region.element == "w-east").collect();
    assert_eq!(east.len(), 2, "the two-layer wall cuts into one region per layer");
    let points: Vec<[f64; 2]> = east.iter().flat_map(|region| region.outer.iter().map(|vertex| [vertex.x, vertex.y])).collect();
    let (u_min, u_max) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p[0]), hi.max(p[0])));
    let (z_min, z_max) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p[1]), hi.max(p[1])));
    assert!((u_max - u_min - 0.3).abs() < 1e-6, "the two layers add up to the 0.3 m wall, got {}", u_max - u_min);
    assert!((z_max - z_min - 3.0).abs() < 1e-6, "the storey top constraint makes the wall as high as its 3 m storey, got {}", z_max - z_min);
}

#[semio_framework_async_macros::async_test]
async fn the_window_shows_the_configured_view_else_the_first_vertical_one() {
    let (snapshot, _) = demo();
    assert_eq!(active_view(&snapshot, &through_the_house()).as_deref(), Some("v-section-a"));
    assert!(active_view(&snapshot, &BimSectionWindowConfig::default()).is_some(), "an unbound window falls back to the first section or elevation");
    assert_eq!(active_view(&snapshot, &BimSectionWindowConfig { view: "v-plan-st-ground".into(), ..BimSectionWindowConfig::default() }).as_deref(), vertical_views(&snapshot).first().map(String::as_str), "a plan view is no section");
    assert!(plane_of(&snapshot, &through_the_house()).is_some());
    assert_eq!(active_view(&ModelSnapshot::default(), &through_the_house()), None);
}

#[semio_framework_async_macros::async_test]
async fn a_plane_beside_the_model_cuts_nothing() {
    let (mut snapshot, _) = demo();
    snapshot.views.insert("v-beside".into(), View::through("bldg-1", "Beside", ViewKind::Section, crate::ViewPlane { start: Point2 { x: -1.0, y: 20.0 }, end: Point2 { x: 9.0, y: 20.0 } }));
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, &snapshot, Clone::clone);
    assert_eq!(inference.view_linework["v-beside"].lines.area_of(PlanKind::SectionCut), 0.0);
}

#[semio_framework_async_macros::async_test]
async fn the_records_carry_the_layers_of_the_drawing_and_an_empty_window_a_prompt() {
    let (snapshot, inference) = demo();
    let drawing = &inference.view_linework["v-section-a"];
    let painted = records(Some(drawing), &[], "select", &BimLabels::NATIVE_EN);
    assert_eq!(painted.len(), 1 + drawing.lines.regions.len() + drawing.lines.polylines.len() + drawing.lines.texts.len());
    assert!(render(&snapshot, &inference, &through_the_house(), &[], "select", 1, &BimLabels::NATIVE_EN).is_ok());
    assert_eq!(records(None, &[], "select", &BimLabels::NATIVE_EN).len(), 2, "the meta record and the prompt");
}
