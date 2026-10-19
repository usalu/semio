//! 🧱️ EN 1999 member checks: the digests and the utilization helper behind `check_member`, which verifies one aluminium member against every clause.

use super::*;

#[path = "🧭️classification-and-axial-checks/🦀️.rs"]
mod classification_and_axial_checks;
use classification_and_axial_checks::{check_classification, check_tension_and_compression, check_combined_axial_bending};

#[path = "🔧️bending-and-shear-checks/🦀️.rs"]
mod bending_and_shear_checks;
use bending_and_shear_checks::{check_bending_y, check_shear_z, check_bending_z, check_shear_y};

#[path = "🏛️buckling-and-interaction-checks/🦀️.rs"]
mod buckling_and_interaction_checks;
use buckling_and_interaction_checks::{check_flexural_buckling, check_slenderness_limits, check_torsional_buckling, check_lateral_torsional_buckling, check_buckling_interaction};

#[path = "📉️serviceability-checks/🦀️.rs"]
mod serviceability_checks;
use serviceability_checks::{check_deflection, check_characteristic_stress, check_frequent_stress};

//#region 🔖️ComplianceHelpers
fn elements_digest(section: &AluminiumSection) -> String {
    if section.elements.is_empty() {
        return format!("kind={k};D={d:.4};t={t:.4}", k=section.kind, d=section.outer_diameter, t=section.web_thickness);
    }
    section.elements.iter().map(|e| format!(
        "{id}:out={out}:weld={w}:wp={wp:.5}:b={b:.5}:t={th:.5}",
        id=e.id, out=e.outstand, w=e.welded, wp=e.weld_position, b=e.width, th=e.thickness
    )).collect::<Vec<_>>().join("|")
}

pub(super) fn actions_digest(actions: &[semio_s_artifact_norm_en1999_model::MemberAction]) -> String {
    actions.iter().map(|a| format!(
        "{id}/{kind}/{cat}/{src}/g={g:.4}/q={q:.4}/N={n:.4}/Vy={vy:.4}/Vz={vz:.4}/My={my:.4}/Mz={mz:.4}",
        id=a.id, kind=a.kind, cat=a.category, src=a.source,
        g=a.g_k_line, q=a.q_k_line, n=a.n_k, vy=a.v_y_k, vz=a.v_z_k, my=a.m_y_k, mz=a.m_z_k
    )).collect::<Vec<_>>().join(" | ")
}

fn member_support_digest(member: &AluminiumMember) -> String {
    format!(
        "support={sup};c1={c1:.4};LcrY={ly:.4};LcrZ={lz:.4};LcrT={lt:.4};Llt={llt:.4};restrained={r}",
        sup=member.support.code(), c1=member.c1, ly=member.buckling_length_y, lz=member.buckling_length_z,
        lt=member.buckling_length_t, llt=member.ltb_length, r=member.restrained_ltb
    )
}

pub(super) fn member_label(id: &str) -> LocalizedCopy {
    loc(&format!("Member {id}"), &format!("Bauteil {id}"))
}

pub(super) fn conn_label(id: &str) -> LocalizedCopy {
    loc(&format!("Connection {id}"), &format!("Anschluss {id}"))
}

pub(super) fn utilization_check(
    id: String,
    part: &str,
    clause: ClauseId,
    subject: SubjectRef,
    title: LocalizedCopy,
    computed: Quantity,
    limit: Quantity,
    annex: AnnexChoice,
    explanation: LocalizedCopy,
    mut remedies: Vec<Remedy>,
) -> CheckResult {
    let u = if limit.value.abs() > 0.0 { computed.value.abs() / limit.value.abs() } else if computed.value.abs() > 0.0 { f64::INFINITY } else { 0.0 };
    if u > 1.0 && remedies.is_empty() && !subject.path.is_empty() {
        let required = if subject.path.contains("bucklingLength") || subject.path.contains("ltbLength") {
            Quantity::new(computed.kind, (computed.value.abs() / u.max(1.0) * 0.99).max(1e-6))
        } else {
            Quantity::new(computed.kind, computed.value * (1.0 / u.max(1.0)) * 0.99)
        };
        remedies.push(Remedy::at_most(
            subject.clone(),
            computed,
            required,
            loc("Reduce the governing demand until utilization ≤ 1.", "Maßgebende Beanspruchung senken, bis Ausnutzung ≤ 1."),
        ));
    }
    let mut b = CheckResult::assess(id, part, clause, subject, title)
        .utilization(computed, limit)
        .annex(annex)
        .explanation(explanation);
    for r in remedies { b = b.remedy(r); }
    b.build()
}

