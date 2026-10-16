//! 🧰️ EN 1993 structure helpers: section and material lookup, member kinds, load combination of scalar forces and member actions, and the referential-integrity checks.

use super::*;

//#region 🔖️ComplianceHelpers
pub(super) fn next_section_options(current: &SteelSection, need_area: f64, need_wpl: f64) -> Vec<String> {
    let mut opts: Vec<String> = rolled_heb_catalogue()
        .into_iter()
        .filter(|s| s.area + 1e-12 >= need_area && s.w_pl_y + 1e-12 >= need_wpl && s.id != current.id)
        .map(|s| s.id)
        .collect();
    if opts.is_empty() {
        opts = rolled_heb_catalogue()
            .into_iter()
            .filter(|s| (s.area > current.area + 1e-12 || s.w_pl_y > current.w_pl_y + 1e-12) && s.id != current.id)
            .map(|s| s.id)
            .collect();
    }
    if opts.is_empty() {
        if let Some(best) = rolled_heb_catalogue().into_iter().max_by(|a, b| a.area.partial_cmp(&b.area).unwrap_or(std::cmp::Ordering::Equal)) {
            if best.id != current.id {
                opts.push(best.id);
            }
        }
    }
    opts
}

pub(super) fn find_material<'a>(doc: &'a En1993Snapshot, id: &str) -> Option<&'a SteelMaterial> {
    doc.materials.iter().find(|m| m.id == id)
}

pub(super) fn find_section<'a>(doc: &'a En1993Snapshot, id: &str) -> Option<&'a SteelSection> {
    doc.sections.iter().find(|s| s.id == id)
}

/// 📋️ Full hierarchical EN 1993 evaluation with remedies.

/// ⚖️ EN 1990 ψ₀/ψ₁/ψ₂ by action category (Table A1.1) — DE NA diverges for snow.
pub(super) fn psi_factors(category: &str, annex: AnnexChoice) -> (f64, f64, f64) {
    match (category, annex) {
        ("office" | "residential" | "congregation", _) => (0.7, 0.5, 0.3),
        ("shopping" | "storage", _) => (0.7, 0.7, 0.6),
        ("snow" | "snow_high", AnnexChoice::De) => (0.7, 0.2, 0.0),
        ("snow" | "snow_high", AnnexChoice::En) => (0.5, 0.2, 0.0),
        ("wind", _) => (0.6, 0.2, 0.0),
        ("temperature", _) => (0.6, 0.5, 0.0),
        ("self" | "permanent", _) => (1.0, 1.0, 1.0),
        ("accidental", _) => (1.0, 1.0, 1.0),
        _ => (0.7, 0.5, 0.3),
    }
}

/// 🧮 EN 1990 γ_G / γ_Q / ξ (Eq. 6.10a/b) — STR values identical for EN recommended and DE NA.
fn gamma_factors() -> (f64, f64, f64) {
    (GAMMA_G_STR, GAMMA_Q_STR, XI_STR)
}

/// 🧮 Combined scalar design effect from characteristic forces linked to load cases.
#[derive(Clone, Debug)]
struct CombinedScalar {
    pub(super) combination_id: String,
    situation: String,
    pub(super) value: f64,
}

/// 🧩 Normalize memberType (beam / column / beamColumn / brace / tie) for check gating.
pub(super) fn member_kind(raw: &str) -> String {
    raw.to_ascii_lowercase().replace(['_', '-'], "")
}

pub(super) fn kind_allows_flexural_buckling(kind: &str) -> bool {
    !matches!(kind, "beam" | "tie")
}

pub(super) fn kind_allows_ltb(kind: &str) -> bool {
    !matches!(kind, "column" | "brace" | "tie")
}

pub(super) fn kind_allows_bending(kind: &str) -> bool {
    !matches!(kind, "brace" | "tie")
}

pub(super) fn kind_allows_compression(kind: &str) -> bool {
    kind != "tie"
}

pub(super) fn kind_allows_member_interaction(kind: &str) -> bool {
    matches!(kind, "beamcolumn" | "column") || kind.is_empty()
}

