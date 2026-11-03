//! 🎯️ EN 1999 checks of the fatigue site, web patch load and ring stiffener rows: Miner damage per fatigue category (EN 1999-1-3 §7), web crippling (EN 1999-1-4 §6.1.7) and transverse web load (EN 1999-1-1 §6.7.5), ring stiffener section and bays (EN 1999-1-5). Coefficients that are not transcribed stay unverified registry rows and the check then reports the formula structure only.

use super::*;
use super::part_1_4_1_5 as thin;
use semio_s_plugin_norm_compliance::CheckStatus;

//#region 🔖️ComplianceHelpers

fn reference_fail(id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, text: LocalizedCopy, options: Vec<String>, annex: AnnexChoice, action: LocalizedCopy) -> CheckResult {
    CheckResult::assess(id, part, clause, subject.clone(), title).status(CheckStatus::Fail).explanation(text).annex(annex).remedy(Remedy::one_of(subject, options, action)).build()
}

pub fn check_fatigue_category(cat: &FatigueCategory, doc: &En1999Snapshot, annex: AnnexChoice) -> Vec<CheckResult> {
    let label = loc(&format!("Fatigue site {}", cat.id), &format!("Ermüdungsstelle {}", cat.id));
    let subject = |path: &str| SubjectRef::new(&cat.id, format!("fatigueCategories[id={}].{path}", cat.id), label.clone());
    let (part, cl) = ("DIN EN 1999-1-3", ClauseId::new("EN 1999-1-3", "§7.1", "7.1"));
    let mut out = Vec::new();
    let Some(member) = doc.members.iter().find(|m| m.id == cat.member_id) else {
        out.push(reference_fail(format!("en1999.1-3.fat.site-ref.{}", cat.id), part, cl, subject("memberId"), loc("Member of the fatigue site", "Bauteil der Ermüdungsstelle"), loc("The fatigue site names no existing member.", "Die Ermüdungsstelle nennt kein vorhandenes Bauteil."), doc.members.iter().map(|m| m.id.clone()).collect(), annex, loc("Point memberId at an existing member.", "memberId auf ein vorhandenes Bauteil setzen.")));
        return out;
    };
    let located = if member.length > 0.0 { cat.location_x.abs() } else { 0.0 };
    let mut place = CheckResult::assess(format!("en1999.1-3.fat.site-location.{}", cat.id), part, cl.clone(), subject("locationX"), loc("The fatigue site lies on the member", "Die Ermüdungsstelle liegt auf dem Bauteil"))
        .utilization(q_length(located), q_length(member.length))
        .annex(annex)
        .explanation(loc(&format!("x = {:.3} m of the member length {:.3} m.", cat.location_x, member.length), &format!("x = {:.3} m von der Bauteillänge {:.3} m.", cat.location_x, member.length)));
    if cat.location_x < 0.0 {
        place = place.status(CheckStatus::Fail).remedy(Remedy::at_least(subject("locationX"), q_length(cat.location_x), q_length(0.0), loc("Move the site onto the member.", "Stelle auf das Bauteil verschieben.")));
    } else if cat.location_x > member.length {
        place = place.remedy(Remedy::at_most(subject("locationX"), q_length(cat.location_x), q_length(member.length), loc("Move the site onto the member.", "Stelle auf das Bauteil verschieben.")));
    }
    out.push(place.build());
    let Some(dc_category) = part_1_3::detail_category_delta_sigma_c(&cat.detail_category) else {
        out.push(reference_fail(format!("en1999.1-3.fat.site-category.{}", cat.id), part, cl, subject("detailCategory"), loc("Annex J detail category", "Kerbfallklasse nach Annex J"), loc(&format!("Unknown detail category '{}'.", cat.detail_category), &format!("Unbekannte Kerbfallklasse '{}'.", cat.detail_category)), part_1_3::CATEGORY_MPA.iter().map(|c| format!("{c:.0}")).collect(), annex, loc("Pick an Annex J detail category.", "Kerbfallklasse aus Annex J wählen.")));
        return out;
    };
    let params = na_de::AnnexParams::for_choice(annex);
    let blocks = part_1_3::category_blocks(cat, &doc.fatigue_details);
    let id = format!("en1999.1-3.fat.category.{}", cat.id);
    if blocks.is_empty() {
        out.push(CheckResult::assess(id, part, cl, subject("detailCategory"), loc("Fatigue damage of the site", "Ermüdungsschädigung der Stelle")).status(CheckStatus::Info).annex(annex).explanation(loc(
            &format!("No fatigueDetails row of member {} with detail category {} carries a stress-range block of this site (Δσ_C = {:.0} MPa).", cat.member_id, cat.detail_category, dc_category / 1e6),
            &format!("Keine fatigueDetails-Zeile des Bauteils {} mit Kerbfall {} liefert einen Spannungsschwingbreiten-Block für diese Stelle (Δσ_C = {:.0} MPa).", cat.member_id, cat.detail_category, dc_category / 1e6))).build());
        return out;
    }
    let Some(d) = part_1_3::category_damage(cat, &doc.fatigue_details, params.gamma_ff, params.gamma_mf) else {
        out.push(CheckResult::assess(id, part, cl, subject("m1"), loc("Fatigue damage of the site", "Ermüdungsschädigung der Stelle")).status(CheckStatus::Fail).annex(annex).explanation(loc("The first S-N slope m1 must be positive (set it on the site or on its first block).", "Die erste Wöhler-Neigung m1 muss positiv sein (an der Stelle oder am ersten Block setzen).")).remedy(Remedy::one_of(subject("m1"), vec!["3".into(), "4".into(), "5".into()], loc("Set the first S-N slope m1 to a positive value, for example 3 to 5 as in the Annex J detail categories.", "Die erste Wöhler-Neigung m1 auf einen positiven Wert setzen, zum Beispiel 3 bis 5 wie bei den Kerbfällen nach Annex J."))).build());
        return out;
    };
    let total = |scale: f64, dc: f64| blocks.iter().map(|b| part_1_3::damage(b.delta_sigma_ed * scale, b.n_cycles, dc, d.m1, d.m2, params.gamma_ff, params.gamma_mf)).sum::<f64>();
    let mut remedies = Vec::new();
    if d.total > 1.0 {
        let (mut lo, mut hi) = (0.0, 1.0);
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if total(mid, d.delta_sigma_c) <= 0.98 { lo = mid; } else { hi = mid; }
        }
        for b in &blocks {
            remedies.push(Remedy::at_most(SubjectRef::new(&b.id, format!("fatigueDetails[id={}].deltaSigmaEd", b.id), label.clone()), q_stress(b.delta_sigma_ed), q_stress(b.delta_sigma_ed * lo), loc(&format!("Scale every block range of the site by {lo:.2}."), &format!("Alle Blockschwingbreiten der Stelle mit {lo:.2} skalieren."))));
        }
        let better: Vec<String> = part_1_3::CATEGORY_MPA.iter().filter(|c| total(1.0, *c * 1.0e6) <= 0.98).map(|c| format!("{c:.0}")).collect();
        if !better.is_empty() {
            remedies.push(Remedy::one_of(subject("detailCategory"), better, loc("Select a higher Annex J detail category.", "Höheren Annex-J-Kerbfall wählen.")));
        }
    }
    let list = blocks.iter().zip(&d.shares).map(|(b, s)| format!("{}: Δσ = {:.1} MPa, n = {:.2e}, D = {s:.3}", b.id, b.delta_sigma_ed / 1e6, b.n_cycles)).collect::<Vec<_>>().join("; ");
    let text = format!("D = Σ n/N = {:.3} over {} blocks at x = {:.2} m ({:?} detail, category {}: Δσ_C = {:.0} MPa, m1 = {:.1}, m2 = {:.1}, γ_Ff = {:.2}, γ_Mf = {:.2}); governing block {} with {:.0} % of the damage; {list}", d.total, blocks.len(), cat.location_x, cat.detail_type, cat.detail_category, d.delta_sigma_c / 1e6, d.m1, d.m2, params.gamma_ff, params.gamma_mf, blocks[d.governing].id, 100.0 * d.shares[d.governing] / d.total.max(1e-300));
    let text_de = format!("D = Σ n/N = {:.3} über {} Blöcke bei x = {:.2} m ({:?}-Detail, Kerbfall {}: Δσ_C = {:.0} MPa, m1 = {:.1}, m2 = {:.1}, γ_Ff = {:.2}, γ_Mf = {:.2}); maßgebender Block {} mit {:.0} % der Schädigung; {list}", d.total, blocks.len(), cat.location_x, cat.detail_type, cat.detail_category, d.delta_sigma_c / 1e6, d.m1, d.m2, params.gamma_ff, params.gamma_mf, blocks[d.governing].id, 100.0 * d.shares[d.governing] / d.total.max(1e-300));
    out.push(utilization_check(id, part, cl, subject("detailCategory"), loc("Fatigue damage of the site D = Σ n/N (Palmgren–Miner)", "Ermüdungsschädigung der Stelle D = Σ n/N (Palmgren–Miner)"), q_dim(d.total), q_dim(1.0), annex, loc(&text, &text_de), remedies));
    out
}

