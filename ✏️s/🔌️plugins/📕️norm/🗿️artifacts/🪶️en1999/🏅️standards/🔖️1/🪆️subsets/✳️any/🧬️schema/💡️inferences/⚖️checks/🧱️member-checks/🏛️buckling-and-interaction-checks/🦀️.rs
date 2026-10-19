//! 🏛️ Buckling and interaction checks: the flexural, torsional and lateral torsional buckling and the interaction of the member checks. See [`super`].

use super::*;

/// 🏛 Checks the flexural buckling resistance about both axes.
pub(super) fn check_flexural_buckling(
    member: &AluminiumMember,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    a_gross: f64,
    i_y: f64,
    i_z: f64,
    a_eff: f64,
    alpha: f64,
    lambda_0: f64,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    n_ed: f64,
    combo: &'static str,
    l_cr_y: f64,
    l_cr_z: f64,
) -> (f64, f64) {
    let lb_y = part_1_1::lambda_bar(l_cr_y, i_y, a_eff.max(a_gross), alloy.f_o_pa);
    let lb_z = part_1_1::lambda_bar(l_cr_z, i_z, a_eff.max(a_gross), alloy.f_o_pa);
    let chi_y = part_1_1::chi_from_lambda(lb_y, alpha, lambda_0);
    let chi_z = part_1_1::chi_from_lambda(lb_z, alpha, lambda_0);
    let chi = chi_y.min(chi_z);
    let n_b_rd_y = chi_y * a_eff * alloy.f_o_pa / params.gamma_m1;
    let n_b_rd_z = chi_z * a_eff * alloy.f_o_pa / params.gamma_m1;
    let n_b_rd = chi * a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_b = Vec::new();
    if n_ed.abs() > n_b_rd && n_b_rd > 0.0 {
        let chi_req = (n_ed.abs() * params.gamma_m1 / (a_eff * alloy.f_o_pa)).min(1.0);
        let mut lo = 0.1; let mut hi = member.buckling_length_y.max(member.buckling_length_z).max(0.5);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let lb = part_1_1::lambda_bar(mid, i_y.min(i_z), a_eff.max(a_gross), alloy.f_o_pa);
            let ch = part_1_1::chi_from_lambda(lb, alpha, lambda_0);
            if ch >= chi_req { lo = mid; } else { hi = mid; }
        }
        remedies_b.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(lo),
            loc(
                &format!("Reduce buckling length L_cr,y from {:.2} m to at most {:.2} m (χ computed = {chi:.3}).", member.buckling_length_y, lo),
                &format!("Biegedrillknicklänge L_cr,y von {:.2} m auf höchstens {:.2} m verkürzen (χ = {chi:.3}).", member.buckling_length_y, lo),
            ),
        ));
        remedies_b.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a stockier section to raise χ·A.", "Stockigeren Querschnitt wählen, um χ·A zu erhöhen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.fb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
        loc("Flexural buckling N_b,Rd (χ computed)", "Biegeknicken N_b,Rd (χ berechnet)"),
        q_force(n_ed.abs()),
        q_force(n_b_rd_y),
        annex,
        loc(
            &format!("λ̄_y={lb_y:.6}, χ_y={chi_y:.6}, N_b,Rd,y={:.1} kN (ULS {combo}, lead {action_id}).", n_b_rd_y/1e3),
            &format!("λ̄_y={lb_y:.6}, χ_y={chi_y:.6}, N_b,Rd,y={:.1} kN (GZT {combo}, führend {action_id}).", n_b_rd_y/1e3),
        ),
        remedies_b,
    ));
    let mut remedies_fb_z = Vec::new();
    if n_ed.abs() > n_b_rd_z && n_b_rd_z > 0.0 {
        let chi_req_z = (n_ed.abs() * params.gamma_m1 / (a_eff * alloy.f_o_pa)).min(1.0);
        let mut lo_z = 0.1;
        let mut hi_z = member.buckling_length_z.max(0.5);
        for _ in 0..40 {
            let mid = 0.5 * (lo_z + hi_z);
            let lb = part_1_1::lambda_bar(mid, i_z, a_eff.max(a_gross), alloy.f_o_pa);
            let ch = part_1_1::chi_from_lambda(lb, alpha, lambda_0);
            if ch >= chi_req_z { lo_z = mid; } else { hi_z = mid; }
        }
        remedies_fb_z.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthZ", member.id), member_label(&member.id)),
            q_length(member.buckling_length_z),
            q_length(lo_z.max(0.05)),
            loc(
                &format!("Reduce buckling length L_cr,z to at most {:.2} m.", lo_z.max(0.05)),
                &format!("Knicklänge L_cr,z auf höchstens {:.2} m verkürzen.", lo_z.max(0.05)),
            ),
        ));
        remedies_fb_z.extend(scale_action_leaf_remedies(
            "members", &member.id, &member.actions, "nK", |a| a.n_k, n_ed, n_b_rd_z, q_force,
            member_label(&member.id),
            "Reduce |N_k| so flexural buckling about z complies.",
            "|N_k| senken, damit Biegeknicken um z erfüllt ist.",
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.fb.z.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthZ", member.id), member_label(&member.id)),
        loc("Flexural buckling N_b,Rd,z", "Biegeknicken N_b,Rd,z"),
        q_force(n_ed.abs()),
        q_force(n_b_rd_z),
        annex,
        loc(
            &format!("λ̄_z={lb_z:.6}, χ_z={chi_z:.6}, N_b,Rd,z={:.1} kN (ULS {combo}, lead {action_id}).", n_b_rd_z/1e3),
            &format!("λ̄_z={lb_z:.6}, χ_z={chi_z:.6}, N_b,Rd,z={:.1} kN (GZT {combo}, führend {action_id}).", n_b_rd_z/1e3),
        ),
        remedies_fb_z,
    ));
    (lb_y, n_b_rd)
}

