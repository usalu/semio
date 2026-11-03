//! 📉️ Prestress loss over time: the time-dependent loss of EN 1992-1-1 5.10.6 (catalogue entry PR-05, Eq. 5.46 with creep and shrinkage of Annex B) at the concrete ages of the creep track, as a force diagram, a colour field on the member side and the scrubbable track `prestress-loss`.

use super::checks::{self, Case};
use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::layout::Layout;
use super::mechanics;
use super::model::{En1992Snapshot, MemberKind, RcMember};
use super::scene::projection_kit::diagram_parts::column;
use super::scene::projection_kit::tracks as kit_tracks;
use super::scene::*;
use super::tracks::{creep_phi, notional_size, shrinkage_strain, side_grid, spec, CREEP_DAYS, DAY, DRYING_START_DAYS, LOADING_AGE_DAYS, RELATIVE_HUMIDITY};
use super::SceneInput;

pub const FIELD_PREFIX: &str = "prestress@";
pub const TRACK: &str = "prestress-loss";
const LIFT: f64 = 0.012;

/// 📊️ One age of the loss series: the relaxation loss, the total time-dependent loss in MPa and the remaining force in N.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LossRow {
    pub days: f64,
    pub relaxation_mpa: f64,
    pub loss_mpa: f64,
    pub force_n: f64,
}

fn width_of(m: &RcMember) -> f64 {
    if m.kind == MemberKind::FlatSlab { m.span } else { m.width }
}

fn outcome(outcomes: &[checks::Outcome], key: &str) -> Option<f64> {
    outcomes.iter().find(|o| o.key == key).map(|o| o.capacity / 1.0e6)
}

/// 🧮️ The loss at every age of the creep track through the catalogue entry PR-05; `None` without prestress, steel, concrete or section. The concrete stress at the tendon is the one of the initial force alone, bending is not combined.
pub fn loss_series(document: &En1992Snapshot, m: &RcMember) -> Option<Vec<LossRow>> {
    let ps = m.prestress.as_ref().filter(|p| p.area > 0.0 && p.force > 0.0)?;
    let steel = document.prestress_steels.iter().find(|s| s.id == m.prestress_steel_id)?;
    let grade = document.concrete(&m.concrete_grade_id)?;
    let sec = mechanics::member_sec(document, m)?;
    let gross = sec.gross_props();
    if gross.area <= 0.0 || gross.i_y <= 0.0 {
        return None;
    }
    let (fck, fpk) = (grade.f_ck / 1.0e6, steel.f_pk / 1.0e6);
    let (ap, e_mm) = (ps.area * 1.0e6, m.tendon_eccentricity_mid().unwrap_or(ps.eccentricity).abs() * 1.0e3);
    let sigma_c = ps.force / gross.area + ps.force * e_mm * e_mm / gross.i_y;
    let sigma_pi = ps.force / ap;
    let h0 = notional_size(width_of(m), m.height);
    let rows: Vec<LossRow> = CREEP_DAYS
        .iter()
        .filter_map(|&days| {
            let phi = creep_phi(fck + 8.0, RELATIVE_HUMIDITY, h0, LOADING_AGE_DAYS, days).max(1.0e-6);
            let case = Case::new("prestress-loss", document.annex)
                .with_n("ecm", checks::conc(fck).ecm)
                .with_n("phi", phi)
                .with_n("ap", ap)
                .with_n("ac", gross.area)
                .with_n("ic", gross.i_y)
                .with_n("zcp", e_mm)
                .with_n("eps_cs", shrinkage_strain(fck, RELATIVE_HUMIDITY, h0, days, DRYING_START_DAYS))
                .with_n("sigma_c_qp", sigma_c)
                .with_n("sigma_pi", sigma_pi)
                .with_n("fpk", fpk)
                .with_n("t_hours", days * 24.0);
            let outcomes = checks::evaluate_outcomes("PR-05", &case).ok()?;
            let (relaxation_mpa, loss_mpa) = (outcome(&outcomes, "relax")?, outcome(&outcomes, "loss")?);
            Some(LossRow { days, relaxation_mpa, loss_mpa, force_n: ps.force - loss_mpa * ap })
        })
        .collect();
    (rows.len() == CREEP_DAYS.len()).then_some(rows)
}

fn field(m: &RcMember, rows: &[LossRow], pose: &super::layout::MemberPose, out: &mut Out) {
    let p0 = m.prestress.as_ref().map_or(1.0, |p| p.force).max(1.0);
    let (points, grid) = side_grid(&pose.frame, m.span, width_of(m), m.height, LIFT);
    let fields: Vec<ScalarField> = rows.iter().map(|r| ScalarField { id: format!("{FIELD_PREFIX}{}", r.days), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: vec![r.force_n / p0; points.len()] } }).collect();
    out.push(spec(m, "prestress-loss", "prestress", grid, LAYER_PRESTRESS, [("prestress over time", "Vorspannung über der Zeit"), ("remaining prestressing force Pm,t / Pm0 over the concrete age", "verbleibende Vorspannkraft Pm,t / Pm0 über dem Betonalter")], fields), &points);
}

