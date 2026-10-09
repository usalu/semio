//! 👁️ Projection with hidden-line removal: what a vertical view sees of the triangle meshes of the solids behind its plane, drawn in orthographic projection onto `(u, z)` with the distance `w` behind
//! the plane kept for the depth test.
//!
//! The meshes are clipped to the slab of the view first. The *feature edges* are the silhouettes (an edge between a front-facing and a back-facing triangle, or whose neighbour is edge-on) and the creases
//! (an angle of 30 degrees or more between the normals of its triangles); a smooth, finely tessellated surface has none inside. An edge is hidden where a nearer front-facing triangle covers it: the
//! covered intervals are found per edge against the triangles of a uniform grid, exactly for polyhedra, so prisms (walls, columns, beams, slabs) come out exact. A triangle is shrunk by [`INSET`] for the test, so
//! an edge that runs along the silhouette of a nearer element stays visible, and the identical edges that the front and the back of a prism project to are drawn once.
//!
//! [`faces`] merges the coplanar front-facing triangles of an element into its planar faces and unites them: the silhouette of the element, exact to the corner.

use super::clip::{clip_triangle, slab};
use super::cut::{group, Cut};
use super::frame::{Drawn, Frame, World};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidFamily};
use semio_framework_2d::booleans::BooleanOperation;
use semio_framework_2d::regions::{region_boolean, Region};
use semio_framework_geometry::section::chain;
use semio_framework_geometry::vector::{cross3, dot3, normalize3, sub3};
use semio_framework_geometry::Point;
use std::collections::{BTreeMap, BTreeSet, HashMap};

//#region 🔖️Tolerances
/// 📏️ Facing within this of zero is edge-on.
pub const FACE_EPS: f64 = 1e-9;
/// 📐️ Cosine of the crease angle: faces whose normals enclose 30 degrees or more meet in a feature edge.
pub const CREASE_COS: f64 = 0.866_025_403_784_438_6;
/// 📏️ An edge is behind a triangle when it is deeper by more than this many metres.
pub const DEPTH_EPS: f64 = 1e-4;
/// 📏️ A triangle covers only what lies this far inside its outline.
pub const INSET: f64 = 1e-7;
/// 📏️ Visible or hidden pieces shorter than this are dropped.
pub const MIN_PIECE: f64 = 1e-5;
/// 📏️ Vertices closer than this are one vertex.
pub const WELD: f64 = 1e-6;
//#endregion 🔖️Tolerances

//#region 🔖️Bodies
/// 🧱️ How a triangle faces the viewer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Facing {
    Front,
    Back,
    EdgeOn,
}

/// 🔺️ One clipped triangle: its building corners, its view corners `(u, z, w)`, its unit normal and how it faces the viewer.
#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub world: [World; 3],
    pub drawn: [Drawn; 3],
    pub normal: World,
    pub facing: Facing,
}

/// 🧊️ The clipped triangles of one element.
#[derive(Clone, Debug)]
pub struct Body {
    pub element: String,
    pub family: SolidFamily,
    pub tris: Vec<Tri>,
}

/// 🧊️ The bodies a frame sees of `solids` within `depth`: clipped to the slab, projected, classified.
pub fn bodies<'a>(frame: &Frame, depth: f64, solids: impl IntoIterator<Item = (&'a str, &'a ElementSolid)>) -> Vec<Body> {
    let halves = slab(frame, depth);
    let look = frame.look3();
    solids
        .into_iter()
        .filter(|(_, solid)| !solid.is_empty() && frame.sees(&solid.bounds, depth))
        .map(|(element, solid)| {
            let mesh = solid.mesh();
            let tris = (0..mesh.triangle_count())
                .flat_map(|index| clip_triangle(mesh.triangle(index), &halves))
                .filter_map(|corners| {
                    let raw = cross3(sub3(corners[1], corners[0]), sub3(corners[2], corners[0]));
                    let length = dot3(raw, raw).sqrt();
                    (length > 1e-12).then(|| {
                        let normal = normalize3(raw);
                        let facing = dot3(normal, look);
                        let facing = if facing < -FACE_EPS { Facing::Front } else if facing > FACE_EPS { Facing::Back } else { Facing::EdgeOn };
                        Tri { world: corners, drawn: corners.map(|corner| frame.draw(corner)), normal, facing }
                    })
                })
                .collect();
            Body { element: element.to_string(), family: solid.family, tris }
        })
        .filter(|body| !body.tris.is_empty())
        .collect()
}
//#endregion 🔖️Bodies

