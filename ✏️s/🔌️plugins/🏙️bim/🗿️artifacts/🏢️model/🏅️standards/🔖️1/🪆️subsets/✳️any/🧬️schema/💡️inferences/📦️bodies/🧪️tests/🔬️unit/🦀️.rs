use super::*;
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::compute_stair_runs;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout;
use crate::{Point2, Vertex};

fn bodies(snapshot: &ModelSnapshot, storey: &str, levels: &BTreeMap<String, StoreyLevel>) -> Vec<Body> {
    let (layouts, runs) = (compute_wall_layout(snapshot), compute_stair_runs(snapshot));
    storey_bodies(snapshot, storey, levels, &layouts.iter().map(|(id, layout)| (id.as_str(), layout)).collect(), &runs.iter().map(|(id, run)| (id.as_str(), run)).collect())
}
use crate::{Slope, TopConstraint};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🗺️plan-linework/🏡️house/📸️snapshot/🔣️.json");

fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("house decodes")
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

#[semio_framework_async_macros::async_test]
async fn profile_outlines_have_the_area_of_their_shape() {
    let area = |profile: Profile| loops::area(&profile_outline(&profile));
    assert!(close(area(Profile::Rectangle { width: 0.4, depth: 0.3 }), 0.12));
    assert!(close(area(Profile::Circle { diameter: 0.5 }), std::f64::consts::PI * 0.0625));
    assert!(close(area(Profile::IShape { width: 0.2, depth: 0.4, web: 0.02, flange: 0.03 }), 2.0 * 0.2 * 0.03 + 0.02 * (0.4 - 0.06)));
    let triangle = vec![Vertex { point: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 0.0, y: 1.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 1.0, y: 0.0 }, bulge: 0.0 }];
    assert!(close(area(Profile::Custom { outline: triangle }), 0.5));
    assert_eq!(extents_of(&Profile::IShape { width: 0.2, depth: 0.4, web: 0.02, flange: 0.03 }), (0.2, 0.4));
}

#[semio_framework_async_macros::async_test]
async fn a_placed_outline_is_rotated_about_the_origin_then_moved() {
    let outline = placed(&profile_outline(&Profile::Rectangle { width: 0.4, depth: 0.2 }), Point::new(1.0, 2.0), std::f64::consts::FRAC_PI_2);
    let xs: Vec<f64> = outline.iter().map(|v| v.point.x).collect();
    let ys: Vec<f64> = outline.iter().map(|v| v.point.y).collect();
    assert!(close(xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - xs.iter().cloned().fold(f64::INFINITY, f64::min), 0.2));
    assert!(close(ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - ys.iter().cloned().fold(f64::INFINITY, f64::min), 0.4));
    assert!(close(loops::centroid(&outline).x, 1.0) && close(loops::centroid(&outline).y, 2.0));
}

