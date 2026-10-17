//! 🌬️ DIN 4108-7 of the compliance checks: the n50 limit and the airtightness check. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part7
use crate::*;

/// 🌬️ DIN 4108-7 n50 limit [1/h]: with / without mechanical ventilation.
pub fn n50_limit(has_mechanical_ventilation: bool) -> f64 {
    if has_mechanical_ventilation {
        1.5
    } else {
        3.0
    }
}

/// ✅️ Airtightness n50 check (DIN 4108-7).
pub fn check_airtightness(n50: f64, has_mechanical_ventilation: bool) -> CheckResult {
    let limit = n50_limit(has_mechanical_ventilation);
    let subject = SubjectRef::new("", "airtightnessN50", loc("Building airtightness", "Gebäudedichtheit"));
    let mut builder = CheckResult::assess(
        "din4108-7.n50",
        "DIN 4108-7",
        ClauseId::new("DIN 4108-7", "§4", "4.2"),
        subject.clone(),
        loc("Airtightness n50", "Luftdichtheit n50"),
    )
    .utilization(Quantity::new(QuantityKind::VentilationRate, n50), Quantity::new(QuantityKind::VentilationRate, limit))
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!("n50 = {n50:.2} h⁻¹ vs limit {limit:.1} h⁻¹ (mechanical ventilation = {has_mechanical_ventilation})."),
        &format!("n50 = {n50:.2} h⁻¹ gegenüber Grenzwert {limit:.1} h⁻¹ (mechanische Lüftung = {has_mechanical_ventilation})."),
    ));
    if n50 > limit {
        builder = builder.remedy(Remedy::at_most(
            subject,
            Quantity::new(QuantityKind::VentilationRate, n50),
            Quantity::new(QuantityKind::VentilationRate, limit),
            loc(
                &format!("Achieve n50 ≤ {limit:.1} h⁻¹ (currently {n50:.2} h⁻¹) by tightening the envelope."),
                &format!("n50 ≤ {limit:.1} h⁻¹ erreichen (aktuell {n50:.2} h⁻¹) durch dichtere Gebäudehülle."),
            ),
        ));
    }
    builder.build()
}
// #endregion 🔖️Part7
//#endregion 🔖️ComplianceHelpers
