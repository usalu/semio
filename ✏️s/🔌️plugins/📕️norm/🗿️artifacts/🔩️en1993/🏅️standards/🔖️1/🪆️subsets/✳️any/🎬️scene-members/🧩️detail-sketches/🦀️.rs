//! 🧩️ Detail sketches derived from existing rows: the effective-width zones of a plated panel (EN 1993-1-5 4.4, uniform compression), the Whitmore spread of a bolted gusset plate (30° through the bolt rows) and the eaves haunch of a moment joint (`HAUNCH_SPAN_FRACTION` of the beam span, as deep as the beam), each in 3D and as a diagram with its table twin.
//!
//! 📏️ The haunch proportion and the 30° spread are drawing rules, not EN 1993 provisions: no haunch or gusset geometry is stored in the snapshot, so the sketches are labelled as derived.

use super::checks::clauses::effective_width as ew;
use super::kit::*;
use super::layout::Layout;
use super::model::{PlatedPanel, SteelJoint};
use super::norm::CheckReport;
use super::scene::projection_kit::diagram_parts::{axis, column, stroke};
use super::scene::*;
use super::SceneInput;

pub const HAUNCH_SPAN_FRACTION: f64 = 0.1;
pub const WHITMORE_ANGLE: f64 = std::f64::consts::FRAC_PI_6;
const OVERLAY_FRACTION: f64 = 0.3;

/// 🪟️ Effective width of a panel in uniform compression, in metres: `(ρ, b_eff, b_e1 = b_e2)`.
pub fn panel_effective_width(p: &PlatedPanel) -> Option<(f64, f64, f64)> {
    (p.b > 1e-6 && p.thickness > 1e-6 && p.fy > 1e-6).then(|| {
        let e = ew::effective_internal(p.b * 1e3, p.thickness * 1e3, p.fy / 1e6, 1.0);
        (e.rho, e.b_eff / 1e3, e.b_e1 / 1e3)
    })
}

/// 🔺️ The Whitmore width at the last bolt row: `gauge (n−1) + 2 pitch (rows−1) tan 30°`, with the width at the first row.
pub fn whitmore(j: &SteelJoint) -> Option<(f64, f64)> {
    let (rows, per_row) = (j.bolt_rows.max(1), j.bolts_per_row.max(1));
    (j.kind != semio_s_artifact_norm_en1993_model::JointKind::Welded && rows >= 2 && j.pitch > 1e-6).then(|| {
        let first = j.gauge * (per_row - 1) as f64;
        (first, first + 2.0 * j.pitch * (rows - 1) as f64 * WHITMORE_ANGLE.tan())
    })
}

/// 📐️ Haunch length and depth below the beam: `(HAUNCH_SPAN_FRACTION · span, beam depth)`.
pub fn haunch(j: &SteelJoint, beam_depth: f64) -> Option<(f64, f64)> {
    (j.beam_span > 1e-6 && beam_depth > 1e-6).then(|| (HAUNCH_SPAN_FRACTION * j.beam_span, beam_depth))
}

fn polygon(points: &[(f64, f64)]) -> Polygon2 {
    Polygon2::solid(points.iter().map(|(x, y)| Vec2::new(*x, *y)).collect())
}

struct Overlay {
    id: String,
    collection: &'static str,
    layer: &'static str,
    frame: Frame,
    outline: Polygon2,
    thickness: f64,
    binding: Option<UtilisationBinding>,
    name: LocalizedCopy,
    detail: LocalizedCopy,
}

