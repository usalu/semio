//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 📏️ Member length, buckling and LTB lengths, section depth, width and wall thicknesses (with every plate element tied to them), weld positions, weld run start, length, position, throat and HAZ extent, outline vertices; bolt pitch, gauge, distances and counts, weld length, throat and HAZ extent; sheet and shell dimensions; fire duration and temperature; fatigue stress range and cycles; the product form of the member material as a choice stepper.

use super::kit::{along, base_id, entity_ref, loc, section_extent, section_depth, v3};

#[path = "🎚️gauges/🦀️.rs"]
pub mod gauges;

const LIP_MARK: &str = ":edge-";
use super::checks;
use super::layout::{Layout, Placement};
use super::model::{AluminiumSection, En1999Snapshot, PlateElement};
use super::scene::*;
use super::scene::projection_kit::handle_kit::{choice_handle, grid, handle, HandleSpec as Spec, identity_binding as binding, round};

#[derive(Clone, Copy, PartialEq)]
enum Var {
    Height,
    Width,
    FlangeThickness,
    WebThickness,
}

fn dims(s: &AluminiumSection) -> [f64; 4] {
    [s.height, s.width, s.flange_thickness, s.web_thickness]
}

fn is_web(section: &AluminiumSection, element: &PlateElement) -> bool {
    let named = element.id.to_ascii_lowercase().contains("web");
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => named,
        _ => named || !element.outstand,
    }
}

fn plate_width(section: &AluminiumSection, element: &PlateElement, d: &[f64; 4]) -> Option<f64> {
    let [h, b, tf, tw] = *d;
    if is_web(section, element) {
        return Some(h - 2.0 * tf);
    }
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => Some(b - 2.0 * tw),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => Some(b - tw),
        semio_s_artifact_norm_en1999_model::SectionKind::Angle | semio_s_artifact_norm_en1999_model::SectionKind::Chs => None,
        _ => Some((b - tw) / 2.0),
    }
}

fn tied_writes(section: &AluminiumSection, var: Var) -> Vec<FieldBinding> {
    let d = dims(section);
    let index = match var {
        Var::Height => 0,
        Var::Width => 1,
        Var::FlangeThickness => 2,
        Var::WebThickness => 3,
    };
    let mut out = Vec::new();
    for element in &section.elements {
        let Some(current) = plate_width(section, element, &d) else { continue };
        if (element.width - current).abs() > 1e-6 {
            continue;
        }
        let mut stepped = d;
        stepped[index] += 1.0;
        let k = plate_width(section, element, &stepped).unwrap_or(current) - current;
        if k.abs() > 1e-12 {
            out.push(FieldBinding { path: format!("sections[id={}].elements[id={}].width", section.id, element.id), map: Affine { base: current - k * d[index], k }, unit: "m".into(), quantise: None });
        }
        let tied_thickness = match var {
            Var::FlangeThickness => !is_web(section, element) && (element.thickness - d[2]).abs() < 1e-9,
            Var::WebThickness => is_web(section, element) && (element.thickness - d[3]).abs() < 1e-9,
            _ => false,
        };
        if tied_thickness {
            out.push(binding(format!("sections[id={}].elements[id={}].thickness", section.id, element.id), "m", None));
        }
    }
    out
}

fn section_write(section: &AluminiumSection, leaf: &str, var: Var) -> Vec<FieldBinding> {
    let mut writes = vec![binding(format!("sections[id={}].{leaf}", section.id), "m", round(0.0005))];
    writes.extend(tied_writes(section, var));
    writes
}

pub fn weld_anchor(section: &AluminiumSection, index: usize) -> Option<(f64, f64, bool)> {
    let element = section.elements.get(index)?;
    let [h, b, tf, tw] = dims(section);
    let hw = h - 2.0 * tf;
    let wp = element.weld_position;
    let web = is_web(section, element);
    let rank = section.elements[..index].iter().filter(|e| is_web(section, e) == web).count();
    let sign = if rank == 0 { 1.0 } else { -1.0 };
    let (y, z) = match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs if web => (sign * (b - tw) / 2.0, hw / 2.0 - wp),
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => (-(b - 2.0 * tw) / 2.0 + wp, sign * (h - tf) / 2.0),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel if web => (-(b - tw) / 2.0, hw / 2.0 - wp),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => (tw / 2.0 + wp - (b - tw) / 2.0, sign * (h - tf) / 2.0),
        semio_s_artifact_norm_en1999_model::SectionKind::Angle | semio_s_artifact_norm_en1999_model::SectionKind::Chs => return None,
        _ if web => (0.0, hw / 2.0 - wp),
        _ => (tw / 2.0 + wp, sign * (h - tf) / 2.0),
    };
    Some((y, z, web))
}

