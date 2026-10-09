use super::frame::Frame;
use super::hidden_lines::{bodies, faces, feature_edges, pieces};
use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{SolidBuilder, SolidFamily};
use crate::{Building, Layer, LayerFunction, Material, MaterialCategory, Phase, Point2, Rgb, Site, Storey, ViewCategory, ViewCrop, ViewPlane, Wall, WallType};
use crate::{Axis, LocationLine, TopConstraint};
use protocol::Inference;
use semio_framework_geometry::mesh::TriMesh;

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-6 * right.abs().max(1.0)
}

fn room() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.materials.insert("m".into(), Material { name: "Brick".into(), category: MaterialCategory::Masonry, color: Rgb { r: 0.7, g: 0.3, b: 0.2 }, density: 1800.0, conductivity: 0.8, specific_heat: 900.0 });
    snapshot.wall_types.insert("wt".into(), WallType { name: "Brick 300".into(), layers: vec![Layer { material: "m".into(), thickness: 0.3, function: LayerFunction::Structure }] });
    snapshot.sites.insert("site".into(), Site { name: "Plot".into(), latitude: 47.0, longitude: 8.0, elevation: 0.0, true_north: 0.0, boundary: Vec::new() });
    snapshot.buildings.insert("b".into(), Building { site: "site".into(), name: "House".into(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 });
    snapshot.storeys.insert("g".into(), Storey { building: "b".into(), name: "Ground".into(), level: 0, height: 3.0, cut_height: None });
    let corners = [(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)];
    for (index, start) in corners.iter().enumerate() {
        let end = corners[(index + 1) % 4];
        snapshot.walls.insert(format!("w{index}"), Wall { storey: "g".into(), wall_type: "wt".into(), axis: Axis::Line { start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } }, location: LocationLine::Center, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: if index == 3 { Phase::Existing } else { Phase::New }, name: format!("Wall {index}") });
    }
    snapshot
}

fn plane(a: (f64, f64), b: (f64, f64)) -> ViewPlane {
    ViewPlane { start: Point2 { x: a.0, y: a.1 }, end: Point2 { x: b.0, y: b.1 } }
}

fn with(mut snapshot: ModelSnapshot, id: &str, view: View) -> ModelSnapshot {
    snapshot.views.insert(id.into(), view);
    snapshot
}

fn drawn(snapshot: &ModelSnapshot, id: &str) -> ViewLinework {
    ModelInference::infer(snapshot).expect("infers").view_linework.remove(id).expect("the view is drawn")
}

fn south() -> View {
    View::through("b", "South", ViewKind::Elevation, plane((-2.0, -2.0), (8.0, -2.0)))
}

fn cuboid(family: SolidFamily, low: [f64; 3], high: [f64; 3]) -> ElementSolid {
    let mut mesh = TriMesh::new();
    let at = |x: usize, y: usize, z: usize| [[low[0], high[0]][x], [low[1], high[1]][y], [low[2], high[2]][z]];
    mesh.push_quad(at(0, 0, 0), at(0, 1, 0), at(1, 1, 0), at(1, 0, 0));
    mesh.push_quad(at(0, 0, 1), at(1, 0, 1), at(1, 1, 1), at(0, 1, 1));
    mesh.push_quad(at(0, 0, 0), at(1, 0, 0), at(1, 0, 1), at(0, 0, 1));
    mesh.push_quad(at(1, 0, 0), at(1, 1, 0), at(1, 1, 1), at(1, 0, 1));
    mesh.push_quad(at(1, 1, 0), at(0, 1, 0), at(0, 1, 1), at(1, 1, 1));
    mesh.push_quad(at(0, 1, 0), at(0, 0, 0), at(0, 0, 1), at(0, 1, 1));
    let mut builder = SolidBuilder::new(family);
    builder.add("body", "m", 0, &mesh);
    builder.build()
}

#[semio_framework_async_macros::async_test]
async fn a_frame_measures_along_and_behind_the_plane() {
    let frame = Frame::of(&plane((-2.0, -2.0), (8.0, -2.0))).expect("a frame");
    assert!(close(frame.length, 10.0));
    assert_eq!(frame.draw([1.0, 3.0, 2.5]), [3.0, 2.5, 5.0], "u along the line, z up, w behind the plane toward +y");
    assert!(Frame::of(&plane((1.0, 1.0), (1.0, 1.0))).is_none());
    let reversed = Frame::of(&plane((8.0, -2.0), (-2.0, -2.0))).expect("a frame");
    assert!(close(reversed.w([0.0, -3.0, 0.0]), -1.0), "the viewer of the reversed line looks toward -y");
}

