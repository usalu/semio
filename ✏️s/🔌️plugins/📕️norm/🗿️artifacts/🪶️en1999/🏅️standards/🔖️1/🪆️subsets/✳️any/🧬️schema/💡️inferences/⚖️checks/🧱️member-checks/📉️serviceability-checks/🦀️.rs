//! 📉️ Serviceability checks: the quasi permanent deflection and the characteristic and frequent stresses of the member checks. See [`super`].

use super::*;

/// 📉 Checks the quasi permanent deflection against the span limit.
pub(super) fn check_deflection(
    member: &AluminiumMember,
    section: &AluminiumSection,
    annex: AnnexChoice,
    out: &mut Vec<CheckResult>,
    part: &str,
    act_dig: &str,
    sup_dig: &str,
) -> part_en1990::GoverningEffects {
    let sls_qp = part_en1990::sls_member_effects(member, annex, "quasiPermanent");
    let sls_char = part_en1990::sls_member_effects(member, annex, "characteristic");
    let (_, iy, _, _, _, _, _) = part_1_1::section_geometry(section);
    let e_mod = na_de::E_PA;
    let w_qp = if e_mod * iy > 0.0 && member.length > 0.0 {
        let m_qp = sls_qp.m_y_ed.abs();
        let q = 8.0 * m_qp / member.length.powi(2);
        5.0 * q * member.length.powi(4) / (384.0 * e_mod * iy)
    } else { 0.0 };
    let w_lim = member.length / 200.0;
    let mut rem_w = Vec::new();
    if w_qp > w_lim {
        if member.length <= 0.0 {
            rem_w.push(Remedy::at_least(
                SubjectRef::new(&member.id, format!("members[id={}].length", member.id), member_label(&member.id)),
                q_length(member.length),
                q_length(1.0),
                loc("Set a positive member length for SLS deflection.", "Positive Bauteillänge für GZG-Durchbiegung setzen."),
            ));
        } else if w_lim > 0.0 {
            rem_w.extend(scale_action_leaf_remedies(
                "members", &member.id, &member.actions, "mYK", |a| a.m_y_k, sls_qp.m_y_ed, sls_qp.m_y_ed.abs() * w_lim / w_qp.max(1e-12), q_moment,
                member_label(&member.id),
                "Reduce M_y,k so quasi-permanent deflection ≤ L/200.",
                "M_y,k senken, damit quasi-ständige Durchbiegung ≤ L/200.",
            ));
            rem_w.extend(scale_action_leaf_remedies(
                "members", &member.id, &member.actions, "gKLine", |a| a.g_k_line, w_qp, w_lim, q_force,
                member_label(&member.id),
                "Reduce g_k so quasi-permanent deflection ≤ L/200.",
                "g_k senken, damit quasi-ständige Durchbiegung ≤ L/200.",
            ));
            rem_w.extend(scale_action_leaf_remedies(
                "members", &member.id, &member.actions, "qKLine", |a| a.q_k_line, w_qp, w_lim, q_force,
                member_label(&member.id),
                "Reduce q_k so quasi-permanent deflection ≤ L/200.",
                "q_k senken, damit quasi-ständige Durchbiegung ≤ L/200.",
            ));
        }
    }
    out.push(utilization_check(
        format!("en1999.7.2.defl.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].length", member.id), member_label(&member.id)),
        loc("SLS deflection (quasi-permanent)", "GZG Durchbiegung (quasi-ständig)"),
        q_length(w_qp),
        q_length(w_lim.max(1e-9)),
        annex,
        loc(
            &format!("w={:.1} mm ≤ L/200={:.1} mm (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}", w_qp*1e3, w_lim*1e3, comb=sls_qp.combination, aid=sls_qp.action_id),
            &format!("w={:.1} mm ≤ L/200={:.1} mm (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}", w_qp*1e3, w_lim*1e3, comb=sls_qp.combination, aid=sls_qp.action_id),
        ),
        rem_w,
    ));
    sls_char
}

