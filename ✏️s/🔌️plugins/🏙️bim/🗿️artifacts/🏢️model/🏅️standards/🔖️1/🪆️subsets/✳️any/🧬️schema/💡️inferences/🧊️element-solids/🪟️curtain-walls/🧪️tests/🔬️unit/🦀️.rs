use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, fixtures::case};
use crate::{CurtainGrid, CurtainPanel, CurtainPanelOverride, ModelSnapshot, Opening, OpeningKind, Profile};

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-12 * right.abs().max(1.0)
}

fn kind(snapshot: &mut ModelSnapshot) -> &mut CurtainWallType {
    snapshot.curtain_wall_types.get_mut("cwt-1").expect("the type of the wall")
}

fn overridden(snapshot: &mut ModelSnapshot, id: &str, u: u32, v: u32, panel: CurtainPanel) {
    snapshot.curtain_panel_overrides.insert(id.into(), CurtainPanelOverride { curtain: "cw-1".into(), u, v, panel });
}

#[test]
fn members_sit_on_their_grid_line_and_the_outer_ones_stay_inside() {
    let (first, last, middle) = (member(6.0, 0.0, 0.06), member(6.0, 6.0, 0.06), member(6.0, 3.0, 0.06));
    assert!(close(first.0, 0.0) && close(first.1, 0.06));
    assert!(close(last.0, 5.94) && close(last.1, 6.0));
    assert!(close(middle.0, 2.97) && close(middle.1, 3.03));
}