/// 🎬️ Adds the prestress colour field of every prestressed member.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in &input.document.members {
        if !out.wants(&m.id) || m.span <= 1e-6 || m.height <= 1e-6 || m.width <= 1e-6 {
            continue;
        }
        let (Some(pose), Some(rows)) = (layout.member(&m.id), loss_series(input.document, m)) else { continue };
        field(m, &rows, pose, out);
    }
}

/// 🎞️ Adds the track `prestress-loss` once any entity carries the field it scrubs.
pub fn finish(scene: &mut StructuralScene) {
    let on = scene.entities.iter().any(|e| matches!(&e.appearance.channel, ColourChannel::Fields { fields, .. } if fields.iter().any(|f| f.id.starts_with(FIELD_PREFIX))));
    if !on || scene.tracks.iter().any(|t| t.id == TRACK) {
        return;
    }
    let stops: Vec<(f64, String, String)> = CREEP_DAYS.iter().map(|d| (d * DAY, format!("{FIELD_PREFIX}{d}"), format!("{d} d"))).collect();
    let rows: Vec<(f64, &str, &str, &str)> = stops.iter().map(|(at, id, name)| (*at, id.as_str(), name.as_str(), name.as_str())).collect();
    scene.tracks.extend(kit_tracks::timed(TRACK, TrackKind::Age, ("Concrete age (prestressing force)", "Betonalter (Vorspannkraft)"), &rows));
}

/// 📈️ Diagram `prestress-loss:<member>`: the force `Pm,t` over the age against `Pm0`, with the relaxation share of the loss.
pub fn diagram(document: &En1992Snapshot, m: &RcMember, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let rows = loss_series(document, m)?;
    let p0 = m.prestress.as_ref()?.force / 1.0e3;
    let ap = m.prestress.as_ref()?.area * 1.0e6;
    let force: Vec<[f64; 2]> = rows.iter().map(|r| [r.days, r.force_n / 1.0e3]).collect();
    let relaxation: Vec<[f64; 2]> = rows.iter().map(|r| [r.days, p0 - r.relaxation_mpa * ap / 1.0e3]).collect();
    let mut g = base(
        m,
        "prestress-loss",
        titled(m, "Prestress loss over time", "Spannkraftverlust über der Zeit"),
        loc("Prestressing force after the time-dependent losses of EN 1992-1-1 5.10.6 (creep, shrinkage and relaxation, Eq. 5.46) against the concrete age, loaded at 28 days, RH 50 %; the dashed line is the force with the relaxation loss alone. Friction and slip are not modelled in the document.", "Spannkraft nach den zeitabhängigen Verlusten nach EN 1992-1-1 5.10.6 (Kriechen, Schwinden und Relaxation, Gl. 5.46) über dem Betonalter, Belastung mit 28 Tagen, RH 50 %; die gestrichelte Linie ist die Kraft nur mit dem Relaxationsverlust. Reibung und Schlupf sind im Dokument nicht modelliert."),
        vec![axis("Concrete age", "Betonalter", QuantityKind::Time, "d"), axis("Prestressing force", "Spannkraft", QuantityKind::Force, "kN")],
        kind,
        path,
    );
    g.series.push(Series::Limit { id: "p0".into(), label: loc("Pm0", "Pm0"), axis: AxisRef::Y, value: p0, style: stroke(1.0, Dash::Dotted, Tone::Info) });
    g.series.push(Series::Polyline { id: "relaxation".into(), label: loc("Relaxation only", "Nur Relaxation"), points: xy(&relaxation), style: stroke(2.0, Dash::Dashed, Tone::Warning) });
    g.series.push(Series::Polyline { id: "pmt".into(), label: loc("Pm,t", "Pm,t"), points: xy(&force), style: stroke(2.0, Dash::Solid, Tone::Info) });
    g.table_twin = DataTable { columns: vec![column("Age", "Alter", "d", rows.iter().map(|r| r.days).collect()), column("Pm,t", "Pm,t", "kN", force.iter().map(|p| p[1]).collect()), column("Loss", "Verlust", "MPa", rows.iter().map(|r| r.loss_mpa).collect()), column("Relaxation loss", "Relaxationsverlust", "MPa", rows.iter().map(|r| r.relaxation_mpa).collect())] };
    Some(g)
}
