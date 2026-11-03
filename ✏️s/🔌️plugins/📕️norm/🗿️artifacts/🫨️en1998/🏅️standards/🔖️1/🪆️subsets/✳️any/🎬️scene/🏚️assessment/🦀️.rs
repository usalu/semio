//! 🏚️ Assessment diagram `assessment-dc:<assessment>`: the demand over capacity D/C of the ductile (chord rotation, θ_Ed/θ_Rd) and the brittle mechanism (cyclic shear, E_d/R_d) of an assessed member against the limit D/C = 1 of EN 1998-3 4.4, with both demand markers draggable onto the demand fields. See [`semio_s_artifact_norm_en1998_schema::part_3_state`].

use super::model::En1998Snapshot;
use super::response::annex_params;
use super::scene::projection_kit::diagram_parts::{self, column, stroke};
use super::scene::projection_kit::loc;
use super::scene::*;
use super::schema::part_3_state;
use super::norm::CheckReport;

const SHEAR_STEP_N: f64 = 1000.0;
const ANGLE_STEP_RAD: f64 = 0.0001;

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn bound(path: &str, capacity: f64, unit: &str, step: f64) -> Option<FieldBinding> {
    (capacity > 0.0).then(|| {
        let mut binding = FieldBinding::new(path.to_owned(), Affine { base: 0.0, k: capacity }, unit);
        binding.quantise = Some(Quantise::Round { step });
        binding
    })
}

/// 🏚️ One diagram per assessment with a section or a demand: the points are the D/C values of the mechanisms present, the markers carry the check they belong to and the field a drag writes (θ_Ed = D/C·θ_Rd, V_Ed = D/C·V_R).
pub fn diagrams(document: &En1998Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    let annex = annex_params(document).choice();
    document.assessments.iter().filter_map(|a| {
        let ratios = part_3_state::member_ratios(a, annex);
        let basis = part_3_state::basis(a, annex);
        if ratios.chord.is_none() && ratios.shear.is_none() && ratios.shear_capacity_n <= 0.0 {
            return None;
        }
        let path = format!("assessments[id={}]", a.id);
        let find = |suffix: &str| report.checks.iter().find(|c| c.id == format!("en1998.3.{}.{suffix}", a.id));
        let marker = |id: &str, x: f64, ratio: f64, suffix: &str, label: LocalizedCopy, binding: Option<FieldBinding>| {
            let check = find(suffix);
            Marker { id: format!("{id}-{}", a.id), at: Vec2::new(x, ratio), shape: MarkerShape::Square, tone: if ratio.is_finite() { tone(ratio) } else { Tone::Error }, label, utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite()), bound_check: check.map(|c| c.id.clone()), binding }
        };
        let mut points = Vec::new();
        let mut markers = Vec::new();
        if let Some(ratio) = ratios.chord.filter(|r| r.is_finite()) {
            points.push(Vec2::new(1.0, ratio));
            markers.push(marker("chord", 1.0, ratio, "chordRotation", loc(&format!("Ductile: \u{3b8}Ed {:.4} rad / \u{3b8}Rd {:.4} rad = {ratio:.2}", a.chord_rotation_demand_rad, ratios.chord_capacity_rad), &format!("Duktil: \u{3b8}Ed {:.4} rad / \u{3b8}Rd {:.4} rad = {ratio:.2}", a.chord_rotation_demand_rad, ratios.chord_capacity_rad)), bound(&format!("{path}.chordRotationDemandRad"), ratios.chord_capacity_rad, "rad", ANGLE_STEP_RAD)));
        }
        if ratios.shear_capacity_n > 0.0 {
            let ratio = ratios.shear.unwrap_or(0.0);
            if ratio.is_finite() {
                points.push(Vec2::new(2.0, ratio));
                markers.push(marker("shear", 2.0, ratio, "cyclicShear", loc(&format!("Brittle: Ed {:.0} N / Rd {:.0} N = {ratio:.2}", a.shear_force_demand_n, ratios.shear_capacity_n), &format!("Spr\u{f6}d: Ed {:.0} N / Rd {:.0} N = {ratio:.2}", a.shear_force_demand_n, ratios.shear_capacity_n)), bound(&format!("{path}.shearForceDemandN"), ratios.shear_capacity_n, "N", SHEAR_STEP_N)));
            }
        }
        if points.is_empty() {
            return None;
        }
        let columns = vec![column("Mechanism (1 ductile, 2 brittle)", "Mechanismus (1 duktil, 2 spr\u{f6}d)", "1", points.iter().map(|p| p.x).collect()), column("D/C", "D/C", "1", points.iter().map(|p| p.y).collect())];
        Some(Diagram2d {
            id: format!("assessment-dc:{}", a.id),
            title: loc(&format!("Capacity and demand \u{2013} {} ({}, CF {:.2}, T_R {:.0} yr)", a.id, part_3_state::knowledge_level_token(a.knowledge_level).to_uppercase(), basis.cf, basis.return_period_yr), &format!("Kapazit\u{e4}t und Einwirkung \u{2013} {} ({}, CF {:.2}, T_R {:.0} a)", a.id, part_3_state::knowledge_level_token(a.knowledge_level).to_uppercase(), basis.cf, basis.return_period_yr)),
            description: loc("Demand over capacity of the ductile mechanism (chord rotation of the limit state, mean strengths over CF) and of the brittle mechanism (cyclic shear capacity, strengths over CF and the partial factor) of the assessed member; both must stay at or below 1 (EN 1998-3 4.4). Drag a marker to change the demand.", "Beanspruchung zu Kapazit\u{e4}t des duktilen Mechanismus (Sehnendrehwinkel des Grenzzustands, mittlere Festigkeiten \u{fc}ber CF) und des spr\u{f6}den Mechanismus (zyklische Querkrafttragf\u{e4}higkeit, Festigkeiten \u{fc}ber CF und Teilsicherheitsbeiwert) des bewerteten Bauteils; beide m\u{fc}ssen h\u{f6}chstens 1 bleiben (EN 1998-3 4.4). Marker ziehen, um die Beanspruchung zu \u{e4}ndern."),
            axes: vec![diagram_parts::axis("Mechanism", "Mechanismus", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, 2.5))), diagram_parts::axis("Demand over capacity", "Beanspruchung zu Kapazit\u{e4}t", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)],
            series: vec![
                Series::Polyline { id: "ratio".into(), label: loc("D/C", "D/C"), points, style: stroke(Tone::Info, 2.0, Dash::Solid) },
                Series::Limit { id: "limit".into(), label: loc("Limit D/C = 1", "Grenze D/C = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
            ],
            markers,
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Group, &a.id, path)),
            legend: true,
            table_twin: DataTable { columns },
        })
    }).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