/// 📍️ The persisted placement rows of the document as scene placements.
fn stored_placements(d: &En1999Snapshot) -> Vec<Placement> {
    d.placements.iter().map(|p| Placement { entity_id: p.id.clone(), origin: v3(p.x, p.y, p.z), yaw: p.yaw }).collect()
}

fn member_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == id), layout.member(id)) else { return Vec::new() };
    let Some(section) = d.sections.iter().find(|s| s.id == member.section_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "members");
    let f = &pose.frame;
    let (depth, width) = section_extent(d, section);
    let tube = super::checks::part_1_1::is_tube(section);
    let on_section = |x: f64, y: f64, z: f64| along(&along(&along(&f.origin, &f.x, x), &f.y, y), &f.z, z);
    let named = |en: &str, de: &str| loc(&format!("{en} of {}", member.id), &format!("{de} von {}", member.id));
    let mut out = vec![handle(Spec { entity: &entity, tag: "length", kind: HandleKind::ScalarAlong { axis: f.x.clone() }, anchor: along(&f.origin, &f.x, member.length), grip: GripStyle::Arrow, writes: vec![binding(format!("members[id={id}].length"), "m", round(0.01))], snap: grid(0.05), limits: (0.3, 60.0), name: named("Length", "Länge") })];
    for (tag, value, leaf, y_off, en, de) in [("buckling-y", member.buckling_length_y, "bucklingLengthY", depth / 2.0 + 0.05, "Buckling length L_cr,y", "Knicklänge L_cr,y"), ("buckling-z", member.buckling_length_z, "bucklingLengthZ", -(width / 2.0 + 0.05), "Buckling length L_cr,z", "Knicklänge L_cr,z")] {
        if value > 1e-6 {
            let anchor = if tag == "buckling-y" { on_section(value.min(member.length), 0.0, y_off) } else { on_section(value.min(member.length), y_off, 0.0) };
            out.push(handle(Spec { entity: &entity, tag, kind: HandleKind::ScalarAlong { axis: f.x.clone() }, anchor, grip: GripStyle::Ring, writes: vec![binding(format!("members[id={id}].{leaf}"), "m", round(0.01))], snap: grid(0.05), limits: (0.1, 120.0), name: named(en, de) }));
        }
    }
    if !member.restrained_ltb && member.ltb_length > 1e-6 {
        out.push(handle(Spec { entity: &entity, tag: "ltb-length", kind: HandleKind::ScalarAlong { axis: f.x.clone() }, anchor: on_section(member.ltb_length.min(member.length), 0.0, depth / 2.0), grip: GripStyle::Ring, writes: vec![binding(format!("members[id={id}].ltbLength"), "m", round(0.01))], snap: grid(0.05), limits: (0.1, 120.0), name: named("Unrestrained length L_LT", "Gabellagerungslänge L_LT") }));
    }
    if tube {
        out.push(handle(Spec { entity: &entity, tag: "diameter", kind: HandleKind::Radial { centre: f.origin.clone() }, anchor: on_section(member.length / 2.0, 0.0, depth / 2.0), grip: GripStyle::Diamond, writes: vec![binding(format!("sections[id={}].outerDiameter", section.id), "m", round(0.0005))], snap: grid(0.005), limits: (0.02, 1.0), name: named("Outer diameter", "Außendurchmesser") }));
        out.push(handle(Spec { entity: &entity, tag: "wall-thickness", kind: HandleKind::FaceOffset { normal: f.y.clone() }, anchor: on_section(member.length / 2.0, width / 2.0, 0.0), grip: GripStyle::Diamond, writes: vec![binding(format!("sections[id={}].webThickness", section.id), "m", round(0.0005))], snap: grid(0.0005), limits: (0.001, 0.05), name: named("Wall thickness", "Wanddicke") }));
    } else {
        out.push(handle(Spec { entity: &entity, tag: "depth", kind: HandleKind::FaceOffset { normal: f.z.clone() }, anchor: on_section(member.length / 2.0, 0.0, depth / 2.0), grip: GripStyle::Diamond, writes: section_write(section, "height", Var::Height), snap: grid(0.005), limits: (0.04, 1.0), name: named("Section depth", "Querschnittshöhe") }));
        out.push(handle(Spec { entity: &entity, tag: "width", kind: HandleKind::FaceOffset { normal: f.y.clone() }, anchor: on_section(member.length / 2.0, width / 2.0, 0.0), grip: GripStyle::Diamond, writes: section_write(section, "width", Var::Width), snap: grid(0.005), limits: (0.02, 0.6), name: named("Section width", "Querschnittsbreite") }));
        out.push(handle(Spec { entity: &entity, tag: "flange-thickness", kind: HandleKind::FaceOffset { normal: f.z.clone() }, anchor: on_section(member.length / 4.0, width / 4.0, (depth - section.flange_thickness) / 2.0), grip: GripStyle::Diamond, writes: section_write(section, "flangeThickness", Var::FlangeThickness), snap: grid(0.0005), limits: (0.001, 0.05), name: named("Flange thickness", "Flanschdicke") }));
        out.push(handle(Spec { entity: &entity, tag: "web-thickness", kind: HandleKind::FaceOffset { normal: f.y.clone() }, anchor: on_section(member.length / 4.0, section.web_thickness / 2.0, 0.0), grip: GripStyle::Diamond, writes: section_write(section, "webThickness", Var::WebThickness), snap: grid(0.0005), limits: (0.001, 0.05), name: named("Web thickness", "Stegdicke") }));
    }
    for (index, element) in section.elements.iter().enumerate().filter(|(_, e)| e.welded) {
        let Some((y, z, web)) = weld_anchor(section, index) else { continue };
        let axis = if web { f.z.clone() } else { f.y.clone() };
        let anchor = on_section(0.0, y, z);
        out.push(handle(Spec { entity: &entity, tag: &format!("weld-{}", element.id), kind: HandleKind::ScalarAlong { axis }, anchor, grip: GripStyle::Ring, writes: vec![binding(format!("sections[id={}].elements[id={}].weldPosition", section.id, element.id), "m", round(0.0005))], snap: grid(0.0025), limits: (0.0, element.width.max(0.01)), name: loc(&format!("Weld position on {} of {}", element.id, member.id), &format!("Schweißnahtlage auf {} von {}", element.id, member.id)) }));
    }
    if let Some(material) = d.materials.iter().find(|m| m.id == member.material_id) {
        out.push(choice_handle(&entity, "product-form", on_section(member.length * 0.75, 0.0, -depth / 2.0 - 0.05), format!("materials[id={}].productForm", material.id), &product_form_options(), loc(&format!("Product form of {} (Table 3.2)", material.id), &format!("Erzeugnisform von {} (Tabelle 3.2)", material.id))));
    }
    let stored = stored_placements(d);
    out.extend(super::scene::projection_kit::handle_kit::move_handles(&entity, id, &f.origin, &stored, 0.0, !super::layout::is_upright(member)));
    out
}

