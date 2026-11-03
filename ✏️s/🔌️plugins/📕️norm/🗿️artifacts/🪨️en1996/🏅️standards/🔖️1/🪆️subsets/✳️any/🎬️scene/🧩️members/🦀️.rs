//! 🧩️ Members: the lintels, piers, ties, anchors, ring ties, tendons, confining columns and reinforcing bars of a wall as scene entities with their drag handles, each bound to the check that judges it.

use super::kit::*;
use super::layout::{openings, WallPose, EPS};
use super::model::{BarKind, MasonryWall, TieKind};
use super::lateral::FLANGE_TAG;
use super::reinforcement::bar_diameter;
use super::scene::projection_kit::handle_kit::{grid, handle, field_binding as binding, round, HandleSpec as HSpec};
use super::scene::*;
use super::{norm, SceneInput};

const PROUD: f64 = 0.01;
const TIE_DIAMETER: f64 = 0.006;
const ANCHOR_DIAMETER: f64 = 0.016;
const RING_DIAMETER: f64 = 0.012;
const ANCHOR_REACH: f64 = 0.6;
const BAR_END_MARGIN: f64 = 0.05;

fn path(wall: &MasonryWall, coll: &str, id: &str, leaf: &str) -> String {
    format!("walls[id={}].{coll}[id={id}].{leaf}", wall.id)
}

fn member_binding(report: &norm::CheckReport, entry: &str, wall: &MasonryWall, coll: &str, id: &str, suffix: &str) -> Option<UtilisationBinding> {
    let tail = format!("{}.{coll}.{id}{suffix}", wall.id);
    let head = format!("en1996.{entry}.");
    binding_by(report, |c| c.id.starts_with(&head) && c.id.ends_with(&tail))
}

/// 🛤️ The lower edge `z` of a lintel: the head of the opening it spans, otherwise hung below the wall head.
pub fn lintel_base(wall: &MasonryWall, position: f64, span: f64, depth: f64) -> f64 {
    openings(wall)
        .into_iter()
        .filter(|o| o.x0 < position + span - EPS && o.x1 > position + EPS)
        .map(|o| o.z1)
        .fold(None, |acc: Option<f64>, z| Some(acc.map_or(z, |a| a.max(z))))
        .unwrap_or_else(|| (wall.height_m - depth).max(0.0))
}

fn cuboid_entity(wall: &MasonryWall, pose: &WallPose, tag: &str, coll: &str, id: &str, x: (f64, f64), y: (f64, f64), z: (f64, f64), role: MaterialRole, layer: &'static str, name: LocalizedCopy, detail: LocalizedCopy) -> Option<(Spec, Vec<Vec3>)> {
    if x.1 - x.0 <= EPS || z.1 - z.0 <= EPS {
        return None;
    }
    let mut mesh = MeshBuilder::default();
    mesh.cuboid(pose, x, y, z);
    let points = mesh.points().to_vec();
    let mut spec = Spec::new(EntityKind::Solid, format!("{}:{tag}{id}", wall.id), mesh.solid(), role, layer, name);
    spec.nested = format!(".{coll}[id={id}]");
    spec.detail = detail;
    Some((spec, points))
}

fn fastener(wall: &MasonryWall, tag: &str, coll: &str, id: &str, kind: FastenerKind, from: Vec3, to: Vec3, diameter: f64, name: LocalizedCopy, detail: LocalizedCopy) -> (Spec, Vec<Vec3>) {
    let geometry = Geometry::Fastener(FastenerGeometry { kind, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter, head: HeadShape::None, pattern: None, hole_clearance: None });
    let mut spec = Spec::new(EntityKind::Fastener, format!("{}:{tag}{id}", wall.id), geometry, MaterialRole::Rebar, LAYER_REINFORCEMENT, name);
    spec.nested = format!(".{coll}[id={id}]");
    spec.detail = detail;
    (spec, vec![from, to])
}

