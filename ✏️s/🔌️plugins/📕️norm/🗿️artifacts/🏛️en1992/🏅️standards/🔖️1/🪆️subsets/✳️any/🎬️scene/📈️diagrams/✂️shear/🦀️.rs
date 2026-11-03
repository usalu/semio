//! ✂️ Shear: the variable-angle truss of EN 1992-1-1 6.2.3 (resistance curves over cot θ and the truss sketch), the strut-and-tie model of a deep member (6.5, indicative lever-arm model) and the critical shear sections drawn as planes at the distance d from the supports (6.2.2).
//! The resistances are the functions of the checks crate (`part_1_1`), so curves, markers and the report cannot disagree.

use super::checks::part_1_1;
use super::kit::*;
use super::layout::{Class, Layout};
use super::model::{MemberKind, RcMember};
use super::scene::diagram_kit::curves;
use super::scene::projection_kit::diagram_parts::{self, column};
use super::scene::*;
use super::SceneInput;

const COT_MIN: f64 = 1.0;
const COT_MAX: f64 = 3.0;
const SAMPLES: usize = 24;
const TRUSS_BAYS: usize = 3;
const DEEP_RATIO: f64 = 3.0;
pub const SHEAR_PLANE_TAG: &str = "shear-plane";

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, None)
}

fn stroke(width_px: f64, dash: Dash, tone: Tone) -> Stroke {
    Stroke { width_px, dash, tone }
}

/// 🧱️ The shear data of one member drawn from the snapshot and the report: width, effective depth, lever arm, link area per length, steel and concrete strength, and the design shear and limit of the shear check.
#[derive(Clone, Debug, PartialEq)]
pub struct ShearBasis {
    pub b: f64,
    pub d: f64,
    pub z: f64,
    pub asw_s: f64,
    pub f_ck: f64,
    pub f_yk: f64,
    pub v_ed: f64,
    pub check: String,
    pub utilisation: f64,
}