/// 🏭️ The product forms of EN 1999-1-1 Table 3.2 in enum order: wire token, English and German label.
fn product_form_options() -> Vec<(String, &'static str, &'static str)> {
    [("sheet", "Sheet, strip, plate", "Blech, Band, Platte"), ("extrudedProfile", "Extruded profile (EP)", "Strangpressprofil (EP)"), ("extrudedTube", "Extruded tube (ET)", "Strangpressrohr (ET)"), ("extrudedBar", "Extruded rod/bar (ER/B)", "Strangpressstange (ER/B)"), ("drawnTube", "Drawn tube (DT)", "Gezogenes Rohr (DT)"), ("forging", "Forging (FO)", "Schmiedestück (FO)")]
        .into_iter()
        .map(|(token, en, de)| (token.to_owned(), en, de))
        .collect()
}

pub const MAX_VERTEX_HANDLES: usize = 24;

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
    if run.direction != semio_s_artifact_norm_en1999_model::WeldDirection::Transverse {
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

fn connection_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(conn) = d.connections.iter().find(|c| c.id == id) else { return Vec::new() };
    let Some(pose) = layout.member(&conn.member_id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "connections");
    let f = &pose.frame;
    let section = d.members.iter().find(|m| m.id == conn.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id));
    let depth = section.map_or(0.2, section_depth);
    let (b, w) = (&conn.bolts, &conn.welds);
    let end = along(&f.origin, &f.x, pose.length + b.plate_thickness);
    let top = b.pitch * (b.rows.max(1) - 1) as f64 / 2.0;
    let half = b.gauge * (b.bolts_per_row.max(1) - 1) as f64 / 2.0;
    let int = Some(Quantise::Int);
    let path = |leaf: &str| format!("connections[id={id}].{leaf}");
    let mut out = Vec::new();
    if conn.kind != semio_s_artifact_norm_en1999_model::ConnectionKind::Welded {
        out.extend([
            handle(Spec { entity: &entity, tag: "pitch", kind: HandleKind::DimensionLine, anchor: along(&along(&end, &f.z, top), &f.y, half + b.edge_distance), grip: GripStyle::Arrow, writes: vec![binding(path("bolts.pitch"), "m", round(0.005))], snap: grid(0.005), limits: (0.02, 0.4), name: loc("Bolt pitch p_1", "Schraubenabstand p_1") }),
            handle(Spec { entity: &entity, tag: "gauge", kind: HandleKind::DimensionLine, anchor: along(&along(&end, &f.z, top + b.edge_distance), &f.y, half), grip: GripStyle::Arrow, writes: vec![binding(path("bolts.gauge"), "m", round(0.005))], snap: grid(0.005), limits: (0.02, 0.4), name: loc("Bolt gauge p_2", "Schraubenabstand p_2") }),
            handle(Spec { entity: &entity, tag: "edge-distance", kind: HandleKind::ScalarAlong { axis: f.y.clone() }, anchor: along(&end, &f.y, half + b.edge_distance), grip: GripStyle::Ring, writes: vec![binding(path("bolts.edgeDistance"), "m", round(0.002))], snap: grid(0.002), limits: (0.01, 0.2), name: loc("Edge and end distances e_1, e_2", "Rand- und Endabstände e_1, e_2") }),
            handle(Spec { entity: &entity, tag: "load-eccentricity", kind: HandleKind::ScalarAlong { axis: f.y.clone() }, anchor: along(&end, &f.y, conn.load_eccentricity), grip: GripStyle::Ring, writes: vec![binding(path("loadEccentricity"), "m", round(0.005))], snap: grid(0.005), limits: (0.0, 0.5), name: loc("Load eccentricity e to the group centroid", "Lastexzentrizität e zum Gruppenschwerpunkt") }),
            handle(Spec { entity: &entity, tag: "rows", kind: HandleKind::CountStepper, anchor: along(&end, &f.z, -top - b.edge_distance), grip: GripStyle::Diamond, writes: vec![binding(path("bolts.rows"), "1", int.clone())], snap: SnapRule::NONE, limits: (1.0, 8.0), name: loc("Number of bolt rows", "Anzahl Schraubenreihen") }),
            handle(Spec { entity: &entity, tag: "per-row", kind: HandleKind::CountStepper, anchor: along(&end, &f.y, -half - b.edge_distance), grip: GripStyle::Diamond, writes: vec![binding(path("bolts.boltsPerRow"), "1", int)], snap: SnapRule::NONE, limits: (1.0, 6.0), name: loc("Bolts per row", "Schrauben je Reihe") }),
            handle(Spec { entity: &entity, tag: "bolt-diameter", kind: HandleKind::Radial { centre: end.clone() }, anchor: along(&end, &f.y, half), grip: GripStyle::Diamond, writes: vec![binding(path("bolts.diameter"), "m", round(0.002))], snap: grid(0.002), limits: (0.004, 0.036), name: loc("Bolt diameter d", "Schraubendurchmesser d") }),
            handle(Spec { entity: &entity, tag: "plate-thickness", kind: HandleKind::FaceOffset { normal: f.x.clone() }, anchor: along(&end, &f.x, b.plate_thickness), grip: GripStyle::Diamond, writes: vec![binding(path("bolts.plateThickness"), "m", round(0.0005))], snap: grid(0.0005), limits: (0.002, 0.05), name: loc("Connected plate thickness", "Anschlussblechdicke") }),
        ]);
    }
    if conn.kind != semio_s_artifact_norm_en1999_model::ConnectionKind::Bolted {
        let weld_at = along(&along(&f.origin, &f.x, pose.length), &f.z, -depth / 2.0);
        out.extend([
            handle(Spec { entity: &entity, tag: "weld-length", kind: HandleKind::ScalarAlong { axis: f.y.clone() }, anchor: along(&weld_at, &f.y, w.length / 2.0), grip: GripStyle::Arrow, writes: vec![binding(path("welds.length"), "m", round(0.005))], snap: grid(0.01), limits: (0.02, 2.0), name: loc("Weld length", "Nahtlänge") }),
            handle(Spec { entity: &entity, tag: "weld-throat", kind: HandleKind::FaceOffset { normal: f.z.clone() }, anchor: along(&weld_at, &f.z, -0.02), grip: GripStyle::Diamond, writes: vec![binding(path("welds.throat"), "m", round(0.0005))], snap: grid(0.0005), limits: (0.002, 0.03), name: loc("Weld throat thickness a", "Nahtdicke a") }),
            handle(Spec { entity: &entity, tag: "haz-extent", kind: HandleKind::DimensionLine, anchor: along(&weld_at, &f.x, -w.haz_extent.max(1e-3)), grip: GripStyle::Ring, writes: vec![binding(path("welds.hazExtent"), "m", round(0.001))], snap: grid(0.005), limits: (0.0, 0.1), name: loc("HAZ width b_haz", "WEZ-Breite b_haz") }),
        ]);
    }
    out
}