/// 📋️ Evaluate one member against EN 1999-1-1 cross-section and stability clauses.
pub fn check_member(
    member: &AluminiumMember,
    section: &AluminiumSection,
    material: &AluminiumMaterial,
    annex: AnnexChoice,
) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![CheckResult::assess(
            format!("en1999.3.2.alloy.{}", member.id),
            "DIN EN 1999-1-1",
            ClauseId::new("EN 1999-1-1", "§3.2", "3.2"),
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            loc("Alloy designation", "Legierungsbezeichnung"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}'.", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}'.", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(),
            loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen."),
        ))
        .build()];
    };
    let (a_gross, i_y, i_z, _wel_y, _wel_z, i_t, i_w) = part_1_1::section_geometry(section);
    let class = part_1_1::section_class(section, alloy.f_o_pa);
    let has_weld = section.elements.iter().any(|e| e.welded);
    let a_eff = part_1_1::effective_area(section, alloy.f_o_pa, has_weld, alloy.rho_o_haz, alloy.rho_u_haz);
    let wel_eff = part_1_1::effective_wel_y(section, alloy.f_o_pa, has_weld, alloy.rho_o_haz, alloy.rho_u_haz);
    let el_dig = elements_digest(section);
    let (alpha, lambda_0) = part_1_1::buckling_curve(alloy.buckling_class_a);
    let mut out = Vec::new();
    let part = "DIN EN 1999-1-1";
    let gov = part_en1990::governing_member_effects(member, annex);
    let action_id = gov.action_id.as_str();
    let (n_ed, v_y, v_z, m_y, m_z) = (gov.n_ed, gov.v_y_ed, gov.v_z_ed, gov.m_y_ed, gov.m_z_ed);
    let combo = gov.combination;
    let act_dig = actions_digest(&member.actions);
    let sup_dig = member_support_digest(member);
    check_classification(member, section, material, annex, &alloy, class, &el_dig, &mut out, part, action_id, combo);
    let path_n = format!("members[id={}].actions[id={}].nK", member.id, action_id);
    let path_m = format!("members[id={}].actions[id={}].mYK", member.id, action_id);
    let path_mz = format!("members[id={}].actions[id={}].mZK", member.id, action_id);
    let path_vy = format!("members[id={}].actions[id={}].vYK", member.id, action_id);
    let k_sup = match member.support {
        SupportCondition::Continuous => 0.7,
        SupportCondition::Cantilever => 2.0,
        SupportCondition::SimplySupported => 1.0,
    };
    let l_cr_y = k_sup * if member.buckling_length_y > 0.0 { member.buckling_length_y } else { member.length };
    let l_cr_z = k_sup * if member.buckling_length_z > 0.0 { member.buckling_length_z } else { member.length };
    let l_cr_t = k_sup * if member.buckling_length_t > 0.0 { member.buckling_length_t } else { member.length };
    let l_lt = if member.ltb_length > 0.0 { member.ltb_length } else { member.length };
    let n_rd = check_tension_and_compression(member, material, annex, &params, &alloy, a_eff, &el_dig, &mut out, part, action_id, n_ed, combo, &act_dig, &sup_dig, &path_n);
    let m_rd = check_bending_y(member, annex, &params, &alloy, wel_eff, &mut out, part, action_id, m_y, combo, &act_dig, &sup_dig, &path_m);
    let v_rd_b = check_shear_z(member, section, annex, &params, &alloy, &mut out, part, action_id, v_z, combo);
    let m_rd_z = check_bending_z(member, section, annex, &params, &alloy, &mut out, part, action_id, m_z, combo, &path_mz);
    check_shear_y(member, annex, &mut out, part, action_id, v_y, combo, &path_vy, v_rd_b);
    check_combined_axial_bending(member, annex, &mut out, part, action_id, m_y, m_z, n_ed, combo, &path_n, n_rd, m_rd, m_rd_z);
    let (lb_y, n_b_rd) = check_flexural_buckling(member, annex, &params, &alloy, a_gross, i_y, i_z, a_eff, alpha, lambda_0, &mut out, part, action_id, n_ed, combo, l_cr_y, l_cr_z);
    check_slenderness_limits(member, annex, &alloy, a_gross, i_y, i_z, a_eff, &mut out, part, action_id, combo, k_sup, l_cr_y, l_cr_z, l_cr_t, lb_y);
    check_torsional_buckling(member, annex, &params, &alloy, a_gross, i_t, i_y, i_z, a_eff, alpha, lambda_0, &mut out, part, action_id, n_ed, combo, &sup_dig, l_cr_t);
    let m_b_rd = check_lateral_torsional_buckling(member, annex, &alloy, i_t, i_w, i_z, wel_eff, alpha, lambda_0, &mut out, part, action_id, m_y, combo, &act_dig, &sup_dig, l_lt, m_rd);
    check_buckling_interaction(member, annex, &mut out, part, action_id, m_y, n_ed, combo, path_n, n_b_rd, m_b_rd);
    let sls_char = check_deflection(member, section, annex, &mut out, part, &act_dig, &sup_dig);
    let sigma_lim = check_characteristic_stress(member, annex, &alloy, wel_eff, &el_dig, &mut out, part, &act_dig, &sup_dig, &sls_char);
    check_frequent_stress(member, annex, wel_eff, &mut out, part, &act_dig, &sup_dig, sigma_lim);
    out
}
//#endregion 🔖️ComplianceHelpers