#[semio_framework_async_macros::async_test]
async fn the_elevation_of_a_room_is_its_outer_rectangle() {
    let view = drawn(&with(room(), "v", south()), "v");
    assert_eq!(view.kind, ViewKind::Elevation);
    assert!(close(view.union_area_of(PlanKind::Silhouette), 6.3 * 3.0), "the four mitred walls project to the outer rectangle: {}", view.union_area_of(PlanKind::Silhouette));
    assert_eq!(view.lines.area_of(PlanKind::SectionCut), 0.0, "an elevation cuts nothing");
    assert!(close(view.length_in(PlanKind::Edge, PlanStyle::Projection), 2.0 * (6.3 + 3.0)), "the outline of the front face: {}", view.length_in(PlanKind::Edge, PlanStyle::Projection));
    assert_eq!(view.length_in(PlanKind::Edge, PlanStyle::Hidden), 0.0, "a medium view draws no hidden edge");
}

#[semio_framework_async_macros::async_test]
async fn a_fine_elevation_dashes_the_edges_it_hides() {
    let view = drawn(&with(room(), "v", View { detail: DetailLevel::Fine, ..south() }), "v");
    assert!(view.length_in(PlanKind::Edge, PlanStyle::Hidden) > 0.0, "the far wall's edges are behind the near wall");
    let coarse = drawn(&with(room(), "v", View { detail: DetailLevel::Coarse, ..south() }), "v");
    assert!(close(coarse.length_in(PlanKind::Edge, PlanStyle::Projection), 2.0 * (6.3 + 3.0)), "a coarse elevation strokes the silhouette: {}", coarse.length_in(PlanKind::Edge, PlanStyle::Projection));
}

#[semio_framework_async_macros::async_test]
async fn a_section_cuts_the_walls_the_plane_passes_through() {
    let view = drawn(&with(room(), "v", View::through("b", "A", ViewKind::Section, plane((-2.0, 2.0), (8.0, 2.0)))), "v");
    assert!(close(view.lines.area_of(PlanKind::SectionCut), 2.0 * 0.3 * 3.0), "the east and west walls are cut {} by 3: {}", 0.3, view.lines.area_of(PlanKind::SectionCut));
    assert!(view.lines.regions.iter().filter(|region| region.kind == PlanKind::SectionCut).all(|region| region.style == PlanStyle::Cut));
    let datums: Vec<_> = view.lines.polylines.iter().filter(|line| line.kind == PlanKind::Datum).collect();
    assert_eq!(datums.len(), 2, "the floor and the top of the storey");
    assert!(close(view.lines.length_of(PlanKind::Datum), 2.0 * 10.0));
}

#[semio_framework_async_macros::async_test]
async fn the_depth_of_a_section_limits_what_lies_behind_the_cut() {
    let deep = drawn(&with(room(), "v", View::through("b", "A", ViewKind::Section, plane((-2.0, 2.0), (8.0, 2.0)))), "v");
    let shallow = drawn(&with(room(), "v", View { depth: 1.0, ..View::through("b", "A", ViewKind::Section, plane((-2.0, 2.0), (8.0, 2.0))) }), "v");
    assert!(deep.union_area_of(PlanKind::Silhouette) > shallow.union_area_of(PlanKind::Silhouette), "the north wall lies 1.85 m behind the plane");
    assert!(close(shallow.lines.area_of(PlanKind::SectionCut), deep.lines.area_of(PlanKind::SectionCut)), "the cut is the same");
}

#[semio_framework_async_macros::async_test]
async fn a_hidden_category_and_a_phase_filter_drop_their_elements() {
    let hidden = drawn(&with(room(), "v", View { hidden: vec![ViewCategory::Walls], ..south() }), "v");
    assert!(hidden.lines.regions.is_empty() && hidden.lines.polylines.iter().all(|line| line.kind == PlanKind::Datum), "no wall remains, only the datums");
    let phased = drawn(&with(room(), "v", View { phase: Some(Phase::Existing), ..south() }), "v");
    assert!(close(phased.union_area_of(PlanKind::Silhouette), 0.3 * 3.0), "only the west wall is existing: {}", phased.union_area_of(PlanKind::Silhouette));
}

#[semio_framework_async_macros::async_test]
async fn a_crop_clips_a_vertical_view_to_its_rectangle() {
    let crop = ViewCrop { min: Point2 { x: 2.0, y: 0.0 }, max: Point2 { x: 5.0, y: 2.0 } };
    let view = drawn(&with(room(), "v", View { crop: Some(crop), ..south() }), "v");
    assert!(close(view.union_area_of(PlanKind::Silhouette), 3.0 * 2.0), "the silhouette is cut to the crop: {}", view.union_area_of(PlanKind::Silhouette));
    assert_eq!((view.lines.bounds.min_x, view.lines.bounds.max_x, view.lines.bounds.min_y, view.lines.bounds.max_y), (2.0, 5.0, 0.0, 2.0));
}