fn unverified_structure(id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, annex: AnnexChoice, ndps: &[&str], text: (String, String)) -> CheckResult {
    CheckResult::assess(id, part, clause, subject, title).status(CheckStatus::UnverifiedBasis).annex(annex).explanation(loc(&text.0, &text.1)).ndps(ndps).ndp_reads().build()
}

fn not_applicable(id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, annex: AnnexChoice, reason: LocalizedCopy) -> CheckResult {
    CheckResult::assess(id, part, clause, subject, title).not_applicable(reason).annex(annex).build()
}

fn grow_until(start: f64, step: f64, steps: usize, ok: impl Fn(f64) -> bool) -> Option<f64> {
    let mut v = start.max(1e-4);
    for _ in 0..steps {
        v *= step;
        if ok(v) { return Some(v); }
    }
    None
}

pub fn check_patch_load(load: &PatchLoad, doc: &En1999Snapshot, annex: AnnexChoice) -> Vec<CheckResult> {
    let label = loc(&format!("Patch load {}", load.id), &format!("Stegeinzellast {}", load.id));
    let subject = |path: &str| SubjectRef::new(&load.id, format!("patchLoads[id={}].{path}", load.id), label.clone());
    let params = na_de::AnnexParams::for_choice(annex);
    if let Some(sheet) = doc.cold_formed.iter().find(|s| s.id == load.subject_id) {
        return sheet_patch(load, sheet, doc, &params, annex, &subject, &label);
    }
    if let Some(member) = doc.members.iter().find(|m| m.id == load.subject_id) {
        return member_patch(load, member, doc, &params, annex, &subject, &label);
    }
    let options = doc.members.iter().map(|m| m.id.clone()).chain(doc.cold_formed.iter().map(|s| s.id.clone())).collect();
    vec![reference_fail(format!("en1999.6.7.5.web-ref.{}", load.id), "DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§6.7.5", "6.7.5"), subject("subjectId"), loc("Web carrying the patch load", "Steg der Einzellast"), loc("The patch load names neither an existing member nor a cold-formed sheet.", "Die Einzellast nennt weder ein vorhandenes Bauteil noch ein kaltgeformtes Blech."), options, annex, loc("Point subjectId at a member or a cold-formed sheet.", "subjectId auf ein Bauteil oder ein kaltgeformtes Blech setzen."))]
}

/// 🧮️ The resistance of a patch load for a trial bearing length: `f_rd` [N], the stiff-bearing spread `l_y` and effective length `l_eff` [m] of the web, its height `h_w` and thickness `t_w` [m] (a sheet web has `h_w` = 0 and `l_y` = `l_eff` = s_s).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PatchResistance {
    pub f_rd: f64,
    pub l_y: f64,
    pub l_eff: f64,
    pub h_w: f64,
    pub t_w: f64,
}

/// 📈️ Resistance of `load` for the bearing length `s_s` with the same rules as [`check_patch_load`] (EN 1999-1-1 §6.7.5 for a member web with the minimum over its panels, EN 1999-1-4 §6.1.7 for a sheet web); `None` while a coefficient is unverified (null registry row, load at a member end), the section is unusable or the references are missing.
pub fn patch_resistance_at(load: &PatchLoad, doc: &En1999Snapshot, annex: AnnexChoice, s_s: f64) -> Option<PatchResistance> {
    let params = na_de::AnnexParams::for_choice(annex);
    if let Some(sheet) = doc.cold_formed.iter().find(|s| s.id == load.subject_id) {
        let material = doc.materials.iter().find(|m| m.id == sheet.material_id)?;
        let alloy = part_1_1::resolve_alloy_for_thickness(&material.designation, sheet.thickness)?;
        let c = thin::web_crippling_coefficients(if load.position == semio_s_artifact_norm_en1999_model::PatchLoadPosition::End { "end" } else { "interior" }, annex)?;
        let phi = if load.web_angle > 0.0 { load.web_angle.to_degrees() } else { 90.0 };
        return Some(PatchResistance { f_rd: thin::web_crippling_resistance(sheet.thickness, alloy.f_o_pa, load.corner_radius, s_s, phi, c, params.gamma_m1), l_y: s_s, l_eff: s_s, h_w: 0.0, t_w: sheet.thickness });
    }
    let member = doc.members.iter().find(|m| m.id == load.subject_id)?;
    let section = doc.sections.iter().find(|s| s.id == member.section_id)?;
    let material = doc.materials.iter().find(|m| m.id == member.material_id)?;
    let alloy = part_1_1::resolve_alloy_for_section(&material.designation, section)?;
    let (t_w, t_f, b_f) = (section.web_thickness, section.flange_thickness, section.width);
    let h_w = if load.web_height > 0.0 { load.web_height } else { section.height - 2.0 * t_f };
    if matches!(section.kind, semio_s_artifact_norm_en1999_model::SectionKind::Chs | semio_s_artifact_norm_en1999_model::SectionKind::Free) || t_w <= 0.0 || t_f <= 0.0 || h_w <= 0.0 || load.position == semio_s_artifact_norm_en1999_model::PatchLoadPosition::End {
        return None;
    }
    let k = thin::transverse_load_constants(annex)?;
    let spacings = web_stiffener::spacings(&web_stiffener::web_panels(member.length, &web_stiffener::positions_of(&doc.transverse_stiffeners, &member.id)));
    let at = |a: f64| thin::transverse_load_resistance(h_w, t_w, t_f, b_f, alloy.f_o_pa, s_s, a, k, params.gamma_m1);
    let r = spacings.iter().skip(1).fold(at(spacings.first().copied().unwrap_or(0.0)), |m, a| { let r = at(*a); if r.f_rd < m.f_rd { r } else { m } });
    Some(PatchResistance { f_rd: r.f_rd, l_y: r.l_y, l_eff: r.l_eff, h_w, t_w })
}