/// 📏 Checks the slenderness about the strong axis and the torsional slenderness and reports the critical lengths.
pub(super) fn check_slenderness_limits(
    member: &AluminiumMember,
    annex: AnnexChoice,
    alloy: &part_1_1::AlloyProps,
    a_gross: f64,
    i_y: f64,
    i_z: f64,
    a_eff: f64,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    combo: &'static str,
    k_sup: f64,
    l_cr_y: f64,
    l_cr_z: f64,
    l_cr_t: f64,
    lb_y: f64,
) {
    let lb_t_slend = part_1_1::lambda_bar(l_cr_t, i_y.min(i_z), a_eff.max(a_gross), alloy.f_o_pa);
    let mut remedies_lam_y = Vec::new();
    if lb_y > 3.0 {
        let l_req = (member.buckling_length_y * 3.0 / lb_y * 0.99).max(0.05);
        remedies_lam_y.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(l_req),
            loc(
                &format!("Reduce L_cr,y to at most {:.2} m so λ̄_y ≤ 3.", l_req),
                &format!("L_cr,y auf höchstens {:.2} m verkürzen, damit λ̄_y ≤ 3.", l_req),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.lambda.y.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
        loc("Non-dimensional slenderness λ̄_y", "Bezogene Schlankheit λ̄_y"),
        q_dim(lb_y),
        q_dim(3.0),
        annex,
        loc(
            &format!("λ̄_y={lb_y:.6} from L_cr,y (ULS {combo}, lead {action_id})."),
            &format!("λ̄_y={lb_y:.6} aus L_cr,y (GZT {combo}, führend {action_id})."),
        ),
        remedies_lam_y,
    ));
    let mut remedies_lam_t = Vec::new();
    if lb_t_slend > 3.0 {
        let l_req = (member.buckling_length_t * 3.0 / lb_t_slend * 0.99).max(0.05);
        remedies_lam_t.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
            q_length(member.buckling_length_t),
            q_length(l_req),
            loc(
                &format!("Reduce L_cr,T to at most {:.2} m so λ̄_T ≤ 3.", l_req),
                &format!("L_cr,T auf höchstens {:.2} m verkürzen, damit λ̄_T ≤ 3.", l_req),
            ),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.lambda.t.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1.4"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
        loc("Non-dimensional slenderness λ̄_T", "Bezogene Schlankheit λ̄_T"),
        q_dim(lb_t_slend),
        q_dim(3.0),
        annex,
        loc(
            &format!("λ̄_T from L_cr,T={l_cr_t:.4} m (ULS {combo}, lead {action_id})."),
            &format!("λ̄_T aus L_cr,T={l_cr_t:.4} m (GZT {combo}, führend {action_id})."),
        ),
        remedies_lam_t,
    ));
    out.push(utilization_check(
        format!("en1999.6.3.1.lcr.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
        loc("Buckling lengths L_cr,y/z/t", "Knicklängen L_cr,y/z/t"),
        q_length(l_cr_y + l_cr_z + l_cr_t),
        q_length(((l_cr_y + l_cr_z + l_cr_t) * 2.0).max(1e-3)),
        annex,
        loc(
            &format!("ΣL_cr={:.3} m (y={:.3}, z={:.3}, t={:.3}); L={:.3} m; support k={k_sup:.2} (ULS {combo}, lead {action_id}).", l_cr_y + l_cr_z + l_cr_t, l_cr_y, l_cr_z, l_cr_t, member.length),
            &format!("ΣL_cr={:.3} m (y={:.3}, z={:.3}, t={:.3}); L={:.3} m; Auflager k={k_sup:.2} (GZT {combo}, führend {action_id}).", l_cr_y + l_cr_z + l_cr_t, l_cr_y, l_cr_z, l_cr_t, member.length),
        ),
        Vec::new(),
    ));
}

