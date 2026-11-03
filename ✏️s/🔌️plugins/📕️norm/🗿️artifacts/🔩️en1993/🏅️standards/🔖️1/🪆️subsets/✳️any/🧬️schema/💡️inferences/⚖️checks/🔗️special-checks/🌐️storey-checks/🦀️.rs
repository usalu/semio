//! 🌐️ Storey checks of EN 1993-1-1 §5: α_cr and the analysis mode (5.2), the sway imperfection φ (5.3.2) and the bracing deflection and stabilising force (5.3.3). See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::global;
use semio_s_artifact_norm_en1993_model::{Storey, SteelMember};

fn alpha_cr_min() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.alpha-cr-min")
}

fn subject(s: &Storey, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&s.id, format!("storeys[id={}].{field}", s.id), loc(en, de))
}

fn finish(report: &mut CheckReport, annex: AnnexChoice, id: String, section: &str, subject: SubjectRef, title: LocalizedCopy, explanation: LocalizedCopy, build: impl FnOnce(semio_s_plugin_norm_compliance::CheckBuilder) -> semio_s_plugin_norm_compliance::CheckBuilder) {
    let base = CheckResult::assess(id, "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "5", section), subject, title).annex(annex).explanation(explanation);
    report.push(build(base).build());
}

fn portal_alpha(document: &En1993Snapshot, s: &Storey) -> Option<f64> {
    if s.columns != 2 {
        return None;
    }
    let column = document.members.iter().find(|m| m.id == s.member_id)?;
    let beam = document.members.iter().find(|m| m.member_type == semio_s_artifact_norm_en1993_model::SteelMemberType::Beam)?;
    let sec = |m: &SteelMember| document.sections.iter().find(|x| x.id == m.section_id);
    let e = document.materials.iter().find(|m| m.id == column.material_id)?.e_modulus;
    global::alpha_cr_portal_fixed(e, sec(column)?.iy, sec(beam)?.iy, s.height, beam.length, s.vertical_load / 2.0)
}

fn check_alpha(annex: AnnexChoice, document: &En1993Snapshot, s: &Storey, frame: Option<f64>, report: &mut CheckReport) {
    let Some(horne) = global::alpha_cr_storey(s.horizontal_load, s.vertical_load, s.height, s.drift) else { return };
    let eigen = frame.or_else(|| portal_alpha(document, s));
    let alpha = eigen.unwrap_or(horne);
    if let Some(eigen) = eigen {
        let excess = global::horne_excess(horne, eigen);
        let explanation = loc(
            &format!("Eq. 5.2 gives {horne:.2}, the eigenvalue of the frame is {eigen:.2}: Eq. 5.2 is {:.1} % {} (accepted within {:.0} %); the eigenvalue governs.", excess.abs() * 100.0, if excess >= 0.0 { "unconservative" } else { "conservative" }, global::horne_tolerance() * 100.0),
            &format!("Gl. 5.2 liefert {horne:.2}, der Eigenwert des Rahmens ist {eigen:.2}: Gl. 5.2 liegt {:.1} % {} (zulässig innerhalb {:.0} %); maßgebend ist der Eigenwert.", excess.abs() * 100.0, if excess >= 0.0 { "auf der unsicheren Seite" } else { "auf der sicheren Seite" }, global::horne_tolerance() * 100.0),
        );
        finish(report, annex, format!("en1993.1-1.5.2.horne.{}", s.id), "5.2.1", subject(s, "drift", "Horizontal drift", "Horizontale Kopfverschiebung"), loc("Validity of the approximate α_cr (Eq. 5.2)", "Gültigkeit des Näherungswerts α_cr (Gl. 5.2)"), explanation, |b| b.ndp("ndp.en1993.v-026.2").utilization(dimensionless(excess.max(0.0)), dimensionless(global::horne_tolerance())));
    }

    let plastic = s.plastic_analysis == 1;
    let mode = global::analysis_mode(alpha, plastic);
    let amplification = global::amplification(alpha);
    let (mode_en, mode_de) = match mode {
        global::AnalysisMode::FirstOrder => ("first-order analysis", "Berechnung nach Theorie I. Ordnung"),
        global::AnalysisMode::AmplifiedFirstOrder => ("first-order analysis with amplified sway moments", "Theorie I. Ordnung mit vergrößerten Horizontalmomenten"),
        global::AnalysisMode::SecondOrder => ("second-order analysis", "Berechnung nach Theorie II. Ordnung"),
    };
    let factor = amplification.map_or("-".to_string(), |k| format!("{k:.3}"));
    let limit = global::alpha_cr_limit(plastic);
    let (source, source_de) = if eigen.is_some() { if frame.is_some() { ("eigenvalue of the frame model (mechanics kernel)", "Eigenwert des Rahmenmodells (Mechanikkern)") } else { ("eigenvalue of the portal", "Eigenwert des Portalrahmens") } } else { ("Eq. 5.2 only, validity conditions 5.2.1(4)B not verified", "nur Gl. 5.2, Gültigkeitsbedingungen 5.2.1(4)B nicht geprüft") };
    let (h, v, d, hh) = (s.horizontal_load, s.vertical_load, s.drift, s.height);
    let explanation = loc(
        &format!("α_cr=(H_Ed/V_Ed)(h/δ_H,Ed)=({:.1} kN/{:.1} kN)({:.2} m/{:.2} mm)={horne:.2}; governing α_cr={alpha:.2}; limit {limit:.0} for {}; amplification 1/(1-1/α_cr)={factor}; required: {mode_en}. Basis: {source}.", h / 1e3, v / 1e3, hh, d * 1e3, if plastic { "plastic analysis" } else { "elastic analysis" }),
        &format!("α_cr=(H_Ed/V_Ed)(h/δ_H,Ed)=({:.1} kN/{:.1} kN)({:.2} m/{:.2} mm)={horne:.2}; maßgebendes α_cr={alpha:.2}; Grenze {limit:.0} bei {}; Vergrößerung 1/(1-1/α_cr)={factor}; erforderlich: {mode_de}. Grundlage: {source_de}.", h / 1e3, v / 1e3, hh, d * 1e3, if plastic { "plastischer Berechnung" } else { "elastischer Berechnung" }),
    );
    let drift_max = h * hh / (v * alpha_cr_min());
    let load_max = h * hh / (d * alpha_cr_min());
    finish(report, annex, format!("en1993.1-1.5.2.alpha.{}", s.id), "5.2.1", subject(s, "drift", "Horizontal drift", "Horizontale Kopfverschiebung"), loc("Elastic critical load factor α_cr of the storey", "Elastischer Verzweigungslastfaktor α_cr des Geschosses"), explanation, |b| {
        let b = b.minimum(dimensionless(alpha), dimensionless(alpha_cr_min()));
        let b = if eigen.is_some() { b } else { b.ndp("ndp.en1993.v-026.2") };
        if alpha >= alpha_cr_min() {
            return b;
        }
        b.remedy(Remedy::at_most(subject(s, "drift", "Horizontal drift", "Horizontale Kopfverschiebung"), length_m(d), length_m(drift_max), loc(&format!("Stiffen the storey so that δ_H,Ed is at most {:.2} mm.", drift_max * 1e3), &format!("Geschoss aussteifen, sodass δ_H,Ed höchstens {:.2} mm beträgt.", drift_max * 1e3))))
            .remedy(Remedy::at_most(subject(s, "verticalLoad", "Vertical load", "Vertikallast"), force_n(v), force_n(load_max), loc(&format!("Or limit V_Ed to at most {:.0} kN.", load_max / 1e3), &format!("Oder V_Ed auf höchstens {:.0} kN begrenzen.", load_max / 1e3))))
    });
}

fn check_sway(annex: AnnexChoice, s: &Storey, report: &mut CheckReport) {
    if s.vertical_load <= 0.0 || s.building_height <= 0.0 {
        return;
    }
    let m = f64::from(s.columns.max(1));
    let phi = global::sway_phi(s.building_height, m);
    let equivalent = global::sway_equivalent_force(phi, s.vertical_load);
    let neglect = global::sway_may_be_neglected(s.horizontal_load, s.vertical_load);
    let explanation = loc(
        &format!("φ=φ0 α_h α_m={phi:.5} (h={:.2} m, m={m:.0}); equivalent horizontal force φ V_Ed={:.2} kN; imperfection {} (H_Ed {} 0.15 V_Ed).", s.building_height, equivalent / 1e3, if neglect { "may be neglected" } else { "must be included" }, if neglect { ">=" } else { "<" }),
        &format!("φ=φ0 α_h α_m={phi:.5} (h={:.2} m, m={m:.0}); Ersatzhorizontalkraft φ V_Ed={:.2} kN; Imperfektion {} (H_Ed {} 0,15 V_Ed).", s.building_height, equivalent / 1e3, if neglect { "darf vernachlässigt werden" } else { "ist anzusetzen" }, if neglect { ">=" } else { "<" }),
    );
    finish(report, annex, format!("en1993.1-1.5.3.2.sway.{}", s.id), "5.3.2", subject(s, "columns", "Columns in the row", "Stützen in der Reihe"), loc("Global sway imperfection φ", "Globale Schiefstellung φ"), explanation, |b| b.informative(force_n(equivalent)));
}

fn check_bracing(annex: AnnexChoice, s: &Storey, report: &mut CheckReport) {
    if s.bracing_load <= 0.0 || s.bracing_length <= 0.0 {
        return;
    }
    let m = f64::from(s.columns.max(1));
    let qd = global::bracing_qd(s.bracing_load, m, s.bracing_length, s.bracing_deflection);
    let divisor = semio_s_plugin_norm_compliance::ndp_number("ndp.en1993.bracing.deflection-limit-divisor", annex);
    let limit = s.bracing_length / divisor;
    let explanation = loc(
        &format!("q_d=ΣN_Ed·8(e_0+δ_q)/L²={:.3} kN/m with ΣN_Ed={:.1} kN, L={:.2} m, δ_q={:.2} mm; limit δ_q<=L/{divisor:.0}={:.2} mm. UNVERIFIED: the divisor {divisor:.0} is not checked against the print of EN 1993-1-1 5.3.3 (L/2500 is also recalled for the stiffness criterion).", qd / 1e3, s.bracing_load / 1e3, s.bracing_length, s.bracing_deflection * 1e3, limit * 1e3),
        &format!("q_d=ΣN_Ed·8(e_0+δ_q)/L²={:.3} kN/m mit ΣN_Ed={:.1} kN, L={:.2} m, δ_q={:.2} mm; Grenze δ_q<=L/{divisor:.0}={:.2} mm. UNGEPRÜFT: der Teiler {divisor:.0} ist nicht an der Druckfassung von EN 1993-1-1 5.3.3 geprüft (für das Steifigkeitskriterium ist auch L/2500 in Erinnerung).", qd / 1e3, s.bracing_load / 1e3, s.bracing_length, s.bracing_deflection * 1e3, limit * 1e3),
    );
    finish(report, annex, format!("en1993.1-1.5.3.3.bracing.{}", s.id), "5.3.3", subject(s, "bracingDeflection", "Bracing deflection", "Auslenkung der Aussteifung"), loc("Bracing system deflection and stabilising force", "Auslenkung und Stabilisierungskraft der Aussteifung"), explanation, |b| {
        let b = b.ndp("ndp.en1993.bracing.deflection-limit-divisor").ndp("ndp.en1993.bracing.imperfection-divisor").utilization(length_m(s.bracing_deflection), length_m(limit));
        if s.bracing_deflection <= limit {
            return b;
        }
        b.remedy(Remedy::at_most(subject(s, "bracingDeflection", "Bracing deflection", "Auslenkung der Aussteifung"), length_m(s.bracing_deflection), length_m(limit), loc(&format!("Stiffen the bracing so that δ_q is at most {:.2} mm.", limit * 1e3), &format!("Aussteifung versteifen, sodass δ_q höchstens {:.2} mm beträgt.", limit * 1e3))))
    });
}

/// 🌐️ Appends the storey checks of every admitted storey row.
pub fn check_storeys(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    let frame = if document.storeys.is_empty() { None } else { joint_frame::alpha_cr(document).ok() };
    for s in document.storeys.iter().filter(|s| scope.admits("storeys", &s.id)) {
        check_alpha(annex, document, s, frame, report);
        check_sway(annex, s, report);
        check_bracing(annex, s, report);
    }
}
