//! ⛓️ Referential integrity of the EN 1990 compliance report: the action and member ids, duplicate and dangling references and unassessable members. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
/// 📋️ `En1990Snapshot -> CheckReport` — full basis-of-design conformance.

fn action_ids(document: &En1990Snapshot) -> Vec<String> {
    let mut ids = Vec::new();
    ids.extend(document.permanents.iter().map(|a| a.id.clone()));
    ids.extend(document.variables.iter().map(|a| a.id.clone()));
    ids.extend(document.accidentals.iter().map(|a| a.id.clone()));
    ids.extend(document.seismics.iter().map(|a| a.id.clone()));
    ids
}

fn member_ids(document: &En1990Snapshot) -> Vec<String> {
    document.members.iter().map(|m| m.id.clone()).collect()
}

fn push_duplicate_ids(report: &mut CheckReport, document: &En1990Snapshot, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, copy(format!("Duplicate {table} id"), format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let mut builder = CheckResult::assess(
            format!("en1990.integrity.duplicate.{table}.{id}"),
            "EN 1990 integrity",
            ClauseId::new("EN 1990", "§2", "id"),
            subject.clone(),
            copy(format!("Unique {table} id"), format!("Eindeutige {table}-Id")),
        )
        .annex(document.annex)
        .explanation(copy(
            format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail);
        builder = builder.remedy(Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            copy(
                format!("Rename the duplicated '{id}' entry to a free id."),
                format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

fn push_dangling_ref(report: &mut CheckReport, document: &En1990Snapshot, check_id: String, path: String, subject_id: &str, label_en: &str, label_de: &str, current: &str, options: Vec<String>) {
    let subject = SubjectRef::new(subject_id, &path, copy(label_en, label_de));
    let opts = if options.is_empty() { vec![format!("{current}-missing")] } else { options };
    report.push(
        CheckResult::assess(
            check_id,
            "EN 1990 integrity",
            ClauseId::new("EN 1990", "§2", "reference"),
            subject.clone(),
            copy(format!("Referential integrity — {label_en}"), format!("Referenzintegrität — {label_de}")),
        )
        .annex(document.annex)
        .explanation(copy(
            format!("'{current}' does not reference an existing target; dependent combination checks for this row must not Pass."),
            format!("'{current}' verweist auf kein vorhandenes Ziel; abhängige Kombinationsnachweise für diese Zeile dürfen nicht bestehen."),
        ))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .remedy(Remedy::one_of(
            subject,
            opts,
            copy(
                format!("Set {label_en} to one of the existing target ids."),
                format!("{label_de} auf eine der vorhandenen Ziel-Ids setzen."),
            ),
        ))
        .build(),
    );
}

/// 🔗 Referential integrity + unique entity ids (CORRECTION 14:42). Returns members that must not silently Pass combination checks.
pub(super) fn push_referential_integrity(report: &mut CheckReport, document: &En1990Snapshot) -> std::collections::BTreeSet<String> {
    let actions = action_ids(document);
    let members = member_ids(document);
    let action_set: std::collections::BTreeSet<&str> = actions.iter().map(String::as_str).collect();
    let member_set: std::collections::BTreeSet<&str> = members.iter().map(String::as_str).collect();
    let mut broken = std::collections::BTreeSet::new();

    push_duplicate_ids(report, document, "permanents", &document.permanents.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("permanents[id={id}].id"));
    push_duplicate_ids(report, document, "variables", &document.variables.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("variables[id={id}].id"));
    push_duplicate_ids(report, document, "accidentals", &document.accidentals.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("accidentals[id={id}].id"));
    push_duplicate_ids(report, document, "seismics", &document.seismics.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("seismics[id={id}].id"));
    push_duplicate_ids(report, document, "members", &members, &|id| format!("members[id={id}].id"));
    push_duplicate_ids(report, document, "bridgeSls", &document.bridge_sls.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("bridgeSls[id={id}].id"));

    for (i, effect) in document.effects.iter().enumerate() {
        let base = format!("effects[{i}]");
        if !member_set.contains(effect.member_id.as_str()) {
            broken.insert(effect.member_id.clone());
            push_dangling_ref(report, document, format!("en1990.integrity.effects.{i}.memberId"), format!("{base}.memberId"), &effect.member_id, "Effect memberId", "Wirkung memberId", &effect.member_id, members.clone());
        }
        if !action_set.contains(effect.action_id.as_str()) {
            broken.insert(effect.member_id.clone());
            push_dangling_ref(report, document, format!("en1990.integrity.effects.{i}.actionId"), format!("{base}.actionId"), &effect.action_id, "Effect actionId", "Wirkung actionId", &effect.action_id, actions.clone());
        }
    }

    for sls in &document.bridge_sls {
        if !member_set.contains(sls.member_id.as_str()) {
            broken.insert(sls.member_id.clone());
            push_dangling_ref(
                report,
                document,
                format!("en1990.integrity.bridgeSls.{}.memberId", sls.id),
                format!("bridgeSls[id={}].memberId", sls.id),
                &sls.member_id,
                "Bridge SLS memberId",
                "Brücken-SLS memberId",
                &sls.member_id,
                members.clone(),
            );
        }
    }

    if !document.effects.is_empty() {
        for m in &document.members {
            let wired = document.effects.iter().any(|e| e.member_id == m.id && action_set.contains(e.action_id.as_str()));
            if !wired {
                broken.insert(m.id.clone());
            }
        }
    }
    broken
}

pub(super) fn push_member_unassessable(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let path = member_path(document, &member.id, "id");
    report.push(
        CheckResult::assess(
            format!("en1990.integrity.member.{}.unassessable", member.id),
            "EN 1990 integrity",
            ClauseId::new("EN 1990", "§2", "reference"),
            SubjectRef::new(&member.id, &path, member_label(member)),
            copy("Member effects wiring", "Bauteil-Wirkungseinbindung"),
        )
        .annex(document.annex)
        .explanation(copy(
            format!("Member '{}' has broken or missing action wiring; ULS/SLS combination checks are not assessed as Pass.", member.id),
            format!("Bauteil '{}' hat fehlerhafte oder fehlende Wirkungseinbindung; ULS/SLS-Kombinationsnachweise gelten nicht als bestanden.", member.id),
        ))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .remedy(Remedy::one_of(
            SubjectRef::new(&member.id, &path, member_label(member)),
            member_ids(document),
            copy(
                "Repair effects[].memberId / effects[].actionId to existing catalogue ids.",
                "effects[].memberId / effects[].actionId auf vorhandene Katalog-Ids reparieren.",
            ),
        ))
        .build(),
    );
}
//#endregion 🔖️ComplianceReport
