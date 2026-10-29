//! 🪣️ Geometry buckets and curve sampling of the CAD pick engine: the anchor rows, the per-kind buckets and the arc, circle, ellipse and NURBS sample points. See [`super`].

use super::*;

//#region 🔖️Buckets
/// ⚓️ One anchor row, read out of `CadGeometry::anchors` — that field is untyped `DslValue` on the
/// import struct (`🚪️io/🗺️geometry-import/🦀️.rs`), so the shape is read here rather than by
/// widening a `pub(crate)` struct other lanes are editing.
#[derive(Clone, Debug, PartialEq)]
pub struct AnchorRow {
    pub id: String,
    pub position: [f64; 3],
    pub attachment: Option<(String, String)>,
}

fn read_vec3(value: Option<&DslValue>) -> Option<[f64; 3]> {
    let items = value?.as_array()?;
    if items.len() < 3 {
        return None;
    }
    Some([items[0].as_f64()?, items[1].as_f64()?, items[2].as_f64()?])
}

fn read_anchor_rows(anchors: &[DslValue]) -> Vec<AnchorRow> {
    anchors
        .iter()
        .filter_map(|row| {
            let id = row.get("id")?.as_str()?.to_string();
            let position = read_vec3(row.get("position"))?;
            let attachment = row.get("attachment").and_then(|attachment| Some((attachment.get("kind")?.as_str()?.to_string(), attachment.get("id")?.as_str()?.to_string())));
            Some(AnchorRow { id, position, attachment })
        })
        .collect()
}

/// 🧲️ Record-shaped views over a pane's ephemeral `CadGeometry` — React's `geometryBuckets`.
pub struct GeometryBuckets<'a> {
    pub anchors: Vec<AnchorRow>,
    pub vertices: HashMap<&'a str, &'a CadVertex>,
    pub edges: HashMap<&'a str, &'a CadEdge>,
    pub wires: HashMap<&'a str, &'a CadWire>,
    pub faces: HashMap<&'a str, &'a CadFace>,
    pub shells: HashMap<&'a str, &'a CadShell>,
    pub solids: HashMap<&'a str, &'a CadSolid>,
    pub(super) order: GeometryOrder<'a>,
}

pub(crate) struct GeometryOrder<'a> {
    pub(super) vertices: Vec<&'a CadVertex>,
    pub(super) edges: Vec<&'a CadEdge>,
    pub(super) wires: Vec<&'a CadWire>,
    pub(super) faces: Vec<&'a CadFace>,
    pub(super) shells: Vec<&'a CadShell>,
    pub(super) solids: Vec<&'a CadSolid>,
}

/// 🧲️ Builds the buckets, preserving authored asset order so pick-target lists stay deterministic.
pub fn geometry_buckets(geometry: &CadGeometry) -> GeometryBuckets<'_> {
    GeometryBuckets {
        anchors: read_anchor_rows(&geometry.anchors),
        vertices: geometry.vertices.iter().map(|row| (row.id.as_str(), row)).collect(),
        edges: geometry.edges.iter().map(|row| (row.id.as_str(), row)).collect(),
        wires: geometry.wires.iter().map(|row| (row.id.as_str(), row)).collect(),
        faces: geometry.faces.iter().map(|row| (row.id.as_str(), row)).collect(),
        shells: geometry.shells.iter().map(|row| (row.id.as_str(), row)).collect(),
        solids: geometry.solids.iter().map(|row| (row.id.as_str(), row)).collect(),
        order: GeometryOrder {
            vertices: geometry.vertices.iter().collect(),
            edges: geometry.edges.iter().collect(),
            wires: geometry.wires.iter().collect(),
            faces: geometry.faces.iter().collect(),
            shells: geometry.shells.iter().collect(),
            solids: geometry.solids.iter().collect(),
        },
    }
}

pub(super) fn point_centroid(points: &[[f64; 3]]) -> Option<[f64; 3]> {
    if points.is_empty() {
        return None;
    }
    let sum = points.iter().fold([0.0; 3], |acc, p| [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]]);
    let n = points.len() as f64;
    Some([sum[0] / n, sum[1] / n, sum[2] / n])
}

