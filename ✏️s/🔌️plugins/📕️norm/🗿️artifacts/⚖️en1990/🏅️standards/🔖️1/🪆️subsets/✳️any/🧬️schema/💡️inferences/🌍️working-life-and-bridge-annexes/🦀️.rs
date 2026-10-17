//! 🌍️ Design working life and bridge annex checks of the EN 1990 compliance report: the working life, annex B and the A2 bridge serviceability. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn push_design_working_life(report: &mut CheckReport, document: &En1990Snapshot) {
    use semio_s_artifact_norm_en1990_checks::design_working_life_indicative;
    let (lo, hi) = design_working_life_indicative(document.design_working_life_category);
    let years = document.design_working_life_years;
    let bridge = is_bridge(&document.structure_kind);
    let years_subject = SubjectRef::new(document.project_id.clone(), "designWorkingLifeYears", copy("Design working life", "Nutzungsdauer"));
    let cat_subject = SubjectRef::new(document.project_id.clone(), "designWorkingLifeCategory", copy("Design working life category", "Nutzungskategorie"));

    let years_ok = if bridge && document.design_working_life_category == 5 {
        (years - 100.0).abs() < 1e-6
    } else if bridge {
        false
    } else {
        years + 1e-9 >= lo && years - 1e-9 <= hi
    };
    let mut years_builder = CheckResult::assess(
        "en1990.table-2.1.design-working-life",
        "EN 1990 §2.3",
        ClauseId::new("EN 1990", "§2.3", "Table 2.1"),
        years_subject.clone(),
        copy("Design working life Table 2.1", "Nutzungsdauer Tabelle 2.1"),
    )
    .annex(document.annex)
    .explanation(copy(
        format!("Project {}: category {} indicative [{lo:.0}; {hi:.0}] a; declared {years:.0} a.", document.project_id, document.design_working_life_category),
        format!("Projekt {}: Kategorie {} indikativ [{lo:.0}; {hi:.0}] a; angegeben {years:.0} a.", document.project_id, document.design_working_life_category),
    ));
    if years_ok {
        years_builder = years_builder.status(semio_s_plugin_norm_compliance::CheckStatus::Pass);
    } else {
        let required_years = if bridge { 100.0 } else { hi };
        years_builder = years_builder.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::exactly(
            years_subject,
            q_dim(years),
            q_dim(required_years),
            copy(
                format!("Set design working life years to {required_years:.0} a."),
                format!("Nutzungsdauer auf {required_years:.0} a setzen."),
            ),
        ));
    }
    let mut years_built = years_builder.build();
    years_built.computed = q_dim(years);
    years_built.limit = q_dim(if bridge { 100.0 } else { hi });
    let lim = if bridge { 100.0 } else { hi }; years_built.utilization = if lim.abs() < f64::EPSILON { 0.0 } else { years / lim };
    report.push(years_built);

    let cat_ok = (1..=5).contains(&document.design_working_life_category) && (!bridge || document.design_working_life_category == 5);
    let mut cat_builder = CheckResult::assess(
        "en1990.table-2.1.category",
        "EN 1990 §2.3",
        ClauseId::new("EN 1990", "§2.3", "Table 2.1"),
        cat_subject.clone(),
        copy("Design working life category", "Nutzungskategorie"),
    )
    .annex(document.annex)
    .explanation(copy(
        format!("Project {}: declared category {}; bridges require Table 2.1 / A2 category 5 (100 a).", document.project_id, document.design_working_life_category),
        format!("Projekt {}: angegebene Kategorie {}; Brücken erfordern Tabelle 2.1 / A2 Kategorie 5 (100 a).", document.project_id, document.design_working_life_category),
    ));
    if cat_ok {
        cat_builder = cat_builder.status(semio_s_plugin_norm_compliance::CheckStatus::Pass);
    } else {
        let target = if bridge { 5u8 } else { 4u8 };
        cat_builder = cat_builder.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::one_of(
            cat_subject,
            vec![target.to_string()],
            copy(
                format!("Set design working life category to {target} (bridges: category 5)."),
                format!("Nutzungskategorie auf {target} setzen (Brücken: Kategorie 5)."),
            ),
        ));
    }
    let mut cat_built = cat_builder.build();
    cat_built.computed = q_dim(document.design_working_life_category as f64);
    cat_built.limit = q_dim(5.0);
    cat_built.utilization = document.design_working_life_category as f64 / 5.0;
    report.push(cat_built);
}