pub(crate) fn project(input: &SceneInput, layout: &super::Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        if wall.length_m <= EPS || wall.height_m <= EPS || wall.thickness_m <= EPS {
            continue;
        }
        let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
        let doc = input.document;
        let push = |out: &mut Out, built: Option<(Spec, Vec<Vec3>)>, binding: Option<UtilisationBinding>| {
            if let Some((mut spec, points)) = built {
                spec.binding = binding;
                spec.handles = super::handles::ids(doc, layout, &spec.id);
                out.push(spec, &points);
            }
        };
        for lt in &wall.lintels {
            let z0 = lintel_base(wall, lt.position_m, lt.span_m, lt.depth_m);
            let built = cuboid_entity(wall, pose, LINTEL_TAG, "lintels", &lt.id, ((lt.position_m - lt.bearing_length_m).max(0.0), (lt.position_m + lt.span_m + lt.bearing_length_m).min(l)), (-PROUD, t + PROUD), (z0, (z0 + lt.depth_m).min(h)), MaterialRole::Concrete, LAYER_OPENINGS, loc(&format!("Lintel {}", lt.id), &format!("Sturz {}", lt.id)), loc(&format!("Span {:.2} m, depth {:.0} mm, bearing {:.0} mm each side", lt.span_m, lt.depth_m * 1000.0, lt.bearing_length_m * 1000.0), &format!("Lichtweite {:.2} m, Höhe {:.0} mm, Auflager {:.0} mm je Seite", lt.span_m, lt.depth_m * 1000.0, lt.bearing_length_m * 1000.0)));
            let covers: Vec<String> = wall.openings.iter().filter(|o| super::masonry::covering_lintel(wall, o).is_some_and(|x| x.id == lt.id)).map(|o| format!("en1996.5.5.lintel-cover.{}.{}", wall.id, o.id)).collect();
            let serviceability = format!("{}.lintels.{}.serviceability", wall.id, lt.id);
            push(out, built, binding_by(input.report, |c| (c.id.starts_with("en1996.m-61.") && c.id.ends_with(&serviceability)) || covers.contains(&c.id)));
        }
        for p in &wall.piers {
            let built = cuboid_entity(wall, pose, PIER_TAG, "piers", &p.id, (p.position_m.max(0.0), (p.position_m + p.width_m).min(l)), (-PROUD, t + PROUD), (0.0, h), MaterialRole::Masonry, LAYER_OPENINGS, loc(&format!("Pier {}", p.id), &format!("Pfeiler {}", p.id)), loc(&format!("Pier {:.2} m wide at x = {:.2} m", p.width_m, p.position_m), &format!("Pfeiler {:.2} m breit bei x = {:.2} m", p.width_m, p.position_m)));
            let built = built.map(|(mut s, p)| {
                s.opacity = 0.35;
                (s, p)
            });
            push(out, built, member_binding(input.report, "m-15", wall, "piers", &p.id, ""));
        }
        for c in &wall.confined {
            let built = cuboid_entity(wall, pose, COLUMN_TAG, "confined", &c.id, ((c.position_m - 0.5 * c.width_m).max(0.0), (c.position_m + 0.5 * c.width_m).min(l)), (0.5 * (t - c.depth_m), 0.5 * (t + c.depth_m)), (0.0, h), MaterialRole::Concrete, LAYER_REINFORCEMENT, loc(&format!("Confining column {}", c.id), &format!("Einfassungsstütze {}", c.id)), loc(&format!("{:.0} × {:.0} mm, A_s = {:.0} mm²", c.width_m * 1000.0, c.depth_m * 1000.0, c.as_m2 * 1e6), &format!("{:.0} × {:.0} mm, A_s = {:.0} mm²", c.width_m * 1000.0, c.depth_m * 1000.0, c.as_m2 * 1e6)));
            push(out, built, member_binding(input.report, "m-43", wall, "confined", &wall.confined[0].id, ""));
        }
        for tie in &wall.ties {
            let x = tie.position_m.clamp(0.0, l);
            let (kind, from, to, d, en, de) = match tie.kind {
                TieKind::WallTie => (FastenerKind::Dowel, pose.at(x, -0.05, tie.level_m), pose.at(x, t + 0.05, tie.level_m), TIE_DIAMETER, "Cavity wall tie", "Drahtanker"),
                TieKind::Anchor => (FastenerKind::Anchor, pose.at(x, 0.0, tie.level_m), pose.at(x, t + ANCHOR_REACH, tie.level_m), ANCHOR_DIAMETER, "Wall-floor anchor", "Wand-Decken-Anker"),
                TieKind::RingTie => (FastenerKind::Rebar { bend: None }, pose.at(0.0, 0.5 * t, tie.level_m), pose.at(l, 0.5 * t, tie.level_m), RING_DIAMETER, "Ring tie", "Ringanker"),
            };
            let (spec, points) = fastener(wall, TIE_TAG, "ties", &tie.id, kind, from, to, d, loc(&format!("{en} {}", tie.id), &format!("{de} {}", tie.id)), loc(&format!("{en}, R_d = {:.1} kN at level {:.2} m", tie.capacity_n / 1000.0, tie.level_m), &format!("{de}, R_d = {:.1} kN auf Höhe {:.2} m", tie.capacity_n / 1000.0, tie.level_m)));
            let (entry, first, suffix) = match tie.kind {
                TieKind::WallTie => ("m-50", wall.ties.iter().find(|x| x.kind == TieKind::WallTie).map_or("", |x| x.id.as_str()), ".cavity"),
                _ => ("m-51", wall.ties.iter().find(|x| x.kind != TieKind::WallTie).map_or("", |x| x.id.as_str()), ".anchor"),
            };
            push(out, Some((spec, points)), member_binding(input.report, entry, wall, "ties", first, suffix));
        }
        for p in &wall.prestress {
            let x = p.position_m.clamp(0.0, l);
            let y = (0.5 * t + p.eccentricity_m).clamp(0.0, t);
            let (from, to) = (pose.at(x, y, 0.0), pose.at(x, y, h));
            let profile = Path3::Polyline(Polyline3 { points: vec![from.clone(), to.clone()] });
            let (spec, points) = fastener(wall, TENDON_TAG, "prestress", &p.id, FastenerKind::Tendon { profile, duct: None }, from, to, bar_diameter(p.area_m2), loc(&format!("Tendon {}", p.id), &format!("Spannglied {}", p.id)), loc(&format!("P0 = {:.1} kN, A_p = {:.0} mm², eccentricity {:.0} mm", p.force_n / 1000.0, p.area_m2 * 1e6, p.eccentricity_m * 1000.0), &format!("P0 = {:.1} kN, A_p = {:.0} mm², Ausmitte {:.0} mm", p.force_n / 1000.0, p.area_m2 * 1e6, p.eccentricity_m * 1000.0)));
            push(out, Some((spec, points)), member_binding(input.report, "m-44", wall, "prestress", &p.id, ""));
        }
        let count = wall.bars.len() as f64;
        for (i, b) in wall.bars.iter().enumerate() {
            let f = (i as f64 + 1.0) / (count + 1.0);
            let (from, to) = match b.kind {
                BarKind::Anchorage => (pose.at(f * l, 0.5 * t, 0.0), pose.at(f * l, 0.5 * t, b.length_m.min(h))),
                BarKind::Shear => (pose.at(BAR_END_MARGIN, 0.5 * t, f * h), pose.at((BAR_END_MARGIN + b.length_m).min(l), 0.5 * t, f * h)),
                BarKind::DeepBeam => (pose.at(BAR_END_MARGIN, 0.5 * t, wall.cover_m + b.spacing_m * i as f64), pose.at((l - BAR_END_MARGIN).max(BAR_END_MARGIN), 0.5 * t, wall.cover_m + b.spacing_m * i as f64)),
            };
            let (en, de) = match b.kind {
                BarKind::Anchorage => ("Anchorage bar", "Verankerungsstab"),
                BarKind::Shear => ("Shear reinforcement bar", "Querkraftbewehrungsstab"),
                BarKind::DeepBeam => ("Deep-beam bar", "Wandträgerstab"),
            };
            let (spec, points) = fastener(wall, BAR_TAG, "bars", &b.id, FastenerKind::Rebar { bend: None }, from, to, b.diameter_m, loc(&format!("{en} {}", b.id), &format!("{de} {}", b.id)), loc(&format!("{en}, Ø{:.0} mm, length {:.2} m, spacing {:.2} m", b.diameter_m * 1000.0, b.length_m, b.spacing_m), &format!("{de}, Ø{:.0} mm, Länge {:.2} m, Abstand {:.2} m", b.diameter_m * 1000.0, b.length_m, b.spacing_m)));
            let (entry, suffix) = match b.kind {
                BarKind::Anchorage => ("m-45", ""),
                BarKind::Shear => ("m-41", ""),
                BarKind::DeepBeam => ("m-42", ".deep"),
            };
            let layout_id = format!("en1996.8.2.bar-layout.{}.{}", wall.id, b.id);
            let tail = format!("{}.bars.{}{suffix}", wall.id, b.id);
            let head = format!("en1996.{entry}.");
            push(out, Some((spec, points)), binding_by(input.report, |c| (c.id.starts_with(&head) && c.id.ends_with(&tail)) || c.id == layout_id));
        }
    }
}