fn sheet_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(sheet) = d.cold_formed.iter().find(|s| s.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "coldFormed");
    let o = layout.origin(id);
    let rib = 0.2 * sheet.width;
    let path = |leaf: &str| format!("coldFormed[id={id}].{leaf}");
    let mut out = vec![
        handle(Spec { entity: &entity, tag: "span", kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: v3(o.x + sheet.span, o.y, o.z), grip: GripStyle::Arrow, writes: vec![binding(path("span"), "m", round(0.01))], snap: grid(0.05), limits: (0.1, 12.0), name: loc("Sheet span", "Stützweite") }),
        handle(Spec { entity: &entity, tag: "width", kind: HandleKind::DimensionLine, anchor: v3(o.x + sheet.span / 2.0, o.y + 0.4 * sheet.width, o.z + rib / 2.0), grip: GripStyle::Arrow, writes: vec![binding(path("width"), "m", round(0.005))], snap: grid(0.005), limits: (0.02, 1.5), name: loc("Flat width", "Flachbreite") }),
        handle(Spec { entity: &entity, tag: "thickness", kind: HandleKind::FaceOffset { normal: Vec3::Z }, anchor: v3(o.x + sheet.span / 2.0, o.y, o.z + rib / 2.0 + sheet.thickness), grip: GripStyle::Diamond, writes: vec![binding(path("thickness"), "m", round(0.0001))], snap: grid(0.0002), limits: (0.0005, 0.02), name: loc("Sheet thickness", "Blechdicke") }),
    ];
    out.extend(super::scene::projection_kit::handle_kit::move_handles(&entity, id, &o, &stored_placements(d), 0.0, false));
    out
}

