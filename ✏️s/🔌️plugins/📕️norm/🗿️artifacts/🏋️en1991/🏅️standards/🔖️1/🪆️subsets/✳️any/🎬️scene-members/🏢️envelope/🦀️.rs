//! 🏢️ Envelope: the building as a translucent prism with its roof body (flat, mono-pitch, duo-pitch or hipped), the roof geometry and wind-zone partition every other module reads, the bound
//! dimensions of the building and the site label.

use super::kit::*;
use super::layout::Pose;
use super::model::{En1991Snapshot, RoofArea};
use super::scene::*;
use super::{checks, handles, Ctx};

const ZONE_LIFT: f64 = 0.03;

/// 🏠 Roof form after the pitch is taken into account: a pitched roof with zero pitch is flat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofKind {
    Flat,
    Mono,
    Duo,
    Hip,
}

pub type Plan = Vec<[f64; 2]>;

/// 🌬️ One part of a wind zone: a lifted polygon in building coordinates and its outward normal.
pub struct ZonePart {
    pub points: Vec<Vec3>,
    pub normal: Vec3,
}

/// 🌬️ A wind zone letter A to J with every polygon it covers.
pub struct WindZone {
    pub letter: char,
    pub parts: Vec<ZonePart>,
}

/// 🏠 The roof surface over the building footprint `d` (along the wind) by `b` (across) at eaves height `h`, in building coordinates.
pub struct RoofShape {
    pub kind: RoofKind,
    pub tan: f64,
    pub d: f64,
    pub b: f64,
    pub h: f64,
}

fn kind_of(roof_type: &str) -> RoofKind {
    let t = roof_type.to_ascii_lowercase();
    if t.contains("hip") { RoofKind::Hip } else if t.contains("flat") { RoofKind::Flat } else if t.contains("mono") { RoofKind::Mono } else { RoofKind::Duo }
}

fn area(plan: &[[f64; 2]]) -> f64 {
    (0..plan.len()).map(|i| plan[i][0] * plan[(i + 1) % plan.len()][1] - plan[(i + 1) % plan.len()][0] * plan[i][1]).sum::<f64>().abs() / 2.0
}

fn clip_rect(plan: &[[f64; 2]], rect: [f64; 4]) -> Plan {
    let mut out = plan.to_vec();
    for (axis, bound, keep_above) in [(0, rect[0], true), (0, rect[2], false), (1, rect[1], true), (1, rect[3], false)] {
        let inside = |p: &[f64; 2]| if keep_above { p[axis] >= bound - 1e-12 } else { p[axis] <= bound + 1e-12 };
        let input = std::mem::take(&mut out);
        for (i, a) in input.iter().enumerate() {
            let b = input[(i + 1) % input.len()];
            if inside(a) != inside(&b) {
                let t = (bound - a[axis]) / (b[axis] - a[axis]);
                out.push([a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]);
            }
            if inside(&b) {
                out.push(b);
            }
        }
        if out.is_empty() {
            break;
        }
    }
    dedup(out)
}

fn clipped(plan: &[[f64; 2]], rect: [f64; 4]) -> Vec<Plan> {
    let part = clip_rect(plan, rect);
    if part.len() >= 3 && area(&part) > 1e-6 { vec![part] } else { Vec::new() }
}

fn newell(points: &[Vec3]) -> Vec3 {
    let mut n = v3(0.0, 0.0, 0.0);
    for (i, a) in points.iter().enumerate() {
        let b = &points[(i + 1) % points.len()];
        n = v3(n.x + (a.y - b.y) * (a.z + b.z), n.y + (a.z - b.z) * (a.x + b.x), n.z + (a.x - b.x) * (a.y + b.y));
    }
    unit(&n)
}

fn positive_rects(rects: Vec<Plan>) -> Vec<Plan> {
    rects.into_iter().filter(|r| r[2][0] - r[0][0] > 1e-6 && r[2][1] - r[0][1] > 1e-6).collect()
}

fn dedup(mut plan: Plan) -> Plan {
    plan.dedup_by(|a, b| (a[0] - b[0]).abs() < 1e-9 && (a[1] - b[1]).abs() < 1e-9);
    if plan.len() > 1 && (plan[0][0] - plan[plan.len() - 1][0]).abs() < 1e-9 && (plan[0][1] - plan[plan.len() - 1][1]).abs() < 1e-9 {
        plan.pop();
    }
    plan
}

impl RoofShape {
    pub fn of(document: &En1991Snapshot, roof: Option<&RoofArea>) -> Self {
        let (kind, pitch) = roof.map_or((RoofKind::Flat, 0.0), |r| (kind_of(&r.roof_type), r.pitch_deg.clamp(0.0, 60.0)));
        let tan = pitch.to_radians().tan();
        let kind = if tan < 1e-6 { RoofKind::Flat } else { kind };
        Self { kind, tan: if kind == RoofKind::Flat { 0.0 } else { tan }, d: document.depth.max(0.1), b: document.width.max(0.1), h: document.height.max(0.1) }
    }

