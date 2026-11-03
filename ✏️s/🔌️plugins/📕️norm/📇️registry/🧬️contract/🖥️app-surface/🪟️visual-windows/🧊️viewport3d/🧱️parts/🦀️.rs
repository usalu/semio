//! 🧱️ Parts: the placed, instanced meshes one scene entity lowers to; `SceneTessellator` is the interface a richer tessellation kernel plugs into.

use super::mesh::*;
use super::playback::{motion_factor, Frame};
use super::pose::*;
use super::scene::{self, Geometry, LoadGlyph, MaterialRole, SceneEntity, StructuralScene};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lod {
    pub stride: usize,
    pub detail: bool,
}

impl Lod {
    pub const FULL: Self = Self { stride: 1, detail: true };
    pub const REDUCED: Self = Self { stride: 2, detail: true };
    pub const COARSE: Self = Self { stride: 3, detail: false };
}

pub struct PartContext<'a> {
    pub scene: &'a StructuralScene,
    pub lod: Lod,
    pub glyph: f64,
    pub deformation_scale: f64,
    pub frame: Frame,
}

#[derive(Clone, Debug)]
pub struct Part {
    pub key: String,
    pub mesh: Arc<MeshData>,
    pub pose: Pose,
    pub role: Option<MaterialRole>,
    pub station: Option<(f64, f64)>,
    pub ghost: bool,
    pub grid: Option<(usize, usize)>,
    pub ink: bool,
}

/// 🔌️ Turns one scene entity into placed parts; implementations may replace [`BasicTessellator`] with a full geometry kernel.
pub trait SceneTessellator {
    fn parts(&self, entity: &SceneEntity, ctx: &PartContext) -> Vec<Part>;
}

pub fn fnv_hex(text: &str) -> String {
    format!("{:016x}", text.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)))
}

#[derive(Default)]
pub struct BasicTessellator {
    cache: RefCell<HashMap<String, Arc<MeshData>>>,
}

impl BasicTessellator {
    pub fn cached_meshes(&self) -> usize {
        self.cache.borrow().len()
    }

    fn mesh(&self, key: &str, build: impl FnOnce() -> MeshData) -> Arc<MeshData> {
        self.cache.borrow_mut().entry(key.to_string()).or_insert_with(|| Arc::new(build())).clone()
    }

    fn part(&self, key: &str, pose: Pose, build: impl FnOnce() -> MeshData) -> Part {
        Part { key: key.to_string(), mesh: self.mesh(key, build), pose, role: None, station: None, ghost: false, grid: None, ink: false }
    }

    fn box_part(&self, pose: Pose) -> Part {
        self.part("box", pose, unit_box)
    }

    fn cylinder_part(&self, pose: Pose, lod: Lod) -> Part {
        let segments = if lod.detail { 24 } else { 8 };
        self.part(&format!("cylinder:{segments}"), pose, || unit_cylinder(segments))
    }

    fn cone_part(&self, pose: Pose, lod: Lod) -> Part {
        let segments = if lod.detail { 16 } else { 6 };
        self.part(&format!("cone:{segments}"), pose, || unit_cone(segments))
    }

    fn prism_part(&self, polygon: &scene::Polygon2, pose: Pose, lod: Lod) -> Part {
        let key = format!("prism:{}:{}", fnv_hex(&scene::canonical_json(polygon)), lod.stride);
        self.part(&key, pose, || prism(polygon, lod.stride))
    }

    fn fresh(&self, key: String, mesh: MeshData) -> Part {
        Part { key, mesh: Arc::new(mesh), pose: Pose { position: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: [1.0; 3] }, role: None, station: None, ghost: false, grid: None, ink: false }
    }

    fn gridded(&self, entity: &SceneEntity, tag: &str, rows: usize, columns: usize, points: &[V3]) -> Part {
        Part { grid: Some((rows, columns)), ..self.custom(entity, tag, grid(rows, columns, points)) }
    }

    fn custom(&self, entity: &SceneEntity, tag: &str, mesh: MeshData) -> Part {
        let key = format!("{tag}:{}:{}", entity.entity.id, entity.content_hash);
        self.part(&key, Pose { position: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: [1.0; 3] }, || mesh)
    }

    /// 🏁️ Hatch ribbons lying on a planar face: `angle` turns `Horizontal` away from the frame x axis, spacing follows the glyph size but never exceeds 40 lines over the face.
    fn hatch_part(&self, polygon: &scene::Polygon2, hatch: scene::Hatch, angle: f64, frame: (V3, V3, V3), at: V3, ctx: &PartContext) -> Part {
        let reach = polygon.outer.points.iter().fold(0.0_f64, |m, p| m.max(p.x.abs()).max(p.y.abs())) * 2.0;
        let spacing = (ctx.glyph * 0.3).max(reach / 40.0).max(1e-6);
        let width = spacing * 0.12;
        let name = match hatch { scene::Hatch::Diagonal => "diagonal", scene::Hatch::CrossDiagonal => "cross", scene::Hatch::Horizontal => "horizontal", scene::Hatch::Dots => "dots", scene::Hatch::Vertical => "vertical" };
        let key = format!("hatch:{name}:{}:{:x}:{:x}", fnv_hex(&scene::canonical_json(polygon)), angle.to_bits(), spacing.to_bits());
        let pose = basis_pose(add(at, mul(frame.2, width * 0.5)), frame.0, frame.1, frame.2, [1.0; 3]);
        Part { ink: true, ..self.part(&key, pose, || hatch_ribbons(polygon, hatch, angle, spacing, width)) }
    }