/// 🔑️ Dedup key for a point. React keys on `p.join(",")`, so equal values collapse; the bit pattern
/// does the same in O(1) once `-0.0` is folded onto `0.0` (JS prints both as `"0"`), and keeps a
/// solid's point set linear instead of the quadratic string scan a `Vec::contains` would cost on a
/// pane with thousands of kernel members.
fn point_key(point: [f64; 3]) -> [u64; 3] {
    [(point[0] + 0.0).to_bits(), (point[1] + 0.0).to_bits(), (point[2] + 0.0).to_bits()]
}

fn unique_points(points: Vec<[f64; 3]>) -> Vec<[f64; 3]> {
    let mut seen: std::collections::HashSet<[u64; 3]> = std::collections::HashSet::new();
    points.into_iter().filter(|point| seen.insert(point_key(*point))).collect()
}

//#region 🔖️CurveSampling
/// 📐️ `edgeSamplePoints`' default segment count (`🧮️preview/🌀️curve-sampling/🟦️.ts:169`).
pub const EDGE_SAMPLE_SEGMENTS: usize = 32;

/// ⭕️ The floor `circleSamplePoints`/`ellipseSamplePoints` raise `segments` to (`:272`, `:274`).
pub const CLOSED_CURVE_SAMPLE_SEGMENTS: usize = 64;

fn v_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn v_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn v_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn v_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn v_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn v_length(a: [f64; 3]) -> f64 {
    v_dot(a, a).sqrt()
}

fn v_normalize(a: [f64; 3]) -> [f64; 3] {
    let length = v_length(a);
    if length < 1e-12 { a } else { v_scale(a, 1.0 / length) }
}

/// 🔵️ The plane frame of an arc — React's `arcPlaneFrame` (`🧮️preview/🌀️curve-sampling/🟦️.ts:24`).
struct ArcPlaneFrame {
    pub(super) center: [f64; 3],
    pub(super) radius: f64,
    u: [f64; 3],
    pub(super) v: [f64; 3],
}

fn arc_plane_frame(center: [f64; 3], start: [f64; 3], end: [f64; 3]) -> Option<ArcPlaneFrame> {
    let rs = v_sub(start, center);
    let re = v_sub(end, center);
    let radius = v_length(rs);
    if radius < 1e-9 {
        return None;
    }
    let mut normal = v_cross(rs, re);
    normal = if v_length(normal) < 1e-9 { [0.0, 0.0, 1.0] } else { v_normalize(normal) };
    let u = v_normalize(rs);
    let v = v_cross(normal, u);
    Some(ArcPlaneFrame { center, radius, u, v })
}

/// 🔵️ React's `arcFrameFromRadiusPoint` (`:154`) — Z-up axis unless the radius is already Z-ish.
fn arc_frame_from_radius_point(center: [f64; 3], on_circle: [f64; 3]) -> Option<ArcPlaneFrame> {
    let rs = v_sub(on_circle, center);
    let radius = v_length(rs);
    if radius < 1e-9 {
        return None;
    }
    let u = v_normalize(rs);
    let axis = if v_dot(u, [0.0, 0.0, 1.0]).abs() > 0.99 { [0.0, 1.0, 0.0] } else { [0.0, 0.0, 1.0] };
    let v = v_normalize(v_cross(axis, u));
    Some(ArcPlaneFrame { center, radius, u, v })
}

/// 🔵️ React's `arcSweepRadians` (`:131`) — positive CCW, and a zero sweep means a full turn.
fn arc_sweep_radians(frame: &ArcPlaneFrame, end: [f64; 3]) -> f64 {
    let re = v_sub(end, frame.center);
    let mut sweep = v_dot(re, frame.v).atan2(v_dot(re, frame.u));
    if sweep < 0.0 {
        sweep += std::f64::consts::TAU;
    }
    if sweep < 1e-9 {
        sweep = std::f64::consts::TAU;
    }
    sweep
}

fn frame_point(frame: &ArcPlaneFrame, angle: f64) -> [f64; 3] {
    v_add(frame.center, v_add(v_scale(frame.u, frame.radius * angle.cos()), v_scale(frame.v, frame.radius * angle.sin())))
}