pub fn basis(input: &SceneInput, m: &RcMember) -> Option<ShearBasis> {
    let doc = input.document;
    let concrete = doc.concrete_grades.iter().find(|g| g.id == m.concrete_grade_id)?;
    let steel = doc.reinforcement_grades.iter().find(|g| g.id == m.reinforcement_grade_id)?;
    let id = format!("en1992.6.2.shear.{}", m.id);
    let check = input.report.checks.iter().find(|c| c.id == id && c.utilization.is_finite())?;
    let sec = super::checks::member_section_in(doc, m, concrete.f_ck, steel.f_yk, steel.e_s, doc.annex, false);
    let geo = super::checks::Geo::of(&sec, m);
    let (d, asw_s) = (geo.d, m.asw_per_s());
    (d > 1e-6 && geo.b_w > 1e-6 && asw_s > 0.0).then(|| ShearBasis { b: geo.b_w, d, z: geo.z(), asw_s, f_ck: concrete.f_ck, f_yk: steel.f_yk, v_ed: check.computed.value.abs(), check: id, utilisation: check.utilization })
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

/// 📐️ The truss resistances over cot θ in kN: link tension `V_Rd,s = A_sw/s·z·f_ywd·cot θ` and strut crushing `V_Rd,max` (6.8, 6.9) from the checks crate.
pub fn truss_resistances(input: &SceneInput, s: &ShearBasis, cot: f64) -> (f64, f64) {
    let f_ywd = s.f_yk / super::norm::ndp_number("ndp.en1992.gamma-s", input.document.annex);
    (part_1_1::shear_v_rd_s_n(s.asw_s, s.z, f_ywd, cot) / 1e3, part_1_1::shear_v_rd_max_n(s.b, s.z, s.f_ck, cot, input.document.annex) / 1e3)
}

fn shear_truss(input: &SceneInput, m: &RcMember, s: &ShearBasis, path: &str, kind: EntityKind) -> Vec<Diagram2d> {
    let (v_cs, cot) = part_1_1::shear_capacity_n(s.b, s.z, s.f_ck, s.f_yk, s.asw_s, 0.0, s.v_ed, input.document.annex);
    let sample = |pick: fn((f64, f64)) -> f64| curves::sample(|c| pick(truss_resistances(input, s, c)), COT_MIN, COT_MAX, SAMPLES);
    let (links, struts) = (sample(|r| r.0), sample(|r| r.1));
    let line = |id: &str, en: &str, de: &str, points: &[[f64; 2]], style: Stroke| Series::Polyline { id: id.into(), label: loc(en, de), points: points.iter().map(|p| Vec2::new(p[0], p[1])).collect(), style };
    let focus = Some(entity_ref(kind, &m.id, path));
    let curves_diagram = Diagram2d {
        id: format!("shear-truss:{}", m.id),
        title: loc(&format!("{} - shear truss", m.label_en), &format!("{} - Fachwerkmodell Querkraft", m.label_de)),
        description: loc("Resistances of the variable-angle truss over the strut inclination cot θ: links V_Rd,s rise with cot θ, strut crushing V_Rd,max falls (EN 1992-1-1, 6.2.3, Eq. 6.8 and 6.9); the marker is the inclination of the check at the shear V_Ed.", "Widerstände des Fachwerkmodells mit veränderlicher Druckstrebenneigung über cot θ: Bügel V_Rd,s steigen mit cot θ, Druckstrebenbruch V_Rd,max fällt (EN 1992-1-1, 6.2.3, Gl. 6.8 und 6.9); der Marker ist die Neigung der Nachweises bei der Querkraft V_Ed."),
        axes: vec![axis("Strut inclination cot θ", "Druckstrebenneigung cot θ", QuantityKind::Dimensionless, "1"), axis("Shear resistance", "Querkrafttragfähigkeit", QuantityKind::Force, "kN")],
        series: vec![
            line("v-rd-s", "Links V_Rd,s", "Bügel V_Rd,s", &links, stroke(2.0, Dash::Solid, Tone::Info)),
            line("v-rd-max", "Strut crushing V_Rd,max", "Druckstrebenbruch V_Rd,max", &struts, stroke(2.0, Dash::Dashed, Tone::Warning)),
            Series::Limit { id: "v-ed".into(), label: loc("Design shear V_Ed", "Bemessungsquerkraft V_Ed"), axis: AxisRef::Y, value: s.v_ed / 1e3, style: stroke(1.5, Dash::Dotted, Tone::Error) },
        ],
        markers: vec![Marker { id: "cot-theta".into(), at: Vec2::new(cot, v_cs / 1e3), shape: MarkerShape::Diamond, tone: tone(s.utilisation), label: loc(&format!("cot θ = {cot:.2}, V_Rd = {:.0} kN", v_cs / 1e3), &format!("cot θ = {cot:.2}, V_Rd = {:.0} kN", v_cs / 1e3)), utilisation: Some(s.utilisation), bound_check: Some(s.check.clone()), binding: None }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: true,
        table_twin: DataTable { columns: vec![column("cot θ", "cot θ", "1", links.iter().map(|p| p[0]).collect()), column("V_Rd,s", "V_Rd,s", "kN", links.iter().map(|p| p[1]).collect()), column("V_Rd,max", "V_Rd,max", "kN", struts.iter().map(|p| p[1]).collect())] },
    };
    let a = s.z * cot;
    let nodes: Vec<usize> = (0..TRUSS_BAYS).collect();
    let mut series = vec![
        line("tension-chord", "Tension chord", "Zuggurt", &[[0.0, 0.0], [a * (TRUSS_BAYS as f64 + 1.0), 0.0]], stroke(3.0, Dash::Solid, Tone::Info)),
        line("compression-chord", "Compression chord", "Druckgurt", &[[a, s.z], [a * (TRUSS_BAYS as f64 + 1.0), s.z]], stroke(3.0, Dash::Solid, Tone::Warning)),
    ];
    for i in &nodes {
        let x0 = *i as f64 * a;
        series.push(line(&format!("strut-{i}"), &format!("Strut {}", i + 1), &format!("Druckstrebe {}", i + 1), &[[x0, 0.0], [x0 + a, s.z]], stroke(2.0, Dash::Solid, Tone::Warning)));
        series.push(line(&format!("tie-{i}"), &format!("Link {}", i + 1), &format!("Bügel {}", i + 1), &[[x0 + a, s.z], [x0 + a, 0.0]], stroke(2.0, Dash::Dashed, Tone::Info)));
    }
    let sketch = Diagram2d {
        id: format!("shear-truss-sketch:{}", m.id),
        title: loc(&format!("{} - truss geometry", m.label_en), &format!("{} - Fachwerkgeometrie", m.label_de)),
        description: loc(&format!("Variable-angle truss of {TRUSS_BAYS} bays with lever arm z = {:.0} mm and strut inclination cot θ = {cot:.2}: struts rise over z·cot θ = {:.0} mm, the links close the nodes (EN 1992-1-1, 6.2.3, Figure 6.5).", s.z * 1e3, a * 1e3), &format!("Fachwerk mit veränderlicher Neigung aus {TRUSS_BAYS} Feldern mit Hebelarm z = {:.0} mm und Druckstrebenneigung cot θ = {cot:.2}: die Druckstreben steigen über z·cot θ = {:.0} mm an, die Bügel schließen die Knoten (EN 1992-1-1, 6.2.3, Bild 6.5).", s.z * 1e3, a * 1e3)),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m"), axis("Height y", "Höhe y", QuantityKind::Length, "m")],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Bay", "Feld", "1", nodes.iter().map(|i| (*i + 1) as f64).collect()), column("Strut foot x", "Strebenfuß x", "m", nodes.iter().map(|i| *i as f64 * a).collect()), column("Strut head x", "Strebenkopf x", "m", nodes.iter().map(|i| *i as f64 * a + a).collect())] },
    };
    vec![curves_diagram, sketch]
}

