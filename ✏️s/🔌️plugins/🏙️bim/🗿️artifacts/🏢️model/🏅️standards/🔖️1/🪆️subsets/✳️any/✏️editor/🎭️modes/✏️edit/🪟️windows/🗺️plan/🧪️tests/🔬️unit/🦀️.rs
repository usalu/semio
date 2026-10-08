use super::*;

fn demo() -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let inference = crate::editor::bim::inference::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

fn ground() -> BimPlanWindowConfig {
    BimPlanWindowConfig { storey: "st-ground".into(), ..BimPlanWindowConfig::default() }
}

#[semio_framework_async_macros::async_test]
async fn the_active_storey_falls_back_to_the_lowest_one() {
    let (snapshot, _) = demo();
    assert_eq!(active_storey(&snapshot, &BimPlanWindowConfig::default()).as_deref(), Some("st-ground"));
    assert_eq!(active_storey(&snapshot, &BimPlanWindowConfig { storey: "st-first".into(), ..BimPlanWindowConfig::default() }).as_deref(), Some("st-first"));
    assert_eq!(active_storey(&ModelSnapshot::default(), &ground()), None);
}

#[semio_framework_async_macros::async_test]
async fn the_plan_paints_the_linework_of_the_storey() {
    let (snapshot, inference) = demo();
    let plan = &inference.plan_linework["st-ground"];
    let painted = records(plan, &[], &[], "select");
    assert_eq!(painted.len(), 1 + plan.regions.len() + plan.polylines.len() + plan.texts.len());
    assert!(plan.regions.iter().any(|region| region.element == "w-south"), "the cut south wall is poché");
    assert!(render(&snapshot, &inference, &ground(), &[], &[], "select", 1, &BimLabels::NATIVE_EN).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn a_selection_recolours_in_place_and_a_hover_adds_one_outline_per_primitive() {
    let (_, inference) = demo();
    let plan = &inference.plan_linework["st-ground"];
    let ids = ["w-south".to_string()];
    let plain = records(plan, &[], &[], "select");
    assert_eq!(records(plan, &ids, &[], "select").len(), plain.len(), "the selection only recolours the shared layers");
    let owned = plan.regions.iter().filter(|r| r.element == "w-south").count() + plan.polylines.iter().filter(|l| l.element == "w-south").count();
    assert_eq!(records(plan, &[], &ids, "select").len(), plain.len() + owned);
    assert_eq!(records(plan, &ids, &ids, "select").len(), plain.len(), "selected wins over hovered, never both");
}

#[semio_framework_async_macros::async_test]
async fn a_pointer_press_picks_the_wall_under_it() {
    let (_, inference) = demo();
    let plan = &inference.plan_linework["st-ground"];
    assert_eq!(pick(plan, (4.0, 0.0), 0.05).as_deref(), Some("w-south"));
    assert_eq!(pick(plan, (4.0, 3.0), 0.05), None);
}

#[semio_framework_async_macros::async_test]
async fn pixels_map_to_model_metres_and_back_through_the_mirror() {
    let viewport = store::Viewport2d { x: 1.0, y: -2.0, zoom: 20.0 };
    let (x, y) = pixel_to_model(&viewport, 400.0, 300.0, 800.0, 600.0);
    assert!((x - 1.0).abs() < 1e-9 && (y - 2.0).abs() < 1e-9);
    let (x, y) = pixel_to_model(&viewport, 420.0, 280.0, 800.0, 600.0);
    assert!((x - 2.0).abs() < 1e-9 && (y - 3.0).abs() < 1e-9);
}