pub(crate) fn grip<'a>(entity: &'a EntityRef, tag: &'a str, kind: HandleKind, anchor: Vec3, style: GripStyle, write: FieldBinding, step: f64, limits: (f64, f64), name: LocalizedCopy) -> Handle {
    handle(HSpec { entity, tag, kind, anchor, grip: style, writes: vec![write], snap: grid(step), limits, name })
}

/// 🔗️ A field binding quantised to the grid step.
pub(crate) fn field(path: String, map: Affine, unit: &str, step: f64) -> FieldBinding {
    binding(path, map, unit, round(step))
}

/// 🕹️ The handles of one member entity (`<wall>:<tag><id>`): position, extent and level grips bound to the member's own leaves.
pub(crate) fn handles(wall: &MasonryWall, pose: &WallPose, tag: &str) -> Vec<Handle> {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let ex = pose.axis_x();
    let ey = pose.axis_y();
    let mk = |kind: EntityKind, id: String, coll: &str, row: &str| EntityRef::new(kind, id, format!("walls[id={}].{coll}[id={row}]", wall.id));
    let pos = |entity: &EntityRef, row: &str, coll: &str, x: f64, z: f64, name: LocalizedCopy| grip(entity, "position", HandleKind::ScalarAlong { axis: ex.clone() }, pose.at(x, 0.5 * t, z), GripStyle::Diamond, binding(path(wall, coll, row, "positionM"), Affine::IDENTITY, "m", round(0.01)), 0.05, (0.0, l), name);
    if let Some(row) = tag.strip_prefix(PIER_TAG) {
        let Some(p) = wall.piers.iter().find(|p| p.id == row) else { return Vec::new() };
        let e = mk(EntityKind::Solid, format!("{}:{tag}", wall.id), "piers", row);
        return vec![
            pos(&e, row, "piers", p.position_m, 0.5 * h, loc("Pier position", "Pfeilerlage")),
            grip(&e, "width", HandleKind::ScalarAlong { axis: ex.clone() }, pose.at(p.position_m + p.width_m, 0.5 * t, 0.5 * h), GripStyle::Arrow, binding(path(wall, "piers", row, "widthM"), Affine { base: -p.position_m, k: 1.0 }, "m", round(0.01)), 0.05, (p.position_m + 0.1, l), loc("Pier width", "Pfeilerbreite")),
        ];
    }
    if let Some(row) = tag.strip_prefix(LINTEL_TAG) {
        let Some(lt) = wall.lintels.iter().find(|x| x.id == row) else { return Vec::new() };
        let z0 = lintel_base(wall, lt.position_m, lt.span_m, lt.depth_m);
        let e = mk(EntityKind::Solid, format!("{}:{tag}", wall.id), "lintels", row);
        return vec![
            pos(&e, row, "lintels", lt.position_m, z0 + lt.depth_m, loc("Lintel position", "Sturzlage")),
            grip(&e, "span", HandleKind::ScalarAlong { axis: ex.clone() }, pose.at(lt.position_m + lt.span_m, 0.5 * t, z0 + lt.depth_m), GripStyle::Arrow, binding(path(wall, "lintels", row, "spanM"), Affine { base: -lt.position_m, k: 1.0 }, "m", round(0.01)), 0.05, (lt.position_m + 0.1, l), loc("Lintel span", "Sturz-Lichtweite")),
            grip(&e, "depth", HandleKind::ScalarAlong { axis: Vec3::Z }, pose.at(lt.position_m + 0.5 * lt.span_m, 0.5 * t, z0 + lt.depth_m), GripStyle::Arrow, binding(path(wall, "lintels", row, "depthM"), Affine { base: -z0, k: 1.0 }, "m", round(0.01)), 0.05, (z0 + 0.1, h), loc("Lintel depth", "Sturzhöhe")),
            grip(&e, "bearing", HandleKind::ScalarAlong { axis: ex.clone() }, pose.at(lt.position_m - lt.bearing_length_m, 0.5 * t, z0), GripStyle::Diamond, binding(path(wall, "lintels", row, "bearingLengthM"), Affine { base: lt.position_m, k: -1.0 }, "m", round(0.01)), 0.05, (lt.position_m - 1.0, lt.position_m - 0.01), loc("Lintel bearing length", "Sturz-Auflagerlänge")),
        ];
    }
    if let Some(row) = tag.strip_prefix(COLUMN_TAG) {
        let Some(c) = wall.confined.iter().find(|x| x.id == row) else { return Vec::new() };
        let e = mk(EntityKind::Solid, format!("{}:{tag}", wall.id), "confined", row);
        return vec![
            pos(&e, row, "confined", c.position_m, 0.5 * h, loc("Column position", "Stützenlage")),
            grip(&e, "width", HandleKind::ScalarAlong { axis: ex.clone() }, pose.at(c.position_m + 0.5 * c.width_m, 0.5 * t, 0.5 * h), GripStyle::Arrow, binding(path(wall, "confined", row, "widthM"), Affine { base: -c.position_m, k: 2.0 }, "m", round(0.01)), 0.01, (c.position_m, l), loc("Column width", "Stützenbreite")),
            grip(&e, "depth", HandleKind::ScalarAlong { axis: ey.clone() }, pose.at(c.position_m, 0.5 * (t + c.depth_m), 0.5 * h), GripStyle::Arrow, binding(path(wall, "confined", row, "depthM"), Affine { base: -0.5 * t, k: 2.0 }, "m", round(0.01)), 0.01, (0.5 * t, 2.0 * t), loc("Column depth", "Stützentiefe")),
        ];
    }
    if let Some(row) = tag.strip_prefix(TIE_TAG) {
        let Some(tie) = wall.ties.iter().find(|x| x.id == row) else { return Vec::new() };
        let e = mk(EntityKind::Fastener, format!("{}:{tag}", wall.id), "ties", row);
        return vec![
            pos(&e, row, "ties", tie.position_m, tie.level_m, loc("Tie position", "Ankerlage")),
            grip(&e, "level", HandleKind::ScalarAlong { axis: Vec3::Z }, pose.at(tie.position_m, 0.5 * t, tie.level_m), GripStyle::Arrow, binding(path(wall, "ties", row, "levelM"), Affine::IDENTITY, "m", round(0.01)), 0.05, (0.0, h), loc("Tie level", "Ankerhöhe")),
        ];
    }
    if let Some(row) = tag.strip_prefix(TENDON_TAG) {
        let Some(p) = wall.prestress.iter().find(|x| x.id == row) else { return Vec::new() };
        let e = mk(EntityKind::Fastener, format!("{}:{tag}", wall.id), "prestress", row);
        return vec![
            pos(&e, row, "prestress", p.position_m, 0.5 * h, loc("Tendon position", "Spanngliedlage")),
            grip(&e, "eccentricity", HandleKind::ScalarAlong { axis: ey.clone() }, pose.at(p.position_m, 0.5 * t + p.eccentricity_m, 0.25 * h), GripStyle::Diamond, binding(path(wall, "prestress", row, "eccentricityM"), Affine { base: -0.5 * t, k: 1.0 }, "m", round(0.005)), 0.005, (0.0, t), loc("Tendon eccentricity", "Spanngliedausmitte")),
        ];
    }
    if let Some(row) = tag.strip_prefix(FLANGE_TAG) {
        let Some(f) = wall.flanges.iter().find(|x| x.id == row) else { return Vec::new() };
        let e = mk(EntityKind::Solid, format!("{}:{tag}", wall.id), "flanges", row);
        return vec![
            pos(&e, row, "flanges", f.position_m, 0.5 * h, loc("Flange position", "Flanschlage")),
            grip(&e, "length", HandleKind::ScalarAlong { axis: ey.clone() }, pose.at(f.position_m + 0.5 * f.thickness_m, t + f.length_m, 0.5 * h), GripStyle::Arrow, binding(path(wall, "flanges", row, "lengthM"), Affine { base: -t, k: 1.0 }, "m", round(0.01)), 0.05, (t + 0.1, t + 6.0), loc("Flange length", "Flanschlänge")),
            grip(&e, "thickness", HandleKind::ScalarAlong { axis: ex.clone() }, pose.at(f.position_m + f.thickness_m, t + 0.5 * f.length_m, 0.5 * h), GripStyle::Diamond, binding(path(wall, "flanges", row, "thicknessM"), Affine { base: -f.position_m, k: 1.0 }, "m", round(0.01)), 0.01, (f.position_m + 0.02, f.position_m + 1.0), loc("Flange thickness", "Flanschdicke")),
        ];
    }
    if let Some(row) = tag.strip_prefix(BAR_TAG) {
        let Some(i) = wall.bars.iter().position(|x| x.id == row) else { return Vec::new() };
        let b = &wall.bars[i];
        let f = (i as f64 + 1.0) / (wall.bars.len() as f64 + 1.0);
        let e = mk(EntityKind::Fastener, format!("{}:{tag}", wall.id), "bars", row);
        let (kind, anchor, aff) = match b.kind {
            BarKind::Anchorage => (HandleKind::ScalarAlong { axis: Vec3::Z }, pose.at(f * l, 0.5 * t, b.length_m.min(h)), Affine::IDENTITY),
            BarKind::Shear => (HandleKind::ScalarAlong { axis: ex.clone() }, pose.at((BAR_END_MARGIN + b.length_m).min(l), 0.5 * t, f * h), Affine { base: -BAR_END_MARGIN, k: 1.0 }),
            BarKind::DeepBeam => (HandleKind::ScalarAlong { axis: ex.clone() }, pose.at((l - BAR_END_MARGIN).max(BAR_END_MARGIN), 0.5 * t, wall.cover_m + b.spacing_m * i as f64), Affine { base: -BAR_END_MARGIN, k: 1.0 }),
        };
        let (gap_kind, gap_anchor, gap_aff) = match b.kind {
            BarKind::Anchorage => (HandleKind::ScalarAlong { axis: ex.clone() }, pose.at(f * l + b.spacing_m, 0.5 * t, 0.5 * b.length_m.min(h)), Affine { base: -f * l, k: 1.0 }),
            BarKind::Shear => (HandleKind::ScalarAlong { axis: Vec3::Z }, pose.at(BAR_END_MARGIN + 0.5 * b.length_m, 0.5 * t, f * h + b.spacing_m), Affine { base: -f * h, k: 1.0 }),
            BarKind::DeepBeam => (HandleKind::ScalarAlong { axis: Vec3::Z }, pose.at(0.5 * l, 0.5 * t, wall.cover_m + b.spacing_m), Affine { base: -wall.cover_m, k: 1.0 }),
        };
        return vec![
            grip(&e, "length", kind, anchor, GripStyle::Arrow, binding(path(wall, "bars", row, "lengthM"), aff, "m", round(0.01)), 0.05, (0.1, l.max(h)), loc("Bar length", "Stablänge")),
            grip(&e, "spacing", gap_kind, gap_anchor, GripStyle::Diamond, binding(path(wall, "bars", row, "spacingM"), gap_aff, "m", round(0.005)), 0.01, (0.0, 2.0 * (l + h)), loc("Bar spacing", "Stababstand")),
        ];
    }
    Vec::new()
}
