use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanBounds;

fn demo_plan(storey: &str) -> PlanLinework {
    let model = parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses");
    crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, &model, |inference| inference.plan_linework.clone()).get(storey).cloned().expect("the storey has a plan")
}

fn vertex(x: f64, y: f64, bulge: f64) -> PlanVertex {
    PlanVertex { x, y, bulge }
}

fn manual(regions: Vec<PlanRegion>, polylines: Vec<PlanPolyline>) -> PlanLinework {
    PlanLinework { storey: "st".into(), cut_height: 1.2, cut_elevation: 1.2, regions, polylines, texts: Vec::new(), bounds: PlanBounds { min_x: -1.0, min_y: -2.0, max_x: 3.0, max_y: 4.0 } }
}

fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("layers are JSON")
}

#[test]
fn canvas_and_plan_coordinates_are_mirrored_inverses() {
    assert_eq!(canvas_point(2.0, 3.0), [2.0, -3.0]);
    assert_eq!(plan_point(canvas_point(2.0, 3.0)[0], canvas_point(2.0, 3.0)[1]), [2.0, 3.0]);
}

#[test]
fn canvas_bounds_mirror_the_plan_bounds_and_an_empty_plan_has_none() {
    assert_eq!(canvas_bounds(&manual(vec![], vec![PlanPolyline { id: "l".into(), element: "e".into(), kind: PlanKind::WallOutline, style: PlanStyle::Cut, closed: false, vertices: vec![vertex(0.0, 0.0, 0.0), vertex(1.0, 0.0, 0.0)] }])), Some([-1.0, -4.0, 3.0, 2.0]));
    assert_eq!(canvas_bounds(&manual(vec![], vec![])), None);
}

#[test]
fn the_demo_ground_plan_draws_its_cut_walls_as_poche_before_any_line() {
    let plan = demo_plan("st-ground");
    assert!(plan.regions.iter().any(|region| region.kind == PlanKind::WallCut), "the cut walls are regions");
    let records = layers(&plan, &[]);
    let region_count = plan.regions.len();
    assert!(records.len() >= region_count);
    for record in records.iter().take(region_count) {
        assert!(!record.get("fill").expect("fill key").is_null(), "regions are filled and come first");
    }
    for record in records.iter().skip(region_count) {
        assert!(record.get("fill").is_none_or(|fill| fill.is_null()) || record.get("text").is_some(), "after the regions only strokes and text follow");
    }
}

#[test]
fn a_bulged_edge_is_flattened_onto_its_circle() {
    let half_circle = PlanPolyline { id: "arc".into(), element: "e".into(), kind: PlanKind::WallOutline, style: PlanStyle::Cut, closed: false, vertices: vec![vertex(1.0, 0.0, 1.0), vertex(-1.0, 0.0, 0.0)] };
    let segments = ring(&half_circle.vertices, false);
    assert!(segments.len() > 8, "a half circle needs many chords at 0.5 mm");
    for segment in &segments {
        if let PathSegment::Move { to } | PathSegment::Line { to } = segment {
            let radius = (to[0] * to[0] + to[1] * to[1]).sqrt();
            assert!((radius - 1.0).abs() < 1e-3 + 1e-12, "every flattened point lies on the unit circle, got {radius}");
        }
    }
    assert!(matches!(segments.first(), Some(PathSegment::Move { .. })));
    assert_eq!(segments.last(), Some(&PathSegment::Line { to: [-1.0, 0.0] }));
}

#[test]
fn a_closed_ring_ends_in_close_without_repeating_its_first_point() {
    let square = vec![vertex(0.0, 0.0, 0.0), vertex(1.0, 0.0, 0.0), vertex(1.0, 1.0, 0.0), vertex(0.0, 1.0, 0.0)];
    let segments = ring(&square, true);
    assert_eq!(segments.len(), 5);
    assert_eq!(segments[4], PathSegment::Close);
    assert_eq!(segments[2], PathSegment::Line { to: [1.0, -1.0] });
}

#[test]
fn a_selected_element_is_drawn_in_the_accent_colour_and_nothing_else_is() {
    let plan = demo_plan("st-ground");
    let element = plan.regions.iter().find(|region| region.kind == PlanKind::WallCut).map(|region| region.element.clone()).expect("a cut wall");
    let accent = ACCENT.to_vec().to_value();
    let accented = |records: &[DslValue]| -> Vec<String> {
        records.iter().filter(|record| record.get("stroke").and_then(|stroke| stroke.get("color")).is_some_and(|color| *color == accent) || record.get("fill").and_then(|fill| fill.get("color")).is_some_and(|color| *color == accent)).filter_map(|record| record.get("id").and_then(DslValue::as_str).map(str::to_string)).collect()
    };
    let marked = accented(&layers(&plan, &[element.clone()]));
    assert!(!marked.is_empty());
    assert!(marked.iter().all(|id| id.starts_with(&format!("{element}/"))), "only the selected element is accented: {marked:?}");
    assert!(accented(&layers(&plan, &[])).is_empty());
}

#[test]
fn the_framing_revision_depends_on_the_storey() {
    assert_ne!(framing_revision("st-ground"), framing_revision("st-first"));
    assert_eq!(framing_revision("st-ground"), framing_revision("st-ground"));
}

#[test]
fn an_unframed_scene_fits_the_plan_and_a_framed_one_keeps_the_viewport() {
    let plan = demo_plan("st-ground");
    let viewport = store::Viewport2d { x: 5.0, y: -3.0, zoom: 40.0 };
    let unframed = scene(Some(&plan), &viewport, false, &[]);
    let framing = unframed.framing.expect("an unframed window asks for a fit");
    assert_eq!(framing.bounds, canvas_bounds(&plan).expect("bounds"));
    assert_eq!(framing.revision, framing_revision("st-ground"));
    let framed = scene(Some(&plan), &viewport, true, &[]);
    assert!(framed.framing.is_none());
    assert_eq!((framed.camera_x, framed.camera_y, framed.zoom), (5.0, -3.0, 40.0));
    assert!(!json(&framed.layers_json).as_array().expect("layers").is_empty());
}

#[test]
fn a_missing_plan_renders_no_layers_and_no_fit() {
    let scene = scene(None, &store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 }, false, &[]);
    assert_eq!(scene.layers_json, "[]");
    assert!(scene.framing.is_none());
}
