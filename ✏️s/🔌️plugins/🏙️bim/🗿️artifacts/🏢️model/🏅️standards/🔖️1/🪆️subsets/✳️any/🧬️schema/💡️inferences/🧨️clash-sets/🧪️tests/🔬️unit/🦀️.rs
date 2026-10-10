//! 🧪️ The clash sets: the sweep of candidate pairs against brute force, hard and soft clashes of boxes, hosting, deduplication and grouping.

use super::*;
use crate::ElementSelector;
use semio_framework_geometry::random::Rng;

type Box3 = ([f64; 3], [f64; 3]);

fn cuboid((lo, hi): Box3) -> TriMesh {
    let c = |x: usize, y: usize, z: usize| [if x == 0 { lo[0] } else { hi[0] }, if y == 0 { lo[1] } else { hi[1] }, if z == 0 { lo[2] } else { hi[2] }];
    let mut mesh = TriMesh::new();
    let faces = [
        [c(0, 0, 0), c(0, 1, 0), c(1, 1, 0), c(1, 0, 0)],
        [c(0, 0, 1), c(1, 0, 1), c(1, 1, 1), c(0, 1, 1)],
        [c(0, 0, 0), c(1, 0, 0), c(1, 0, 1), c(0, 0, 1)],
        [c(1, 0, 0), c(1, 1, 0), c(1, 1, 1), c(1, 0, 1)],
        [c(1, 1, 0), c(0, 1, 0), c(0, 1, 1), c(1, 1, 1)],
        [c(0, 1, 0), c(0, 0, 0), c(0, 0, 1), c(0, 1, 1)],
    ];
    for face in faces {
        mesh.push_quad(face[0], face[1], face[2], face[3]);
    }
    mesh
}

fn probe(bounds: Box3) -> SolidProbe {
    let mesh = cuboid(bounds);
    SolidProbe { class: Some(ElementClass::Beam), storey: "st-ground".into(), bounds: collision::mesh_aabb(&mesh), bvh: Bvh::build(&mesh), mesh }
}

fn set(tolerance: f64, clearance: f64) -> ClashSet {
    ClashSet { name: "Test".into(), a: ElementSelector::all(), b: ElementSelector::all(), tolerance, clearance }
}

fn run(set: &ClashSet, a: &[(&str, &SolidProbe)], b: &[(&str, &SolidProbe)], relations: &BTreeMap<String, String>) -> ClashSetResult {
    clashes_of(set, a, b, relations, &|_| "st-ground".to_string(), &|| false)
}

#[test]
fn the_sweep_finds_exactly_the_pairs_whose_grown_boxes_overlap() {
    let mut rng = Rng::from_seed(0x5eed);
    let mut range = |lo: f64, hi: f64| lo + (hi - lo) * rng.next_f64();
    let mut boxes = move |count: usize| -> Vec<SolidProbe> {
        (0..count)
            .map(|_| {
                let lo = [range(-10.0, 10.0), range(-10.0, 10.0), range(0.0, 6.0)];
                probe((lo, [lo[0] + range(0.1, 3.0), lo[1] + range(0.1, 3.0), lo[2] + range(0.1, 3.0)]))
            })
            .collect()
    };
    let (left, right) = (boxes(60), boxes(80));
    let ids_a: Vec<String> = (0..left.len()).map(|at| format!("a{at:03}")).collect();
    let ids_b: Vec<String> = (0..right.len()).map(|at| format!("b{at:03}")).collect();
    let a: Vec<(&str, &SolidProbe)> = ids_a.iter().map(String::as_str).zip(left.iter()).collect();
    let b: Vec<(&str, &SolidProbe)> = ids_b.iter().map(String::as_str).zip(right.iter()).collect();
    for margin in [0.0, 0.05, 1.5] {
        let brute: Vec<(usize, usize)> = (0..a.len()).flat_map(|i| (0..b.len()).map(move |j| (i, j))).filter(|&(i, j)| near(a[i].1, b[j].1, margin)).collect();
        assert_eq!(candidate_pairs(&a, &b, margin), brute, "margin {margin}");
    }
}

#[test]
fn interpenetrating_boxes_clash_hard_by_their_smallest_overlap() {
    let (beam, wall) = (probe(([0.0, 0.0, 2.9], [4.0, 0.3, 3.4])), probe(([1.0, -1.0, 0.0], [1.2, 1.0, 3.0])));
    let result = run(&set(0.001, 0.0), &[("beam", &beam)], &[("wall", &wall)], &BTreeMap::new());
    assert_eq!(result.tested, 1);
    assert_eq!(result.clashes.len(), 1);
    let clash = &result.clashes[0];
    assert_eq!((clash.first.as_str(), clash.second.as_str(), clash.kind), ("beam", "wall", ClashKind::Hard));
    assert!((clash.distance + 0.1).abs() < 1e-9, "{}", clash.distance);
    assert_eq!(clash.storey, "st-ground");
    assert!((clash.point.z - 2.95).abs() < 1e-9 && (clash.min.z - 2.9).abs() < 1e-9 && (clash.max.z - 3.0).abs() < 1e-9);
}