fn sheet_patch(load: &PatchLoad, sheet: &ColdFormedSheet, doc: &En1999Snapshot, params: &na_de::AnnexParams, annex: AnnexChoice, subject: &dyn Fn(&str) -> SubjectRef, label: &LocalizedCopy) -> Vec<CheckResult> {
    let (part, cl) = ("DIN EN 1999-1-4", ClauseId::new("EN 1999-1-4", "§6.1.7", "6.1.7"));
    let Some(material) = doc.materials.iter().find(|m| m.id == sheet.material_id) else {
        return vec![reference_fail(format!("en1999.1-4.web-ref.{}", load.id), part, cl, subject("subjectId"), loc("Material of the loaded sheet", "Werkstoff des belasteten Blechs"), loc("The loaded sheet has no existing material.", "Das belastete Blech hat keinen vorhandenen Werkstoff."), doc.materials.iter().map(|m| m.id.clone()).collect(), annex, loc("Point the sheet materialId at an existing material.", "materialId des Blechs auf einen vorhandenen Werkstoff setzen."))];
    };
    let Some(alloy) = part_1_1::resolve_alloy_for_thickness(&material.designation, sheet.thickness) else {
        return vec![alloy_unknown(format!("en1999.1-4.web-alloy.{}", load.id), &load.id, material, annex)];
    };
    let pos = if load.position == semio_s_artifact_norm_en1999_model::PatchLoadPosition::End { "end" } else { "interior" };
    let keys: Vec<String> = thin::WEB_CRIPPLING_KEYS.iter().map(|k| format!("ndp.en1999.web-crippling.{pos}.{k}")).collect();
    let mut ndps: Vec<&str> = keys.iter().map(String::as_str).collect();
    ndps.push(na_de::AnnexParams::GAMMA_M1);
    let phi = if load.web_angle > 0.0 { load.web_angle.to_degrees() } else { 90.0 };
    let (t, f_o) = (sheet.thickness, alloy.f_o_pa);
    let structure = format!("R_w,Rd = α·t²·√(f_o·E)·(1 − c_r·√(r/t))·(c_a + √(c_l·s_s/t))·(c_φ + (φ/90°)²)/γ_M1 for the {pos} load position with t = {:.2} mm, r = {:.1} mm, s_s = {:.1} mm, φ = {phi:.0}°, f_o = {:.0} MPa, γ_M1 = {:.2}, F_Ed = {:.2} kN", t * 1e3, load.corner_radius * 1e3, load.bearing_length * 1e3, f_o / 1e6, params.gamma_m1, load.load / 1e3);
    let title = loc("Web crippling R_w,Rd of the sheet web", "Stegkrüppeln R_w,Rd des Blechstegs");
    let id = format!("en1999.1-4.web-crippling.{}", load.id);
    let Some(c) = thin::web_crippling_coefficients(pos, annex) else {
        let missing: Vec<&str> = thin::WEB_CRIPPLING_KEYS.iter().copied().filter(|k| semio_s_plugin_norm_compliance::ndp_cell_opt("ndp.en1999.web-crippling", &[pos, *k], annex).is_none()).collect();
        return vec![unverified_structure(id, part, cl, subject("load"), title, annex, &ndps, (format!("Awaiting verified coefficients: {structure}; the ten rows ndp.en1999.web-crippling.{{end,interior}}.{{alpha,c-r,c-a,c-l,c-phi}} (EN 1999-1-4 §6.1.7) are null in the registry, so no resistance is evaluated until a reviewer enters them (load position {pos}: {})", missing.join(", ")), format!("Wartet auf geprüfte Beiwerte: {structure}; die zehn Zeilen ndp.en1999.web-crippling.{{end,interior}}.{{alpha,c-r,c-a,c-l,c-phi}} (EN 1999-1-4 §6.1.7) sind im Register null, daher wird kein Widerstand berechnet, bis ein Prüfer sie einträgt (Lastlage {pos}: {})", missing.join(", "))))];
    };
    let resistance = |t: f64, s: f64| thin::web_crippling_resistance(t, f_o, load.corner_radius, s, phi, c, params.gamma_m1);
    let r_rd = resistance(t, load.bearing_length);
    let mut remedies = Vec::new();
    if load.load.abs() > r_rd {
        if let Some(s) = grow_until(load.bearing_length, 1.25, 40, |s| resistance(t, s) >= load.load.abs()) {
            remedies.push(Remedy::at_least(subject("bearingLength"), q_length(load.bearing_length), q_length(s), loc("Lengthen the bearing length.", "Lasteinleitungslänge vergrößern.")));
        }
        if let Some(tt) = grow_until(t, 1.05, 80, |tt| resistance(tt, load.bearing_length) >= load.load.abs()) {
            remedies.push(Remedy::at_least(SubjectRef::new(&sheet.id, format!("coldFormed[id={}].thickness", sheet.id), label.clone()), q_length(t), q_length(tt), loc("Increase the sheet thickness.", "Blechdicke erhöhen.")));
        }
    }
    let mut b = CheckResult::assess(id, part, cl, subject("load"), title).utilization(q_force(load.load.abs()), q_force(r_rd)).annex(annex)
        .explanation(loc(&format!("{structure}; R_w,Rd = {:.2} kN", r_rd / 1e3), &format!("{structure}; R_w,Rd = {:.2} kN", r_rd / 1e3))).ndps(&ndps).ndp_reads();
    for r in remedies { b = b.remedy(r); }
    vec![b.build()]
}

