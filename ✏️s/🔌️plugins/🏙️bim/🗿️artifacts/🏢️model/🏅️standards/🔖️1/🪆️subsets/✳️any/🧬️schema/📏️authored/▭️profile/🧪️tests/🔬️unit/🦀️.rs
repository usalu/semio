use super::*;

#[test]
fn profiles_flatten_to_centred_counter_clockwise_outlines() {
    let rectangle = profile_polygon(&Profile::Rectangle { width: 0.4, depth: 0.2 });
    assert_eq!(profile_extents(&rectangle), (0.4, 0.2));
    let circle = profile_polygon(&Profile::Circle { diameter: 1.0 });
    assert!(circle.iter().all(|p| (p.x.hypot(p.y) - 0.5).abs() < 1e-12) && circle.len() >= 16);
    let area = |ring: &[Point]| (0..ring.len()).map(|i| ring[i].x * ring[(i + 1) % ring.len()].y - ring[(i + 1) % ring.len()].x * ring[i].y).sum::<f64>() / 2.0;
    assert!((area(&profile_polygon(&Profile::IShape { width: 0.2, depth: 0.3, web: 0.01, flange: 0.02 })) - (2.0 * 0.2 * 0.02 + 0.01 * 0.26)).abs() < 1e-12);
    assert_eq!(profile_extents(&[]), (0.0, 0.0));
}

#[test]
fn a_family_profile_has_no_authored_outline() {
    assert!(profile_polygon(&Profile::Family { family: "f".into() }).is_empty());
}
