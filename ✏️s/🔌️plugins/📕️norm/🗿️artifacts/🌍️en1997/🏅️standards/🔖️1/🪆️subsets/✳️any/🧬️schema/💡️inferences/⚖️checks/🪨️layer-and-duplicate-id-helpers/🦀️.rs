//! 🪨️ EN 1997 layer lookup, required-width search, design-situation labels and the duplicate-id and governing-situation checks.

use super::*;

pub(super) fn layer_at(layers: &[SoilLayer], depth: f64) -> Option<&SoilLayer> {
    layers.iter().find(|l| depth >= l.depth_top && depth <= l.depth_bottom).or_else(|| layers.first())
}

pub(super) fn design_actions(p: &AnnexParams, g: f64, q: f64) -> f64 { p.gamma_g * g + p.gamma_q * q }

pub(super) fn find_required_width(
    footing: &SpreadFoundation, layer: &SoilLayer, phi_deg: f64, lc: &FoundationLoadCase,
    approach: DesignApproach, annex: AnnexChoice, situation: &str, gwl: f64, start: f64,
) -> f64 {
    let p = resolve_params(approach, annex, situation);
    let length = footing.length.max(0.3);
    let mut b = start.max(0.3);
    for _ in 0..80 {
        let v = design_actions(&p, lc.vertical_permanent, lc.vertical_variable);
        let h = design_actions(&p, lc.horizontal_permanent, lc.horizontal_variable);
        let m = design_actions(&p, lc.moment_permanent, lc.moment_variable);
        let e = m / v.max(1.0);
        let gamma_b = part_1::effective_gamma_below_pa_m(layer.gamma, layer.gamma_prime, footing.embedment, gwl, b);
        let r = part_1::design_bearing_resistance_with_params(
            phi_deg, layer.cohesion_effective, layer.gamma, gamma_b, b, length, footing.embedment,
            e, 0.0, h, v, footing.base_inclination_deg, &p,
        );
        if v <= r {
            return ((b * 1.02) * 1000.0).ceil() / 1000.0;
        }
        b *= 1.08;
        if b > 80.0 {
            break;
        }
    }
    b
}

pub(super) fn situation_short(situation: &str) -> &'static str {
    match parse_design_situation(situation) {
        DesignSituation::Transient => "BS-T",
        DesignSituation::Accidental => "BS-A",
        DesignSituation::Persistent => "BS-P",
    }
}

pub(super) fn situation_for_load_case(doc: &En1997Snapshot, lc: &FoundationLoadCase) -> String {
    if !lc.design_situation.trim().is_empty() {
        lc.design_situation.clone()
    } else {
        doc.design_situation.clone()
    }
}

/// 🔗 Fail when an id-bearing collection contains duplicate entity ids.
fn push_duplicate_ids(
    report: &mut CheckReport,
    annex: AnnexChoice,
    table: &str,
    table_en: &str,
    table_de: &str,
    ids: &[String],
    path_for: &dyn Fn(&str) -> String,
) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = subject_entity(
            &id,
            &path,
            &format!("Duplicate {table_en} id"),
            &format!("Doppelte {table_de}-Id"),
        );
        let free = format!("{id}-2");
        report.push(
            CheckResult::assess(
                format!("en1997.integrity.duplicate.{table}.{id}"),
                "EN 1997 integrity",
                ClauseId::new("EN 1997-1", "1", "1.5"),
                subject.clone(),
                loc(
                    &format!("Unique {table_en} id"),
                    &format!("Eindeutige {table_de}-Id"),
                ),
            )
            .annex(annex)
            .explanation(loc(
                &format!("Duplicate {table_en} id '{id}' appears {count} times; each entity id must be unique."),
                &format!("Doppelte {table_de}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
            ))
            .status(CheckStatus::Fail)
            .remedy(Remedy::one_of(
                subject,
                vec![free.clone(), format!("{id}-unique")],
                loc(
                    &format!("Rename the duplicated '{id}' entry to a free id such as '{free}'."),
                    &format!("Den doppelten '{id}'-Eintrag auf eine freie Id wie '{free}' umbenennen."),
                ),
            ))
            .build(),
        );
    }
}

pub(super) fn push_all_duplicate_ids(report: &mut CheckReport, doc: &En1997Snapshot) {
    let annex = doc.annex;
    push_duplicate_ids(
        report,
        annex,
        "layers",
        "layer",
        "Schicht",
        &doc.layers.iter().map(|l| l.id.clone()).collect::<Vec<_>>(),
        &|id| format!("layers[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "footings",
        "footing",
        "Fundament",
        &doc.footings.iter().map(|f| f.id.clone()).collect::<Vec<_>>(),
        &|id| format!("footings[id={id}].id"),
    );
    for footing in &doc.footings {
        push_duplicate_ids(
            report,
            annex,
            "loadCases",
            "load case",
            "Lastfall",
            &footing.load_cases.iter().map(|lc| lc.id.clone()).collect::<Vec<_>>(),
            &|id| format!("footings[id={}].loadCases[id={id}].id", footing.id),
        );
    }
    push_duplicate_ids(
        report,
        annex,
        "piles",
        "pile",
        "Pfahl",
        &doc.piles.iter().map(|p| p.id.clone()).collect::<Vec<_>>(),
        &|id| format!("piles[id={id}].id"),
    );
    for pile in &doc.piles {
        push_duplicate_ids(
            report,
            annex,
            "testProfiles",
            "test profile",
            "Prüfprofil",
            &pile.test_profiles.iter().map(|t| t.id.clone()).collect::<Vec<_>>(),
            &|id| format!("piles[id={}].testProfiles[id={id}].id", pile.id),
        );
    }
    push_duplicate_ids(
        report,
        annex,
        "retainingWalls",
        "retaining wall",
        "Stützwand",
        &doc.retaining_walls.iter().map(|w| w.id.clone()).collect::<Vec<_>>(),
        &|id| format!("retainingWalls[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "slopes",
        "slope",
        "Böschung",
        &doc.slopes.iter().map(|s| s.id.clone()).collect::<Vec<_>>(),
        &|id| format!("slopes[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "upliftCases",
        "uplift case",
        "Auftriebslastfall",
        &doc.uplift_cases.iter().map(|u| u.id.clone()).collect::<Vec<_>>(),
        &|id| format!("upliftCases[id={id}].id"),
    );
}

pub(super) fn push_governing_situation_summary(
    report: &mut CheckReport,
    id: String,
    part: &str,
    annex: AnnexChoice,
    subject: SubjectRef,
    sit: &str,
    approach: &str,
    util: f64,
    governing_check: &str,
) {
    let sit_lbl = situation_short(sit);
    report.push(
        CheckResult::assess(
            id,
            part,
            ClauseId::new("EN 1997-1", "2", "2.4.7"),
            subject,
            loc("Governing design situation", "Maßgebende Bemessungssituation"),
        )
        .utilization(
            Quantity::new(QuantityKind::Dimensionless, util),
            Quantity::new(QuantityKind::Dimensionless, util.max(1.0)),
        )
        .annex(annex)
        .explanation(loc(
            &format!("Governing: {sit_lbl} ({sit}) + approach {approach}; check {governing_check}; u={util:.3}."),
            &format!("Maßgebend: {sit_lbl} ({sit}) + Ansatz {approach}; Nachweis {governing_check}; u={util:.3}."),
        ))
        .build(),
    );
}

pub(super) fn id_path(collection: &str, id: &str, field: &str) -> String {
    format!("{collection}[id={id}].{field}")
}