    pub fn z(&self, x: f64, y: f64) -> f64 {
        let across = self.d - x;
        self.h
            + self.tan
                * match self.kind {
                    RoofKind::Flat => 0.0,
                    RoofKind::Mono => x,
                    RoofKind::Duo => x.min(across),
                    RoofKind::Hip => x.min(across).min(y.min(self.b - y)),
                }
    }

    pub fn rise(&self) -> f64 {
        self.tan
            * match self.kind {
                RoofKind::Flat => 0.0,
                RoofKind::Mono => self.d,
                RoofKind::Duo => self.d / 2.0,
                RoofKind::Hip => self.d.min(self.b) / 2.0,
            }
    }

    pub fn lift(&self, p: [f64; 2], above: f64) -> Vec3 {
        v3(p[0], p[1], self.z(p[0], p[1]) + above)
    }

    pub fn planar_limit(&self) -> Option<f64> {
        match self.kind {
            RoofKind::Flat | RoofKind::Mono => Some(self.d),
            RoofKind::Duo => Some(self.d / 2.0),
            RoofKind::Hip => None,
        }
    }

    pub fn slopes(&self) -> Vec<(&'static str, Plan)> {
        let (d, b) = (self.d, self.b);
        match self.kind {
            RoofKind::Flat => vec![("flat", rect2(0.0, 0.0, d, b))],
            RoofKind::Mono => vec![("slope", rect2(0.0, 0.0, d, b))],
            RoofKind::Duo => vec![("left", rect2(0.0, 0.0, d / 2.0, b)), ("right", rect2(d / 2.0, 0.0, d, b))],
            RoofKind::Hip => {
                let m = d.min(b) / 2.0;
                let (c0, c1, c2, c3, r1, r2) = ([0.0, 0.0], [d, 0.0], [d, b], [0.0, b], [m, m], [d - m, b - m]);
                let faces = if d >= b {
                    vec![("south", vec![c0, c1, r2, r1]), ("east", vec![c1, c2, r2]), ("north", vec![c2, c3, r1, r2]), ("west", vec![c3, c0, r1])]
                } else {
                    vec![("south", vec![c0, c1, r1]), ("east", vec![c1, c2, r2, r1]), ("north", vec![c2, c3, r2]), ("west", vec![c3, c0, r1, r2])]
                };
                faces.into_iter().map(|(tag, plan)| (tag, dedup(plan))).collect()
            }
        }
    }

    pub fn body(&self) -> Option<(Vec<Vec3>, Vec<Triangle>)> {
        if self.kind == RoofKind::Flat {
            return None;
        }
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut fan = |ring: Vec<Vec3>, reverse: bool| {
            let base = vertices.len() as u32;
            let n = ring.len() as u32;
            vertices.extend(ring);
            for k in 1..n - 1 {
                triangles.push(if reverse { Triangle { a: base, b: base + k + 1, c: base + k } } else { Triangle { a: base, b: base + k, c: base + k + 1 } });
            }
        };
        for (_, plan) in self.slopes() {
            fan(plan.iter().map(|p| self.lift(*p, 0.0)).collect(), false);
        }
        fan(rect2(0.0, 0.0, self.d, self.b).iter().map(|p| v3(p[0], p[1], self.h)).collect(), true);
        Some((vertices, triangles))
    }

