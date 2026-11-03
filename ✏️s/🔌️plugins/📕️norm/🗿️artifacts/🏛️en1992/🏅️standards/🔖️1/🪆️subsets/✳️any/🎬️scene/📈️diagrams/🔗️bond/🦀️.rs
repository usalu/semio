//! 🔗️ Bond and anchorage (EN 1992-1-1 8.4.2, 8.4.3, 8.7): the ultimate bond stress `f_bd` (B-01) and the basic anchorage length `l_b,rqd` (B-02) of every bar layer as two diagrams over the bar diameter, and the required anchorage lengths `l_bd` and lap lengths `l_0` as translucent zones on the bars in 3D, toned by the anchorage and lap checks. The numbers are the catalogue entries on the very case the entity checks build, the zone lengths the limits of the member checks; the diameter marker drags the layer diameter and the report follows live. See [`super`].

use super::checks::{self, Case};
use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::layout::Layout;
use super::model::{BarLayer, BondCondition, RcMember};
use super::norm::{CheckReport, CheckResult};
use super::reinforcement::{bar_layout, lap_spans, STOCK_LENGTH};
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

const DIAMETERS_MM: [f64; 11] = [6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 20.0, 25.0, 28.0, 32.0, 40.0];
const ZONE_THICKNESS: f64 = 2.5;

fn value(outcomes: &[checks::Outcome], key: &str) -> Option<f64> {
    outcomes.iter().find(|o| o.key == key).map(|o| o.capacity)
}

fn check_of<'a>(report: &'a CheckReport, id: &str) -> Option<&'a CheckResult> {
    report.checks.iter().find(|c| c.id == id)
}

fn case_of(input: &SceneInput, m: &RcMember, layer: &BarLayer, phi_mm: f64) -> Option<Case> {
    let document = input.document;
    let (concrete, steel) = (document.concrete(&m.concrete_grade_id)?, document.reinforcement(&m.reinforcement_grade_id)?);
    Some(Case::new(&format!("{}-{}", m.id, layer.id), document.annex).with_n("fck", concrete.f_ck * 1.0e-6).with_n("phi", phi_mm).with_s("bond", if layer.bond_condition == BondCondition::Poor { "poor" } else { "good" }).with_n("fyk", steel.f_yk * 1.0e-6))
}

/// 🧮️ `(f_bd in MPa, l_b,rqd in mm)` of a layer at the diameter `phi_mm` from B-01 and B-02.
pub fn bond_values(input: &SceneInput, m: &RcMember, layer: &BarLayer, phi_mm: f64) -> Option<(f64, f64)> {
    let case = case_of(input, m, layer, phi_mm)?;
    let f_bd = value(&checks::evaluate_outcomes("B-01", &case).ok()?, "fbd")? * 1.0e-6;
    let l_b = value(&checks::evaluate_outcomes("B-02", &case).ok()?, "lb_rqd")? * 1.0e3;
    Some((f_bd, l_b))
}

fn grip(m: &RcMember, layer: &BarLayer) -> FieldBinding {
    FieldBinding { path: format!("members[id={}].longitudinal[id={}].diameter", m.id, layer.id), map: Affine { base: 0.0, k: 1.0e-3 }, unit: "m".into(), quantise: Some(Quantise::Round { step: 1.0e-3 }) }
}

fn marker(input: &SceneInput, m: &RcMember, layer: &BarLayer, key: &str, y: f64, en: String, de: String) -> Marker {
    let check = check_of(input.report, &format!("en1992.{key}.{}-{}", m.id, layer.id));
    let (tone, bound_check) = check.map_or((Tone::Info, None), |c| (tone_of(&Some(projection_kit::binding_of_check(c))), Some(c.id.clone())));
    Marker { id: format!("layer-{}", layer.id), at: Vec2::new(layer.diameter * 1.0e3, y), shape: MarkerShape::Diamond, tone, label: loc(&en, &de), utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite() && *u > 0.0), bound_check, binding: Some(grip(m, layer)) }
}