pub(super) fn combine_scalar_forces(document: &En1993Snapshot, forces: &[(String, f64)]) -> Vec<CombinedScalar> {
    let mut g = 0.0;
    let mut variables: Vec<(&LoadCase, f64)> = Vec::new();
    for (lc_id, force) in forces {
        let Some(lc) = document.load_cases.iter().find(|l| l.id == *lc_id) else { continue };
        match lc.kind.as_str() {
            "permanent" | "prestress" => g += *force,
            _ => variables.push((lc, *force)),
        }
    }
    let (gamma_g, gamma_q, xi) = gamma_factors();
    let mut out = Vec::new();
    if variables.is_empty() {
        if g.abs() > 0.0 {
            out.push(CombinedScalar { combination_id: "uls-g".into(), situation: "uls".into(), value: gamma_g * g });
        }
        out.push(CombinedScalar { combination_id: "sls-char".into(), situation: "sls".into(), value: g });
        return out;
    }
    for (i, (lead, lead_v)) in variables.iter().enumerate() {
        let mut a = gamma_g * g + gamma_q * lead_v;
        let mut b = xi * gamma_g * g + gamma_q * lead_v;
        for (j, (other, ov)) in variables.iter().enumerate() {
            if i == j { continue; }
            let (psi0, _, _) = psi_factors(&other.category, document.annex);
            a += gamma_q * psi0 * ov;
            b += gamma_q * psi0 * ov;
        }
        out.push(CombinedScalar { combination_id: format!("uls-6.10a-{}", lead.id), situation: "uls".into(), value: a });
        out.push(CombinedScalar { combination_id: format!("uls-6.10b-{}", lead.id), situation: "uls".into(), value: b });
    }
    let (lead, lead_v) = &variables[0];
    let mut s = g + lead_v;
    for (j, (other, ov)) in variables.iter().enumerate() {
        if j == 0 { continue; }
        let (psi0, _, _) = psi_factors(&other.category, document.annex);
        s += psi0 * ov;
    }
    out.push(CombinedScalar { combination_id: format!("sls-char-{}", lead.id), situation: "sls".into(), value: s });
    out
}

pub(super) fn governing_scalar<'a>(effects: &'a [CombinedScalar], sit: &str) -> Option<&'a CombinedScalar> {
    effects.iter().filter(|e| e.situation == sit).max_by(|a, b| a.value.abs().partial_cmp(&b.value.abs()).unwrap_or(std::cmp::Ordering::Equal))
}

/// 🧮 Combined design effects for one member (EN 1990 §6.4.3 + DE NA ψ).
#[derive(Clone, Debug)]
pub(super) struct CombinedEffects {
    pub(super) combination_id: String,
    situation: String,
    pub(super) action: semio_s_artifact_norm_en1993_model::DesignAction,
}