#[semio_framework_async_macros::async_test]
async fn a_plan_view_cuts_at_the_height_it_resolves() {
    let snapshot = with(room(), "v", View::of_storey("b", "Ground plan", ViewKind::Plan, "g"));
    let view = drawn(&snapshot, "v");
    let plan = ModelInference::infer(&snapshot).expect("infers").plan_linework.remove("g").expect("the storey plan");
    assert!(close(view.lines.area_of(PlanKind::WallCut), plan.area_of(PlanKind::WallCut)), "the default cut is the plan convention");
    assert!(view.lines.area_of(PlanKind::WallCut) > 0.0);
    assert_eq!(view.lines.storey, "g");
    let high = drawn(&with(room(), "v", View { cut_height: Some(3.5), ..View::of_storey("b", "Ground plan", ViewKind::Plan, "g") }), "v");
    assert_eq!(high.lines.area_of(PlanKind::WallCut), 0.0, "a cut above the walls cuts nothing");
    assert!(high.lines.length_of(PlanKind::WallOutline) > 0.0, "the walls below it are projection");
}

#[semio_framework_async_macros::async_test]
async fn a_ceiling_plan_looks_up_and_swaps_projection_and_hidden() {
    let plan = drawn(&with(room(), "v", View { cut_height: Some(3.5), ..View::of_storey("b", "Ground plan", ViewKind::Plan, "g") }), "v");
    let ceiling = drawn(&with(room(), "v", View { cut_height: Some(3.5), ..View::of_storey("b", "Ground ceiling", ViewKind::CeilingPlan, "g") }), "v");
    assert!(plan.lines.polylines.iter().any(|line| line.kind == PlanKind::WallOutline && line.style == PlanStyle::Projection));
    assert!(ceiling.lines.polylines.iter().any(|line| line.kind == PlanKind::WallOutline && line.style == PlanStyle::Hidden), "what lies below a ceiling cut is dashed");
}

#[semio_framework_async_macros::async_test]
async fn a_plan_view_hides_categories_filters_phases_and_crops() {
    let base = View::of_storey("b", "Ground plan", ViewKind::Plan, "g");
    let hidden = drawn(&with(room(), "v", View { hidden: vec![ViewCategory::Walls], ..base.clone() }), "v");
    assert_eq!(hidden.lines.area_of(PlanKind::WallCut), 0.0);
    let existing = drawn(&with(room(), "v", View { phase: Some(Phase::Existing), ..base.clone() }), "v");
    let all = drawn(&with(room(), "v", base.clone()), "v");
    assert!(existing.lines.area_of(PlanKind::WallCut) > 0.0 && existing.lines.area_of(PlanKind::WallCut) < all.lines.area_of(PlanKind::WallCut) / 2.0, "one of four walls is existing");
    let crop = ViewCrop { min: Point2 { x: -1.0, y: -1.0 }, max: Point2 { x: 3.0, y: 1.0 } };
    let cropped = drawn(&with(room(), "v", View { crop: Some(crop), ..base.clone() }), "v");
    assert!(cropped.lines.regions.len() < all.lines.regions.len());
    assert_eq!((cropped.lines.bounds.min_x, cropped.lines.bounds.max_x), (-1.0, 3.0));
    let coarse = drawn(&with(room(), "v", View { detail: DetailLevel::Coarse, ..base }), "v");
    assert_eq!(coarse.lines.count_of(PlanKind::WallLayer), 0, "a coarse plan draws no layer lines");
}

