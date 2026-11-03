//! 🪨️ Solids: prisms, boxes, cylinders, wedges, meshes, soil strata, footings, layered walls with openings and piles; polygon-with-holes extrusion as the only boolean.

use super::extrude::{prepare, prism};
use super::shells::revolve;
use super::vector::*;
use super::{Lod, MeshBuilder, MeshData, Tolerance};
use crate::{Frame, MaterialRole, Polygon2, SolidShape, Validate, Vec2, Vec3, Violation};

/// 🕳️ Rectangular wall opening in wall coordinates: `x` along the wall from its start, `z` above its base.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Opening {
    pub x: f64,
    pub z: f64,
    pub width: f64,
    pub height: f64,
}

/// 📦️ Axis-aligned box between two corners.
pub fn cuboid(b: &mut MeshBuilder, lo: P, hi: P) {
    let c = |x: usize, y: usize, z: usize| [if x == 0 { lo[0] } else { hi[0] }, if y == 0 { lo[1] } else { hi[1] }, if z == 0 { lo[2] } else { hi[2] }];
    b.quad_toward([c(0, 0, 0), c(0, 1, 0), c(1, 1, 0), c(1, 0, 0)], [0.0, 0.0, -1.0]);
    b.quad_toward([c(0, 0, 1), c(1, 0, 1), c(1, 1, 1), c(0, 1, 1)], [0.0, 0.0, 1.0]);
    b.quad_toward([c(0, 0, 0), c(1, 0, 0), c(1, 0, 1), c(0, 0, 1)], [0.0, -1.0, 0.0]);
    b.quad_toward([c(0, 1, 0), c(0, 1, 1), c(1, 1, 1), c(1, 1, 0)], [0.0, 1.0, 0.0]);
    b.quad_toward([c(0, 0, 0), c(0, 0, 1), c(0, 1, 1), c(0, 1, 0)], [-1.0, 0.0, 0.0]);
    b.quad_toward([c(1, 0, 0), c(1, 1, 0), c(1, 1, 1), c(1, 0, 1)], [1.0, 0.0, 0.0]);
}

fn rect(w: f64, h: f64) -> Polygon2 {
    Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(w, 0.0), Vec2::new(w, h), Vec2::new(0.0, h)])
}

fn axes(f: &Frame) -> (P, P, P, P) {
    (p(&f.origin), p(&f.x), p(&f.y), p(&f.z))
}

/// 🧱️ Wall panel from `a` to `b` standing on `a`'s level, `thickness` across to its left, with rectangular openings cut as polygon holes.
pub fn wall_with_openings(a: P, b: P, height: f64, thickness: f64, lateral: f64, openings: &[Opening]) -> Result<MeshData, Vec<Violation>> {
    let run = len(sub(b, a));
    let fits = openings.iter().all(|o| o.width > 0.0 && o.height > 0.0 && o.x >= 0.0 && o.z >= 0.0 && o.x + o.width <= run && o.z + o.height < height);
    if !fits {
        return Err(vec![Violation { path: "$.openings".into(), rule: "inside-wall".into() }]);
    }
    let mut notches: Vec<&Opening> = openings.iter().filter(|o| o.z <= 0.0).collect();
    notches.sort_by(|a, c| a.x.total_cmp(&c.x));
    let mut ring = vec![Vec2::new(0.0, 0.0)];
    for o in notches {
        ring.extend([Vec2::new(o.x, 0.0), Vec2::new(o.x, o.height), Vec2::new(o.x + o.width, o.height), Vec2::new(o.x + o.width, 0.0)]);
    }
    ring.extend([Vec2::new(run, 0.0), Vec2::new(run, height), Vec2::new(0.0, height)]);
    ring.dedup_by(|a, c| (a.x - c.x).abs() < 1e-12 && (a.y - c.y).abs() < 1e-12);
    let mut poly = Polygon2::solid(ring);
    poly.holes = openings.iter().filter(|o| o.z > 0.0).map(|o| crate::Ring2 { points: rect(o.width, o.height).outer.points.iter().map(|c| Vec2::new(c.x + o.x, c.y + o.z)).collect() }).collect();
    let dir = unit(sub(b, a));
    let n = unit(cross([0.0, 0.0, 1.0], dir));
    let mut builder = MeshBuilder::new();
    prism(&mut builder, &prepare(&[poly]), add(a, mul(n, lateral)), dir, [0.0, 0.0, 1.0], n, thickness);
    Ok(builder.build(35.0))
}

