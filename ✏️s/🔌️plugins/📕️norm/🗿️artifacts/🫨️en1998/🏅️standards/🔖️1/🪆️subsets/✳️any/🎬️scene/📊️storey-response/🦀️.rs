//! 📊️ Storey response: the storey shear and the interstorey drift of every storey per horizontal direction over the elevation, each against its limit (shear resistance of the storey, damage limitation drift of EN 1998-1, 4.4.3.2), derived from the same response and checks as the report so picture and verdict cannot disagree.

use super::model::{En1998Building, En1998Snapshot};
use super::norm::CheckReport;
use super::response::{respond, site, Response};
use super::scene::projection_kit::diagram_parts::{self, column, stroke};
use super::scene::projection_kit::loc;
use super::scene::*;
use super::schema::part_1;

const MM: f64 = 1000.0;
const KN: f64 = 1000.0;

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, None)
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn direction(along_y: bool) -> &'static str {
    if along_y { "y" } else { "x" }
}

/// 📊️ The storey shear `V_i = Σ_{j ≥ i} F_j` of every storey of one force vector, bottom up.
pub fn storey_shears(forces: &[f64]) -> Vec<f64> {
    let mut sum = 0.0;
    let mut shears: Vec<f64> = forces.iter().rev().map(|f| { sum += f; sum }).collect();
    shears.reverse();
    shears
}

/// 📊️ The interstorey drift limit `θ·h/ν` of every storey [m] (damage limitation, EN 1998-1 4.4.3.2).
pub fn drift_limits_m(building: &En1998Building) -> Vec<f64> {
    let theta = part_1::drift_theta(building.drift_limit_class);
    building.storeys.iter().map(|s| theta * s.height_m / building.nu.max(1e-9)).collect()
}

fn bottoms(response: &Response) -> Vec<f64> {
    response.elevations.iter().zip(&response.heights).map(|(z, h)| z - h).collect()
}

fn focus(building: &En1998Building) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Group, &building.id, format!("buildings[id={}]", building.id)))
}

fn storey_check(report: &CheckReport, building: &str, tail: &str) -> Option<(String, f64)> {
    report.checks.iter().find(|c| c.subject.entity_id == building && c.id.ends_with(tail)).map(|c| (c.id.clone(), if c.utilization.is_finite() { c.utilization } else { 0.0 }))
}

