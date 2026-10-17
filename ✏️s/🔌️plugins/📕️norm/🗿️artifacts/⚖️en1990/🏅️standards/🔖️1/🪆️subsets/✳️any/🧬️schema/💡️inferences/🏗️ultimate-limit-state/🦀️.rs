//! 🏗️ Ultimate limit state checks of the EN 1990 compliance report: the structural, equilibrium, accidental, seismic, geotechnical and stability combinations. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn push_uls_str_geo<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let ed_a = combination_str_geo_610a_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_b = combination_str_geo_610b_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_610 = ed_a.max(ed_b);
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    for (rule, ed, section) in [("6.10a", ed_a, "6.10a"), ("6.10b", ed_b, "6.10b"), ("6.10", ed_610, "6.10")] { // governing 6.10 = max(6.10a,6.10b) per DIN EN 1990/NA NDP A1.3.1(4)
        let id = format!("en1990.str.{}.{}.{leading}", member.id, rule);
        let mut builder = CheckResult::assess(id, if is_bridge(&document.structure_kind) { "EN 1990 STR A2.4(B)" } else { "EN 1990 STR A1.2(B)" }, ClauseId::new("EN 1990", "§6.4", section), subject.clone(), str_title(section))
            .utilization(q_force(ed), q_force(rd))
            .annex(annex.choice())
            .explanation({
                let k = k_fi(document.consequence_class);
                if rule == "6.10" {
                    copy(
                        format!("Governing E_d = max(6.10a, 6.10b) = {ed:.1} N per DIN EN 1990/NA NDP A1.3.1(4); R_d = {rd:.1} N, K_FI = {k:.2}, leading={leading}"),
                        format!("Maßgebend E_d = max(6.10a, 6.10b) = {ed:.1} N nach DIN EN 1990/NA NDP A1.3.1(4); R_d = {rd:.1} N, K_FI = {k:.2}, führend={leading}"),
                    )
                } else {
                    copy(
                        format!("E_d = {ed:.1} N, R_d = {rd:.1} N, K_FI = {k:.2}, leading={leading}"),
                        format!("E_d = {ed:.1} N, R_d = {rd:.1} N, K_FI = {k:.2}, führend={leading}"),
                    )
                }
            });
        if ed > rd {
            builder = builder.remedy(Remedy::at_least(
                subject.clone(),
                q_force(rd),
                q_force(ed),
                copy(
                    format!("Increase R_d,STR of {} from {:.1} kN to at least {:.1} kN.", member.label_en, rd / 1000.0, ed / 1000.0),
                    format!("R_d,STR von {} von {:.1} kN auf mindestens {:.1} kN erhöhen.", member.label_de, rd / 1000.0, ed / 1000.0),
                ),
            ));
        }
        report.push(builder.build());
    }
}