    /// 🌾️ Grain and cue-pattern lines of an entity on the top faces of its solids and plates: boxes, prisms and plates take `Appearance.hatch`, laminate layers always show their fibre direction (`GrainLayer.grain`) as lines along it.
    fn hatching(&self, entity: &SceneEntity, ctx: &PartContext) -> Vec<Part> {
        let hatch = entity.appearance.hatch;
        match &entity.geometry {
            Geometry::Solid(s) => match &s.shape {
                scene::SolidShape::Prism { frame, base, height } => hatch.map(|h| { let f = frame_axes(frame); self.hatch_part(base, h, 0.0, f, add(v3(&frame.origin), mul(f.2, *height)), ctx) }).into_iter().collect(),
                scene::SolidShape::Box { frame, size } => hatch.map(|h| { let f = frame_axes(frame); self.hatch_part(&rect_polygon(size.x, size.y), h, 0.0, f, add(v3(&frame.origin), mul(f.2, size.z)), ctx) }).into_iter().collect(),
                scene::SolidShape::Laminate { frame, footprint, layers } => {
                    let f = frame_axes(frame);
                    let mut rise = 0.0;
                    layers.iter().map(|l| { rise += l.thickness; self.hatch_part(footprint, scene::Hatch::Horizontal, l.grain, f, add(v3(&frame.origin), mul(f.2, rise)), ctx) }).collect()
                }
                _ => Vec::new(),
            },
            Geometry::Plate(g) => hatch.map(|h| { let f = frame_axes(&g.frame); self.hatch_part(&g.outline, h, 0.0, f, add(v3(&g.frame.origin), mul(f.2, g.offset + g.thickness)), ctx) }).into_iter().collect(),
            _ => Vec::new(),
        }
    }

    fn arrow(&self, tip: V3, dir: V3, length: f64, ctx: &PartContext, double: bool) -> Vec<Part> {
        let (dir, head) = (unit(dir), (length * 0.35).max(ctx.glyph * 0.15).min(length));
        let (thick, wide) = (ctx.glyph * 0.06, ctx.glyph * 0.2);
        let hint = orthogonal(dir, [0.0, 0.0, 1.0]);
        let mut parts = vec![
            self.box_part(span_pose(sub(tip, mul(dir, length)), sub(tip, mul(dir, head)), hint, (thick, thick))),
            self.cone_part(span_pose(sub(tip, mul(dir, head)), tip, hint, (wide, wide)), ctx.lod),
        ];
        if double {
            parts.push(self.cone_part(span_pose(sub(tip, mul(dir, 2.0 * head)), sub(tip, mul(dir, head)), hint, (wide, wide)), ctx.lod));
        }
        parts
    }
}

fn glyph_length(glyph: f64, magnitude: f64) -> f64 {
    glyph * (0.5 + 0.5 * ((1.0 + magnitude.abs()).ln() / (1.0e7_f64).ln()).min(1.0))
}

pub fn arc_points(centre: V3, normal: V3, start: V3, sweep: f64, n: usize) -> Vec<V3> {
    let (k, v) = (unit(normal), sub(start, centre));
    (0..=n).map(|i| {
        let a = sweep * i as f64 / n as f64;
        add(centre, add(add(mul(v, a.cos()), mul(cross(k, v), a.sin())), mul(k, dot(k, v) * (1.0 - a.cos()))))
    }).collect()
}

/// 🧱️ Rectangles `(x0, x1, z0, z1)` of a wall of `run` by `height` outside the `cuts`: full-height piers between cuts, sill below and lintel above each cut.
fn wall_pieces(run: f64, height: f64, cuts: &[(f64, f64, f64, f64)]) -> Vec<(f64, f64, f64, f64)> {
    let mut sorted = cuts.to_vec();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (mut pieces, mut x) = (Vec::new(), 0.0);
    for (x0, x1, z0, z1) in sorted {
        let (x0, x1) = (x0.max(x), x1.min(run));
        if x0 > x + 1e-9 {
            pieces.push((x, x0, 0.0, height));
        }
        if z0 > 1e-9 {
            pieces.push((x0, x1, 0.0, z0));
        }
        if z1 < height - 1e-9 {
            pieces.push((x0, x1, z1, height));
        }
        x = x1;
    }
    if run > x + 1e-9 {
        pieces.push((x, run, 0.0, height));
    }
    pieces
}

pub fn path_points(path: &scene::Path3) -> Vec<V3> {
    match path {
        scene::Path3::Polyline(p) => p.points.iter().map(v3).collect(),
        scene::Path3::Arc(a) => arc_points(v3(&a.centre), v3(&a.normal), v3(&a.start), a.sweep, 16),
        scene::Path3::Spline(s) => s.control.iter().map(v3).collect(),
    }
}

fn polyline_length(points: &[V3]) -> f64 {
    points.windows(2).map(|w| len(sub(w[1], w[0]))).sum()
}

pub fn point_at(points: &[V3], t: f64) -> V3 {
    let (total, mut run) = (polyline_length(points), 0.0);
    let target = t.clamp(0.0, 1.0) * total;
    for w in points.windows(2) {
        let seg = len(sub(w[1], w[0]));
        if run + seg >= target && seg > 0.0 { return lerp(w[0], w[1], (target - run) / seg); }
        run += seg;
    }
    points.last().copied().unwrap_or([0.0; 3])
}

fn displacement(samples: &[scene::DisplacementSample], t: f64) -> V3 {
    let Some(first) = samples.first() else { return [0.0; 3] };
    if t <= first.at { return v3(&first.delta); }
    samples.windows(2).find(|w| t <= w[1].at).map_or_else(|| v3(&samples[samples.len() - 1].delta), |w| lerp(v3(&w[0].delta), v3(&w[1].delta), (t - w[0].at) / (w[1].at - w[0].at).max(1e-12)))
}

fn overlay_factor(entity: &SceneEntity, ctx: &PartContext) -> f64 {
    entity.appearance.deformation.as_ref().map_or(0.0, |o| o.scale * ctx.deformation_scale * motion_factor(o.motion, ctx.frame))
}

