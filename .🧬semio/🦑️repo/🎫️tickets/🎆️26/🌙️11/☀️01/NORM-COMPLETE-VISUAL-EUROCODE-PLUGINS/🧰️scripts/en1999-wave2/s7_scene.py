"""Stage 7: 3D weld beads, HAZ zones and outline extrusion, handles and diagrams for weld runs and outline vertices. Idempotent."""
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from spec import *

S = ART / "🎬️scene"


def rw(p, fn):
    p = pathlib.Path(p)
    t = p.read_text(encoding="utf8")
    n = fn(t)
    if n != t:
        p.write_text(n, encoding="utf8")


def must(t, a):
    assert a in t, a[:100]
    return a


def kit(t):
    if "section_shape_of" in t:
        return t
    t = t.replace('pub(crate) const LAYER_LOADS', 'pub(crate) const LAYER_WELDS: &str = "welds";\npub(crate) const LAYER_LOADS', 1)
    t = t.replace("const LAYERS: [(&str, &str, &str); 11] = [", "const LAYERS: [(&str, &str, &str); 12] = [", 1)
    t = t.replace('    (LAYER_CONNECTIONS, "Connections", "Anschlüsse"),\n', '    (LAYER_CONNECTIONS, "Connections", "Anschlüsse"),\n    (LAYER_WELDS, "Weld runs", "Schweißnähte"),\n', 1)
    t = t.replace('    kind("fatigueDetail", "fatigueDetails", "Fatigue detail", "Ermüdungsdetail", "repeat", EntityKind::Group),\n', '    kind("fatigueDetail", "fatigueDetails", "Fatigue detail", "Ermüdungsdetail", "repeat", EntityKind::Group),\n    kind("weldRun", "weldRuns", "Weld run", "Schweißnaht", "flame", EntityKind::Fastener),\n', 1)
    t = t.replace("let found: [(&str, bool); 6] = [", "let found: [(&str, bool); 7] = [", 1)
    t = t.replace('        ("fatigueDetails", d.fatigue_details.iter().any(|e| e.id == base)),\n    ];', '        ("fatigueDetails", d.fatigue_details.iter().any(|e| e.id == base)),\n        ("weldRuns", d.weld_runs.iter().any(|e| e.id == base)),\n    ];', 1)
    t = t.replace("        out.extend(d.fatigue_details.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));\n    }", "        out.extend(d.fatigue_details.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));\n        out.extend(d.weld_runs.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));\n        out.extend(d.outline_vertices.iter().filter(|v| v.section_id == m.section_id).map(|v| v.id.clone()));\n    }\n    for run in d.weld_runs.iter().filter(|e| e.id == base) {\n        out.insert(run.member_id.clone());\n    }", 1)
    t = t.replace("/// 📐️ The overall height of the drawn section (tube: diameter).", '''/// 🔷️ The free outline of a section as a planar polygon with holes, when its vertices form a valid polygon.
pub(crate) fn outline_polygon(d: &model::En1999Snapshot, s: &model::AluminiumSection) -> Option<Polygon2> {
    let rings = checks::rings(&d.outline_vertices, &s.id);
    checks::outline_props(&rings)?;
    let ring = |r: &checks::Ring| Ring2 { points: r.points.iter().map(|p| Vec2::new(p.0, p.1)).collect() };
    let outer = rings.iter().find(|r| r.name == "outer")?;
    Some(Polygon2 { outer: ring(outer), holes: rings.iter().filter(|r| r.name != "outer").map(ring).collect() })
}

/// 🧱️ The drawn section: the outline polygon when the section has a valid outline, else the parametric source of [`section_shape`].
pub(crate) fn section_shape_of(d: &model::En1999Snapshot, s: &model::AluminiumSection) -> SectionShape {
    outline_polygon(d, s).map_or_else(|| section_shape(s), |polygon| SectionShape::Polygon { polygon })
}

/// 📐️ Overall depth and width of the drawn section (bounding box of the outline when there is one).
pub(crate) fn section_extent(d: &model::En1999Snapshot, s: &model::AluminiumSection) -> (f64, f64) {
    match outline_polygon(d, s) {
        Some(p) => {
            let span = |f: fn(&Vec2) -> f64| {
                let v: Vec<f64> = p.outer.points.iter().map(f).collect();
                v.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - v.iter().cloned().fold(f64::INFINITY, f64::min)
            };
            (span(|q| q.y).max(1e-3), span(|q| q.x).max(1e-3))
        }
        None => (section_depth(s), section_width(s)),
    }
}

/// 📐️ The overall height of the drawn section (tube: diameter).''', 1)
    return t