/// 🔵️ React's `arcSamplePoints` (`:140`).
pub fn arc_sample_points(center: [f64; 3], start: [f64; 3], end: [f64; 3], segments: usize) -> Vec<[f64; 3]> {
    let Some(frame) = arc_plane_frame(center, start, end) else { return vec![start, end] };
    let sweep = arc_sweep_radians(&frame, end);
    let n = segments.max(2);
    (0..=n).map(|index| frame_point(&frame, index as f64 / n as f64 * sweep)).collect()
}

/// ⭕️ React's `circleSamplePoints` (`:183`).
///
/// ⚖️ The frame is built from `center + normalize(normal) * radius`, so the circle's own `u` IS the
/// normal direction — a quirk of React's implementation, ported verbatim rather than "corrected",
/// because a divergence here would move every circle both hosts draw.
pub fn circle_sample_points(center: [f64; 3], normal: [f64; 3], radius: f64, segments: usize) -> Vec<[f64; 3]> {
    let Some(frame) = arc_frame_from_radius_point(center, v_add(center, v_scale(v_normalize(normal), radius))) else { return vec![center] };
    let n = segments.max(8);
    (0..=n).map(|index| frame_point(&frame, index as f64 / n as f64 * std::f64::consts::TAU)).collect()
}

/// 🥚️ React's `ellipseSamplePoints` (`:196`).
pub fn ellipse_sample_points(center: [f64; 3], normal: [f64; 3], major_axis: [f64; 3], major_radius: f64, minor_radius: f64, segments: usize) -> Vec<[f64; 3]> {
    let u = v_normalize(major_axis);
    let v = v_normalize(v_cross(normal, u));
    let n = segments.max(8);
    (0..=n)
        .map(|index| {
            let angle = index as f64 / n as f64 * std::f64::consts::TAU;
            v_add(center, v_add(v_scale(u, major_radius * angle.cos()), v_scale(v, minor_radius * angle.sin())))
        })
        .collect()
}

/// 📈️ React's `nurbsDisplaySamplePoints` (`:209`) — centripetal Catmull-Rom through the poles.
pub fn nurbs_display_sample_points(poles: &[[f64; 3]], segments_per_span: usize) -> Vec<[f64; 3]> {
    if poles.len() < 3 {
        return poles.to_vec();
    }
    let n = poles.len();
    let segs = segments_per_span.max(1);
    let mut out: Vec<[f64; 3]> = Vec::new();
    for index in 0..n - 1 {
        let p0 = poles[index.saturating_sub(1)];
        let p1 = poles[index];
        let p2 = poles[index + 1];
        let p3 = poles[(index + 2).min(n - 1)];
        for step in 0..segs {
            let t = step as f64 / segs as f64;
            let t2 = t * t;
            let t3 = t2 * t;
            out.push([0, 1, 2].map(|axis| 0.5 * (2.0 * p1[axis] + (-p0[axis] + p2[axis]) * t + (2.0 * p0[axis] - 5.0 * p1[axis] + 4.0 * p2[axis] - p3[axis]) * t2 + (-p0[axis] + 3.0 * p1[axis] - 3.0 * p2[axis] + p3[axis]) * t3)));
        }
    }
    out.push(poles[n - 1]);
    out
}

/// 📍️ React's `edgeSamplePoints` (`🧮️preview/🌀️curve-sampling/🟦️.ts:169`) — the exact family ladder, over the curve
/// parameters `CadEdgeCurve` now carries. A producer-supplied `points` polyline wins outright.
pub fn edge_sample_points(curve: &CadEdgeCurve, ends: &[[f64; 3]], segments: usize) -> Vec<[f64; 3]> {
    if !curve.points.is_empty() {
        return curve.points.clone();
    }
    if ends.len() < 1 {
        return ends.to_vec();
    }
    match curve.kind.as_str() {
        "arc" if ends.len() >= 2 => match curve.center {
            Some(center) => arc_sample_points(center, ends[0], ends[1], segments),
            None => ends.to_vec(),
        },
        "circle" => match (curve.center, curve.normal, curve.radius) {
            (Some(center), Some(normal), Some(radius)) => circle_sample_points(center, normal, radius, segments.max(CLOSED_CURVE_SAMPLE_SEGMENTS)),
            _ => ends.to_vec(),
        },
        "ellipse" => match (curve.center, curve.normal, curve.major_axis, curve.major_radius, curve.minor_radius) {
            (Some(center), Some(normal), Some(major_axis), Some(major_radius), Some(minor_radius)) => ellipse_sample_points(center, normal, major_axis, major_radius, minor_radius, segments.max(CLOSED_CURVE_SAMPLE_SEGMENTS)),
            _ => ends.to_vec(),
        },
        "nurbs" if !curve.poles.is_empty() => {
            let span = if curve.through { 12.max(curve.poles.len() * 8) } else { 4.max(segments.div_ceil(4)) };
            nurbs_display_sample_points(&curve.poles, span)
        }
        _ => ends.to_vec(),
    }
}
//#endregion 🔖️CurveSampling