/// 📈️ Diagrams `<member>/bond-strength` (`f_bd` over the diameter, B-01, EN 1992-1-1 8.4.2) and `<member>/anchorage-length` (`l_b,rqd` over the diameter with the provided anchorage length of each layer, B-02, 8.4.3); one curve and one draggable diameter marker per bar layer.
pub fn diagrams(input: &SceneInput, m: &RcMember, kind: EntityKind, path: &str) -> Vec<Diagram2d> {
    let layers: Vec<&BarLayer> = m.longitudinal.iter().filter(|l| l.diameter > 0.0).collect();
    let sweeps: Vec<Vec<(f64, f64, f64)>> = layers.iter().map(|l| DIAMETERS_MM.iter().filter_map(|phi| bond_values(input, m, l, *phi).map(|(f, b)| (*phi, f, b))).collect()).collect();
    if layers.is_empty() || sweeps.iter().any(|s| s.len() < 3) {
        return Vec::new();
    }
    let mut strength = base(m, "bond-strength", titled(m, "Bond strength", "Verbundspannung"), loc("Ultimate bond stress f_bd = 2.25 η1 η2 f_ctd of EN 1992-1-1 8.4.2 (catalogue entry B-01) over the bar diameter for the good or poor bond condition of each layer; the diamond is the layer and drags its diameter.", "Verbundspannung f_bd = 2,25 η1 η2 f_ctd nach EN 1992-1-1 8.4.2 (Katalogeintrag B-01) über dem Stabdurchmesser für die guten oder mäßigen Verbundbedingungen jeder Lage; die Raute ist die Lage und verschiebt deren Durchmesser."), vec![axis("Bar diameter", "Stabdurchmesser", QuantityKind::Length, "mm"), axis("Ultimate bond stress f_bd", "Verbundspannung f_bd", QuantityKind::Stress, "MPa")], kind, path);
    let mut anchorage = base(m, "anchorage-length", titled(m, "Anchorage length", "Verankerungslänge"), loc("Basic required anchorage length l_b,rqd = (φ/4)(σ_sd/f_bd) of EN 1992-1-1 8.4.3 (catalogue entry B-02) over the bar diameter, with the anchorage length provided in each layer as a limit line; the diamond is the layer and drags its diameter.", "Grundmaß der Verankerungslänge l_b,rqd = (φ/4)(σ_sd/f_bd) nach EN 1992-1-1 8.4.3 (Katalogeintrag B-02) über dem Stabdurchmesser, mit der in jeder Lage vorhandenen Verankerungslänge als Grenzlinie; die Raute ist die Lage und verschiebt deren Durchmesser."), vec![axis("Bar diameter", "Stabdurchmesser", QuantityKind::Length, "mm"), axis("Anchorage length l_b,rqd", "Verankerungslänge l_b,rqd", QuantityKind::Length, "mm")], kind, path);
    for (layer, sweep) in layers.iter().zip(&sweeps) {
        let dash = if layer.bond_condition == BondCondition::Poor { Dash::Dashed } else { Dash::Solid };
        let here = bond_values(input, m, layer, layer.diameter * 1.0e3);
        let (f_bd, l_b) = here.unwrap_or((0.0, 0.0));
        let name = |en: &str, de: &str| loc(&format!("{en} {}", layer.id), &format!("{de} {}", layer.id));
        strength.series.push(Series::Polyline { id: format!("fbd-{}", layer.id), label: name("f_bd, layer", "f_bd, Lage"), points: xy(&sweep.iter().map(|r| [r.0, r.1]).collect::<Vec<_>>()), style: stroke(2.0, dash, Tone::Info) });
        anchorage.series.push(Series::Polyline { id: format!("lb-{}", layer.id), label: name("l_b,rqd, layer", "l_b,rqd, Lage"), points: xy(&sweep.iter().map(|r| [r.0, r.2]).collect::<Vec<_>>()), style: stroke(2.0, dash, Tone::Info) });
        if layer.anchorage_length > 0.0 {
            anchorage.series.push(Series::Limit { id: format!("provided-{}", layer.id), label: name("Provided anchorage, layer", "Vorhandene Verankerung, Lage"), axis: AxisRef::Y, value: layer.anchorage_length * 1.0e3, style: stroke(1.0, Dash::Dotted, Tone::Warning) });
        }
        strength.markers.push(marker(input, m, layer, "b-01.fbd", f_bd, format!("{}: Ø{:.0} mm, f_bd = {f_bd:.2} MPa", layer.id, layer.diameter * 1.0e3), format!("{}: Ø{:.0} mm, f_bd = {f_bd:.2} MPa", layer.id, layer.diameter * 1.0e3)));
        anchorage.markers.push(marker(input, m, layer, "b-02.lb_rqd", l_b, format!("{}: Ø{:.0} mm, l_b,rqd = {l_b:.0} mm", layer.id, layer.diameter * 1.0e3), format!("{}: Ø{:.0} mm, l_b,rqd = {l_b:.0} mm", layer.id, layer.diameter * 1.0e3)));
    }
    let first = &sweeps[0];
    strength.table_twin = DataTable { columns: std::iter::once(column("Bar diameter", "Stabdurchmesser", "mm", first.iter().map(|r| r.0).collect())).chain(layers.iter().zip(&sweeps).map(|(l, s)| column(&format!("f_bd {}", l.id), &format!("f_bd {}", l.id), "MPa", s.iter().map(|r| r.1).collect()))).collect() };
    anchorage.table_twin = DataTable { columns: std::iter::once(column("Bar diameter", "Stabdurchmesser", "mm", first.iter().map(|r| r.0).collect())).chain(layers.iter().zip(&sweeps).map(|(l, s)| column(&format!("l_b,rqd {}", l.id), &format!("l_b,rqd {}", l.id), "mm", s.iter().map(|r| r.2).collect()))).collect() };
    vec![strength, anchorage]
}

