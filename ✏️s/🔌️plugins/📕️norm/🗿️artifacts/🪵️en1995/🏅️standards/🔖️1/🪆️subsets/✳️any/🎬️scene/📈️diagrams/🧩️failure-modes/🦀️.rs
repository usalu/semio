//! 🔩️ Johansen failure modes as sketches: one cell per mode of EN 1995-1-1 Figures 8.2 and 8.3 (letters a to m) showing the members, the deformed fastener, its plastic hinges and the crushed embedment zones, with the governing mode highlighted in the tone of the connection check. Every sketch is schematic and has a table twin with the capacity of the mode.

use super::*;
use crate::norm::design_case::DesignCase;

const AMPLITUDE: f64 = 0.17;
const AXIS: f64 = 0.6;
const CRUSH: f64 = 0.07;

type Rod = &'static [(f64, f64)];
type Crush = &'static [(f64, f64, f64)];

struct Sketch {
    rod: Rod,
    hinges: &'static [usize],
    crush: Crush,
}

const fn sk(rod: Rod, hinges: &'static [usize], crush: Crush) -> Sketch {
    Sketch { rod, hinges, crush }
}

fn members(config: &str) -> &'static [(f64, f64, bool)] {
    match config {
        "tt_double" => &[(0.04, 0.3, false), (0.3, 0.7, false), (0.7, 0.96, false)],
        "single" => &[(0.38, 0.5, true), (0.5, 0.96, false)],
        "double_center" => &[(0.04, 0.4, false), (0.4, 0.6, true), (0.6, 0.96, false)],
        "double_outer" => &[(0.04, 0.16, true), (0.16, 0.84, false), (0.84, 0.96, true)],
        _ => &[(0.04, 0.5, false), (0.5, 0.96, false)],
    }
}

fn sketch(config: &str, letter: &str) -> Option<Sketch> {
    Some(match (config, letter) {
        ("tt_single", "a") => sk(&[(0.04, 0.9), (0.96, -0.1)], &[], &[(0.04, 0.5, 1.0)]),
        ("tt_single", "b") => sk(&[(0.04, 0.1), (0.96, -0.9)], &[], &[(0.5, 0.96, -1.0)]),
        ("tt_single", "c") => sk(&[(0.04, 1.0), (0.96, -1.0)], &[], &[(0.04, 0.5, 1.0), (0.5, 0.96, -1.0)]),
        ("tt_single", "d") => sk(&[(0.04, 0.55), (0.27, 0.05), (0.96, -1.0)], &[1], &[(0.27, 0.5, 1.0), (0.5, 0.96, -1.0)]),
        ("tt_single", "e") => sk(&[(0.04, 1.0), (0.73, -0.05), (0.96, -0.55)], &[1], &[(0.04, 0.5, 1.0), (0.5, 0.73, -1.0)]),
        ("tt_single", "f") => sk(&[(0.04, 0.6), (0.27, 0.05), (0.73, -0.05), (0.96, -0.6)], &[1, 2], &[(0.27, 0.5, 1.0), (0.5, 0.73, -1.0)]),
        ("tt_double", "g") => sk(&[(0.04, 0.6), (0.96, 0.6)], &[], &[(0.04, 0.3, 1.0), (0.7, 0.96, 1.0)]),
        ("tt_double", "h") => sk(&[(0.04, -0.6), (0.96, -0.6)], &[], &[(0.3, 0.7, -1.0)]),
        ("tt_double", "j") => sk(&[(0.04, 0.1), (0.17, 0.6), (0.83, 0.6), (0.96, 0.1)], &[1, 2], &[(0.17, 0.3, 1.0), (0.7, 0.83, 1.0)]),
        ("tt_double", "k") => sk(&[(0.04, 0.1), (0.17, 0.5), (0.3, 0.6), (0.7, 0.6), (0.83, 0.5), (0.96, 0.1)], &[1, 2, 3, 4], &[(0.17, 0.3, 1.0), (0.7, 0.83, 1.0)]),
        ("single", "a") => sk(&[(0.38, 0.5), (0.96, -0.7)], &[], &[(0.5, 0.96, -1.0)]),
        ("single", "b") => sk(&[(0.38, 0.4), (0.62, -0.05), (0.96, -0.7)], &[1], &[(0.5, 0.96, -1.0)]),
        ("single", "c") => sk(&[(0.38, 0.9), (0.96, -0.9)], &[], &[(0.5, 0.96, -1.0)]),
        ("single", "d" | "interpolated") => sk(&[(0.38, 0.4), (0.62, -0.05), (0.96, -0.9)], &[1], &[(0.5, 0.96, -1.0)]),
        ("single", "e") => sk(&[(0.38, 0.3), (0.44, 0.1), (0.7, -0.05), (0.96, -0.5)], &[1, 2], &[(0.5, 0.96, -1.0)]),
        ("double_center", "f") => sk(&[(0.04, 0.6), (0.96, 0.6)], &[], &[(0.04, 0.4, 1.0), (0.6, 0.96, 1.0)]),
        ("double_center", "g") => sk(&[(0.04, 0.1), (0.25, 0.55), (0.75, 0.55), (0.96, 0.1)], &[1, 2], &[(0.04, 0.4, 1.0), (0.6, 0.96, 1.0)]),
        ("double_center", "h") => sk(&[(0.04, 0.1), (0.17, 0.4), (0.4, 0.6), (0.6, 0.6), (0.83, 0.4), (0.96, 0.1)], &[1, 2, 3, 4], &[(0.04, 0.4, 1.0), (0.6, 0.96, 1.0)]),
        ("double_outer", "j" | "l") => sk(&[(0.04, 0.6), (0.96, 0.6)], &[], &[(0.16, 0.84, 1.0)]),
        ("double_outer", "k") => sk(&[(0.04, 0.1), (0.3, 0.55), (0.7, 0.55), (0.96, 0.1)], &[1, 2], &[(0.3, 0.7, 1.0)]),
        ("double_outer", "m") => sk(&[(0.04, 0.1), (0.2, 0.4), (0.35, 0.6), (0.65, 0.6), (0.8, 0.4), (0.96, 0.1)], &[1, 2, 3, 4], &[(0.35, 0.65, 1.0)]),
        _ => return None,
    })
}