def handles(t):
    if "weld_run_handles" in t:
        return t
    t = t.replace("use super::kit::{along, base_id, entity_ref, loc, section_depth, section_width, v3};", "use super::kit::{along, base_id, entity_ref, loc, section_extent, section_depth, v3};\nuse super::checks;", 1)
    t = t.replace("fn weld_anchor(", "pub(crate) fn weld_anchor(", 1)
    t = t.replace("    let (depth, width) = (section_depth(section), section_width(section));\n    let tube", "    let (depth, width) = section_extent(d, section);\n    let tube", 1)
    new = r'''
const MAX_VERTEX_HANDLES: usize = 24;

fn run_anchor(d: &En1999Snapshot, member: &super::model::AluminiumMember, section: &AluminiumSection, run: &super::model::WeldRun, offset: f64) -> Option<(f64, f64, bool)> {
    let rings = checks::rings(&d.outline_vertices, &section.id);
    if checks::outline_props(&rings).is_some() {
        for ring in &rings {
            let n = ring.points.len();
            if let Some(i) = ring.elements.iter().position(|e| !run.element_id.is_empty() && *e == run.element_id) {
                let (p, q) = (ring.points[i], ring.points[(i + 1) % n]);
                let len = ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt().max(1e-9);
                let t = (offset / len).clamp(0.0, 1.0);
                return Some((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1), (q.1 - p.1).abs() > (q.0 - p.0).abs()));
            }
        }
    }
    let mut moved = run.clone();
    moved.offset = offset;
    let (_, resolved) = checks::resolved(member, section, &[&moved]);
    let index = resolved.elements.iter().position(|e| e.id == run.element_id)?;
    weld_anchor(&resolved, index)
}

/// 🧵️ Start, length, position across the element, throat and HAZ extent of a weld run.
fn weld_run_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(run) = d.weld_runs.iter().find(|r| r.id == id) else { return Vec::new() };
    let Some(member) = d.members.iter().find(|m| m.id == run.member_id) else { return Vec::new() };
    let (Some(pose), Some(section)) = (layout.member(&member.id), d.sections.iter().find(|s| s.id == member.section_id)) else { return Vec::new() };
    let Some((y, z, web)) = run_anchor(d, member, section, run, run.offset) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Fastener, id, "weldRuns");
    let f = &pose.frame;
    let point = |x: f64, y: f64, z: f64| along(&along(&along(&f.origin, &f.x, x), &f.y, y), &f.z, z);
    let path = |leaf: &str| format!("weldRuns[id={id}].{leaf}");
    let named = |en: &str, de: &str| loc(&format!("{en} of {id}"), &format!("{de} von {id}"));
    let mut out = vec![handle(Spec { entity: &entity, tag: "start", kind: HandleKind::ScalarAlong { axis: f.x.clone() }, anchor: point(run.start, y, z), grip: GripStyle::Arrow, writes: vec![binding(path("start"), "m", round(0.005))], snap: grid(0.01), limits: (0.0, member.length), name: named("Weld start", "Nahtanfang") })];
    if run.direction != "transverse" {
        let length = FieldBinding { path: path("length"), map: Affine { base: -run.start, k: 1.0 }, unit: "m".into(), quantise: round(0.005) };
        out.push(handle(Spec { entity: &entity, tag: "length", kind: HandleKind::ScalarAlong { axis: f.x.clone() }, anchor: point((run.start + run.length).min(member.length), y, z), grip: GripStyle::Arrow, writes: vec![length], snap: grid(0.01), limits: (run.start, member.length), name: named("Weld length", "Nahtlänge") }));
    }
    if let Some((y1, z1, _)) = run_anchor(d, member, section, run, run.offset + 1.0) {
        let (a0, a1) = if web { (z, z1) } else { (y, y1) };
        if (a1 - a0).abs() > 1e-9 {
            let k = 1.0 / (a1 - a0);
            let position = FieldBinding { path: path("offset"), map: Affine { base: run.offset - a0 * k, k }, unit: "m".into(), quantise: round(0.0005) };
            let axis = if web { f.z.clone() } else { f.y.clone() };
            out.push(handle(Spec { entity: &entity, tag: "position", kind: HandleKind::ScalarAlong { axis }, anchor: point(run.start, y, z), grip: GripStyle::Ring, writes: vec![position], snap: grid(0.0025), limits: (-1.0e3, 1.0e3), name: named("Weld position", "Nahtlage") }));
        }
    }
    out.push(handle(Spec { entity: &entity, tag: "throat", kind: HandleKind::FaceOffset { normal: f.z.clone() }, anchor: point(run.start, y, z + run.throat), grip: GripStyle::Diamond, writes: vec![binding(path("throat"), "m", round(0.0005))], snap: grid(0.0005), limits: (0.002, 0.03), name: named("Weld throat a", "Kehlnahtdicke a") }));
    out.push(handle(Spec { entity: &entity, tag: "haz-extent", kind: HandleKind::DimensionLine, anchor: point(run.start - run.haz_extent.max(1e-3), y, z), grip: GripStyle::Ring, writes: vec![binding(path("hazExtent"), "m", round(0.001))], snap: grid(0.005), limits: (0.0, 0.1), name: named("HAZ width b_haz", "WEZ-Breite b_haz") }));
    out
}

/// 🔷️ The y and z drag of every outline vertex of the section of a member.
fn vertex_handles(d: &En1999Snapshot, layout: &Layout, member_id: &str) -> Vec<Handle> {
    let Some(member) = d.members.iter().find(|m| m.id == member_id) else { return Vec::new() };
    let Some(pose) = layout.member(member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, member_id, "members");
    let f = &pose.frame;
    let mut out = Vec::new();
    for v in d.outline_vertices.iter().filter(|v| v.section_id == member.section_id).take(MAX_VERTEX_HANDLES) {
        let at = along(&along(&f.origin, &f.y, v.y), &f.z, v.z);
        let named = |axis: &str| loc(&format!("Outline vertex {} {axis}", v.id), &format!("Umrissknoten {} {axis}", v.id));
        out.push(handle(Spec { entity: &entity, tag: &format!("vertex-{}-y", v.id), kind: HandleKind::ScalarAlong { axis: f.y.clone() }, anchor: at.clone(), grip: GripStyle::Diamond, writes: vec![binding(format!("outlineVertices[id={}].y", v.id), "m", round(0.0005))], snap: grid(0.0025), limits: (-2.0, 2.0), name: named("y") }));
        out.push(handle(Spec { entity: &entity, tag: &format!("vertex-{}-z", v.id), kind: HandleKind::ScalarAlong { axis: f.z.clone() }, anchor: at, grip: GripStyle::Diamond, writes: vec![binding(format!("outlineVertices[id={}].z", v.id), "m", round(0.0005))], snap: grid(0.0025), limits: (-2.0, 2.0), name: named("z") }));
    }
    out
}
'''
    t = t.replace("fn connection_handles(", new.lstrip("\n") + "\nfn connection_handles(", 1)
    t = t.replace("    out\n}\n\nconst MAX_VERTEX_HANDLES", "    out\n}\n\nconst MAX_VERTEX_HANDLES", 1)
    t = t.replace("[member_handles(document, layout, id), connection_handles(document, layout, id), sheet_handles(document, layout, id), shell_handles(document, layout, id), fire_handles(document, layout, id)].concat()", "[member_handles(document, layout, id), vertex_handles(document, layout, id), weld_run_handles(document, layout, id), connection_handles(document, layout, id), sheet_handles(document, layout, id), shell_handles(document, layout, id), fire_handles(document, layout, id)].concat()", 1)
    t = t.replace("weld positions; bolt pitch", "weld positions, weld run start, length, position, throat and HAZ extent, outline vertices; bolt pitch", 1)
    return t