/// 🌊️ Displacement at a plane point by inverse-distance weighting (power 2) of the surface samples; a sample hit returns its own delta.
pub fn surface_delta(samples: &[scene::SurfaceSample], at: [f64; 2]) -> V3 {
    let (mut sum, mut weights) = ([0.0; 3], 0.0);
    for s in samples {
        let d2 = (s.at.x - at[0]).powi(2) + (s.at.y - at[1]).powi(2);
        if d2 < 1e-18 { return v3(&s.delta); }
        sum = add(sum, mul(v3(&s.delta), 1.0 / d2));
        weights += 1.0 / d2;
    }
    if weights > 0.0 { mul(sum, 1.0 / weights) } else { [0.0; 3] }
}

/// 📐️ Physical surface coordinates of a row-major shell grid in metres: the arc length along the columns (averaged over all rows) of every column and along the rows (averaged over all columns) of every row, both starting at 0.
pub fn surface_coordinates(points: &[V3], rows: usize, columns: usize) -> (Vec<f64>, Vec<f64>) {
    let run = |count: usize, others: usize, node: &dyn Fn(usize, usize) -> usize| -> Vec<f64> {
        let mut at = vec![0.0; count];
        for i in 1..count {
            let step: f64 = (0..others).map(|o| len(sub(points[node(i, o)], points[node(i - 1, o)]))).sum();
            at[i] = at[i - 1] + step / others.max(1) as f64;
        }
        at
    };
    (run(columns, rows, &|c, r| r * columns + c), run(rows, columns, &|r, c| r * columns + c))
}

fn inside(polygon: &scene::Polygon2, x: f64, y: f64) -> bool {
    let crossings = |ring: &scene::Ring2| {
        let n = ring.points.len();
        (0..n).filter(|&i| {
            let (a, b) = (&ring.points[i], &ring.points[(i + 1) % n]);
            (a.y > y) != (b.y > y) && x < (b.x - a.x) * (y - a.y) / (b.y - a.y) + a.x
        }).count() % 2 == 1
    };
    crossings(&polygon.outer) && !polygon.holes.iter().any(crossings)
}

fn centroid(points: &[V3]) -> V3 {
    mul(points.iter().fold([0.0; 3], |s, p| add(s, *p)), 1.0 / points.len().max(1) as f64)
}

fn frame_axes(f: &scene::Frame) -> (V3, V3, V3) {
    (unit(v3(&f.x)), unit(v3(&f.y)), unit(v3(&f.z)))
}

fn rect_polygon(w: f64, h: f64) -> scene::Polygon2 {
    scene::Polygon2::solid(vec![scene::Vec2::new(-w / 2.0, -h / 2.0), scene::Vec2::new(w / 2.0, -h / 2.0), scene::Vec2::new(w / 2.0, h / 2.0), scene::Vec2::new(-w / 2.0, h / 2.0)])
}

impl BasicTessellator {
    fn member(&self, entity: &SceneEntity, g: &scene::MemberGeometry, ctx: &PartContext) -> Vec<Part> {
        let (fx, fy, fz) = frame_axes(&g.frame);
        let mut axis = path_points(&g.axis);
        let regions = g.section.outline().unwrap_or_default();
        if axis.len() < 2 || regions.is_empty() { return Vec::new(); }
        let last = axis.len() - 1;
        axis[0] = add(axis[0], add(mul(fy, g.start_offset.x), mul(fz, g.start_offset.y)));
        axis[last] = add(axis[last], add(mul(fy, g.end_offset.x), mul(fz, g.end_offset.y)));
        let overlay = entity.appearance.deformation.as_ref();
        let factor = overlay_factor(entity, ctx);
        let staged = entity.utilisation.as_ref().is_some_and(|u| u.per_station.is_some()) || matches!(&entity.appearance.channel, scene::ColourChannel::Fields { fields, .. } if fields.iter().any(|f| matches!(f.samples, scene::FieldSamples::Station { .. })));
        let mut parts = Vec::new();
        let mut emit = |points: &[V3], ghost: bool, parts: &mut Vec<Part>| {
            let total = polyline_length(points).max(1e-12);
            let mut run = 0.0;
            for w in points.windows(2) {
                let seg = len(sub(w[1], w[0]));
                if seg < 1e-9 { continue; }
                for region in &regions {
                    let mut p = self.prism_part(region, span_pose(w[0], w[1], fy, (1.0, 1.0)), ctx.lod);
                    p.station = (staged && !ghost).then(|| (run / total, (run + seg) / total));
                    p.ghost = ghost;
                    parts.push(p);
                }
                if ctx.lod.detail && !ghost {
                    if let scene::SectionShape::Composite { rebar, .. } = &g.section {
                        for layout in rebar {
                            for pos in &layout.positions {
                                let off = add(mul(fy, pos.x), mul(fz, pos.y));
                                let d = layout.bar_diameter;
                                let mut bar = self.cylinder_part(span_pose(add(w[0], off), add(w[1], off), fy, (d, d)), ctx.lod);
                                bar.role = Some(MaterialRole::Rebar);
                                parts.push(bar);
                            }
                        }
                    }
                }
                run += seg;
            }
        };
        match overlay {
            Some(o) if !o.samples.is_empty() => {
                emit(&axis, true, &mut parts);
                let n = (axis.len() * 8).max(8);
                let deformed: Vec<V3> = (0..=n).map(|i| { let t = i as f64 / n as f64; add(point_at(&axis, t), mul(displacement(&o.samples, t), factor)) }).collect();
                emit(&deformed, false, &mut parts);
            }
            _ => emit(&axis, false, &mut parts),
        }
        let _ = fx;
        parts
    }

