//! 🧵️ Fatigue details in 3D and 2D: the rainflow-counted stress-range histogram as bars beside the detail, the Palmgren-Miner damage sum as one bar against its limit of 1, and the same two as diagrams.

use super::kit::*;
use super::layout::Layout;
use super::model::{En1993Snapshot, FatigueDetail};
use super::norm::CheckReport;
use super::scene::projection_kit::diagram_parts::{axis, column, stroke};
use super::scene::*;
use super::{checks, SceneInput};
use checks::part_1_9;

const CLASSES: usize = 8;
const BAR_WIDTH: f64 = 0.06;
const BAR_GAP: f64 = 0.02;
const BAR_HEIGHT: f64 = 0.4;
const DAMAGE_HEIGHT: f64 = 0.5;
const DAMAGE_CAP: f64 = 3.0;

/// 🧵️ The `(range in MPa, count)` blocks of a detail: the rainflow count of its stress history when the spectrum is empty, otherwise the spectrum bands.
pub fn blocks(detail: &FatigueDetail) -> Vec<(f64, f64)> {
    let history: Vec<f64> = detail.stress_history.split_whitespace().filter_map(|v| v.parse::<f64>().ok()).collect();
    if detail.spectrum.is_empty() && history.len() >= 2 {
        checks::clauses::fatigue_fire::rainflow(&history)
    } else {
        detail.spectrum.iter().filter(|b| b.delta_sigma > 0.0 && b.cycles > 0.0).map(|b| (b.delta_sigma / 1.0e6, b.cycles)).collect()
    }
}

/// 📊️ Equal-width classes over `0..max range`: `(class upper edge in MPa, summed count)`.
pub fn histogram(blocks: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let top = blocks.iter().fold(0.0_f64, |m, (r, _)| m.max(*r));
    if top <= 0.0 {
        return Vec::new();
    }
    let width = top / CLASSES as f64;
    let mut counts = vec![0.0; CLASSES];
    for (range, count) in blocks {
        counts[(((range / width).ceil() as usize).max(1) - 1).min(CLASSES - 1)] += count;
    }
    counts.into_iter().enumerate().map(|(k, c)| ((k + 1) as f64 * width, c)).collect()
}

/// 🧮️ Palmgren-Miner damage sum D of a detail from its blocks.
pub fn damage(detail: &FatigueDetail) -> f64 {
    let spectrum: Vec<(f64, f64)> = blocks(detail).into_iter().map(|(r, n)| (r * 1.0e6, n)).collect();
    part_1_9::miner_damage(&spectrum, part_1_9::detail_category_pa(detail.category), part_1_9::gamma_mf(part_1_9::parse_method(detail.method.as_str())))
}

fn bar(input: &SceneInput, id: String, name: LocalizedCopy, detail: LocalizedCopy, base: &Vec3, frame: &Frame, offset: f64, width: f64, depth: f64, height: f64, owner: &str, utilised: bool, out: &mut Out) {
    let origin = along(&along(base, &frame.y, offset), &frame.z, height / 2.0);
    let (w, h) = (width / 2.0, depth / 2.0);
    out.push(
        Spec {
            kind: EntityKind::Plate,
            id,
            collection: "fatigueDetails",
            geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: origin.clone(), x: frame.x.clone(), y: frame.y.clone(), z: frame.z.clone() }, outline: Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)]), thickness: height.max(1e-3), offset: 0.0, stiffeners: Vec::new() }),
            role: MaterialRole::Steel,
            binding: if utilised { binding(input.report, &[owner]) } else { None },
            name,
            detail,
            layer: LAYER_FATIGUE,
            handles: Vec::new(),
            deformation: None,
        },
        &[base.clone(), origin],
    );
}