/// 🧪 Checks the elastic stress of the characteristic combination.
pub(super) fn check_characteristic_stress(
    member: &AluminiumMember,
    annex: AnnexChoice,
    alloy: &part_1_1::AlloyProps,
    wel_eff: f64,
    el_dig: &str,
    out: &mut Vec<CheckResult>,
    part: &str,
    act_dig: &str,
    sup_dig: &str,
    sls_char: &part_en1990::GoverningEffects,
) -> f64 {
    let sigma_sls = if wel_eff > 0.0 { sls_char.m_y_ed.abs() / wel_eff } else { 0.0 };
    let sigma_lim = 0.8 * alloy.f_o_pa;
    let mut rem_s = Vec::new();
    if sigma_sls > sigma_lim {
        let m_lim = if wel_eff > 0.0 { sigma_lim * wel_eff } else { 0.0 };
        rem_s.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "mYK", |a| a.m_y_k, sls_char.m_y_ed, m_lim, q_moment,
            member_label(&member.id),
            "Reduce M_y,k so characteristic SLS σ ≤ 0.8 f_o.",
            "M_y,k senken, damit charakteristische GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_s.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "gKLine", |a| a.g_k_line, sigma_sls, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce g_k so characteristic SLS σ ≤ 0.8 f_o.",
            "g_k senken, damit charakteristische GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_s.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "qKLine", |a| a.q_k_line, sigma_sls, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce q_k so characteristic SLS σ ≤ 0.8 f_o.",
            "q_k senken, damit charakteristische GZG-Spannung σ ≤ 0,8 f_o.",
        ));
    }
    out.push(utilization_check(
        format!("en1999.7.2.stress.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, sls_char.action_id), member_label(&member.id)),
        loc("SLS elastic stress (characteristic)", "GZG elastische Spannung (charakteristisch)"),
        q_stress(sigma_sls),
        q_stress(sigma_lim.max(1e-9)),
        annex,
        loc(
            &format!("σ={:.0} MPa ≤ 0.8 f_o={:.0} MPa (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}; {el_dig}", sigma_sls/1e6, sigma_lim/1e6, comb=sls_char.combination, aid=sls_char.action_id),
            &format!("σ={:.0} MPa ≤ 0,8 f_o={:.0} MPa (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}; {el_dig}", sigma_sls/1e6, sigma_lim/1e6, comb=sls_char.combination, aid=sls_char.action_id),
        ),
        rem_s,
    ));
    sigma_lim
}

/// 🔁 Checks the elastic stress of the frequent combination.
pub(super) fn check_frequent_stress(
    member: &AluminiumMember,
    annex: AnnexChoice,
    wel_eff: f64,
    out: &mut Vec<CheckResult>,
    part: &str,
    act_dig: &str,
    sup_dig: &str,
    sigma_lim: f64,
) {
    let sls_freq = part_en1990::sls_member_effects(member, annex, "frequent");
    let sigma_freq = if wel_eff > 0.0 { sls_freq.m_y_ed.abs() / wel_eff } else { 0.0 };
    let mut rem_f = Vec::new();
    if sigma_freq > sigma_lim {
        let m_lim = if wel_eff > 0.0 { sigma_lim * wel_eff } else { 0.0 };
        rem_f.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "mYK", |a| a.m_y_k, sls_freq.m_y_ed, m_lim, q_moment,
            member_label(&member.id),
            "Reduce M_y,k so frequent SLS σ ≤ 0.8 f_o.",
            "M_y,k senken, damit häufige GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_f.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "gKLine", |a| a.g_k_line, sigma_freq, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce g_k so frequent SLS σ ≤ 0.8 f_o.",
            "g_k senken, damit häufige GZG-Spannung σ ≤ 0,8 f_o.",
        ));
        rem_f.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "qKLine", |a| a.q_k_line, sigma_freq, sigma_lim, q_force,
            member_label(&member.id),
            "Reduce q_k so frequent SLS σ ≤ 0.8 f_o.",
            "q_k senken, damit häufige GZG-Spannung σ ≤ 0,8 f_o.",
        ));
    }
    out.push(utilization_check(
        format!("en1999.7.2.sls-freq.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§7.2", "7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, sls_freq.action_id), member_label(&member.id)),
        loc("SLS elastic stress (frequent ψ₁)", "GZG elastische Spannung (häufig ψ₁)"),
        q_stress(sigma_freq),
        q_stress(sigma_lim.max(1e-9)),
        annex,
        loc(
            &format!("σ={:.0} MPa ≤ 0.8 f_o={:.0} MPa (SLS {comb}, lead {aid}); {act_dig}; {sup_dig}", sigma_freq/1e6, sigma_lim/1e6, comb=sls_freq.combination, aid=sls_freq.action_id),
            &format!("σ={:.0} MPa ≤ 0,8 f_o={:.0} MPa (GZG {comb}, führend {aid}); {act_dig}; {sup_dig}", sigma_freq/1e6, sigma_lim/1e6, comb=sls_freq.combination, aid=sls_freq.action_id),
        ),
        rem_f,
    ));
}