fn shell_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(shell) = d.shells.iter().find(|s| s.id == id) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Shell, id, "shells");
    let o = layout.origin(id);
    let path = |leaf: &str| format!("shells[id={id}].{leaf}");
    let mut out = vec![
        handle(Spec { entity: &entity, tag: "radius", kind: HandleKind::Radial { centre: o.clone() }, anchor: v3(o.x + shell.radius, o.y, o.z + shell.length / 2.0), grip: GripStyle::Arrow, writes: vec![binding(path("radius"), "m", round(0.01))], snap: grid(0.05), limits: (0.05, 10.0), name: loc("Shell radius", "Schalenradius") }),
        handle(Spec { entity: &entity, tag: "length", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x, o.y, o.z + shell.length), grip: GripStyle::Arrow, writes: vec![binding(path("length"), "m", round(0.05))], snap: grid(0.1), limits: (0.2, 30.0), name: loc("Shell length", "Schalenlänge") }),
        handle(Spec { entity: &entity, tag: "thickness", kind: HandleKind::FaceOffset { normal: Vec3::X }, anchor: v3(o.x + shell.radius, o.y, o.z + shell.length / 4.0), grip: GripStyle::Diamond, writes: vec![binding(path("thickness"), "m", round(0.0005))], snap: grid(0.0005), limits: (0.0005, 0.05), name: loc("Shell thickness", "Schalendicke") }),
    ];
    out.extend(super::scene::projection_kit::handle_kit::move_handles(&entity, id, &o, &stored_placements(d), 0.0, false));
    out
}

fn fire_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(fire) = d.fire_scenarios.iter().find(|f| f.id == id) else { return Vec::new() };
    let Some((pose, section)) = layout.member(&fire.member_id).zip(d.members.iter().find(|m| m.id == fire.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Load, id, "fireScenarios");
    let f = &pose.frame;
    let at = along(&along(&f.origin, &f.x, pose.length / 2.0), &f.z, -section_depth(section) / 2.0 - 0.2);
    vec![
        handle(Spec { entity: &entity, tag: "duration", kind: HandleKind::DimensionLine, anchor: along(&at, &f.x, 0.3), grip: GripStyle::Arrow, writes: vec![binding(format!("fireScenarios[id={id}].durationS"), "s", round(60.0))], snap: grid(60.0), limits: (0.0, 7200.0), name: loc("Fire duration", "Branddauer") }),
        handle(Spec { entity: &entity, tag: "start-temperature", kind: HandleKind::FaceOffset { normal: f.z.clone() }, anchor: along(&at, &f.z, -0.2), grip: GripStyle::Diamond, writes: vec![binding(format!("fireScenarios[id={id}].thetaA"), "°C", round(5.0))], snap: grid(5.0), limits: (20.0, 550.0), name: loc("Member start temperature θ_a", "Starttemperatur des Bauteils θ_a") }),
    ]
}

fn fatigue_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(fat) = d.fatigue_details.iter().find(|f| f.id == id) else { return Vec::new() };
    let Some((pose, section)) = layout.member(&fat.member_id).zip(d.members.iter().find(|m| m.id == fat.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Group, id, "fatigueDetails");
    let f = &pose.frame;
    let top = along(&along(&f.origin, &f.x, pose.length * 0.75), &f.z, section_depth(section) / 2.0);
    let path = |leaf: &str| format!("fatigueDetails[id={id}].{leaf}");
    vec![
        handle(Spec { entity: &entity, tag: "delta-sigma", kind: HandleKind::FaceOffset { normal: f.z.clone() }, anchor: along(&top, &f.z, 0.3), grip: GripStyle::Diamond, writes: vec![binding(path("deltaSigmaEd"), "Pa", round(1.0e5))], snap: grid(1.0e5), limits: (1.0e6, 3.0e8), name: loc("Stress range Δσ_Ed", "Spannungsschwingbreite Δσ_Ed") }),
        handle(Spec { entity: &entity, tag: "cycles", kind: HandleKind::DimensionLine, anchor: along(&top, &f.x, 0.3), grip: GripStyle::Arrow, writes: vec![binding(path("nCycles"), "1", round(1.0e4))], snap: grid(1.0e4), limits: (1.0e3, 1.0e9), name: loc("Number of cycles n", "Lastspielzahl n") }),
    ]
}

