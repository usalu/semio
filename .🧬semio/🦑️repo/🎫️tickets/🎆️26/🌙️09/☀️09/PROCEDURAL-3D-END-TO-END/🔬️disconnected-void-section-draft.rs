//! 🕳️ Scoped native regression for separated cavity contours and material islands.

use super::*;
use semio_framework_3d_brep_construction::{euler::make_vertex, primitives::{attach_face, finish_solid, line_edge}};
use semio_framework_3d_brep_representation::{arena::SolidId, contract::SectionPolygon};

fn polygon_solid(body: &mut Body, points: &[Pnt3], rings: Vec<Vec<usize>>) -> SolidId {
    let mut record = OpRecorder::new();
    let vertices: Vec<_> = points.iter().map(|&point| make_vertex(body, point, Default::default(), &mut record)).collect();
    let mut edges = std::collections::BTreeMap::new();
    let faces = rings.iter().map(|ring| {
        let [a, b, c] = [ring[0], ring[1], ring[2]].map(|i| points[i]);
        let frame = Frame3::from_normal(a, (b-a).cross(c-a)).unwrap();
        let support = body.surfaces.insert(Surface::Plane { frame });
        let members: Vec<_> = ring.iter().copied().zip(ring.iter().copied().cycle().skip(1)).map(|(from,to)| {
            let (lo,hi) = (from.min(to),from.max(to));
            let edge = *edges.entry((lo,hi)).or_insert_with(|| line_edge(
                body, points[lo], points[hi], vertices[lo], vertices[hi], Default::default(), &mut record,
            ));
            (edge, from==lo)
        }).collect();
        attach_face(body, support, &members, false, Default::default(), &mut record)
    }).collect();
    finish_solid(body, faces, &mut record)
}

fn layer(profile: &Value, z: f64) -> impl Iterator<Item = Pnt3> + '_ {
    profile.as_array().unwrap().iter().map(move |point| Pnt3::new(number(&point[0]), number(&point[1]), z))
}

fn add_cavity(body: &mut Body, stock: SolidId, points: &[Pnt3], rings: Vec<Vec<usize>>) {
    let void = polygon_solid(body, points, rings);
    let shell = body.solids.get(void).unwrap().outer;
    for face in body.shells.get(shell).unwrap().faces.clone() {
        body.faces.get_mut(face).unwrap().flipped = true;
    }
    body.solids.get_mut(stock).unwrap().inners.push(shell);
}

fn signed_components(body: &Body, solid: SolidId, origin: [f64;3], normal: [f64;3]) -> Vec<Vec<f64>> {
    use semio_framework_3d_brep_queries::planar_section::PlanarSectionRegion;
    let section = planar_section(body, solid, origin, normal, 1e-6).unwrap().unwrap();
    let regions: Vec<PlanarSectionRegion> = match section {
        PlanarSection::Region(region) => vec![region],
        PlanarSection::Regions(regions) => regions.into_regions().collect(),
        _ => panic!("Section must contain a cavity"),
    };
    let signed = |ring: &SectionPolygon| {
        let points: Vec<_> = (0..ring.count()).map(|i| Pnt3::from_array(ring.point(i).unwrap())).collect();
        let anchor = points[0];
        let axis = Vec3::from_array(normal);
        points.iter().zip(points.iter().cycle().skip(1)).map(|(&a, &b)| (a-anchor).cross(b-anchor).dot(axis)).sum::<f64>()/2.
    };
    let mut areas: Vec<_> = regions.into_iter().map(|region| {
        let (outer, holes) = region.into_rings();
        let mut rings: Vec<_> = std::iter::once(signed(&outer)).chain(holes.iter().flatten().map(signed)).collect();
        rings.sort_by(|a,b| b.total_cmp(a));
        rings
    }).collect();
    areas.sort_by(|a,b| a.iter().sum::<f64>().total_cmp(&b.iter().sum()));
    areas
}

#[test]
fn one_cavity_produces_multiple_section_holes_or_a_material_island() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/disconnected-void-section.json")).unwrap();

    let mut body = Body::new();
    let stock = placed_box(&mut body, &fixture["stock"], &serde_json::json!([0,0,0]));
    let void = &fixture["void"];
    let base = number(&void["base"]);
    let profile = &void["profile"];
    let points: Vec<_> = layer(profile, base).chain(layer(profile, base+number(&void["height"]))).collect();
    let count = points.len()/2;
    let mut rings = vec![(0..count).rev().collect(), (count..2*count).collect()];
    rings.extend((0..count).map(|i| {
        let next = (i+1)%count;
        vec![i,next,count+next,count+i]
    }));
    add_cavity(&mut body, stock, &points, rings);
    let wanted: Vec<f64> = serde_json::from_value(fixture["rings"].clone()).unwrap();
    for sign in [-1.,1.] {
        assert_eq!(signed_components(&body, stock, [0.,number(&fixture["cut"]),0.], [0.,sign,0.]), [wanted.clone()]);
    }

    let island = &fixture["island"];
    let mut body = Body::new();
    let stock = placed_box(&mut body, &island["stock"], &serde_json::json!([0,0,0]));
    let base = number(&island["base"]);
    let points: Vec<_> = [base,base+number(&island["height"])].into_iter().flat_map(|z| {
        layer(&island["outer"],z).chain(layer(&island["inner"],z))
    }).collect();
    let rings = (0..4).flat_map(|i| {
        let j = (i+1)%4;
        [[j,i,4+i,4+j], [8+i,8+j,12+j,12+i], [i,j,8+j,8+i], [4+j,4+i,12+i,12+j]]
            .map(|quad| quad.to_vec())
    }).collect();
    add_cavity(&mut body, stock, &points, rings);
    let wanted: Vec<Vec<f64>> = island["components"].as_array().unwrap().iter()
        .map(|face| serde_json::from_value(face["rings"].clone()).unwrap()).collect();
    for sign in [-1.,1.] {
        assert_eq!(signed_components(&body, stock, [0.,0.,number(&island["cut"])], [0.,0.,sign]), wanted);
    }
}