    fn roof_zones(&self, e: f64) -> Vec<(char, Vec<Plan>)> {
        let (d, b) = (self.d, self.b);
        let ey = (e / 4.0).min(b / 2.0);
        let rect = |x0: f64, y0: f64, x1: f64, y1: f64| rect2(x0, y0, x1, y1);
        let corner = |ex: f64| (positive_rects(vec![rect(0.0, 0.0, ex, ey), rect(0.0, b - ey, ex, b)]), positive_rects(vec![rect(0.0, ey, ex, b - ey)]));
        match self.kind {
            RoofKind::Hip => {
                let planes = self.slopes();
                let plane = |tag: &str| planes.iter().find(|(t, _)| *t == tag).map(|(_, plan)| plan.clone()).unwrap_or_default();
                let (west, east, ex, mid) = (plane("west"), plane("east"), (e / 10.0).min(d), d / 2.0);
                let (mut f, mut k, mut l) = (clipped(&west, [0.0, 0.0, ex, ey]), Vec::new(), Vec::new());
                f.extend(clipped(&west, [0.0, b - ey, ex, b]));
                for tag in ["south", "north"] {
                    k.extend(clipped(&plane(tag), [0.0, 0.0, mid, b]));
                    l.extend(clipped(&plane(tag), [mid, 0.0, d, b]));
                }
                vec![('F', f), ('G', clipped(&west, [0.0, ey, ex, b - ey])), ('H', clipped(&west, [ex, 0.0, d, b])), ('I', clipped(&east, [0.0, 0.0, d, b])), ('K', k), ('L', l)]
            }
            RoofKind::Flat | RoofKind::Mono => {
                let ex = (e / 10.0).min(d);
                let (f, g) = corner(ex);
                let reach = if self.kind == RoofKind::Flat { (e / 2.0).min(d) } else { d };
                let mut zones = vec![('F', f), ('G', g), ('H', positive_rects(vec![rect(ex, 0.0, reach, b)]))];
                if self.kind == RoofKind::Flat {
                    zones.push(('I', positive_rects(vec![rect(reach, 0.0, d, b)])));
                }
                zones
            }
            RoofKind::Duo => {
                let w = d / 2.0;
                let f_width = (e / 10.0).min(w / 3.0);
                let (f, g) = corner(f_width);
                vec![('F', f), ('G', g), ('H', positive_rects(vec![rect(f_width, 0.0, w - f_width, b)])), ('I', positive_rects(vec![rect(w, 0.0, d, b)])), ('J', positive_rects(vec![rect(w - f_width, 0.0, w, b)]))]
            }
        }
    }

    fn wall_zones(&self, e: f64) -> Vec<WindZone> {
        let (d, b, h) = (self.d, self.b, self.h);
        let (a, bw, _) = checks::part_1_4::wall_zone_widths(e, d);
        let mut zones = Vec::new();
        let spans = [('A', 0.0, a), ('B', a, a + bw), ('C', a + bw, d)];
        for (letter, x0, x1) in spans {
            if x1 - x0 <= 1e-6 {
                continue;
            }
            let south = ZonePart { points: vec![v3(x0, -ZONE_LIFT, 0.0), v3(x1, -ZONE_LIFT, 0.0), v3(x1, -ZONE_LIFT, h), v3(x0, -ZONE_LIFT, h)], normal: v3(0.0, -1.0, 0.0) };
            let north = ZonePart { points: vec![v3(x1, b + ZONE_LIFT, 0.0), v3(x0, b + ZONE_LIFT, 0.0), v3(x0, b + ZONE_LIFT, h), v3(x1, b + ZONE_LIFT, h)], normal: v3(0.0, 1.0, 0.0) };
            zones.push(WindZone { letter, parts: vec![south, north] });
        }
        let windward = ZonePart { points: vec![v3(-ZONE_LIFT, 0.0, 0.0), v3(-ZONE_LIFT, 0.0, h), v3(-ZONE_LIFT, b, h), v3(-ZONE_LIFT, b, 0.0)], normal: v3(-1.0, 0.0, 0.0) };
        let leeward = ZonePart { points: vec![v3(d + ZONE_LIFT, 0.0, 0.0), v3(d + ZONE_LIFT, b, 0.0), v3(d + ZONE_LIFT, b, h), v3(d + ZONE_LIFT, 0.0, h)], normal: v3(1.0, 0.0, 0.0) };
        zones.push(WindZone { letter: 'D', parts: vec![windward] });
        zones.push(WindZone { letter: 'E', parts: vec![leeward] });
        zones
    }

    /// 🌬️ The wind zones A–L of the wind direction θ = 0°, sorted by letter, with `e = min(b, 2h)` (EN 1991-1-4 Fig. 7.5, 7.6, 7.7, 7.8). The hipped-roof partition (F, G, H on the windward plane, I on the leeward plane, K and L on the hip planes) is a pure-geometry reconstruction of Fig. 7.9 that is not checked against the printed figure; J, M and N are not placed.
    pub fn wind_zones(&self) -> Vec<WindZone> {
        let e = checks::part_1_4::reference_e(self.b, self.h);
        let mut zones = self.wall_zones(e);
        for (letter, plans) in self.roof_zones(e) {
            let parts: Vec<ZonePart> = plans
                .iter()
                .map(|plan| {
                    let points: Vec<Vec3> = plan.iter().map(|p| self.lift(*p, ZONE_LIFT)).collect();
                    let raw = newell(&points);
                    ZonePart { normal: if raw.z < 0.0 { mul(&raw, -1.0) } else { raw }, points }
                })
                .collect();
            if !parts.is_empty() {
                zones.push(WindZone { letter, parts });
            }
        }
        zones.sort_by_key(|z| z.letter);
        zones
    }
}

pub fn sub(a: &Vec3, b: &Vec3) -> Vec3 {
    v3(a.x - b.x, a.y - b.y, a.z - b.z)
}

