use super::*;
use crate::standards::v1::subsets::any::schema::inferences::ramp_runs::run_of as ramp_run_of;
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::run_of as stair_run_of;
use crate::{Infill, Point2, Stair, StairFlight, TopConstraint, Turn, Vertex};

const EPS: f64 = 1e-9;

fn level() -> StoreyLevel {
    StoreyLevel { elevation: 0.0, top_elevation: 3.0, absolute_elevation: 0.0, absolute_top_elevation: 3.0 }
}

fn stair(flight: StairFlight) -> Stair {
    Stair {
        storey: "st".into(),
        start: Point2 { x: 0.0, y: 0.0 },
        direction: 0.0,
        width: 1.0,
        flight,
        top: TopConstraint::Unconnected { height: 2.8 },
        max_riser: 0.18,
        min_tread: 0.25,
        stringer: crate::STANDARD_STRINGER,
        nosing: 0.0,
        tread_thickness: 0.04,
        riser: crate::STANDARD_RISER,
        landing_depth: 1.0,
        name: String::new(),
    }
}

fn railing(host: Option<RailingHost>) -> Railing {
    Railing {
        storey: "st".into(),
        path: Vec::new(),
        height: 1.0,
        post_spacing: 1.2,
        profile: crate::standard_rail_profile(),
        post_profile: crate::standard_post_profile(),
        baluster: None,
        infill: Infill::None,
        material: "m".into(),
        base_offset: 0.0,
        host,
        name: String::new(),
    }
}

fn on(element: &str, side: HostSide, inset: f64) -> RailingHost {
    RailingHost { element: element.into(), side, edge: 0, inset }
}

#[test]
fn a_rail_on_a_straight_stair_follows_the_pitch_line_of_the_nosings() {
    let run = stair_run_of(&stair(StairFlight::Straight), &level(), None);
    let host = on("s", HostSide::Left, 0.0);
    let paths = host_paths(&railing(Some(host.clone())), &host, &Host::Stair(&run)).expect("a stair always yields paths");
    assert_eq!(paths.len(), 1);
    let path = &paths[0];
    assert_eq!(path.len(), 2);
    assert!((path[0].y - 0.5).abs() < EPS && (path[1].y - 0.5).abs() < EPS);
    assert!(path[0].x.abs() < EPS && (path[1].x - run.flights[0].length).abs() < EPS);
    assert!((path[0].z - run.riser_height).abs() < EPS);
    assert!((path[1].z - 2.8).abs() < 1e-9);
    let pitch = run.riser_height / run.tread;
    assert!(((path[1].z - path[0].z) / (path[1].x - path[0].x) - pitch).abs() < 1e-9);
}

#[test]
fn the_side_and_the_inset_move_the_rail_across_the_stair() {
    let run = stair_run_of(&stair(StairFlight::Straight), &level(), None);
    let right = on("s", HostSide::Right, 0.1);
    let paths = host_paths(&railing(Some(right.clone())), &right, &Host::Stair(&run)).expect("paths");
    assert!((paths[0][0].y + 0.4).abs() < EPS);
}

#[test]
fn the_base_offset_lifts_the_whole_rail() {
    let run = stair_run_of(&stair(StairFlight::Straight), &level(), None);
    let host = on("s", HostSide::Left, 0.0);
    let lifted = Railing { base_offset: 0.1, ..railing(Some(host.clone())) };
    let (plain, raised) = (host_paths(&railing(Some(host.clone())), &host, &Host::Stair(&run)).expect("paths"), host_paths(&lifted, &host, &Host::Stair(&run)).expect("paths"));
    assert!((raised[0][1].z - plain[0][1].z - 0.1).abs() < EPS);
}

