//! 🧮️ EN 1992 design effects: EN 1990 combination of member actions and the governing effect.

use super::*;

/// 📦 Design effects after EN 1990 combination (SI).
#[derive(Clone, Debug)]
pub struct DesignEffects {
    pub m_ed: f64,
    pub n_ed: f64,
    pub v_ed: f64,
    pub t_ed: f64,
    pub v_ed_punch: f64,
    pub combination_id: String,
    pub situation: String,
}

pub(super) fn psi_factors(category: &str) -> (f64, f64, f64) {
    match category {
        "office" | "residential" | "congregation" => (0.7, 0.5, 0.3),
        "shopping" | "storage" => (0.7, 0.7, 0.6),
        "snow" | "snow_high" => (0.5, 0.2, 0.0),
        "wind" => (0.6, 0.2, 0.0),
        "temperature" => (0.6, 0.5, 0.0),
        "accidental" => (1.0, 1.0, 1.0),
        _ => (0.7, 0.5, 0.3),
    }
}

fn characteristic_effects(member: &RcMember, action: &LoadCaseActions) -> (f64, f64, f64, f64, f64) {
    if action.source == "external" || member.use_fem {
        return (action.m_k, action.n_k, action.v_k, action.t_k, action.v_k_punch);
    }
    let l = member.span.max(1e-6);
    let g_line = action.g_k_line + if action.kind == "permanent" { member.udl } else { 0.0 };
    let q_line = action.q_k_line;
    let q = match action.kind.as_str() {
        "permanent" | "prestress" => g_line,
        _ => q_line,
    };
    let (mut m, mut v) = match member.support {
        SupportCondition::SimplySupported => (q * l * l / 8.0, q * l / 2.0),
        SupportCondition::Cantilever => (q * l * l / 2.0, q * l),
        SupportCondition::Continuous | SupportCondition::Fixed => (q * l * l / 12.0, q * l / 2.0),
    };
    if action.point_force.abs() > 0.0 {
        let p = action.point_force.abs();
        m += p * l / 4.0;
        v += p / 2.0;
    }
    (m, 0.0, v, action.t_k, action.v_k_punch)
}

