use super::*;
use crate::TopConstraint;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, fixtures::case, ElementSolid};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout;
use semio_framework_geometry::mesh::TriMesh;
use std::collections::BTreeMap;

fn solids(name: &str) -> (ModelSnapshot, BTreeMap<String, ElementSolid>) {
    let (snapshot, _) = case(name);
    let solids = compute_element_solids(&snapshot);
    (snapshot, solids)
}

fn close(left: f64, right: f64, tolerance: f64) -> bool {
    (left - right).abs() <= tolerance * right.abs().max(1.0)
}

fn group(solid: &ElementSolid, index: usize) -> TriMesh {
    let whole = solid.mesh();
    let mut mesh = TriMesh::new();
    for (triangle, owner) in solid.face_groups.iter().enumerate() {
        if *owner as usize == index {
            let [a, b, c] = whole.triangle(triangle);
            mesh.push_triangle(a, b, c);
        }
    }
    mesh
}

fn bounds(solid: &ElementSolid) -> ([f64; 3], [f64; 3]) {
    ([solid.bounds.min.x, solid.bounds.min.y, solid.bounds.min.z], [solid.bounds.max.x, solid.bounds.max.y, solid.bounds.max.z])
}

#[semio_framework_async_macros::async_test]
async fn a_plain_wall_is_a_watertight_prism_with_the_analytic_measures() {
    let (_, solids) = solids("straight-openings");
    let wall = &solids["w-plain"];
    assert!(close(wall.volume, 5.0 * 0.2 * 2.5, 1e-12), "volume {}", wall.volume);
    assert!(close(wall.area, 2.0 * 5.0 * 2.5 + 2.0 * 0.2 * 2.5 + 2.0 * 5.0 * 0.2, 1e-12), "area {}", wall.area);
    let (low, high) = bounds(wall);
    assert!(low.iter().zip([0.0, -0.1, 0.0]).all(|(a, b)| close(*a, b, 1e-12)) && high.iter().zip([5.0, 0.1, 2.5]).all(|(a, b)| close(*a, b, 1e-12)), "{low:?} {high:?}");
    assert!(wall.mesh().is_watertight());
    assert_eq!(wall.triangle_count(), 12);
    assert_eq!((wall.family, wall.groups.len()), (SolidFamily::Wall, 1));
    assert_eq!((wall.groups[0].part.as_str(), wall.groups[0].material.as_str(), wall.groups[0].layer), ("layer", "m-brick", 0));
}

#[semio_framework_async_macros::async_test]
async fn a_window_and_a_void_are_holes_with_reveals() {
    let (_, solids) = solids("straight-openings");
    let wall = &solids["w-window"];
    assert!(close(wall.volume, 5.0 * 0.2 * 2.5 - (1.2 * 1.0 + 0.5 * 0.5) * 0.2, 1e-12), "volume {}", wall.volume);
    let reveals = 2.0 * (1.2 + 1.0) * 0.2 + 2.0 * (0.5 + 0.5) * 0.2;
    assert!(close(wall.area, 2.0 * (5.0 * 2.5 - 1.2 - 0.25) + 2.0 * 0.2 * 2.5 + 2.0 * 5.0 * 0.2 + reveals, 1e-12), "area {}", wall.area);
    assert!(wall.mesh().is_watertight());
}

#[semio_framework_async_macros::async_test]
async fn a_door_notches_the_bottom_edge() {
    let (_, solids) = solids("straight-openings");
    let wall = &solids["w-door"];
    assert!(close(wall.volume, 6.0 * 0.2 * 2.5 - (0.9 * 2.1 + 1.2 * 1.0) * 0.2, 1e-12), "volume {}", wall.volume);
    let reveals = (2.0 * 2.1 + 0.9) * 0.2 + 2.0 * (1.2 + 1.0) * 0.2;
    let expected = 2.0 * (6.0 * 2.5 - 0.9 * 2.1 - 1.2) + 2.0 * 0.2 * 2.5 + 2.0 * 6.0 * 0.2 - 0.9 * 0.2 + reveals;
    assert!(close(wall.area, expected, 1e-12), "area {} against {expected}", wall.area);
    assert!(wall.mesh().is_watertight());
}