#[test]
fn an_l_turn_stair_yields_one_path_per_flight_and_the_outer_rail_walks_the_landing() {
    let run = stair_run_of(&stair(StairFlight::LTurn { split: 0.5, turn: Turn::Left }), &level(), None);
    assert_eq!(run.flights.len(), 2);
    let inner = on("s", HostSide::Left, 0.0);
    let outer = on("s", HostSide::Right, 0.0);
    let inside = host_paths(&railing(Some(inner.clone())), &inner, &Host::Stair(&run)).expect("paths");
    let outside = host_paths(&railing(Some(outer.clone())), &outer, &Host::Stair(&run)).expect("paths");
    assert_eq!(inside.len(), 2);
    assert_eq!(outside.len(), 2);
    assert!(outside[0].len() > inside[0].len(), "the outer rail walks around the landing corner");
    let landing = run.landings[0].z;
    assert!((outside[0].last().expect("a point").z - landing).abs() < 1e-9);
    assert!((outside[1][0].z - (landing + run.riser_height)).abs() < 1e-9);
    let (a, b) = (outside[0].last().expect("a point"), &outside[1][0]);
    assert!((a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS);
}

#[test]
fn a_u_turn_stair_joins_its_flights_on_the_far_side_of_the_landing() {
    let run = stair_run_of(&stair(StairFlight::UTurn { gap: 0.2 }), &level(), None);
    let host = on("s", HostSide::Right, 0.0);
    let paths = host_paths(&railing(Some(host.clone())), &host, &Host::Stair(&run)).expect("paths");
    assert_eq!(paths.len(), 2);
    assert!(paths[0].len() >= 4);
}

#[test]
fn landing_walk_goes_counter_clockwise_on_the_right_and_clockwise_on_the_left() {
    use crate::standards::v1::subsets::any::schema::inferences::stair_runs::StairLanding;
    let landing = StairLanding { after_flight: 0, z: 1.0, centre: Point2 { x: 0.0, y: 0.0 }, direction: 0.0, width: 2.0, depth: 2.0 };
    let entry = (-1.0, -1.0);
    let exit = (1.0, 1.0);
    let ccw = landing_walk(&landing, entry, exit, HostSide::Right, 0.0);
    assert_eq!(ccw, vec![(1.0, -1.0)]);
    let cw = landing_walk(&landing, entry, exit, HostSide::Left, 0.0);
    assert_eq!(cw, vec![(-1.0, 1.0)]);
    assert!(landing_walk(&landing, entry, entry, HostSide::Right, 0.0).is_empty());
}

#[test]
fn a_spiral_rail_climbs_on_a_circle() {
    let run = stair_run_of(&stair(StairFlight::Spiral { radius: 1.6, sweep: std::f64::consts::TAU }), &level(), None);
    let host = on("s", HostSide::Left, 0.05);
    let paths = host_paths(&railing(Some(host.clone())), &host, &Host::Stair(&run)).expect("paths");
    assert_eq!(paths.len(), 1);
    let winder = run.flights[0].winder.expect("a winder");
    let radius = winder.inner_radius + 0.05;
    assert!(paths[0].iter().all(|p| ((p.x - winder.centre.x).hypot(p.y - winder.centre.y) - radius).abs() < 1e-9));
    assert!(paths[0].windows(2).all(|pair| pair[1].z >= pair[0].z));
    assert!((paths[0].last().expect("a point").z - 2.8).abs() < 1e-9);
}

fn ramp() -> Ramp {
    Ramp {
        storey: "st".into(),
        path: vec![Vertex { point: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 10.0, y: 0.0 }, bulge: 0.0 }],
        width: 1.2,
        landing_start: 1.5,
        landing_end: 1.5,
        landing_turn: 1.5,
        max_slope: 1.0 / 12.0,
        thickness: 0.2,
        material: "m".into(),
        base_offset: 0.0,
        top: TopConstraint::Unconnected { height: 0.5 },
        railing_left: false,
        railing_right: false,
        name: String::new(),
    }
}

#[test]
fn a_rail_on_a_ramp_follows_the_walking_surface_inside_its_edge() {
    let ramp = ramp();
    let run = ramp_run_of(&ramp, &level(), None);
    let host = on("r", HostSide::Left, 0.1);
    let paths = host_paths(&railing(Some(host.clone())), &host, &Host::Ramp { ramp: &ramp, run: &run }).expect("paths");
    assert_eq!(paths.len(), 1);
    assert!(paths[0].iter().all(|p| (p.y - 0.5).abs() < EPS));
    assert!(paths[0].iter().all(|p| (p.z - run.z_at(p.x)).abs() < EPS));
    assert_eq!(paths[0].len(), 4);
}