impl Overlay {
    fn push(self, out: &mut Out, points: &[Vec3]) {
        let geometry = Geometry::Plate(PlateGeometry { frame: self.frame, outline: self.outline, thickness: self.thickness, offset: 0.0, stiffeners: Vec::new() });
        out.push(Spec { kind: EntityKind::Plate, id: self.id, collection: self.collection, geometry, role: MaterialRole::Steel, binding: self.binding, name: self.name, detail: self.detail, layer: self.layer, handles: Vec::new(), deformation: None }, points);
    }
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for panel in &d.plated_panels {
        if !out.own("platedPanels", &panel.id) {
            continue;
        }
        let Some((_, b_eff, _)) = panel_effective_width(panel) else { continue };
        let lost = panel.b - b_eff;
        if lost <= 1e-6 || panel.a <= 1e-6 {
            continue;
        }
        let o = layout.origin(&panel.id);
        let centre = along(&v3(o.x + panel.a / 2.0, o.y + panel.b / 2.0, o.z), &Vec3::Z, panel.thickness * (0.5 + OVERLAY_FRACTION / 2.0));
        Overlay {
            id: format!("{}:ineffective", panel.id),
            collection: "platedPanels",
            layer: LAYER_PLATES,
            frame: Frame { origin: centre, x: Vec3::X, y: Vec3::Y, z: Vec3::Z },
            outline: polygon(&[(-panel.a / 2.0, -lost / 2.0), (panel.a / 2.0, -lost / 2.0), (panel.a / 2.0, lost / 2.0), (-panel.a / 2.0, lost / 2.0)]),
            thickness: panel.thickness * OVERLAY_FRACTION,
            binding: binding(input.report, &[panel.id.as_str()]),
            name: loc(&format!("Ineffective zone of {}", panel.id), &format!("Nicht mittragende Zone von {}", panel.id)),
            detail: loc(&format!("b − b_eff = {:.0} mm of {:.0} mm", lost * 1e3, panel.b * 1e3), &format!("b − b_eff = {:.0} mm von {:.0} mm", lost * 1e3, panel.b * 1e3)),
        }
        .push(out, &[o.clone(), v3(o.x + panel.a, o.y + panel.b, o.z)]);
    }
    for cf in d.cold_formed_members.iter().filter(|c| c.support_length > 1e-6 && c.b_bar > 1e-6 && c.thickness > 1e-6) {
        if !out.own("coldFormedMembers", &cf.id) {
            continue;
        }
        let o = layout.origin(&cf.id);
        let cf_binding = binding(input.report, &[cf.id.as_str()]);
        let (la, flange) = (cf.support_length, 0.6 * cf.b_bar);
        let patch = along(&along(&o, &Vec3::X, la / 2.0), &Vec3::Z, -(cf.b_bar / 2.0 + cf.thickness * (0.5 + OVERLAY_FRACTION / 2.0)));
        Overlay {
            id: format!("{}:bearing", cf.id),
            collection: "coldFormedMembers",
            layer: LAYER_PLATES,
            frame: Frame { origin: patch, x: Vec3::X, y: Vec3::Y, z: Vec3::Z },
            outline: polygon(&[(-la / 2.0, -flange / 2.0), (la / 2.0, -flange / 2.0), (la / 2.0, flange / 2.0), (-la / 2.0, flange / 2.0)]),
            thickness: cf.thickness * OVERLAY_FRACTION,
            binding: cf_binding.clone(),
            name: loc(&format!("Bearing patch of {}", cf.id), &format!("Lasteinleitungsfläche von {}", cf.id)),
            detail: loc(&format!("l_a = {:.0} mm, F_Ed = {:.1} kN", la * 1e3, cf.support_reaction / 1e3), &format!("l_a = {:.0} mm, F_Ed = {:.1} kN", la * 1e3, cf.support_reaction / 1e3)),
        }
        .push(out, &[o.clone()]);
        let zone = along(&o, &Vec3::X, la / 2.0);
        Overlay {
            id: format!("{}:crippling", cf.id),
            collection: "coldFormedMembers",
            layer: LAYER_PLATES,
            frame: Frame { origin: zone, x: Vec3::X, y: Vec3::Z, z: mul(&Vec3::Y, -1.0) },
            outline: polygon(&[(-la / 2.0, -cf.b_bar / 2.0), (la / 2.0, -cf.b_bar / 2.0), (la / 2.0, cf.b_bar / 2.0), (-la / 2.0, cf.b_bar / 2.0)]),
            thickness: cf.thickness * (1.0 + OVERLAY_FRACTION),
            binding: cf_binding,
            name: loc(&format!("Web crippling zone of {}", cf.id), &format!("Stegkrüppelzone von {}", cf.id)),
            detail: loc(&format!("web h = {:.0} mm under l_a = {:.0} mm", cf.b_bar * 1e3, la * 1e3), &format!("Steg h = {:.0} mm unter l_a = {:.0} mm", cf.b_bar * 1e3, la * 1e3)),
        }
        .push(out, &[o.clone()]);
    }
    for joint in &d.joints {
        if !out.own("joints", &joint.id) {
            continue;
        }
        let (Some(pose), Some(member)) = (layout.member(&joint.member_id), d.members.iter().find(|m| m.id == joint.member_id)) else { continue };
        let section = d.sections.iter().find(|s| s.id == member.section_id);
        let f = &pose.frame;
        let end = along(&f.origin, &f.x, pose.length);
        let joint_binding = binding(input.report, &[joint.id.as_str()]);
        let thickness = joint.plate_thickness.max(0.004);
        if let Some((first, last)) = whitmore(joint) {
            let top = joint.pitch * (joint.bolt_rows.max(1) - 1) as f64 / 2.0;
            let bottom = -top;
            let plate = Frame { origin: along(&end, &f.x, thickness + thickness * OVERLAY_FRACTION / 2.0), x: f.y.clone(), y: f.z.clone(), z: f.x.clone() };
            Overlay {
                id: format!("{}:gusset", joint.id),
                collection: "joints",
                layer: LAYER_JOINTS,
                frame: plate,
                outline: polygon(&[(-first / 2.0, top), (first / 2.0, top), (last / 2.0, bottom), (-last / 2.0, bottom)]),
                thickness: thickness * OVERLAY_FRACTION,
                binding: joint_binding.clone(),
                name: loc(&format!("Whitmore zone of {}", joint.id), &format!("Whitmore-Zone von {}", joint.id)),
                detail: loc(&format!("30° spread: {:.0} mm at the first row, {:.0} mm at the last", first * 1e3, last * 1e3), &format!("30°-Ausbreitung: {:.0} mm in der ersten, {:.0} mm in der letzten Reihe", first * 1e3, last * 1e3)),
            }
            .push(out, &[end.clone()]);
        }
        let Some(section) = section else { continue };
        if let Some((length, depth)) = haunch(joint, section.h) {
            let origin = along(&end, &f.z, -section.h / 2.0);
            let points = [along(&origin, &f.x, -length), along(&origin, &f.z, -depth)];
            Overlay {
                id: format!("{}:haunch", joint.id),
                collection: "joints",
                layer: LAYER_JOINTS,
                frame: Frame { origin, x: mul(&f.x, -1.0), y: mul(&f.z, -1.0), z: mul(&f.y, -1.0) },
                outline: polygon(&[(0.0, 0.0), (length, 0.0), (0.0, depth)]),
                thickness: section.tw.max(0.006),
                binding: joint_binding,
                name: loc(&format!("Haunch of {}", joint.id), &format!("Voute von {}", joint.id)),
                detail: loc(&format!("{:.0} mm long, {:.0} mm deep at the column face", length * 1e3, depth * 1e3), &format!("{:.0} mm lang, {:.0} mm hoch am Stützenanschnitt", length * 1e3, depth * 1e3)),
            }
            .push(out, &points);
        }
    }
}