pub(super) fn combine_member_actions(document: &En1993Snapshot, member_id: &str) -> Vec<CombinedEffects> {
    let mut g = semio_s_artifact_norm_en1993_model::DesignAction { n: 0.0, vy: 0.0, vz: 0.0, my: 0.0, mz: 0.0, t: 0.0 };
    let mut variables: Vec<(&LoadCase, semio_s_artifact_norm_en1993_model::DesignAction)> = Vec::new();
    for row in document.member_actions.iter().filter(|a| a.member_id == member_id) {
        let Some(lc) = document.load_cases.iter().find(|l| l.id == row.load_case_id) else { continue };
        match lc.kind.as_str() {
            "permanent" | "prestress" => {
                g.n += row.action.n; g.vy += row.action.vy; g.vz += row.action.vz;
                g.my += row.action.my; g.mz += row.action.mz; g.t += row.action.t;
            }
            _ => variables.push((lc, row.action.clone())),
        }
    }
    let (gamma_g, gamma_q, xi) = gamma_factors();
    let mut out = Vec::new();
    if variables.is_empty() {
        if g.n.abs() + g.my.abs() + g.vz.abs() > 0.0 {
            out.push(CombinedEffects {
                combination_id: "uls-g".into(),
                situation: "uls".into(),
                action: semio_s_artifact_norm_en1993_model::DesignAction {
                    n: gamma_g * g.n, vy: gamma_g * g.vy, vz: gamma_g * g.vz,
                    my: gamma_g * g.my, mz: gamma_g * g.mz, t: gamma_g * g.t,
                },
            });
        }
    } else {
        for (i, (lead, lead_a)) in variables.iter().enumerate() {
            let mut a = semio_s_artifact_norm_en1993_model::DesignAction {
                n: gamma_g * g.n + gamma_q * lead_a.n,
                vy: gamma_g * g.vy + gamma_q * lead_a.vy,
                vz: gamma_g * g.vz + gamma_q * lead_a.vz,
                my: gamma_g * g.my + gamma_q * lead_a.my,
                mz: gamma_g * g.mz + gamma_q * lead_a.mz,
                t: gamma_g * g.t + gamma_q * lead_a.t,
            };
            let mut b = semio_s_artifact_norm_en1993_model::DesignAction {
                n: xi * gamma_g * g.n + gamma_q * lead_a.n,
                vy: xi * gamma_g * g.vy + gamma_q * lead_a.vy,
                vz: xi * gamma_g * g.vz + gamma_q * lead_a.vz,
                my: xi * gamma_g * g.my + gamma_q * lead_a.my,
                mz: xi * gamma_g * g.mz + gamma_q * lead_a.mz,
                t: xi * gamma_g * g.t + gamma_q * lead_a.t,
            };
            for (j, (other, oa)) in variables.iter().enumerate() {
                if i == j { continue; }
                let (psi0, _, _) = psi_factors(&other.category, document.annex);
                a.n += gamma_q * psi0 * oa.n; a.vy += gamma_q * psi0 * oa.vy; a.vz += gamma_q * psi0 * oa.vz;
                a.my += gamma_q * psi0 * oa.my; a.mz += gamma_q * psi0 * oa.mz; a.t += gamma_q * psi0 * oa.t;
                b.n += gamma_q * psi0 * oa.n; b.vy += gamma_q * psi0 * oa.vy; b.vz += gamma_q * psi0 * oa.vz;
                b.my += gamma_q * psi0 * oa.my; b.mz += gamma_q * psi0 * oa.mz; b.t += gamma_q * psi0 * oa.t;
            }
            out.push(CombinedEffects { combination_id: format!("uls-6.10a-{}", lead.id), situation: "uls".into(), action: a });
            out.push(CombinedEffects { combination_id: format!("uls-6.10b-{}", lead.id), situation: "uls".into(), action: b });
        }
    }
    if variables.is_empty() {
        out.push(CombinedEffects { combination_id: "sls-char".into(), situation: "sls".into(), action: g });
    } else {
        let (lead, lead_a) = &variables[0];
        let mut s = semio_s_artifact_norm_en1993_model::DesignAction {
            n: g.n + lead_a.n, vy: g.vy + lead_a.vy, vz: g.vz + lead_a.vz,
            my: g.my + lead_a.my, mz: g.mz + lead_a.mz, t: g.t + lead_a.t,
        };
        for (j, (other, oa)) in variables.iter().enumerate() {
            if j == 0 { continue; }
            let (psi0, _, _) = psi_factors(&other.category, document.annex);
            s.n += psi0 * oa.n; s.vy += psi0 * oa.vy; s.vz += psi0 * oa.vz;
            s.my += psi0 * oa.my; s.mz += psi0 * oa.mz; s.t += psi0 * oa.t;
        }
        out.push(CombinedEffects { combination_id: format!("sls-char-{}", lead.id), situation: "sls".into(), action: s });
    }
    out
}

pub(super) fn governing_effects<'a>(effects: &'a [CombinedEffects], sit: &str) -> Option<&'a CombinedEffects> {
    effects.iter().filter(|e| e.situation == sit).max_by(|a, b| {
        let ka = a.action.n.abs() + a.action.my.abs() + a.action.vz.abs();
        let kb = b.action.n.abs() + b.action.my.abs() + b.action.vz.abs();
        ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
    })
}

