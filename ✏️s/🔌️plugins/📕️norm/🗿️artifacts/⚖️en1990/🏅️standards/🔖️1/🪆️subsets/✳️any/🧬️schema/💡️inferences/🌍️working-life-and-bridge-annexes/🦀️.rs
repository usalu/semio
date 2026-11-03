//! 🌍️ Design working life, Annex B quality levels and Annex A2 bridge serviceability of the EN 1990 compliance report. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn push_design_working_life(report: &mut CheckReport, document: &En1990Snapshot) {
    let years = document.design_working_life_years;
    let category = document.design_working_life_category;
    let years_subject = SubjectRef::new(document.project_id.clone(), "designWorkingLifeYears", copy("Design working life", "Nutzungsdauer"));
    let cat_subject = SubjectRef::new(document.project_id.clone(), "designWorkingLifeCategory", copy("Design working life category", "Nutzungskategorie"));
    let clause = ClauseId::new("EN 1990", "§2.3", "Table 2.1");
    let Some((lo, hi)) = design_working_life_indicative(category) else {
        report.push(
            CheckResult::assess("en1990.table-2.1.category", "EN 1990 §2.3", clause, cat_subject.clone(), copy("Design working life category", "Nutzungskategorie"))
                .status(CheckStatus::Fail)
                .annex(document.annex)
                .explanation(copy(format!("Category {category} is not defined; Table 2.1 knows 1..5."), format!("Kategorie {category} ist nicht definiert; Tabelle 2.1 kennt 1..5.")))
                .remedy(Remedy::one_of(cat_subject, (1..=5).map(|c| c.to_string()).collect(), copy("Set the design working life category to 1..5.", "Nutzungskategorie auf 1..5 setzen.")))
                .build(),
        );
        return;
    };
    let within = years + 1e-9 >= lo && years - 1e-9 <= hi;
    let mut builder = CheckResult::assess("en1990.table-2.1.design-working-life", "EN 1990 §2.3", clause.clone(), years_subject.clone(), copy("Design working life Table 2.1", "Nutzungsdauer Tabelle 2.1"))
        .utilization(q_dim(years), q_dim(hi))
        .status(if within { CheckStatus::Pass } else { CheckStatus::Warning })
        .annex(document.annex)
        .explanation(copy(
            format!("Project {}: category {category} indicative [{lo:.0}; {hi:.0}] a; declared {years:.0} a.", document.project_id),
            format!("Projekt {}: Kategorie {category} indikativ [{lo:.0}; {hi:.0}] a; angegeben {years:.0} a.", document.project_id),
        ));
    if !within {
        let required = years.clamp(lo, hi);
        builder = builder.remedy(Remedy::exactly(years_subject.clone(), q_dim(years), q_dim(required), copy(format!("Set design working life years to {required:.0} a."), format!("Nutzungsdauer auf {required:.0} a setzen."))));
    }
    report.push(builder.build());
    if document.structure_kind.is_bridge() {
        let hundred = (years - 100.0).abs() < 1e-6;
        let mut bridge = CheckResult::assess("en1990.a2.working-life", "EN 1990 Annex A2", ClauseId::new("EN 1990", "A2.1", "A2.1(1)"), years_subject.clone(), copy("Bridge design working life 100 a", "Brücken-Nutzungsdauer 100 a"))
            .utilization(q_dim(years.min(100.0)), q_dim(100.0))
            .status(if hundred { CheckStatus::Pass } else { CheckStatus::Warning })
            .annex(document.annex)
            .explanation(copy(format!("Bridges are designed for 100 a unless specified otherwise; declared {years:.0} a."), format!("Brücken werden für 100 a bemessen, sofern nichts anderes festgelegt ist; angegeben {years:.0} a.")));
        if !hundred {
            bridge = bridge.remedy(Remedy::exactly(years_subject, q_dim(years), q_dim(100.0), copy("Set the design working life of the bridge to 100 a (Annex A2.1(1)) or specify the deviation in the project.", "Nutzungsdauer der Brücke auf 100 a setzen (Anhang A2.1(1)) oder die Abweichung im Projekt festlegen.")));
        }
        report.push(bridge.build());
    }
}

