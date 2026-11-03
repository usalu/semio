//! 🏗️ Ultimate limit state checks of the EN 1990 compliance report: STR, GEO sets B and C, EQU, accidental and seismic combinations generated from the member's actions. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
fn worst(ctx: &Context, actions: &[Action], equation: Equation, net: bool) -> Option<Combination> {
    combinations(ctx, actions, equation).ok().and_then(|all| governing(&all, net).cloned())
}

fn describe(combination: &Combination) -> String {
    let text = combination.describe();
    if combination.deviates() { format!("{text} [deviates from NA | weicht vom NA ab]") } else { text }
}

fn ultimate_equations(annex: AnnexChoice, domain: Domain) -> &'static [Equation] {
    if offers_610ab(annex, domain) { &[Equation::E610a, Equation::E610b] } else { &[Equation::E610] }
}

pub(super) fn push_uls_str_geo(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    for (kind, field, set, resistance, family) in [("str", "rdStr", FactorSet::StrB, member.rd_str, "STR"), ("geo", "rdGeo", FactorSet::StrB, member.rd_geo, "GEO")] {
        let Some(ctx) = context_for(document, set) else { return };
        for &equation in ultimate_equations(document.annex, ctx.domain) {
            let Some(combination) = worst(&ctx, actions, equation, false) else { continue };
            let title = (equation.title().0, equation.title().1);
            report.push(
                ResistanceCheck {
                    id: format!("en1990.{kind}.{}.{}", member.id, equation.label()),
                    part: annex_tag(document, set),
                    clause: ClauseId::new("EN 1990", "§6.4.3.2", equation.label()),
                    subject: member_subject(member, field),
                    title: copy(format!("{family} Eq. {} – {}", equation.label(), title.0), format!("{family} Gl. {} – {}", equation.label(), title.1)),
                    annex: document.annex,
                    effect: combination.total(),
                    resistance,
                    explanation: copy(
                        format!("E_d = {:.1} N ({}), R_d,{family} = {resistance:.1} N, K_FI = {:.2}", combination.total(), describe(&combination), ctx.k_fi),
                        format!("E_d = {:.1} N ({}), R_d,{family} = {resistance:.1} N, K_FI = {:.2}", combination.total(), describe(&combination), ctx.k_fi),
                    ),
                    remedy_en: &format!("Increase R_d,{family} of {}", member.label_en),
                    remedy_de: &format!("R_d,{family} von {} erhöhen", member.label_de),
                }
                .build_citing(&uls_ndps(document, false)),
            );
            let reversed = combinations_dir(&ctx, actions, equation, -1.0).ok().and_then(|all| governing_min(&all).cloned());
            if let Some(reversed) = reversed.filter(|c| c.total() < 0.0) {
                report.push(
                    ResistanceCheck {
                        id: format!("en1990.{kind}.{}.{}.min", member.id, equation.label()),
                        part: annex_tag(document, set),
                        clause: ClauseId::new("EN 1990", "§6.4.3.2", equation.label()),
                        subject: member_subject(member, field),
                        title: copy(format!("{family} Eq. {} – effect reversal (minimum)", equation.label()), format!("{family} Gl. {} – Vorzeichenwechsel (Minimum)", equation.label())),
                        annex: document.annex,
                        effect: -reversed.total(),
                        resistance,
                        explanation: copy(
                            format!("E_d,min = {:.1} N ({}); |E_d,min| against R_d,{family} = {resistance:.1} N", reversed.total(), describe(&reversed)),
                            format!("E_d,min = {:.1} N ({}); |E_d,min| gegenüber R_d,{family} = {resistance:.1} N", reversed.total(), describe(&reversed)),
                        ),
                        remedy_en: &format!("Increase R_d,{family} of {} for the reversed effect", member.label_en),
                        remedy_de: &format!("R_d,{family} von {} für die umgekehrte Wirkung erhöhen", member.label_de),
                    }
                    .build_citing(&uls_ndps(document, false)),
                );
            }
        }
    }
}