#[test]
fn touching_boxes_and_overlaps_below_the_tolerance_do_not_clash() {
    let slab = probe(([0.0, 0.0, -0.2], [8.0, 6.0, 0.0]));
    let resting = probe(([1.0, 1.0, 0.0], [1.3, 1.3, 3.0]));
    let sunk = probe(([3.0, 1.0, -0.001], [3.3, 1.3, 3.0]));
    let result = run(&set(0.002, 0.0), &[("slab", &slab)], &[("resting", &resting), ("sunk", &sunk)], &BTreeMap::new());
    assert!(result.clashes.is_empty(), "{result:?}");
    assert_eq!(result.tested, 2);
    let strict = run(&set(0.0, 0.0), &[("slab", &slab)], &[("resting", &resting), ("sunk", &sunk)], &BTreeMap::new());
    assert_eq!(strict.clashes.iter().map(|clash| clash.second.as_str()).collect::<Vec<_>>(), ["sunk"]);
    assert!((strict.clashes[0].distance + 0.001).abs() < 1e-9);
}

#[test]
fn a_clearance_reports_soft_clashes_closer_than_it_and_keeps_hard_ones_hard() {
    let duct = probe(([0.0, 0.0, 2.0], [4.0, 0.4, 2.3]));
    let beam = probe(([0.0, 0.0, 2.33], [4.0, 0.4, 2.8]));
    let far = probe(([0.0, 0.0, 2.6], [4.0, 0.4, 3.0]));
    let result = run(&set(0.0, 0.05), &[("duct", &duct)], &[("beam", &beam), ("far", &far)], &BTreeMap::new());
    assert_eq!(result.clashes.len(), 1);
    assert_eq!((result.clashes[0].second.as_str(), result.clashes[0].kind), ("beam", ClashKind::Clearance));
    assert!((result.clashes[0].distance - 0.03).abs() < 1e-9);
    assert_eq!((result.hard(), result.soft()), (0, 1));
    let none = run(&set(0.0, 0.03), &[("duct", &duct)], &[("beam", &beam)], &BTreeMap::new());
    assert!(none.clashes.is_empty(), "a gap equal to the clearance is no clash");
}

#[test]
fn a_host_and_its_hosted_element_never_clash_and_a_pair_picked_from_both_sides_is_reported_once() {
    let wall = probe(([0.0, 0.0, 0.0], [4.0, 0.3, 3.0]));
    let window = probe(([1.0, -0.1, 1.0], [2.0, 0.4, 2.0]));
    let other = probe(([3.5, 0.0, 0.0], [4.5, 0.3, 3.0]));
    let relations: BTreeMap<String, String> = [("window".to_string(), "wall".to_string())].into();
    let all = [("other", &other), ("wall", &wall), ("window", &window)];
    let result = run(&set(0.001, 0.0), &all, &all, &relations);
    assert_eq!(result.tested, 2, "wall/other and window/other are tested once each, the hosted pair is skipped, and nothing is tested against itself");
    assert_eq!(result.clashes.iter().map(|clash| (clash.first.as_str(), clash.second.as_str())).collect::<Vec<_>>(), [("other", "wall")]);
}

#[test]
fn clashes_are_grouped_by_the_element_that_takes_part_in_most_of_them() {
    let clash = |first: &str, second: &str| Clash { first: first.into(), second: second.into(), kind: ClashKind::Hard, distance: -0.1, point: Point3 { x: 0.0, y: 0.0, z: 0.0 }, min: Point3 { x: 0.0, y: 0.0, z: 0.0 }, max: Point3 { x: 0.0, y: 0.0, z: 0.0 }, storey: String::new() };
    let clashes = vec![clash("a", "z"), clash("b", "z"), clash("c", "z"), clash("d", "e"), clash("e", "f")];
    let groups = grouped(&clashes);
    assert_eq!(groups[0], ClashGroup { anchor: "z".into(), members: vec![0, 1, 2] });
    assert_eq!(groups[1], ClashGroup { anchor: "e".into(), members: vec![3, 4] });
    assert_eq!(groups.len(), 2);
    assert!(grouped(&[]).is_empty());
}

#[test]
fn a_cancelled_run_stops_at_once() {
    let a = probe(([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]));
    let b = probe(([0.5, 0.5, 0.5], [1.5, 1.5, 1.5]));
    let result = clashes_of(&set(0.0, 0.0), &[("a", &a)], &[("b", &b)], &BTreeMap::new(), &|_| String::new(), &|| true);
    assert!(result.clashes.is_empty() && result.tested == 0);
}

#[test]
fn a_placed_solid_lands_in_the_world() {
    let mut solid = ElementSolid::default();
    solid.positions = vec![1.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0, 1.0, 0.0];
    solid.indices = vec![0, 1, 2];
    solid.placement = super::super::super::element_solids::SolidPlacement { x: 10.0, y: 20.0, z: 3.0, rotation: std::f64::consts::FRAC_PI_2 };
    let mesh = world_mesh(&solid);
    let expect = [[10.0, 21.0, 3.0], [10.0, 22.0, 3.0], [9.0, 21.0, 3.0]];
    for (got, want) in mesh.positions.iter().zip(expect) {
        assert!(got.iter().zip(want).all(|(a, b)| (a - b).abs() < 1e-12), "{got:?} {want:?}");
    }
}