#[semio_framework_async_macros::async_test]
async fn every_placed_element_of_a_storey_has_one_body_between_its_heights() {
    let snapshot = house();
    let levels = compute_storey_levels(&snapshot);
    let ground = bodies(&snapshot, "st-ground", &levels);
    let count = |kind: BodyKind| ground.iter().filter(|body| body.kind == kind).count();
    assert_eq!((count(BodyKind::Wall), count(BodyKind::Column), count(BodyKind::Beam), count(BodyKind::Slab), count(BodyKind::Stair)), (5, 1, 1, 1, 1));
    let column = ground.iter().find(|body| body.id == "c-1").expect("column");
    assert!(close(column.z_min, 0.0) && close(column.z_max, 3.0) && close(column.area(), 0.16));
    assert!(column.bounds.iter().zip([5.8, 2.8, 6.2, 3.2]).all(|(have, want)| close(*have, want)), "the footprint rectangle");
    let beam = ground.iter().find(|body| body.id == "b-1").expect("beam");
    assert!(close(beam.z_min, 2.25) && close(beam.z_max, 2.75), "a beam of depth 0.5 hangs 0.25 below the storey top");
    let slab = ground.iter().find(|body| body.id == "sl-ground").expect("slab");
    assert!(close(slab.z_min, -0.22) && close(slab.z_max, 0.0) && close(slab.area(), 48.0));
    let stair = ground.iter().find(|body| body.id == "s-1").expect("stair");
    assert!(close(stair.z_min, 0.0) && close(stair.z_max, 3.0));
    let first = bodies(&snapshot, "st-first", &levels);
    assert_eq!(first.iter().filter(|body| body.kind == BodyKind::CurtainWall).count(), 1);
    assert_eq!(first.len(), 5, "three walls, the curtain wall and the slab; a roof has no body");
    assert!(bodies(&snapshot, "st-nowhere", &levels).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_wall_body_is_the_join_trimmed_footprint() {
    let snapshot = house();
    let levels = compute_storey_levels(&snapshot);
    let ground = bodies(&snapshot, "st-ground", &levels);
    let south = ground.iter().find(|body| body.id == "w-south").expect("wall");
    assert!(close(south.area(), 8.0 * 0.3), "mitered at both ends: a trapezoid whose faces are 8.3 and 7.7 long");
    let part = ground.iter().find(|body| body.id == "w-part").expect("wall");
    assert!(close(part.area(), 2.85 * 0.15), "butted to the near face of the south wall");
    assert!(close(part.bounds[1], 0.15));
}

#[semio_framework_async_macros::async_test]
async fn a_sloped_slab_spans_the_rise_across_its_footprint() {
    let mut snapshot = house();
    snapshot.slabs.get_mut("sl-ground").expect("slab").slope = Some(Slope { direction: 0.0, angle: 0.1 });
    let levels = compute_storey_levels(&snapshot);
    let (low, high) = slab_span(&snapshot, &snapshot.slabs["sl-ground"], &levels["st-ground"]);
    assert!(close(low, -0.22) && close(high, 8.0 * 0.1_f64.tan()));
}

#[semio_framework_async_macros::async_test]
async fn bodies_with_a_non_finite_extent_do_not_exist() {
    let region = Region::new(&[[0.0, 0.0], [1.0, 0.0], [1.0, f64::NAN], [0.0, 1.0]], &[]);
    assert!(Body::new("x", BodyKind::Wall, "s", 0.0, 1.0, vec![region]).is_none());
    assert!(Body::new("x", BodyKind::Wall, "s", 0.0, 1.0, Vec::new()).is_none());
}

#[semio_framework_async_macros::async_test]
async fn the_storeys_an_element_set_depends_on_are_its_own_then_the_targeted_ones() {
    let mut snapshot = house();
    assert_eq!(level_storeys(&snapshot, "st-first"), vec!["st-first"]);
    snapshot.walls.get_mut("w-first-west").expect("wall").top = TopConstraint::Storey { storey: "st-ground".into(), offset: 0.5 };
    snapshot.columns.get_mut("c-1").expect("column").storey = "st-first".into();
    assert_eq!(level_storeys(&snapshot, "st-first"), vec!["st-first", "st-ground"]);
    assert_eq!(level_storeys(&snapshot, "st-nowhere"), Vec::<String>::new());
}


#[semio_framework_async_macros::async_test]
async fn the_plan_dependency_of_a_storey_changes_with_its_elements_and_not_with_the_others_or_with_names() {
    use crate::standards::v1::subsets::any::schema::inferences::plan_linework::dependency;
    let before = dependency(&house(), "st-ground");
    let mut edited = house();
    edited.columns.get_mut("c-1").expect("column").name = "Renamed".into();
    assert_eq!(before, dependency(&edited, "st-ground"), "no inference reads a name");
    let mut other = house();
    other.beams.get_mut("b-1").expect("beam").storey = "st-first".into();
    assert_ne!(before, dependency(&other, "st-ground"), "a beam that leaves the storey is part of the difference");
    let mut retyped = house();
    retyped.wall_types.get_mut("wt-300").expect("type").name = "Other".into();
    assert_eq!(before, dependency(&retyped, "st-ground"), "wall types are read through the layouts, not by the plan");
}
