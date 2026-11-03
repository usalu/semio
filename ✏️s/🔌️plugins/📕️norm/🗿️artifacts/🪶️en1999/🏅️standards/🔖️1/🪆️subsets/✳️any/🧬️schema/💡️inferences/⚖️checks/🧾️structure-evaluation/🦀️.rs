//! 🧾️ EN 1999 structure evaluation: the duplicate-id check and `evaluate_structure`, which runs every clause check over a whole snapshot.

use super::*;
use super::{outline_checks, weld_run_checks};

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

fn detail_fail(report: &mut CheckReport, annex: AnnexChoice, id: String, owner: &str, path: String, title: (&str, &str), en: String, de: String, options: Vec<String>) {
    let subject = SubjectRef::new(owner, path, loc(title.0, title.1));
    report.push(
        CheckResult::assess(id, "DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§6.1.6", "6.1.6"), subject.clone(), loc(title.0, title.1))
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .explanation(loc(&en, &de))
            .annex(annex)
            .remedy(Remedy::one_of(subject, options, loc("Pick a supported value.", "Unterstützten Wert wählen.")))
            .build(),
    );
}

/// 🧪 Refuses incomplete or misplaced edge stiffeners.
fn push_detail_fails(report: &mut CheckReport, doc: &En1999Snapshot) {
    let annex = doc.annex;
    for sec in &doc.sections {
        for el in &sec.elements {
            let base = format!("sections[id={}].elements[id={}]", sec.id, el.id);
            let key = format!("{}.{}", sec.id, el.id);
            let any = el.edge_stiffener_depth > 0.0 || el.edge_stiffener_thickness > 0.0;
            if any && (!el.outstand || el.edge_stiffener_depth <= 0.0 || el.edge_stiffener_thickness <= 0.0) {
                let subject = SubjectRef::new(&sec.id, format!("{base}.edgeStiffenerDepth"), loc("Edge stiffener", "Randsteife"));
                report.push(
                    CheckResult::assess(format!("en1999.ref.element.stiffener.{key}"), "DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§6.1.4", "6.1.4.3"), subject.clone(), loc("Edge stiffener", "Randsteife"))
                        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                        .explanation(loc(
                            &format!("Edge stiffener of element {} needs a positive depth and thickness on an outstand.", el.id),
                            &format!("Die Randsteife des Elements {} braucht positive Tiefe und Dicke an einem auskragenden Element.", el.id),
                        ))
                        .annex(annex)
                        .remedy(Remedy::exactly(subject, q_length(el.edge_stiffener_depth), q_length(0.0), loc("Remove the stiffener.", "Randsteife entfernen.")))
                        .build(),
                );
            }
        }
    }
    let placeable: Vec<String> = doc.members.iter().map(|m| m.id.clone()).chain(doc.connections.iter().map(|c| c.id.clone())).chain(doc.cold_formed.iter().map(|c| c.id.clone())).chain(doc.shells.iter().map(|c| c.id.clone())).collect();
    for placement in &doc.placements {
        let finite = [placement.x, placement.y, placement.z, placement.yaw].iter().all(|v| v.is_finite());
        if !placeable.contains(&placement.id) || !finite {
            let subject = SubjectRef::new(&placement.id, format!("placements[id={}].id", placement.id), loc("Placement", "Platzierung"));
            let mut check = CheckResult::assess(format!("en1999.ref.placement.{}", placement.id), "DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§1", "1"), subject.clone(), loc("Placement reference", "Platzierungsverweis"))
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(
                    &format!("Placement '{}' must name an existing member, connection, sheet or shell and carry finite coordinates.", placement.id),
                    &format!("Platzierung '{}' muss ein vorhandenes Bauteil, einen Anschluss, ein Blech oder eine Schale benennen und endliche Koordinaten tragen.", placement.id),
                ))
                .annex(annex);
            if !placeable.is_empty() { check = check.remedy(Remedy::one_of(subject, placeable.clone(), loc("Point the placement at an existing entity.", "Platzierung auf ein vorhandenes Element richten."))); }
            report.push(check.build());
        }
    }
    for vertex in &doc.outline_vertices {
        if !doc.sections.iter().any(|s| s.id == vertex.section_id) {
            detail_fail(report, annex, format!("en1999.ref.vertex.{}", vertex.id), &vertex.id, format!("outlineVertices[id={}].sectionId", vertex.id), ("Outline vertex section", "Querschnitt des Umrissknotens"),
                format!("Outline vertex '{}' names the unknown section '{}'.", vertex.id, vertex.section_id), format!("Umrissknoten '{}' nennt den unbekannten Querschnitt '{}'.", vertex.id, vertex.section_id), doc.sections.iter().map(|s| s.id.clone()).collect());
        }
    }
    for mat in &doc.materials {
        let tube_member = doc.members.iter().find(|m| m.material_id == mat.id && doc.sections.iter().any(|s| s.id == m.section_id && part_1_1::is_tube(s)));
        if let (Some(member), false) = (tube_member, part_1_1::form_supplies_tube(mat.product_form)) {
            let subject = SubjectRef::new(&mat.id, format!("materials[id={}].productForm", mat.id), loc("Product form", "Erzeugnisform"));
            report.push(
                CheckResult::assess(format!("en1999.3.2.form.{}", mat.id), "DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§3.2", "3.2"), subject.clone(), loc("Product form against section", "Erzeugnisform gegen Querschnitt"))
                    .status(semio_s_plugin_norm_compliance::CheckStatus::Warning)
                    .explanation(loc(
                        &format!("Member {} is a circular hollow section but material {} is '{:?}'; Table 3.2 only lists hollow sections as extruded profile, extruded tube or drawn tube.", member.id, mat.id, mat.product_form),
                        &format!("Bauteil {} ist ein Kreishohlprofil, Werkstoff {} aber '{:?}'; Tabelle 3.2 führt Hohlquerschnitte nur als Strangpressprofil, Strangpressrohr oder gezogenes Rohr.", member.id, mat.id, mat.product_form),
                    ))
                    .annex(annex)
                    .remedy(Remedy::one_of(subject, vec!["extrudedProfile".into(), "extrudedTube".into(), "drawnTube".into()], loc("Pick a tube product form.", "Rohr-Erzeugnisform wählen.")))
                    .build(),
            );
        }
    }
    for shell in &doc.shells {
    }
}