    fn plate_mode(&self, entity: &SceneEntity, g: &scene::PlateGeometry, samples: &[scene::SurfaceSample], factor: f64, ctx: &PartContext) -> Part {
        let (fx, fy, fz) = frame_axes(&g.frame);
        let origin = add(add(v3(&g.frame.origin), mul(fz, g.offset)), mul(fz, g.thickness / 2.0));
        let xs = g.outline.outer.points.iter().map(|p| p.x);
        let ys = g.outline.outer.points.iter().map(|p| p.y);
        let (x0, x1) = (xs.clone().fold(f64::MAX, f64::min), xs.fold(f64::MIN, f64::max));
        let (y0, y1) = (ys.clone().fold(f64::MAX, f64::min), ys.fold(f64::MIN, f64::max));
        let n = (24 / ctx.lod.stride).max(6);
        let node = |i: usize, j: usize| {
            let (x, y) = (x0 + (x1 - x0) * i as f64 / n as f64, y0 + (y1 - y0) * j as f64 / n as f64);
            add(add(add(origin, mul(fx, x)), mul(fy, y)), mul(surface_delta(samples, [x, y]), factor))
        };
        let nodes: Vec<V3> = (0..=n).flat_map(|j| (0..=n).map(move |i| (i, j))).map(|(i, j)| node(i, j)).collect();
        let at = |i: usize, j: usize| nodes[j.min(n) * (n + 1) + i.min(n)];
        let normal = |i: usize, j: usize| unit(cross(sub(at(i + 1, j), at(i.saturating_sub(1), j)), sub(at(i, j + 1), at(i, j.saturating_sub(1)))));
        let mut mesh = MeshData::default();
        for back in [false, true] {
            for j in 0..=n { for i in 0..=n { mesh.vertex(at(i, j), mul(normal(i, j), if back { -1.0 } else { 1.0 })); } }
        }
        let (count, stride) = (nodes.len() as u32, (n + 1) as u32);
        for j in 0..n { for i in 0..n {
            let (cx, cy) = (x0 + (x1 - x0) * (i as f64 + 0.5) / n as f64, y0 + (y1 - y0) * (j as f64 + 0.5) / n as f64);
            if !inside(&g.outline, cx, cy) { continue; }
            let a = j as u32 * stride + i as u32;
            let (b, d, e) = (a + 1, a + stride, a + stride + 1);
            mesh.tri(a, b, e);
            mesh.tri(a, e, d);
            mesh.tri(count + a, count + e, count + b);
            mesh.tri(count + a, count + d, count + e);
        } }
        self.fresh(format!("plate-mode:{}:{}:{factor:.6}", entity.entity.id, entity.content_hash), mesh)
    }

    fn plate(&self, entity: &SceneEntity, g: &scene::PlateGeometry, ctx: &PartContext) -> Vec<Part> {
        let (fx, fy, fz) = frame_axes(&g.frame);
        let origin = add(v3(&g.frame.origin), mul(fz, g.offset));
        let mut parts = vec![self.prism_part(&g.outline, basis_pose(origin, fx, fy, fz, [1.0, 1.0, g.thickness]), ctx.lod)];
        if let Some(o) = entity.appearance.deformation.as_ref().filter(|o| !o.surface.is_empty()) {
            parts[0].ghost = true;
            parts.push(self.plate_mode(entity, g, &o.surface, overlay_factor(entity, ctx), ctx));
        }
        if ctx.lod.detail {
            for s in &g.stiffeners {
                let regions = s.section.outline().unwrap_or_default();
                for w in s.path.points.windows(2) {
                    let lift = |p: &scene::Vec2| add(add(origin, add(mul(fx, p.x), mul(fy, p.y))), mul(fz, g.thickness));
                    for region in &regions { parts.push(self.prism_part(region, span_pose(lift(&w[0]), lift(&w[1]), fz, (1.0, 1.0)), ctx.lod)); }
                }
            }
        }
        parts
    }

    fn shell(&self, entity: &SceneEntity, g: &scene::ShellGeometry, ctx: &PartContext) -> Vec<Part> {
        let (rows, columns, points) = match &g.shape {
            scene::ShellShape::Revolved { origin, axis, generatrix, sweep } => {
                let (axis, o) = (unit(v3(axis)), v3(origin));
                let radial = orthogonal(axis, [1.0, 0.0, 0.0]);
                let segments = (((48.0 * sweep.abs() / std::f64::consts::TAU).ceil() as usize) / ctx.lod.stride).max(8);
                let points = generatrix.points.iter().flat_map(|p| (0..=segments).map(move |c| {
                    let a = sweep * c as f64 / segments as f64;
                    let dir = add(mul(radial, a.cos()), mul(cross(axis, radial), a.sin()));
                    add(o, add(mul(dir, p.x), mul(axis, p.y)))
                })).collect::<Vec<_>>();
                (generatrix.points.len(), segments + 1, points)
            }
            scene::ShellShape::Patch { grid } => (grid.rows as usize, grid.columns as usize, grid.points.iter().map(v3).collect()),
        };
        let overlay = entity.appearance.deformation.as_ref().filter(|o| !o.surface.is_empty());
        let Some(o) = overlay else { return vec![self.gridded(entity, "shell", rows, columns, &points)] };
        let factor = overlay_factor(entity, ctx);
        let (along, across) = surface_coordinates(&points, rows, columns);
        let moved: Vec<V3> = points.iter().enumerate().map(|(k, p)| add(*p, mul(surface_delta(&o.surface, [along[k % columns], across[k / columns]]), factor))).collect();
        let mut ghost = self.gridded(entity, "shell", rows, columns, &points);
        ghost.ghost = true;
        let mut mode = self.fresh(format!("shell-mode:{}:{}:{factor:.6}", entity.entity.id, entity.content_hash), grid(rows, columns, &moved));
        mode.grid = Some((rows, columns));
        vec![ghost, mode]
    }