pub fn world(pose: &Pose, points: &[Vec3]) -> Vec<Vec3> {
    points.iter().map(|p| pose.at(p.x, p.y, p.z)).collect()
}

fn dimension(out: &mut Out, pose: &Pose, id: &str, from: Vec3, to: Vec3, value: f64, path: &str) {
    let (a, b) = (pose.at(from.x, from.y, from.z), pose.at(to.x, to.y, to.z));
    out.annotate(Annotation::Dimension { id: id.into(), from: a.clone(), to: b, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, value), binding: Some(FieldBinding::new(path, Affine::IDENTITY, "m")) }, &a);
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    building(ctx, out);
    roofs(ctx, out);
}

fn building(ctx: &Ctx, out: &mut Out) {
    if !out.own(global::ENVELOPE) {
        return;
    }
    let (d, layout) = (ctx.input.document, ctx.layout);
    let pose = &layout.building;
    let (depth, width, height) = (d.depth.max(0.1), d.width.max(0.1), d.height.max(0.1));
    let corners = [pose.at(0.0, 0.0, 0.0), pose.at(depth, width, height)];
    out.push(
        Spec::new(
            EntityKind::Solid,
            "building",
            "@site",
            Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([0.0, 0.0, 0.0]), base: polygon2(&rect2(0.0, 0.0, depth, width)), height } }),
            MaterialRole::Neutral,
            loc("Building envelope", "Gebäudehülle"),
            LAYER_ENVELOPE,
        )
        .opacity(0.3)
        .detail(bi(|de| format!("{} x {} m, {} {} m", num(depth, 2, de), num(width, 2, de), if de { "Traufhöhe" } else { "eaves height" }, num(height, 2, de))))
        .handles(handles::ids(d, layout, "building")),
        &corners,
    );
    dimension(out, pose, "building:height", v3(depth + 1.0, 0.0, 0.0), v3(depth + 1.0, 0.0, height), height, "height");
    dimension(out, pose, "building:width", v3(-1.0, 0.0, 0.0), v3(-1.0, width, 0.0), width, "width");
    dimension(out, pose, "building:depth", v3(0.0, -1.5, 0.0), v3(depth, -1.5, 0.0), depth, "depth");
    let rise = RoofShape::of(d, d.roofs.first()).rise();
    let top = pose.at(depth / 2.0, width / 2.0, height + rise + 1.5);
    out.label(
        "building:site".into(),
        top,
        bi(|de| {
            if de {
                format!("Standort: Windzone {}, Geländekategorie {}, Schneezone {}, {} m ü. NHN", d.wind_zone, d.terrain_category, d.snow_zone, num(d.altitude, 0, true))
            } else {
                format!("Site: wind zone {}, terrain category {}, snow zone {}, altitude {} m", d.wind_zone, d.terrain_category, d.snow_zone, num(d.altitude, 0, false))
            }
        }),
        Tone::Neutral,
    );
    let arrow = pose.at(-6.0, width / 2.0, 1.0);
    out.label("building:wind-direction".into(), arrow, loc("Wind direction θ = 0° (+X)", "Windrichtung θ = 0° (+X)"), Tone::Info);
}

fn roofs(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    for (index, roof) in d.roofs.iter().enumerate() {
        if !out.own(&roof.id) {
            continue;
        }
        let shape = RoofShape::of(d, Some(roof));
        let roof_pose = layout.roof(index);
        let ids = handles::ids(d, layout, &roof.id);
        let name = bi(|de| format!("{} {}", if de { "Dach" } else { "Roof" }, roof.id));
        let detail = bi(|de| format!("{}, {} {}°", roof.roof_type, if de { "Neigung" } else { "pitch" }, num(roof.pitch_deg, 1, de)));
        if index == 0 {
            if let Some((vertices, triangles)) = shape.body() {
                let world_vertices = world(roof_pose, &vertices);
                out.push(Spec::new(EntityKind::Solid, format!("{}:body", roof.id), "roofs", Geometry::Solid(SolidGeometry { shape: SolidShape::Mesh { vertices: world_vertices.clone(), triangles } }), MaterialRole::Neutral, name, LAYER_ENVELOPE).opacity(0.45).detail(detail).handles(ids), &world_vertices);
            }
            continue;
        }
        for (tag, plan) in shape.slopes() {
            let points = world(roof_pose, &plan.iter().map(|p| shape.lift(*p, 0.0)).collect::<Vec<_>>());
            out.push(
                Spec::new(EntityKind::Surface, format!("{}:body-{tag}", roof.id), "roofs", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&points) } }), MaterialRole::Neutral, name.clone(), LAYER_ENVELOPE)
                    .opacity(0.6)
                    .detail(detail.clone())
                    .handles(ids.clone()),
                &points,
            );
        }
    }
}
