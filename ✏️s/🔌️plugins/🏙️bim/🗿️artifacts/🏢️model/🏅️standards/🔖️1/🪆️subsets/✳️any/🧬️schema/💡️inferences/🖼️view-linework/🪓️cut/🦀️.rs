//! 🪓️ The cut of a section: the vertical plane through the section line cuts the triangle mesh of a solid, the chained segments are the outlines of the material (counter-clockwise) and of its
//! holes (clockwise), in view coordinates `(u, z)`. The cut is taken from the whole mesh and clipped to the plane's length afterwards, so a cut element stays closed.

use super::clip::{clip_ring, Rect};
use super::frame::Frame;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use semio_framework_geometry::section::{chain, section_plane};

/// 📏️ The classification band, in metres, of the plane section.
pub const SECTION_EPSILON: f64 = 1e-6;

/// 🪓️ One cut region: an outer ring and its holes in `(u, z)`.
#[derive(Clone, Debug, PartialEq)]
pub struct Cut {
    pub outer: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
}

fn area(ring: &[[f64; 2]]) -> f64 {
    let count = ring.len();
    (0..count).map(|index| ring[index][0] * ring[(index + 1) % count][1] - ring[(index + 1) % count][0] * ring[index][1]).sum::<f64>() / 2.0
}

/// 🔎️ Whether `point` lies inside the ring (even-odd).
pub fn inside(ring: &[[f64; 2]], point: [f64; 2]) -> bool {
    let count = ring.len();
    (0..count).fold(false, |odd, index| {
        let (a, b) = (ring[index], ring[(index + 1) % count]);
        odd ^ ((a[1] > point[1]) != (b[1] > point[1]) && point[0] < (b[0] - a[0]) * (point[1] - a[1]) / (b[1] - a[1]) + a[0])
    })
}

/// 🧩️ Groups closed rings into regions: a counter-clockwise ring is an outer, a clockwise ring a hole of the smallest outer that contains it; a hole in no outer is dropped.
pub fn group(rings: Vec<Vec<[f64; 2]>>) -> Vec<Cut> {
    let mut cuts: Vec<Cut> = rings.iter().filter(|ring| area(ring) > 0.0).map(|ring| Cut { outer: ring.clone(), holes: Vec::new() }).collect();
    for hole in rings.iter().filter(|ring| area(ring) < 0.0) {
        let owner = cuts.iter_mut().filter(|cut| inside(&cut.outer, hole[0])).min_by(|a, b| area(&a.outer).total_cmp(&area(&b.outer)));
        if let Some(cut) = owner {
            cut.holes.push(hole.clone());
        }
    }
    cuts
}

/// 🪓️ The regions the plane of `frame` cuts out of `solid`, clipped to `[0, length]` along the plane; none when the solid does not reach the plane.
pub fn cut_regions(solid: &ElementSolid, frame: &Frame) -> Vec<Cut> {
    if !frame.crosses(&solid.bounds) {
        return Vec::new();
    }
    let origin = [frame.origin[0], frame.origin[1], 0.0];
    let normal = [-frame.look[0], -frame.look[1], 0.0];
    let along = [frame.along[0], frame.along[1], 0.0];
    let segments = section_plane(&solid.mesh(), origin, normal, along, SECTION_EPSILON);
    let rect = Rect { x0: 0.0, y0: f64::NEG_INFINITY, x1: frame.length, y1: f64::INFINITY };
    let rings: Vec<Vec<[f64; 2]>> = chain(&segments, SECTION_EPSILON).into_iter().filter(|line| line.closed && line.points.len() >= 3).map(|line| line.points.iter().map(|p| [p.x, p.y]).collect()).collect();
    group(rings)
        .into_iter()
        .filter_map(|cut| {
            let outer = clip_ring(&cut.outer, &rect);
            (!outer.is_empty()).then(|| Cut { outer, holes: cut.holes.iter().map(|hole| clip_ring(hole, &rect)).filter(|hole| !hole.is_empty()).collect() })
        })
        .collect()
}
