//! 🔩️ Joint row checks: one check per explicit bolt row of an end plate joint against the isolated row resistance of its weakest component. The component model lives in `joint_row_model`. See [EN 1993-1-8 §6](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::joint_components::{self as jc, RowGeom};
use semio_s_artifact_norm_en1993_model::JointRow;

pub use joint_row_model::{component_result, joint_row_summary, rows_of, JointRowSummary};
use joint_row_model::{explicit, geometry_of};

/// 🔩️ Elastic distribution of the joint moment over the rows, F_r,Ed = M_j,Ed h_r/Σ h² (N, mm).
fn row_demands(levers_mm: &[f64], m_ed_nm: f64) -> Vec<f64> {
    let sum: f64 = levers_mm.iter().map(|h| h * h).sum();
    levers_mm.iter().map(|h| m_ed_nm * 1e3 * h / sum.max(1e-9)).collect()
}

fn subject(row: &JointRow, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&row.id, format!("jointRows[id={}].{field}", row.id), loc(en, de))
}

/// 🔩️ Appends one check per explicit bolt row: the elastic row force of M_j,Ed against the isolated row resistance of the weakest component, with the end plate thickness as remedy.
pub(super) fn check_joint_rows(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for j in document.joints.iter().filter(|j| j.kind == semio_s_artifact_norm_en1993_model::JointKind::EndPlate && j.plate_thickness > 0.0 && j.column_flange_thickness > 0.0 && j.compression_lever > 0.0) {
        let entities = explicit(document, j);
        if entities.is_empty() {
            continue;
        }
        let rows: Vec<RowGeom> = entities.iter().map(|r| geometry_of(r, params)).collect();
        let tp = j.plate_thickness * 1e3;
        let result = component_result(j, &rows, tp, params);
        let demands = row_demands(&rows.iter().map(|r| r.lever).collect::<Vec<_>>(), j.moment_demand);
        for (index, row) in entities.iter().enumerate().filter(|(_, r)| scope.admits("jointRows", &r.id)) {
            let (res, demand) = (&result.rows[index], demands[index]);
            let explanation = loc(
                &format!("F_t,fc,Rd={:.0} kN (mode {}), F_t,ep,Rd={:.0} kN (mode {}), F_t,wc,Rd={:.0} kN; governing: {}; plastic distribution F_tr,Rd={:.0} kN; elastic F_tr,Ed={:.0} kN at h={:.0} mm.", res.f_column_flange / 1e3, res.mode_column, res.f_end_plate / 1e3, res.mode_plate, res.f_web_tension / 1e3, res.governing.en(), res.f_rd / 1e3, demand / 1e3, rows[index].lever),
                &format!("F_t,fc,Rd={:.0} kN (Modus {}), F_t,ep,Rd={:.0} kN (Modus {}), F_t,wc,Rd={:.0} kN; maßgebend: {}; plastische Verteilung F_tr,Rd={:.0} kN; elastisch F_tr,Ed={:.0} kN bei h={:.0} mm.", res.f_column_flange / 1e3, res.mode_column, res.f_end_plate / 1e3, res.mode_plate, res.f_web_tension / 1e3, res.governing.de(), res.f_rd / 1e3, demand / 1e3, rows[index].lever),
            );
            let mut b = CheckResult::assess(format!("en1993.1-8.6.2.row.{}", row.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "6", "6.2.7.2"), subject(row, "lever", "Row lever arm", "Reihenhebelarm"), loc("Bolt row tension resistance", "Zugtragfähigkeit der Schraubenreihe"))
                .annex(annex)
                .ndp(AnnexParams::GAMMA_M0)
                .ndp(AnnexParams::GAMMA_M1)
                .ndp(AnnexParams::GAMMA_M2)
                .ndp("ndp.en1993.v-420.1")
                .ndp("ndp.en1993.v-420.2")
                .utilization(force_n(demand), force_n(res.f_iso))
                .explanation(explanation);
            if demand > res.f_iso {
                let subject = SubjectRef::new(&j.id, format!("joints[id={}].plateThickness", j.id), loc("End plate thickness", "Stirnplattendicke"));
                b = b.remedy(match super::special_parts_checks::grow(tp, Some(60.0), |x| component_result(j, &rows, x, params).rows[index].f_iso >= demand) {
                    Some(t_req) => Remedy::at_least(subject, length_m(j.plate_thickness), length_m(t_req / 1e3), loc(&format!("Increase the end plate thickness to at least {t_req:.0} mm."), &format!("Stirnplattendicke auf mindestens {t_req:.0} mm erhöhen."))),
                    None => Remedy::advisory(subject, loc("No end plate thickness up to 60 mm carries this bolt row: use a larger bolt diameter or grade, or a thicker column flange.", "Keine Stirnplattendicke bis 60 mm trägt diese Schraubenreihe: größeren Schraubendurchmesser oder höhere Festigkeitsklasse bzw. dickeren Stützenflansch wählen.")),
                });
            }
            report.push(b.build());
        }
        if scope.admits("joints", &j.id) && result.z > 0.0 && j.moment_demand != 0.0 {
            report.push(compression_check(j, &result, annex));
        }
    }
}