    fn solid(&self, entity: &SceneEntity, g: &scene::SolidGeometry, ctx: &PartContext) -> Vec<Part> {
        match &g.shape {
            scene::SolidShape::Prism { frame, base, height } => {
                let (fx, fy, fz) = frame_axes(frame);
                vec![self.prism_part(base, basis_pose(v3(&frame.origin), fx, fy, fz, [1.0, 1.0, *height]), ctx.lod)]
            }
            scene::SolidShape::Box { frame, size } => {
                let (fx, fy, fz) = frame_axes(frame);
                vec![self.box_part(basis_pose(v3(&frame.origin), fx, fy, fz, [size.x, size.y, size.z]))]
            }
            scene::SolidShape::Cylinder { frame, radius, height } => {
                let (fx, fy, fz) = frame_axes(frame);
                vec![self.cylinder_part(basis_pose(v3(&frame.origin), fx, fy, fz, [2.0 * radius, 2.0 * radius, *height]), ctx.lod)]
            }
            scene::SolidShape::Wedge { frame, base, height } => {
                let (fx, fy, fz) = frame_axes(frame);
                let triangle = scene::Polygon2::solid(vec![scene::Vec2::new(0.0, 0.0), scene::Vec2::new(base.x, 0.0), scene::Vec2::new(0.0, base.y)]);
                vec![self.prism_part(&triangle, basis_pose(v3(&frame.origin), fx, fy, fz, [1.0, 1.0, *height]), ctx.lod)]
            }
            scene::SolidShape::Mesh { vertices, triangles } => {
                let v: Vec<V3> = vertices.iter().map(v3).collect();
                let t: Vec<[u32; 3]> = triangles.iter().map(|t| [t.a, t.b, t.c]).collect();
                vec![self.custom(entity, "mesh", soup(&v, &t))]
            }
            scene::SolidShape::StrataColumn { frame, footprint, strata } => {
                let (fx, fy, fz) = frame_axes(frame);
                let mut depth = 0.0;
                strata.iter().map(|s| {
                    depth += s.thickness;
                    let mut p = self.prism_part(footprint, basis_pose(sub(v3(&frame.origin), mul(fz, depth)), fx, fy, fz, [1.0, 1.0, s.thickness]), ctx.lod);
                    p.role = Some(s.role.clone());
                    p
                }).collect()
            }
            scene::SolidShape::FootingBody { frame, footprint, thickness, pedestal, pedestal_height } => {
                let (fx, fy, fz) = frame_axes(frame);
                let origin = v3(&frame.origin);
                let mut parts = vec![self.prism_part(footprint, basis_pose(origin, fx, fy, fz, [1.0, 1.0, *thickness]), ctx.lod)];
                if let Some(p) = pedestal.as_ref().filter(|_| *pedestal_height > 0.0) {
                    parts.push(self.box_part(basis_pose(add(origin, mul(fz, *thickness)), fx, fy, fz, [p.x, p.y, *pedestal_height])));
                }
                parts
            }
            scene::SolidShape::WallAssembly { path, height, layers, openings } => {
                let total: f64 = layers.iter().map(|l| l.thickness).sum();
                let mut parts = Vec::new();
                let mut start = 0.0;
                for w in path.points.windows(2) {
                    let (a, b) = (v3(&w[0]), v3(&w[1]));
                    let (lateral, run) = (unit(cross([0.0, 0.0, 1.0], sub(b, a))), polyline_length(&[a, b]));
                    let dir = unit(sub(b, a));
                    let cuts: Vec<(f64, f64, f64, f64)> = openings.iter().filter(|o| o.offset < start + run - 1e-9 && o.offset + o.width > start + 1e-9).map(|o| (o.offset - start, o.offset - start + o.width, o.sill, o.sill + o.height)).collect();
                    let mut across = 0.0;
                    for l in layers {
                        for (x0, x1, z0, z1) in wall_pieces(run, *height, &cuts) {
                            let shift = add(mul(lateral, across + l.thickness / 2.0 - total / 2.0), [0.0, 0.0, (z0 + z1) / 2.0]);
                            let (from, to) = (add(a, add(shift, mul(dir, x0))), add(a, add(shift, mul(dir, x1))));
                            let mut p = self.box_part(span_pose(from, to, lateral, (l.thickness, z1 - z0)));
                            p.role = Some(l.role.clone());
                            parts.push(p);
                        }
                        across += l.thickness;
                    }
                    start += run;
                }
                parts
            }
            scene::SolidShape::Laminate { frame, footprint, layers } => {
                let (fx, fy, fz) = frame_axes(frame);
                let mut rise = 0.0;
                layers.iter().map(|l| {
                    let mut p = self.prism_part(footprint, basis_pose(add(v3(&frame.origin), mul(fz, rise)), fx, fy, fz, [1.0, 1.0, l.thickness]), ctx.lod);
                    p.role = Some(l.role.clone());
                    rise += l.thickness;
                    p
                }).collect()
            }
        }
    }