/// 🕸️ The indicative strut-and-tie model of a deep simply supported member under a central load `2·V_Ed`: two struts from the load node to the supports, a tie between the supports; `tan θ = z / (L/2)`, `T = V_Ed·cot θ`, `C = V_Ed / sin θ` (EN 1992-1-1, 6.5.2).
#[derive(Clone, Debug, PartialEq)]
pub struct StrutAndTie {
    pub theta: f64,
    pub tie: f64,
    pub strut: f64,
    pub strut_stress: f64,
    pub strut_limit: f64,
}

pub fn strut_and_tie(s: &ShearBasis, span: f64) -> StrutAndTie {
    let half = 0.5 * span;
    let theta = (s.z / half).atan();
    let (tie, strut) = (s.v_ed * half / s.z, s.v_ed / theta.sin());
    let fcd = s.f_ck / 1.5;
    let nu = 1.0 - s.f_ck / 250.0e6;
    let width = 0.25 * s.z;
    StrutAndTie { theta, tie, strut, strut_stress: strut / (s.b * width), strut_limit: 0.85 * nu * fcd }
}

fn strut_tie_diagram(m: &RcMember, s: &ShearBasis, path: &str, kind: EntityKind) -> Diagram2d {
    let model = strut_and_tie(s, m.span);
    let (half, z) = (0.5 * m.span, s.z);
    let line = |id: &str, en: &str, de: &str, points: [[f64; 2]; 2], style: Stroke| Series::Polyline { id: id.into(), label: loc(en, de), points: points.iter().map(|p| Vec2::new(p[0], p[1])).collect(), style };
    let nodes = [("node-a", 0.0, 0.0, "CCT"), ("node-b", half, z, "CCC"), ("node-c", m.span, 0.0, "CCT")];
    let ratio = model.strut_stress / model.strut_limit;
    Diagram2d {
        id: format!("strut-and-tie:{}", m.id),
        title: loc(&format!("{} - strut-and-tie model", m.label_en), &format!("{} - Stabwerkmodell", m.label_de)),
        description: loc(&format!("Indicative strut-and-tie model of a deep member (L/h ≤ {DEEP_RATIO}): two struts C = {:.0} kN at θ = {:.1}° and a tie T = {:.0} kN; strut stress {:.1} MPa of {:.1} MPa (EN 1992-1-1, 6.5.2 and 6.5.4, strut width assumed z/4, nodes CCC and CCT).", model.strut / 1e3, model.theta.to_degrees(), model.tie / 1e3, model.strut_stress / 1e6, model.strut_limit / 1e6), &format!("Richtungsweisendes Stabwerkmodell eines wandartigen Trägers (L/h ≤ {DEEP_RATIO}): zwei Druckstreben C = {:.0} kN unter θ = {:.1}° und ein Zugband T = {:.0} kN; Strebenspannung {:.1} MPa von {:.1} MPa (EN 1992-1-1, 6.5.2 und 6.5.4, Strebenbreite z/4 angenommen, Knoten CCC und CCT).", model.strut / 1e3, model.theta.to_degrees(), model.tie / 1e3, model.strut_stress / 1e6, model.strut_limit / 1e6)),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m"), axis("Height y", "Höhe y", QuantityKind::Length, "m")],
        series: vec![
            line("tie", "Tie T", "Zugband T", [[0.0, 0.0], [m.span, 0.0]], stroke(3.0, Dash::Solid, Tone::Info)),
            line("strut-left", "Strut C", "Druckstrebe C", [[0.0, 0.0], [half, z]], stroke(3.0, Dash::Dashed, Tone::Warning)),
            line("strut-right", "Strut C", "Druckstrebe C", [[m.span, 0.0], [half, z]], stroke(3.0, Dash::Dashed, Tone::Warning)),
        ],
        markers: nodes.iter().map(|(id, x, y, kind)| Marker { id: (*id).into(), at: Vec2::new(*x, *y), shape: MarkerShape::Square, tone: tone(ratio), label: loc(&format!("Node {kind}"), &format!("Knoten {kind}")), utilisation: (*id == "node-b").then_some(s.utilisation), bound_check: (*id == "node-b").then(|| s.check.clone()), binding: None }).collect(),
        annotations: Vec::new(),
        focus: Some(entity_ref(kind, &m.id, path)),
        legend: true,
        table_twin: DataTable { columns: vec![column("Force", "Kraft", "1", vec![1.0, 2.0]), column("Magnitude", "Betrag", "kN", vec![model.tie / 1e3, model.strut / 1e3]), column("Angle θ", "Winkel θ", "°", vec![0.0, model.theta.to_degrees()])] },
    }
}

