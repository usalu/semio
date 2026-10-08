use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, fixtures::case};
use crate::Profile;

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-12 * right.abs().max(1.0)
}

#[test]
fn members_sit_on_their_grid_line_and_the_outer_ones_stay_inside() {
    let (first, last, middle) = (member(6.0, 4, 0, 0.06), member(6.0, 4, 4, 0.06), member(6.0, 4, 2, 0.06));
    assert!(close(first.0, 0.0) && close(first.1, 0.06));
    assert!(close(last.0, 5.94) && close(last.1, 6.0));
    assert!(close(middle.0, 2.97) && close(middle.1, 3.03));
}

#[semio_framework_async_macros::async_test]
async fn a_curtain_wall_is_mullions_on_every_grid_line_and_a_panel_in_every_cell() {
    let (snapshot, _) = case("curtain-grid");
    let solids = compute_element_solids(&snapshot);
    let wall = &solids["cw-1"];
    let (width, depth, run, rise) = (0.06, 0.12, 6.0 - 5.0 * 0.06, 3.0 - 4.0 * 0.06);
    let mullions = 5.0 * width * depth * 3.0 + 4.0 * width * depth * run;
    let panels = run * rise * PANEL_THICKNESS;
    assert!(close(wall.volume, mullions + panels), "volume {} against {}", wall.volume, mullions + panels);
    assert_eq!(wall.family, SolidFamily::CurtainWall);
    assert_eq!(wall.groups.iter().map(|g| (g.part.as_str(), g.material.as_str())).collect::<Vec<_>>(), vec![("mullion", "m-steel"), ("panel", "m-glass")]);
    let (low, high) = (wall.bounds.min, wall.bounds.max);
    assert!(close(low.x, 0.0) && close(high.x, 6.0) && close(low.y, -0.06) && close(high.y, 0.06) && close(low.z, 0.0) && close(high.z, 3.0), "{low:?} {high:?}");
    let panel_triangles = 12 * 4 * 3;
    let mullion_triangles = 12 * (5 + 4 * 4);
    assert_eq!(wall.triangle_count(), panel_triangles + mullion_triangles);
}

#[semio_framework_async_macros::async_test]
async fn the_grid_follows_the_storey_height_and_the_spacings() {
    let (mut snapshot, _) = case("curtain-grid");
    snapshot.curtain_walls.get_mut("cw-1").expect("curtain wall").top = crate::TopConstraint::StoreyTop { offset: 0.0 };
    let before = compute_element_solids(&snapshot);
    snapshot.storeys.get_mut("st-1").expect("storey").height = 3.5;
    let after = compute_element_solids(&snapshot);
    assert!(close(after["cw-1"].bounds.max.z - before["cw-1"].bounds.max.z, 0.5));
    assert_ne!(after["cw-1"].triangle_count(), before["cw-1"].triangle_count(), "a fourth row appears");
    snapshot.curtain_walls.get_mut("cw-1").expect("curtain wall").u_spacing = 3.0;
    let coarse = compute_element_solids(&snapshot);
    assert!(coarse["cw-1"].triangle_count() < after["cw-1"].triangle_count());
}

#[semio_framework_async_macros::async_test]
async fn round_and_profile_mullions_use_their_outline() {
    let (mut snapshot, _) = case("curtain-grid");
    snapshot.curtain_walls.get_mut("cw-1").expect("curtain wall").mullion = Profile::Circle { diameter: 0.08 };
    let round = compute_element_solids(&snapshot);
    assert!(round["cw-1"].volume > 0.0 && round["cw-1"].triangle_count() > 400);
    snapshot.curtain_walls.get_mut("cw-1").expect("curtain wall").mullion = Profile::IShape { width: 0.08, depth: 0.12, web: 0.01, flange: 0.01 };
    let profile = compute_element_solids(&snapshot);
    assert!(profile["cw-1"].volume > 0.0 && profile["cw-1"].volume < round["cw-1"].volume * 4.0);
}

#[semio_framework_async_macros::async_test]
async fn a_curved_curtain_wall_has_a_solid_following_the_arc() {
    let (mut snapshot, _) = case("curtain-grid");
    snapshot.curtain_walls.get_mut("cw-1").expect("curtain wall").axis = crate::Axis::Arc { start: crate::Point2 { x: 4.0, y: 0.0 }, end: crate::Point2 { x: 0.0, y: 4.0 }, bulge: (std::f64::consts::PI / 8.0).tan() };
    let solids = compute_element_solids(&snapshot);
    let wall = &solids["cw-1"];
    assert!(wall.volume > 0.0 && wall.bounds.max.x > 3.9 && wall.bounds.max.y > 3.9 && wall.bounds.min.x < 0.1);
}