    fn fastener(&self, g: &scene::FastenerGeometry, ctx: &PartContext) -> Vec<Part> {
        let (from, to) = (v3(&g.axis.from), v3(&g.axis.to));
        let dir = unit(sub(to, from));
        let (u, v) = (orthogonal(dir, [1.0, 0.0, 0.0]), cross(dir, orthogonal(dir, [1.0, 0.0, 0.0])));
        let offsets: Vec<V3> = match &g.pattern {
            None => vec![[0.0; 3]],
            Some(scene::Pattern::Grid { nx, ny, sx, sy }) => (0..*nx).flat_map(|i| (0..*ny).map(move |j| (i, j))).map(|(i, j)| add(mul(u, (i as f64 - (*nx as f64 - 1.0) / 2.0) * sx), mul(v, (j as f64 - (*ny as f64 - 1.0) / 2.0) * sy))).collect(),
            Some(scene::Pattern::Row { n, s }) => (0..*n).map(|i| mul(u, (i as f64 - (*n as f64 - 1.0) / 2.0) * s)).collect(),
            Some(scene::Pattern::Along { path, spacing }) => {
                let pts = path_points(path);
                let total = polyline_length(&pts);
                (0..=((total / spacing).floor() as usize)).map(|i| sub(point_at(&pts, i as f64 * spacing / total.max(1e-12)), pts[0])).collect()
            }
            Some(scene::Pattern::Cage { zones, .. }) => zones.iter().flat_map(|z| (0..z.count).map(move |i| z.start + i as f64 * z.spacing)).map(|x| mul(dir, x)).collect(),
            Some(scene::Pattern::Points { offsets }) => offsets.iter().map(|o| add(mul(dir, o.x), add(mul(u, o.y), mul(v, o.z)))).collect(),
        };
        let cage = matches!(g.pattern, Some(scene::Pattern::Cage { .. }));
        let d = g.diameter;
        let mut parts = Vec::new();
        for off in offsets {
            if let Some(scene::Pattern::Cage { outline, reference, .. }) = &g.pattern {
                let (cu, cv) = (orthogonal(dir, v3(reference)), cross(dir, orthogonal(dir, v3(reference))));
                let ring: Vec<V3> = outline.points.iter().map(|q| add(add(from, off), add(mul(cu, q.x), mul(cv, q.y)))).collect();
                for i in 0..ring.len() {
                    parts.push(self.cylinder_part(span_pose(ring[i], ring[(i + 1) % ring.len()], dir, (d, d)), ctx.lod));
                }
                continue;
            }
            let (a, b) = (add(from, off), if cage { add(add(from, off), mul(dir, d)) } else { add(to, off) });
            match &g.kind {
                scene::FastenerKind::WeldRun { throat, path, .. } => {
                    for w in path_points(path).windows(2) { parts.push(self.box_part(span_pose(add(w[0], off), add(w[1], off), u, (throat * 1.4, throat * 1.4)))); }
                }
                scene::FastenerKind::Weld { throat, .. } => parts.push(self.box_part(span_pose(a, b, u, (throat * 1.4, throat * 1.4)))),
                scene::FastenerKind::Tendon { profile, duct } => {
                    let dd = duct.unwrap_or(d);
                    for w in path_points(profile).windows(2) { parts.push(self.cylinder_part(span_pose(add(w[0], off), add(w[1], off), u, (dd, dd)), ctx.lod)); }
                }
                kind => {
                    parts.push(self.cylinder_part(span_pose(a, b, u, (d, d)), ctx.lod));
                    if ctx.lod.detail && !cage {
                        let (head_h, head_d) = match kind {
                            scene::FastenerKind::Stud { head_height, head_diameter } => (*head_height, *head_diameter),
                            _ => (0.6 * d, 1.7 * d),
                        };
                        let head_a = sub(a, mul(dir, head_h));
                        match g.head {
                            scene::HeadShape::None => {}
                            scene::HeadShape::Hex => {
                                let hex = scene::Polygon2::solid((0..6).map(|i| { let t = std::f64::consts::TAU * i as f64 / 6.0; scene::Vec2::new(0.5 * t.cos(), 0.5 * t.sin()) }).collect());
                                parts.push(self.prism_part(&hex, span_pose(head_a, a, u, (head_d, head_d)), ctx.lod));
                            }
                            _ => parts.push(self.cylinder_part(span_pose(head_a, a, u, (head_d, head_d)), ctx.lod)),
                        }
                    }
                    if let scene::FastenerKind::Rebar { bend: Some(bend) } = kind {
                        let leg = add(mul(dir, bend.bend_angle.cos()), mul(u, bend.bend_angle.sin()));
                        parts.push(self.cylinder_part(span_pose(b, add(b, mul(leg, bend.leg_length)), v, (d, d)), ctx.lod));
                        if bend.both_ends {
                            let start_leg = add(mul(dir, -bend.bend_angle.cos()), mul(u, bend.bend_angle.sin()));
                            parts.push(self.cylinder_part(span_pose(a, add(a, mul(start_leg, bend.leg_length)), v, (d, d)), ctx.lod));
                        }
                    }
                }
            }
        }
        parts
    }

    fn support(&self, entity: &SceneEntity, g: &scene::SupportGeometry, ctx: &PartContext) -> Vec<Part> {
        let (fx, fy, fz) = frame_axes(&g.frame);
        let (at, s) = (v3(&g.at), ctx.glyph * 0.6);
        let slab = |size: f64, thick: f64| self.box_part(basis_pose(sub(at, mul(fz, thick)), fx, fy, fz, [size, size, thick]));
        let cone = || self.cone_part(basis_pose(sub(at, mul(fz, s)), fx, fy, fz, [s, s, s]), ctx.lod);
        match &g.kind {
            scene::SupportKind::Fixed => vec![slab(1.4 * s, 0.25 * s)],
            scene::SupportKind::Pinned => vec![cone()],
            scene::SupportKind::Roller { .. } => vec![cone(), self.cylinder_part(basis_pose(sub(at, mul(fz, 1.3 * s)), fx, fy, fz, [0.5 * s, 0.5 * s, 0.3 * s]), ctx.lod)],
            scene::SupportKind::Spring { .. } => vec![self.cylinder_part(basis_pose(sub(at, mul(fz, 1.5 * s)), fx, fy, fz, [0.25 * s, 0.25 * s, 1.5 * s]), ctx.lod)],
            scene::SupportKind::ElasticBed { extent, .. } => vec![self.custom(entity, "bed", flat_polygon(&extent.outer.points.iter().map(v3).collect::<Vec<_>>()))],
            scene::SupportKind::PileHead => vec![self.cylinder_part(basis_pose(sub(at, mul(fz, 2.0 * s)), fx, fy, fz, [0.6 * s, 0.6 * s, 2.0 * s]), ctx.lod)],
        }
    }