/// 🌀 Checks the torsional and flexural torsional buckling resistance.
pub(super) fn check_torsional_buckling(
    member: &AluminiumMember,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    a_gross: f64,
    i_t: f64,
    i_y: f64,
    i_z: f64,
    a_eff: f64,
    alpha: f64,
    lambda_0: f64,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    n_ed: f64,
    combo: &'static str,
    sup_dig: &str,
    l_cr_t: f64,
) {
    let lb_t = part_1_1::torsional_lambda_bar(a_eff.max(a_gross), i_t, i_y, i_z, l_cr_t, alloy.f_o_pa);
    let chi_t = part_1_1::chi_from_lambda(lb_t, alpha, lambda_0);
    let n_t_rd = chi_t * a_eff * alloy.f_o_pa / params.gamma_m1;
    let mut remedies_t = Vec::new();
    if n_ed.abs() > n_t_rd && n_t_rd > 0.0 {
        remedies_t.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-box120".into(), "sec-tube114".into()],
            loc("Select a closed section to raise I_t.", "Geschlossenen Querschnitt wählen, um I_t zu erhöhen."),
        ));
        remedies_t.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
            q_length(l_cr_t),
            q_length(l_cr_t * 0.5),
            loc("Halve torsional buckling length L_cr,T.", "Torsionsknicklänge L_cr,T halbieren."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.1.tb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.1", "6.3.1.4"),
        SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), member_label(&member.id)),
        loc("Torsional / flexural-torsional buckling", "Torsions- / Biegedrillknicken"),
        q_force(n_ed.abs()),
        q_force(n_t_rd),
        annex,
        loc(
            &format!("L_cr,T={l_cr_t:.4} m, λ̄_T={lb_t:.6}, χ_T={chi_t:.6}, N_t,Rd={:.1} kN (ULS {combo}, lead {action_id}); {sup_dig}", n_t_rd/1e3),
            &format!("L_cr,T={l_cr_t:.4} m, λ̄_T={lb_t:.6}, χ_T={chi_t:.6}, N_t,Rd={:.1} kN (GZT {combo}, führend {action_id}); {sup_dig}", n_t_rd/1e3),
        ),
        remedies_t,
    ));
}

