//! 🧮️ Partial factor sets of the EN 1990 compliance report: the K_FI and geotechnical set C factors. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn push_k_fi(report: &mut CheckReport, document: &En1990Snapshot) {
    let expected = k_fi(document.consequence_class);
    let declared = document.k_fi_declared;
    let subject = SubjectRef::new(document.project_id.clone(), "kFiDeclared", copy(format!("Declared K_FI ({})", document.project_id), format!("Deklariertes K_FI ({})", document.project_id)));
    let mut builder = CheckResult::assess(
        "en1990.annex-b.kfi",
        "EN 1990 Annex B",
        ClauseId::new("EN 1990", "Annex B", "K_FI"),
        subject.clone(),
        copy("Declared K_FI vs consequence class", "Deklariertes K_FI gegenüber Schadensfolgeklasse"),
    )
    .status(if (declared - expected).abs() < 1e-9 { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
    .annex(document.annex)
    .explanation(copy(
        format!("Declared K_FI = {declared:.2}; tabulated for CC{} = {expected:.2}", document.consequence_class),
        format!("Deklariertes K_FI = {declared:.2}; tabellarisch für CC{} = {expected:.2}", document.consequence_class),
    ));
    if (declared - expected).abs() >= 1e-9 {
        builder = builder.remedy(Remedy::exactly(
            subject,
            q_dim(declared),
            q_dim(expected),
            copy(
                format!("Set declared K_FI to {expected:.2} for CC{}.", document.consequence_class),
                format!("Deklariertes K_FI für CC{} auf {expected:.2} setzen.", document.consequence_class),
            ),
        ));
    }
    let mut built = builder.build();
    built.computed = q_dim(declared);
    built.limit = q_dim(expected);
    built.utilization = if expected.abs() < f64::EPSILON { 0.0 } else { declared / expected };
    report.push(built);
}

pub(super) fn push_geo_set_c<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    if !is_bridge(&document.structure_kind) {
        return;
    }
    let ed = combination_geo_a24c(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let rd = member.rd_geo;
    let path = member_path(document, &member.id, "rdGeo");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.geo.set-c.{}.{leading}", member.id),
        "EN 1990 GEO A2.4(C)",
        ClauseId::new("EN 1990", "Annex A2.4(C)", "GEO"),
        subject.clone(),
        copy("GEO set C", "GEO Satz C"),
    )
    .utilization(q_force(ed), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("GEO set C E_d = {ed:.1} N vs R_d,GEO = {rd:.1} N"),
        format!("GEO Satz C E_d = {ed:.1} N gegenüber R_d,GEO = {rd:.1} N"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,GEO to at least {:.1} kN for set C.", ed / 1000.0),
                format!("R_d,GEO für Satz C auf mindestens {:.1} kN erhöhen.", ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}
//#endregion 🔖️ComplianceReport