pub(super) fn push_annex_b_dsl_il(report: &mut CheckReport, document: &En1990Snapshot) {
    let Some((dsl_req, il_req)) = annex_b_levels(document.reliability_class.number()) else { return };
    for (id, path, prefix, declared, required, title_en, title_de, clause) in [
        ("en1990.annex-b.dsl", "supervisionLevel", "DSL", document.supervision_level.as_str(), dsl_req, "Design supervision level vs RC", "Entwurfsüberwachung gegenüber RC", "Table B4"),
        ("en1990.annex-b.il", "inspectionLevel", "IL", document.inspection_level.as_str(), il_req, "Inspection level vs RC", "Ausführungsüberwachung gegenüber RC", "Table B5"),
    ] {
        let subject = SubjectRef::new(document.project_id.clone(), path, copy(title_en, title_de));
        let builder = CheckResult::assess(id, "EN 1990 Annex B", ClauseId::new("EN 1990", "Annex B", clause), subject.clone(), copy(title_en, title_de)).annex(document.annex);
        let recommendation = format!("{prefix}{required}");
        let remedy = |text_en: String, text_de: String| Remedy::one_of(subject.clone(), vec![recommendation.clone()], copy(text_en, text_de));
        let built = match parse_level(declared, prefix) {
            None => builder
                .status(CheckStatus::Fail)
                .explanation(copy(format!("'{declared}' is not {prefix}1..{prefix}3."), format!("'{declared}' ist nicht {prefix}1..{prefix}3.")))
                .remedy(remedy(format!("Set {path} to {recommendation}."), format!("{path} auf {recommendation} setzen.")))
                .build(),
            Some(level) => {
                let mut level_check = builder
                    .minimum(q_dim(f64::from(level)), q_dim(f64::from(required)))
                    .status(if level >= required { CheckStatus::Pass } else { CheckStatus::Warning })
                    .explanation(copy(format!("Declared {declared} for RC{}; Annex B recommends at least {recommendation}.", document.reliability_class.number()), format!("Angegeben {declared} für RC{}; Anhang B empfiehlt mindestens {recommendation}.", document.reliability_class.number())));
                if level < required {
                    level_check = level_check.remedy(remedy(format!("Raise {path} to {recommendation}."), format!("{path} auf {recommendation} anheben.")));
                }
                level_check.build()
            }
        };
        report.push(built);
    }
}

fn a2_subject(sls_id: &str, member: &Member, field: &str) -> SubjectRef {
    SubjectRef::new(sls_id, format!("bridgeSls[id={sls_id}].{field}"), member_label(member))
}

fn a2_limit(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, sls: &BridgeSls, key: &str, field: &str, clause: &str, title: (&str, &str), value: f64, limit: f64, kind: QuantityKind, remedy: (String, String)) {
    let subject = a2_subject(&sls.id, member, field);
    let mut builder = CheckResult::assess(format!("en1990.a2.{key}.{}.{}", member.id, sls.id), "EN 1990 Annex A2.4", ClauseId::new("EN 1990", format!("A{clause}"), clause), subject.clone(), copy(title.0, title.1))
        .utilization(Quantity::new(kind, value), Quantity::new(kind, limit))
        .annex(document.annex)
        .explanation(copy(format!("{value:.4} vs limit {limit:.4}"), format!("{value:.4} gegenüber Grenze {limit:.4}")));
    if value > limit {
        builder = builder.remedy(Remedy::at_most(subject, Quantity::new(kind, value), Quantity::new(kind, limit), copy(remedy.0, remedy.1)));
    }
    report.push(builder.build());
}

pub(super) fn push_a2_bridge_sls(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let id = format!("en1990.a2.bridge.{}", member.id);
    let clause = ClauseId::new("EN 1990", "Annex A2", "SLS");
    let title = copy("Annex A2 bridge SLS", "Anhang A2 Brücken-GZG");
    let subject = member_subject(member, "rdStr");
    if !document.structure_kind.is_bridge() {
        report.push(not_applicable(id, "EN 1990 Annex A2", clause, subject, title, document.annex, copy("Structure kind is not a bridge; Annex A2 SLS criteria live on bridgeSls[].", "Tragwerksart ist keine Brücke; Anhang-A2-GZG-Kriterien liegen auf bridgeSls[].")));
        return;
    }
    let Some(sls) = document.bridge_sls.iter().find(|b| b.member_id == member.id) else {
        report.push(not_applicable(id, "EN 1990 Annex A2", clause, subject, title, document.annex, copy(format!("No bridgeSls entry for member {}.", member.id), format!("Kein bridgeSls-Eintrag für Bauteil {}.", member.id))));
        return;
    };
    let rail = document.structure_kind == semio_s_artifact_norm_en1990_model::StructureKind::RailBridge;
    if document.structure_kind == semio_s_artifact_norm_en1990_model::StructureKind::Footbridge || rail {
        a2_limit(report, document, member, sls, "acceleration", "deckAcceleration", if rail { "2.4.4.2.1" } else { "2.4.3.2" }, ("Deck acceleration", "Deckbeschleunigung"), sls.deck_acceleration, sls.deck_acceleration_limit, QuantityKind::Acceleration, (format!("Reduce deck acceleration to at most {:.3} m/s².", sls.deck_acceleration_limit), format!("Deckbeschleunigung auf höchstens {:.3} m/s² reduzieren.", sls.deck_acceleration_limit)));
    }
    if rail {
        a2_limit(report, document, member, sls, "twist", "deckTwist", "2.4.4.2.2", ("Deck twist", "Deckverwindung"), sls.deck_twist, sls.deck_twist_limit, QuantityKind::Dimensionless, (format!("Reduce deck twist to at most {:.5} rad.", sls.deck_twist_limit), format!("Deckverwindung auf höchstens {:.5} rad reduzieren.", sls.deck_twist_limit)));
    }
    if sls.bridge_deflection_limit > 0.0 {
        a2_limit(report, document, member, sls, "deflection", "bridgeDeflection", "2.4.4.3", ("Bridge vertical deflection", "Vertikale Brückendurchbiegung"), sls.bridge_deflection, sls.bridge_deflection_limit, QuantityKind::Length, (format!("Reduce bridge deflection to at most {:.1} mm.", sls.bridge_deflection_limit * 1000.0), format!("Brückendurchbiegung auf höchstens {:.1} mm reduzieren.", sls.bridge_deflection_limit * 1000.0)));
    }
}
//#endregion 🔖️ComplianceReport
