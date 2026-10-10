//! 🌀️ MEP elements as solids: the section swept along the centre line, one mitred prism per segment (the ends of two neighbouring prisms lie in the bisector plane of their bend), one solid per element. The solid has one
//! group named after the system of the element, which is what the renderers colour (`mep::part_colour`).
//!
//! The frame of a segment keeps duct and tray upright: `width` along the horizontal normal of the run, `height` along the up direction in the vertical plane of the run. A vertical run takes the horizontal normal of the run
//! it continues, so the rectangle does not turn on a riser. The prism between two mitre planes has exactly `section area x centre-line length`, which is the closed form of the value of the element; a pipe is tessellated within the
//! chord tolerance of the element solids like the circle of a family profile.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};
use crate::standards::v1::subsets::any::schema::inferences::mep::{key, MepSectionKind, MepValue};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::TriMesh;
use semio_framework_geometry::vector::{add3, cross3, dot3, normalize3, scale3, sub3, Xyz};
use semio_framework_geometry::Point;

/// 📏️ A run whose direction has a vertical component above this (cosine of the angle to the vertical) is vertical.
const VERTICAL: f64 = 0.999_999;
/// 📏️ The smallest cosine between a mitre normal and a run: a sharper bend is clamped.
const MITRE_LIMIT: f64 = 0.2;

struct Frame {
    d: Xyz,
    r: Xyz,
    u: Xyz,
}

fn upright(d: Xyz) -> Frame {
    let z = [0.0, 0.0, 1.0];
    let u = normalize3(sub3(z, scale3(d, dot3(z, d))));
    Frame { d, r: cross3(d, u), u }
}

fn carried(d: Xyz, reference: Xyz) -> Frame {
    let mut r = normalize3(sub3(reference, scale3(d, dot3(reference, d))));
    if dot3(r, r) < 0.5 {
        r = normalize3(sub3([1.0, 0.0, 0.0], scale3(d, d[0])));
    }
    Frame { d, u: cross3(r, d), r }
}

fn frames(directions: &[Xyz]) -> Vec<Frame> {
    let mut found: Vec<Option<Frame>> = directions.iter().map(|d| (d[2].abs() < VERTICAL).then(|| upright(*d))).collect();
    let mut reference: Option<Xyz> = None;
    for index in 0..found.len() {
        match (&found[index], reference) {
            (Some(frame), _) => reference = Some(frame.r),
            (None, Some(r)) => found[index] = Some(carried(directions[index], r)),
            (None, None) => {}
        }
    }
    let mut reference: Option<Xyz> = None;
    for index in (0..found.len()).rev() {
        match (&found[index], reference) {
            (Some(frame), _) => reference = Some(frame.r),
            (None, Some(r)) => found[index] = Some(carried(directions[index], r)),
            (None, None) => {}
        }
    }
    found.into_iter().zip(directions).map(|(frame, d)| frame.unwrap_or_else(|| carried(*d, [1.0, 0.0, 0.0]))).collect()
}

fn section_ring(value: &MepValue) -> Vec<(f64, f64)> {
    match value.section.kind {
        MepSectionKind::Pipe => {
            let radius = value.section.width / 2.0;
            loops::flatten(&[Vertex::new(Point::new(radius, 0.0), 1.0), Vertex::new(Point::new(-radius, 0.0), 1.0)], CHORD_TOLERANCE).into_iter().map(|point| (point.x, point.y)).collect()
        }
        MepSectionKind::Duct | MepSectionKind::Tray => {
            let (w, h) = (value.section.width / 2.0, value.section.height / 2.0);
            vec![(-w, -h), (w, -h), (w, h), (-w, h)]
        }
    }
}

fn ring_at(point: Xyz, frame: &Frame, section: &[(f64, f64)], normal: Xyz) -> Vec<Xyz> {
    let slant = dot3(normal, frame.d).max(MITRE_LIMIT);
    section
        .iter()
        .map(|(a, b)| {
            let offset = add3(scale3(frame.r, *a), scale3(frame.u, *b));
            add3(add3(point, offset), scale3(frame.d, -dot3(normal, offset) / slant))
        })
        .collect()
}

fn prism(start: &[Xyz], end: &[Xyz]) -> TriMesh {
    let count = start.len();
    let mut triangles: Vec<[Xyz; 3]> = Vec::with_capacity(4 * count);
    for j in 0..count {
        let k = (j + 1) % count;
        triangles.push([start[j], start[k], end[k]]);
        triangles.push([start[j], end[k], end[j]]);
    }
    for j in 1..count - 1 {
        triangles.push([start[0], start[j + 1], start[j]]);
        triangles.push([end[0], end[j], end[j + 1]]);
    }
    let volume: f64 = triangles.iter().map(|[a, b, c]| dot3(*a, cross3(*b, *c)) / 6.0).sum();
    let mut mesh = TriMesh::new();
    for [a, b, c] in triangles {
        if volume < 0.0 {
            mesh.push_triangle(a, c, b);
        } else {
            mesh.push_triangle(a, b, c);
        }
    }
    mesh
}

/// 🌀️ The solid of an element: empty while the element is not buildable.
pub fn mep_solid(value: &MepValue) -> ElementSolid {
    let mut builder = SolidBuilder::new(SolidFamily::Mep);
    if !value.buildable() {
        return builder.build();
    }
    let at = |point: &crate::Point3| [point.x, point.y, point.z];
    let directions: Vec<Xyz> = value.segments.iter().map(|segment| normalize3(sub3(at(&segment.to), at(&segment.from)))).collect();
    let frames = frames(&directions);
    let normals: Vec<Xyz> = (0..=directions.len())
        .map(|vertex| {
            if vertex == 0 {
                directions[0]
            } else if vertex == directions.len() {
                directions[vertex - 1]
            } else {
                let sum = add3(directions[vertex - 1], directions[vertex]);
                if dot3(sum, sum) > 1e-12 {
                    normalize3(sum)
                } else {
                    directions[vertex]
                }
            }
        })
        .collect();
    let section = section_ring(value);
    let mut mesh = TriMesh::new();
    for (index, segment) in value.segments.iter().enumerate() {
        let start = ring_at(at(&segment.from), &frames[index], &section, normals[index]);
        let end = ring_at(at(&segment.to), &frames[index], &section, normals[index + 1]);
        mesh.append(&prism(&start, &end));
    }
    let mesh = if value.section.kind == MepSectionKind::Pipe { mesh.crease_normals(35f64.to_radians()) } else { mesh };
    builder.add(key(value.system), "", 0, &mesh);
    builder.build()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
