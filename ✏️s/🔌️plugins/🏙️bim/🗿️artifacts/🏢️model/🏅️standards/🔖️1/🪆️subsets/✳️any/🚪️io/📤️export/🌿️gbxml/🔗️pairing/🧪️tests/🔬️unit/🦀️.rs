use super::*;
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::Vec3;

fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

fn wall(id: &str, adjacent: &str, azimuth: f64, polygon: Vec<Vec3>) -> EnvelopeSurface {
    EnvelopeSurface { id: id.into(), kind: SurfaceKind::Wall, element: "w".into(), boundary: Boundary::Adjacent, adjacent: adjacent.into(), area: 12.0, gross_area: 12.0, azimuth, tilt: 90.0, polygon, ..EnvelopeSurface::default() }
}

fn horizontal(id: &str, kind: SurfaceKind, adjacent: &str, z: f64) -> EnvelopeSurface {
    let polygon = if kind == SurfaceKind::Floor { vec![v(0.0, 0.0, z), v(0.0, 3.0, z), v(4.0, 3.0, z), v(4.0, 0.0, z)] } else { vec![v(0.0, 0.0, z), v(4.0, 0.0, z), v(4.0, 3.0, z), v(0.0, 3.0, z)] };
    EnvelopeSurface { id: id.into(), kind, element: "s".into(), boundary: Boundary::Adjacent, adjacent: adjacent.into(), area: 12.0, gross_area: 12.0, tilt: if kind == SurfaceKind::Floor { 180.0 } else { 0.0 }, polygon, ..EnvelopeSurface::default() }
}

fn envelope(surfaces: Vec<EnvelopeSurface>) -> EnvelopeSpace {
    EnvelopeSpace { surfaces, ..EnvelopeSpace::default() }
}

fn partition() -> BTreeMap<String, EnvelopeSpace> {
    let east = wall("a/w1", "b", 90.0, vec![v(4.0, 0.0, 0.0), v(4.0, 3.0, 0.0), v(4.0, 3.0, 3.0), v(4.0, 0.0, 3.0)]);
    let west = wall("b/w1", "a", 270.0, vec![v(4.3, 3.0, 0.0), v(4.3, 0.0, 0.0), v(4.3, 0.0, 3.0), v(4.3, 3.0, 3.0)]);
    let mut door = wall("a/o1", "b", 90.0, vec![v(4.0, 1.0, 0.0), v(4.0, 2.0, 0.0), v(4.0, 2.0, 2.1), v(4.0, 1.0, 2.1)]);
    door.kind = SurfaceKind::Door;
    door.element = "o-door".into();
    door.parent = "a/w1".into();
    let mut theirs = door.clone();
    theirs.id = "b/o1".into();
    theirs.adjacent = "a".into();
    theirs.parent = "b/w1".into();
    BTreeMap::from([("a".to_string(), envelope(vec![east, door])), ("b".to_string(), envelope(vec![west, theirs]))])
}

#[test]
fn the_two_sides_of_a_wall_are_one_surface_of_the_lower_space_with_two_adjacent_spaces_and_one_door() {
    let envelopes = partition();
    let merged = merge(&envelopes);
    assert_eq!(merged.len(), 1, "{merged:?}");
    assert_eq!((merged[0].space, merged[0].surface.id.as_str()), ("a", "a/w1"));
    assert_eq!(merged[0].other.map(|(space, surface)| (space, surface.id.as_str())), Some(("b", "b/w1")));
    assert_eq!(merged[0].spaces(), vec!["a", "b"]);
    assert_eq!(merged[0].children.len(), 1, "the door of both sides is one opening");
}

#[test]
fn walls_that_do_not_overlap_stay_two_surfaces_with_one_adjacent_space_listed_twice_never() {
    let mut envelopes = partition();
    envelopes.get_mut("b").expect("b").surfaces[0].polygon = vec![v(9.0, 3.0, 0.0), v(9.0, 7.0, 0.0), v(9.0, 7.0, 3.0), v(9.0, 3.0, 3.0)];
    let merged = merge(&envelopes);
    assert_eq!(merged.len(), 2);
    assert!(merged.iter().all(|surface| surface.other.is_none()));
    assert_eq!(merged[0].spaces(), vec!["a", "b"], "an unpaired partition still names the space behind it");
}

#[test]
fn a_floor_and_the_ceiling_under_it_are_one_surface_owned_by_the_upper_space() {
    let envelopes = BTreeMap::from([
        ("low".to_string(), envelope(vec![horizontal("low/c1", SurfaceKind::Ceiling, "up", 3.0)])),
        ("up".to_string(), envelope(vec![horizontal("up/f1", SurfaceKind::Floor, "low", 3.3)])),
    ]);
    let merged = merge(&envelopes);
    assert_eq!(merged.len(), 1);
    assert_eq!((merged[0].space, merged[0].surface.id.as_str(), merged[0].spaces()), ("up", "up/f1", vec!["up", "low"]));
}

#[test]
fn outer_surfaces_are_never_paired() {
    let mut roof = horizontal("low/c1", SurfaceKind::Ceiling, "", 3.0);
    roof.boundary = Boundary::Exterior;
    let envelopes = BTreeMap::from([("low".to_string(), envelope(vec![roof]))]);
    let merged = merge(&envelopes);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].spaces(), vec!["low"]);
}
