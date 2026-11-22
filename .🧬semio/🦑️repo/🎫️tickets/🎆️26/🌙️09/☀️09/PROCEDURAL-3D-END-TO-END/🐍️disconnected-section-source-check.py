"""✂️ Runs the retained section walk against the neutral disconnected prism fixture."""
import json
from pathlib import Path

ticket = Path(__file__).resolve().parent
source = ticket / "🐍️skew-box-intersection-source-check.py"
fixture_path = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/disconnected-planar-section.json")
fixture = json.loads(fixture_path.read_text())
profile = "[" + ",".join(f"[{float(x)},{float(y)}]" for x, y in fixture["profile"]) + "]"
cavity = fixture["cases"][1]["voids"][0]
void_x, _, void_z = cavity["origin"]
width, _, depth = cavity["size"]
hole = repr([[float(x), float(z)] for x, z in (
    (void_x, void_z), (void_x + width, void_z),
    (void_x + width, void_z + depth), (void_x, void_z + depth),
)])
witness = """
#[test]
fn disconnected_section_crosses_both_arms() {
    let profile = PROFILE;
    let count = profile.len();
    let mut points = [pnt([0.; 3]); MAX_VERTICES];
    let mut supports = [(pnt([0.; 3]), pnt([0.; 3])); MAX_FACES];
    let mut endpoints = [[0_u8; 2]; MAX_EDGES];
    let mut face_edges = [[0_u8; MAX_VERTICES]; MAX_FACES];
    let mut face_sizes = [4; MAX_FACES];
    face_sizes[..2].fill(count);
    supports[0] = (pnt([0.; 3]), pnt([0., 0., -1.]));
    supports[1] = (pnt([0., 0., HEIGHT]), pnt([0., 0., 1.]));
    for (i, [x, y]) in profile.iter().copied().enumerate() {
        points[i] = pnt([x, y, 0.]);
        points[count+i] = pnt([x, y, HEIGHT]);
    }
    for i in 0..count {
        let next = (i + 1) % count;
        let outward = (points[next] - points[i]).cross(pnt([0., 0., 1.])).normalized().unwrap();
        supports[i+2] = (points[i], outward);
        endpoints[i] = [i as u8, next as u8];
        endpoints[i+count] = [(i+count) as u8, (next+count) as u8];
        endpoints[i+2*count] = [i as u8, (i+count) as u8];
        face_edges[0][i] = (count-i-1) as u8;
        face_edges[1][i] = (i+count) as u8;
        face_edges[i+2][..4].copy_from_slice(&[i, next+2*count, i+count, i+2*count].map(|edge| edge as u8));
    }
    let shell = PlanarShell { points, supports, endpoints, face_edges, face_sizes,
        vertex_count: count*2, face_count: count+2, edge_count: count*3 };
    assert!(closed_planar_manifold(&shell));
    for direction in [-1., 1.] {
        let origin = [0., CUT, 0.];
        let normal = [0., direction, 0.];
        let (anchor, axis) = section_plane(origin, normal).unwrap();
        let section = section_on_shell(&shell, origin, normal, anchor, axis, 1e-6).expect("certified cut");
        let PlanarSection::Regions(mut regions) = section.expect("disjoint faces are material") else { panic!("two face regions"); };
        let signed = |ring: &SectionPolygon| {
            let points: Vec<_> = (0..ring.count()).map(|i| pnt(ring.point(i).unwrap())).collect();
            points.iter().zip(points.iter().cycle().skip(1))
                .map(|(a,b)| a.z * b.x - a.x * b.z).sum::<f64>() * direction / 2.
        };
        let measure = |regions: &PlanarSectionRegions| {
            let mut components: Vec<_> = regions.0.iter().flatten().map(|region| {
                let mut rings = vec![signed(&region.outer)];
                rings.extend(region.holes.iter().flatten().map(signed));
                rings
            }).collect();
            components.sort_by(|a,b| a.iter().sum::<f64>().total_cmp(&b.iter().sum()));
            components
        };
        assert_eq!(measure(&regions), EXPECTED_PLAIN);
        let mut void = HOLE.map(|[x,z]|pnt([x, CUT, z]));
        if direction < 0. { void.reverse(); }
        regions.add_void(SectionPolygon::ordered_ring(&void).unwrap(), &Frame3::from_normal(anchor,axis).unwrap()).unwrap();
        assert_eq!(measure(&regions), EXPECTED_CAVITY);
        prove_section_publication(regions, direction);
    }
}

fn prove_section_publication(regions: PlanarSectionRegions, sign: f64) {
    use crate::polygon_face_publication::{FaceOpening, PolygonSectionSet, SECTION_FACE_LIMIT};
    let mut kernel = crate::BrepRegistry::default();
    let mut faces = PolygonSectionSet::default();
    for region in regions.into_regions() {
        let (outer, holes) = region.into_rings();
        let points: Vec<_> = (0..outer.count()).map(|i| outer.point(i).unwrap()).collect();
        let mut face = kernel.plan_polygon_face_points(outer.count()).unwrap();
        if holes.iter().any(Option::is_some) {
            let holes = holes.map(|hole| hole.map(|hole| {
                let points: Vec<_> = (0..hole.count()).map(|i| pnt(hole.point(i).unwrap())).collect();
                crate::polygon_opening::SectionPolygon::ordered_ring(&points).unwrap()
            }));
            face.opening = Some(FaceOpening::Polygon(crate::polygon_opening::PolygonOpening::new(holes)));
        }
        for _ in 0..500 {
            if face.topology_ready() { break; }
            crate::advance(&mut kernel, &mut face, &points).unwrap();
        }
        assert!(face.topology_ready());
        assert!(kernel.live.0.is_empty());
        faces.push((face, outer));
    }
    let count = faces.count();
    assert_eq!(count, 2);
    let mut keys: [String; SECTION_FACE_LIMIT] = std::array::from_fn(|i| {
        if i < count { faces.face(i).0.handle().to_owned() } else { String::new() }
    });
    let expected = keys.clone();
    let denied = crate::RetirementDemand { maximum_items: 0, ..Default::default() };
    assert!(kernel.publish_polygon_section(&mut faces, &mut keys, denied).is_err());
    assert_eq!(keys, expected);
    assert!(kernel.live.0.is_empty());
    let grant = crate::RetirementDemand { maximum_items: 1, ..Default::default() };
    kernel.publish_polygon_section(&mut faces, &mut keys, grant).unwrap();
    assert!(keys.iter().all(String::is_empty));
    assert_eq!(kernel.unpublished_topology, 0);
    assert_eq!(kernel.live.0.len(), count);
    let mut inner_counts = Vec::new();
    for i in 0..count {
        let crate::Entity::Face(id) = kernel.live.0[&expected[i]] else { panic!("published Section face"); };
        let face = kernel.body.faces.get(id).unwrap();
        let crate::Surface::Plane { frame } = kernel.body.surfaces.get(face.surface).unwrap() else { panic!("original section plane"); };
        assert!((frame.z.y - sign).abs() < 1e-10);
        inner_counts.push(face.inners.len());
        crate::close(&mut kernel, &mut faces.face_mut(i).0);
        assert!(kernel.body.faces.get(id).is_some());
    }
    inner_counts.sort_unstable();
    assert_eq!(inner_counts, [0, 1]);
    assert_eq!(kernel.live.0.len(), count);
}
""".replace("PROFILE", profile).replace("HEIGHT", str(float(fixture["height"]))).replace("CUT", str(float(fixture["cut"]))).replace("HOLE", hole)
for key, row in zip(("EXPECTED_PLAIN", "EXPECTED_CAVITY"), fixture["cases"]):
    rings = ", ".join("vec!" + repr(list(map(float, face["rings"]))) for face in row["faces"])
    witness = witness.replace(key, "vec![" + rings + "]")
live = source.read_text()
marker = 'section_source = section_source.replace("Pnt3::default()", "pnt([0.;3])")'
assert live.count(marker) == 1
exec(compile(live.replace(marker, marker+"\nsection_source += "+repr(witness)), str(source), "exec"), {"__file__": str(source)})