//#region 🔖️Welding
type Quantised = (i64, i64, i64);

fn quantise(point: World) -> Quantised {
    ((point[0] / WELD).round() as i64, (point[1] / WELD).round() as i64, (point[2] / WELD).round() as i64)
}

/// 🧵️ The triangles of a body as indices into a welded vertex list of view coordinates.
pub struct Welded {
    pub vertices: Vec<Drawn>,
    pub worlds: Vec<World>,
    pub triangles: Vec<[usize; 3]>,
}

/// 🧵️ Welds the corners of a body: corners within [`WELD`] of each other become one vertex, in order of first appearance.
pub fn weld(body: &Body) -> Welded {
    let mut index: HashMap<Quantised, usize> = HashMap::new();
    let mut welded = Welded { vertices: Vec::new(), worlds: Vec::new(), triangles: Vec::with_capacity(body.tris.len()) };
    for tri in &body.tris {
        let ids = [0, 1, 2].map(|corner| {
            *index.entry(quantise(tri.world[corner])).or_insert_with(|| {
                welded.vertices.push(tri.drawn[corner]);
                welded.worlds.push(tri.world[corner]);
                welded.vertices.len() - 1
            })
        });
        welded.triangles.push(ids);
    }
    welded
}
//#endregion 🔖️Welding

//#region 🔖️FeatureEdges
/// 〰️ A feature edge in view coordinates, with the element it belongs to.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub element: String,
    pub a: Drawn,
    pub b: Drawn,
}

fn on_slab(frame: &Frame, depth: f64, point: World) -> bool {
    let (u, w) = (frame.u(point), frame.w(point));
    u.abs() < 10.0 * WELD || (u - frame.length).abs() < 10.0 * WELD || w.abs() < 10.0 * WELD || (depth < 1e6 && (w - depth).abs() < 10.0 * WELD)
}

fn sign(facing: Facing) -> i8 {
    match facing {
        Facing::Front => -1,
        Facing::Back => 1,
        Facing::EdgeOn => 0,
    }
}

fn is_feature(tris: &[&Tri]) -> bool {
    tris.iter().enumerate().any(|(first, a)| {
        tris.iter().skip(first + 1).any(|b| {
            let (sa, sb) = (sign(a.facing), sign(b.facing));
            (sa * sb <= 0 && !(sa == 0 && sb == 0)) || dot3(a.normal, b.normal) < CREASE_COS
        })
    })
}

/// 〰️ The feature edges of a body, in a deterministic order. Boundary edges that lie on a clipping plane of the slab are artefacts of the clipping and are left out.
pub fn feature_edges(body: &Body, frame: &Frame, depth: f64) -> Vec<Edge> {
    let welded = weld(body);
    let mut adjacent: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
    for (index, ids) in welded.triangles.iter().enumerate() {
        for corner in 0..3 {
            let (a, b) = (ids[corner], ids[(corner + 1) % 3]);
            if a != b {
                adjacent.entry((a.min(b), a.max(b))).or_default().push(index);
            }
        }
    }
    adjacent
        .into_iter()
        .filter(|((a, b), owners)| {
            if owners.len() == 1 {
                !(on_slab(frame, depth, welded.worlds[*a]) && on_slab(frame, depth, welded.worlds[*b]))
            } else {
                is_feature(&owners.iter().map(|owner| &body.tris[*owner]).collect::<Vec<_>>())
            }
        })
        .map(|((a, b), _)| Edge { element: body.element.clone(), a: welded.vertices[a], b: welded.vertices[b] })
        .filter(|edge| (edge.b[0] - edge.a[0]).hypot(edge.b[1] - edge.a[1]) > MIN_PIECE)
        .collect()
}
//#endregion 🔖️FeatureEdges

//#region 🔖️Occluders
struct Occluder {
    q: [[f64; 2]; 3],
    base: [f64; 3],
    gradient: [f64; 2],
    low: [f64; 2],
    high: [f64; 2],
}