impl<'a> GeometryBuckets<'a> {
    /// 📍️ React's `edgeSamplePoints` for one edge — the boundary vertices for a straight edge, and
    /// the tessellated polyline for every curved family [`edge_sample_points`] knows.
    ///
    /// 🩸️ This used to return the boundary vertices UNCONDITIONALLY, because `CadEdgeCurve` carried
    /// only `kind`: a circle picked and drew as one chord where React draws 64 samples. The struct
    /// now carries the curve parameters (ticket 26/09/17 packet W14g).
    pub fn edge_points(&self, edge: &CadEdge) -> Vec<[f64; 3]> {
        let ends: Vec<[f64; 3]> = edge.vertex_ids.iter().filter_map(|id| self.vertices.get(id.as_str()).map(|vertex| vertex.position)).collect();
        edge_sample_points(&edge.curve, &ends, EDGE_SAMPLE_SEGMENTS)
    }

    pub fn wire_points(&self, wire: &CadWire) -> Vec<[f64; 3]> {
        unique_points(wire.edge_ids.iter().filter_map(|id| self.edges.get(id.as_str())).flat_map(|edge| self.edge_points(edge)).collect())
    }

    pub fn face_points(&self, face: &CadFace) -> Vec<[f64; 3]> {
        let edge_ids: Vec<&str> = face.wire_ids.iter().filter_map(|id| self.wires.get(id.as_str())).flat_map(|wire| wire.edge_ids.iter().map(String::as_str)).collect();
        unique_points(edge_ids.iter().filter_map(|id| self.edges.get(*id)).flat_map(|edge| self.edge_points(edge)).collect())
    }

    pub fn shell_points(&self, shell: &CadShell) -> Vec<[f64; 3]> {
        unique_points(shell.face_ids.iter().filter_map(|id| self.faces.get(id.as_str())).flat_map(|face| self.face_points(face)).collect())
    }

    pub fn solid_points(&self, solid: &CadSolid) -> Vec<[f64; 3]> {
        unique_points(solid.shell_ids.iter().filter_map(|id| self.shells.get(id.as_str())).flat_map(|shell| self.shell_points(shell)).collect())
    }

    pub fn all_vertex_points(&self) -> Vec<[f64; 3]> {
        self.order.vertices.iter().map(|vertex| vertex.position).collect()
    }