    fn load(&self, entity: &SceneEntity, g: &LoadGlyph, ctx: &PartContext) -> Vec<Part> {
        let l = ctx.glyph;
        let anchor_of = |id: &str| ctx.scene.entities.iter().find(|e| e.entity.id == id).map(|e| centroid(&entity_points(&e.geometry))).unwrap_or([0.0; 3]);
        match g {
            LoadGlyph::Point { at, direction, magnitude, .. } | LoadGlyph::Impact { at, direction, energy_or_force: magnitude, .. } => self.arrow(v3(at), v3(direction), glyph_length(l, magnitude.value), ctx, false),
            LoadGlyph::Moment { at, axis, magnitude, .. } => self.arrow(v3(at), v3(axis), glyph_length(l, magnitude.value), ctx, true),
            LoadGlyph::Line { path, direction, profile, .. } => {
                let pts = path_points(path);
                let top = profile_peak(profile).max(1e-12);
                let n = if ctx.lod.detail { 6 } else { 3 };
                (0..n).flat_map(|i| {
                    let t = i as f64 / (n - 1) as f64;
                    self.arrow(point_at(&pts, t), v3(direction), glyph_length(l, 0.0) * 0.3 + l * 0.7 * profile_at(profile, t).abs() / top, ctx, false)
                }).collect()
            }
            LoadGlyph::Area { polygon, direction, intensity, .. } => {
                let ring: Vec<V3> = polygon.outer.points.iter().map(v3).collect();
                let c = centroid(&ring);
                std::iter::once(c).chain(ring.iter().map(|p| lerp(*p, c, 0.2)).take(if ctx.lod.detail { 8 } else { 3 })).flat_map(|p| self.arrow(p, v3(direction), glyph_length(l, intensity.value), ctx, false)).collect()
            }
            LoadGlyph::Pressure { zones, .. } => zones.iter().flat_map(|z| {
                let ring: Vec<V3> = z.polygon.outer.points.iter().map(v3).collect();
                let normal = unit(ring.iter().enumerate().fold([0.0; 3], |n, (i, p)| add(n, cross(*p, ring[(i + 1) % ring.len()]))));
                self.arrow(centroid(&ring), mul(normal, -z.value.value.signum()), glyph_length(l, z.value.value), ctx, false)
            }).collect(),
            LoadGlyph::Thermal { target, .. } => {
                let c = anchor_of(&target.id);
                vec![self.cylinder_part(basis_pose(c, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.3 * l, 0.3 * l, 0.1 * l]), ctx.lod)]
            }
            LoadGlyph::Inertial { direction, masses, .. } => masses.iter().flat_map(|m| self.arrow(v3(&m.at), v3(direction), glyph_length(l, m.mass.value), ctx, false)).collect(),
            LoadGlyph::Explosion { centre, pressure, .. } => {
                let d = glyph_length(l, pressure.value) * 2.0;
                vec![self.part("sphere", basis_pose(v3(centre), [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [d, d, d]), || unit_sphere(16))]
            }
        }
        .into_iter().map(|mut p| { p.role = Some(entity.appearance.role.clone()); p }).collect()
    }

    fn surface(&self, entity: &SceneEntity, g: &scene::SurfaceGeometry) -> Vec<Part> {
        match &g.shape {
            scene::SurfaceShape::Polygon { polygon } => vec![self.custom(entity, "surface", flat_polygon(&polygon.outer.points.iter().map(v3).collect::<Vec<_>>()))],
            scene::SurfaceShape::Arc { arc, width } => {
                let pts = arc_points(v3(&arc.centre), v3(&arc.normal), v3(&arc.start), arc.sweep, 24);
                let up = mul(unit(v3(&arc.normal)), *width);
                let grid_points: Vec<V3> = pts.iter().copied().chain(pts.iter().map(|p| add(*p, up))).collect();
                vec![self.gridded(entity, "slip", 2, pts.len(), &grid_points)]
            }
            scene::SurfaceShape::Grid { grid: g } => vec![self.gridded(entity, "terrain", g.rows as usize, g.columns as usize, &g.points.iter().map(v3).collect::<Vec<_>>())],
        }
    }
}

pub fn profile_at(profile: &scene::LoadProfile, t: f64) -> f64 {
    match profile {
        scene::LoadProfile::Uniform { value } => value.value,
        scene::LoadProfile::Linear { start, end } => start.value + (end.value - start.value) * t,
        scene::LoadProfile::Stepped { steps } => steps.iter().rev().find(|s| s.at <= t).or(steps.first()).map_or(0.0, |s| s.value.value),
        scene::LoadProfile::Sampled { samples } => samples.windows(2).find(|w| t <= w[1].at).map(|w| w[0].value.value + (w[1].value.value - w[0].value.value) * (t - w[0].at) / (w[1].at - w[0].at).max(1e-12)).or_else(|| samples.last().map(|s| s.value.value)).unwrap_or(0.0),
    }
}

pub fn profile_peak(profile: &scene::LoadProfile) -> f64 {
    (0..=10).map(|i| profile_at(profile, i as f64 / 10.0).abs()).fold(0.0, f64::max)
}

