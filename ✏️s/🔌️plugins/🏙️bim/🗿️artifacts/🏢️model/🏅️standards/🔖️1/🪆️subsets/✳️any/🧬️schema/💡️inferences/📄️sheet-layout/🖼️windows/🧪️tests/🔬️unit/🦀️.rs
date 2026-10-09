//! 🧪️ The paper frame and the window of a viewport: sizes follow the scale to the millimetre, a crop wins over the bounds, a missing drawing leaves a minimal window.

use super::*;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanBounds, PlanKind, PlanLinework, PlanPolyline, PlanStyle, PlanVertex};
use crate::{DetailLevel, Point2, ViewCrop, ViewKind};

fn drawing(scale: u32, bounds: (f64, f64, f64, f64)) -> ViewLinework {
    let stroke = PlanPolyline { id: "l-1".into(), element: "w-1".into(), kind: PlanKind::WallOutline, style: PlanStyle::Cut, closed: false, vertices: vec![PlanVertex { x: bounds.0, y: bounds.1, bulge: 0.0 }, PlanVertex { x: bounds.2, y: bounds.3, bulge: 0.0 }] };
    let lines = PlanLinework { storey: "st-ground".into(), cut_height: 1.2, cut_elevation: 1.2, regions: Vec::new(), polylines: vec![stroke], texts: Vec::new(), bounds: PlanBounds { min_x: bounds.0, min_y: bounds.1, max_x: bounds.2, max_y: bounds.3 } };
    ViewLinework { view: "v-ground".into(), kind: ViewKind::Plan, scale, detail: DetailLevel::Medium, lines }
}

fn view() -> View {
    View::of_storey("bldg-1", "Ground plan", ViewKind::Plan, "st-ground")
}

fn viewport(scale: u32) -> Viewport {
    Viewport { scale, ..Viewport::standard("sh-1", "v-ground", Point2 { x: 30.0, y: 40.0 }) }
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} != {b}");
}

#[test]
fn the_frame_leaves_twenty_millimetres_at_the_binding_edge_and_ten_elsewhere() {
    assert_eq!(frame_of(420.0, 297.0), PaperRect { x: 20.0, y: 10.0, width: 390.0, height: 277.0 });
    assert_eq!(frame_of(10.0, 10.0).width, 0.0);
}

#[test]
fn a_scale_of_one_to_n_draws_a_thousand_over_n_millimetres_per_metre() {
    [(1, 1000.0), (20, 50.0), (50, 20.0), (100, 10.0), (200, 5.0), (1000, 1.0)].into_iter().for_each(|(scale, mm)| near(mm_per_metre(scale), mm));
    near(mm_per_metre(0), 1000.0);
}

#[test]
fn an_uncropped_window_is_the_bounds_with_five_millimetres_of_air_at_the_scale() {
    let placed = place("vp-1", &viewport(100), &view(), Some(&drawing(100, (0.0, 0.0, 8.0, 6.0))));
    near(placed.window.width, 8.0 * 10.0 + 2.0 * WINDOW_PADDING);
    near(placed.window.height, 6.0 * 10.0 + 2.0 * WINDOW_PADDING);
    assert_eq!((placed.window.x, placed.window.y), (30.0, 40.0));
    near(placed.map.min_x, -0.5);
    near(placed.map.max_y, 6.5);
    near(placed.map.mm, 10.0);
    assert!(!placed.cropped && !placed.empty);
    assert_eq!((placed.view.as_str(), placed.label.as_str(), placed.kind, placed.scale), ("v-ground", "Ground plan", ViewKind::Plan, 100));
}

#[test]
fn the_window_scales_with_the_viewport_not_with_the_view() {
    let at_fifty = place("vp-1", &viewport(50), &view(), Some(&drawing(100, (0.0, 0.0, 8.0, 6.0))));
    near(at_fifty.window.width, 8.0 * 20.0 + 2.0 * WINDOW_PADDING);
    near(at_fifty.window.height, 6.0 * 20.0 + 2.0 * WINDOW_PADDING);
}

#[test]
fn a_crop_is_the_window_whatever_the_drawing_holds() {
    let crop = ViewCrop { min: Point2 { x: 0.0, y: 0.0 }, max: Point2 { x: 6.0, y: 4.0 } };
    let placed = place("vp-1", &Viewport { crop: Some(crop), ..viewport(50) }, &view(), Some(&drawing(100, (-20.0, -20.0, 80.0, 60.0))));
    near(placed.window.width, 120.0);
    near(placed.window.height, 80.0);
    assert!(placed.cropped);
    let (x, y) = placed.map.point(&placed.window, 3.0, 2.0);
    near(x, 30.0 + 60.0);
    near(y, 40.0 + 40.0);
    let (corner_x, corner_y) = placed.map.point(&placed.window, 0.0, 4.0);
    near(corner_x, 30.0);
    near(corner_y, 40.0);
}

#[test]
fn a_missing_or_tiny_drawing_leaves_a_window_of_at_least_ten_millimetres() {
    let missing = place("vp-1", &viewport(100), &view(), None);
    assert_eq!((missing.window.width, missing.window.height, missing.empty), (MIN_WINDOW, MIN_WINDOW, true));
    let tiny = ViewCrop { min: Point2 { x: 0.0, y: 0.0 }, max: Point2 { x: 0.1, y: 0.1 } };
    let small = place("vp-1", &Viewport { crop: Some(tiny), ..viewport(100) }, &view(), None);
    assert_eq!((small.window.width, small.window.height), (MIN_WINDOW, MIN_WINDOW));
}

#[test]
fn the_label_is_the_authored_one_else_the_name_of_the_view() {
    let named = place("vp-1", &Viewport { label: Some("Entrance floor".into()), ..viewport(100) }, &view(), None);
    assert_eq!(named.label, "Entrance floor");
    assert_eq!(place("vp-1", &viewport(100), &view(), None).label, "Ground plan");
}
