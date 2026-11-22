"""🕳️ Tests the original planar Section query on one cavity with two contour components."""
import json
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
source = ticket / "🐍️skew-box-intersection-source-check.py"
fixture = json.loads((ticket / "🔣️disconnected-void-section.json").read_text())
witness = """
fn append_prism(body: &mut Body, profile: &[[f64;2]], z: f64, height: f64, inward: bool) -> ShellId {
    let mut points = [pnt([0.;3]); 32];
    for (point, [x,y]) in points.iter_mut().zip(profile) { *point = pnt([*x,*y,z]); }
    let recipe = crate::PrismBoundary::from_planar_ring(points, profile.len(), pnt([0.,0.,height]), pnt([0.,0.,1.])).unwrap();
    let (vertex_start, edge_start, curve_start) = (body.vertices.0.len(), body.edges.0.len(), body.curves3.0.len());
    for i in 0..recipe.vertex_count() { body.vertices.0.push(Vertex { position: recipe.vertex(i) }); }
    for i in 0..recipe.edge_count() {
        let row = recipe.edge(i);
        let [v0,v1] = row.vertices.map(|vertex| vertex_start+vertex);
        body.edges.0.push(Edge { v0, v1, curve: curve_start+i, range: row.range });
        body.curves3.0.push(row.curve);
    }
    let first_face = body.faces.0.len();
    for (local, &count) in recipe.ring_sizes().iter().enumerate() {
        let surface = body.surfaces.0.len();
        let ring = body.loops.0.len();
        let first = body.coedges.0.len();
        body.surfaces.0.push(recipe.surface(local));
        body.faces.0.push(Face { surface, outer: Some(ring), inners: Vec::new(), flipped: inward });
        body.loops.0.push(Loop { face: first_face+local, first });
        for i in 0..count {
            let trim = recipe.trim(local,i);
            body.coedges.0.push(Coedge { edge: edge_start+trim.edge, loop_id: ring, next: first+(i+1)%count,
                forward: trim.forward, pcurve: None, prange: trim.range });
        }
    }
    let id = body.shells.0.len();
    body.shells.0.push(Shell { faces: (first_face..body.faces.0.len()).collect() });
    id
}

#[test]
fn one_concave_void_contributes_both_holes() {
    let mut body = empty_body();
    let [width,depth,height] = STOCK;
    let outer = append_prism(&mut body, &[[0.,0.],[width,0.],[width,depth],[0.,depth]], 0., height, false);
    let cavity = append_prism(&mut body, &CAVITY, BASE, HEIGHT, true);
    body.solids.0.push(Solid { outer, inners: vec![cavity] });
    for sign in [-1.,1.] {
        let PlanarSection::Region(region) = planar_section(&body, 0, [0.,CUT,0.], [0.,sign,0.], 1e-6).unwrap().unwrap()
            else { panic!("one connected material section"); };
        let signed = |ring: &SectionPolygon| {
            let points: Vec<_> = (0..ring.count()).map(|i| pnt(ring.point(i).unwrap())).collect();
            points.iter().zip(points.iter().cycle().skip(1)).map(|(a,b)| a.z*b.x-a.x*b.z).sum::<f64>()*sign/2.
        };
        let (outer,holes) = region.into_rings();
        let mut rings: Vec<_> = std::iter::once(signed(&outer)).chain(holes.iter().flatten().map(signed)).collect();
        rings.sort_by(|a,b| b.total_cmp(a));
        assert_eq!(rings, RINGS);
        assert_eq!(rings.iter().sum::<f64>(), AREA);
    }
}
fn empty_body() -> Body {
    Body {
        solids: Arena(Vec::new()), shells: Arena(Vec::new()), faces: Arena(Vec::new()),
        surfaces: Arena(Vec::new()), loops: Arena(Vec::new()), coedges: Arena(Vec::new()),
        edges: Arena(Vec::new()), curves3: Arena(Vec::new()), curves2: Arena(Vec::new()), vertices: Arena(Vec::new()),
    }
}

fn append_annular_prism(body: &mut Body, outside: [[f64;2];4], inside: [[f64;2];4], z: f64, height: f64) -> ShellId {
    let base = body.vertices.0.len();
    for layer in [z, z+height] {
        for [x,y] in outside.into_iter().chain(inside) { body.vertices.0.push(Vertex { position: pnt([x,y,layer]) }); }
    }
    let first_face = body.faces.0.len();
    let mut edges = std::collections::BTreeMap::new();
    for i in 0..4 {
        let j = (i+1)%4;
        for corners in [[j,i,4+i,4+j], [8+i,8+j,12+j,12+i], [i,j,8+j,8+i], [4+j,4+i,12+i,12+j]] {
            let vertices = corners.map(|i| i+base);
            let positions = vertices.map(|i| body.vertices.0[i].position);
            let normal = (positions[1]-positions[0]).cross(positions[2]-positions[0]);
            let surface = body.surfaces.0.len();
            body.surfaces.0.push(Surface::Plane { frame: Frame3::from_normal(positions[0], normal).unwrap() });
            let face = body.faces.0.len();
            let ring = body.loops.0.len();
            let first = body.coedges.0.len();
            body.faces.0.push(Face { surface, outer: Some(ring), inners: Vec::new(), flipped: true });
            body.loops.0.push(Loop { face, first });
            for i in 0..4 {
                let (a,b) = (vertices[i],vertices[(i+1)%4]);
                let (v0,v1) = (a.min(b),a.max(b));
                let edge = *edges.entry((v0,v1)).or_insert_with(|| {
                    let origin = body.vertices.0[v0].position;
                    let curve = body.curves3.0.len();
                    body.curves3.0.push(Curve3::Line { origin, dir: body.vertices.0[v1].position-origin });
                    let edge = body.edges.0.len();
                    body.edges.0.push(Edge { v0, v1, curve, range: (0.,1.) });
                    edge
                });
                body.coedges.0.push(Coedge { edge, loop_id: ring, next: first+(i+1)%4, forward: a==v0, pcurve: None, prange: (0.,1.) });
            }
        }
    }
    let id = body.shells.0.len();
    body.shells.0.push(Shell { faces: (first_face..body.faces.0.len()).collect() });
    id
}

#[test]
fn annular_void_leaves_an_independent_material_island() {
    let mut body = empty_body();
    let [width,depth,height] = ENCLOSURE;
    let outer = append_prism(&mut body, &[[0.,0.],[width,0.],[width,depth],[0.,depth]], 0., height, false);
    let void = append_annular_prism(&mut body, RING_OUTSIDE, RING_INSIDE, FLOOR, SPAN);
    body.solids.0.push(Solid { outer, inners: vec![void] });
    for sign in [-1.,1.] {
        let PlanarSection::Regions(regions) = planar_section(&body,0,[0.,0.,SLICE],[0.,0.,sign],1e-6).unwrap().unwrap()
            else { panic!("annular material plus island"); };
        let signed = |ring: &SectionPolygon| {
            let points: Vec<_> = (0..ring.count()).map(|i| pnt(ring.point(i).unwrap())).collect();
            points.iter().zip(points.iter().cycle().skip(1)).map(|(a,b)| a.x*b.y-a.y*b.x).sum::<f64>()*sign/2.
        };
        let mut regions: Vec<_> = regions.into_regions().map(|region| {
            let (outer, holes) = region.into_rings();
            std::iter::once(signed(&outer)).chain(holes.iter().flatten().map(signed)).collect::<Vec<_>>()
        }).collect();
        regions.sort_by(|a,b| a.iter().sum::<f64>().total_cmp(&b.iter().sum()));
        assert_eq!(regions, EXPECTED_ISLAND_COMPONENTS);
    }
}

"""
cavity = fixture["void"]
for name, value in {
    "STOCK": list(map(float, fixture["stock"])),
    "CAVITY": [list(map(float, pt)) for pt in cavity["profile"]],
    "BASE": float(cavity["base"]), "HEIGHT": float(cavity["height"]), "CUT": float(fixture["cut"]),
    "RINGS": list(map(float, fixture["rings"])), "AREA": float(fixture["area"]),
}.items():
    witness = witness.replace(name, repr(value))