fn slab() -> Slab {
    let corner = |x: f64, y: f64| Vertex { point: Point2 { x, y }, bulge: 0.0 };
    Slab { storey: "st".into(), slab_type: "t".into(), boundary: vec![corner(0.0, 0.0), corner(4.0, 0.0), corner(4.0, 3.0), corner(0.0, 3.0)], holes: Vec::new(), offset: 0.1, slope: None, name: String::new() }
}

#[test]
fn a_rail_on_a_slab_edge_stands_inside_or_outside_on_the_top_plane() {
    let slab = slab();
    let level = level();
    let mut inward = on("sl", HostSide::Left, 0.2);
    inward.edge = 1;
    let paths = host_paths(&railing(Some(inward.clone())), &inward, &Host::Slab { slab: &slab, own: &level }).expect("paths");
    assert_eq!(paths[0].len(), 2);
    assert!((paths[0][0].x - 3.8).abs() < EPS && paths[0][0].y.abs() < EPS && (paths[0][0].z - 0.1).abs() < EPS);
    assert!((paths[0][1].x - 3.8).abs() < EPS && (paths[0][1].y - 3.0).abs() < EPS);
    let mut outward = inward.clone();
    outward.side = HostSide::Right;
    let outside = host_paths(&railing(Some(outward.clone())), &outward, &Host::Slab { slab: &slab, own: &level }).expect("paths");
    assert!((outside[0][0].x - 4.2).abs() < EPS);
}

#[test]
fn a_sloped_slab_lifts_the_rail_with_its_plane() {
    let slab = Slab { slope: Some(crate::Slope { direction: 0.0, angle: 0.1 }), ..slab() };
    let level = level();
    let mut host = on("sl", HostSide::Left, 0.0);
    host.edge = 0;
    let paths = host_paths(&railing(Some(host.clone())), &host, &Host::Slab { slab: &slab, own: &level }).expect("paths");
    assert!((paths[0][0].z - 0.1).abs() < EPS);
    assert!((paths[0][1].z - (0.1 - 4.0 * 0.1f64.tan())).abs() < 1e-9);
}

#[test]
fn a_missing_or_curved_slab_edge_cannot_be_followed() {
    let slab = slab();
    let level = level();
    let mut far = on("sl", HostSide::Left, 0.0);
    far.edge = 9;
    assert_eq!(host_paths(&railing(Some(far.clone())), &far, &Host::Slab { slab: &slab, own: &level }), Err(HostFault::Edge));
    assert!(unresolved(&railing(Some(far)), Some(&slab)));
    let mut curved = crate::Slab { boundary: slab.boundary.clone(), ..slab.clone() };
    curved.boundary[0].bulge = 0.4;
    let edge = on("sl", HostSide::Left, 0.0);
    assert_eq!(host_paths(&railing(Some(edge.clone())), &edge, &Host::Slab { slab: &curved, own: &level }), Err(HostFault::Edge));
    assert!(!unresolved(&railing(None), Some(&slab)) && !unresolved(&railing(Some(edge)), Some(&slab)));
}

#[test]
fn a_hosted_railing_is_a_solid_of_its_authored_sections_and_absent_without_a_host() {
    let run = stair_run_of(&stair(StairFlight::Straight), &level(), None);
    let host = on("s", HostSide::Left, 0.0);
    let hosted = railing(Some(host));
    let solid = hosted_solid(&hosted, Some(&Host::Stair(&run)));
    assert_eq!(solid.family, SolidFamily::Railing);
    assert!(solid.volume > 0.0 && solid.groups.iter().any(|group| group.part == parts::POST) && solid.groups.iter().any(|group| group.part == parts::RAIL));
    assert!(hosted_solid(&hosted, None).is_empty());
    let thick = Railing { profile: crate::Profile::Rectangle { width: 0.12, depth: 0.08 }, ..hosted.clone() };
    assert!(hosted_solid(&thick, Some(&Host::Stair(&run))).volume > solid.volume);
}

#[test]
fn the_rail_length_is_measured_in_three_dimensions() {
    let path = vec![vec![RailPoint { x: 0.0, y: 0.0, z: 0.0 }, RailPoint { x: 3.0, y: 0.0, z: 4.0 }]];
    assert!((length_of(&path) - 5.0).abs() < EPS);
}
