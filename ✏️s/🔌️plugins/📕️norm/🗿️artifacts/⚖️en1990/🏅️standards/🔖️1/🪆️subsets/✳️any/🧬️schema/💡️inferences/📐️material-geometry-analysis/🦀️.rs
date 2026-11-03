//! 📐️ EN 1990 §4.2, §4.3 and §5.1: design value of a material property X_d = η·X_k/γ_m (E90.10), design value of a geometrical quantity a_d = a_nom ± Δa (E90.11) and the analysis model against the elastic critical load factor α_cr (E90.12). See [`super`].

use super::*;

//#region 🔖️ComplianceReport
fn subject(member: &Member, field: &str) -> SubjectRef {
    member_subject(member, field)
}

fn base(document: &En1990Snapshot, member: &Member, key: &str, clause: &'static str, field: &str, title: (&'static str, &'static str)) -> semio_s_plugin_norm_compliance::CheckBuilder {
    CheckResult::assess(format!("en1990.{key}.{}", member.id), "EN 1990 §4-5", ClauseId::new("EN 1990", clause, clause), subject(member, field), copy(title.0, title.1)).annex(document.annex)
}

/// 🧱️ E90.10 (Eq. 6.3): X_d = η·X_k/γ_m from the member's characteristic strength, conversion factor (0 = 1.0) and material partial factor; a declared design strength may not exceed it.
pub(super) fn push_material_design(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if member.material_strength_k <= 0.0 || member.material_partial_factor <= 0.0 {
        return;
    }
    let eta = if member.material_conversion_factor > 0.0 { member.material_conversion_factor } else { 1.0 };
    let xd = eta * member.material_strength_k / member.material_partial_factor;
    let stress = |value: f64| Quantity::new(QuantityKind::Stress, value);
    let detail = copy(
        format!("X_d = η·X_k/γ_m = {eta:.3}·{:.2} MPa/{:.3} = {:.2} MPa", member.material_strength_k / 1e6, member.material_partial_factor, xd / 1e6),
        format!("X_d = η·X_k/γ_m = {eta:.3}·{:.2} MPa/{:.3} = {:.2} MPa", member.material_strength_k / 1e6, member.material_partial_factor, xd / 1e6),
    );
    let builder = base(document, member, "e10.material", "§4.2", "materialDesignStrength", ("Design value of a material property", "Bemessungswert einer Materialeigenschaft"));
    if member.material_design_strength <= 0.0 {
        report.push(builder.informative(stress(xd)).explanation(detail).build());
        return;
    }
    let mut builder = builder.utilization(stress(member.material_design_strength), stress(xd)).explanation(detail);
    if member.material_design_strength > xd {
        builder = builder.remedy(Remedy::at_most(subject(member, "materialDesignStrength"), stress(member.material_design_strength), stress(xd), copy(format!("Reduce the declared X_d to at most {:.2} MPa.", xd / 1e6), format!("Das angesetzte X_d auf höchstens {:.2} MPa verringern.", xd / 1e6))));
    }
    report.push(builder.build());
}

/// 📏️ E90.11 (Eq. 6.4): a_d = a_nom ± Δa; a declared design dimension must deviate from the nominal one by at least |Δa| on its side (the unfavourable direction is the designer's choice).
pub(super) fn push_geometry_design(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if member.nominal_dimension <= 0.0 || member.dimension_tolerance == 0.0 {
        return;
    }
    let tolerance = member.dimension_tolerance.abs();
    let length = |value: f64| Quantity::new(QuantityKind::Length, value);
    let builder = base(document, member, "e11.geometry", "§4.3", "designDimension", ("Design value of a geometrical quantity", "Bemessungswert einer geometrischen Größe"));
    let range = copy(
        format!("a_d = a_nom ± Δa = {:.4} ± {tolerance:.4} m", member.nominal_dimension),
        format!("a_d = a_nom ± Δa = {:.4} ± {tolerance:.4} m", member.nominal_dimension),
    );
    if member.design_dimension <= 0.0 {
        report.push(builder.informative(length(member.nominal_dimension - tolerance)).explanation(range).build());
        return;
    }
    let deviation = (member.design_dimension - member.nominal_dimension).abs();
    let mut builder = builder.minimum(length(deviation), length(tolerance)).explanation(range);
    if deviation < tolerance {
        let side = if member.design_dimension >= member.nominal_dimension { 1.0 } else { -1.0 };
        let required = member.nominal_dimension + side * tolerance;
        builder = builder.remedy(Remedy::exactly(subject(member, "designDimension"), length(member.design_dimension), length(required), copy(format!("Set a_d to {required:.4} m (a_nom {} Δa).", if side > 0.0 { "+" } else { "−" }), format!("a_d auf {required:.4} m setzen (a_nom {} Δa).", if side > 0.0 { "+" } else { "−" }))));
    }
    report.push(builder.build());
}

/// 🧮️ E90.12 (§5.1.1-5.1.4): a member analysed by first-order theory needs α_cr at least the registry threshold; otherwise second-order effects govern which resistances apply. The thresholds of the material parts stay with them.
pub(super) fn push_analysis_model(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if member.member_alpha_cr <= 0.0 {
        return;
    }
    let threshold = ndp(document.annex, "ndp.en1990.e90-12.alpha_cr_first_order");
    let factor = |value: f64| q_dim(value);
    let second = member.second_order_analysis >= 0.5;
    let builder = base(document, member, "e12.analysis", "§5.1.1", "secondOrderAnalysis", ("Analysis model against α_cr", "Berechnungsmodell gegenüber α_cr"));
    let text = copy(
        format!("α_cr = {:.2} against the first-order limit {threshold:.1}; {} analysis declared", member.member_alpha_cr, if second { "second-order" } else { "first-order" }),
        format!("α_cr = {:.2} gegenüber der Grenze {threshold:.1} für Theorie 1. Ordnung; angesetzt: Theorie {}. Ordnung", member.member_alpha_cr, if second { 2 } else { 1 }),
    );
    let mut builder = if second { builder.informative(factor(member.member_alpha_cr)).explanation(text) } else { builder.minimum(factor(member.member_alpha_cr), factor(threshold)).explanation(text) };
    if !second && member.member_alpha_cr < threshold {
        builder = builder.remedy(Remedy::exactly(subject(member, "secondOrderAnalysis"), factor(0.0), factor(1.0), copy("Analyse by second-order theory (set the flag to 1) or stiffen the member until α_cr reaches the limit.", "Nach Theorie 2. Ordnung rechnen (Kennzeichen auf 1) oder das Bauteil aussteifen, bis α_cr die Grenze erreicht.")));
    }
    report.push(builder.ndp("ndp.en1990.e90-12.alpha_cr_first_order").ndp_reads().build());
}
//#endregion 🔖️ComplianceReport