/// 📍️ Characteristic points of a geometry: bounds, anchors and camera framing derive from them.
pub fn entity_points(geometry: &Geometry) -> Vec<V3> {
    match geometry {
        Geometry::Member(m) => path_points(&m.axis),
        Geometry::Plate(p) => p.outline.outer.points.iter().map(|q| add(add(v3(&p.frame.origin), mul(v3(&p.frame.x), q.x)), mul(v3(&p.frame.y), q.y))).collect(),
        Geometry::Shell(s) => match &s.shape {
            scene::ShellShape::Revolved { origin, axis, generatrix, .. } => {
                let (o, a) = (v3(origin), unit(v3(axis)));
                let r = generatrix.points.iter().map(|p| p.x.abs()).fold(0.0, f64::max);
                let (lo, hi) = generatrix.points.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.y), m.1.max(p.y)));
                let radial = orthogonal(a, [1.0, 0.0, 0.0]);
                let side = cross(a, radial);
                [(r, 0.0), (-r, 0.0), (0.0, r), (0.0, -r)].iter().flat_map(|(x, y)| [lo, hi].map(|s| add(o, add(add(mul(radial, *x), mul(side, *y)), mul(a, s))))).collect()
            }
            scene::ShellShape::Patch { grid } => grid.points.iter().map(v3).collect(),
        },
        Geometry::Solid(s) => match &s.shape {
            scene::SolidShape::Prism { frame, base, height } => base.outer.points.iter().flat_map(|q| [0.0, *height].map(|z| add(add(v3(&frame.origin), add(mul(v3(&frame.x), q.x), mul(v3(&frame.y), q.y))), mul(v3(&frame.z), z)))).collect(),
            scene::SolidShape::Box { frame, size } => [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)].iter().flat_map(|(x, y)| [0.0, size.z].map(|z| add(add(v3(&frame.origin), add(mul(v3(&frame.x), x * size.x), mul(v3(&frame.y), y * size.y))), mul(v3(&frame.z), z)))).collect(),
            scene::SolidShape::Cylinder { frame, radius, height } => [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)].iter().flat_map(|(x, y)| [0.0, *height].map(|z| add(add(v3(&frame.origin), add(mul(v3(&frame.x), x * radius), mul(v3(&frame.y), y * radius))), mul(v3(&frame.z), z)))).collect(),
            scene::SolidShape::Wedge { frame, base, height } => [(0.0, 0.0), (base.x, 0.0), (0.0, base.y)].iter().flat_map(|(x, y)| [0.0, *height].map(|z| add(add(v3(&frame.origin), add(mul(v3(&frame.x), *x), mul(v3(&frame.y), *y))), mul(v3(&frame.z), z)))).collect(),
            scene::SolidShape::Mesh { vertices, .. } => vertices.iter().map(v3).collect(),
            scene::SolidShape::StrataColumn { frame, footprint, strata } => {
                let depth: f64 = strata.iter().map(|s| s.thickness).sum();
                footprint.outer.points.iter().flat_map(|q| [0.0, -depth].map(|z| add(add(v3(&frame.origin), add(mul(v3(&frame.x), q.x), mul(v3(&frame.y), q.y))), mul(v3(&frame.z), z)))).collect()
            }
            scene::SolidShape::FootingBody { frame, footprint, thickness, pedestal_height, .. } => footprint.outer.points.iter().flat_map(|q| [0.0, thickness + pedestal_height].map(|z| add(add(v3(&frame.origin), add(mul(v3(&frame.x), q.x), mul(v3(&frame.y), q.y))), mul(v3(&frame.z), z)))).collect(),
            scene::SolidShape::WallAssembly { path, height, .. } => path.points.iter().flat_map(|p| [0.0, *height].map(|z| add(v3(p), [0.0, 0.0, z]))).collect(),
            scene::SolidShape::Laminate { frame, footprint, layers } => {
                let rise: f64 = layers.iter().map(|l| l.thickness).sum();
                footprint.outer.points.iter().flat_map(|q| [0.0, rise].map(|z| add(add(v3(&frame.origin), add(mul(v3(&frame.x), q.x), mul(v3(&frame.y), q.y))), mul(v3(&frame.z), z)))).collect()
            }
        },
        Geometry::Fastener(f) => vec![v3(&f.axis.from), v3(&f.axis.to)],
        Geometry::Support(s) => vec![v3(&s.at)],
        Geometry::Load(l) => match l {
            LoadGlyph::Point { at, .. } | LoadGlyph::Moment { at, .. } | LoadGlyph::Impact { at, .. } => vec![v3(at)],
            LoadGlyph::Line { path, .. } => path_points(path),
            LoadGlyph::Area { polygon, .. } => polygon.outer.points.iter().map(v3).collect(),
            LoadGlyph::Pressure { zones, .. } => zones.iter().flat_map(|z| z.polygon.outer.points.iter().map(v3)).collect(),
            LoadGlyph::Inertial { origin, masses, .. } => std::iter::once(v3(origin)).chain(masses.iter().map(|m| v3(&m.at))).collect(),
            LoadGlyph::Explosion { centre, .. } => vec![v3(centre)],
            LoadGlyph::Thermal { .. } => Vec::new(),
        },
        Geometry::Surface(s) => match &s.shape {
            scene::SurfaceShape::Polygon { polygon } => polygon.outer.points.iter().map(v3).collect(),
            scene::SurfaceShape::Arc { arc, .. } => arc_points(v3(&arc.centre), v3(&arc.normal), v3(&arc.start), arc.sweep, 8),
            scene::SurfaceShape::Grid { grid } => grid.points.iter().map(v3).collect(),
        },
        Geometry::Group { .. } => Vec::new(),
    }
}

pub fn entity_anchor(geometry: &Geometry) -> V3 {
    let pts = entity_points(geometry);
    match geometry {
        Geometry::Member(_) => point_at(&pts, 0.5),
        _ => centroid(&pts),
    }
}

impl SceneTessellator for BasicTessellator {
    fn parts(&self, entity: &SceneEntity, ctx: &PartContext) -> Vec<Part> {
        let mut parts = match &entity.geometry {
            Geometry::Member(g) => self.member(entity, g, ctx),
            Geometry::Plate(g) => self.plate(entity, g, ctx),
            Geometry::Shell(g) => self.shell(entity, g, ctx),
            Geometry::Solid(g) => self.solid(entity, g, ctx),
            Geometry::Fastener(g) => self.fastener(g, ctx),
            Geometry::Support(g) => self.support(entity, g, ctx),
            Geometry::Load(g) => self.load(entity, g, ctx),
            Geometry::Surface(g) => self.surface(entity, g),
            Geometry::Group { .. } => Vec::new(),
        };
        if ctx.lod.detail { parts.extend(self.hatching(entity, ctx)); }
        parts
    }
}