pub fn diagrams(input: &SceneInput, layout: &Layout) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for m in &input.document.members {
        let Some(pose) = layout.member(&m.id) else { continue };
        let Some(s) = basis(input, m) else { continue };
        if m.span <= 1e-6 || pose.class == Class::Upright {
            continue;
        }
        let kind = if pose.class == Class::Slab { EntityKind::Plate } else { EntityKind::Member };
        let path = format!("members[id={}]", m.id);
        out.extend(shear_truss(input, m, &s, &path, kind));
        if m.span / m.height.max(1e-6) <= DEEP_RATIO && m.kind != MemberKind::FlatSlab {
            out.push(strut_tie_diagram(m, &s, &path, kind));
        }
    }
    out
}

/// 🪟️ The critical shear sections at the distance `d` from both supports (6.2.2): translucent planes across the section, coloured by the shear utilisation.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in &input.document.members {
        if !out.wants(&m.id) {
            continue;
        }
        let Some(pose) = layout.member(&m.id) else { continue };
        let Some(s) = basis(input, m) else { continue };
        if pose.class == Class::Upright || m.span <= 2.0 * s.d {
            continue;
        }
        let f = &pose.frame;
        let width = if m.kind == MemberKind::FlatSlab { m.span } else { m.width };
        let path = format!("members[id={}]", m.id);
        let binding = binding(input.report, &m.id, &["en1992.6.2.shear."]);
        for (tag, x, en, de) in [("start", s.d, "at d from the left support", "im Abstand d vom linken Auflager"), ("end", m.span - s.d, "at d from the right support", "im Abstand d vom rechten Auflager")] {
            let points: Vec<Vec3> = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)].iter().map(|(sy, sz)| at(f, x, sy * width / 2.0, sz * m.height / 2.0)).collect();
            let spec = Spec::new(EntityKind::Surface, format!("{}:{SHEAR_PLANE_TAG}-{tag}", m.id), path.clone(), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: ring3(points.clone()) } }), MaterialRole::Neutral, LAYER_SHEAR, loc(&format!("{} critical shear section {en}", m.label_en), &format!("{} kritischer Schubschnitt {de}", m.label_de)), loc(&format!("Shear plane {en}: V_Ed = {:.0} kN, d = {:.0} mm", s.v_ed / 1e3, s.d * 1e3), &format!("Schubschnitt {de}: V_Ed = {:.0} kN, d = {:.0} mm", s.v_ed / 1e3, s.d * 1e3)))
                .bound(binding.clone())
                .translucent(0.35)
                .ghosted();
            out.push(spec, &points);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
