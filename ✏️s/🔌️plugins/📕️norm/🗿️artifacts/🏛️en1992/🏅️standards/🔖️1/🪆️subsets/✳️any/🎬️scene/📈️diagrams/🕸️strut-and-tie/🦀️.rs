//! 🕸️ Strut-and-tie model of the deep beams (D-regions, EN 1992-1-1 6.5 and 5.6.4): the statically determinate truss of two struts and a tie between the supports and the load point, its forces from the catalogue truss solver of ST-04, drawn in 3D as struts of the strut width and a tie bar of the required area.

use super::checks;
use super::kit::*;
use super::{axis, base, stroke, titled};
use super::layout::{Class, Layout};
use super::scene::projection_kit::diagram_parts::column;
use super::model::{En1992Snapshot, RcMember};
use super::scene::*;
use super::SceneInput;
use std::f64::consts::PI;

const STRUT_WIDTH: f64 = 0.2;

/// 🕸️ The truss in the member plane, the one of the tie check: see [`checks::DeepTruss`].
pub type Truss = checks::DeepTruss;

/// 🧮️ The truss of a deep beam (span not above three times the height) of the tie check [`checks::deep_beam_truss`]; `None` for other members.
pub fn truss(document: &En1992Snapshot, m: &RcMember) -> Option<Truss> {
    checks::deep_beam_truss(document, m)
}

fn strut(c: &Frame, m: &RcMember, id: usize, from: (f64, f64), to: (f64, f64), force: f64, width: f64, path: &str) -> (Spec, [Vec3; 2]) {
    let (a, b) = (at(c, from.0, 0.0, from.1), at(c, to.0, 0.0, to.1));
    let centre = mul(&add(&a, &b), 0.5);
    let axis = add(&b, &mul(&a, -1.0));
    let frame = projection_kit::member_frame(&centre, &axis);
    let size = Vec3::new(axis.length(), m.width, width);
    let geometry = Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, size } });
    let detail = loc(&format!("concrete strut, compression {} kN", kn(-force)), &format!("Druckstrebe, Druckkraft {} kN", kn(-force)));
    let spec = Spec::new(EntityKind::Solid, format!("{}:stm-strut-{id}", m.id), path.to_string(), geometry, MaterialRole::Concrete, LAYER_STRUT_AND_TIE, loc(&format!("{} strut", m.label_en), &format!("{} Druckstrebe", m.label_de)), detail).translucent(0.55);
    (spec, [a, b])
}

fn tie(c: &Frame, m: &RcMember, t: &Truss, path: &str) -> (Spec, [Vec3; 2]) {
    let (from, to) = (t.nodes[t.members[0].0], t.nodes[t.members[0].1]);
    let (a, b) = (at(c, from.0, 0.0, from.1), at(c, to.0, 0.0, to.1));
    let diameter = 2.0 * (t.tie_area / PI).sqrt();
    let geometry = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: None }, axis: Segment3 { from: a.clone(), to: b.clone() }, diameter, head: HeadShape::None, pattern: None, hole_clearance: None });
    let detail = loc(&format!("tie, tension {} kN, A_s,req = {:.0} mm², equivalent bar Ø{:.0} mm", kn(t.forces[0]), t.tie_area * 1.0e6, diameter * 1000.0), &format!("Zugband, Zugkraft {} kN, A_s,erf = {:.0} mm², Ersatzstab Ø{:.0} mm", kn(t.forces[0]), t.tie_area * 1.0e6, diameter * 1000.0));
    let spec = Spec::new(EntityKind::Fastener, format!("{}:stm-tie", m.id), path.to_string(), geometry, MaterialRole::Rebar, LAYER_STRUT_AND_TIE, loc(&format!("{} tie", m.label_en), &format!("{} Zugband", m.label_de)), detail);
    (spec, [a, b])
}

/// 🎬️ Adds the strut-and-tie entities of every deep beam: two concrete struts, the tie and a label with the forces at the nodes.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in &input.document.members {
        if !out.wants(&m.id) {
            continue;
        }
        let Some(pose) = layout.member(&m.id).filter(|p| p.class == Class::Beam) else { continue };
        let Some(t) = truss(input.document, m) else { continue };
        let (f, path) = (&pose.frame, format!("members[id={}]", m.id));
        let width = STRUT_WIDTH * m.height;
        let (spec, points) = tie(f, m, &t, &path);
        out.push(spec, &points);
        for k in 1..3 {
            let (from, to) = (t.nodes[t.members[k].0], t.nodes[t.members[k].1]);
            let (spec, points) = strut(f, m, k - 1, from, to, t.forces[k], width, &path);
            out.push(spec, &points);
        }
        let apex = at(f, t.nodes[2].0, 0.0, t.nodes[2].1 + 0.05);
        let text = loc(&format!("Strut-and-tie model: F = {} kN, tie {} kN, struts {} kN", kn(t.load), kn(t.forces[0]), kn(-t.forces[1])), &format!("Stabwerkmodell: F = {} kN, Zugband {} kN, Druckstreben {} kN", kn(t.load), kn(t.forces[0]), kn(-t.forces[1])));
        out.annotate(Annotation::Label { id: format!("{}:stm-label", m.id), anchor: apex.clone(), text, tone: Tone::Info }, &apex);
        for (k, node) in t.nodes.iter().enumerate() {
            let (prefix, en, de) = if k == 2 { ("en1992.6.5.node-ccc.", "CCC node", "CCC-Knoten") } else { ("en1992.6.5.node-cct.", "CCT node", "CCT-Knoten") };
            let Some(bound) = binding(input.report, &m.id, &[prefix]) else { continue };
            let anchor = at(f, node.0, 0.0, node.1);
            let text = loc(&format!("{en}: {:.0} % of the node limit", bound.ratio * 100.0), &format!("{de}: {:.0} % der Knotengrenze", bound.ratio * 100.0));
            out.annotate(Annotation::Label { id: format!("{}:stm-node-{k}", m.id), anchor: anchor.clone(), text, tone: tone_of(&Some(bound)) }, &anchor);
        }
    }
}