island = fixture["island"]
for name, value in {
    "ENCLOSURE": list(map(float, island["stock"])),
    "RING_OUTSIDE": [list(map(float, point)) for point in island["outer"]],
    "RING_INSIDE": [list(map(float, point)) for point in island["inner"]],
    "FLOOR": float(island["base"]), "SPAN": float(island["height"]), "SLICE": float(island["cut"]),
}.items():
    witness = witness.replace(name, repr(value))
rings = ", ".join("vec!" + repr(list(map(float, face["rings"]))) for face in island["components"])
witness = witness.replace("EXPECTED_ISLAND_COMPONENTS", "vec![" + rings + "]")
through = json.loads(Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/through-hole-section.json").read_text())
through_cases = []
for stock in through:
    through_expected = []
    for cut in stock["cuts"]:
        through_expected.append("(" + repr(list(map(float, cut["origin"]))) + ", "
            + repr(list(map(float, cut["normal"]))) + ", vec!["
            + ", ".join("vec!" + repr(list(map(float, ring))) for ring in cut["rings"]) + "])")
    profile = lambda ring: repr([list(map(float, point)) for point in ring])
    through_cases.append("(" + profile(stock["outside"]) + ", vec![" + ", ".join(map(profile, stock["bores"]))
        + "], " + repr(float(stock["height"])) + ", vec![" + ", ".join(through_expected) + "])")
witness += r"""
#[test]
fn round_trip_section_of_prism_with_a_square_through_hole() {
    for (outside,bores,height,cuts) in TUNNEL_CASES {
    let mut body = empty_body();
    let profiles:Vec<_>=std::iter::once(outside).chain(bores).collect();
    let mut points = Vec::new();
    for z in [0., height] {
        for &[x,y] in profiles.iter().flatten() {
            points.push(pnt([x,y,z]));
        }
    }
    for &position in &points { body.vertices.0.push(Vertex { position }); }
    let span=points.len()/2;
    let mut bottom=Vec::new();
    let mut top=Vec::new();
    let mut sides=Vec::new();
    for (index,_) in profiles.iter().enumerate() {
        let mut ring:Vec<_>=(index*4..index*4+4).collect();
        if index>0 {ring.reverse();}
        bottom.push(ring.iter().copied().rev().collect());
        top.push(ring.iter().map(|vertex| vertex+span).collect());
        for (&a,&b) in ring.iter().zip(ring.iter().cycle().skip(1)){
            sides.push(vec![vec![a,b,b+span,a+span]]);
        }
    }
    let faces:Vec<Vec<Vec<usize>>>=std::iter::once(bottom).chain([top]).chain(sides).collect();
    let mut edges = std::collections::BTreeMap::new();
    for (face,rings) in faces.iter().enumerate() {
        let [a,b,c] = [rings[0][0],rings[0][1],rings[0][2]].map(|i| points[i]);
        let surface=body.surfaces.0.len();
        body.surfaces.0.push(Surface::Plane { frame: Frame3::from_normal(a,(b-a).cross(c-a)).unwrap() });
        let mut loop_ids=Vec::new();
        for boundary in rings {
            let ring=body.loops.0.len();
            let first=body.coedges.0.len();
            body.loops.0.push(Loop { face, first });
            loop_ids.push(ring);
            for (i,(&from,&to)) in boundary.iter().zip(boundary.iter().cycle().skip(1)).enumerate() {
                let (v0,v1)=(from.min(to),from.max(to));
                let edge=*edges.entry((v0,v1)).or_insert_with(|| {
                    let curve=body.curves3.0.len();
                    body.curves3.0.push(Curve3::Line { origin: points[v0], dir: points[v1]-points[v0] });
                    let edge=body.edges.0.len();
                    body.edges.0.push(Edge { v0, v1, curve, range: (0.,1.) });
                    edge
                });
                body.coedges.0.push(Coedge { edge, loop_id:ring, next:first+(i+1)%boundary.len(),
                    forward:from==v0, pcurve:None, prange:(0.,1.) });
            }
        }
        body.faces.0.push(Face { surface, outer: Some(loop_ids.remove(0)), inners:loop_ids, flipped:false });
    }
    body.shells.0.push(Shell { faces: (0..faces.len()).collect() });
    body.solids.0.push(Solid { outer:0, inners:Vec::new() });
    for (origin,axis,wanted) in cuts {
        for sign in [-1.,1.] {
            let normal=axis.map(|coordinate| coordinate*sign);
            let signed=|ring:&SectionPolygon| {
                let points:Vec<_>=(0..ring.count()).map(|i|pnt(ring.point(i).unwrap())).collect();
                let anchor=points[0];
                points.iter().zip(points.iter().cycle().skip(1))
                    .map(|(&a,&b)|(a-anchor).cross(b-anchor).dot(pnt(normal))).sum::<f64>()/2.
            };
            let result=planar_section(&body,0,origin,normal,1e-6).expect("closed planar through-hole shell").unwrap();
            let area=|region: PlanarSectionRegion| {
                let (outer,holes)=region.into_rings();
                std::iter::once(signed(&outer)).chain(holes.iter().flatten().map(signed)).collect::<Vec<_>>()
            };
            let mut regions=match result {
                PlanarSection::Region(region)=>vec![area(region)],
                PlanarSection::Regions(regions)=>regions.into_regions().map(area).collect(),
                _=>panic!("through-hole shell is not convex"),
            };
            regions.sort_by(|a,b| a.iter().sum::<f64>().total_cmp(&b.iter().sum()));
            assert_eq!(regions,wanted);
        }
    }
}
}
""".replace("TUNNEL_CASES", "[" + ", ".join(through_cases) + "]")
live = source.read_text()
marker = 'section_source = section_source.replace("Pnt3::default()", "pnt([0.;3])")'
assert live.count(marker) == 1
patched = marker
if "--candidate" in sys.argv:
    pairs = json.loads((ticket / "🔣️disconnected-void-section-patch.json").read_text())
    for before, after in pairs:
        patched += "\nassert section_source.count("+repr(before)+") == 1"
        patched += "\nsection_source = section_source.replace("+repr(before)+","+repr(after)+")"
exec(compile(live.replace(marker, patched+"\nsection_source += "+repr(witness)), str(source), "exec"), {"__file__": str(source)})
