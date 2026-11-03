//! 📏️ Serviceability and fatigue checks of the EN 1990 compliance report: the SLS combinations against C_d (Eq. 6.13), Annex A1.4 deflection and vibration, and the fatigue action range. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn push_sls_action_effects(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    let Some(ctx) = context_for(document, FactorSet::StrB) else { return };
    for (equation, field, limit) in [(Equation::Characteristic, "slsLimitChar", member.sls_limit_char), (Equation::Frequent, "slsLimitFreq", member.sls_limit_freq), (Equation::QuasiPermanent, "slsLimitQp", member.sls_limit_qp)] {
        let id = format!("en1990.sls.{}.{}", member.id, equation.label());
        let clause = ClauseId::new("EN 1990", "§6.5.3", equation.label());
        let title = equation.title();
        let subject = member_subject(member, field);
        let Some(combination) = combinations(&ctx, actions, equation).ok().and_then(|all| governing(&all, false).cloned()) else { continue };
        if limit <= 0.0 {
            report.push(not_applicable(id, "EN 1990 SLS", clause, subject, copy(title.0, title.1), document.annex, copy("No serviceability criterion C_d defined for this combination.", "Kein Gebrauchstauglichkeitskriterium C_d für diese Kombination definiert.")));
            continue;
        }
        report.push(
            ResistanceCheck {
                id,
                part: "EN 1990 SLS".into(),
                clause,
                subject,
                title: copy(title.0, title.1),
                annex: document.annex,
                effect: combination.total(),
                resistance: limit,
                explanation: copy(format!("E_d = {:.1} N ({}) ≤ C_d = {limit:.1} N (Eq. 6.13)", combination.total(), combination.describe()), format!("E_d = {:.1} N ({}) ≤ C_d = {limit:.1} N (Gl. 6.13)", combination.total(), combination.describe())),
                remedy_en: &format!("Raise the criterion C_d of {} ({})", member.label_en, equation.label()),
                remedy_de: &format!("Kriterium C_d von {} ({}) erhöhen", member.label_de, equation.label()),
            }
            .build_citing(&psi_ndps(document)),
        );
    }
}

const SLS_STATIONS: usize = 10;

/// 📐️ Elastic line of a simply supported prismatic member under uniform load, `w(s) = 16/5 · (s − 2s³ + s⁴)` normalised to a unit peak at midspan (EN 1990 Annex A1.4, Fig. A1.1); the stations of the deflection checks follow it.
fn elastic_line(s: f64) -> f64 {
    3.2 * s * (1.0 - 2.0 * s * s + s * s * s)
}

fn deflection_check(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, key: &str, field: &str, value: f64, ratio: f64, title: (&str, &str), remedy: Option<Remedy>) {
    let clause = ClauseId::new("EN 1990", "A1.4.2", "A1.4.2");
    let id = format!("en1990.a14.{key}.{}", member.id);
    let subject = member_subject(member, field);
    if ratio <= 0.0 || member.span <= 0.0 {
        report.push(not_applicable(id, "EN 1990 Annex A1.4", clause, subject, copy(title.0, title.1), document.annex, copy("No deflection limit L/n defined.", "Keine Durchbiegungsgrenze L/n definiert.")));
        return;
    }
    let limit = member.span / ratio;
    let mut builder = CheckResult::assess(id, "EN 1990 Annex A1.4", clause, subject.clone(), copy(title.0, title.1)).ndp("ndp.en1990.e90-24.1").utilization(q_len(value), q_len(limit)).annex(document.annex).explanation(copy(
        format!("{:.1} mm vs L/{ratio:.0} = {:.1} mm (L = {:.2} m)", value * 1000.0, limit * 1000.0, member.span),
        format!("{:.1} mm gegenüber L/{ratio:.0} = {:.1} mm (L = {:.2} m)", value * 1000.0, limit * 1000.0, member.span),
    ))
    .sampled(SLS_STATIONS, |s| {
        let w = value.abs() * elastic_line(s);
        (w / limit, vec![q_len(w), q_len(limit)])
    });
    if value > limit {
        builder = builder.remedy(remedy.unwrap_or_else(|| {
            Remedy::at_most(subject, q_len(value), q_len(limit), copy(format!("Reduce the deflection of {} to at most {:.1} mm.", member.label_en, limit * 1000.0), format!("Durchbiegung von {} auf höchstens {:.1} mm reduzieren.", member.label_de, limit * 1000.0)))
        }));
    }
    report.push(builder.build());
}