def members(t):
    if "weld_run_entities" in t:
        return t
    t = t.replace("use super::{eigen, handles, norm, SceneInput};", "use super::{checks, eigen, handles, norm, SceneInput};\nuse super::model::{PlateElement, WeldRun};", 1)
    new = r'''
fn run_geometry(d: &En1999Snapshot, member: &AluminiumMember, section: &AluminiumSection, run: &WeldRun, frame: &Frame) -> Option<(Vec3, Vec3, f64, f64, bool, PlateElement)> {
    let element = section.elements.iter().find(|e| e.id == run.element_id)?.clone();
    let rings = checks::rings(&d.outline_vertices, &section.id);
    let (y, z, web) = if checks::outline_props(&rings).is_some() {
        let ring = rings.iter().find(|r| r.elements.iter().any(|e| *e == run.element_id))?;
        let i = ring.elements.iter().position(|e| *e == run.element_id)?;
        let (p, q) = (ring.points[i], ring.points[(i + 1) % ring.points.len()]);
        let len = ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt().max(1e-9);
        let t = (run.offset / len).clamp(0.0, 1.0);
        (p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1), (q.1 - p.1).abs() > (q.0 - p.0).abs())
    } else {
        let (_, resolved) = checks::resolved(member, section, &[run]);
        let index = resolved.elements.iter().position(|e| e.id == run.element_id)?;
        handles::weld_anchor(&resolved, index)?
    };
    let at = |x: f64, dy: f64, dz: f64| along(&along(&along(&frame.origin, &frame.x, x), &frame.y, y + dy), &frame.z, z + dz);
    let (a, b) = if run.direction == "transverse" {
        let half = element.width / 2.0;
        let (dy, dz) = if web { (0.0, half) } else { (half, 0.0) };
        (at(run.start, -dy, -dz), at(run.start, dy, dz))
    } else {
        (at(run.start, 0.0, 0.0), at((run.start + run.length).min(member.length), 0.0, 0.0))
    };
    Some((a, b, y, z, web, element))
}

fn weld_run_entities(input: &SceneInput, member: &AluminiumMember, section: &AluminiumSection, frame: &Frame, out: &mut Out) {
    let d = input.document;
    for run in d.weld_runs.iter().filter(|r| r.member_id == member.id) {
        let Some((a, b, y, z, web, element)) = run_geometry(d, member, section, run, frame) else { continue };
        let throat = run.throat.max(1e-3);
        let b_haz = checks::haz_extent(run, element.thickness);
        let binding = binding(input.report, &[run.id.as_str()]);
        out.push(
            Spec {
                kind: EntityKind::Fastener,
                id: run.id.clone(),
                collection: "weldRuns",
                geometry: Geometry::Fastener(FastenerGeometry { kind: FastenerKind::WeldRun { throat, weld: WeldKind::Fillet, path: polyline(&[a.clone(), b.clone()]), pitch: None }, axis: Segment3 { from: a.clone(), to: b.clone() }, diameter: throat, head: HeadShape::None, pattern: None, hole_clearance: None }),
                role: MaterialRole::Weld,
                binding,
                name: loc(&format!("Weld run {}", run.id), &format!("Schweißnaht {}", run.id)),
                detail: loc(&format!("{} {} weld on {}, a = {:.1} mm, b_haz = {:.1} mm", run.process, run.direction, run.element_id, throat * 1e3, b_haz * 1e3), &format!("{}-{}-Naht an {}, a = {:.1} mm, b_haz = {:.1} mm", run.process, run.direction, run.element_id, throat * 1e3, b_haz * 1e3)),
                layer: LAYER_WELDS,
                handles: handles::ids(d, &Layout::of(d, input.placements), &run.id),
                deformation: None,
                style: None,
            },
            &[a.clone(), b.clone()],
        );
        let skin = 2.0 * HAZ_SKIN;
        let (span_from, span_to, size) = if run.direction == "transverse" {
            let from = along(&a, &frame.x, -b_haz);
            let to = along(&a, &frame.x, b_haz);
            (from, to, if web { (element.thickness + skin, element.width) } else { (element.width, element.thickness + skin) })
        } else {
            (a.clone(), b.clone(), if web { (element.thickness + skin, 2.0 * b_haz) } else { (2.0 * b_haz, element.thickness + skin) })
        };
        let centre = if run.direction == "transverse" { ((0.0, 0.0), size) } else { ((0.0, 0.0), size) };
        let _ = centre;
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: format!("{}:haz", run.id),
                collection: "weldRuns",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[span_from.clone(), span_to.clone()]),
                    frame: Frame { origin: span_from.clone(), ..frame.clone() },
                    section: SectionShape::Polygon { polygon: rect_polygon((y, z), size) },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                role: MaterialRole::Weld,
                binding: None,
                name: loc(&format!("Heat-affected zone of run {}", run.id), &format!("Wärmeeinflusszone der Naht {}", run.id)),
                detail: loc(&format!("b_haz = {:.1} mm per side", b_haz * 1e3), &format!("b_haz = {:.1} mm je Seite", b_haz * 1e3)),
                layer: LAYER_HAZ,
                handles: Vec::new(),
                deformation: None,
                style: Some((StyleState::Normal, 0.55, Some(Hatch::Dots))),
            },
            &[span_from, span_to],
        );
    }
}
'''
    t = t.replace("fn effective_entity(", new.lstrip("\n") + "\nfn effective_entity(", 1)
    t = t.replace("        let depth = section_depth(section);\n", "        let (depth, section_wide) = section_extent(d, section);\n", 1)
    t = t.replace("section: section_shape(section),\n                    section_taper: None,\n                    start_offset: Vec2::ZERO,\n                    end_offset: Vec2::ZERO,\n                    start_release: EndRelease::Rigid,\n                    end_release: if is_upright", "section: section_shape_of(d, section),\n                    section_taper: None,\n                    start_offset: Vec2::ZERO,\n                    end_offset: Vec2::ZERO,\n                    start_release: EndRelease::Rigid,\n                    end_release: if is_upright", 1)
    t = t.replace("tracks::push_mode(out, mode, &member.id, section_shape(section),", "tracks::push_mode(out, mode, &member.id, section_shape_of(d, section),", 1)
    t = t.replace("-(section_width(section) / 2.0 + 0.1)", "-(section_wide / 2.0 + 0.1)", 1)
    t = t.replace("        if let Some(alloy) = facts::alloy_of(d, &member.material_id, section) {\n            haz_entities(", "        weld_run_entities(input, member, section, &frame, out);\n        if let Some(alloy) = facts::alloy_of(d, &member.material_id, section).filter(|_| !checks::is_free(section)) {\n            haz_entities(", 1)
    t = t.replace("Heat-affected zone {} of {}", "Heat-affected zone {} of {}", 1)
    return t