#[allow(clippy::too_many_arguments)]
fn zone(m: &RcMember, pose: &super::layout::MemberPose, layer: &BarLayer, centre: (f64, f64), width: f64, span: (f64, f64), tag: &str, check: Option<&CheckResult>, en: &str, de: &str, out: &mut Out) {
    if span.1 - span.0 <= 1e-4 {
        return;
    }
    let f = &pose.frame;
    let geometry = Geometry::Member(MemberGeometry {
        axis: polyline(&[at(f, span.0, centre.0, centre.1), at(f, span.1, centre.0, centre.1)]),
        frame: f.clone(),
        section: rect_section(width, ZONE_THICKNESS * layer.diameter),
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: Vec::new(),
    });
    let named = |text: &str, other: &str| loc(&format!("{} {text}", m.label_en), &format!("{} {other}", m.label_de));
    let detail = named(&format!("{en}, {:.0} mm, layer {}", (span.1 - span.0) * 1.0e3, layer.id), &format!("{de}, {:.0} mm, Lage {}", (span.1 - span.0) * 1.0e3, layer.id));
    let binding = check.map(projection_kit::binding_of_check);
    let spec = Spec::new(EntityKind::Member, format!("{}:{tag}", m.id), format!("members[id={}].longitudinal[id={}]", m.id, layer.id), geometry, MaterialRole::Rebar, LAYER_BOND, named(en, de), detail).bound(binding).translucent(0.35);
    out.push(spec, &[at(f, span.0, centre.0, centre.1), at(f, span.1, centre.0, centre.1)]);
}

/// 🎬️ The anchorage zones `l_bd` at both bar ends and the lap zones `l_0` between the lapped pieces of every layer, the lengths of the limits of the checks `en1992.8.4.anchorage.<member>.<layer>` and `en1992.8.7.lap.<member>.<layer>`, drawn on the bars and toned by those checks; with a label of `f_bd` and `l_b,rqd` per layer.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in &input.document.members {
        let (true, Some(pose)) = (out.wants(&m.id), layout.member(&m.id)) else { continue };
        if m.span <= 1e-6 || m.height <= 1e-6 || m.width <= 1e-6 {
            continue;
        }
        let w = if m.kind == super::model::MemberKind::FlatSlab { m.span } else { m.width };
        let (l, cv) = (m.span, m.cover);
        let (from, to) = if l - 2.0 * cv > 0.1 { (cv, l - cv) } else { (0.0, l) };
        for bars in bar_layout(m, w) {
            let Some(layer) = m.longitudinal.get(bars.layer).filter(|l| l.diameter > 0.0) else { continue };
            let Some(&first) = bars.points.first() else { continue };
            let (y_lo, y_hi) = bars.points.iter().fold((first.0, first.0), |(lo, hi), p| (lo.min(p.0), hi.max(p.0)));
            let z = bars.points.iter().map(|p| p.1).sum::<f64>() / bars.points.len() as f64;
            let (centre, width) = (((y_lo + y_hi) / 2.0, z), y_hi - y_lo + 2.0 * layer.diameter);
            let anchorage = check_of(input.report, &format!("en1992.8.4.anchorage.{}.{}", m.id, layer.id));
            let lap = check_of(input.report, &format!("en1992.8.7.lap.{}.{}", m.id, layer.id));
            if let Some(required) = anchorage.map(|c| c.limit.value).filter(|v| *v > 0.0) {
                let reach = required.min((to - from) / 2.0);
                zone(m, pose, layer, centre, width, (from, from + reach), &format!("anchorage-{}-a", layer.id), anchorage, "anchorage zone", "Verankerungsbereich", out);
                zone(m, pose, layer, centre, width, (to - reach, to), &format!("anchorage-{}-b", layer.id), anchorage, "anchorage zone", "Verankerungsbereich", out);
            }
            if let Some(required) = lap.map(|c| c.limit.value).filter(|v| *v > 0.0) {
                let pieces = lap_spans(from, to, STOCK_LENGTH, if layer.lap_share > 0.0 { layer.lap_length } else { 0.0 });
                for (j, pair) in pieces.windows(2).enumerate() {
                    let start = pair[1].0;
                    zone(m, pose, layer, centre, width, (start, start + required), &format!("lap-{}-{j}", layer.id), lap, "lap zone", "Übergreifungsbereich", out);
                }
            }
            if let Some((f_bd, l_b)) = bond_values(input, m, layer, layer.diameter * 1.0e3) {
                let tone = tone_of(&anchorage.map(projection_kit::binding_of_check));
                let anchor = at(&pose.frame, from, centre.0, z + 3.0 * layer.diameter);
                let text = loc(&format!("{}: f_bd = {f_bd:.2} MPa, l_b,rqd = {l_b:.0} mm", layer.id), &format!("{}: f_bd = {f_bd:.2} MPa, l_b,rqd = {l_b:.0} mm", layer.id));
                out.annotate(Annotation::Label { id: format!("{}:bond-label-{}", m.id, layer.id), anchor: anchor.clone(), text, tone }, &anchor);
            }
        }
    }
}