/// 🔩️ Compression zone of an end plate joint: F_c,Ed = M_j,Ed/z against the least of the column web in compression F_c,wc,Rd (6.2.6.2), the column web panel V_wp,Rd/β (6.2.6.1) and the beam flange and web in compression F_c,fb,Rd (6.2.6.7, V-426); the beam web in tension (6.2.6.8, V-427) needs a beam web thickness the model does not carry.
fn compression_check(j: &SteelJoint, result: &jc::RowsResult, annex: AnnexChoice) -> CheckResult {
    let demand = j.moment_demand.abs() * 1e3 / result.z;
    let (name_en, name_de) = if result.cap == result.f_beam { ("beam flange and web in compression", "Trägerflansch und -steg auf Druck") } else if result.cap == result.v_wp_rd { ("column web panel in shear", "Stützenstegfeld auf Schub") } else { ("column web in compression", "Stützensteg auf Druck") };
    let m_max = result.cap * result.z / 1e3;
    let mut builder = CheckResult::assess(format!("en1993.1-8.6.2.comp.{}", j.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "6", "6.2.6.7"), SubjectRef::new(&j.id, format!("joints[id={}].beamMomentResistance", j.id), loc("Beam moment resistance", "Trägermomententragfähigkeit")), loc("Joint compression zone", "Druckzone des Anschlusses"))
        .annex(annex)
        .ndp(AnnexParams::GAMMA_M0)
        .ndp(AnnexParams::GAMMA_M1)
        .utilization(force_n(demand), force_n(result.cap))
        .explanation(loc(
            &format!("F_c,Ed={:.0} kN = M_j,Ed/z (z={:.0} mm); F_c,wc,Rd={:.0} kN, V_wp,Rd/β={:.0} kN, F_c,fb,Rd={:.0} kN; governing: {name_en}.", demand / 1e3, result.z, result.f_c_wc_rd / 1e3, result.v_wp_rd / 1e3, result.f_beam / 1e3),
            &format!("F_c,Ed={:.0} kN = M_j,Ed/z (z={:.0} mm); F_c,wc,Rd={:.0} kN, V_wp,Rd/β={:.0} kN, F_c,fb,Rd={:.0} kN; maßgebend: {name_de}.", demand / 1e3, result.z, result.f_c_wc_rd / 1e3, result.v_wp_rd / 1e3, result.f_beam / 1e3),
        ));
    if demand > result.cap {
        builder = builder.remedy(Remedy::at_most(
            SubjectRef::new(&j.id, format!("joints[id={}].momentDemand", j.id), loc("Joint design moment", "Bemessungsmoment des Anschlusses")),
            moment_nm(j.moment_demand),
            moment_nm(m_max),
            loc(&format!("Reduce the joint moment to at most {:.0} kNm or stiffen the compression zone.", m_max / 1e3), &format!("Anschlussmoment auf höchstens {:.0} kNm senken oder die Druckzone aussteifen.", m_max / 1e3)),
        ));
    }
    builder.build()
}