fn rod_at(rod: Rod, u: f64) -> f64 {
    rod.windows(2).find(|w| u >= w[0].0 && u <= w[1].0).map_or(rod.last().map_or(0.0, |p| p.1), |w| w[0].1 + (w[1].1 - w[0].1) * (u - w[0].0) / (w[1].0 - w[0].0))
}

fn rect(x0: f64, x1: f64, y0: f64, y1: f64) -> Vec<Vec2> {
    vec![Vec2::new(x0, y0), Vec2::new(x1, y0), Vec2::new(x1, y1), Vec2::new(x0, y1)]
}

fn tone_for(check: Option<&crate::norm::CheckResult>) -> Tone {
    check.map_or(Tone::Info, |k| utilisation_tone(k.utilization))
}

/// 🔩️ The failure-mode sketches of one connection configuration with its modes (letter, F_v,Rk per shear plane in N); `None` when no mode has a sketch.
fn build(id: String, label: (&str, &str), config: &str, modes: &[(&'static str, f64)], governing: usize, check: Option<&crate::norm::CheckResult>, focus: Option<EntityRef>) -> Option<Diagram2d> {
    let cells: Vec<(usize, &(&'static str, f64), Sketch)> = modes.iter().enumerate().filter_map(|(i, m)| sketch(config, m.0).map(|s| (i, m, s))).collect();
    if cells.is_empty() {
        return None;
    }
    let tone = tone_for(check);
    let n = cells.len() as f64;
    let (mut series, mut markers, mut notes) = (Vec::new(), Vec::new(), Vec::new());
    for (cell, (index, (letter, rk), s)) in cells.iter().enumerate() {
        let x = cell as f64;
        let is_governing = index == &governing;
        series.push(Series::Polygon { id: format!("cell-{letter}"), label: loc(&format!("Mode ({letter})"), &format!("Form ({letter})")), ring: rect(x + 0.01, x + 0.99, 0.02, 1.2), fill: Fill { tone: if is_governing { tone } else { Tone::Neutral }, opacity: if is_governing { 0.3 } else { 0.05 } }, hatch: None });
        for (k, (u0, u1, steel)) in members(config).iter().enumerate() {
            series.push(Series::Polygon { id: format!("member-{letter}-{k}"), label: loc(if *steel { "Steel plate" } else { "Timber member" }, if *steel { "Stahlblech" } else { "Holzbauteil" }), ring: rect(x + u0, x + u1, 0.15, 1.05), fill: Fill { tone: if *steel { Tone::Info } else { Tone::Neutral }, opacity: 0.28 }, hatch: None });
        }
        for (k, (u0, u1, side)) in s.crush.iter().enumerate() {
            let (y0, y1) = (AXIS + AMPLITUDE * rod_at(s.rod, *u0), AXIS + AMPLITUDE * rod_at(s.rod, *u1));
            series.push(Series::Polygon { id: format!("crush-{letter}-{k}"), label: loc("Embedment failure", "Lochleibungsversagen"), ring: vec![Vec2::new(x + u0, y0), Vec2::new(x + u1, y1), Vec2::new(x + u1, y1 + side * CRUSH), Vec2::new(x + u0, y0 + side * CRUSH)], fill: Fill { tone: Tone::Error, opacity: 0.6 }, hatch: Some(Hatch::Diagonal) });
        }
        let rod: Vec<Vec2> = s.rod.iter().map(|(u, w)| Vec2::new(x + u, AXIS + AMPLITUDE * w)).collect();
        series.push(Series::Polyline { id: format!("rod-{letter}"), label: loc(&format!("Fastener in mode ({letter})"), &format!("Verbindungsmittel in Form ({letter})")), points: rod.clone(), style: stroke(if is_governing { tone } else { Tone::Neutral }, if is_governing { 3.0 } else { 1.6 }, Dash::Solid) });
        for (k, h) in s.hinges.iter().enumerate() {
            markers.push(Marker { id: format!("reference:hinge-{letter}-{k}"), at: rod[*h].clone(), shape: MarkerShape::Circle, tone: Tone::Error, label: loc(&format!("Plastic hinge in mode ({letter})"), &format!("Fließgelenk in Form ({letter})")), utilisation: None, bound_check: None, binding: None });
        }
        notes.push(DiagramNote { id: format!("note-{letter}"), at: Vec2::new(x + 0.5, 0.08), text: loc(&format!("({letter}) {:.2} kN", rk / 1e3), &format!("({letter}) {:.2} kN", rk / 1e3)), tone: if is_governing { tone } else { Tone::Neutral } });
        if is_governing {
            markers.push(Marker { id: "marker-governing".into(), at: Vec2::new(x + 0.5, 1.15), shape: MarkerShape::Triangle, tone, label: loc(&format!("Governing mode ({letter}): {:.2} kN", rk / 1e3), &format!("Maßgebende Form ({letter}): {:.2} kN", rk / 1e3)), utilisation: check.map(|k| k.utilization), bound_check: check.map(|k| k.id.clone()), binding: None });
        }
    }
    Some(Diagram2d {
        id,
        title: loc(&format!("Failure modes of the fastener – {}", label.0), &format!("Versagensformen des Verbindungsmittels – {}", label.1)),
        description: loc("Schematic sketches of the failure modes of the European Yield Model (EN 1995-1-1, Figures 8.2 and 8.3): crushed embedment zones hatched, plastic hinges as dots; the governing mode is highlighted.", "Schematische Skizzen der Versagensformen des Europäischen Fließgelenkmodells (EN 1995-1-1, Bilder 8.2 und 8.3): Lochleibungszonen schraffiert, Fließgelenke als Punkte; die maßgebende Form ist hervorgehoben."),
        axes: vec![axis("Failure mode", "Versagensform", QuantityKind::Dimensionless, "1", Some((0.0, n)), TickRule::Explicit { values: (0..cells.len()).map(|i| i as f64 + 0.5).collect() }), axis("Lateral deformation (schematic)", "Seitliche Verformung (schematisch)", QuantityKind::Dimensionless, "1", Some((0.0, 1.25)), TickRule::Explicit { values: vec![0.0] })],
        series,
        markers,
        annotations: notes,
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Mode position", "Position der Form", "1", (1..=cells.len()).map(|i| i as f64).collect()), column("Capacity F_v,Rk", "Tragfähigkeit F_v,Rk", "kN", cells.iter().map(|(_, m, _)| m.1 / 1e3).collect()), column("Plastic hinges", "Fließgelenke", "1", cells.iter().map(|(_, _, s)| s.hinges.len() as f64).collect())] },
    })
}

/// 🔩️ The sketches of a connection row.
pub(super) fn connection(report: &CheckReport, c: &TimberConnection) -> Option<Diagram2d> {
    let j = facts::johansen(c)?;
    build(format!("failure-modes:{}", c.id), (c.label_en.as_str(), c.label_de.as_str()), j.config, &j.modes, j.governing, worst(report, &c.id, "en1995.8.2"), focus_connection(c))
}

/// 🔩️ The sketches of a T-71, T-72 or T-73 case.
pub(super) fn case(document: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    if !matches!(c.entry_id.as_str(), "T-71" | "T-72" | "T-73") {
        return None;
    }
    let input = t::case_input(document.annex, &DesignCase::from(c));
    let (config, modes) = t::case_modes(&c.entry_id, &input).ok()?;
    let governing = modes.iter().enumerate().min_by(|a, b| a.1 .1.total_cmp(&b.1 .1)).map(|(i, _)| i)?;
    build(format!("failure-modes:{}", c.id), (c.label_en.as_str(), c.label_de.as_str()), &config, &modes, governing, worst(report, &c.id, "en1995.t-7"), focus_case(c))
}