fn occluder(tri: &Tri) -> Option<Occluder> {
    let [p0, p1, p2] = tri.drawn;
    let det = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]);
    if det.abs() < 1e-12 {
        return None;
    }
    let gu = ((p1[2] - p0[2]) * (p2[1] - p0[1]) - (p2[2] - p0[2]) * (p1[1] - p0[1])) / det;
    let gz = ((p1[0] - p0[0]) * (p2[2] - p0[2]) - (p2[0] - p0[0]) * (p1[2] - p0[2])) / det;
    let q = if det > 0.0 { [[p0[0], p0[1]], [p1[0], p1[1]], [p2[0], p2[1]]] } else { [[p0[0], p0[1]], [p2[0], p2[1]], [p1[0], p1[1]]] };
    let low = [q[0][0].min(q[1][0]).min(q[2][0]), q[0][1].min(q[1][1]).min(q[2][1])];
    let high = [q[0][0].max(q[1][0]).max(q[2][0]), q[0][1].max(q[1][1]).max(q[2][1])];
    Some(Occluder { q, base: p0, gradient: [gu, gz], low, high })
}

struct Grid {
    origin: [f64; 2],
    cell: f64,
    columns: usize,
    rows: usize,
    cells: Vec<Vec<u32>>,
}

impl Grid {
    fn build(occluders: &[Occluder]) -> Self {
        let low = occluders.iter().fold([f64::INFINITY; 2], |acc, o| [acc[0].min(o.low[0]), acc[1].min(o.low[1])]);
        let high = occluders.iter().fold([f64::NEG_INFINITY; 2], |acc, o| [acc[0].max(o.high[0]), acc[1].max(o.high[1])]);
        let (width, height) = ((high[0] - low[0]).max(1e-3), (high[1] - low[1]).max(1e-3));
        let target = (occluders.len() as f64 / 4.0).clamp(1.0, 16384.0);
        let cell = (width * height / target).sqrt().max(width.max(height) / 512.0).max(1e-3);
        let (columns, rows) = (((width / cell).ceil() as usize).max(1), ((height / cell).ceil() as usize).max(1));
        let mut grid = Self { origin: low, cell, columns, rows, cells: vec![Vec::new(); columns * rows] };
        for (index, o) in occluders.iter().enumerate() {
            let ((c0, r0), (c1, r1)) = (grid.locate(o.low), grid.locate(o.high));
            for row in r0..=r1 {
                for column in c0..=c1 {
                    grid.cells[row * columns + column].push(index as u32);
                }
            }
        }
        grid
    }

    fn locate(&self, point: [f64; 2]) -> (usize, usize) {
        let column = (((point[0] - self.origin[0]) / self.cell).floor().max(0.0) as usize).min(self.columns - 1);
        let row = (((point[1] - self.origin[1]) / self.cell).floor().max(0.0) as usize).min(self.rows - 1);
        (column, row)
    }

    fn candidates(&self, low: [f64; 2], high: [f64; 2], stamp: &mut [u32], mark: u32) -> Vec<u32> {
        let ((c0, r0), (c1, r1)) = (self.locate(low), self.locate(high));
        let mut found = Vec::new();
        for row in r0..=r1 {
            for column in c0..=c1 {
                for &index in &self.cells[row * self.columns + column] {
                    if stamp[index as usize] != mark {
                        stamp[index as usize] = mark;
                        found.push(index);
                    }
                }
            }
        }
        found
    }
}

fn inside_interval(q: &[[f64; 2]; 3], a: [f64; 2], d: [f64; 2]) -> Option<(f64, f64)> {
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for index in 0..3 {
        let (p, r) = (q[index], q[(index + 1) % 3]);
        let e = [r[0] - p[0], r[1] - p[1]];
        let length = e[0].hypot(e[1]);
        let g0 = e[0] * (a[1] - p[1]) - e[1] * (a[0] - p[0]) - INSET * length;
        let g1 = e[0] * d[1] - e[1] * d[0];
        if g1.abs() < 1e-18 {
            if g0 < 0.0 {
                return None;
            }
        } else {
            let s = -g0 / g1;
            if g1 > 0.0 {
                lo = lo.max(s);
            } else {
                hi = hi.min(s);
            }
        }
        if lo >= hi {
            return None;
        }
    }
    Some((lo, hi))
}