pub(super) fn push_annex_b_dsl_il(report: &mut CheckReport, document: &En1990Snapshot) {
    use semio_s_artifact_norm_en1990_checks::annex_b_dsl_il_for_rc;
    let (dsl_req, il_req) = annex_b_dsl_il_for_rc(document.reliability_class);
    for (id, path, declared, required, title_en, title_de) in [
        ("en1990.annex-b.dsl", "supervisionLevel", document.supervision_level.as_str(), dsl_req, "Design supervision level vs RC", "Überwachungsstufe gegenüber RC"),
        ("en1990.annex-b.il", "inspectionLevel", document.inspection_level.as_str(), il_req, "Inspection level vs RC", "Inspektionsstufe gegenüber RC"),
    ] {
        let subject = SubjectRef::new("", path, copy(title_en, title_de));
        let ok = declared == required;
        let mut builder = CheckResult::assess(id, "EN 1990 Annex B", ClauseId::new("EN 1990", "Annex B", path), subject.clone(), copy(title_en, title_de))
            .annex(document.annex)
            .explanation(copy(
                format!("Declared {declared} for RC{}; Annex B recommends {required}.", document.reliability_class),
                format!("Angegeben {declared} für RC{}; Anhang B empfiehlt {required}.", document.reliability_class),
            ));
        if ok {
            builder = builder.status(semio_s_plugin_norm_compliance::CheckStatus::Pass);
        } else {
            builder = builder.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::one_of(
                subject,
                vec![required.to_string()],
                copy(
                    format!("Set {path} to {required} for the reliability class."),
                    format!("{path} für die Zuverlässigkeitsklasse auf {required} setzen."),
                ),
            ));
        }
        let mut built = builder.build();
        built.computed = q_dim(if ok { 1.0 } else { 0.0 });
        built.limit = q_dim(1.0);
        built.utilization = if ok { 1.0 } else { 2.0 };
        report.push(built);
    }
}