fn member_patch(load: &PatchLoad, member: &AluminiumMember, doc: &En1999Snapshot, params: &na_de::AnnexParams, annex: AnnexChoice, subject: &dyn Fn(&str) -> SubjectRef, label: &LocalizedCopy) -> Vec<CheckResult> {
    let (part, cl) = ("DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§6.7.5", "6.7.5"));
    let id = format!("en1999.6.7.5.web.{}", load.id);
    let title = loc("Transverse load resistance F_Rd of the web", "Querlastwiderstand F_Rd des Stegs");
    let (Some(section), Some(material)) = (doc.sections.iter().find(|s| s.id == member.section_id), doc.materials.iter().find(|m| m.id == member.material_id)) else {
        return vec![reference_fail(format!("en1999.6.7.5.web-ref.{}", load.id), part, cl, subject("subjectId"), loc("Section and material of the loaded member", "Querschnitt und Werkstoff des belasteten Bauteils"), loc("The loaded member has no existing section or material.", "Das belastete Bauteil hat keinen vorhandenen Querschnitt oder Werkstoff."), doc.members.iter().map(|m| m.id.clone()).collect(), annex, loc("Fix the member references first.", "Zuerst die Bauteilverweise korrigieren."))];
    };
    let Some(alloy) = part_1_1::resolve_alloy_for_section(&material.designation, section) else {
        return vec![alloy_unknown(format!("en1999.6.7.5.web-alloy.{}", load.id), &load.id, material, annex)];
    };
    let (t_w, t_f, b_f) = (section.web_thickness, section.flange_thickness, section.width);
    let h_w = if load.web_height > 0.0 { load.web_height } else { section.height - 2.0 * t_f };
    if matches!(section.kind, semio_s_artifact_norm_en1999_model::SectionKind::Chs | semio_s_artifact_norm_en1999_model::SectionKind::Free) || t_w <= 0.0 || t_f <= 0.0 || h_w <= 0.0 {
        return vec![not_applicable(id, part, cl, subject("subjectId"), title, annex, loc("The section has no flanged web (h_w, t_w, t_f): the rule of EN 1999-1-1 §6.7.5 does not apply.", "Der Querschnitt hat keinen Steg mit Flanschen (h_w, t_w, t_f): die Regel nach EN 1999-1-1 §6.7.5 gilt nicht."))];
    }
    let s_s = if load.bearing_length > 0.0 { load.bearing_length } else { member.bearing_length };
    let keys = ["kf-base", "kf-slope", "fcr-factor", "chi-const", "m2-factor"].map(|k| format!("ndp.en1999.transverse-load.{k}"));
    let mut ndps: Vec<&str> = keys.iter().map(String::as_str).collect();
    ndps.push(na_de::AnnexParams::GAMMA_M1);
    let f_o = alloy.f_o_pa;
    let web_panels = web_stiffener::web_panels(member.length, &web_stiffener::positions_of(&doc.transverse_stiffeners, &member.id));
    let spacings = web_stiffener::spacings(&web_panels);
    let structure = format!("F_Rd = f_o·χ_F·l_y·t_w/γ_M1 with k_F = k0 + k1·(h_w/a)², l_y = s_s + 2·t_f·(1 + √(m_1 + m_2)) ≤ a, F_cr = c·k_F·E·t_w³/h_w, χ_F = min(χ0/λ̄_F, 1); h_w = {:.1} mm, t_w = {:.1} mm, t_f = {:.1} mm, b_f = {:.1} mm, s_s = {:.1} mm, a = {:.0} mm, f_o = {:.0} MPa, F_Ed = {:.2} kN", h_w * 1e3, t_w * 1e3, t_f * 1e3, b_f * 1e3, s_s * 1e3, web_stiffener::widest(&web_panels) * 1e3, f_o / 1e6, load.load / 1e3);
    let constants = if load.position == semio_s_artifact_norm_en1999_model::PatchLoadPosition::End { None } else { thin::transverse_load_constants(annex) };
    let Some(k) = constants else {
        let why = if load.position == semio_s_artifact_norm_en1999_model::PatchLoadPosition::End { ("the rule for a load at the end of the member (near a support) is not transcribed, so no resistance is evaluated", "die Regel für eine Last am Bauteilende (nahe Auflager) ist nicht übernommen, daher wird kein Widerstand berechnet") } else { ("the structure constants ndp.en1999.transverse-load.* are not registered as numbers", "die Strukturkonstanten ndp.en1999.transverse-load.* sind nicht als Zahlen registriert") };
        return vec![unverified_structure(id, part, cl, subject("load"), title, annex, &ndps, (format!("{structure}; {}.", why.0), format!("{structure}; {}.", why.1)))];
    };
    let at = |tw: f64, s: f64, a: f64| thin::transverse_load_resistance(h_w, tw, t_f, b_f, f_o, s, a, k, params.gamma_m1);
    let res = |tw: f64, s: f64| spacings.iter().skip(1).fold(at(tw, s, spacings.first().copied().unwrap_or(0.0)), |m, a| { let r = at(tw, s, *a); if r.f_rd < m.f_rd { r } else { m } });
    let r = res(t_w, s_s);
    let mut remedies = Vec::new();
    if load.load.abs() > r.f_rd {
        if let Some(s) = grow_until(s_s, 1.25, 40, |s| res(t_w, s).f_rd >= load.load.abs()) {
            remedies.push(Remedy::at_least(subject("bearingLength"), q_length(s_s), q_length(s), loc("Lengthen the bearing length.", "Lasteinleitungslänge vergrößern.")));
        }
        if let Some(tw) = grow_until(t_w, 1.05, 80, |tw| res(tw, s_s).f_rd >= load.load.abs()) {
            remedies.push(Remedy::at_least(SubjectRef::new(&section.id, format!("sections[id={}].webThickness", section.id), label.clone()), q_length(t_w), q_length(tw), loc("Increase the web thickness.", "Stegdicke erhöhen.")));
        }
    }
    let text = format!("{structure}; k_F = {:.2}, l_y = {:.1} mm, F_cr = {:.1} kN, λ̄_F = {:.3}, χ_F = {:.3}, L_eff = {:.1} mm, F_Rd = {:.2} kN", r.k_f, r.l_y * 1e3, r.f_cr / 1e3, r.lambda_f, r.chi_f, r.l_eff * 1e3, r.f_rd / 1e3);
    let mut b = CheckResult::assess(id, part, cl, subject("load"), title).utilization(q_force(load.load.abs()), q_force(r.f_rd)).annex(annex).explanation(loc(&text, &text)).ndps(&ndps).ndp_reads();
    for rem in remedies { b = b.remedy(rem); }
    vec![b.build()]
}

/// 🧱 Properties of the insulation of a fire scenario (EN 1999-1-2 4.3.4): the thickness is not negative (0 = unprotected member), and conductivity, specific heat and density are positive once there is a thickness; the insulation names an existing fire scenario.
pub fn check_fire_insulation(insulation: &semio_s_artifact_norm_en1999_model::FireInsulation, doc: &En1999Snapshot, annex: AnnexChoice) -> Vec<CheckResult> {
    let label = loc(&format!("Fire insulation {}", insulation.id), &format!("Brandschutzbekleidung {}", insulation.id));
    let subject = |path: &str| SubjectRef::new(&insulation.id, format!("fireInsulations[id={}].{path}", insulation.id), label.clone());
    let (part, cl) = ("DIN EN 1999-1-2", ClauseId::new("EN 1999-1-2", "§4.3.4", "4.3.4"));
    if !doc.fire_scenarios.iter().any(|f| f.id == insulation.fire_id) {
        return vec![reference_fail(format!("en1999.1-2.insulation-ref.{}", insulation.id), part, cl, subject("fireId"), loc("Fire scenario of the insulation", "Brandszenario der Bekleidung"), loc("The insulation names no existing fire scenario.", "Die Bekleidung nennt kein vorhandenes Brandszenario."), doc.fire_scenarios.iter().map(|f| f.id.clone()).collect(), annex, loc("Point fireId at an existing fire scenario.", "fireId auf ein vorhandenes Brandszenario setzen."))];
    }
    let fields = [("thickness", insulation.thickness, "Insulation thickness d_p", "Bekleidungsdicke d_p"), ("conductivity", insulation.conductivity, "Conductivity λ_p", "Wärmeleitfähigkeit λ_p"), ("specificHeat", insulation.specific_heat, "Specific heat c_p", "Spezifische Wärmekapazität c_p"), ("density", insulation.density, "Density ρ_p", "Rohdichte ρ_p")];
    let invalid = |path: &str, value: f64| if path == "thickness" { value < 0.0 } else { insulation.thickness > 0.0 && value <= 0.0 };
    let text = format!("d_p = {:.1} mm, λ_p = {:.3} W/(m·K), c_p = {:.0} J/(kg·K), ρ_p = {:.0} kg/m³", insulation.thickness * 1e3, insulation.conductivity, insulation.specific_heat, insulation.density);
    let mut check = CheckResult::assess(format!("en1999.1-2.insulation.{}", insulation.id), part, cl, subject("thickness"), loc("Insulation properties are positive", "Bekleidungskennwerte sind positiv"))
        .utilization(q_dim(if fields.iter().any(|f| invalid(f.0, f.1)) { 2.0 } else { 0.0 }), q_dim(1.0))
        .annex(annex)
        .explanation(loc(&text, &text));
    for (path, value, en, de) in fields {
        if invalid(path, value) {
            let (now, wanted) = if path == "thickness" { (q_length(value), q_length(0.0)) } else { (Quantity::new(QuantityKind::Dimensionless, value), Quantity::new(QuantityKind::Dimensionless, 1.0)) };
            check = check.status(CheckStatus::Fail).remedy(Remedy::at_least(subject(path), now, wanted, loc(&format!("Give {en} a valid value."), &format!("{de} einen gültigen Wert geben."))));
        }
    }
    vec![check.build()]
}