/// 🦋 Checks the lateral torsional buckling resistance.
pub(super) fn check_lateral_torsional_buckling(
    member: &AluminiumMember,
    annex: AnnexChoice,
    alloy: &part_1_1::AlloyProps,
    i_t: f64,
    i_w: f64,
    i_z: f64,
    wel_eff: f64,
    alpha: f64,
    lambda_0: f64,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    m_y: f64,
    combo: &'static str,
    act_dig: &str,
    sup_dig: &str,
    l_lt: f64,
    m_rd: f64,
) -> f64 {
    let m_cr = if member.restrained_ltb {
        f64::INFINITY
    } else {
        part_1_1::m_cr_ltb(member.c1, na_de::E_PA, i_z, i_w, i_t, l_lt)
    };
    let lambda_lt = if m_cr.is_finite() && m_cr > 0.0 { (wel_eff * alloy.f_o_pa / m_cr).sqrt() } else { 0.0 };
    let chi_lt = part_1_1::chi_from_lambda(lambda_lt, alpha, lambda_0);
    let m_b_rd = chi_lt * m_rd;
    let mut remedies_lt = Vec::new();
    if m_y.abs() > m_b_rd && m_b_rd > 0.0 {
        remedies_lt.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].ltbLength", member.id), member_label(&member.id)),
            q_length(l_lt),
            q_length(l_lt * 0.5),
            loc(
                &format!("Reduce LTB length from {:.2} m (M_cr={:.1} kNm, χ_LT={chi_lt:.3}).", l_lt, m_cr/1e3),
                &format!("Biegedrillknicklänge von {:.2} m verkürzen (M_cr={:.1} kNm, χ_LT={chi_lt:.3}).", l_lt, m_cr/1e3),
            ),
        ));
        remedies_lt.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a section with larger I_z / I_w.", "Querschnitt mit größerem I_z / I_w wählen."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.2.ltb.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.2", "6.3.2"),
        SubjectRef::new(&member.id, format!("members[id={}].ltbLength", member.id), member_label(&member.id)),
        loc("Lateral-torsional buckling M_b,Rd", "Biegedrillknicken M_b,Rd"),
        q_moment(m_y.abs()),
        q_moment(m_b_rd),
        annex,
        loc(
            &format!("M_cr={:.1} kNm, λ̄_LT={lambda_lt:.6}, χ_LT={chi_lt:.6}, M_b,Rd={:.2} kNm (ULS {combo}, lead {action_id}); {sup_dig}; {act_dig}", m_cr/1e3, m_b_rd/1e3),
            &format!("M_cr={:.1} kNm, λ̄_LT={lambda_lt:.6}, χ_LT={chi_lt:.6}, M_b,Rd={:.2} kNm (GZT {combo}, führend {action_id}); {sup_dig}; {act_dig}", m_cr/1e3, m_b_rd/1e3),
        ),
        remedies_lt,
    ));
    m_b_rd
}

/// 🧲 Checks the interaction of compression and bending buckling.
pub(super) fn check_buckling_interaction(
    member: &AluminiumMember,
    annex: AnnexChoice,
    out: &mut Vec<CheckResult>,
    part: &str,
    action_id: &str,
    m_y: f64,
    n_ed: f64,
    combo: &'static str,
    path_n: String,
    n_b_rd: f64,
    m_b_rd: f64,
) {
    let u_i = if n_b_rd > 0.0 && m_b_rd > 0.0 {
        n_ed.abs() / n_b_rd + m_y.abs() / m_b_rd
    } else { 0.0 };
    let mut remedies_i = Vec::new();
    if u_i > 1.0 {
        remedies_i.push(Remedy::one_of(
            SubjectRef::new(&member.id, format!("members[id={}].sectionId", member.id), member_label(&member.id)),
            vec!["sec-i160".into(), "sec-i200".into()],
            loc("Select a larger section for member buckling interaction.", "Größeren Querschnitt für Knickinteraktion wählen."),
        ));
        remedies_i.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthY", member.id), member_label(&member.id)),
            q_length(member.buckling_length_y),
            q_length(member.buckling_length_y * 0.6),
            loc("Shorten buckling / LTB lengths (add restraints).", "Knick-/Biegedrillknicklängen verkürzen (Halterungen)."),
        ));
    }
    out.push(utilization_check(
        format!("en1999.6.3.3.int.{}", member.id),
        part,
        ClauseId::new("EN 1999-1-1", "§6.3.3", "6.3.3"),
        SubjectRef::new(&member.id, path_n, member_label(&member.id)),
        loc("Member buckling interaction", "Interaktion Stabknicken"),
        q_dim(u_i),
        q_dim(1.0),
        annex,
        loc(
            &format!("N_Ed/N_b,Rd + M_Ed/M_b,Rd = {u_i:.3} (ULS {combo}, lead {action_id})."),
            &format!("N_Ed/N_b,Rd + M_Ed/M_b,Rd = {u_i:.3} (GZT {combo}, führend {action_id})."),
        ),
        remedies_i,
    ));
}