def diagrams(t):
    if "fn outline_diagram" in t:
        return t
    new = r'''
fn outline_diagram(d: &En1999Snapshot, section: &AluminiumSection) -> Option<Diagram2d> {
    let rings = checks::rings(&d.outline_vertices, &section.id);
    let props = checks::outline_props(&rings)?;
    let mut series = Vec::new();
    let (mut ys, mut zs, mut ids) = (Vec::new(), Vec::new(), Vec::new());
    for ring in &rings {
        let mut points: Vec<Vec2> = ring.points.iter().map(|q| Vec2::new(q.0, q.1)).collect();
        points.push(points[0]);
        for q in &ring.points {
            ys.push(q.0);
            zs.push(q.1);
        }
        ids.push(ring.name.clone());
        series.push(Series::Polyline { id: format!("ring-{}", ring.name), label: loc(&format!("Ring {}", ring.name), &format!("Ring {}", ring.name)), points, style: stroke(if ring.name == "outer" { Tone::Info } else { Tone::Warning }, 2.0, Dash::Solid) });
    }
    let extent = |v: &[f64]| (v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let (y0, y1) = extent(&ys);
    let (z0, z1) = extent(&zs);
    let pad = 0.1 * (y1 - y0).max(z1 - z0).max(1e-3);
    Some(Diagram2d {
        id: format!("outline:{}", section.id),
        title: loc(&format!("Section outline – {}", section.id), &format!("Querschnittsumriss – {}", section.id)),
        description: loc(&format!("Polygon with holes of section {}: A = {:.1} mm², I_y = {:.3e} mm⁴, I_z = {:.3e} mm⁴; the centroid is marked.", section.id, props.a * 1e6, props.iy * 1e12, props.iz * 1e12), &format!("Polygon mit Löchern des Querschnitts {}: A = {:.1} mm², I_y = {:.3e} mm⁴, I_z = {:.3e} mm⁴; der Schwerpunkt ist markiert.", section.id, props.a * 1e6, props.iy * 1e12, props.iz * 1e12)),
        axes: vec![axis("Coordinate y", "Koordinate y", QuantityKind::Length, "m", AxisScale::Linear, Some((y0 - pad, y1 + pad))), axis("Coordinate z", "Koordinate z", QuantityKind::Length, "m", AxisScale::Linear, Some((z0 - pad, z1 + pad)))],
        series,
        markers: vec![Marker { id: "centroid".into(), at: Vec2::new(props.yg, props.zg), shape: MarkerShape::Circle, tone: Tone::Info, label: loc(&format!("Centroid ({:.1}, {:.1}) mm", props.yg * 1e3, props.zg * 1e3), &format!("Schwerpunkt ({:.1}, {:.1}) mm", props.yg * 1e3, props.zg * 1e3)) }],
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Coordinate y", "Koordinate y", "m", ys), column("Coordinate z", "Koordinate z", "m", zs)] },
    })
}

fn weld_run_diagram(d: &En1999Snapshot, member: &AluminiumMember, section: &AluminiumSection) -> Option<Diagram2d> {
    let runs: Vec<&WeldRun> = d.weld_runs.iter().filter(|r| r.member_id == member.id).collect();
    if runs.is_empty() {
        return None;
    }
    let mut series = Vec::new();
    let (mut starts, mut ends, mut hazs) = (Vec::new(), Vec::new(), Vec::new());
    for (k, run) in runs.iter().enumerate() {
        let t = section.elements.iter().find(|e| e.id == run.element_id).map_or(0.006, |e| e.thickness);
        let b = checks::haz_extent(run, t);
        let (from, to) = if run.direction == "transverse" { (run.start, run.start) } else { (run.start, (run.start + run.length).min(member.length)) };
        let level = k as f64 + 1.0;
        series.push(Series::Polyline { id: format!("haz-{}", run.id), label: loc(&format!("HAZ of {}", run.id), &format!("WEZ von {}", run.id)), points: vec![Vec2::new((from - b).max(0.0), level), Vec2::new((to + b).min(member.length), level)], style: stroke(Tone::Warning, 6.0, Dash::Solid) });
        series.push(Series::Polyline { id: format!("run-{}", run.id), label: loc(&format!("Weld {}", run.id), &format!("Naht {}", run.id)), points: vec![Vec2::new(from, level), Vec2::new(to.max(from + 1e-6), level)], style: stroke(Tone::Error, 2.5, Dash::Solid) });
        starts.push(from);
        ends.push(to);
        hazs.push(b);
    }
    Some(Diagram2d {
        id: format!("weld-runs:{}", member.id),
        title: loc(&format!("Weld runs and HAZ along {}", member.id), &format!("Schweißnähte und WEZ entlang {}", member.id)),
        description: loc("Each weld run along the member axis (red) with its heat-affected zone b_haz on both sides (EN 1999-1-1, 6.1.6.3); a transverse run is a point and drives the localized-weld factor ω_x (6.3.3.3).", "Jede Schweißnaht entlang der Bauteilachse (rot) mit ihrer Wärmeeinflusszone b_haz beidseitig (EN 1999-1-1, 6.1.6.3); eine Quernaht ist ein Punkt und bestimmt den Faktor ω_x der örtlichen Schweißnaht (6.3.3.3)."),
        axes: vec![axis("Position x along the member", "Lage x entlang des Bauteils", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, member.length))), axis("Weld run", "Schweißnaht", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, runs.len() as f64 + 1.0)))],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: member_focus(&member.id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Start x", "Anfang x", "m", starts), column("End x", "Ende x", "m", ends), column("HAZ b_haz", "WEZ b_haz", "m", hazs)] },
    })
}
'''
    t = t.replace("/// 📈️ Every diagram of the document;", new.lstrip("\n") + "\n/// 📈️ Every diagram of the document;", 1)
    t = t.replace("use super::model::{AluminiumMember, AluminiumSection, AluminiumShell, En1999Snapshot, FatigueDetail, FireScenario};", "use super::checks;\nuse super::model::{AluminiumMember, AluminiumSection, AluminiumShell, En1999Snapshot, FatigueDetail, FireScenario, WeldRun};", 1)
    t = t.replace("            if let Some((section, alloy)) = facts::member_parts(document, member) {", "            out.extend(document.sections.iter().find(|s| s.id == member.section_id).and_then(|s| outline_diagram(document, s)));\n            out.extend(document.sections.iter().find(|s| s.id == member.section_id).and_then(|s| weld_run_diagram(document, member, s)));\n            if let Some((section, alloy)) = facts::member_parts(document, member).filter(|(s, _)| !checks::is_free(s)) {", 1)
    return t


rw(S / "🧰️kit/🦀️.rs", kit)
rw(S / "🕹️handles/🦀️.rs", handles)
rw(S / "🏗️members/🦀️.rs", members)
rw(S / "📈️diagrams/🦀️.rs", diagrams)
print("stage 7 done")