/// 🪗 Geometry of the trapezoidal rib of a profiled sheet (EN 1999-1-4 5.5): positive flanges and height and a pitch that holds both flanges, `b_t + b_b ≤ p`; the profile names an existing sheet.
pub fn check_sheet_profile(profile: &semio_s_artifact_norm_en1999_model::SheetProfile, doc: &En1999Snapshot, annex: AnnexChoice) -> Vec<CheckResult> {
    let label = loc(&format!("Sheet profile {}", profile.id), &format!("Blechprofil {}", profile.id));
    let subject = |path: &str| SubjectRef::new(&profile.id, format!("sheetProfiles[id={}].{path}", profile.id), label.clone());
    let (part, cl) = ("DIN EN 1999-1-4", ClauseId::new("EN 1999-1-4", "§5.5", "5.5"));
    let Some(sheet) = doc.cold_formed.iter().find(|s| s.id == profile.sheet_id) else {
        return vec![reference_fail(format!("en1999.1-4.profile-ref.{}", profile.id), part, cl, subject("sheetId"), loc("Sheet of the profile", "Blech des Profils"), loc("The profile names no existing sheet.", "Das Profil nennt kein vorhandenes Blech."), doc.cold_formed.iter().map(|s| s.id.clone()).collect(), annex, loc("Point sheetId at an existing sheet.", "sheetId auf ein vorhandenes Blech setzen."))];
    };
    let rib = thin::Rib { pitch: profile.pitch, top: profile.top_flange, bottom: profile.bottom_flange, height: profile.web_height };
    let flanges = profile.top_flange + profile.bottom_flange;
    let minimum = 10.0 * sheet.thickness;
    let text = format!("p = {:.1} mm, b_t = {:.1} mm, b_b = {:.1} mm, h = {:.1} mm, t = {:.2} mm: web projection {:.1} mm, s_w = {:.1} mm, φ = {:.1}°", profile.pitch * 1e3, profile.top_flange * 1e3, profile.bottom_flange * 1e3, profile.web_height * 1e3, sheet.thickness * 1e3, rib.projection() * 1e3, rib.web_length() * 1e3, rib.web_angle().to_degrees());
    let mut check = CheckResult::assess(format!("en1999.1-4.profile.{}", profile.id), part, cl, subject("pitch"), loc("Rib geometry b_t + b_b ≤ p, h > 0", "Rippengeometrie b_t + b_b ≤ p, h > 0"))
        .utilization(q_length(flanges), q_length(profile.pitch.max(1e-9)))
        .annex(annex)
        .explanation(loc(&text, &text));
    if !rib.is_valid() {
        check = check.status(CheckStatus::Fail);
        if flanges > profile.pitch { check = check.remedy(Remedy::at_least(subject("pitch"), q_length(profile.pitch), q_length(flanges + 2.0 * profile.web_height.max(minimum)), loc("Widen the pitch so that both flanges and the webs fit.", "Rippenabstand so vergrößern, dass beide Gurte und die Stege passen."))); }
        if profile.web_height <= 0.0 { check = check.remedy(Remedy::at_least(subject("webHeight"), q_length(profile.web_height), q_length(minimum), loc("Give the profile a height.", "Dem Profil eine Höhe geben."))); }
        if profile.top_flange <= 0.0 { check = check.remedy(Remedy::at_least(subject("topFlange"), q_length(profile.top_flange), q_length(minimum), loc("Give the top flange a width.", "Dem Obergurt eine Breite geben."))); }
        if profile.bottom_flange <= 0.0 { check = check.remedy(Remedy::at_least(subject("bottomFlange"), q_length(profile.bottom_flange), q_length(minimum), loc("Give the bottom flange a width.", "Dem Untergurt eine Breite geben."))); }
    }
    vec![check.build()]
}