/// 📈️ Diagram `strut-and-tie:<member>`: the elevation of the truss of every deep beam with the two struts (solid, compression) and the tie (dashed, tension), the node markers (coloured by the CCT and CCC node checks, 6.5.4), the struts (coloured by the strut check, 6.5.2) and the member forces in the table twin; the tie marker carries the tie check.
pub fn diagrams(input: &SceneInput, layout: &Layout) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for m in &input.document.members {
        if layout.member(&m.id).is_none_or(|p| p.class != Class::Beam) {
            continue;
        }
        let Some(t) = truss(input.document, m) else { continue };
        let (kind, path) = (EntityKind::Member, format!("members[id={}]", m.id));
        let strut_binding = binding(input.report, &m.id, &["en1992.6.5.strut."]);
        let node_binding = |k: usize| binding(input.report, &m.id, &[if k == 2 { "en1992.6.5.node-ccc." } else { "en1992.6.5.node-cct." }]);
        let binding = binding(input.report, &m.id, &["en1992.6.5.tie."]);
        let tone = tone_of(&binding);
        let mut g = base(
            m,
            "strut-and-tie",
            titled(m, "Strut-and-tie model", "Stabwerkmodell"),
            loc("Truss of the deep beam (EN 1992-1-1 6.5): two concrete struts from the load point to the support nodes and the tie at the effective depth; forces in kN, compression negative.", "Fachwerk des wandartigen Trägers (EN 1992-1-1 6.5): zwei Betondruckstreben vom Lastpunkt zu den Auflagerknoten und das Zugband in der Nutzhöhe; Kräfte in kN, Druck negativ."),
            vec![axis("Position along the span", "Lage entlang der Spannweite", QuantityKind::Length, "m"), axis("Height above the centroid", "Höhe über dem Schwerpunkt", QuantityKind::Length, "m")],
            kind,
            &path,
        );
        for (k, (from, to)) in t.members.iter().enumerate() {
            let (a, b) = (t.nodes[*from], t.nodes[*to]);
            let (id, en, de) = if k == 0 { ("tie", "Tie", "Zugband") } else { (if k == 1 { "strut-left" } else { "strut-right" }, "Strut", "Druckstrebe") };
            let style = if k == 0 { stroke(3.0, Dash::Dashed, tone) } else { stroke(3.0, Dash::Solid, strut_binding.as_ref().map_or(Tone::Info, |_| tone_of(&strut_binding))) };
            g.series.push(Series::Polyline { id: id.into(), label: loc(&format!("{en} {:.0} kN", t.forces[k] / 1e3), &format!("{de} {:.0} kN", t.forces[k] / 1e3)), points: vec![Vec2::new(a.0, a.1), Vec2::new(b.0, b.1)], style });
        }
        for (k, node) in t.nodes.iter().enumerate() {
            let node_bound = node_binding(k);
            g.markers.push(Marker { id: format!("node-{k}"), at: Vec2::new(node.0, node.1), shape: MarkerShape::Diamond, tone: tone_of(&node_bound), label: loc(&format!("Node {}", k + 1), &format!("Knoten {}", k + 1)), utilisation: node_bound.as_ref().map(|b| b.ratio), bound_check: node_bound.as_ref().map(|b| b.check_id.clone()), binding: None });
        }
        let (a, b) = (t.nodes[t.members[0].0], t.nodes[t.members[0].1]);
        g.markers.push(Marker { id: "tie".into(), at: Vec2::new(0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1)), shape: MarkerShape::Square, tone, label: loc("Tie", "Zugband"), utilisation: binding.as_ref().map(|b| b.ratio), bound_check: binding.as_ref().map(|b| b.check_id.clone()), binding: None });
        g.table_twin = DataTable { columns: vec![
            column("Member", "Stab", "1", (0..3).map(|k| k as f64).collect()),
            column("Axial force", "Normalkraft", "kN", t.forces.iter().map(|f| f / 1e3).collect()),
        ] };
        out.push(g);
    }
    out
}