    /// 📐️ Points of one kernel entity by kind + id — React's `geometryEntityPoints`.
    pub fn entity_points(&self, kind: ModelEntityKind, id: &str) -> Vec<[f64; 3]> {
        match kind {
            ModelEntityKind::Anchor => self.anchors.iter().find(|anchor| anchor.id == id).map(|anchor| vec![anchor.position]).unwrap_or_default(),
            ModelEntityKind::Vertex => self.vertices.get(id).map(|vertex| vec![vertex.position]).unwrap_or_default(),
            ModelEntityKind::Edge => self.edges.get(id).map(|edge| self.edge_points(edge)).unwrap_or_default(),
            ModelEntityKind::Wire => self.wires.get(id).map(|wire| self.wire_points(wire)).unwrap_or_default(),
            ModelEntityKind::Face => self.faces.get(id).map(|face| self.face_points(face)).unwrap_or_default(),
            ModelEntityKind::Shell => self.shells.get(id).map(|shell| self.shell_points(shell)).unwrap_or_default(),
            ModelEntityKind::Solid => self.solids.get(id).map(|solid| self.solid_points(solid)).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    /// 📐️ Consecutive segment pairs along every straight edge of one entity.
    pub fn entity_wire_segments(&self, kind: ModelEntityKind, id: &str) -> Vec<([f64; 3], [f64; 3])> {
        let polyline = |points: Vec<[f64; 3]>| -> Vec<([f64; 3], [f64; 3])> { points.windows(2).map(|pair| (pair[0], pair[1])).collect() };
        match kind {
            ModelEntityKind::Edge => self.edges.get(id).map(|edge| polyline(self.edge_points(edge))).unwrap_or_default(),
            ModelEntityKind::Wire => self.wires.get(id).map(|wire| wire.edge_ids.iter().filter_map(|id| self.edges.get(id.as_str())).flat_map(|edge| polyline(self.edge_points(edge))).collect()).unwrap_or_default(),
            ModelEntityKind::Face => self
                .faces
                .get(id)
                .map(|face| face.wire_ids.iter().filter_map(|id| self.wires.get(id.as_str())).flat_map(|wire| wire.edge_ids.iter().filter_map(|id| self.edges.get(id.as_str())).flat_map(|edge| polyline(self.edge_points(edge))).collect::<Vec<_>>()).collect())
                .unwrap_or_default(),
            ModelEntityKind::Shell => self.shells.get(id).map(|shell| shell.face_ids.iter().flat_map(|face_id| self.entity_wire_segments(ModelEntityKind::Face, face_id)).collect()).unwrap_or_default(),
            ModelEntityKind::Solid => self.solids.get(id).map(|solid| solid.shell_ids.iter().flat_map(|shell_id| self.entity_wire_segments(ModelEntityKind::Shell, shell_id)).collect()).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    /// 📐️ Every straight-edge segment in the pane, for the factory wireframe layer.
    pub fn all_edge_segments(&self) -> Vec<([f64; 3], [f64; 3])> {
        self.order.edges.iter().flat_map(|edge| self.edge_points(edge).windows(2).map(|pair| (pair[0], pair[1])).collect::<Vec<_>>()).collect()
    }

    /// 📐️ Straight-edge segments limited to revealed kernel members (`edge:<id>` keys).
    pub fn edge_segments_for_members(&self, revealed_member_keys: &BTreeSet<String>) -> Vec<([f64; 3], [f64; 3])> {
        self.order
            .edges
            .iter()
            .filter(|edge| revealed_member_keys.contains(&format!("edge:{}", edge.id)))
            .flat_map(|edge| self.edge_points(edge).windows(2).map(|pair| (pair[0], pair[1])).collect::<Vec<_>>())
            .collect()
    }

    /// 🧲️ Every `kind:id` member key reachable from a solid, the solid itself included.
    pub fn solid_primitive_member_ids(&self, solid_id: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let Some(solid) = self.solids.get(solid_id) else { return out };
        out.insert(format!("solid:{solid_id}"));
        let mut shell_queue: Vec<&str> = solid.shell_ids.iter().map(String::as_str).collect();
        while let Some(shell_id) = shell_queue.pop() {
            if !out.insert(format!("shell:{shell_id}")) {
                continue;
            }
            let Some(shell) = self.shells.get(shell_id) else { continue };
            for face_id in &shell.face_ids {
                if !out.insert(format!("face:{face_id}")) {
                    continue;
                }
                let Some(face) = self.faces.get(face_id.as_str()) else { continue };
                for wire_id in &face.wire_ids {
                    if !out.insert(format!("wire:{wire_id}")) {
                        continue;
                    }
                    let Some(wire) = self.wires.get(wire_id.as_str()) else { continue };
                    for edge_id in &wire.edge_ids {
                        if !out.insert(format!("edge:{edge_id}")) {
                            continue;
                        }
                        let Some(edge) = self.edges.get(edge_id.as_str()) else { continue };
                        for vertex_id in &edge.vertex_ids {
                            out.insert(format!("vertex:{vertex_id}"));
                        }
                    }
                }
            }
        }
        out
    }
}
//#endregion 🔖️Buckets