#[test]
fn an_opening_cuts_up_to_four_rectangles_out_of_a_cell() {
    let cut = OpeningCut { s_min: 1.0, s_max: 2.0, z_min: 0.5, z_max: 1.5 };
    let parts = without([0.0, 3.0, 0.0, 2.0], &cut);
    assert_eq!(parts.len(), 4);
    let area: f64 = parts.iter().map(|part| (part[1] - part[0]) * (part[3] - part[2])).sum();
    assert!(close(area, 6.0 - 1.0), "the cut removes exactly its overlap");
    assert_eq!(without([0.0, 1.0, 0.0, 1.0], &cut), vec![[0.0, 1.0, 0.0, 1.0]], "a disjoint cut changes nothing");
    assert!(without([1.2, 1.8, 0.6, 1.4], &cut).is_empty(), "a cell inside the cut vanishes");
    assert!(touched([0.0, 3.0, 0.0, 2.0], &[cut]) && !touched([0.0, 1.0, 0.0, 1.0], &[cut]));
    let two = remaining([0.0, 4.0, 0.0, 1.0], &[OpeningCut { s_min: 1.0, s_max: 1.5, z_min: -1.0, z_max: 2.0 }, OpeningCut { s_min: 2.5, s_max: 3.0, z_min: -1.0, z_max: 2.0 }]);
    assert_eq!(two.len(), 3);
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
async fn the_border_mullions_use_the_border_section_and_the_interior_ones_the_interior_section() {
    let (mut snapshot, _) = case("curtain-grid");
    kind(&mut snapshot).border_mullion = Profile::Rectangle { width: 0.1, depth: 0.2 };
    let wall = &compute_element_solids(&snapshot)["cw-1"];
    let (interior, border, depth_border, depth_interior) = (0.06, 0.1, 0.2, 0.12);
    let vertical_gap = 6.0 - (3.0 * interior + 2.0 * border);
    let horizontal_gap = 3.0 - (2.0 * interior + 2.0 * border);
    let verticals = 3.0 * interior * depth_interior * 3.0 + 2.0 * border * depth_border * 3.0;
    let horizontals = (2.0 * interior * depth_interior + 2.0 * border * depth_border) * vertical_gap;
    let panels = vertical_gap * horizontal_gap * PANEL_THICKNESS;
    assert!(close(wall.volume, verticals + horizontals + panels), "{} against {}", wall.volume, verticals + horizontals + panels);
    assert!(close(wall.bounds.max.y, 0.1) && close(wall.bounds.min.y, -0.1), "the outer mullions are as deep as the border section");
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
    kind(&mut snapshot).u_grid = CurtainGrid::Spacing { spacing: 3.0 };
    let coarse = compute_element_solids(&snapshot);
    assert!(coarse["cw-1"].triangle_count() < after["cw-1"].triangle_count());
}

#[semio_framework_async_macros::async_test]
async fn explicit_grid_lines_cut_unequal_cells() {
    let (mut snapshot, _) = case("curtain-grid");
    snapshot.curtain_walls.get_mut("cw-1").expect("curtain wall").u_grid = Some(CurtainGrid::Lines { positions: vec![1.0, 5.0] });
    let wall = &compute_element_solids(&snapshot)["cw-1"];
    let (width, depth, run, rise) = (0.06, 0.12, 6.0 - 4.0 * 0.06, 3.0 - 4.0 * 0.06);
    let mullions = 4.0 * width * depth * 3.0 + 4.0 * width * depth * run;
    assert!(close(wall.volume, mullions + run * rise * PANEL_THICKNESS), "unequal cells keep the same total panel area");
    assert_eq!(wall.triangle_count(), 12 * (3 * 3) + 12 * (4 + 4 * 3), "three cells per row, three rows");
}

#[semio_framework_async_macros::async_test]
async fn round_and_profile_mullions_use_their_outline() {
    let (mut snapshot, _) = case("curtain-grid");
    kind(&mut snapshot).interior_mullion = Profile::Circle { diameter: 0.08 };
    kind(&mut snapshot).border_mullion = Profile::Circle { diameter: 0.08 };
    let round = compute_element_solids(&snapshot);
    assert!(round["cw-1"].volume > 0.0 && round["cw-1"].triangle_count() > 400);
    kind(&mut snapshot).interior_mullion = Profile::IShape { width: 0.08, depth: 0.12, web: 0.01, flange: 0.01 };
    kind(&mut snapshot).border_mullion = Profile::IShape { width: 0.08, depth: 0.12, web: 0.01, flange: 0.01 };
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

#[semio_framework_async_macros::async_test]
async fn an_override_changes_the_panel_of_exactly_its_cell() {
    let (mut snapshot, _) = case("curtain-grid");
    let plain = compute_element_solids(&snapshot)["cw-1"].volume;
    let (cell_width, cell_height) = (1.5 - 0.06, 1.0 - 0.03 - 0.06);
    overridden(&mut snapshot, "ov-solid", 1, 0, CurtainPanel::Solid { material: "m-wood".into() });
    let solid = &compute_element_solids(&snapshot)["cw-1"];
    assert!(close(solid.volume - plain, cell_width * cell_height * (SOLID_THICKNESS.min(0.12) - PANEL_THICKNESS)), "only the thickness of one cell changes");
    assert!(solid.groups.iter().any(|group| group.part == "panel" && group.material == "m-wood"));
    overridden(&mut snapshot, "ov-solid", 1, 0, CurtainPanel::Empty);
    let empty = &compute_element_solids(&snapshot)["cw-1"];
    assert!(close(plain - empty.volume, cell_width * cell_height * PANEL_THICKNESS), "an empty cell has no panel");
}

#[semio_framework_async_macros::async_test]
async fn a_door_panel_fills_its_cell_with_a_frame_and_a_leaf() {
    let (mut snapshot, _) = case("curtain-grid");
    overridden(&mut snapshot, "ov-door", 2, 0, CurtainPanel::Door { door_type: "dr-1".into() });
    let wall = &compute_element_solids(&snapshot)["cw-1"];
    let parts_of = |part: &str| wall.groups.iter().filter(|group| group.part == part).map(|group| group.material.as_str()).collect::<Vec<_>>();
    assert_eq!(parts_of("frame"), ["m-wood"]);
    assert_eq!(parts_of("leaf"), ["m-wood"]);
    overridden(&mut snapshot, "ov-door", 2, 0, CurtainPanel::Window { window_type: "wn-1".into() });
    let window = &compute_element_solids(&snapshot)["cw-1"];
    assert!(window.groups.iter().any(|group| group.part == "glass" && group.material == "m-glass"), "the glass of a window panel is the glass material of the type");
    overridden(&mut snapshot, "ov-door", 2, 0, CurtainPanel::Door { door_type: "dr-9".into() });
    let missing = &compute_element_solids(&snapshot)["cw-1"];
    assert!(missing.groups.iter().all(|group| group.part != "leaf"), "a door panel without its type draws nothing");
}

#[semio_framework_async_macros::async_test]
async fn an_override_outside_the_grid_is_ignored_by_the_solid() {
    let (mut snapshot, _) = case("curtain-grid");
    let plain = compute_element_solids(&snapshot)["cw-1"].volume;
    overridden(&mut snapshot, "ov-far", 9, 9, CurtainPanel::Empty);
    assert!(close(compute_element_solids(&snapshot)["cw-1"].volume, plain));
}

#[semio_framework_async_macros::async_test]
async fn a_hosted_opening_cuts_the_panels_it_overlaps() {
    let (mut snapshot, _) = case("curtain-grid");
    let plain = compute_element_solids(&snapshot)["cw-1"].volume;
    snapshot.openings.insert("o-1".into(), Opening { host: "cw-1".into(), kind: OpeningKind::Void { width: 1.0, height: 0.5 }, offset: 2.25, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: "Hole".into(), reveal_depth: None, reveal_material: None });
    let cut = compute_element_solids(&snapshot)["cw-1"].volume;
    let removed = ((2.75 - 1.75) * (0.5 - 0.06)) * PANEL_THICKNESS;
    assert!(close(plain - cut, removed), "the void removes exactly its rectangle of the pane: {} against {removed}", plain - cut);
}