pub(super) fn push_geo_set_c(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    let Some(ctx) = context_for(document, FactorSet::GeoC) else { return };
    let Some(combination) = worst(&ctx, actions, Equation::E610, false) else { return };
    report.push(
        ResistanceCheck {
            id: format!("en1990.geo.{}.set-c", member.id),
            part: annex_tag(document, FactorSet::GeoC),
            clause: ClauseId::new("EN 1990", "Table A1.2(C)", "GEO"),
            subject: member_subject(member, "rdGeo"),
            title: copy("GEO set C", "GEO Satz C"),
            annex: document.annex,
            effect: combination.total(),
            resistance: member.rd_geo,
            explanation: copy(format!("GEO set C E_d = {:.1} N ({}) vs R_d,GEO = {:.1} N", combination.total(), describe(&combination), member.rd_geo), format!("GEO Satz C E_d = {:.1} N ({}) gegenüber R_d,GEO = {:.1} N", combination.total(), describe(&combination), member.rd_geo)),
            remedy_en: "Increase R_d,GEO for set C",
            remedy_de: "R_d,GEO für Satz C erhöhen",
        }
        .build_citing(&psi_ndps(document)),
    );
}

pub(super) fn push_uls_equ(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    let Some(ctx) = context_for(document, FactorSet::EquA) else { return };
    let Some(combination) = worst(&ctx, actions, Equation::E610, true) else { return };
    let (dst, stb) = (combination.unfavourable, combination.stabilising());
    let part = annex_tag(document, FactorSet::EquA);
    report.push(
        ResistanceCheck {
            id: format!("en1990.equ.destab.{}", member.id),
            part: part.clone(),
            clause: ClauseId::new("EN 1990", "Table A1.2(A)", "EQU"),
            subject: member_subject(member, "rdEquDestab"),
            title: copy("EQU equilibrium: net destabilising effect", "EQU Gleichgewicht: resultierende destabilisierende Wirkung"),
            annex: document.annex,
            effect: (dst - stb).max(0.0),
            resistance: member.rd_equ_destab,
            explanation: copy(format!("E_d,dst − E_d,stb = {dst:.1} − {stb:.1} N ({}) vs R_d,EQU = {:.1} N", describe(&combination), member.rd_equ_destab), format!("E_d,dst − E_d,stb = {dst:.1} − {stb:.1} N ({}) gegenüber R_d,EQU = {:.1} N", describe(&combination), member.rd_equ_destab)),
            remedy_en: &format!("Increase R_d,EQU (destabilising) of {}", member.label_en),
            remedy_de: &format!("R_d,EQU (entstabilisierend) von {} erhöhen", member.label_de),
        }
        .build_citing(&uls_ndps(document, true)),
    );
    let capacity = member.rd_equ_stab + stb;
    let mut builder = CheckResult::assess(format!("en1990.equ.stab.{}", member.id), part, ClauseId::new("EN 1990", "Table A1.2(A)", "EQU-stab"), member_subject(member, "rdEquStab"), copy("EQU equilibrium: stabilising capacity", "EQU Gleichgewicht: stabilisierende Kapazität"))
        .utilization(q_force(dst), q_force(capacity))
        .annex(document.annex)
        .explanation(copy(format!("E_d,dst = {dst:.1} N vs E_d,stb + R_d = {stb:.1} + {:.1} N", member.rd_equ_stab), format!("E_d,dst = {dst:.1} N gegenüber E_d,stb + R_d = {stb:.1} + {:.1} N", member.rd_equ_stab)));
    if dst > capacity {
        let required = (dst - stb).max(0.0);
        builder = builder.remedy(Remedy::at_least(
            member_subject(member, "rdEquStab"),
            q_force(member.rd_equ_stab),
            q_force(required),
            copy(format!("Increase R_d,EQU (stabilising) of {} to at least {:.1} kN.", member.label_en, required / 1000.0), format!("R_d,EQU (stabilisierend) von {} auf mindestens {:.1} kN erhöhen.", member.label_de, required / 1000.0)),
        ));
    }
    for id in uls_ndps(document, true) {
        builder = builder.ndp(id);
    }
    report.push(builder.ndp_reads().build());
}