fn covered(o: &Occluder, a: Drawn, b: Drawn) -> Option<(f64, f64)> {
    let d = [b[0] - a[0], b[1] - a[1]];
    let (lo, hi) = inside_interval(&o.q, [a[0], a[1]], d)?;
    let depth_at = |u: f64, z: f64| o.base[2] + o.gradient[0] * (u - o.base[0]) + o.gradient[1] * (z - o.base[1]);
    let f0 = a[2] - depth_at(a[0], a[1]);
    let f1 = (b[2] - a[2]) - (o.gradient[0] * d[0] + o.gradient[1] * d[1]);
    let (fl, fh) = (f0 + f1 * lo, f0 + f1 * hi);
    if fl <= DEPTH_EPS && fh <= DEPTH_EPS {
        None
    } else if fl > DEPTH_EPS && fh > DEPTH_EPS {
        Some((lo, hi))
    } else {
        let crossing = ((DEPTH_EPS - f0) / f1).clamp(lo, hi);
        Some(if fl > DEPTH_EPS { (lo, crossing) } else { (crossing, hi) })
    }
}

fn merged(mut intervals: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out: Vec<(f64, f64)> = Vec::new();
    for (lo, hi) in intervals {
        match out.last_mut() {
            Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
            _ => out.push((lo, hi)),
        }
    }
    out
}

fn complement(hidden: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let mut at = 0.0;
    for &(lo, hi) in hidden {
        if lo > at {
            out.push((at, lo));
        }
        at = at.max(hi);
    }
    if at < 1.0 {
        out.push((at, 1.0));
    }
    out
}
//#endregion 🔖️Occluders

//#region 🔖️Pieces
/// 〰️ A visible or hidden piece of a feature edge in `(u, z)`.
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub element: String,
    pub a: [f64; 2],
    pub b: [f64; 2],
    pub visible: bool,
}

fn key(point: [f64; 2]) -> (i64, i64) {
    ((point[0] / WELD).round() as i64, (point[1] / WELD).round() as i64)
}

/// 👁️ The visible and hidden pieces of `edges` against the front-facing triangles of `bodies` and the `caps` (the faces a section closes its cut with). Identical pieces (the same two end points) are kept once, the visible one winning.
pub fn pieces(edges: &[Edge], bodies: &[Body], caps: &[Tri]) -> Vec<Piece> {
    let occluders: Vec<Occluder> = bodies.iter().flat_map(|body| body.tris.iter()).chain(caps.iter()).filter(|tri| tri.facing == Facing::Front).filter_map(occluder).collect();
    let grid = (!occluders.is_empty()).then(|| Grid::build(&occluders));
    let mut stamp = vec![0_u32; occluders.len()];
    let mut seen: BTreeMap<((i64, i64), (i64, i64)), usize> = BTreeMap::new();
    let mut out: Vec<Piece> = Vec::new();
    for (index, edge) in edges.iter().enumerate() {
        let (low, high) = ([edge.a[0].min(edge.b[0]), edge.a[1].min(edge.b[1])], [edge.a[0].max(edge.b[0]), edge.a[1].max(edge.b[1])]);
        let hidden = grid.as_ref().map_or_else(Vec::new, |grid| merged(grid.candidates(low, high, &mut stamp, index as u32 + 1).into_iter().filter_map(|candidate| covered(&occluders[candidate as usize], edge.a, edge.b)).collect()));
        let length = (edge.b[0] - edge.a[0]).hypot(edge.b[1] - edge.a[1]);
        let at = |s: f64| [edge.a[0] + (edge.b[0] - edge.a[0]) * s, edge.a[1] + (edge.b[1] - edge.a[1]) * s];
        let visible = complement(&hidden).into_iter().map(|span| (span, true));
        let obscured = hidden.iter().map(|span| (*span, false));
        for ((lo, hi), shown) in visible.chain(obscured) {
            if (hi - lo) * length < MIN_PIECE {
                continue;
            }
            let (a, b) = (at(lo), at(hi));
            let ends = if key(a) <= key(b) { (key(a), key(b)) } else { (key(b), key(a)) };
            match seen.get(&ends) {
                Some(&position) => {
                    if shown && !out[position].visible {
                        out[position].visible = true;
                    }
                }
                None => {
                    seen.insert(ends, out.len());
                    out.push(Piece { element: edge.element.clone(), a, b, visible: shown });
                }
            }
        }
    }
    out
}
//#endregion 🔖️Pieces