pub(super) fn push_a2_bridge_sls(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if !is_bridge(&document.structure_kind) {
        report.push(
            CheckResult::assess(
                format!("en1990.a2.bridge.{}", member.id),
                "EN 1990 Annex A2",
                ClauseId::new("EN 1990", "Annex A2", "SLS"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Annex A2 bridge SLS", "Anhang A2 Brücken-GZG"),
            )
            .not_applicable(copy(
                "Structure kind is not a bridge; Annex A2 SLS criteria live on bridgeSls[].",
                "Tragwerksart ist keine Brücke; Anhang-A2-GZG-Kriterien liegen auf bridgeSls[].",
            ))
            .annex(document.annex)
            .build(),
        );
        return;
    }
    let Some(sls) = document.bridge_sls.iter().find(|b| b.member_id == member.id) else {
        report.push(
            CheckResult::assess(
                format!("en1990.a2.bridge.{}", member.id),
                "EN 1990 Annex A2",
                ClauseId::new("EN 1990", "Annex A2", "SLS"),
                SubjectRef::new(&member.id, member_path(document, &member.id, "rdStr"), member_label(member)),
                copy("Annex A2 bridge SLS", "Anhang A2 Brücken-GZG"),
            )
            .not_applicable(copy(
                format!("No bridgeSls entry for member {}.", member.id),
                format!("Kein bridgeSls-Eintrag für Bauteil {}.", member.id),
            ))
            .annex(document.annex)
            .build(),
        );
        return;
    };
    let sls_path = |field: &str| format!("bridgeSls[id={}].{field}", sls.id);
    if matches!(document.structure_kind.as_str(), "footbridge" | "rail_bridge") {
        let path = sls_path("deckAcceleration");
        let subject = SubjectRef::new(&sls.id, &path, member_label(member));
        let mut builder = CheckResult::assess(
            format!("en1990.a2.acceleration.{}.{}", member.id, sls.id),
            "EN 1990 Annex A2.4",
            ClauseId::new("EN 1990", "Annex A2.4", "acceleration"),
            subject.clone(),
            copy("Deck acceleration", "Deckbeschleunigung"),
        )
        .utilization(q_dim(sls.deck_acceleration), q_dim(sls.deck_acceleration_limit))
        .annex(document.annex)
        .explanation(copy(
            format!("a = {:.3} m/s² vs limit {:.3} m/s²", sls.deck_acceleration, sls.deck_acceleration_limit),
            format!("a = {:.3} m/s² gegenüber Grenze {:.3} m/s²", sls.deck_acceleration, sls.deck_acceleration_limit),
        ));
        if sls.deck_acceleration > sls.deck_acceleration_limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                q_dim(sls.deck_acceleration),
                q_dim(sls.deck_acceleration_limit),
                copy(
                    format!("Reduce deck acceleration to at most {:.3} m/s².", sls.deck_acceleration_limit),
                    format!("Deckbeschleunigung auf höchstens {:.3} m/s² reduzieren.", sls.deck_acceleration_limit),
                ),
            ));
        }
        report.push(builder.build());
    }
    if document.structure_kind == "rail_bridge" {
        let path = sls_path("deckTwist");
        let subject = SubjectRef::new(&sls.id, &path, member_label(member));
        let mut builder = CheckResult::assess(
            format!("en1990.a2.twist.{}.{}", member.id, sls.id),
            "EN 1990 Annex A2.4",
            ClauseId::new("EN 1990", "Annex A2.4", "twist"),
            subject.clone(),
            copy("Deck twist", "Deckverwindung"),
        )
        .utilization(q_dim(sls.deck_twist), q_dim(sls.deck_twist_limit))
        .annex(document.annex)
        .explanation(copy(
            format!("twist = {:.5} rad vs limit {:.5} rad", sls.deck_twist, sls.deck_twist_limit),
            format!("Verwindung = {:.5} rad gegenüber Grenze {:.5} rad", sls.deck_twist, sls.deck_twist_limit),
        ));
        if sls.deck_twist > sls.deck_twist_limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                q_dim(sls.deck_twist),
                q_dim(sls.deck_twist_limit),
                copy(
                    format!("Reduce deck twist to at most {:.5} rad.", sls.deck_twist_limit),
                    format!("Deckverwindung auf höchstens {:.5} rad reduzieren.", sls.deck_twist_limit),
                ),
            ));
        }
        report.push(builder.build());
    }
    if sls.bridge_deflection_limit > 0.0 {
        let path = sls_path("bridgeDeflection");
        let subject = SubjectRef::new(&sls.id, &path, member_label(member));
        let mut builder = CheckResult::assess(
            format!("en1990.a2.deflection.{}.{}", member.id, sls.id),
            "EN 1990 Annex A2.4",
            ClauseId::new("EN 1990", "Annex A2.4", "deflection"),
            subject.clone(),
            copy("Bridge vertical deflection", "Vertikale Brückendurchbiegung"),
        )
        .utilization(q_len(sls.bridge_deflection), q_len(sls.bridge_deflection_limit))
        .annex(document.annex)
        .explanation(copy(
            format!("δ = {:.1} mm vs limit {:.1} mm", sls.bridge_deflection * 1000.0, sls.bridge_deflection_limit * 1000.0),
            format!("δ = {:.1} mm gegenüber Grenze {:.1} mm", sls.bridge_deflection * 1000.0, sls.bridge_deflection_limit * 1000.0),
        ));
        if sls.bridge_deflection > sls.bridge_deflection_limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                q_len(sls.bridge_deflection),
                q_len(sls.bridge_deflection_limit),
                copy(
                    format!("Reduce bridge deflection to at most {:.1} mm.", sls.bridge_deflection_limit * 1000.0),
                    format!("Brückendurchbiegung auf höchstens {:.1} mm reduzieren.", sls.bridge_deflection_limit * 1000.0),
                ),
            ));
        }
        report.push(builder.build());
    }
}
//#endregion 🔖️ComplianceReport