fn design_action_check(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action], equation: Equation, absent: (&str, &str)) {
    let id = format!("en1990.{}.{}", equation.label(), member.id);
    let clause = ClauseId::new("EN 1990", if equation == Equation::E611b { "§6.4.3.3" } else { "§6.4.3.4" }, equation.label());
    let title = equation.title();
    let ctx = context_for(document, FactorSet::StrB);
    let combination = ctx.and_then(|ctx| worst(&ctx, actions, equation, false));
    let Some(combination) = combination else {
        report.push(not_applicable(id, "EN 1990", clause, member_subject(member, "rdStr"), copy(title.0, title.1), document.annex, copy(absent.0, absent.1)));
        return;
    };
    report.push(
        ResistanceCheck {
            id,
            part: "EN 1990".into(),
            clause,
            subject: member_subject(member, "rdStr"),
            title: copy(title.0, title.1),
            annex: document.annex,
            effect: combination.total(),
            resistance: member.rd_str,
            explanation: copy(format!("E_d = {:.1} N ({}) vs R_d = {:.1} N", combination.total(), describe(&combination), member.rd_str), format!("E_d = {:.1} N ({}) gegenüber R_d = {:.1} N", combination.total(), describe(&combination), member.rd_str)),
            remedy_en: &format!("Increase R_d,STR for Eq. {}", equation.label()),
            remedy_de: &format!("R_d,STR für Gl. {} erhöhen", equation.label()),
        }
        .build(),
    );
}

pub(super) fn push_accidental(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    design_action_check(report, document, member, actions, Equation::E611b, ("No accidental action A_d defined.", "Keine außergewöhnliche Einwirkung A_d definiert."));
}

pub(super) fn push_seismic(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    design_action_check(report, document, member, actions, Equation::E612b, ("No seismic action A_Ed defined.", "Keine seismische Einwirkung A_Ed definiert."));
}

/// 🚦️ The governing STR combination of one member per traffic load group of the `trafficGroups` table: the components of every other group are removed from the generator input, so the group acts alone with the permanent, wind and thermal actions; the governing group is reported last (EN 1990 A2.2.2, EN 1991-2 Table 4.4a).
pub(super) fn push_traffic_group_combinations(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    if !document.structure_kind.is_bridge() || document.traffic_groups.is_empty() {
        return;
    }
    let Some(ctx) = context_for(document, FactorSet::StrB) else { return };
    let groups: std::collections::BTreeSet<LoadGroup> = document.traffic_groups.iter().filter(|row| document.variables.iter().any(|variable| variable.id == row.variable_id)).map(|row| row.group).collect();
    let mut governing: Option<(LoadGroup, f64)> = None;
    for group in &groups {
        let others: Vec<&str> = document.traffic_groups.iter().filter(|row| row.group != *group).map(|row| row.variable_id.as_str()).collect();
        let kept: Vec<Action> = actions.iter().filter(|action| !others.contains(&action.id.as_str())).cloned().collect();
        let Some(combination) = worst(&ctx, &kept, ultimate_equations(document.annex, ctx.domain)[0], false) else { continue };
        let total = combination.total();
        if governing.map_or(true, |(_, best)| total > best) {
            governing = Some((*group, total));
        }
        report.push(
            CheckResult::assess(format!("en1990.a2.traffic-group-combination.{}.{}", group.as_str(), member.id), "EN 1990 Annex A2", ClauseId::new("EN 1990", "A2.2.2", "Table 4.4a"), member_subject(member, "rdStr"), copy(format!("Traffic group {} – governing STR combination", group.as_str()), format!("Verkehrslastgruppe {} – maßgebende STR-Kombination", group.as_str())))
                .informative(q_force(total))
                .annex(document.annex)
                .explanation(copy(format!("E_d = {total:.1} N ({}) with {} as the only traffic load group", describe(&combination), group.as_str()), format!("E_d = {total:.1} N ({}) mit {} als einziger Verkehrslastgruppe", describe(&combination), group.as_str())))
                .build(),
        );
    }
    if let Some((group, total)) = governing.filter(|_| groups.len() > 1) {
        report.push(
            CheckResult::assess(format!("en1990.a2.traffic-group-governing.{}", member.id), "EN 1990 Annex A2", ClauseId::new("EN 1990", "A2.2.2", "Table 4.4a"), member_subject(member, "rdStr"), copy("Governing traffic load group", "Maßgebende Verkehrslastgruppe"))
                .informative(q_force(total))
                .annex(document.annex)
                .explanation(copy(format!("{} governs {} with E_d = {total:.1} N among {} groups", group.as_str(), member.id, groups.len()), format!("{} ist für {} maßgebend mit E_d = {total:.1} N unter {} Gruppen", group.as_str(), member.id, groups.len())))
                .build(),
        );
    }
}
//#endregion 🔖️ComplianceReport