/// 🧭️ Where a patch load stands: the host's origin, axes, length and depth (member: pose and section depth; sheet: origin on the +X span and the rib height).
pub struct PatchPose {
    pub origin: Vec3,
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
    pub length: f64,
    pub depth: f64,
}

pub fn patch_pose(d: &En1999Snapshot, layout: &Layout, load: &super::model::PatchLoad) -> Option<PatchPose> {
    if let Some(member) = d.members.iter().find(|m| m.id == load.subject_id) {
        let (pose, section) = layout.member(&member.id).zip(d.sections.iter().find(|s| s.id == member.section_id))?;
        return Some(PatchPose { origin: pose.frame.origin.clone(), x: pose.frame.x.clone(), y: pose.frame.y.clone(), z: pose.frame.z.clone(), length: pose.length, depth: section_depth(section) });
    }
    let sheet = d.cold_formed.iter().find(|s| s.id == load.subject_id)?;
    Some(PatchPose { origin: layout.origin(&sheet.id), x: Vec3::X, y: Vec3::Y, z: Vec3::Z, length: sheet.span, depth: 0.2 * sheet.width })
}

fn site_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(cat) = d.fatigue_categories.iter().find(|c| c.id == id) else { return Vec::new() };
    let Some((pose, section)) = layout.member(&cat.member_id).zip(d.members.iter().find(|m| m.id == cat.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Solid, id, "fatigueCategories");
    let f = &pose.frame;
    let at = along(&along(&f.origin, &f.x, cat.location_x.clamp(0.0, pose.length)), &f.z, section_depth(section) / 2.0);
    vec![handle(Spec { entity: &entity, tag: "location", kind: HandleKind::ScalarAlong { axis: f.x.clone() }, anchor: at, grip: GripStyle::Arrow, writes: vec![binding(format!("fatigueCategories[id={id}].locationX"), "m", round(0.01))], snap: grid(0.01), limits: (0.0, pose.length), name: loc("Fatigue site location x", "Lage x der Ermüdungsstelle") })]
}

fn patch_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(load) = d.patch_loads.iter().find(|l| l.id == id) else { return Vec::new() };
    let Some(pose) = patch_pose(d, layout, load) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Load, id, "patchLoads");
    let top = along(&along(&pose.origin, &pose.x, load.x.clamp(0.0, pose.length)), &pose.z, pose.depth / 2.0);
    let path = |leaf: &str| format!("patchLoads[id={id}].{leaf}");
    let web_height = if load.web_height > 0.0 { load.web_height } else { d.members.iter().find(|m| m.id == load.subject_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id)).map_or(pose.depth, |s| s.height - 2.0 * s.flange_thickness) };
    let mut out = vec![
        handle(Spec { entity: &entity, tag: "position", kind: HandleKind::ScalarAlong { axis: pose.x.clone() }, anchor: top.clone(), grip: GripStyle::Arrow, writes: vec![binding(path("x"), "m", round(0.01))], snap: grid(0.01), limits: (0.0, pose.length), name: loc("Patch load position x", "Lage x der Einzellast") }),
        handle(Spec { entity: &entity, tag: "bearing", kind: HandleKind::DimensionLine, anchor: along(&top, &pose.x, load.bearing_length), grip: GripStyle::Ring, writes: vec![binding(path("bearingLength"), "m", round(0.005))], snap: grid(0.005), limits: (0.005, 0.5), name: loc("Stiff bearing length s_s", "Lasteinleitungslänge s_s (steife Auflagerung)") }),
        handle(Spec { entity: &entity, tag: "force", kind: HandleKind::FaceOffset { normal: pose.z.clone() }, anchor: along(&top, &pose.z, 0.3), grip: GripStyle::Diamond, writes: vec![binding(path("load"), "N", round(100.0))], snap: grid(100.0), limits: (0.0, 1.0e7), name: loc("Transverse force F_Ed", "Querkraft F_Ed") }),
        choice_handle(&entity, "kind", along(&top, &pose.x, -0.1), path("position"), &[("end".to_owned(), "End (near support)", "Ende (nahe Auflager)"), ("interior".to_owned(), "Interior", "Innen")], loc("Load position (end or interior)", "Lastlage (Ende oder innen)")),
    ];
    if d.members.iter().any(|m| m.id == load.subject_id) {
        out.push(handle(Spec { entity: &entity, tag: "web-height", kind: HandleKind::DimensionLine, anchor: along(&top, &pose.z, -web_height), grip: GripStyle::Ring, writes: vec![binding(path("webHeight"), "m", round(0.005))], snap: grid(0.005), limits: (0.01, 2.0), name: loc("Web height h_w", "Steghöhe h_w") }));
    }
    out
}