/// 📏️ The reference stress ranges of the S-N curve of a detail in MPa, drawn as planes beside the histogram: `(tag, Δσ, EN label, DE label)` for Δσ_C (2 million cycles), Δσ_D (constant amplitude limit) and Δσ_L (cut-off), each within the span of the histogram classes.
pub fn limits(detail: &FatigueDetail) -> Vec<(&'static str, f64, &'static str, &'static str)> {
    let category = part_1_9::detail_category_pa(detail.category) / 1.0e6;
    let (d, l) = (checks::clauses::fatigue_fire::delta_sigma_d(category), checks::clauses::fatigue_fire::delta_sigma_l(category));
    let top = histogram(&blocks(detail)).last().map_or(0.0, |c| c.0);
    [("c", category, "Δσ_C", "Δσ_C"), ("d", d, "Δσ_D (fatigue limit)", "Δσ_D (Dauerfestigkeit)"), ("l", l, "Δσ_L (cut-off)", "Δσ_L (Schwellenwert)")].into_iter().filter(|(_, range, _, _)| *range > 0.0 && *range <= top).collect()
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for detail in &d.fatigue_details {
        if !out.own("fatigueDetails", &detail.id) {
            continue;
        }
        let Some((pose, section)) = layout.member(&detail.member_id).zip(d.members.iter().find(|m| m.id == detail.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { continue };
        let classes = histogram(&blocks(detail));
        if classes.is_empty() {
            continue;
        }
        let f = &pose.frame;
        let base = along(&along(&along(&f.origin, &f.x, pose.length * 0.75), &f.z, section.h / 2.0 + 0.8), &f.y, -0.5 * (CLASSES as f64 + 2.0) * (BAR_WIDTH + BAR_GAP));
        let peak = classes.iter().fold(0.0_f64, |m, (_, c)| m.max(*c));
        for (k, (edge, count)) in classes.iter().enumerate().filter(|(_, (_, c))| *c > 0.0) {
            let text = format!("Δσ ≤ {edge:.0} MPa: {count:.1} cycles");
            let text_de = format!("Δσ ≤ {edge:.0} MPa: {count:.1} Zyklen");
            bar(input, format!("{}:rainflow-{}", detail.id, k + 1), loc(&format!("Rainflow class {} of {}", k + 1, detail.id), &format!("Rainflow-Klasse {} von {}", k + 1, detail.id)), loc(&text, &text_de), &base, f, k as f64 * (BAR_WIDTH + BAR_GAP), BAR_WIDTH, BAR_WIDTH, BAR_HEIGHT * count / peak, &detail.id, false, out);
        }
        let pitch = BAR_WIDTH + BAR_GAP;
        let width = classes[0].0;
        for (tag, range, en, de) in limits(detail) {
            let at = (range / width - 0.5) * pitch;
            let text = format!("{en} = {range:.1} MPa");
            let text_de = format!("{de} = {range:.1} MPa");
            bar(input, format!("{}:limit-{tag}", detail.id), loc(&format!("{en} of {}", detail.id), &format!("{de} von {}", detail.id)), loc(&text, &text_de), &base, f, at, 3.0 * BAR_WIDTH, 0.004, BAR_HEIGHT * 1.15, &detail.id, false, out);
            let top = along(&along(&base, &f.y, at), &f.z, BAR_HEIGHT * 1.15 + 0.06);
            out.annotate(Annotation::Label { id: format!("{}:limit-{tag}-label", detail.id), anchor: top.clone(), text: loc(&text, &text_de), tone: Tone::Neutral }, &top);
        }
        let total = damage(detail);
        let offset = (CLASSES as f64 + 1.0) * (BAR_WIDTH + BAR_GAP);
        let text = format!("D = {total:.3} (limit 1)");
        let text_de = format!("D = {total:.3} (Grenze 1)");
        bar(input, format!("{}:damage", detail.id), loc(&format!("Damage sum of {}", detail.id), &format!("Schadenssumme von {}", detail.id)), loc(&text, &text_de), &base, f, offset, 2.0 * BAR_WIDTH, BAR_WIDTH, DAMAGE_HEIGHT * total.min(DAMAGE_CAP), &detail.id, true, out);
        bar(input, format!("{}:damage-limit", detail.id), loc(&format!("Damage limit D = 1 of {}", detail.id), &format!("Schadensgrenze D = 1 von {}", detail.id)), loc("Palmgren-Miner limit D = 1", "Palmgren-Miner-Grenze D = 1"), &along(&base, &f.z, DAMAGE_HEIGHT), f, offset, 3.0 * BAR_WIDTH, BAR_WIDTH, 0.004, &detail.id, false, out);
        let at = along(&along(&base, &f.y, offset), &f.z, DAMAGE_HEIGHT * total.min(DAMAGE_CAP) + 0.1);
        out.annotate(Annotation::Label { id: format!("{}:damage-label", detail.id), anchor: at.clone(), text: loc(&text, &text_de), tone: tone_of(&binding(input.report, &[detail.id.as_str()])) }, &at);
    }
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn rect(x0: f64, x1: f64, y1: f64) -> Vec<Vec2> {
    vec![Vec2::new(x0, 0.0), Vec2::new(x1, 0.0), Vec2::new(x1, y1), Vec2::new(x0, y1)]
}

fn rainflow(detail: &FatigueDetail, classes: &[(f64, f64)], total: f64, status: Tone) -> Diagram2d {
    let width = classes.first().map_or(1.0, |(edge, _)| *edge);
    let peak = classes.iter().fold(0.0_f64, |m, (_, c)| m.max(*c)).max(1.0);
    let series = classes.iter().enumerate().filter(|(_, (_, c))| *c > 0.0).map(|(k, (edge, count))| Series::Polygon { id: format!("class-{}", k + 1), label: loc(&format!("Δσ ≤ {edge:.0} MPa"), &format!("Δσ ≤ {edge:.0} MPa")), ring: rect(edge - width, *edge, *count), fill: Fill { tone: status, opacity: 0.45 }, hatch: None }).collect();
    Diagram2d {
        id: format!("rainflow:{}", detail.id),
        title: loc(&format!("Rainflow histogram – {}", detail.id), &format!("Rainflow-Histogramm – {}", detail.id)),
        description: loc(&format!("Stress-range classes of the rainflow count (ASTM three-point, half cycles 0.5) of the stress history; damage sum D = {total:.3}."), &format!("Schwingbreitenklassen der Rainflow-Zählung (ASTM-Dreipunkt, Halbzyklen 0,5) der Spannungsgeschichte; Schadenssumme D = {total:.3}.")),
        axes: vec![axis("Stress range Δσ", "Spannungsschwingbreite Δσ", QuantityKind::Stress, "MPa", AxisScale::Linear, Some((0.0, width * classes.len() as f64))), axis("Cycles n", "Lastspiele n", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, peak * 1.15)))],
        series,
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "damage".into(), at: Vec2::new(width * 0.5, peak * 1.05), text: loc(&format!("D = {total:.3}"), &format!("D = {total:.3}")), tone: status }],
        focus: Some(EntityRef::new(EntityKind::Group, &detail.id, format!("fatigueDetails[id={}]", detail.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Class upper edge Δσ", "Klassenobergrenze Δσ", "MPa", classes.iter().map(|c| c.0).collect()), column("Cycles n", "Lastspiele n", "1", classes.iter().map(|c| c.1).collect())] },
    }
}

fn damage_bar(detail: &FatigueDetail, total: f64, status: Tone) -> Diagram2d {
    let top = total.max(1.0) * 1.2;
    Diagram2d {
        id: format!("damage:{}", detail.id),
        title: loc(&format!("Damage sum – {}", detail.id), &format!("Schadenssumme – {}", detail.id)),
        description: loc(&format!("Palmgren-Miner damage sum D = {total:.3} of detail category {} against the limit 1.", detail.category), &format!("Palmgren-Miner-Schadenssumme D = {total:.3} des Kerbfalls {} gegen die Grenze 1.", detail.category)),
        axes: vec![axis("Detail", "Detail", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.0))), axis("Damage sum D", "Schadenssumme D", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, top)))],
        series: vec![
            Series::Polygon { id: "damage".into(), label: loc("D", "D"), ring: rect(0.3, 0.7, total), fill: Fill { tone: status, opacity: 0.55 }, hatch: None },
            Series::Limit { id: "limit".into(), label: loc("Limit D = 1", "Grenze D = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "value".into(), at: Vec2::new(0.5, total + 0.05 * top), text: loc(&format!("D = {total:.3}"), &format!("D = {total:.3}")), tone: status }],
        focus: Some(EntityRef::new(EntityKind::Group, &detail.id, format!("fatigueDetails[id={}]", detail.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Damage sum D", "Schadenssumme D", "1", vec![total])] },
    }
}

/// 📈️ The rainflow histogram and the damage-sum bar of every fatigue detail (of the focused entity only when `focus` is set).
pub fn diagrams(document: &En1993Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    document
        .fatigue_details
        .iter()
        .filter(|detail| focus.is_none_or(|f| f == detail.id || f == detail.member_id))
        .flat_map(|detail| {
            let classes = histogram(&blocks(detail));
            if classes.is_empty() {
                return Vec::new();
            }
            let total = damage(detail);
            let ratio = report.checks.iter().find(|c| c.id == format!("en1993.1-9.fat.{}", detail.id)).map_or(total, |c| c.utilization);
            let status = tone(ratio);
            vec![rainflow(detail, &classes, total, status), damage_bar(detail, total, status)]
        })
        .collect()
}
