use super::*;
use crate::standards::v1::subsets::any::schema::inferences::ramp_runs::run_of;
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{TopConstraint, Vertex};
use std::f64::consts::{FRAC_PI_2, PI};

const EPS: f64 = 1e-9;

fn level() -> StoreyLevel {
    StoreyLevel { elevation: 0.0, top_elevation: 3.0, absolute_elevation: 0.0, absolute_top_elevation: 3.0 }
}

fn vertex(x: f64, y: f64, bulge: f64) -> Vertex {
    Vertex { point: crate::Point2 { x, y }, bulge }
}

fn ramp(path: Vec<Vertex>, rise: f64) -> Ramp {
    Ramp {
        storey: "st-ground".into(),
        path,
        width: 1.2,
        landing_start: 1.5,
        landing_end: 1.5,
        landing_turn: 1.5,
        max_slope: 1.0 / 12.0,
        thickness: 0.2,
        material: "m-concrete".into(),
        base_offset: 0.0,
        top: TopConstraint::Unconnected { height: rise },
        railing_left: false,
        railing_right: false,
        name: "Ramp".into(),
    }
}

fn solid_of(ramp: &Ramp) -> ElementSolid {
    ramp_solid(ramp, &run_of(ramp, &level(), None))
}

#[test]
fn a_straight_ramp_is_a_closed_slab_of_plan_area_times_thickness() {
    let straight = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(10.0, 0.0, 0.0)], 0.5);
    let solid = solid_of(&straight);
    assert_eq!(solid.family, SolidFamily::Ramp);
    assert!((solid.volume - 1.2 * 10.0 * 0.2).abs() < 1e-9);
    assert!((solid.bounds.min.z + 0.2).abs() < EPS);
    assert!((solid.bounds.max.z - 0.5).abs() < EPS);
    assert!((solid.bounds.min.x).abs() < EPS && (solid.bounds.max.x - 10.0).abs() < EPS);
    assert!((solid.bounds.max.y - 0.6).abs() < EPS && (solid.bounds.min.y + 0.6).abs() < EPS);
}

#[test]
fn the_walking_surface_area_exceeds_the_plan_area_by_the_slope() {
    let straight = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(10.0, 0.0, 0.0)], 0.5);
    let slope = 0.5 / 7.0;
    let expected = 1.2 * (3.0 + 7.0 * (1.0 + slope * slope).sqrt());
    let solid = solid_of(&straight);
    let top: f64 = {
        let mesh = solid.mesh();
        (0..mesh.triangle_count())
            .map(|index| {
                let [a, b, c] = mesh.triangle(index);
                let n = semio_framework_geometry::vector::cross3(semio_framework_geometry::vector::sub3(b, a), semio_framework_geometry::vector::sub3(c, a));
                (n[2] > 1e-12).then(|| semio_framework_geometry::vector::length3(n) / 2.0).unwrap_or(0.0)
            })
            .sum()
    };
    assert!((top - expected).abs() < 1e-9, "{top} vs {expected}");
}

#[test]
fn a_bent_ramp_has_the_volume_of_its_mitred_strip() {
    let bent = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(6.0, 0.0, 0.0), vertex(6.0, 5.0, 0.0)], 0.6);
    assert!((solid_of(&bent).volume - 1.2 * 11.0 * 0.2).abs() < 1e-9);
}

#[test]
fn a_curved_ramp_has_the_volume_of_its_arc_strip_within_the_chord_tolerance() {
    let quarter = ramp(vec![vertex(0.0, 0.0, (PI / 8.0).tan()), vertex(5.0, 5.0, 0.0)], 0.3);
    let expected = 1.2 * 5.0 * FRAC_PI_2 * 0.2;
    let volume = solid_of(&quarter).volume;
    assert!((volume - expected).abs() / expected < 2e-4, "{volume} vs {expected}");
}