/// ⚖️ EN 1990 (+ DE NA) combinations — ULS 6.10a/b, accidental 6.11, SLS char/freq/qp.
pub fn combine_member_actions(member: &RcMember) -> Vec<DesignEffects> {
    let mut g = (0.0, 0.0, 0.0, 0.0, 0.0);
    let mut variables2: Vec<(&LoadCaseActions, (f64, f64, f64, f64, f64))> = Vec::new();
    let mut accidentals2 = Vec::new();
    let mut has_permanent = false;
    for a in &member.actions {
        let eff = characteristic_effects(member, a);
        match a.kind.as_str() {
            "permanent" | "prestress" => {
                has_permanent = true;
                g.0 += eff.0; g.1 += eff.1; g.2 += eff.2; g.3 += eff.3; g.4 += eff.4;
            }
            "accidental" => accidentals2.push((a, eff)),
            _ => variables2.push((a, eff)),
        }
    }
    if !has_permanent && member.udl > 0.0 && !member.use_fem {
        let l = member.span.max(1e-6);
        let q = member.udl;
        let (m, v) = match member.support {
            SupportCondition::SimplySupported => (q * l * l / 8.0, q * l / 2.0),
            SupportCondition::Cantilever => (q * l * l / 2.0, q * l),
            _ => (q * l * l / 12.0, q * l / 2.0),
        };
        g = (m, 0.0, v, 0.0, 0.0);
    }
    let gamma_g = 1.35;
    let gamma_q = 1.5;
    let mut out = Vec::new();
    if variables2.is_empty() {
        out.push(DesignEffects {
            m_ed: gamma_g * g.0, n_ed: gamma_g * g.1, v_ed: gamma_g * g.2, t_ed: gamma_g * g.3, v_ed_punch: gamma_g * g.4,
            combination_id: "ULS-6.10-G".into(), situation: "uls".into(),
        });
        out.push(DesignEffects {
            m_ed: g.0, n_ed: g.1, v_ed: g.2, t_ed: g.3, v_ed_punch: g.4,
            combination_id: "SLS-qp".into(), situation: "sls_qp".into(),
        });
        out.push(DesignEffects {
            m_ed: g.0, n_ed: g.1, v_ed: g.2, t_ed: g.3, v_ed_punch: g.4,
            combination_id: "SLS-char".into(), situation: "sls_char".into(),
        });
    } else {
        for (lead_i, (lead_a, lead_e)) in variables2.iter().enumerate() {
            let mut m = gamma_g * g.0 + gamma_q * lead_e.0;
            let mut n = gamma_g * g.1 + gamma_q * lead_e.1;
            let mut v = gamma_g * g.2 + gamma_q * lead_e.2;
            let mut t = gamma_g * g.3 + gamma_q * lead_e.3;
            let mut vp = gamma_g * g.4 + gamma_q * lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(&oa.category);
                m += gamma_q * psi0 * oe.0; n += gamma_q * psi0 * oe.1; v += gamma_q * psi0 * oe.2;
                t += gamma_q * psi0 * oe.3; vp += gamma_q * psi0 * oe.4;
            }
            let xi = 0.85;
            let mut m_b = xi * gamma_g * g.0 + gamma_q * lead_e.0;
            let mut n_b = xi * gamma_g * g.1 + gamma_q * lead_e.1;
            let mut v_b = xi * gamma_g * g.2 + gamma_q * lead_e.2;
            let mut t_b = xi * gamma_g * g.3 + gamma_q * lead_e.3;
            let mut vp_b = xi * gamma_g * g.4 + gamma_q * lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(&oa.category);
                m_b += gamma_q * psi0 * oe.0; n_b += gamma_q * psi0 * oe.1; v_b += gamma_q * psi0 * oe.2;
                t_b += gamma_q * psi0 * oe.3; vp_b += gamma_q * psi0 * oe.4;
            }
            let (mu, nu, vu, tu, vpu, tag) = if m.abs() >= m_b.abs() {
                (m, n, v, t, vp, "6.10a")
            } else {
                (m_b, n_b, v_b, t_b, vp_b, "6.10b")
            };
            out.push(DesignEffects { m_ed: mu, n_ed: nu, v_ed: vu, t_ed: tu, v_ed_punch: vpu,
                combination_id: format!("ULS-{}-{}", tag, lead_a.id), situation: "uls".into() });
            let mut ms = g.0 + lead_e.0;
            let mut ns = g.1 + lead_e.1;
            let mut vs = g.2 + lead_e.2;
            let mut ts = g.3 + lead_e.3;
            let mut vps = g.4 + lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(&oa.category);
                ms += psi0 * oe.0; ns += psi0 * oe.1; vs += psi0 * oe.2; ts += psi0 * oe.3; vps += psi0 * oe.4;
            }
            out.push(DesignEffects { m_ed: ms, n_ed: ns, v_ed: vs, t_ed: ts, v_ed_punch: vps,
                combination_id: format!("SLS-char-{}", lead_a.id), situation: "sls_char".into() });
            let (_, psi1_l, _) = psi_factors(&lead_a.category);
            let mut mf = g.0 + psi1_l * lead_e.0;
            let mut nf = g.1 + psi1_l * lead_e.1;
            let mut vf = g.2 + psi1_l * lead_e.2;
            let mut tf = g.3 + psi1_l * lead_e.3;
            let mut vpf = g.4 + psi1_l * lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (_, _, psi2) = psi_factors(&oa.category);
                mf += psi2 * oe.0; nf += psi2 * oe.1; vf += psi2 * oe.2; tf += psi2 * oe.3; vpf += psi2 * oe.4;
            }
            out.push(DesignEffects { m_ed: mf, n_ed: nf, v_ed: vf, t_ed: tf, v_ed_punch: vpf,
                combination_id: format!("SLS-freq-{}", lead_a.id), situation: "sls_freq".into() });
        }
        let mut mq = g.0; let mut nq = g.1; let mut vq = g.2; let mut tq = g.3; let mut vpq = g.4;
        for (oa, oe) in &variables2 {
            let (_, _, psi2) = psi_factors(&oa.category);
            mq += psi2 * oe.0; nq += psi2 * oe.1; vq += psi2 * oe.2; tq += psi2 * oe.3; vpq += psi2 * oe.4;
        }
        out.push(DesignEffects { m_ed: mq, n_ed: nq, v_ed: vq, t_ed: tq, v_ed_punch: vpq,
            combination_id: "SLS-qp".into(), situation: "sls_qp".into() });
    }
    for (aa, ae) in &accidentals2 {
        let mut m = g.0 + ae.0; let mut n = g.1 + ae.1; let mut v = g.2 + ae.2;
        let mut tt = g.3 + ae.3; let mut vp = g.4 + ae.4;
        for (oa, oe) in &variables2 {
            let (_, psi1, _) = psi_factors(&oa.category);
            m += psi1 * oe.0; n += psi1 * oe.1; v += psi1 * oe.2; tt += psi1 * oe.3; vp += psi1 * oe.4;
        }
        out.push(DesignEffects { m_ed: m, n_ed: n, v_ed: v, t_ed: tt, v_ed_punch: vp,
            combination_id: format!("ACC-6.11-{}", aa.id), situation: "accidental".into() });
    }
    out
}

pub(super) fn governing<'a>(effects: &'a [DesignEffects], sit: &str) -> Option<&'a DesignEffects> {
    effects.iter().filter(|e| e.situation == sit).max_by(|a, b| {
        a.m_ed.abs().partial_cmp(&b.m_ed.abs()).unwrap_or(std::cmp::Ordering::Equal)
    })
}