/// 🎯 The checks of one document entity (member, connection, sheet, shell, fire scenario, fatigue detail) and its governing one.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckUnit {
    pub entity_kind: &'static str,
    pub entity_id: String,
    pub check_ids: Vec<String>,
    pub governing_id: String,
    pub utilization: f64,
}

/// 🎯 Groups a report by the entity each check is about, so a viewer can colour an entity by its governing check.
pub fn check_units(doc: &En1999Snapshot, report: &CheckReport) -> Vec<CheckUnit> {
    let kinds: Vec<(&'static str, Vec<&str>)> = vec![
        ("member", doc.members.iter().map(|m| m.id.as_str()).collect()),
        ("connection", doc.connections.iter().map(|c| c.id.as_str()).collect()),
        ("coldFormedSheet", doc.cold_formed.iter().map(|c| c.id.as_str()).collect()),
        ("shell", doc.shells.iter().map(|c| c.id.as_str()).collect()),
        ("fireScenario", doc.fire_scenarios.iter().map(|c| c.id.as_str()).collect()),
        ("fatigueDetail", doc.fatigue_details.iter().map(|c| c.id.as_str()).collect()),
        ("fatigueCategory", doc.fatigue_categories.iter().map(|c| c.id.as_str()).collect()),
        ("patchLoad", doc.patch_loads.iter().map(|c| c.id.as_str()).collect()),
        ("ringStiffener", doc.ring_stiffeners.iter().map(|c| c.id.as_str()).collect()),
        ("fireInsulation", doc.fire_insulations.iter().map(|c| c.id.as_str()).collect()),
        ("sheetProfile", doc.sheet_profiles.iter().map(|c| c.id.as_str()).collect()),
        ("transverseStiffener", doc.transverse_stiffeners.iter().map(|c| c.id.as_str()).collect()),
    ];
    let mut out = Vec::new();
    for (entity_kind, ids) in kinds {
        for id in ids {
            let mine: Vec<&CheckResult> = report.checks.iter().filter(|c| c.subject.entity_id == id).collect();
            let Some(worst) = mine.iter().max_by(|a, b| a.utilization.partial_cmp(&b.utilization).unwrap_or(std::cmp::Ordering::Equal)) else { continue };
            out.push(CheckUnit { entity_kind, entity_id: id.to_owned(), check_ids: mine.iter().map(|c| c.id.clone()).collect(), governing_id: worst.id.clone(), utilization: worst.utilization });
        }
    }
    out
}