//#region 🔖️Faces
fn find(parents: &mut [usize], mut node: usize) -> usize {
    while parents[node] != node {
        parents[node] = parents[parents[node]];
        node = parents[node];
    }
    node
}

fn area2(p: [Drawn; 3]) -> f64 {
    (p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[2][0] - p[0][0]) * (p[1][1] - p[0][1])
}

/// ▭️ The silhouette of an element: its coplanar front-facing triangles merged into planar faces, whose outlines are the boundary edges of each merged face, then united. Outer rings run counter-clockwise, holes clockwise.
pub fn faces(body: &Body) -> Vec<Cut> {
    let welded = weld(body);
    let front: Vec<usize> = (0..body.tris.len()).filter(|index| body.tris[*index].facing == Facing::Front && area2(body.tris[*index].drawn).abs() > 1e-12).collect();
    let mut parents: Vec<usize> = (0..body.tris.len()).collect();
    let mut owner: HashMap<(usize, usize), usize> = HashMap::new();
    for &index in &front {
        let ids = welded.triangles[index];
        for corner in 0..3 {
            let (a, b) = (ids[corner], ids[(corner + 1) % 3]);
            let edge = (a.min(b), a.max(b));
            match owner.get(&edge) {
                Some(&other) if dot3(body.tris[index].normal, body.tris[other].normal) > 1.0 - 1e-9 => {
                    let (x, y) = (find(&mut parents, index), find(&mut parents, other));
                    parents[x] = y;
                }
                Some(_) => {}
                None => {
                    owner.insert(edge, index);
                }
            }
        }
    }
    let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &index in &front {
        let root = find(&mut parents, index);
        groups.entry(root).or_default().push(index);
    }
    let mut regions: Vec<Region> = Vec::new();
    for members in groups.values() {
        let mut directed: BTreeSet<(usize, usize)> = BTreeSet::new();
        for &index in members {
            let ids = welded.triangles[index];
            let ids = if area2(body.tris[index].drawn) > 0.0 { ids } else { [ids[0], ids[2], ids[1]] };
            for corner in 0..3 {
                directed.insert((ids[corner], ids[(corner + 1) % 3]));
            }
        }
        let segments: Vec<[Point; 2]> = directed.iter().filter(|(a, b)| !directed.contains(&(*b, *a))).map(|(a, b)| [Point::new(welded.vertices[*a][0], welded.vertices[*a][1]), Point::new(welded.vertices[*b][0], welded.vertices[*b][1])]).collect();
        let rings = chain(&segments, WELD).into_iter().filter(|line| line.closed && line.points.len() >= 3).map(|line| line.points.iter().map(|p| [p.x, p.y]).collect()).collect();
        regions.extend(group(rings).into_iter().map(|cut| Region::new(&cut.outer, &cut.holes)));
    }
    if regions.len() <= 1 {
        return regions.into_iter().map(|region| Cut { outer: region.outer, holes: region.holes }).collect();
    }
    match region_boolean(BooleanOperation::Union, &regions, &[], &mut |_| true) {
        Ok(united) => united.into_iter().map(|region| Cut { outer: region.outer, holes: region.holes }).collect(),
        Err(_) => regions.into_iter().map(|region| Cut { outer: region.outer, holes: region.holes }).collect(),
    }
}

/// 🔭️ The mean depth behind the plane of a body's front-facing triangles: the order the faces of a view are painted in, farthest first.
pub fn mean_depth(body: &Body) -> f64 {
    let front: Vec<&Tri> = body.tris.iter().filter(|tri| tri.facing == Facing::Front).collect();
    if front.is_empty() {
        return 0.0;
    }
    front.iter().map(|tri| (tri.drawn[0][2] + tri.drawn[1][2] + tri.drawn[2][2]) / 3.0).sum::<f64>() / front.len() as f64
}
//#endregion 🔖️Faces