pub(super) fn push_uls_equ<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let ed = combination_equ_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let rd = member.rd_equ_destab;
    let path = member_path(document, &member.id, "rdEquDestab");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.equ.destab.{}.{leading}", member.id),
        if is_bridge(&document.structure_kind) { "EN 1990 EQU A2.4(A)" } else { "EN 1990 EQU A1.2(A)" },
        ClauseId::new("EN 1990", "Table A1.2(A)", "EQU"),
        subject.clone(),
        copy("EQU equilibrium", "EQU Gleichgewicht"),
    )
    .utilization(q_force(ed.max(0.0)), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("Destabilising E_d = {ed:.1} N vs R_d,EQU = {rd:.1} N"),
        format!("Entstabilisierendes E_d = {ed:.1} N gegenüber R_d,EQU = {rd:.1} N"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,EQU (destabilising) of {} to at least {:.1} kN.", member.label_en, ed / 1000.0),
                format!("R_d,EQU (entstabilisierend) von {} auf mindestens {:.1} kN erhöhen.", member.label_de, ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

pub(super) fn push_accidental<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    if actions.a_d.abs() < f64::EPSILON && document.accidentals.is_empty() {
        report.push(
            CheckResult::assess(
                format!("en1990.6.11.{}", member.id),
                "EN 1990",
                ClauseId::new("EN 1990", "§6.4", "6.11"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Accidental combination 6.11", "Außergewöhnliche Kombination 6.11"),
            )
            .not_applicable(copy("No accidental action A_d defined.", "Keine außergewöhnliche Einwirkung A_d definiert."))
            .annex(annex.choice())
            .build(),
        );
        return;
    }
    let ed = combination_6_11(annex, actions, leading);
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.6.11.{}.{leading}", member.id),
        "EN 1990",
        ClauseId::new("EN 1990", "§6.4", "6.11"),
        subject.clone(),
        copy("Accidental combination 6.11", "Außergewöhnliche Kombination 6.11"),
    )
    .utilization(q_force(ed), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("Accidental ULS: E_d = ΣG + A_d + ψ₁·Q₁ + Σψ₂·Qᵢ = {ed:.1} N (leading={leading})"),
        format!("Außergewöhnlicher GZT: E_d = ΣG + A_d + ψ₁·Q₁ + Σψ₂·Qᵢ = {ed:.1} N (führend={leading})"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,STR to at least {:.1} kN for accidental 6.11.", ed / 1000.0),
                format!("R_d,STR für außergewöhnliche Kombination 6.11 auf mindestens {:.1} kN erhöhen.", ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

pub(super) fn push_seismic<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet) {
    if actions.a_ed.abs() < f64::EPSILON {
        report.push(
            CheckResult::assess(
                format!("en1990.6.12b.{}", member.id),
                "EN 1990",
                ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Seismic combination 6.12b", "Erdbebenkombination 6.12b"),
            )
            .not_applicable(copy("No seismic action A_Ed defined.", "Keine seismische Einwirkung A_Ed definiert."))
            .annex(annex.choice())
            .build(),
        );
        return;
    }
    let ed = combination_6_12b(annex, actions, actions.a_ed);
    let rd = member.rd_str;
    let path = member_path(document, &member.id, "rdStr");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.6.12b.{}", member.id),
        "EN 1990",
        ClauseId::new("EN 1990", "§6.4.3.4", "6.12b"),
        subject.clone(),
        copy("Seismic combination 6.12b", "Erdbebenkombination 6.12b"),
    )
    .utilization(q_force(ed), q_force(rd))
    .annex(annex.choice())
    .explanation(copy(
        format!("Seismic ULS: E_d = ΣG + A_Ed + Σψ₂·Q = {ed:.1} N"),
        format!("Erdbeben-GZT: E_d = ΣG + A_Ed + Σψ₂·Q = {ed:.1} N"),
    ));
    if ed > rd {
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(rd),
            q_force(ed),
            copy(
                format!("Increase R_d,STR to at least {:.1} kN for seismic 6.12b.", ed / 1000.0),
                format!("R_d,STR für Erdbebenkombination 6.12b auf mindestens {:.1} kN erhöhen.", ed / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}

pub(super) fn push_uls_geo<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let ed_a = combination_str_geo_610a_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_b = combination_str_geo_610b_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let ed_610 = ed_a.max(ed_b);
    let rd = member.rd_geo;
    let path = member_path(document, &member.id, "rdGeo");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    for (rule, ed, section) in [("6.10a", ed_a, "6.10a"), ("6.10b", ed_b, "6.10b"), ("6.10", ed_610, "6.10")] { // governing 6.10 = max(6.10a,6.10b) per DIN EN 1990/NA NDP A1.3.1(4)
        let id = format!("en1990.geo.{}.{}.{leading}", member.id, rule);
        let mut builder = CheckResult::assess(
            id,
            if is_bridge(&document.structure_kind) { "EN 1990 GEO A2.4(B)" } else { "EN 1990 GEO A1.2(B)" },
            ClauseId::new("EN 1990", "§6.4", section),
            subject.clone(),
            geo_title(section),
        )
        .utilization(q_force(ed), q_force(rd))
        .annex(annex.choice())
        .explanation(copy(
            format!("E_d = {ed:.1} N, R_d,GEO = {rd:.1} N, leading={leading}"),
            format!("E_d = {ed:.1} N, R_d,GEO = {rd:.1} N, führend={leading}"),
        ));
        if ed > rd {
            builder = builder.remedy(Remedy::at_least(
                subject.clone(),
                q_force(rd),
                q_force(ed),
                copy(
                    format!("Increase R_d,GEO of {} to at least {:.1} kN.", member.label_en, ed / 1000.0),
                    format!("R_d,GEO von {} auf mindestens {:.1} kN erhöhen.", member.label_de, ed / 1000.0),
                ),
            ));
        }
        report.push(builder.build());
    }
}

pub(super) fn push_uls_equ_stab<A: NationalAnnex>(report: &mut CheckReport, annex: &A, document: &En1990Snapshot, member: &Member, actions: &ActionSet, leading: usize) {
    let k = k_fi(document.consequence_class);
    let ed_net = combination_equ_for(annex, actions, leading, document.consequence_class, &document.structure_kind);
    let demand = ed_net + k * (if is_bridge(&document.structure_kind) { 0.95 } else { 0.90 }) * actions.g_k_inf;
    let g_stab = k * (if is_bridge(&document.structure_kind) { 0.95 } else { 0.90 }) * actions.g_k_inf;
    let capacity = member.rd_equ_stab + g_stab;
    let path = member_path(document, &member.id, "rdEquStab");
    let subject = SubjectRef::new(&member.id, &path, member_label(member));
    let mut builder = CheckResult::assess(
        format!("en1990.equ.stab.{}.{leading}", member.id),
        if is_bridge(&document.structure_kind) { "EN 1990 EQU A2.4(A)" } else { "EN 1990 EQU A1.2(A)" },
        ClauseId::new("EN 1990", "Table A1.2(A)", "EQU-stab"),
        subject.clone(),
        copy("EQU stabilising", "EQU stabilisierend"),
    )
    .utilization(q_force(demand), q_force(capacity))
    .annex(annex.choice())
    .explanation(copy(
        format!("Destabilising demand {demand:.1} N vs stabilising capacity {capacity:.1} N"),
        format!("Entstabilisierende Beanspruchung {demand:.1} N gegenüber stabilisierender Kapazität {capacity:.1} N"),
    ));
    if demand > capacity {
        let required_rd = (demand - g_stab).max(0.0);
        builder = builder.remedy(Remedy::at_least(
            subject,
            q_force(member.rd_equ_stab),
            q_force(required_rd),
            copy(
                format!("Increase R_d,EQU (stabilising) of {} to at least {:.1} kN.", member.label_en, required_rd / 1000.0),
                format!("R_d,EQU (stabilisierend) von {} auf mindestens {:.1} kN erhöhen.", member.label_de, required_rd / 1000.0),
            ),
        ));
    }
    report.push(builder.build());
}
//#endregion 🔖️ComplianceReport
