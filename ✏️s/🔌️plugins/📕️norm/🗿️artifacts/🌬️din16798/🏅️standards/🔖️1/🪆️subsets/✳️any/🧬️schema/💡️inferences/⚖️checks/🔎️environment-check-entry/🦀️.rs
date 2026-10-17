//! 🔎️ DIN EN 16798 environment check entry: duplicate ids and the complete building evaluation.

use super::*;

/// ✅️ Evaluate the complete DIN EN 16798 building subject.
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
    let occupied: std::collections::BTreeSet<&str> = ids.iter().map(|s| s.as_str()).collect();
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(
            &id,
            path,
            loc(
                format!("Duplicate {table_en} id"),
                format!("Doppelte {table_de}-Id"),
            ),
        );
        let mut options = Vec::new();
        for suffix in ["-2", "-b", "-unique"] {
            let candidate = format!("{id}{suffix}");
            if !occupied.contains(candidate.as_str()) {
                options.push(candidate);
            }
            if options.len() >= 2 {
                break;
            }
        }
        if options.is_empty() {
            options.push(format!("{id}-renamed"));
        }
        let free = options[0].clone();
        report.push(
            CheckResult::assess(
                format!("din16798.integrity.duplicate.{table}.{id}"),
                "DIN EN 16798 integrity",
                ClauseId::new("EN 16798-1", "§5", "identity"),
                subject.clone(),
                loc(
                    format!("Unique {table_en} id"),
                    format!("Eindeutige {table_de}-Id"),
                ),
            )
            .annex(annex)
            .explanation(loc(
                format!("Duplicate {table_en} id '{id}' appears {count} times; each entity id must be unique."),
                format!("Die {table_de}-Id '{id}' ist {count}-fach vergeben; jede Entitäts-Id muss eindeutig sein."),
            ))
            .status(CheckStatus::Fail)
            .remedy(Remedy::one_of(
                subject,
                options,
                loc(
                    format!("Rename the duplicated '{id}' entry to a free id such as '{free}'."),
                    format!("Den doppelten Eintrag '{id}' auf eine freie Id wie '{free}' umbenennen."),
                ),
            ))
            .build(),
        );
    }
}

pub fn check_full_environment(doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot) -> CheckReport {
    let annex = annex_params::AnnexParams::for_choice(doc.annex);
    let mut report = CheckReport::default();
    push_duplicate_ids(
        &mut report,
        doc.annex,
        "zones",
        "zone",
        "Zone",
        &doc.zones.iter().map(|z| z.id.clone()).collect::<Vec<_>>(),
        &|id| format!("zones[id={id}].id"),
    );
    push_duplicate_ids(
        &mut report,
        doc.annex,
        "ventSystems",
        "ventilation system",
        "Lüftungsanlage",
        &doc.vent_systems.iter().map(|v| v.id.clone()).collect::<Vec<_>>(),
        &|id| format!("ventSystems[id={id}].id"),
    );
    if doc.zones.is_empty() {
        report.push(CheckResult::assess(
            "din16798.zones.empty", "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7", "scope"),
            SubjectRef::whole(loc("Building", "Gebäude")),
            loc("Zone inventory", "Zonenverzeichnis"),
        ).not_applicable(loc("No zones defined.", "Keine Zonen definiert.")).annex(doc.annex).build());
    }
    for (zi, zone) in doc.zones.iter().enumerate() {
        report.extend(evaluate_zone(doc, zi, zone, &annex));
    }
    for (vi, vent) in doc.vent_systems.iter().enumerate() {
        report.extend(evaluate_vent(doc, vi, vent, &annex));
    }
    report.extend(evaluate_envelope(doc, &annex));
    report
}