pub fn evaluate_structure(doc: &En1999Snapshot) -> CheckReport {
    evaluate_scoped(doc, Scope::All)
}

/// 🎯️ The evaluation restricted to `scope`: the named parts `integrity` and `<collection>-empty` (verdicts) or one row of `members`, `connections`, `fireScenarios`, `fatigueDetails`, `coldFormed`, `shells`, `weldRuns`, `sectionOutlines` or the action rows `memberActions`, `connectionActions`, `coldFormedActions`, `shellActions`; the ordered concatenation of the units in [`En1999Units`] is [`evaluate_structure`].
pub fn evaluate_scoped(doc: &En1999Snapshot, scope: Scope<'_>) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = doc.annex;

    if scope.part("integrity") {
    push_duplicate_id_fails(&mut report, annex, "materials", "Material", "Material", doc.materials.iter().map(|m| m.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "sections", "Section", "Querschnitt", doc.sections.iter().map(|s| s.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "members", "Member", "Bauteil", doc.members.iter().map(|m| m.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "connections", "Connection", "Anschluss", doc.connections.iter().map(|c| c.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "fireScenarios", "Fire scenario", "Brandszenario", doc.fire_scenarios.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "fatigueDetails", "Fatigue detail", "Ermüdungsdetail", doc.fatigue_details.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "fatigueCategories", "Fatigue category", "Ermüdungskategorie", doc.fatigue_categories.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "patchLoads", "Patch load", "Stegeinzellast", doc.patch_loads.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "ringStiffeners", "Ring stiffener", "Ringsteife", doc.ring_stiffeners.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "fireInsulations", "Fire insulation", "Brandschutzbekleidung", doc.fire_insulations.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "sheetProfiles", "Sheet profile", "Blechprofil", doc.sheet_profiles.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "transverseStiffeners", "Transverse stiffener", "Quersteife", doc.transverse_stiffeners.iter().map(|f| f.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "coldFormed", "Cold-formed sheet", "Kaltprofil", doc.cold_formed.iter().map(|s| s.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "shells", "Shell", "Schale", doc.shells.iter().map(|s| s.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "placements", "Placement", "Platzierung", doc.placements.iter().map(|p| p.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "weldRuns", "Weld run", "Schweißnaht", doc.weld_runs.iter().map(|p| p.id.clone()));
    push_duplicate_id_fails(&mut report, annex, "outlineVertices", "Outline vertex", "Umrissknoten", doc.outline_vertices.iter().map(|p| p.id.clone()));
    push_detail_fails(&mut report, doc);
    }

    for (i, m) in doc.members.iter().enumerate().filter(|(i, _)| scope.admits("memberActions", *i)) { push_action_field_fails(&mut report, annex, "members", &m.id, &m.actions); }
    for (i, c) in doc.connections.iter().enumerate().filter(|(i, _)| scope.admits("connectionActions", *i)) { push_action_field_fails(&mut report, annex, "connections", &c.id, &c.actions); }
    for (i, s) in doc.cold_formed.iter().enumerate().filter(|(i, _)| scope.admits("coldFormedActions", *i)) { push_action_field_fails(&mut report, annex, "coldFormed", &s.id, &s.actions); }
    for (i, s) in doc.shells.iter().enumerate().filter(|(i, _)| scope.admits("shellActions", *i)) { push_action_field_fails(&mut report, annex, "shells", &s.id, &s.actions); }

    if doc.members.is_empty() && scope.part("members-empty") {
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

    for (_, member) in doc.members.iter().enumerate().filter(|(i, _)| scope.admits("members", *i)) {
        let section = doc.sections.iter().find(|s| s.id == member.section_id);
        let material = doc.materials.iter().find(|m| m.id == member.material_id);
        match (section, material) {
            (Some(sec), Some(mat)) if outline_checks::is_free(sec) => report.extend(check_free_member(member, sec, &outline_checks::rings(&doc.outline_vertices, &sec.id), mat, annex, doc.reliability_class)),
            (Some(sec), Some(mat)) => {
                let runs = weld_run_checks::runs_of(&doc.weld_runs, &member.id);
                let (resolved_member, resolved_section) = weld_run_checks::resolved(member, sec, &runs);
                report.extend(check_member(&resolved_member, &resolved_section, mat, annex, doc.reliability_class, &weld_run_checks::haz_overrides(&runs), &web_stiffener::rows_of(&doc.transverse_stiffeners, &member.id)));
                report.extend(check_edge_stiffeners(member, sec, mat, annex));
            }
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

    if doc.connections.is_empty() && scope.part("connections-empty") {
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
    for (_, conn) in doc.connections.iter().enumerate().filter(|(i, _)| scope.admits("connections", *i)) {
        match doc.materials.iter().find(|m| m.id == conn.material_id) {
            Some(mat) => {
                let mem = doc.members.iter().find(|m| m.id == conn.member_id);
                let member_ids: Vec<String> = doc.members.iter().map(|m| m.id.clone()).collect();
                report.extend(check_connection(conn, mat, mem, &member_ids, annex, doc.reliability_class));
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

    if doc.fire_scenarios.is_empty() && scope.part("fire-empty") {
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
    for (_, fire) in doc.fire_scenarios.iter().enumerate().filter(|(i, _)| scope.admits("fireScenarios", *i)) {
        let member = doc.members.iter().find(|m| m.id == fire.member_id);
        let section = member.and_then(|m| doc.sections.iter().find(|s| s.id == m.section_id));
        let material = member.and_then(|m| doc.materials.iter().find(|mat| mat.id == m.material_id));
        match (member, section, material) {
            (Some(_), Some(sec), Some(_)) if outline_checks::is_free(sec) => report.push(
                CheckResult::assess(format!("en1999.1-2.fire.{}", fire.id), "DIN EN 1999-1-2", ClauseId::new("EN 1999-1-2", "§4", "4.2"), SubjectRef::new(&fire.id, format!("fireScenarios[id={}].memberId", fire.id), loc("Fire", "Brand")), loc("Fire on a free outline", "Brand am freien Umriss"))
                    .not_applicable(loc("The fire resistance of a free outline is not evaluated.", "Der Feuerwiderstand eines freien Umrisses wird nicht ausgewertet."))
                    .annex(annex)
                    .build(),
            ),
            (Some(mem), Some(sec), Some(mat)) => report.extend(check_fire(fire, mem, sec, mat, doc.fire_insulations.iter().find(|i| i.fire_id == fire.id), annex)),
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

    if doc.fatigue_details.is_empty() && scope.part("fatigue-empty") {
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
    let spectra = part_1_3::spectrum_groups(&doc.fatigue_details);
    for (i, fat) in doc.fatigue_details.iter().enumerate().filter(|(i, _)| scope.admits("fatigueDetails", *i)) {
        let mem = doc.members.iter().find(|m| m.id == fat.member_id);
        report.extend(check_fatigue(fat, mem, annex));
        if let Some(group) = spectra.iter().filter(|g| !part_1_3::group_has_category(g, &doc.fatigue_details, &doc.fatigue_categories)).find(|g| g[0] == i) {
            report.extend(check_fatigue_spectrum(&group.iter().map(|j| &doc.fatigue_details[*j]).collect::<Vec<_>>(), annex));
        }
    }

    for (_, category) in doc.fatigue_categories.iter().enumerate().filter(|(i, _)| scope.admits("fatigueCategories", *i)) {
        report.extend(check_fatigue_category(category, doc, annex));
    }
    for (_, load) in doc.patch_loads.iter().enumerate().filter(|(i, _)| scope.admits("patchLoads", *i)) {
        report.extend(check_patch_load(load, doc, annex));
    }
    for (_, insulation) in doc.fire_insulations.iter().enumerate().filter(|(i, _)| scope.admits("fireInsulations", *i)) {
        report.extend(check_fire_insulation(insulation, doc, annex));
    }
    for (_, profile) in doc.sheet_profiles.iter().enumerate().filter(|(i, _)| scope.admits("sheetProfiles", *i)) {
        report.extend(check_sheet_profile(profile, doc, annex));
    }
    for (_, ring) in doc.ring_stiffeners.iter().enumerate().filter(|(i, _)| scope.admits("ringStiffeners", *i)) {
        report.extend(check_ring_stiffener(ring, doc, annex, doc.reliability_class));
    }
    for (_, stiffener) in doc.transverse_stiffeners.iter().enumerate().filter(|(i, _)| scope.admits("transverseStiffeners", *i)) {
        report.extend(check_transverse_stiffener(stiffener, doc, annex, doc.reliability_class));
    }

    if doc.cold_formed.is_empty() && scope.part("coldFormed-empty") {
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
    for (_, sheet) in doc.cold_formed.iter().enumerate().filter(|(i, _)| scope.admits("coldFormed", *i)) {
        if let Some(mat) = doc.materials.iter().find(|m| m.id == sheet.material_id) {
            report.extend(check_cold_formed(sheet, mat, doc.sheet_profiles.iter().find(|p| p.sheet_id == sheet.id), annex, doc.reliability_class));
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

    if doc.shells.is_empty() && scope.part("shells-empty") {
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
    for (_, shell) in doc.shells.iter().enumerate().filter(|(i, _)| scope.admits("shells", *i)) {
        if let Some(mat) = doc.materials.iter().find(|m| m.id == shell.material_id) {
            report.extend(check_shell(shell, mat, &doc.ring_stiffeners.iter().filter(|r| r.shell_id == shell.id).map(|r| r.x).collect::<Vec<_>>(), annex, doc.reliability_class));
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

    for (_, run) in doc.weld_runs.iter().enumerate().filter(|(i, _)| scope.admits("weldRuns", *i)) {
        report.extend(check_run(doc, run));
    }
    for (_, section) in doc.sections.iter().enumerate().filter(|(i, _)| scope.admits("sectionOutlines", *i)) {
        report.extend(check_outline(section, &doc.outline_vertices, annex));
    }

    report
}

fn push_action_field_fails(report: &mut CheckReport, annex: AnnexChoice, owner_path: &str, owner_id: &str, actions: &[semio_s_artifact_norm_en1999_model::MemberAction]) {
    for a in actions {
        let base = format!("{owner_path}[id={owner_id}].actions[id={}]", a.id);
        let permanent = a.kind == semio_s_artifact_norm_en1999_model::ActionKind::Permanent;
        let self_cat = a.category == semio_s_artifact_norm_en1999_model::ActionCategory::SelfWeight;
        if permanent && !self_cat {
            report.push(CheckResult::assess(
                format!("en1999.ref.action.category.kind.{owner_id}.{}", a.id),
                "DIN EN 1990",
                ClauseId::new("EN 1990", "§A1.2.2", "A.1.2.2"),
                SubjectRef::new(owner_id, format!("{base}.category"), loc("Action category", "Einwirkungskategorie")),
                loc("Permanent action category", "Kategorie ständiger Einwirkung"),
            ).status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .explanation(loc(
                &format!("Permanent action '{}' must use category 'self', not '{:?}'.", a.id, a.category),
                &format!("Ständige Einwirkung '{}' muss Kategorie 'self' nutzen, nicht '{:?}'.", a.id, a.category),
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
//#endregion 🔖️ComplianceHelpers
