//! 🧾️ EN 1999 structure evaluation: the duplicate-id check and `evaluate_structure`, which runs every clause check over a whole snapshot.

use super::*;

//#region 🔖️ComplianceHelpers
/// 📋️ Exhaustive evaluate for the aluminium-structure subject.
fn push_duplicate_id_fails(report: &mut CheckReport, annex: AnnexChoice, list_path: &str, label_en: &str, label_de: &str, ids: impl IntoIterator<Item = String>) {
    use std::collections::{BTreeMap, BTreeSet};
    let id_list: Vec<String> = ids.into_iter().collect();
    let occupied: BTreeSet<&str> = id_list.iter().map(|s| s.as_str()).collect();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for id in &id_list {
        *counts.entry(id.clone()).or_default() += 1;
    }
    for (id, n) in counts {
        if n < 2 { continue; }
        let mut options = Vec::new();
        for suffix in ["-2", "-b", "-unique"] {
            let candidate = format!("{id}{suffix}");
            if !occupied.contains(candidate.as_str()) {
                options.push(candidate);
            }
            if options.len() >= 2 { break; }
        }
        if options.is_empty() {
            options.push(format!("{id}-renamed"));
        }
        let free = options[0].clone();
        let subject = SubjectRef::new(&id, format!("{list_path}[id={id}].id"), loc(label_en, label_de));
        report.push(
            CheckResult::assess(
                format!("en1999.ref.duplicate.{list_path}.{id}"),
                "DIN EN 1999-1-1",
                ClauseId::new("EN 1999-1-1", "§1", "1"),
                subject.clone(),
                loc("Unique entity id", "Eindeutige Entitäts-Id"),
            )
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .explanation(loc(
                &format!("Duplicate {label_en} id '{id}' appears {n} times; each entity id must be unique."),
                &format!("Die {label_de}-Id '{id}' ist {n}-fach vergeben; jede Entitäts-Id muss eindeutig sein."),
            ))
            .annex(annex)
            .remedy(Remedy::one_of(
                subject,
                options,
                loc(
                    &format!("Rename the duplicated '{id}' entry to a free id such as '{free}'."),
                    &format!("Den doppelten Eintrag '{id}' auf eine freie Id wie '{free}' umbenennen."),
                ),
            ))
            .build(),
        );
    }
}

