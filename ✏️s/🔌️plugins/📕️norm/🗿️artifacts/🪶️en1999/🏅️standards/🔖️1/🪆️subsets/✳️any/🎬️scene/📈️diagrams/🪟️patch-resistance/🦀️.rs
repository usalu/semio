//! 🪟️ Web crippling diagram of a patch load: the resistance F_Rd over the bearing length s_s (EN 1999-1-1 §6.7.5 for a member web, EN 1999-1-4 §6.1.7 for a sheet web) with the design force F_Ed as a limit and the design point as a marker that drags the bearing length. While a coefficient is unverified (null registry row, load at a member end) only the design force and the point are drawn.

use super::*;
use crate::scene::projection_kit::handle_kit::{identity_binding, round};

const S_MIN: f64 = 0.005;
const S_MAX: f64 = 0.5;

/// 🪟️ The diagram `patch-resistance:<id>` of a patch load, `None` without a usable subject (member or sheet) of the load.
pub(super) fn patch_resistance(d: &En1999Snapshot, report: &CheckReport, load: &PatchLoad) -> Option<Diagram2d> {
    let member = d.members.iter().find(|m| m.id == load.subject_id);
    if member.is_none() && !d.cold_formed.iter().any(|s| s.id == load.subject_id) {
        return None;
    }
    let s_s = if load.bearing_length > 0.0 { load.bearing_length } else { member.map_or(0.0, |m| m.bearing_length) };
    let f_ed = load.load.abs() / 1.0e3;
    let samples: Vec<f64> = (0..=SAMPLES).map(|i| S_MIN + (S_MAX - S_MIN) * i as f64 / SAMPLES as f64).collect();
    let curve: Option<Vec<f64>> = samples.iter().map(|s| checks::patch_resistance_at(load, d, d.annex, *s).map(|r| r.f_rd / 1.0e3)).collect();
    let design = checks::patch_resistance_at(load, d, d.annex, s_s);
    let check = worst(report, &load.id, "en1999.");
    let u = design.map(|r| if r.f_rd > 0.0 { f_ed / (r.f_rd / 1.0e3) } else { f64::INFINITY });
    let tone = u.map_or(Tone::Neutral, utilisation_tone);
    let top = curve.as_ref().map_or(f_ed, |c| c.iter().cloned().fold(f_ed, f64::max)).max(1e-3) * 1.1;
    let clause = if member.is_some() { "EN 1999-1-1, 6.7.5" } else { "EN 1999-1-4, 6.1.7" };
    let mut series = Vec::new();
    if let Some(c) = &curve {
        series.push(Series::Polyline { id: "f-rd".into(), label: loc("Resistance F_Rd", "Widerstand F_Rd"), points: samples.iter().zip(c).map(|(s, r)| Vec2::new(*s, *r)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) });
    }
    series.push(Series::Limit { id: "f-ed".into(), label: loc("Design force F_Ed", "Bemessungskraft F_Ed"), axis: AxisRef::Y, value: f_ed, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    let point_y = design.map_or(f_ed, |r| r.f_rd / 1.0e3);
    let label = match (design, u) {
        (Some(r), Some(u)) => loc(&format!("Design point: s_s = {:.0} mm, F_Rd = {:.1} kN, F_Ed = {f_ed:.1} kN, utilisation {u:.2} (coefficients unverified)", s_s * 1e3, r.f_rd / 1e3), &format!("Bemessungspunkt: s_s = {:.0} mm, F_Rd = {:.1} kN, F_Ed = {f_ed:.1} kN, Ausnutzung {u:.2} (Beiwerte ungeprüft)", s_s * 1e3, r.f_rd / 1e3)),
        _ => loc(&format!("s_s = {:.0} mm, F_Ed = {f_ed:.1} kN; no resistance until the coefficients ({clause}) are verified", s_s * 1e3), &format!("s_s = {:.0} mm, F_Ed = {f_ed:.1} kN; kein Widerstand, bis die Beiwerte ({clause}) geprüft sind", s_s * 1e3)),
    };
    let marker = Marker {
        id: "design".into(),
        at: Vec2::new(s_s.clamp(S_MIN, S_MAX), point_y.min(top)),
        shape: MarkerShape::Circle,
        tone,
        label,
        utilisation: u,
        bound_check: check.map(|c| c.id.clone()),
        binding: Some(identity_binding(format!("patchLoads[id={}].bearingLength", load.id), "m", round(0.005))),
    };
    let notes = if curve.is_none() { vec![DiagramNote { id: "awaiting".into(), at: Vec2::new(0.5 * (S_MIN + S_MAX), f_ed), text: loc("Awaiting verified coefficients", "Wartet auf geprüfte Beiwerte"), tone: Tone::Warning }] } else { Vec::new() };
    let mut columns = vec![column("Bearing length s_s", "Lasteinleitungslänge s_s", "m", samples.clone())];
    if let Some(c) = &curve {
        columns.push(column("Resistance F_Rd", "Widerstand F_Rd", "kN", c.clone()));
    }
    columns.push(column("Design force F_Ed", "Bemessungskraft F_Ed", "kN", vec![f_ed; samples.len()]));
    Some(Diagram2d {
        id: format!("patch-resistance:{}", load.id),
        title: loc(&format!("Web crippling over the bearing length – {}", load.id), &format!("Stegkrüppeln über der Lasteinleitungslänge – {}", load.id)),
        description: loc(
            &format!("Resistance F_Rd of the web against the transverse force over the stiff bearing length s_s ({clause}, load position {:?}); the circle is the design point, drag it to change s_s and the zone in the 3D view follows. Coefficients unverified.", load.position),
            &format!("Widerstand F_Rd des Stegs gegen die Querkraft über der Lasteinleitungslänge s_s ({clause}, Lastlage {:?}); der Kreis ist der Bemessungspunkt, ziehen ändert s_s und die Zone in der 3D-Ansicht folgt. Beiwerte ungeprüft.", load.position),
        ),
        axes: vec![axis("Bearing length s_s", "Lasteinleitungslänge s_s", QuantityKind::Length, "m", AxisScale::Linear, Some((S_MIN, S_MAX))), axis("Transverse force", "Querkraft", QuantityKind::Force, "kN", AxisScale::Linear, Some((0.0, top)))],
        series,
        markers: vec![marker],
        annotations: notes,
        focus: Some(EntityRef::new(EntityKind::Load, &load.id, format!("patchLoads[id={}]", load.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}