#[test]
fn a_descending_ramp_stays_a_closed_positive_solid() {
    let down = Ramp { base_offset: 0.5, top: TopConstraint::Unconnected { height: -0.5 }, ..ramp(vec![vertex(0.0, 0.0, 0.0), vertex(10.0, 0.0, 0.0)], 0.0) };
    let solid = solid_of(&down);
    assert!((solid.volume - 2.4).abs() < 1e-9);
    assert!((solid.bounds.max.z - 0.5).abs() < EPS && (solid.bounds.min.z + 0.2).abs() < EPS);
}

#[test]
fn the_stations_break_at_every_landing_and_flight_end() {
    let straight = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(10.0, 0.0, 0.0)], 0.5);
    let run = run_of(&straight, &level(), None);
    let stations = stations_of(&strip_of(&straight), &run);
    let marks: Vec<f64> = stations.iter().map(|station| station.s).collect();
    assert_eq!(marks.len(), 4);
    for expected in [0.0, 1.5, 8.5, 10.0] {
        assert!(marks.iter().any(|mark| (mark - expected).abs() < EPS), "{marks:?}");
    }
    assert!(stations.iter().all(|station| (station.z - run.z_at(station.s)).abs() < EPS));
}

#[test]
fn a_side_railing_adds_posts_and_a_rail_without_changing_the_slab() {
    let plain = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(10.0, 0.0, 0.0)], 0.5);
    let fenced = Ramp { railing_left: true, railing_right: true, ..plain.clone() };
    let (bare, rails) = (solid_of(&plain), solid_of(&fenced));
    assert!(bare.groups.iter().all(|group| group.part == parts::BODY));
    assert!(rails.groups.iter().any(|group| group.part == parts::POST) && rails.groups.iter().any(|group| group.part == parts::RAIL));
    assert!(rails.triangle_count() > bare.triangle_count());
    assert!((rails.bounds.max.z - 1.5).abs() < EPS, "{}", rails.bounds.max.z);
}

#[test]
fn rail_posts_stand_at_both_ends_at_every_corner_and_within_the_spacing() {
    let path = vec![RailPoint { x: 0.0, y: 0.0, z: 0.0 }, RailPoint { x: 3.0, y: 0.0, z: 0.0 }, RailPoint { x: 3.0, y: 4.0, z: 0.0 }];
    let posts = post_positions(&path, 1.5);
    assert_eq!(posts.len(), 3 + 3);
    assert!(posts.iter().any(|(at, _, _)| (at.x - 3.0).abs() < EPS && at.y.abs() < EPS));
    assert!(posts.windows(2).all(|pair| pair[0].0.distance(pair[1].0) <= 1.5 + 1e-9));
}

#[test]
fn a_slope_change_is_a_corner_of_the_rail() {
    let path = vec![RailPoint { x: 0.0, y: 0.0, z: 0.0 }, RailPoint { x: 2.0, y: 0.0, z: 0.0 }, RailPoint { x: 4.0, y: 0.0, z: 0.2 }];
    let posts = post_positions(&path, 10.0);
    assert_eq!(posts.len(), 3);
    assert!((posts[1].2).abs() < EPS && (posts[2].2 - 0.2).abs() < EPS);
}

#[test]
fn the_rail_follows_the_height_of_its_path() {
    let path = vec![RailPoint { x: 0.0, y: 0.0, z: 0.0 }, RailPoint { x: 4.0, y: 0.0, z: 0.4 }];
    let mesh = rail_mesh(&path, &RailSpec::standard());
    let (low, high) = mesh.bounds().expect("a rail");
    assert!((high[2] - 1.4).abs() < EPS && (low[2] - (1.0 - RailSpec::standard().rail.1)).abs() < EPS);
    assert!(mesh.signed_volume() > 0.0);
    assert!((mesh.volume() - 4.0 * RailSpec::standard().rail.0 * RailSpec::standard().rail.1).abs() < 1e-9);
}

#[test]
fn the_dependency_is_the_ramp_record() {
    let straight = ramp(vec![vertex(0.0, 0.0, 0.0), vertex(10.0, 0.0, 0.0)], 0.5);
    assert_eq!(dependency(&straight), dep_value(&straight));
}