pub fn evaluate_structure(doc: &En1999Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = doc.annex;

    push_duplicate_id_fails(&mut report, annex, "materials", "Material", "Material", doc.materials.iter().map(|m| m.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "sections", "Section", "Querschnitt", doc.sections.iter().map(|s| s.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "members", "Member", "Bauteil", doc.members.iter().map(|m| m.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "connections", "Connection", "Anschluss", doc.connections.iter().map(|c| c.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "fireScenarios", "Fire scenario", "Brandszenario", doc.fire_scenarios.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "fatigueDetails", "Fatigue detail", "Ermüdungsdetail", doc.fatigue_details.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "coldFormed", "Cold-formed sheet", "Kaltprofil", doc.cold_formed.iter().map(|s| s.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "shells", "Shell", "Schale", doc.shells.iter().map(|s| s.id.clone()));

    fn push_action_field_fails(report: &mut CheckReport, annex: AnnexChoice, owner_path: &str, owner_id: &str, actions: &[semio_s_artifact_norm_en1999_model::MemberAction]) {
        const KINDS: &[&str] = &["permanent", "imposed", "snow", "wind", "temperature", "fire"];
        const CATS: &[&str] = &["self", "office", "snow", "wind", "residential", "A", "B", "congregation", "C", "retail", "D", "storage", "E", "snow_high", "temperature", "fire"];
        const SOURCES: &[&str] = &["udl", "external"];
        for a in actions {
            let base = format!("{owner_path}[id={owner_id}].actions[id={}]", a.id);
            if !KINDS.contains(&a.kind.as_str()) {
                report.push(CheckResult::assess(
                    format!("en1999.ref.action.kind.{owner_id}.{}", a.id),
                    "DIN EN 1990",
                    ClauseId::new("EN 1990", "§6", "6.10"),
                    SubjectRef::new(owner_id, format!("{base}.kind"), loc("Action kind", "Einwirkungsart")),
                    loc("Action kind", "Einwirkungsart"),
                ).status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(&format!("Unknown action kind '{}'.", a.kind), &format!("Unbekannte Einwirkungsart '{}'.", a.kind)))
                .annex(annex)
                .remedy(Remedy::one_of(SubjectRef::new(owner_id, format!("{base}.kind"), loc("Action kind", "Einwirkungsart")), KINDS.iter().map(|s| (*s).to_string()).collect(), loc("Pick a recognised EN 1990 action kind.", "Anerkannte EN-1990-Einwirkungsart wählen.")))
                .build());
            }
            if !CATS.contains(&a.category.as_str()) {
                report.push(CheckResult::assess(
                    format!("en1999.ref.action.category.{owner_id}.{}", a.id),
                    "DIN EN 1990",
                    ClauseId::new("EN 1990", "§A1.2.2", "A.1.2.2"),
                    SubjectRef::new(owner_id, format!("{base}.category"), loc("Action category", "Einwirkungskategorie")),
                    loc("Action category", "Einwirkungskategorie"),
                ).status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(&format!("Unknown action category '{}'.", a.category), &format!("Unbekannte Einwirkungskategorie '{}'.", a.category)))
                .annex(annex)
                .remedy(Remedy::one_of(SubjectRef::new(owner_id, format!("{base}.category"), loc("Action category", "Einwirkungskategorie")), CATS.iter().map(|s| (*s).to_string()).collect(), loc("Pick a recognised ψ-category.", "Anerkannte ψ-Kategorie wählen.")))
                .build());
            }
            if !SOURCES.contains(&a.source.as_str()) {
                report.push(CheckResult::assess(
                    format!("en1999.ref.action.source.{owner_id}.{}", a.id),
                    "DIN EN 1990",
                    ClauseId::new("EN 1990", "§6", "6.10"),
                    SubjectRef::new(owner_id, format!("{base}.source"), loc("Action source", "Einwirkungsquelle")),
                    loc("Action source", "Einwirkungsquelle"),
                ).status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(&format!("Unknown action source '{}'.", a.source), &format!("Unbekannte Einwirkungsquelle '{}'.", a.source)))
                .annex(annex)
                .remedy(Remedy::one_of(SubjectRef::new(owner_id, format!("{base}.source"), loc("Action source", "Einwirkungsquelle")), SOURCES.iter().map(|s| (*s).to_string()).collect(), loc("Pick udl or external.", "udl oder external wählen.")))
                .build());
            }
            let permanent = a.kind == "permanent";
            let self_cat = a.category == "self";
            if permanent && !self_cat && CATS.contains(&a.category.as_str()) {
                report.push(CheckResult::assess(
                    format!("en1999.ref.action.category.kind.{owner_id}.{}", a.id),
                    "DIN EN 1990",
                    ClauseId::new("EN 1990", "§A1.2.2", "A.1.2.2"),
                    SubjectRef::new(owner_id, format!("{base}.category"), loc("Action category", "Einwirkungskategorie")),
                    loc("Permanent action category", "Kategorie ständiger Einwirkung"),
                ).status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(
                    &format!("Permanent action '{}' must use category 'self', not '{}'.", a.id, a.category),
                    &format!("Ständige Einwirkung '{}' muss Kategorie 'self' nutzen, nicht '{}'.", a.id, a.category),
                ))
                .annex(annex)
                .remedy(Remedy::one_of(SubjectRef::new(owner_id, format!("{base}.category"), loc("Action category", "Einwirkungskategorie")), vec!["self".into()], loc("Set category to self for permanent actions.", "Kategorie self für ständige Einwirkungen setzen.")))
                .build());
            }
            if !permanent && self_cat {
                report.push(CheckResult::assess(
                    format!("en1999.ref.action.category.variable.{owner_id}.{}", a.id),
                    "DIN EN 1990",
                    ClauseId::new("EN 1990", "§A1.2.2", "A.1.2.2"),
                    SubjectRef::new(owner_id, format!("{base}.category"), loc("Action category", "Einwirkungskategorie")),
                    loc("Variable action category", "Kategorie veränderlicher Einwirkung"),
                ).status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(
                    &format!("Variable action '{}' cannot use permanent category 'self'.", a.id),
                    &format!("Veränderliche Einwirkung '{}' darf nicht Kategorie 'self' nutzen.", a.id),
                ))
                .annex(annex)
                .remedy(Remedy::one_of(SubjectRef::new(owner_id, format!("{base}.category"), loc("Action category", "Einwirkungskategorie")), vec!["office".into(), "snow".into(), "wind".into(), "storage".into()], loc("Pick a variable ψ-category.", "Veränderliche ψ-Kategorie wählen.")))
                .build());
            }
        }
    }

    for m in &doc.members { push_action_field_fails(&mut report, annex, "members", &m.id, &m.actions); }
    for c in &doc.connections { push_action_field_fails(&mut report, annex, "connections", &c.id, &c.actions); }
    for s in &doc.cold_formed { push_action_field_fails(&mut report, annex, "coldFormed", &s.id, &s.actions); }
    for s in &doc.shells { push_action_field_fails(&mut report, annex, "shells", &s.id, &s.actions); }

    if doc.members.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-1.empty",
                "DIN EN 1999-1-1",
                ClauseId::new("EN 1999-1-1", "§6", "6"),
                SubjectRef::whole(loc("Structure", "Tragwerk")),
                loc("Member ULS checks", "Bauteil-ULS-Nachweise"),
            )
            .not_applicable(loc("No members in the subject.", "Keine Bauteile im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }

    for member in &doc.members {
        let section = doc.sections.iter().find(|s| s.id == member.section_id);
        let material = doc.materials.iter().find(|m| m.id == member.material_id);
        match (section, material) {
            (Some(sec), Some(mat)) => report.extend(check_member(member, sec, mat, annex)),
            _ => report.push(
                CheckResult::assess(
                    format!("en1999.ref.{}", member.id),
                    "DIN EN 1999-1-1",
                    ClauseId::new("EN 1999-1-1", "§6", "6"),
                    SubjectRef::new(&member.id, format!("members[id={}]", member.id), member_label(&member.id)),
                    loc("Member references", "Bauteil-Referenzen"),
                )
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc("Missing section or material reference.", "Querschnitt- oder Materialverweis fehlt."))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
                    doc.sections.iter().map(|s| s.id.clone()).collect(),
                    loc("Pick an existing section id.", "Vorhandene Querschnitt-Id wählen."),
                ))
                .build(),
            ),
        }
    }

    if doc.connections.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.8.empty",
                "DIN EN 1999-1-1",
                ClauseId::new("EN 1999-1-1", "§8", "8"),
                SubjectRef::whole(loc("Connections", "Anschlüsse")),
                loc("Connection checks", "Anschlussnachweise"),
            )
            .not_applicable(loc("No connections in the subject.", "Keine Anschlüsse im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }
    for conn in &doc.connections {
        match doc.materials.iter().find(|m| m.id == conn.material_id) {
            Some(mat) => {
                let mem = doc.members.iter().find(|m| m.id == conn.member_id);
                let member_ids: Vec<String> = doc.members.iter().map(|m| m.id.clone()).collect();
                report.extend(check_connection(conn, mat, mem, &member_ids, annex));
            }
            None => report.push(
                CheckResult::assess(
                    format!("en1999.ref.conn.material.{}", conn.id),
                    "DIN EN 1999-1-1",
                    ClauseId::new("EN 1999-1-1", "§8", "8"),
                    SubjectRef::new(&conn.id, format!("connections[id={}].materialId", conn.id), loc("Connection", "Anschluss")),
                    loc("Connection material reference", "Anschluss-Materialverweis"),
                )
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(
                    &format!("Connection '{}' references unknown materialId '{}'.", conn.id, conn.material_id),
                    &format!("Anschluss '{}' verweist auf unbekannte materialId '{}'.", conn.id, conn.material_id),
                ))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&conn.id, format!("connections[id={}].materialId", conn.id), loc("Connection", "Anschluss")),
                    doc.materials.iter().map(|m| m.id.clone()).collect(),
                    loc("Point connection.materialId at an existing material.", "connection.materialId auf vorhandenes Material setzen."),
                ))
                .build(),
            ),
        }
    }

    if doc.fire_scenarios.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-2.empty",
                "DIN EN 1999-1-2",
                ClauseId::new("EN 1999-1-2", "§4", "4"),
                SubjectRef::whole(loc("Fire", "Brand")),
                loc("Fire checks", "Brandschutznachweise"),
            )
            .not_applicable(loc("No fire scenarios claimed.", "Keine Brandszenarien angegeben."))
            .annex(annex)
            .build(),
        );
    }
    for fire in &doc.fire_scenarios {
        let member = doc.members.iter().find(|m| m.id == fire.member_id);
        let section = member.and_then(|m| doc.sections.iter().find(|s| s.id == m.section_id));
        let material = member.and_then(|m| doc.materials.iter().find(|mat| mat.id == m.material_id));
        match (member, section, material) {
            (Some(mem), Some(sec), Some(mat)) => report.push(check_fire(fire, mem, sec, mat, annex)),
            _ => report.push(
                CheckResult::assess(
                    format!("en1999.1-2.fire.{}", fire.id),
                    "DIN EN 1999-1-2",
                    ClauseId::new("EN 1999-1-2", "§4", "4.2"),
                    SubjectRef::new(&fire.id, format!("fireScenarios[id={}].memberId", fire.id), loc("Fire", "Brand")),
                    loc("Fire member reference", "Brand-Bauteilverweis"),
                )
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(
                    "Fire scenario references a missing member, section or material.",
                    "Brandszenario verweist auf fehlendes Bauteil, Querschnitt oder Material.",
                ))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&fire.id, format!("fireScenarios[id={}].memberId", fire.id), loc("Fire", "Brand")),
                    doc.members.iter().map(|m| m.id.clone()).collect(),
                    loc("Point fire.memberId at an existing member.", "fire.memberId auf vorhandenes Bauteil setzen."),
                ))
                .build(),
            ),
        }
    }

    if doc.fatigue_details.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-3.empty",
                "DIN EN 1999-1-3",
                ClauseId::new("EN 1999-1-3", "§7", "7"),
                SubjectRef::whole(loc("Fatigue", "Ermüdung")),
                loc("Fatigue checks", "Ermüdungsnachweise"),
            )
            .not_applicable(loc("No fatigue details claimed.", "Keine Ermüdungsdetails angegeben."))
            .annex(annex)
            .build(),
        );
    }
    for fat in &doc.fatigue_details {
        let mem = doc.members.iter().find(|m| m.id == fat.member_id);
        report.push(check_fatigue(fat, mem, annex));
    }

    if doc.cold_formed.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-4.empty",
                "DIN EN 1999-1-4",
                ClauseId::new("EN 1999-1-4", "§5", "5.4"),
                SubjectRef::whole(loc("Cold-formed sheeting", "Kaltprofile / Bleche")),
                loc("Cold-formed sheeting checks", "Nachweise für kaltgeformte Bleche"),
            )
            .not_applicable(loc("No cold-formed sheeting in the subject.", "Keine kaltgeformten Bleche im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }
    for sheet in &doc.cold_formed {
        if let Some(mat) = doc.materials.iter().find(|m| m.id == sheet.material_id) {
            report.extend(check_cold_formed(sheet, mat, annex));
        } else {
            report.push(
                CheckResult::assess(
                    format!("en1999.1-4.ref.{}", sheet.id),
                    "DIN EN 1999-1-4",
                    ClauseId::new("EN 1999-1-4", "§5", "5.4"),
                    SubjectRef::new(&sheet.id, format!("coldFormed[id={}].materialId", sheet.id), loc("Sheet", "Blech")),
                    loc("Sheeting material reference", "Blech-Materialverweis"),
                )
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc("Missing material reference for cold-formed sheet.", "Materialverweis für kaltgeformtes Blech fehlt."))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&sheet.id, format!("coldFormed[id={}].materialId", sheet.id), loc("Sheet", "Blech")),
                    doc.materials.iter().map(|m| m.id.clone()).collect(),
                    loc("Point sheet.materialId at an existing material.", "sheet.materialId auf vorhandenes Material setzen."),
                ))
                .build(),
            );
        }
    }

    if doc.shells.is_empty() {
        report.push(
            CheckResult::assess(
                "en1999.1-5.empty",
                "DIN EN 1999-1-5",
                ClauseId::new("EN 1999-1-5", "§5", "5.3"),
                SubjectRef::whole(loc("Shells", "Schalen")),
                loc("Shell buckling checks", "Schalenbeulnachweise"),
            )
            .not_applicable(loc("No shells in the subject.", "Keine Schalen im Gegenstand."))
            .annex(annex)
            .build(),
        );
    }
    for shell in &doc.shells {
        if let Some(mat) = doc.materials.iter().find(|m| m.id == shell.material_id) {
            report.extend(check_shell(shell, mat, annex));
        } else {
            report.push(
                CheckResult::assess(
                    format!("en1999.1-5.ref.{}", shell.id),
                    "DIN EN 1999-1-5",
                    ClauseId::new("EN 1999-1-5", "§5", "5.3"),
                    SubjectRef::new(&shell.id, format!("shells[id={}].materialId", shell.id), loc("Shell", "Schale")),
                    loc("Shell material reference", "Schalen-Materialverweis"),
                )
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc("Missing material reference for shell.", "Materialverweis für Schale fehlt."))
                .annex(annex)
                .remedy(Remedy::one_of(
                    SubjectRef::new(&shell.id, format!("shells[id={}].materialId", shell.id), loc("Shell", "Schale")),
                    doc.materials.iter().map(|m| m.id.clone()).collect(),
                    loc("Point shell.materialId at an existing material.", "shell.materialId auf vorhandenes Material setzen."),
                ))
                .build(),
            );
        }
    }

    report
}
//#endregion 🔖️ComplianceHelpers