pub fn check_ring_stiffener(ring: &RingStiffener, doc: &En1999Snapshot, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<CheckResult> {
    let label = loc(&format!("Ring stiffener {}", ring.id), &format!("Ringsteife {}", ring.id));
    let subject = |path: &str| SubjectRef::new(&ring.id, format!("ringStiffeners[id={}].{path}", ring.id), label.clone());
    let (part, cl) = ("DIN EN 1999-1-5", ClauseId::new("EN 1999-1-5", "§5", "5.3"));
    let Some(shell) = doc.shells.iter().find(|s| s.id == ring.shell_id) else {
        return vec![reference_fail(format!("en1999.1-5.stiffener-ref.{}", ring.id), part, cl, subject("shellId"), loc("Shell of the ring stiffener", "Schale der Ringsteife"), loc("The ring stiffener names no existing shell.", "Die Ringsteife nennt keine vorhandene Schale."), doc.shells.iter().map(|s| s.id.clone()).collect(), annex, loc("Point shellId at an existing shell.", "shellId auf eine vorhandene Schale setzen."))];
    };
    let mut out = Vec::new();
    let inside = ring.x > 0.0 && ring.x < shell.length;
    let mut place = CheckResult::assess(format!("en1999.1-5.stiffener-position.{}", ring.id), part, cl.clone(), subject("x"), loc("The ring stands inside the shell length", "Die Ringsteife steht innerhalb der Schalenlänge"))
        .utilization(q_length(ring.x.abs()), q_length(shell.length))
        .annex(annex)
        .explanation(loc(&format!("x = {:.3} m of the shell length {:.3} m (a ring at an end or outside does not divide the shell).", ring.x, shell.length), &format!("x = {:.3} m von der Schalenlänge {:.3} m (eine Ringsteife am Ende oder außerhalb teilt die Schale nicht).", ring.x, shell.length)));
    if !inside {
        place = place.status(CheckStatus::Fail).remedy(Remedy::exactly(subject("x"), q_length(ring.x), q_length(0.5 * shell.length), loc("Move the ring into the shell length.", "Ringsteife in die Schalenlänge verschieben.")));
    }
    out.push(place.build());
    let factor_id = "ndp.en1999.ring-stiffener.effective-width-factor";
    let required_id = "ndp.en1999.ring-stiffener.required-inertia-factor";
    let factor = semio_s_plugin_norm_compliance::ndp_number_opt(factor_id, annex);
    let margin = semio_s_plugin_norm_compliance::ndp_number_opt(required_id, annex);
    let gamma_r = margin.unwrap_or(1.0);
    let b_e = thin::ring_effective_width(shell.radius, shell.thickness, factor.unwrap_or(0.0));
    let (area, inertia, z_c) = thin::ring_section(shell.thickness, b_e, ring.depth, ring.thickness);
    let rings: Vec<f64> = doc.ring_stiffeners.iter().filter(|r| r.shell_id == shell.id).map(|r| r.x).collect();
    let bays = thin::ring_bays(shell.length, &rings);
    let longest = bays.iter().copied().fold(0.0, f64::max);
    let tributary = thin::ring_tributary(shell.length, &rings, ring.x);
    let params = na_de::AnnexParams::for_choice(annex);
    let sigma_theta = part_en1990::uls_combinations(&part_en1990::shell_proxy(shell), &params, reliability_class).iter().map(|g| g.m_y_ed.hypot(g.m_z_ed)).fold(0.0, f64::max);
    let w_ed = sigma_theta * shell.thickness / shell.radius.max(1e-9) * tributary;
    let line_load = |depth: f64, thickness: f64| thin::ring_critical_line_load(thin::ring_section(shell.thickness, b_e, depth, thickness).1, shell.radius);
    let w_cr = line_load(ring.depth, ring.thickness);
    let u = gamma_r * w_ed / w_cr.max(1e-9);
    let awaiting_en = if margin.is_none() { "; awaiting verified margin: γ_r is not registered, evaluated with 1.0" } else { "" };
    let awaiting_de = if margin.is_none() { "; wartet auf geprüften Sicherheitsbeiwert: γ_r ist nicht registriert, mit 1,0 ausgewertet" } else { "" };
    let text = format!("Flexural stability of the ring {:.0} × {:.0} mm ({} the shell) with the effective shell width b_e = {:.1} mm: A = {:.0} mm², I = {:.3e} mm⁴ about the combined centroid ({:.1} mm from the shell mid-surface); tributary length {:.3} m of {} bays (longest {:.3} m, ω = {:.2}); radial line load w_Ed = σ_θ,Ed·t/r·L = {:.2} kN/m against w_cr = 3·E·I/r³ = {:.2} kN/m, γ_r = {:.2}{awaiting_en}; the width factor ({factor_id}) is an unverified registry row", ring.depth * 1e3, ring.thickness * 1e3, if ring.internal { "inside" } else { "outside" }, b_e * 1e3, area * 1e6, inertia * 1e12, z_c * 1e3, tributary, bays.len(), longest, thin::omega(longest, shell.radius, shell.thickness), w_ed / 1e3, w_cr / 1e3, gamma_r);
    let text_de = format!("Biegestabilität der Ringsteife {:.0} × {:.0} mm ({} der Schale) mit mitwirkender Schalenbreite b_e = {:.1} mm: A = {:.0} mm², I = {:.3e} mm⁴ um den gemeinsamen Schwerpunkt ({:.1} mm von der Schalenmittelfläche); Einzugslänge {:.3} m von {} Feldern (längstes {:.3} m, ω = {:.2}); radiale Linienlast w_Ed = σ_θ,Ed·t/r·L = {:.2} kN/m gegen w_cr = 3·E·I/r³ = {:.2} kN/m, γ_r = {:.2}{awaiting_de}; der Breitenbeiwert ({factor_id}) ist eine ungeprüfte Registerzeile", ring.depth * 1e3, ring.thickness * 1e3, if ring.internal { "innerhalb" } else { "außerhalb" }, b_e * 1e3, area * 1e6, inertia * 1e12, z_c * 1e3, tributary, bays.len(), longest, thin::omega(longest, shell.radius, shell.thickness), w_ed / 1e3, w_cr / 1e3, gamma_r);
    let mut remedies = Vec::new();
    if u > 1.0 {
        if let Some(d) = grow_until(ring.depth, 1.05, 120, |d| gamma_r * w_ed <= line_load(d, ring.thickness)) {
            remedies.push(Remedy::at_least(subject("depth"), q_length(ring.depth), q_length(d), loc("Deepen the ring stiffener.", "Ringsteife höher machen.")));
        }
        if let Some(tt) = grow_until(ring.thickness, 1.05, 120, |tt| gamma_r * w_ed <= line_load(ring.depth, tt)) {
            remedies.push(Remedy::at_least(subject("thickness"), q_length(ring.thickness), q_length(tt), loc("Thicken the ring stiffener.", "Ringsteife dicker machen.")));
        }
        if remedies.is_empty() {
            remedies.push(Remedy::at_least(subject("depth"), q_length(ring.depth), q_length(ring.depth * u.cbrt() * 1.05), loc("Deepen the ring stiffener.", "Ringsteife höher machen.")));
        }
    }
    let mut b = CheckResult::assess(format!("en1999.1-5.stiffener.{}", ring.id), part, ClauseId::new("EN 1999-1-1", "§6.7.6", "6.7.6"), subject("depth"), loc("Ring stiffener flexural stability w_Ed ≤ w_cr/γ_r", "Biegestabilität der Ringsteife w_Ed ≤ w_cr/γ_r"))
        .utilization(q_force(gamma_r * w_ed), q_force(w_cr)).annex(annex).explanation(loc(&text, &text_de)).ndps(&[factor_id, required_id]);
    for r in remedies { b = b.remedy(r); }
    out.push(b.build());
    out
}

pub fn check_transverse_stiffener(st: &TransverseStiffener, doc: &En1999Snapshot, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<CheckResult> {
    let label = loc(&format!("Transverse stiffener {}", st.id), &format!("Quersteife {}", st.id));
    let subject = |path: &str| SubjectRef::new(&st.id, format!("transverseStiffeners[id={}].{path}", st.id), label.clone());
    let (part, cl) = ("DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§6.7.6", "6.7.6"));
    let members = || doc.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>();
    let Some(member) = doc.members.iter().find(|m| m.id == st.girder_id) else {
        return vec![reference_fail(format!("en1999.6.7.6.web-ref.{}", st.id), part, cl, subject("girderId"), loc("Plate girder of the transverse stiffener", "Blechträger der Quersteife"), loc("The transverse stiffener names no existing member.", "Die Quersteife nennt kein vorhandenes Bauteil."), members(), annex, loc("Point girderId at an existing member.", "girderId auf ein vorhandenes Bauteil setzen."))];
    };
    let (Some(section), Some(material)) = (doc.sections.iter().find(|s| s.id == member.section_id), doc.materials.iter().find(|m| m.id == member.material_id)) else {
        return vec![reference_fail(format!("en1999.6.7.6.web-section.{}", st.id), part, cl, subject("girderId"), loc("Section and material of the plate girder", "Querschnitt und Werkstoff des Blechträgers"), loc("The plate girder has no existing section or material.", "Der Blechträger hat keinen vorhandenen Querschnitt oder Werkstoff."), members(), annex, loc("Fix the member references first.", "Zuerst die Bauteilverweise korrigieren."))];
    };
    let Some(alloy) = part_1_1::resolve_alloy_for_section(&material.designation, section) else {
        return vec![alloy_unknown(format!("en1999.6.7.6.web-alloy.{}", st.id), &st.id, material, annex)];
    };
    let mut out = Vec::new();
    let inside = st.position > 0.0 && st.position < member.length;
    let mut place = CheckResult::assess(format!("en1999.6.7.6.web-position.{}", st.id), part, cl.clone(), subject("position"), loc("The stiffener stands inside the girder length", "Die Quersteife steht innerhalb der Trägerlänge"))
        .utilization(q_length(st.position.abs()), q_length(member.length))
        .annex(annex)
        .explanation(loc(&format!("x = {:.3} m of the girder length {:.3} m (a stiffener at an end is a bearing stiffener, outside it does not divide the web).", st.position, member.length), &format!("x = {:.3} m von der Trägerlänge {:.3} m (eine Steife am Ende ist eine Auflagersteife, außerhalb teilt sie den Steg nicht).", st.position, member.length)));
    if !inside {
        place = place.status(CheckStatus::Fail).remedy(Remedy::exactly(subject("position"), q_length(st.position), q_length(0.5 * member.length), loc("Move the stiffener into the girder length.", "Quersteife in die Trägerlänge verschieben.")));
    }
    out.push(place.build());
    let (t_w, t_f) = (section.web_thickness, section.flange_thickness);
    let h_w = section.height - 2.0 * t_f;
    let title = loc("Transverse stiffener stiffness I_st ≥ I_st,min", "Quersteifen-Steifigkeit I_st ≥ I_st,min");
    if matches!(section.kind, semio_s_artifact_norm_en1999_model::SectionKind::Chs | semio_s_artifact_norm_en1999_model::SectionKind::Free) || t_w <= 0.0 || t_f <= 0.0 || h_w <= 0.0 {
        out.push(not_applicable(format!("en1999.6.7.6.web-stiffness.{}", st.id), part, cl, subject("girderId"), title, annex, loc("The section has no flanged web (h_w, t_w, t_f): the rule for transverse web stiffeners does not apply.", "Der Querschnitt hat keinen Steg mit Flanschen (h_w, t_w, t_f): die Regel für Quersteifen gilt nicht.")));
        return out;
    }
    let ids: Vec<String> = web_stiffener::KEYS.iter().map(|k| format!("ndp.en1999.web-stiffener.{k}")).collect();
    let mut ndps: Vec<&str> = ids.iter().map(String::as_str).collect();
    ndps.push(na_de::AnnexParams::GAMMA_M1);
    let two = st.sides == semio_s_artifact_norm_en1999_model::StiffenerSides::Two;
    let Some(k) = web_stiffener::constants(annex) else {
        out.push(unverified_structure(format!("en1999.6.7.6.web-stiffness.{}", st.id), part, cl, subject("width"), title, annex, &ndps, ("Required stiffness of the transverse stiffener with its effective web strip: the structure constants ndp.en1999.web-stiffener.* are not registered as numbers.".to_owned(), "Erforderliche Steifigkeit der Quersteife mit mitwirkendem Stegstreifen: die Strukturkonstanten ndp.en1999.web-stiffener.* sind nicht als Zahlen registriert.".to_owned())));
        return out;
    };
    let params = na_de::AnnexParams::for_choice(annex);
    let f_o = alloy.f_o_pa;
    let eps = part_1_1::epsilon(f_o);
    let positions: Vec<f64> = doc.transverse_stiffeners.iter().filter(|s| s.girder_id == st.girder_id).map(|s| s.position).collect();
    let (before, after) = if inside { web_stiffener::panels(member.length, &positions, st.position) } else { (member.length, member.length) };
    let (a_min, a_max) = (before.min(after).max(1e-6), before.max(after).max(1e-6));
    let eff_of = |width: f64, thickness: f64| web_stiffener::effective_section(width, thickness, two, t_w, k.strip * eps * t_w);
    let i_min = web_stiffener::required_inertia(h_w, t_w, a_min, &k);
    let eff = eff_of(st.width, st.thickness);
    let ratio = |width: f64, thickness: f64| i_min / eff_of(width, thickness).inertia.max(1e-30);
    let mut remedies = Vec::new();
    if ratio(st.width, st.thickness) > 1.0 {
        if let Some(w) = grow_until(st.width, 1.05, 120, |w| ratio(w, st.thickness) <= 1.0) {
            remedies.push(Remedy::at_least(subject("width"), q_length(st.width), q_length(w), loc("Widen the transverse stiffener.", "Quersteife breiter machen.")));
        }
        if let Some(tt) = grow_until(st.thickness, 1.05, 120, |tt| ratio(st.width, tt) <= 1.0) {
            remedies.push(Remedy::at_least(subject("thickness"), q_length(st.thickness), q_length(tt), loc("Thicken the transverse stiffener.", "Quersteife dicker machen.")));
        }
    }
    let limb = if a_min / h_w < k.aspect { format!("I_st,min = {:.2}·h_w³·t_w³/a²", k.short) } else { format!("I_st,min = {:.2}·h_w·t_w³", k.long) };
    let text = format!("{limb} (aspect limit a/h_w = {:.3}) with the narrowest adjacent panel a = {:.0} mm (a/h_w = {:.2}), h_w = {:.1} mm, t_w = {:.2} mm: I_st,min = {:.4e} mm⁴ against I_st = {:.4e} mm⁴ of the {} stiffener {:.1} × {:.2} mm with the effective web strip {:.1} mm (±{:.0}·ε·t_w, ε = {:.3}), A = {:.0} mm²; the constants are unverified registry rows", k.aspect, a_min * 1e3, a_min / h_w, h_w * 1e3, t_w * 1e3, i_min * 1e12, eff.inertia * 1e12, if two { "two-sided" } else { "one-sided" }, st.width * 1e3, st.thickness * 1e3, eff.strip * 1e3, k.strip, eps, eff.area * 1e6);
    let text_de = format!("{limb} (Grenzwert a/h_w = {:.3}) mit dem schmalsten angrenzenden Feld a = {:.0} mm (a/h_w = {:.2}), h_w = {:.1} mm, t_w = {:.2} mm: I_st,min = {:.4e} mm⁴ gegen I_st = {:.4e} mm⁴ der {} Steife {:.1} × {:.2} mm mit mitwirkendem Stegstreifen {:.1} mm (±{:.0}·ε·t_w, ε = {:.3}), A = {:.0} mm²; die Konstanten sind ungeprüfte Registerzeilen", k.aspect, a_min * 1e3, a_min / h_w, h_w * 1e3, t_w * 1e3, i_min * 1e12, eff.inertia * 1e12, if two { "zweiseitigen" } else { "einseitigen" }, st.width * 1e3, st.thickness * 1e3, eff.strip * 1e3, k.strip, eps, eff.area * 1e6);
    let mut b = CheckResult::assess(format!("en1999.6.7.6.web-stiffness.{}", st.id), part, cl.clone(), subject("width"), title).utilization(q_dim(ratio(st.width, st.thickness)), q_dim(1.0)).annex(annex).explanation(loc(&text, &text_de)).ndps(&ndps).ndp_reads();
    for r in remedies { b = b.remedy(r); }
    out.push(b.build());
    let v_ed = part_en1990::uls_combinations(member, &params, reliability_class).iter().map(|g| g.v_z_ed.abs()).fold(0.0, f64::max);
    let f_loc = part_en1990::uls_combinations(member, &params, reliability_class).iter().map(|g| g.f_loc_ed.abs()).fold(0.0, f64::max);
    let rho_v = part_1_1::shear_buckling_rho(h_w / t_w, f_o, h_w, a_max);
    let n_ed = web_stiffener::axial_demand(v_ed, rho_v, h_w, t_w, f_o, f_loc, params.gamma_m1);
    let res = |width: f64, thickness: f64| web_stiffener::strut(&eff_of(width, thickness), h_w, f_o, alloy.buckling_class_a, params.gamma_m1, &k);
    let s = res(st.width, st.thickness);
    let mut remedies = Vec::new();
    if n_ed > s.n_rd {
        if let Some(w) = grow_until(st.width, 1.05, 120, |w| res(w, st.thickness).n_rd >= n_ed) {
            remedies.push(Remedy::at_least(subject("width"), q_length(st.width), q_length(w), loc("Widen the transverse stiffener.", "Quersteife breiter machen.")));
        }
        if let Some(tt) = grow_until(st.thickness, 1.05, 120, |tt| res(st.width, tt).n_rd >= n_ed) {
            remedies.push(Remedy::at_least(subject("thickness"), q_length(st.thickness), q_length(tt), loc("Thicken the transverse stiffener.", "Quersteife dicker machen.")));
        }
    }
    let text = format!("N_Ed = max(0, V_Ed − ρ_v·h_w·t_w·f_o/(√3·γ_M1)) + F_loc,Ed = {:.2} kN (V_Ed = {:.2} kN, ρ_v = {:.3} for the widest adjacent panel a = {:.0} mm, F_loc,Ed = {:.2} kN assumed to act on the stiffener) against N_b,Rd = χ·A·f_o/γ_M1 = {:.2} kN of the stiffener with its web strip as a strut of length {:.2}·h_w = {:.0} mm (A = {:.0} mm², I = {:.4e} mm⁴, λ̄ = {:.3}, χ = {:.3}, f_o = {:.0} MPa); the constants are unverified registry rows", n_ed / 1e3, v_ed / 1e3, rho_v, a_max * 1e3, f_loc / 1e3, s.n_rd / 1e3, k.length, s.l_cr * 1e3, eff.area * 1e6, eff.inertia * 1e12, s.lambda, s.chi, f_o / 1e6);
    let text_de = format!("N_Ed = max(0, V_Ed − ρ_v·h_w·t_w·f_o/(√3·γ_M1)) + F_loc,Ed = {:.2} kN (V_Ed = {:.2} kN, ρ_v = {:.3} für das breiteste angrenzende Feld a = {:.0} mm, F_loc,Ed = {:.2} kN wirkt als an der Steife angreifend angenommen) gegen N_b,Rd = χ·A·f_o/γ_M1 = {:.2} kN der Steife mit Stegstreifen als Druckstab der Länge {:.2}·h_w = {:.0} mm (A = {:.0} mm², I = {:.4e} mm⁴, λ̄ = {:.3}, χ = {:.3}, f_o = {:.0} MPa); die Konstanten sind ungeprüfte Registerzeilen", n_ed / 1e3, v_ed / 1e3, rho_v, a_max * 1e3, f_loc / 1e3, s.n_rd / 1e3, k.length, s.l_cr * 1e3, eff.area * 1e6, eff.inertia * 1e12, s.lambda, s.chi, f_o / 1e6);
    let mut b = CheckResult::assess(format!("en1999.6.7.6.web-strength.{}", st.id), part, cl, subject("thickness"), loc("Transverse stiffener strength N_Ed ≤ N_b,Rd", "Quersteifen-Tragfähigkeit N_Ed ≤ N_b,Rd")).utilization(q_force(n_ed), q_force(s.n_rd)).annex(annex).explanation(loc(&text, &text_de)).ndps(&ndps).ndp_reads();
    for r in remedies { b = b.remedy(r); }
    out.push(b.build());
    out
}

pub fn check_edge_stiffeners(member: &AluminiumMember, section: &AluminiumSection, material: &AluminiumMaterial, annex: AnnexChoice) -> Vec<CheckResult> {
    let Some(alloy) = part_1_1::resolve_alloy_for_section(&material.designation, section) else { return Vec::new() };
    let limit_id = "ndp.en1999.stiffener.edge-slenderness-limit";
    let limit = semio_s_plugin_norm_compliance::ndp_number_opt(limit_id, annex);
    let (part, cl) = ("DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§6.7.7", "6.7.7"));
    let mut out = Vec::new();
    for el in section.elements.iter().filter(|e| e.edge_stiffener_depth > 0.0 && e.edge_stiffener_thickness > 0.0 && e.outstand && e.width > 0.0 && e.thickness > 0.0) {
        let path = |leaf: &str| format!("sections[id={}].elements[id={}].{leaf}", section.id, el.id);
        let label = loc(&format!("Edge stiffener of element {}", el.id), &format!("Randsteife des Elements {}", el.id));
        let subject = |leaf: &str| SubjectRef::new(&member.id, path(leaf), label.clone());
        let id = format!("en1999.6.7.7.edge.{}.{}", member.id, el.id);
        let title = loc("Edge stiffener flexural stability λ̄_s ≤ λ̄_s,max", "Biegestabilität der Randsteife λ̄_s ≤ λ̄_s,max");
        let Some(limit) = limit else {
            out.push(unverified_structure(id, part, cl.clone(), subject("edgeStiffenerDepth"), title, annex, &[limit_id], ("Edge stiffener flexural stability: σ_cr,s = 2·√(K·E·I_s)/A_s, λ̄_s = √(f_o/σ_cr,s); awaiting verified limit λ̄_s,max.".to_owned(), "Biegestabilität der Randsteife: σ_cr,s = 2·√(K·E·I_s)/A_s, λ̄_s = √(f_o/σ_cr,s); wartet auf geprüften Grenzwert λ̄_s,max.".to_owned())));
            continue;
        };
        let f_o = alloy.f_o_pa;
        let sigma_of = |d: f64, ts: f64| {
            let (a_s, i_s) = thin::edge_stiffener_section(d, ts);
            thin::edge_stiffener_sigma_cr(thin::edge_stiffener_spring(el.thickness, el.width), i_s, a_s)
        };
        let sigma_cr = sigma_of(el.edge_stiffener_depth, el.edge_stiffener_thickness);
        let u = thin::edge_stiffener_utilization(f_o, sigma_cr, limit);
        let lambda_s = (f_o / sigma_cr.max(1e-9)).sqrt();
        let ok = |d: f64, ts: f64| thin::edge_stiffener_utilization(f_o, sigma_of(d, ts), limit) <= 1.0;
        let mut remedies = Vec::new();
        if u > 1.0 {
            if let Some(d) = grow_until(el.edge_stiffener_depth, 1.05, 120, |d| ok(d, el.edge_stiffener_thickness)) {
                remedies.push(Remedy::at_least(subject("edgeStiffenerDepth"), q_length(el.edge_stiffener_depth), q_length(d), loc("Deepen the edge stiffener.", "Randsteife höher machen.")));
            }
            if let Some(ts) = grow_until(el.edge_stiffener_thickness, 1.05, 120, |ts| ok(el.edge_stiffener_depth, ts)) {
                remedies.push(Remedy::at_least(subject("edgeStiffenerThickness"), q_length(el.edge_stiffener_thickness), q_length(ts), loc("Thicken the edge stiffener.", "Randsteife dicker machen.")));
            }
            if remedies.is_empty() {
                remedies.push(Remedy::at_least(subject("edgeStiffenerDepth"), q_length(el.edge_stiffener_depth), q_length(el.edge_stiffener_depth * u.max(1.0) * 1.05), loc("Deepen the edge stiffener.", "Randsteife höher machen.")));
            }
        }
        let (a_s, i_s) = thin::edge_stiffener_section(el.edge_stiffener_depth, el.edge_stiffener_thickness);
        let k = thin::edge_stiffener_spring(el.thickness, el.width);
        let text = format!("Lip {:.1} × {:.1} mm on the outstand {:.1} × {:.1} mm of member {}: K = {:.3e} N/m², A_s = {:.0} mm², I_s = {:.3e} mm⁴, σ_cr,s = 2·√(K·E·I_s)/A_s = {:.1} MPa, f_o = {:.0} MPa, λ̄_s = {:.3} against λ̄_s,max = {:.2} (registry row, mechanics threshold)", el.edge_stiffener_depth * 1e3, el.edge_stiffener_thickness * 1e3, el.width * 1e3, el.thickness * 1e3, member.id, k, a_s * 1e6, i_s * 1e12, sigma_cr / 1e6, f_o / 1e6, lambda_s, limit);
        let text_de = format!("Randsteife {:.1} × {:.1} mm am Kragarm {:.1} × {:.1} mm des Bauteils {}: K = {:.3e} N/m², A_s = {:.0} mm², I_s = {:.3e} mm⁴, σ_cr,s = 2·√(K·E·I_s)/A_s = {:.1} MPa, f_o = {:.0} MPa, λ̄_s = {:.3} gegen λ̄_s,max = {:.2} (Registerzeile, mechanische Schwelle)", el.edge_stiffener_depth * 1e3, el.edge_stiffener_thickness * 1e3, el.width * 1e3, el.thickness * 1e3, member.id, k, a_s * 1e6, i_s * 1e12, sigma_cr / 1e6, f_o / 1e6, lambda_s, limit);
        let mut b = CheckResult::assess(id, part, cl.clone(), subject("edgeStiffenerDepth"), title).utilization(q_dim(u), q_dim(1.0)).annex(annex).explanation(loc(&text, &text_de)).ndps(&[limit_id]);
        for r in remedies { b = b.remedy(r); }
        out.push(b.build());
    }
    out
}
//#endregion 🔖️ComplianceHelpers