fn push_duplicate_ids(report: &mut CheckReport, annex: AnnexChoice, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, loc(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let mut builder = CheckResult::assess(
            format!("en1993.integrity.duplicate.{table}.{id}"),
            "EN 1993 integrity",
            ClauseId::new("EN 1993", "§1", "id"),
            subject.clone(),
            loc(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
        )
        .annex(annex)
        .explanation(loc(
            &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(CheckStatus::Fail);
        builder = builder.remedy(Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            loc(
                &format!("Rename the duplicated '{id}' entry to a free id."),
                &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

fn push_dangling_ref(report: &mut CheckReport, annex: AnnexChoice, check_id: String, path: String, subject_id: &str, label_en: &str, label_de: &str, current: &str, options: Vec<String>) {
    let subject = SubjectRef::new(subject_id, &path, loc(label_en, label_de));
    let opts = if options.is_empty() { vec![format!("{current}-missing")] } else { options };
    report.push(
        CheckResult::assess(
            check_id,
            "EN 1993 integrity",
            ClauseId::new("EN 1993", "§1", "reference"),
            subject.clone(),
            loc(&format!("Referential integrity — {label_en}"), &format!("Referenzintegrität — {label_de}")),
        )
        .annex(annex)
        .explanation(loc(
            &format!("'{current}' does not reference an existing target; dependent checks for this row must not Pass."),
            &format!("'{current}' verweist auf kein vorhandenes Ziel; abhängige Nachweise für diese Zeile dürfen nicht bestehen."),
        ))
        .status(CheckStatus::Fail)
        .remedy(Remedy::one_of(
            subject,
            opts,
            loc(
                &format!("Set {label_en} to one of the existing target ids."),
                &format!("{label_de} auf eine der vorhandenen Ziel-Ids setzen."),
            ),
        ))
        .build(),
    );
}

pub(super) fn push_referential_integrity(report: &mut CheckReport, document: &En1993Snapshot) {
    let annex = document.annex;
    let material_ids: Vec<String> = document.materials.iter().map(|m| m.id.clone()).collect();
    let section_ids: Vec<String> = document.sections.iter().map(|s| s.id.clone()).collect();
    let member_ids: Vec<String> = document.members.iter().map(|m| m.id.clone()).collect();
    let load_case_ids: Vec<String> = document.load_cases.iter().map(|l| l.id.clone()).collect();
    let action_ids: Vec<String> = document.member_actions.iter().map(|a| a.id.clone()).collect();

    push_duplicate_ids(report, annex, "materials", &material_ids, &|id| format!("materials[id={id}].id"));
    push_duplicate_ids(report, annex, "sections", &section_ids, &|id| format!("sections[id={id}].id"));
    push_duplicate_ids(report, annex, "members", &member_ids, &|id| format!("members[id={id}].id"));
    push_duplicate_ids(report, annex, "loadCases", &load_case_ids, &|id| format!("loadCases[id={id}].id"));
    push_duplicate_ids(report, annex, "memberActions", &action_ids, &|id| format!("memberActions[id={id}].id"));
    push_duplicate_ids(report, annex, "joints", &document.joints.iter().map(|j| j.id.clone()).collect::<Vec<_>>(), &|id| format!("joints[id={id}].id"));
    push_duplicate_ids(report, annex, "fatigueDetails", &document.fatigue_details.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("fatigueDetails[id={id}].id"));
    push_duplicate_ids(report, annex, "fireExposures", &document.fire_exposures.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("fireExposures[id={id}].id"));
    push_duplicate_ids(report, annex, "coldFormedMembers", &document.cold_formed_members.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), &|id| format!("coldFormedMembers[id={id}].id"));
    push_duplicate_ids(report, annex, "platedPanels", &document.plated_panels.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("platedPanels[id={id}].id"));
    push_duplicate_ids(report, annex, "siloShells", &document.silo_shells.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("siloShells[id={id}].id"));
    push_duplicate_ids(report, annex, "tensionComponents", &document.tension_components.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("tensionComponents[id={id}].id"));
    push_duplicate_ids(report, annex, "bridgeFatigue", &document.bridge_fatigue.iter().map(|b| b.id.clone()).collect::<Vec<_>>(), &|id| format!("bridgeFatigue[id={id}].id"));
    push_duplicate_ids(report, annex, "towerLegs", &document.tower_legs.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("towerLegs[id={id}].id"));
    push_duplicate_ids(report, annex, "piles", &document.piles.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("piles[id={id}].id"));
    push_duplicate_ids(report, annex, "craneRunways", &document.crane_runways.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), &|id| format!("craneRunways[id={id}].id"));

    let designations: Vec<String> = rolled_heb_catalogue().into_iter().map(|s| s.designation).collect();
    for section in &document.sections {
        if !designations.iter().any(|d| d == &section.designation) {
            push_dangling_ref(
                report,
                annex,
                format!("en1993.integrity.sections.{}.designation", section.id),
                format!("sections[id={}].designation", section.id),
                &section.id,
                "Section designation",
                "Querschnittsbezeichnung",
                &section.designation,
                designations.clone(),
            );
        }
    }

    for member in &document.members {
        if !section_ids.iter().any(|id| id == &member.section_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.members.{}.sectionId", member.id), format!("members[id={}].sectionId", member.id), &member.id, "Member sectionId", "Bauteil sectionId", &member.section_id, section_ids.clone());
        }
        if !material_ids.iter().any(|id| id == &member.material_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.members.{}.materialId", member.id), format!("members[id={}].materialId", member.id), &member.id, "Member materialId", "Bauteil materialId", &member.material_id, material_ids.clone());
        }
    }
    for action in &document.member_actions {
        if !member_ids.iter().any(|id| id == &action.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.memberActions.{}.memberId", action.id), format!("memberActions[id={}].memberId", action.id), &action.id, "Action memberId", "Einwirkung memberId", &action.member_id, member_ids.clone());
        }
        if !load_case_ids.iter().any(|id| id == &action.load_case_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.memberActions.{}.loadCaseId", action.id), format!("memberActions[id={}].loadCaseId", action.id), &action.id, "Action loadCaseId", "Einwirkung loadCaseId", &action.load_case_id, load_case_ids.clone());
        }
    }
    for joint in &document.joints {
        if !member_ids.iter().any(|id| id == &joint.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.joints.{}.memberId", joint.id), format!("joints[id={}].memberId", joint.id), &joint.id, "Joint memberId", "Anschluss memberId", &joint.member_id, member_ids.clone());
        }
        for ja in &joint.actions {
            if !load_case_ids.iter().any(|id| id == &ja.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.joints.{}.actions.{}.loadCaseId", joint.id, ja.id), format!("joints[id={}].actions[id={}].loadCaseId", joint.id, ja.id), &joint.id, "Joint action loadCaseId", "Anschluss-Einwirkung loadCaseId", &ja.load_case_id, load_case_ids.clone());
            }
        }
    }
    for fat in &document.fatigue_details {
        if !member_ids.iter().any(|id| id == &fat.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.fatigueDetails.{}.memberId", fat.id), format!("fatigueDetails[id={}].memberId", fat.id), &fat.id, "Fatigue memberId", "Ermüdung memberId", &fat.member_id, member_ids.clone());
        }
    }
    for fire in &document.fire_exposures {
        if !member_ids.iter().any(|id| id == &fire.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.fireExposures.{}.memberId", fire.id), format!("fireExposures[id={}].memberId", fire.id), &fire.id, "Fire memberId", "Brand memberId", &fire.member_id, member_ids.clone());
        }
    }
    for br in &document.bridge_fatigue {
        if !member_ids.iter().any(|id| id == &br.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.bridgeFatigue.{}.memberId", br.id), format!("bridgeFatigue[id={}].memberId", br.id), &br.id, "Bridge memberId", "Brücke memberId", &br.member_id, member_ids.clone());
        }
    }
    for tower in &document.tower_legs {
        if !member_ids.iter().any(|id| id == &tower.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.towerLegs.{}.memberId", tower.id), format!("towerLegs[id={}].memberId", tower.id), &tower.id, "Tower memberId", "Turm memberId", &tower.member_id, member_ids.clone());
        }
        for fa in &tower.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.towerLegs.{}.actions.{}.loadCaseId", tower.id, fa.id), format!("towerLegs[id={}].actions[id={}].loadCaseId", tower.id, fa.id), &tower.id, "Tower action loadCaseId", "Turm-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for pile in &document.piles {
        if !section_ids.iter().any(|id| id == &pile.section_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.piles.{}.sectionId", pile.id), format!("piles[id={}].sectionId", pile.id), &pile.id, "Pile sectionId", "Pfahl sectionId", &pile.section_id, section_ids.clone());
        }
        if !material_ids.iter().any(|id| id == &pile.material_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.piles.{}.materialId", pile.id), format!("piles[id={}].materialId", pile.id), &pile.id, "Pile materialId", "Pfahl materialId", &pile.material_id, material_ids.clone());
        }
        for fa in &pile.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.piles.{}.actions.{}.loadCaseId", pile.id, fa.id), format!("piles[id={}].actions[id={}].loadCaseId", pile.id, fa.id), &pile.id, "Pile action loadCaseId", "Pfahl-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for crane in &document.crane_runways {
        if !member_ids.iter().any(|id| id == &crane.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.craneRunways.{}.memberId", crane.id), format!("craneRunways[id={}].memberId", crane.id), &crane.id, "Crane memberId", "Kranbahn memberId", &crane.member_id, member_ids.clone());
        }
        for fa in &crane.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.craneRunways.{}.actions.{}.loadCaseId", crane.id, fa.id), format!("craneRunways[id={}].actions[id={}].loadCaseId", crane.id, fa.id), &crane.id, "Crane action loadCaseId", "Kranbahn-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for cf in &document.cold_formed_members {
        for fa in &cf.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.coldFormedMembers.{}.actions.{}.loadCaseId", cf.id, fa.id), format!("coldFormedMembers[id={}].actions[id={}].loadCaseId", cf.id, fa.id), &cf.id, "Cold-formed action loadCaseId", "Kaltprofil-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for pp in &document.plated_panels {
        for fa in &pp.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.platedPanels.{}.actions.{}.loadCaseId", pp.id, fa.id), format!("platedPanels[id={}].actions[id={}].loadCaseId", pp.id, fa.id), &pp.id, "Plated action loadCaseId", "Beulpanel-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for ten in &document.tension_components {
        for fa in &ten.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.tensionComponents.{}.actions.{}.loadCaseId", ten.id, fa.id), format!("tensionComponents[id={}].actions[id={}].loadCaseId", ten.id, fa.id), &ten.id, "Tension action loadCaseId", "Zugglied-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
}
//#endregion 🔖️ComplianceHelpers