pub(super) fn push_sls_deflection(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let total = member.deflection_w1 + member.deflection_w2 + member.deflection_w3;
    let net = total - member.precamber;
    let limit = if member.deflection_limit_ratio > 0.0 { member.span / member.deflection_limit_ratio } else { 0.0 };
    let camber = Remedy::at_least(
        member_subject(member, "precamber"),
        q_len(member.precamber),
        q_len(total - limit),
        copy(format!("Pre-camber {} by at least {:.1} mm.", member.label_en, (total - limit) * 1000.0), format!("{} um mindestens {:.1} mm überhöhen.", member.label_de, (total - limit) * 1000.0)),
    );
    deflection_check(report, document, member, "deflection", "deflectionW3", net, member.deflection_limit_ratio, ("Net deflection w_net,fin = w1 + w2 + w3 − w_c", "Enddurchbiegung w_net,fin = w1 + w2 + w3 − w_c"), Some(camber));
    deflection_check(report, document, member, "deflection-w3", "deflectionW3", member.deflection_w3, member.deflection_limit_ratio_w3, ("Variable-action deflection w3", "Durchbiegung aus veränderlicher Einwirkung w3"), None);
}

pub(super) fn push_sls_vibration(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let id = format!("en1990.a14.vibration.{}", member.id);
    let clause = ClauseId::new("EN 1990", "A1.4.4", "A1.4.4");
    let subject = member_subject(member, "vibrationFrequency");
    if member.vibration_frequency_min <= 0.0 {
        report.push(not_applicable(id, "EN 1990 Annex A1.4", clause, subject, copy("Vibration frequency", "Schwingungsfrequenz"), document.annex, copy("No vibration frequency criterion defined.", "Kein Schwingungsfrequenzkriterium definiert.")));
        return;
    }
    let mut builder = CheckResult::assess(id, "EN 1990 Annex A1.4", clause, subject.clone(), copy("Vibration frequency", "Schwingungsfrequenz"))
        .minimum(q_dim(member.vibration_frequency), q_dim(member.vibration_frequency_min))
        .ndp("ndp.en1990.e90-24.2")
        .annex(document.annex)
        .explanation(copy(format!("f = {:.2} Hz vs f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min), format!("f = {:.2} Hz gegenüber f_min = {:.2} Hz", member.vibration_frequency, member.vibration_frequency_min)));
    if member.vibration_frequency < member.vibration_frequency_min {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_dim(member.vibration_frequency),
            q_dim(member.vibration_frequency_min),
            copy(format!("Increase natural frequency of {} from {:.2} Hz to at least {:.2} Hz.", member.label_en, member.vibration_frequency, member.vibration_frequency_min), format!("Eigenfrequenz von {} von {:.2} Hz auf mindestens {:.2} Hz erhöhen.", member.label_de, member.vibration_frequency, member.vibration_frequency_min)),
        ));
    }
    report.push(builder.build());
}

pub(super) fn push_fat(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    let id = format!("en1990.fat.{}", member.id);
    let clause = ClauseId::new("EN 1990", "§6.4.5", "6.4.5");
    let subject = member_subject(member, "rdFat");
    let title = copy("Fatigue limit state (advisory): variable part of the frequent combination", "Ermüdungsgrenzzustand (Hinweis): veränderlicher Anteil der häufigen Kombination");
    let permanent: f64 = actions.iter().filter(|a| matches!(a.class, ActionClass::Permanent | ActionClass::Prestress)).map(|a| a.value).sum();
    let combination = context_for(document, FactorSet::StrB).and_then(|ctx| combinations(&ctx, actions, Equation::Frequent).ok()).and_then(|all| governing(&all, false).cloned());
    let (true, Some(combination)) = (member.rd_fat > 0.0, combination) else {
        report.push(not_applicable(id, "EN 1990 FAT", clause, subject, title, document.annex, copy("FAT resistance not applicable for this member.", "FAT-Widerstand für dieses Bauteil nicht anwendbar.")));
        return;
    };
    let range = combination.total() - permanent;
    let mut check =
        ResistanceCheck {
            id,
            part: "EN 1990 FAT".into(),
            clause,
            subject,
            title,
            annex: document.annex,
            effect: range,
            resistance: member.rd_fat,
            explanation: copy(format!("Variable part of the frequent combination ΔE = {range:.1} N vs R_d,FAT = {:.1} N (detailed damage: E90.21)", member.rd_fat), format!("Veränderlicher Anteil der häufigen Kombination ΔE = {range:.1} N gegenüber R_d,FAT = {:.1} N (Schädigung: E90.21)", member.rd_fat)),
            remedy_en: "Increase R_d,FAT",
            remedy_de: "R_d,FAT erhöhen",
        }
        .build();
    if check.status == CheckStatus::Fail {
        check.status = CheckStatus::Warning;
    }
    report.push(check);
}
//#endregion 🔖️ComplianceReport