/// 🧊️ Solid parts with their material roles where the shape carries them (strata, wall layers).
pub fn solid_parts(shape: &SolidShape, tol: Tolerance, lod: Lod) -> Result<Vec<(Option<MaterialRole>, MeshData)>, Vec<Violation>> {
    shape.validate()?;
    let one = |f: &dyn Fn(&mut MeshBuilder)| {
        let mut b = MeshBuilder::new();
        f(&mut b);
        b.build(35.0)
    };
    Ok(match shape {
        SolidShape::Prism { frame, base, height } => {
            let (o, x, y, z) = axes(frame);
            vec![(None, one(&|b| prism(b, &prepare(std::slice::from_ref(base)), o, x, y, z, *height)))]
        }
        SolidShape::Box { frame, size } => {
            let (o, x, y, z) = axes(frame);
            let m = one(&|b| cuboid(b, [-size.x / 2.0, -size.y / 2.0, 0.0], [size.x / 2.0, size.y / 2.0, size.z]));
            vec![(None, m.placed(o, x, y, z))]
        }
        SolidShape::Cylinder { frame, radius, height } => {
            let (o, x, y, z) = axes(frame);
            vec![(None, one(&|b| revolve(b, &[[0.0, 0.0], [*radius, 0.0], [*radius, *height], [0.0, *height]], o, x, y, z, std::f64::consts::TAU, tol, lod)))]
        }
        SolidShape::Wedge { frame, base, height } => {
            let (o, x, y, z) = axes(frame);
            let tri = Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(base.x, 0.0), Vec2::new(0.0, *height)]);
            vec![(None, one(&|b| prism(b, &prepare(std::slice::from_ref(&tri)), o, x, z, y, base.y)))]
        }
        SolidShape::Mesh { vertices, triangles } => vec![(None, one(&|b| triangles.iter().for_each(|t| b.tri(p(&vertices[t.a as usize]), p(&vertices[t.b as usize]), p(&vertices[t.c as usize])))))],
        SolidShape::StrataColumn { frame, footprint, strata } => {
            let (o, x, y, z) = axes(frame);
            let mut depth = 0.0;
            strata.iter().map(|s| {
                depth += s.thickness;
                (Some(s.role.clone()), one(&|b| prism(b, &prepare(std::slice::from_ref(footprint)), add(o, mul(z, -depth)), x, y, z, s.thickness)))
            }).collect()
        }
        SolidShape::FootingBody { frame, footprint, thickness, pedestal, pedestal_height } => {
            let (o, x, y, z) = axes(frame);
            let mut parts = vec![(Some(MaterialRole::Concrete), one(&|b| prism(b, &prepare(std::slice::from_ref(footprint)), o, x, y, z, *thickness)))];
            if let (Some(ped), true) = (pedestal, *pedestal_height > 0.0) {
                let base = add(o, mul(z, *thickness));
                parts.push((Some(MaterialRole::Concrete), one(&|b| cuboid(b, [-ped.x / 2.0, -ped.y / 2.0, 0.0], [ped.x / 2.0, ped.y / 2.0, *pedestal_height])).placed(base, x, y, z)));
            }
            parts
        }
        SolidShape::WallAssembly { path, height, layers, openings } => {
            let total: f64 = layers.iter().map(|l| l.thickness).sum();
            let pts: Vec<P> = path.points.iter().map(p).collect();
            let mut parts = Vec::new();
            let mut start = 0.0;
            for w in pts.windows(2) {
                let run = len(sub(w[1], w[0]));
                let cuts: Vec<Opening> = openings.iter().filter(|o| o.offset < start + run - 1e-9 && o.offset + o.width > start + 1e-9).map(|o| Opening { x: o.offset - start, z: o.sill, width: o.width, height: o.height }).collect();
                let mut lateral = -total / 2.0;
                for l in layers {
                    parts.push((Some(l.role.clone()), wall_with_openings(w[0], w[1], *height, l.thickness, lateral, &cuts)?));
                    lateral += l.thickness;
                }
                start += run;
            }
            parts
        }
        SolidShape::Laminate { frame, footprint, layers } => {
            let (o, x, y, z) = axes(frame);
            let mut rise = 0.0;
            layers.iter().map(|l| {
                let base = add(o, mul(z, rise));
                rise += l.thickness;
                (Some(l.role.clone()), one(&|b| prism(b, &prepare(std::slice::from_ref(footprint)), base, x, y, z, l.thickness)))
            }).collect()
        }
    })
}

/// 🧊️ Merged solid mesh.
pub fn solid_mesh(shape: &SolidShape, tol: Tolerance, lod: Lod) -> Result<MeshData, Vec<Violation>> {
    Ok(MeshData::merged(solid_parts(shape, tol, lod)?.into_iter().map(|(_, m)| m)))
}

/// 🪈️ Vertical bored pile from `head` down `length` metres, `diameter` wide (frame with `z` pointing down).
pub fn pile(head: Vec3, length: f64, diameter: f64) -> SolidShape {
    SolidShape::Cylinder { frame: Frame { origin: head, x: Vec3::X, y: Vec3::new(0.0, -1.0, 0.0), z: Vec3::new(0.0, 0.0, -1.0) }, radius: diameter / 2.0, height: length }
}
