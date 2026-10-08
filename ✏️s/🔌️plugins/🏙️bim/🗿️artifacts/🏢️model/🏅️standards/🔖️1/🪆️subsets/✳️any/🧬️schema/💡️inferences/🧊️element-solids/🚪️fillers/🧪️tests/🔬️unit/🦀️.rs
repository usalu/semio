use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{compute_element_solids, fixtures::case, ElementSolid};
use std::collections::BTreeMap;

fn solids(name: &str) -> (ModelSnapshot, BTreeMap<String, ElementSolid>) {
    let (snapshot, _) = case(name);
    let solids = compute_element_solids(&snapshot);
    (snapshot, solids)
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-12 * right.abs().max(1.0)
}

fn parts(solid: &ElementSolid) -> Vec<(&str, &str)> {
    solid.groups.iter().map(|group| (group.part.as_str(), group.material.as_str())).collect()
}

fn box_of(solid: &ElementSolid) -> [f64; 6] {
    [solid.bounds.min.x, solid.bounds.min.y, solid.bounds.min.z, solid.bounds.max.x, solid.bounds.max.y, solid.bounds.max.z]
}

#[semio_framework_async_macros::async_test]
async fn a_window_is_a_frame_muntins_and_glass() {
    let (_, solids) = solids("straight-openings");
    let window = &solids["o-win-1"];
    let frame = 0.12 * (1.2 * 1.0 - 1.08 * 0.88);
    let muntin = 0.03 * 0.12 * 0.88;
    let glass = 2.0 * 0.525 * 0.02 * 0.88;
    assert!(close(window.volume, frame + muntin + glass), "volume {} against {}", window.volume, frame + muntin + glass);
    assert_eq!(window.family, SolidFamily::Window);
    assert_eq!(parts(window), vec![("frame", "m-wood"), ("glass", ""), ("muntin", "m-wood")]);
    let wanted = [1.9, 10.0 - 0.06, 0.9, 3.1, 10.0 + 0.06, 1.9];
    assert!(box_of(window).iter().zip(wanted).all(|(a, b)| close(*a, b)), "{:?}", box_of(window));
    for index in 0..window.groups.len() {
        let mut mesh = semio_framework_geometry::mesh::TriMesh::new();
        let whole = window.mesh();
        window.face_groups.iter().enumerate().filter(|(_, owner)| **owner as usize == index).for_each(|(triangle, _)| {
            let [a, b, c] = whole.triangle(triangle);
            mesh.push_triangle(a, b, c);
        });
        assert!(mesh.volume() > 0.0);
    }
}

#[semio_framework_async_macros::async_test]
async fn a_door_is_a_frame_and_its_leaves() {
    let (_, solids) = solids("straight-openings");
    let single = &solids["o-door-1"];
    assert!(close(single.volume, 0.1 * (0.9 * 0.05 + 2.0 * 0.05 * 2.05) + 0.8 * 0.04 * 2.05), "volume {}", single.volume);
    assert_eq!(single.family, SolidFamily::Door);
    assert_eq!(parts(single), vec![("frame", "m-wood"), ("leaf", "m-wood")]);
    let wanted = [1.05, 20.0 - 0.05, 0.0, 1.95, 20.0 + 0.05, 2.1];
    assert!(box_of(single).iter().zip(wanted).all(|(a, b)| close(*a, b)), "{:?}", box_of(single));
    let double = &solids["o-door-2"];
    assert!(close(double.volume, 0.1 * (1.6 * 0.05 + 2.0 * 0.05 * 2.05) + 1.5 * 0.04 * 2.05), "volume {}", double.volume);
    assert_eq!(double.triangle_count(), single.triangle_count() + 12, "a second leaf is one more box");
}

#[semio_framework_async_macros::async_test]
async fn voids_and_invalid_placements_have_no_filler() {
    let (_, solids) = solids("straight-openings");
    assert!(!solids.contains_key("o-void-1"), "a void is only a hole");
    assert!(!solids.contains_key("o-bad-1"), "the window outside its host is not built");
    let fillers: Vec<&String> = solids.iter().filter(|(_, solid)| matches!(solid.family, SolidFamily::Window | SolidFamily::Door)).map(|(id, _)| id).collect();
    assert_eq!(fillers, vec!["o-door-1", "o-door-2", "o-win-1", "o-win-2", "o-win-3", "o-win-4"]);
}

#[semio_framework_async_macros::async_test]
async fn a_window_follows_the_axis_of_a_rotated_host() {
    let (_, solids) = solids("straight-openings");
    let wanted = [2.0 - 0.516, 41.5 - 0.408, 0.9, 2.0 + 0.516, 41.5 + 0.408, 1.9];
    assert!(box_of(&solids["o-win-4"]).iter().zip(wanted).all(|(a, b)| (a - b).abs() < 1e-9), "{:?}", box_of(&solids["o-win-4"]));
}

#[semio_framework_async_macros::async_test]
async fn a_filler_is_centred_in_the_thickness_of_its_host_and_lifts_with_it() {
    let (mut snapshot, _) = solids("straight-openings");
    snapshot.openings.insert("o-win-ext".into(), Opening { host: "w-exterior".into(), kind: OpeningKind::Window { window_type: "wn-1".into() }, offset: 2.5, sill_override: Some(0.9), width: None, height: None, flip_hand: false, flip_facing: false, name: String::new() });
    let solids = compute_element_solids(&snapshot);
    let (low, high) = (solids["o-win-ext"].bounds.min.y, solids["o-win-ext"].bounds.max.y);
    assert!(close(low, 0.15 - 0.06) && close(high, 0.15 + 0.06), "the Exterior wall extends to the left, so the frame sits at +0.15: {low} {high}");
    snapshot.walls.get_mut("w-exterior").expect("wall").base_offset = 0.2;
    let lifted = compute_element_solids(&snapshot);
    assert!(close(lifted["o-win-ext"].bounds.min.z, solids["o-win-ext"].bounds.min.z + 0.2), "the sill is relative to the host base");
    snapshot.openings.get_mut("o-win-ext").expect("window").flip_facing = true;
    let flipped = compute_element_solids(&snapshot);
    let (low, high) = (flipped["o-win-ext"].bounds.min.y, flipped["o-win-ext"].bounds.max.y);
    assert!(close(low, 0.15 - 0.06) && close(high, 0.15 + 0.06), "a flipped facing keeps the frame in the thickness: {low} {high}");
}

#[semio_framework_async_macros::async_test]
async fn an_opening_override_resizes_the_filler() {
    let (mut snapshot, before) = solids("straight-openings");
    snapshot.openings.get_mut("o-win-3").expect("window").width = Some(1.0);
    let after = compute_element_solids(&snapshot);
    assert!(box_of(&after["o-win-3"])[3] - box_of(&after["o-win-3"])[0] - 1.0 < 1e-9);
    assert!(after["o-win-3"].volume < before["o-win-3"].volume);
}