fn ring_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(ring) = d.ring_stiffeners.iter().find(|r| r.id == id) else { return Vec::new() };
    let Some(shell) = d.shells.iter().find(|s| s.id == ring.shell_id) else { return Vec::new() };
    let o = layout.origin(&shell.id);
    let entity = entity_ref(EntityKind::Shell, id, "ringStiffeners");
    let rim = v3(o.x + shell.radius, o.y, o.z + ring.x);
    let path = |leaf: &str| format!("ringStiffeners[id={id}].{leaf}");
    vec![
        handle(Spec { entity: &entity, tag: "position", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: rim.clone(), grip: GripStyle::Arrow, writes: vec![binding(path("x"), "m", round(0.01))], snap: grid(0.01), limits: (0.0, shell.length), name: loc("Ring position x", "Lage x der Ringsteife") }),
        handle(Spec { entity: &entity, tag: "depth", kind: HandleKind::DimensionLine, anchor: along(&rim, &Vec3::X, ring.depth), grip: GripStyle::Ring, writes: vec![binding(path("depth"), "m", round(0.005))], snap: grid(0.005), limits: (0.01, 0.5), name: loc("Ring depth", "Ringhöhe") }),
    ]
}

/// 🦴️ Handles of a transverse web stiffener: position along the girder, outstand width and thickness, each written alone (one commit per drag) to `transverseStiffeners[id=..]` through the kinds `change-transverse-position`, `-width` and `-thickness`.
fn web_stiffener_handles(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(st) = d.transverse_stiffeners.iter().find(|s| s.id == id) else { return Vec::new() };
    let Some(member) = d.members.iter().find(|m| m.id == st.girder_id) else { return Vec::new() };
    let (Some(pose), Some(section)) = (layout.member(&member.id), d.sections.iter().find(|s| s.id == member.section_id)) else { return Vec::new() };
    let [h, _, tf, tw] = dims(section);
    let h_w = h - 2.0 * tf;
    if h_w <= 1e-6 { return Vec::new() }
    let f = &pose.frame;
    let entity = entity_ref(EntityKind::Member, id, "transverseStiffeners");
    let at = |x: f64, y: f64, z: f64| along(&along(&along(&f.origin, &f.x, x), &f.y, y), &f.z, z);
    let path = |leaf: &str| format!("transverseStiffeners[id={id}].{leaf}");
    vec![
        handle(Spec { entity: &entity, tag: "position", kind: HandleKind::ScalarAlong { axis: f.x.clone() }, anchor: at(st.position, 0.5 * tw + 0.5 * st.width, 0.5 * h_w), grip: GripStyle::Arrow, writes: vec![binding(path("position"), "m", round(0.01))], snap: grid(0.01), limits: (0.0, member.length), name: loc("Stiffener position along the girder", "Lage der Quersteife entlang des Trägers") }),
        handle(Spec { entity: &entity, tag: "width", kind: HandleKind::DimensionLine, anchor: at(st.position, 0.5 * tw + st.width, 0.0), grip: GripStyle::Ring, writes: vec![binding(path("width"), "m", round(0.005))], snap: grid(0.005), limits: (0.01, 0.5), name: loc("Stiffener width b_st", "Steifenbreite b_st") }),
        handle(Spec { entity: &entity, tag: "thickness", kind: HandleKind::FaceOffset { normal: f.x.clone() }, anchor: at(st.position + 0.5 * st.thickness, 0.5 * tw + 0.5 * st.width, -0.5 * h_w), grip: GripStyle::Diamond, writes: vec![binding(path("thickness"), "m", round(0.0005))], snap: grid(0.0005), limits: (0.002, 0.05), name: loc("Stiffener thickness t_st", "Steifendicke t_st") }),
    ]
}