fn xy(points: &[(f64, f64)]) -> Vec<Vec2> {
    points.iter().map(|(x, y)| Vec2::new(*x, *y)).collect()
}

fn closed(points: &[(f64, f64)]) -> Vec<Vec2> {
    let mut ring = xy(points);
    ring.extend(ring.first().cloned());
    ring
}

fn worst_of(report: &CheckReport, entity: &str) -> f64 {
    report.checks.iter().filter(|c| c.subject.entity_id == entity && c.utilization.is_finite()).map(|c| c.utilization).fold(0.0, f64::max)
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn zone(id: &str, label: LocalizedCopy, ring: &[(f64, f64)], tone: Tone, opacity: f64, hatch: Option<Hatch>) -> Series {
    Series::Polygon { id: id.into(), label, ring: xy(ring), fill: Fill { tone, opacity }, hatch }
}

fn effective_width_sketch(report: &CheckReport, p: &PlatedPanel) -> Option<Diagram2d> {
    let (rho, b_eff, edge) = panel_effective_width(p)?;
    let (a, b, lost) = (p.a.max(p.b), p.b, p.b - b_eff);
    let ratio = worst_of(report, &p.id);
    let half = b / 2.0;
    let series = vec![
        Series::Polyline { id: "gross".into(), label: loc("Gross panel", "Brutto-Beulfeld"), points: closed(&[(0.0, 0.0), (a, 0.0), (a, b), (0.0, b)]), style: stroke(Tone::Neutral, 1.5, Dash::Solid) },
        zone("effective-1", loc("Effective zone b_e1", "Mittragende Zone b_e1"), &[(0.0, 0.0), (a, 0.0), (a, edge), (0.0, edge)], Tone::Info, 0.35, None),
        zone("effective-2", loc("Effective zone b_e2", "Mittragende Zone b_e2"), &[(0.0, b - edge), (a, b - edge), (a, b), (0.0, b)], Tone::Info, 0.35, None),
        zone("ineffective", loc("Ineffective zone", "Nicht mittragende Zone"), &[(0.0, half - lost / 2.0), (a, half - lost / 2.0), (a, half + lost / 2.0), (0.0, half + lost / 2.0)], Tone::Warning, 0.15, Some(Hatch::Diagonal)),
    ];
    Some(Diagram2d {
        id: format!("effective-width:{}", p.id),
        title: loc(&format!("Effective width sketch – {}", p.id), &format!("Skizze der mittragenden Breite – {}", p.id)),
        description: loc(&format!("Plan of the panel in uniform compression (ψ = 1): ρ = {rho:.3}, b_eff = {:.0} mm of b = {:.0} mm, two effective zones of {:.0} mm at the edges (EN 1993-1-5 4.4, Table 4.1).", b_eff * 1e3, b * 1e3, edge * 1e3), &format!("Draufsicht des Beulfelds unter gleichmäßigem Druck (ψ = 1): ρ = {rho:.3}, b_eff = {:.0} mm von b = {:.0} mm, zwei mittragende Zonen von {:.0} mm an den Rändern (EN 1993-1-5 4.4, Tabelle 4.1).", b_eff * 1e3, b * 1e3, edge * 1e3)),
        axes: vec![axis("Panel length a", "Feldlänge a", QuantityKind::Length, "m", AxisScale::Linear, Some((-0.1 * a, 1.1 * a))), axis("Panel width b", "Feldbreite b", QuantityKind::Length, "m", AxisScale::Linear, Some((-0.1 * b, 1.1 * b)))],
        series,
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "b-eff".into(), at: Vec2::new(a / 2.0, half), text: loc(&format!("ρ = {rho:.3}, b_eff = {:.0} mm", b_eff * 1e3), &format!("ρ = {rho:.3}, b_eff = {:.0} mm", b_eff * 1e3)), tone: tone(ratio) }],
        focus: Some(EntityRef::new(EntityKind::Plate, &p.id, format!("platedPanels[id={}]", p.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Width b", "Breite b", "mm", vec![b * 1e3]), column("Thickness t", "Dicke t", "mm", vec![p.thickness * 1e3]), column("Reduction ρ", "Abminderung ρ", "1", vec![rho]), column("Effective width b_eff", "Mittragende Breite b_eff", "mm", vec![b_eff * 1e3]), column("Edge zone b_e1 = b_e2", "Randzone b_e1 = b_e2", "mm", vec![edge * 1e3]), column("Ineffective width", "Nicht mittragende Breite", "mm", vec![lost * 1e3])] },
    })
}

fn gusset_sketch(report: &CheckReport, j: &SteelJoint) -> Option<Diagram2d> {
    let (first, last) = whitmore(j)?;
    let (rows, per_row) = (j.bolt_rows.max(1), j.bolts_per_row.max(1));
    let top = j.pitch * (rows - 1) as f64 / 2.0;
    let across = (j.gauge * (per_row - 1) as f64 + 2.0 * j.edge_distance).max(last);
    let down = j.pitch * (rows - 1) as f64 + 2.0 * j.end_distance;
    let ratio = worst_of(report, &j.id);
    let bolts: Vec<(f64, f64)> = (0..rows).flat_map(|r| (0..per_row).map(move |c| (j.gauge * (c as f64 - (per_row - 1) as f64 / 2.0), top - j.pitch * r as f64))).collect();
    let extent = across.max(down) * 0.65;
    Some(Diagram2d {
        id: format!("gusset:{}", j.id),
        title: loc(&format!("Gusset plate and Whitmore zone – {}", j.id), &format!("Knotenblech und Whitmore-Zone – {}", j.id)),
        description: loc(&format!("Plate outline, bolt grid and the 30° spread from the first bolt row: {:.0} mm wide at the first, {:.0} mm at the last row (drawing rule derived from the bolt grid, not an EN 1993 provision).", first * 1e3, last * 1e3), &format!("Blechumriss, Schraubenbild und 30°-Ausbreitung ab der ersten Schraubenreihe: {:.0} mm breit in der ersten, {:.0} mm in der letzten Reihe (Zeichenregel aus dem Schraubenbild, keine Regel der EN 1993).", first * 1e3, last * 1e3)),
        axes: vec![axis("Across y", "Quer y", QuantityKind::Length, "m", AxisScale::Linear, Some((-extent, extent))), axis("Along z", "Längs z", QuantityKind::Length, "m", AxisScale::Linear, Some((-extent, extent)))],
        series: vec![
            Series::Polyline { id: "plate".into(), label: loc("Plate", "Blech"), points: closed(&[(-across / 2.0, -down / 2.0), (across / 2.0, -down / 2.0), (across / 2.0, down / 2.0), (-across / 2.0, down / 2.0)]), style: stroke(Tone::Neutral, 1.5, Dash::Solid) },
            zone("whitmore", loc("Whitmore zone", "Whitmore-Zone"), &[(-first / 2.0, top), (first / 2.0, top), (last / 2.0, -top), (-last / 2.0, -top)], Tone::Info, 0.25, Some(Hatch::Diagonal)),
            Series::Scatter { id: "bolts".into(), label: loc("Bolts", "Schrauben"), points: bolts.iter().map(|(x, y)| ScatterPoint { at: Vec2::new(*x, *y), tone: Tone::Neutral, label: None }).collect() },
        ],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "width".into(), at: Vec2::new(0.0, -top - 0.6 * j.end_distance), text: loc(&format!("b_W = {:.0} mm", last * 1e3), &format!("b_W = {:.0} mm", last * 1e3)), tone: tone(ratio) }],
        focus: Some(EntityRef::new(EntityKind::Group, &j.id, format!("joints[id={}]", j.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Bolt y", "Schraube y", "mm", bolts.iter().map(|b| b.0 * 1e3).collect()), column("Bolt z", "Schraube z", "mm", bolts.iter().map(|b| b.1 * 1e3).collect())] },
    })
}

fn haunch_sketch(report: &CheckReport, j: &SteelJoint, beam_depth: f64) -> Option<Diagram2d> {
    let (length, depth) = haunch(j, beam_depth)?;
    let ratio = worst_of(report, &j.id);
    let run = 2.0 * length;
    let extent = run.max(beam_depth + depth);
    Some(Diagram2d {
        id: format!("haunch:{}", j.id),
        title: loc(&format!("Eaves haunch – {}", j.id), &format!("Traufvoute – {}", j.id)),
        description: loc(&format!("Beam and haunch at the column face: {:.0} mm long ({:.0} % of the {:.1} m span) and {:.0} mm deep (drawing rule, no haunch geometry is stored).", length * 1e3, HAUNCH_SPAN_FRACTION * 100.0, j.beam_span, depth * 1e3), &format!("Riegel und Voute am Stützenanschnitt: {:.0} mm lang ({:.0} % der Spannweite {:.1} m) und {:.0} mm hoch (Zeichenregel, keine Voutengeometrie gespeichert).", length * 1e3, HAUNCH_SPAN_FRACTION * 100.0, j.beam_span, depth * 1e3)),
        axes: vec![axis("Along the beam", "Entlang des Riegels", QuantityKind::Length, "m", AxisScale::Linear, Some((-run, 0.1 * run))), axis("Height", "Höhe", QuantityKind::Length, "m", AxisScale::Linear, Some((-extent, 0.5 * extent)))],
        series: vec![
            zone("beam", loc("Beam", "Riegel"), &[(-run, 0.0), (0.0, 0.0), (0.0, -beam_depth), (-run, -beam_depth)], Tone::Neutral, 0.2, None),
            zone("haunch", loc("Haunch", "Voute"), &[(-length, -beam_depth), (0.0, -beam_depth), (0.0, -beam_depth - depth)], Tone::Info, 0.3, Some(Hatch::Diagonal)),
        ],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "size".into(), at: Vec2::new(-length / 2.0, -beam_depth - depth / 3.0), text: loc(&format!("{:.0} × {:.0} mm", length * 1e3, depth * 1e3), &format!("{:.0} × {:.0} mm", length * 1e3, depth * 1e3)), tone: tone(ratio) }],
        focus: Some(EntityRef::new(EntityKind::Group, &j.id, format!("joints[id={}]", j.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Span", "Spannweite", "m", vec![j.beam_span]), column("Haunch length", "Voutenlänge", "mm", vec![length * 1e3]), column("Haunch depth", "Vouteneinhöhe", "mm", vec![depth * 1e3]), column("Depth at the column face", "Höhe am Stützenanschnitt", "mm", vec![(beam_depth + depth) * 1e3])] },
    })
}

/// 🧊 The block of a bolted plate that tears out along the outer bolt lines (EN 1993-1-8 3.10.2): two shear faces, the tension face at the free end and the bolt grid.
fn block_tearing_sketch(report: &CheckReport, j: &SteelJoint) -> Option<Diagram2d> {
    let (rows, per_row) = (j.bolt_rows.max(1), j.bolts_per_row.max(1));
    if j.kind != semio_s_artifact_norm_en1993_model::JointKind::Bolted || j.pitch <= 1e-6 || j.end_distance <= 1e-6 {
        return None;
    }
    let width = j.gauge * (per_row - 1) as f64;
    let length = j.pitch * (rows - 1) as f64 + j.end_distance;
    let (plate_w, plate_l) = (width + 2.0 * j.edge_distance, length + j.end_distance);
    let bolts: Vec<(f64, f64)> = (0..rows).flat_map(|r| (0..per_row).map(move |c| (j.gauge * (c as f64 - (per_row - 1) as f64 / 2.0), -j.pitch * r as f64))).collect();
    let ratio = report.checks.iter().find(|c| c.id == format!("en1993.1-8.3.10.block.{}", j.id)).map_or(worst_of(report, &j.id), |c| c.utilization);
    let extent = plate_w.max(plate_l) * 0.65;
    let (half, bottom) = (width / 2.0, -length);
    Some(Diagram2d {
        id: format!("block-tearing:{}", j.id),
        title: loc(&format!("Block tearing – {}", j.id), &format!("Blockversagen – {}", j.id)),
        description: loc(
            &format!("Plate, bolt grid and the block around the outer bolt lines: two shear faces of {:.0} mm along the load and a tension face of {:.0} mm at the free end (EN 1993-1-8 3.10.2, concentric loading); utilisation {:.2}.", length * 1e3, width * 1e3, ratio),
            &format!("Blech, Schraubenbild und der Block um die äußeren Schraubenlinien: zwei Schubflächen von {:.0} mm in Lastrichtung und eine Zugfläche von {:.0} mm am freien Ende (EN 1993-1-8 3.10.2, zentrische Belastung); Ausnutzung {:.2}.", length * 1e3, width * 1e3, ratio),
        ),
        axes: vec![axis("Across y", "Quer y", QuantityKind::Length, "m", AxisScale::Linear, Some((-extent, extent))), axis("Along the load z", "In Lastrichtung z", QuantityKind::Length, "m", AxisScale::Linear, Some((-extent - plate_l / 2.0, extent - plate_l / 2.0)))],
        series: vec![
            Series::Polyline { id: "plate".into(), label: loc("Plate", "Blech"), points: closed(&[(-plate_w / 2.0, j.end_distance), (plate_w / 2.0, j.end_distance), (plate_w / 2.0, bottom - j.end_distance), (-plate_w / 2.0, bottom - j.end_distance)]), style: stroke(Tone::Neutral, 1.5, Dash::Solid) },
            zone("block", loc("Torn-out block", "Ausreißender Block"), &[(-half, 0.0), (half, 0.0), (half, bottom), (-half, bottom)], tone(ratio), 0.25, Some(Hatch::Diagonal)),
            Series::Polyline { id: "shear-1".into(), label: loc("Shear face A_nv", "Schubfläche A_nv"), points: xy(&[(-half, 0.0), (-half, bottom)]), style: stroke(Tone::Warning, 2.5, Dash::Solid) },
            Series::Polyline { id: "shear-2".into(), label: loc("Shear face A_nv", "Schubfläche A_nv"), points: xy(&[(half, 0.0), (half, bottom)]), style: stroke(Tone::Warning, 2.5, Dash::Solid) },
            Series::Polyline { id: "tension".into(), label: loc("Tension face A_nt", "Zugfläche A_nt"), points: xy(&[(-half, bottom), (half, bottom)]), style: stroke(Tone::Error, 2.5, Dash::Solid) },
            Series::Scatter { id: "bolts".into(), label: loc("Bolts", "Schrauben"), points: bolts.iter().map(|(x, y)| ScatterPoint { at: Vec2::new(*x, *y), tone: Tone::Neutral, label: None }).collect() },
        ],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "ratio".into(), at: Vec2::new(0.0, j.end_distance * 0.5), text: loc(&format!("V_Ed/V_eff,Rd = {ratio:.2}"), &format!("V_Ed/V_eff,Rd = {ratio:.2}")), tone: tone(ratio) }],
        focus: Some(EntityRef::new(EntityKind::Group, &j.id, format!("joints[id={}]", j.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Bolt y", "Schraube y", "mm", bolts.iter().map(|b| b.0 * 1e3).collect()), column("Bolt z", "Schraube z", "mm", bolts.iter().map(|b| b.1 * 1e3).collect())] },
    })
}

/// 🧩️ The sketch diagrams; `focus` limits them to one entity id (or the member of a joint).
pub fn diagrams(d: &super::model::En1993Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let wanted = |id: &str, related: &str| focus.is_none_or(|f| f == id || f == related);
    let mut out = Vec::new();
    for panel in d.plated_panels.iter().filter(|p| wanted(&p.id, &p.id)) {
        out.extend(effective_width_sketch(report, panel));
    }
    for joint in d.joints.iter().filter(|j| wanted(&j.id, &j.member_id)) {
        out.extend(gusset_sketch(report, joint));
        out.extend(block_tearing_sketch(report, joint));
        let depth = d.members.iter().find(|m| m.id == joint.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id)).map_or(0.0, |s| s.h);
        out.extend(haunch_sketch(report, joint, depth));
    }
    out
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
