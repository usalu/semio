//! 🕵️ Fatigue assessment method selection under EN 1993-1-9:2005 §3.

use super::{Inputs, Outcome, Row};

#[derive(Debug, PartialEq, Eq)]
pub enum Selection {
    Pass,
    SafeLifeRequired,
    MaintenanceRequired,
    Unverified,
}

pub fn selection(method: &str, rapid: &str, inspection: &str, basis: &str) -> Selection {
    match (method, rapid, inspection, basis) {
        ("safeLife", _, _, _) => Selection::Pass,
        ("damageTolerant", "yes", _, _) => Selection::SafeLifeRequired,
        ("damageTolerant", "no", "no", _) => Selection::MaintenanceRequired,
        ("damageTolerant", "no", "yes", "redistribution" | "steel") => Selection::Pass,
        _ => Selection::Unverified,
    }
}

pub fn evaluate(q: &Inputs<'_>) -> Outcome {
    let method = q.t("fatigue_method")?;
    let rapid = q.t("fatigue_rapid_failure").unwrap_or("unknown");
    let mut inspection = q.t("fatigue_lifetime_maintenance").unwrap_or("unknown");
    let basis = q.t("fatigue_tolerance_basis").unwrap_or("unknown");
    let inspection_record = q.t("fatigue_maintenance_record").unwrap_or("").trim();
    if inspection == "yes" && inspection_record.is_empty() {
        inspection = "unknown";
    }
    let (score, rationale) = match selection(method, rapid, inspection, basis) {
        Selection::Pass if method == "safeLife" => (1.0, "Safe-life method selected. Nachweis der sicheren Lebensdauer gewählt."),
        Selection::Pass => (1.0, "Documented design-life inspection and maintenance and damage-tolerance basis. Dokumentierte Inspektion und Instandhaltung über die Nutzungsdauer mit Grundlage der Schadenstoleranz."),
        Selection::SafeLifeRequired => (2.0, "Rapid failure from local cracking calls for the safe-life method. Bei raschem Versagen durch lokale Risse ist die sichere Lebensdauer anzusetzen."),
        Selection::MaintenanceRequired => (2.0, "The damage-tolerant method depends on design-life inspection and maintenance. Das schadenstolerante Verfahren setzt Inspektion und Instandhaltung über die Nutzungsdauer voraus."),
        Selection::Unverified => return Err("Provide the method and applicable evidence for the crack consequence, maintenance programme and damage-tolerance basis. Nachweiskonzept und zutreffende Nachweise zu Rissfolgen, Instandhaltung und Schadenstoleranz erfassen.".into()),
    };
    Ok(vec![Row::ratio("eligibility", "Fatigue assessment method eligibility", "Eignung des Ermüdungsnachweiskonzepts", score, rationale.into())])
}
