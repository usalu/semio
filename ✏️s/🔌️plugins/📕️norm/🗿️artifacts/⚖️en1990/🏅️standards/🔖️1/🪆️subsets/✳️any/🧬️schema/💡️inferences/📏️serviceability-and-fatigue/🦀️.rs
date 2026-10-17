//! 📏️ Serviceability and fatigue checks of the EN 1990 compliance report: the action effects, deflection, vibration and fatigue combinations. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn push_sls_action_effects<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    for (rule, ed, section, title_en, title_de) in [
        ("6.14b", combination_sls_char(annex, actions, leading), "6.14b", "SLS characteristic", "GZG charakteristisch"),
        ("6.15b", combination_sls_frequent(annex, actions, leading), "6.15b", "SLS frequent", "GZG häufig"),
        ("6.16b", combination_sls_quasi_permanent(annex, actions), "6.16b", "SLS quasi-permanent", "GZG quasi-ständig"),
    ] {
        let id = format!("en1990.sls.{}.{}.{leading}", member.id, rule);
        let mut builder = CheckResult::assess(id, "EN 1990 SLS", ClauseId::new("EN 1990", "§6.5", section), subject.clone(), copy(title_en, title_de))
            .utilization(q_force(ed), q_force(rd))
            .annex(annex.choice())
            .explanation(copy(format!("Service E = {ed:.1} N vs capacity {rd:.1} N"), format!("Gebrauchstauglichkeit E = {ed:.1} N gegenüber {rd:.1} N")));
        if ed > rd {
            builder = builder.remedy(Remedy::at_least(
                subject.clone(),
                q_force(rd),
                q_force(ed),
                copy(format!("Increase resistance to at least {:.1} kN for {title_en}.", ed / 1000.0), format!("Widerstand für {title_de} auf mindestens {:.1} kN erhöhen.", ed / 1000.0)),
            ));
        }
        report.push(builder.build());
        if rule == "6.16b" {
            break;
        }
    }
}

pub(super) fn push_sls_deflection(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let limit = if member.deflection_limit_ratio > 0.0 { member.span / member.deflection_limit_ratio } else { member.span / 250.0 };
    let path = member_path(document, &member.id, "deflectionW");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.a14.deflection.{}", member.id),
        "EN 1990 Annex A1.4",
        ClauseId::new("EN 1990", "Annex A1.4", "deflection"),
        subject.clone(),
        copy("Deflection limit", "Durchbiegungsgrenze"),
    )
    .utilization(q_len(member.deflection_w), q_len(limit))
    .annex(document.annex)
    .explanation(copy(
        format!("w = {:.1} mm vs l/{:.0} = {:.1} mm", member.deflection_w * 1000.0, member.deflection_limit_ratio, limit * 1000.0),
        format!("w = {:.1} mm gegenüber l/{:.0} = {:.1} mm", member.deflection_w * 1000.0, member.deflection_limit_ratio, limit * 1000.0),
    ));
    if member.deflection_w > limit {
        let required_stiffness_ratio = member.deflection_w / limit.max(f64::EPSILON);
        builder = builder.remedy(Remedy::at_most(
            subject,
            q_len(member.deflection_w),
            q_len(limit),
            copy(
                format!(
                    "Reduce deflection of {} from {:.1} mm to at most {:.1} mm (increase stiffness by ≥ {:.2}×).",
                    member.label_en,
                    member.deflection_w * 1000.0,
                    limit * 1000.0,
                    required_stiffness_ratio
                ),
                format!(
                    "Durchbiegung von {} von {:.1} mm auf höchstens {:.1} mm reduzieren (Steifigkeit um ≥ {:.2}× erhöhen).",
                    member.label_de,
                    member.deflection_w * 1000.0,
                    limit * 1000.0,
                    required_stiffness_ratio
                ),
            ),
        ));
    }
    report.push(builder.build());
}

pub(super) fn push_sls_vibration(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if member.vibration_frequency_min <= 0.0 {
        report.push(
            CheckResult::assess(
                format!("en1990.a14.vibration.{}", member.id),
                "EN 1990 Annex A1.4",
                ClauseId::new("EN 1990", "Annex A1.4", "vibration"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "vibrationFrequency"), member_label(member)),
                copy("Vibration frequency", "Schwingungsfrequenz"),
            )
            .not_applicable(copy("No vibration frequency criterion defined.", "Kein Schwingungsfrequenzkriterium definiert."))
            .annex(document.annex)
            .build(),
        );
        return;
    }
    let path = member_path(document, &member.id, "vibrationFrequency");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.a14.vibration.{}", member.id),
        "EN 1990 Annex A1.4",
        ClauseId::new("EN 1990", "Annex A1.4", "vibration"),
        subject.clone(),
        copy("Vibration frequency", "Schwingungsfrequenz"),
    )
    .minimum(q_dim(member.vibration_frequency), q_dim(member.vibration_frequency_min))
    .annex(document.annex)
    .explanation(copy(
        format!("f = {:.2} Hz vs f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min),
        format!("f = {:.2} Hz gegenüber f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min),
    ));
    if member.vibration_frequency < member.vibration_frequency_min {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_dim(member.vibration_frequency),
            q_dim(member.vibration_frequency_min),
            copy(
                format!("Increase natural frequency of {} from {:.2} Hz to at least {:.2} Hz.", member.label_en, member.vibration_frequency, member.vibration_frequency_min),
                format!("Eigenfrequenz von {} von {:.2} Hz auf mindestens {:.2} Hz erhöhen.", member.label_de, member.vibration_frequency, member.vibration_frequency_min),
            ),
        ));
    }
    report.push(builder.build());
}

pub(super) fn push_fat<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet) {
    if member.rd_fat <= 0.0 {
        report.push(
            CheckResult::assess(
                format!("en1990.fat.{}", member.id),
                "EN 1990 FAT",
                ClauseId::new("EN 1990", "§2", "FAT"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdFat"), member_label(member)),
                copy("Fatigue limit state", "Ermüdungsgrenzzustand"),
            )
            .not_applicable(copy("FAT resistance not applicable for this member.", "FAT-Widerstand für dieses Bauteil nicht anwendbar."))
            .annex(document.annex)
            .build(),
        );
        return;
    }
    let ed = combination_sls_frequent(annex, actions, 0);
    let path = member_path(document, &member.id, "rdFat");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(format!("en1990.fat.{}", member.id), "EN 1990 FAT", ClauseId::new("EN 1990", "§2", "FAT"), subject.clone(), copy("Fatigue limit state", "Ermüdungsgrenzzustand"))
        .utilization(q_force(ed), q_force(member.rd_fat))
        .annex(document.annex)
        .explanation(copy(format!("Fatigue effect {ed:.1} N vs R_d,FAT {:.1} N", member.rd_fat), format!("Ermüdungswirkung {ed:.1} N gegenüber R_d,FAT {:.1} N", member.rd_fat)));
    if ed > member.rd_fat {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(member.rd_fat),
            q_force(ed),
            copy(format!("Increase R_d,FAT to at least {:.1} kN.", ed / 1000.0), format!("R_d,FAT auf mindestens {:.1} kN erhöhen.", ed / 1000.0)),
        ));
    }
    report.push(builder.build());
}

pub(super) fn is_bridge(kind: &str) -> bool {
    matches!(kind, "road_bridge" | "footbridge" | "rail_bridge")
}
//#endregion 🔖️ComplianceReport