#[semio_framework_async_macros::async_test]
async fn a_camera_view_and_a_view_of_a_missing_storey_draw_nothing() {
    let camera = View { camera: Some(crate::ViewCamera { target: Point2 { x: 0.0, y: 0.0 }, target_height: 1.0, azimuth: 0.0, pitch: 0.3, distance: 10.0 }), ..View::standard("b", "Iso", ViewKind::Orthographic) };
    let view = drawn(&with(room(), "v", camera), "v");
    assert!(view.lines.regions.is_empty() && view.lines.polylines.is_empty() && view.lines.texts.is_empty());
    let lost = drawn(&with(room(), "v", View::of_storey("b", "Lost", ViewKind::Plan, "nowhere")), "v");
    assert!(lost.lines.regions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_nearer_element_hides_the_edges_behind_it() {
    let frame = Frame::of(&plane((-2.0, -5.0), (8.0, -5.0))).expect("a frame");
    let near = cuboid(SolidFamily::Column, [2.0, -1.0, 0.0], [4.0, 0.0, 2.0]);
    let far = cuboid(SolidFamily::Wall, [0.0, 1.0, 0.0], [6.0, 2.0, 1.0]);
    let solids = [("near", &near), ("far", &far)];
    let seen = bodies(&frame, 100.0, solids);
    assert_eq!(seen.len(), 2);
    let edges: Vec<_> = seen.iter().flat_map(|body| feature_edges(body, &frame, 100.0)).collect();
    let all = pieces(&edges, &seen, &[]);
    let visible: f64 = all.iter().filter(|piece| piece.visible).map(|piece| (piece.b[0] - piece.a[0]).hypot(piece.b[1] - piece.a[1])).sum();
    let hidden: f64 = all.iter().filter(|piece| !piece.visible).map(|piece| (piece.b[0] - piece.a[0]).hypot(piece.b[1] - piece.a[1])).sum();
    assert!(close(visible, 8.0 + 10.0), "the column's 2 x 2 outline plus the wall's outline without the 2 m behind the column on both long edges: {visible}");
    assert!(close(hidden, 2.0 * 2.0 + 0.0), "the wall's two long edges are hidden for 2 m each behind the column: {hidden}");
    assert!(faces(&seen[0]).len() == 1 && faces(&seen[1]).len() == 1, "each prism projects to one face");
    let area = |body: &hidden_lines::Body| faces(body).iter().map(|cut| Region::new(&cut.outer, &cut.holes).area()).sum::<f64>();
    assert!(close(area(&seen[0]), 4.0) && close(area(&seen[1]), 6.0), "the faces are the exact rectangles");
}

#[semio_framework_async_macros::async_test]
async fn editing_one_view_computes_that_view_only() {
    use super::super::model_graph::ModelInferenceSession;
    let first = with(room(), "v-south", south());
    let second = with(first.clone(), "v-plan", View::of_storey("b", "Ground plan", ViewKind::Plan, "g"));
    let mut session = ModelInferenceSession::new();
    session.refresh(&second);
    assert_eq!(session.inference().view_linework.len(), 2);
    let mut changed = second.clone();
    changed.views.get_mut("v-south").expect("the view").scale = 50;
    let diff = crate::ModelDiff::views("v-south", crate::Entry::Patched(crate::ViewPatch { scale: Some(50), ..Default::default() }));
    session.update(&changed, &diff);
    let report = session.report();
    assert!(!report.gated, "a view edit touches the graph");
    assert_eq!(report.computed_by_kind.get("view").copied(), Some(1), "only the edited view is recomputed: {:?}", report.computed_by_kind);
    assert_eq!(report.computed_by_kind.values().sum::<usize>(), 1, "nothing else is recomputed: {:?}", report.computed_by_kind);
    assert_eq!(session.inference().view_linework["v-south"].scale, 50);
    assert_eq!(session.inference(), &ModelInference::infer(&changed).expect("infers"), "the session agrees with a fresh inference");
}

#[semio_framework_async_macros::async_test]
async fn renaming_a_view_recomputes_nothing_and_moving_a_wall_redraws_the_views_that_see_it() {
    use super::super::model_graph::ModelInferenceSession;
    let first = with(room(), "v-south", south());
    let mut session = ModelInferenceSession::new();
    session.refresh(&first);
    let mut renamed = first.clone();
    renamed.views.get_mut("v-south").expect("the view").name = "Front".into();
    let diff = crate::ModelDiff::views("v-south", crate::Entry::Patched(crate::ViewPatch { name: Some("Front".into()), ..Default::default() }));
    session.update(&renamed, &diff);
    assert_eq!(session.report().computed_by_kind.values().sum::<usize>(), 0, "a name is never read by the drawing: {:?}", session.report().computed_by_kind);
    let mut moved = renamed.clone();
    moved.storeys.get_mut("g").expect("the storey").height = 3.5;
    let diff = crate::ModelDiff::storeys("g", crate::Entry::Patched(crate::StoreyPatch { height: Some(3.5), ..Default::default() }));
    session.update(&moved, &diff);
    assert_eq!(session.report().computed_by_kind.get("view").copied(), Some(1), "{:?}", session.report().computed_by_kind);
    assert!(close(session.inference().view_linework["v-south"].union_area_of(PlanKind::Silhouette), 6.3 * 3.5));
}

#[semio_framework_async_macros::async_test]
async fn the_inference_default_and_determinism_laws_hold_for_views() {
    assert_eq!(ModelInference::infer(&ModelSnapshot::default()).expect("infers").view_linework, std::collections::BTreeMap::new());
    let snapshot = with(with(room(), "v-south", south()), "v-plan", View::of_storey("b", "Ground plan", ViewKind::Plan, "g"));
    assert_eq!(ModelInference::infer(&snapshot).expect("infers"), ModelInference::infer(&snapshot).expect("infers"));
}