/// 📊️ Diagram `storey-shear:<building>`: the storey shear as a step profile per direction over the elevation with the storey shear resistances as limit steps; each storey midpoint carries the storeys force check.
pub fn storey_shear(document: &En1998Snapshot, report: &CheckReport, building: &En1998Building) -> Option<Diagram2d> {
    let response = respond(&site(document), building);
    let bottom = bottoms(&response);
    let mid: Vec<f64> = bottom.iter().zip(&response.elevations).map(|(b, t)| 0.5 * (b + t)).collect();
    let steps = |values: &[f64]| -> Vec<Vec2> { values.iter().enumerate().flat_map(|(i, v)| [Vec2::new(*v / KN, bottom[i]), Vec2::new(*v / KN, response.elevations[i])]).collect() };
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut columns = vec![column("Elevation z", "H\u{f6}he z", "m", mid.clone())];
    for system in response.systems.iter().filter(|s| s.forces.len() == mid.len() && !mid.is_empty()) {
        let shears = storey_shears(&system.forces);
        let dir = direction(system.along_y);
        series.push(Series::Polyline { id: format!("shear-{}", system.id), label: loc(&format!("Storey shear {dir}, system {}", system.id), &format!("Geschossquerkraft {dir}, System {}", system.id)), points: steps(&shears), style: stroke(if system.along_y { Tone::Warning } else { Tone::Info }, 2.0, if system.along_y { Dash::Dashed } else { Dash::Solid }) });
        for (index, storey) in building.storeys.iter().enumerate() {
            let checked = storey_check(report, &building.id, &format!(".storeyForces.{}", storey.id));
            let ratio = if storey.shear_resistance_n > 0.0 { shears[index] / storey.shear_resistance_n } else { checked.as_ref().map_or(0.0, |c| c.1) };
            markers.push(Marker { id: format!("shear-{}-{}", system.id, storey.id), at: Vec2::new(shears[index] / KN, mid[index]), shape: MarkerShape::Circle, tone: if storey.shear_resistance_n > 0.0 || checked.is_some() { tone(ratio) } else { Tone::Neutral }, label: loc(&format!("{} ({dir}): V = {:.0} kN", storey.id, shears[index] / KN), &format!("{} ({dir}): V = {:.0} kN", storey.id, shears[index] / KN)), utilisation: checked.as_ref().map(|c| c.1), bound_check: checked.map(|c| c.0), binding: None });
        }
        columns.push(column(&format!("Storey shear {dir}, system {}", system.id), &format!("Geschossquerkraft {dir}, System {}", system.id), "kN", shears.iter().map(|v| v / KN).collect()));
    }
    if series.is_empty() {
        return None;
    }
    if building.storeys.iter().any(|s| s.shear_resistance_n > 0.0) {
        let resistance: Vec<f64> = building.storeys.iter().map(|s| s.shear_resistance_n).collect();
        series.push(Series::Polyline { id: "resistance".into(), label: loc("Storey shear resistance", "Geschossquerkrafttragf\u{e4}higkeit"), points: steps(&resistance), style: stroke(Tone::Error, 1.5, Dash::Dashed) });
        columns.push(column("Shear resistance", "Querkrafttragf\u{e4}higkeit", "kN", resistance.iter().map(|v| v / KN).collect()));
    }
    Some(Diagram2d {
        id: format!("storey-shear:{}", building.id),
        title: loc(&format!("Storey shear \u{2013} {}", building.name), &format!("Geschossquerkraft \u{2013} {}", building.name)),
        description: loc("Storey shear V_i = \u{3a3} F_j of the lateral force method per horizontal direction over the elevation (EN 1998-1, 4.3.3.2.3) against the storey shear resistance where entered.", "Geschossquerkraft V_i = \u{3a3} F_j des Ersatzkraftverfahrens je Horizontalrichtung \u{fc}ber der H\u{f6}he (EN 1998-1, 4.3.3.2.3) gegen die eingegebene Geschossquerkrafttragf\u{e4}higkeit."),
        axes: vec![axis("Storey shear V", "Geschossquerkraft V", QuantityKind::Force, "kN"), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m")],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(building),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 📊️ Diagram `storey-drift:<building>`: the interstorey drift d_r of every storey per direction over the elevation against the damage limitation limit θ·h/ν; the markers are draggable and write the drift inputs of their storey.
pub fn storey_drift(document: &En1998Snapshot, report: &CheckReport, building: &En1998Building) -> Option<Diagram2d> {
    let response = respond(&site(document), building);
    let limits = drift_limits_m(building);
    let mid: Vec<f64> = bottoms(&response).iter().zip(&response.elevations).map(|(b, t)| 0.5 * (b + t)).collect();
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut columns = vec![column("Elevation z", "H\u{f6}he z", "m", mid.clone())];
    for system in &response.systems {
        let field = if system.along_y { "driftYM" } else { "driftXM" };
        let dir = direction(system.along_y);
        let mut points = Vec::new();
        let mut values = Vec::new();
        for (index, storey) in building.storeys.iter().enumerate() {
            let check = format!("en1998.1.{}.{}.drift.{}", building.id, system.id, storey.id);
            let Some(c) = report.checks.iter().find(|c| c.id == check) else { continue };
            let util = if c.utilization.is_finite() { c.utilization } else { 0.0 };
            let drift_mm = util * limits[index] * MM;
            points.push(Vec2::new(drift_mm, mid[index]));
            values.push(drift_mm);
            let mut binding = FieldBinding::new(format!("buildings[id={}].storeys[id={}].{field}", building.id, storey.id), Affine { base: 0.0, k: 1.0 / MM }, "m");
            binding.quantise = Some(Quantise::Round { step: 0.0001 });
            markers.push(Marker { id: format!("drift-{}-{}", system.id, storey.id), at: Vec2::new(drift_mm, mid[index]), shape: MarkerShape::Circle, tone: tone(util), label: loc(&format!("{} ({dir}): d_r = {drift_mm:.1} mm of {:.1} mm", storey.id, limits[index] * MM), &format!("{} ({dir}): d_r = {drift_mm:.1} mm von {:.1} mm", storey.id, limits[index] * MM)), utilisation: Some(util), bound_check: Some(check), binding: Some(binding) });
        }
        if points.len() >= 2 {
            series.push(Series::Polyline { id: format!("drift-{}", system.id), label: loc(&format!("Interstorey drift {dir}, system {}", system.id), &format!("Stockwerksverschiebung {dir}, System {}", system.id)), points, style: stroke(if system.along_y { Tone::Warning } else { Tone::Info }, 2.0, if system.along_y { Dash::Dashed } else { Dash::Solid }) });
        }
        if !values.is_empty() {
            columns.push(column(&format!("d_r {dir}, system {}", system.id), &format!("d_r {dir}, System {}", system.id), "mm", values));
        }
    }
    if series.is_empty() {
        return None;
    }
    series.push(Series::Polyline { id: "limit".into(), label: loc("Damage limitation limit \u{3b8}\u{b7}h/\u{3bd}", "Schadensbegrenzung \u{3b8}\u{b7}h/\u{3bd}"), points: limits.iter().zip(&mid).map(|(l, z)| Vec2::new(l * MM, *z)).collect(), style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    let rows = columns.iter().map(|c| c.values.len()).min().unwrap_or(0);
    columns.iter_mut().for_each(|c| c.values.truncate(rows));
    Some(Diagram2d {
        id: format!("storey-drift:{}", building.id),
        title: loc(&format!("Interstorey drift \u{2013} {}", building.name), &format!("Stockwerksverschiebung \u{2013} {}", building.name)),
        description: loc("Interstorey drift d_r of every storey per horizontal direction over the elevation against the damage limitation limit \u{3b8}\u{b7}h/\u{3bd} (EN 1998-1, 4.4.3.2); the points left of the dashed limit comply.", "Stockwerksverschiebung d_r je Geschoss und Horizontalrichtung \u{fc}ber der H\u{f6}he gegen die Grenze der Schadensbegrenzung \u{3b8}\u{b7}h/\u{3bd} (EN 1998-1, 4.4.3.2); Punkte links der gestrichelten Grenze sind nachgewiesen."),
        axes: vec![axis("Interstorey drift d_r", "Stockwerksverschiebung d_r", QuantityKind::Length, "mm"), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m")],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(building),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 📊️ Both storey response diagrams of one building.
pub fn diagrams(document: &En1998Snapshot, report: &CheckReport, building: &En1998Building) -> Vec<Diagram2d> {
    [storey_shear(document, report, building), storey_drift(document, report, building)].into_iter().flatten().collect()
}