/// 🦴️ Handles of an edge stiffener (lip) entity `<member>:edge-<element>`: lip depth and thickness, written to the nested element row `sections[id=S].elements[id=E]` through the parent-selector kinds `change-element-stiffener-depth` and `change-element-stiffener-thk`.
fn lip_handles(d: &En1999Snapshot, layout: &Layout, raw: &str) -> Vec<Handle> {
    let Some((member_id, element_id)) = raw.split_once(LIP_MARK) else { return Vec::new() };
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == member_id), layout.member(member_id)) else { return Vec::new() };
    let Some(section) = d.sections.iter().find(|s| s.id == member.section_id) else { return Vec::new() };
    let Some((index, el)) = section.elements.iter().enumerate().find(|(_, e)| e.id == element_id) else { return Vec::new() };
    if !el.outstand || el.edge_stiffener_depth <= 0.0 || el.edge_stiffener_thickness <= 0.0 { return Vec::new() }
    let Some((y, z, false)) = weld_anchor(section, index) else { return Vec::new() };
    let f = &pose.frame;
    let entity = entity_ref(EntityKind::Member, raw, "members");
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let (tip, ts, depth) = (y - el.weld_position + el.width, el.edge_stiffener_thickness, el.edge_stiffener_depth);
    let at = |lateral: f64, vertical: f64| along(&along(&along(&f.origin, &f.x, member.length / 2.0), &f.y, lateral), &f.z, vertical);
    let down = v3(sign * f.z.x, sign * f.z.y, sign * f.z.z);
    let path = |leaf: &str| format!("sections[id={}].elements[id={}].{leaf}", section.id, el.id);
    vec![
        handle(Spec { entity: &entity, tag: "depth", kind: HandleKind::FaceOffset { normal: down }, anchor: at(tip - 0.5 * ts, z + sign * (0.5 * el.thickness + depth)), grip: GripStyle::Diamond, writes: vec![binding(path("edgeStiffenerDepth"), "m", round(0.0005))], snap: grid(0.0025), limits: (0.002, 0.15), name: loc(&format!("Lip depth of {} on {}", el.id, member.id), &format!("Randsteifentiefe von {} an {}", el.id, member.id)) }),
        handle(Spec { entity: &entity, tag: "thickness", kind: HandleKind::FaceOffset { normal: f.y.clone() }, anchor: at(tip, z + sign * (0.5 * el.thickness + 0.5 * depth)), grip: GripStyle::Diamond, writes: vec![binding(path("edgeStiffenerThickness"), "m", round(0.0005))], snap: grid(0.0005), limits: (0.0005, 0.02), name: loc(&format!("Lip thickness of {} on {}", el.id, member.id), &format!("Randsteifendicke von {} an {}", el.id, member.id)) }),
    ]
}

/// 🏹 The magnitude grip of one load glyph `<owner>:load-<action>:<tag>` of a member, connection or cold-formed sheet.
fn action_handles(d: &En1999Snapshot, layout: &Layout, raw: &str) -> Vec<Handle> {
    let Some((owner, _)) = raw.split_once(super::members::ACTION_MARK) else { return Vec::new() };
    let section_of = |member: &str| d.members.iter().find(|m| m.id == member).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id));
    let grips = if let Some((member, pose)) = d.members.iter().find(|m| m.id == owner).zip(layout.member(owner)) {
        let depth = section_of(owner).map_or(0.2, |s| section_extent(d, s).0);
        super::members::action_grips(owner, "members", &member.actions, &pose.frame, member.length, depth)
    } else if let Some((conn, pose)) = d.connections.iter().find(|c| c.id == owner).and_then(|c| layout.member(&c.member_id).map(|p| (c, p))) {
        let depth = section_of(&conn.member_id).map_or(0.2, section_depth);
        super::members::action_grips(owner, "connections", &conn.actions, &pose.frame, pose.length, depth)
    } else if let Some(sheet) = d.cold_formed.iter().find(|s| s.id == owner) {
        let frame = super::kit::member_frame(&layout.origin(owner), &Vec3::X);
        super::members::action_grips(owner, "coldFormed", &sheet.actions, &frame, sheet.span, 0.2 * sheet.width)
    } else {
        return Vec::new();
    };
    grips.into_iter().filter(|h| h.id.starts_with(&format!("{raw}:"))).collect()
}

/// 🕹️ The handles of one document entity, anchored in the given layout.
pub fn build(document: &En1999Snapshot, layout: &Layout, raw: &str) -> Vec<Handle> {
    if raw.contains(super::members::ACTION_MARK) { return action_handles(document, layout, raw); }
    if raw.contains(LIP_MARK) { return lip_handles(document, layout, raw); }
    if raw.ends_with(super::web_stiffeners::MIRROR_MARK) { return Vec::new(); }
    let id = base_id(raw);
    let mut out = [member_handles(document, layout, id), vertex_handles(document, layout, id), weld_run_handles(document, layout, id), connection_handles(document, layout, id), sheet_handles(document, layout, id), shell_handles(document, layout, id), fire_handles(document, layout, id), fatigue_handles(document, layout, id), site_handles(document, layout, id), patch_handles(document, layout, id), ring_handles(document, layout, id), web_stiffener_handles(document, layout, id)].concat();
    let taken: std::collections::BTreeSet<String> = out.iter().flat_map(|h| h.writes.iter().map(|w| w.path.clone())).collect();
    out.extend(gauges::of(document, layout, id).into_iter().filter(|h| h.writes.iter().all(|w| !taken.contains(&w.path))));
    out
}

/// 🆔️ The handle ids a scene entity advertises.
pub fn ids(document: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🕹️ The handles of every selected entity (derived scene ids resolve to their document entity), in selection order, without duplicates.
pub fn handles(document: &En1999Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        for handle in build(document, &layout, &entity.id) {
            if !out.iter().any(|h| h.id == handle.id) {
                out.push(handle);
            }
        }
    }
    out
}