#[semio_framework_async_macros::async_test]
async fn every_layer_is_a_closed_group_with_its_material() {
    let (_, solids) = solids("straight-openings");
    let wall = &solids["w-layered"];
    let net = 6.0 * 3.0 - (1.6 * 2.1 + 1.2 * 1.0);
    assert!(close(wall.volume, net * 0.3, 1e-12), "volume {}", wall.volume);
    assert_eq!(wall.groups.iter().map(|g| (g.material.as_str(), g.layer)).collect::<Vec<_>>(), vec![("m-brick", 0), ("m-insulation", 1)]);
    for (index, thickness) in [0.2, 0.1].into_iter().enumerate() {
        let layer = group(wall, index);
        assert!(close(layer.volume(), net * thickness, 1e-12), "layer {index}: {}", layer.volume());
        assert!(layer.is_watertight(), "layer {index} is closed");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_location_line_decides_which_side_the_body_extends_to() {
    let (_, solids) = solids("straight-openings");
    let (exterior, interior) = (&solids["w-exterior"], &solids["w-interior"]);
    let (low, high) = bounds(exterior);
    assert!(close(low[1], 0.0, 1e-12) && close(high[1], 0.3, 1e-12), "exterior {low:?} {high:?}");
    let (low, high) = bounds(interior);
    assert!(close(low[1], 5.0 - 0.3, 1e-12) && close(high[1], 5.0, 1e-12), "interior {low:?} {high:?}");
    let first = group(exterior, 0).bounds().expect("layer");
    assert!(close(first.0[1], 0.1, 1e-12) && close(first.1[1], 0.3, 1e-12), "the first layer sits on the left (interior) face: {first:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_diagonal_wall_keeps_its_volume_and_cuts_its_hole_along_the_axis() {
    let (_, solids) = solids("straight-openings");
    let wall = &solids["w-diagonal"];
    assert!(close(wall.volume, 5.0 * 0.2 * 2.5 - 1.2 * 1.0 * 0.2, 1e-12), "volume {}", wall.volume);
    assert!(wall.mesh().is_watertight());
    let along = |point: &[f64]| ((point[0] - 0.0) * 0.8 + (point[1] - 40.0) * 0.6, (point[0] - 0.0) * -0.6 + (point[1] - 40.0) * 0.8);
    let reveal = wall.positions.chunks_exact(3).filter(|p| p[2] > 0.89 && p[2] < 1.91).map(|p| along(p).0).fold((f64::MAX, f64::MIN), |(lo, hi), s| (lo.min(s), hi.max(s)));
    assert!(close(reveal.0, 1.9, 1e-9) && close(reveal.1, 3.1, 1e-9), "hole s range {reveal:?}");
}

#[semio_framework_async_macros::async_test]
async fn an_invalid_opening_cuts_nothing() {
    let (_, solids) = solids("straight-openings");
    assert!(close(solids["w-plain"].volume, 2.5, 1e-12), "the window of o-bad-1 is outside its host");
}

#[semio_framework_async_macros::async_test]
async fn a_curved_wall_is_an_annular_sector_within_the_chord_tolerance() {
    let (_, solids) = solids("arc-window");
    let wall = &solids["w-arc"];
    let expected = std::f64::consts::FRAC_PI_2 * 4.0 * 0.2 * 2.5 - 1.2 * 1.0 * 0.2;
    assert!(close(wall.volume, expected, 1e-4), "volume {} against {expected}", wall.volume);
    assert!(wall.volume < expected, "an inscribed tessellation never exceeds the exact volume");
    assert!(wall.mesh().is_watertight());
    let (low, high) = bounds(wall);
    assert!(close(low[2], 0.0, 1e-12) && close(high[2], 2.5, 1e-12));
    assert!(close(high[0], 4.1, 1e-9) && close(high[1], 4.1, 1e-9), "the arc ends reach the outer radius: {high:?}");
}

#[semio_framework_async_macros::async_test]
async fn joined_walls_are_trimmed_and_tile_the_union_exactly() {
    let (snapshot, solids) = solids("room-joins");
    let total: f64 = snapshot.walls.keys().map(|id| solids[id].volume).sum();
    assert!(close(total, (4.0 + 0.2 * 3.8) * 2.5, 1e-12), "total {total}");
    assert!(close(solids["w-south"].volume, 6.0 * 0.2 * 2.5, 1e-12), "mitered corners keep the axis length times thickness");
    assert!(close(solids["w-partition"].volume, 3.8 * 0.2 * 2.5, 1e-12), "the T branch ends on the faces of the walls it butts against");
    let (low, high) = bounds(&solids["w-south"]);
    assert!(close(low[0], -0.1, 1e-12) && close(high[0], 6.1, 1e-12), "the outer face is extended to the miter: {low:?} {high:?}");
    for (id, solid) in &solids {
        assert!(solid.mesh().is_watertight(), "{id} is closed");
    }
}

#[semio_framework_async_macros::async_test]
async fn every_solid_volume_matches_the_footprint_volume_of_the_layout() {
    for name in ["straight-openings", "room-joins"] {
        let (snapshot, solids) = solids(name);
        let layouts = compute_wall_layout(&snapshot);
        for (id, layout) in layouts.iter().filter(|(id, _)| !snapshot.openings.values().any(|opening| &opening.host == *id)) {
            assert!(close(solids[id].volume, layout.volume, 1e-9), "{name}/{id}: solid {} layout {}", solids[id].volume, layout.volume);
        }
    }
    let (snapshot, solids) = solids("arc-window");
    assert!(close(solids["w-arc"].volume + 1.2 * 0.2, compute_wall_layout(&snapshot)["w-arc"].volume, 1e-4));
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_stretches_the_wall_and_moving_an_opening_moves_its_hole() {
    let (mut snapshot, before) = solids("straight-openings");
    snapshot.storeys.get_mut("st-1").expect("storey").height = 3.4;
    let after = compute_element_solids(&snapshot);
    assert!(close(after["w-layered"].bounds.max.z - before["w-layered"].bounds.max.z, 0.4, 1e-12), "the StoreyTop wall grows");
    assert!(close(after["w-layered"].volume - before["w-layered"].volume, 6.0 * 0.4 * 0.3, 1e-12));
    assert_eq!(after["w-plain"], before["w-plain"], "an unconnected wall keeps its mesh");
    snapshot.storeys.get_mut("st-1").expect("storey").height = 3.0;
    snapshot.openings.get_mut("o-win-1").expect("window").offset += 0.5;
    let moved = compute_element_solids(&snapshot);
    assert!(close(moved["w-window"].volume, before["w-window"].volume, 1e-12), "the hole keeps its size");
    assert_ne!(moved["w-window"].positions, before["w-window"].positions);
    let hole = |solid: &ElementSolid| solid.positions.chunks_exact(3).filter(|p| p[2] > 0.85 && p[2] < 1.95 && p[0] > 1.5 && p[0] < 3.7).map(|p| p[0]).fold((f64::MAX, f64::MIN), |(lo, hi), x| (lo.min(x), hi.max(x)));
    let (start, end) = (hole(&before["w-window"]), hole(&moved["w-window"]));
    assert!(close(end.0 - start.0, 0.5, 1e-9) && close(end.1 - start.1, 0.5, 1e-9), "the reveal moved by the offset: {start:?} {end:?}");
}

#[semio_framework_async_macros::async_test]
async fn walls_without_a_type_a_storey_or_a_height_have_no_solid() {
    let (mut snapshot, _) = solids("straight-openings");
    snapshot.walls.get_mut("w-plain").expect("wall").wall_type = "wt-missing".into();
    snapshot.walls.get_mut("w-window").expect("wall").top = TopConstraint::Unconnected { height: 0.0 };
    snapshot.walls.get_mut("w-door").expect("wall").storey = "st-missing".into();
    let solids = compute_element_solids(&snapshot);
    assert!(!solids.contains_key("w-plain") && !solids.contains_key("w-window") && !solids.contains_key("w-door"));
    assert!(solids.contains_key("w-layered"));
}

const JOINS: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🔗️wall-joins/📸️snapshot/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn every_wall_of_the_join_fixture_fills_exactly_its_trimmed_footprint() {
    let snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(JOINS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the join fixture decodes");
    let (layouts, solids) = (compute_wall_layout(&snapshot), compute_element_solids(&snapshot));
    assert_eq!(solids.len(), snapshot.walls.len());
    for (id, wall) in &snapshot.walls {
        let curved = matches!(wall.axis, crate::Axis::Arc { .. }) || layouts[id].footprint.iter().any(|vertex| vertex.bulge.abs() > 1e-12);
        let tolerance = if curved { 1e-4 } else { 1e-9 };
        assert!(close(solids[id].volume, layouts[id].volume, tolerance), "{id}: solid {} against footprint {}", solids[id].volume, layouts[id].volume);
        let layers = snapshot.wall_types[&wall.wall_type].layers.len();
        for index in 0..layers {
            assert!(group(&solids[id], index).is_watertight(), "{id}: layer {index} is closed");
        }
        let (low, high) = bounds(&solids[id]);
        let footprint_x = layouts[id].footprint.iter().map(|vertex| vertex.point.x).fold((f64::MAX, f64::MIN), |(lo, hi), x| (lo.min(x), hi.max(x)));
        assert!(low[0] >= footprint_x.0 - 1e-3 && high[0] <= footprint_x.1 + 1e-3, "{id}: solid x range {} {} against footprint {footprint_x:?}", low[0], high[0]);
    }
}
