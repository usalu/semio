//! 🛢️ Liquid-retaining wall (EN 1992-3): the hydrostatic pressure over the retained head h_D = (h_D/h) h of the wall, coloured by the utilisation of the crack-width check of the tightness class.

use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::model::RcMember;
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

const WATER_UNIT_WEIGHT: f64 = 9.81;
const SAMPLES: usize = 11;

/// 📈️ Diagram `liquid-pressure:<member>`: the hydrostatic pressure p(z) = γ_liq (h_D - z) in kPa over the height z above the wall foot (γ_liq = 9.81 kN/m³ for water, a physical constant), with the head h_D = (h_D/h) h from the member's `hdOverH` and thickness; `None` without a tightness class or head ratio.
pub fn diagram(input: &SceneInput, m: &RcMember, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let class = m.tightness?;
    if m.hd_over_h <= 0.0 || m.height <= 0.0 {
        return None;
    }
    let head = m.hd_over_h * m.height;
    let curve: Vec<[f64; 2]> = (0..SAMPLES).map(|i| head * i as f64 / (SAMPLES - 1) as f64).map(|z| [WATER_UNIT_WEIGHT * (head - z), z]).collect();
    let check = input.report.checks.iter().filter(|c| c.subject.entity_id == m.id && c.id.starts_with("en1992-3.crack.")).max_by(|a, b| a.utilization.total_cmp(&b.utilization));
    let tone = check.map_or(Tone::Info, |c| super::scene::utilisation_tone(c.utilization));
    let mut g = base(
        m,
        "liquid-pressure",
        titled(m, "Liquid pressure on the wall", "Flüssigkeitsdruck auf die Wand"),
        loc(&format!("Hydrostatic pressure on the liquid-retaining wall (EN 1992-3, tightness class {class:?}) over the head h_D = (h_D/h) h; the colour follows the utilisation of the crack-width check against the class limit."), &format!("Hydrostatischer Druck auf die Flüssigkeitsbehälter-Wand (EN 1992-3, Dichtheitsklasse {class:?}) über die Druckhöhe h_D = (h_D/h) h; die Farbe folgt der Ausnutzung des Rissbreitennachweises gegen den Klassengrenzwert.")),
        vec![axis("Pressure p", "Druck p", QuantityKind::Stress, "kPa"), axis("Height above foot z", "Höhe über Fuß z", QuantityKind::Length, "m")],
        kind,
        path,
    );
    g.series.push(Series::Polyline { id: "pressure".into(), label: loc("Hydrostatic pressure", "Hydrostatischer Druck"), points: xy(&curve), style: stroke(2.0, Dash::Solid, tone) });
    g.markers.push(Marker { id: "foot".into(), at: Vec2::new(WATER_UNIT_WEIGHT * head, 0.0), shape: MarkerShape::Diamond, tone, label: loc(&format!("Foot pressure {:.1} kPa at h_D = {head:.2} m", WATER_UNIT_WEIGHT * head), &format!("Fußdruck {:.1} kPa bei h_D = {head:.2} m", WATER_UNIT_WEIGHT * head)), utilisation: check.map(|c| c.utilization), bound_check: check.map(|c| c.id.clone()), binding: None });
    g.table_twin = DataTable { columns: vec![column("Height z", "Höhe z", "m", curve.iter().map(|p| p[1]).collect()), column("Pressure p", "Druck p", "kPa", curve.iter().map(|p| p[0]).collect())] };
    Some(g)
}
