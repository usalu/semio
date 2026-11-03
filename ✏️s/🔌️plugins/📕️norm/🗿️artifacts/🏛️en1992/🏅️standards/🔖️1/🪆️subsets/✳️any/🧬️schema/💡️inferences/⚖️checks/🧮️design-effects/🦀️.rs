//! 🧮️ EN 1992 design effects: EN 1990 combination of member actions and the governing effect.

use super::*;
use semio_s_plugin_norm_compliance::{ActionFactors, DesignSituation};

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

/// 🗝️ The registry key `ndp.en1990.psi.<key>` of a ψ category; the accidental category has no row and carries (1, 1, 1).
pub fn psi_key(category: semio_s_artifact_norm_en1992_model::PsiCategory) -> &'static str {
    use semio_s_artifact_norm_en1992_model::PsiCategory as C;
    match category {
        C::Office | C::SelfWeight => "office",
        C::Residential => "residential",
        C::Congregation => "congregation",
        C::Shopping => "retail",
        C::Storage => "storage",
        C::Snow => "snow",
        C::SnowHigh => "snow_high",
        C::Wind => "wind",
        C::Temperature => "temperature",
        C::Accidental => "",
    }
}

/// 📊 ψ0, ψ1, ψ2 of a category from the registry rows of the annex of `factors`.
pub fn psi_factors(factors: &ActionFactors, category: semio_s_artifact_norm_en1992_model::PsiCategory) -> (f64, f64, f64) {
    factors.psi(psi_key(category)).as_tuple()
}

type Eff = (f64, f64, f64, f64, f64);

fn line_effects(support: SupportCondition, span: f64, q: f64, p: f64) -> (f64, f64) {
    let l = span.max(1e-6);
    let (mq, vq) = match support {
        SupportCondition::SimplySupported => (q * l * l / 8.0, q * l / 2.0),
        SupportCondition::Cantilever => (q * l * l / 2.0, q * l),
        SupportCondition::Continuous | SupportCondition::Fixed => (q * l * l / 12.0, q * l / 2.0),
    };
    let (mp, vp) = match support {
        SupportCondition::SimplySupported => (p * l / 4.0, p / 2.0),
        SupportCondition::Cantilever => (p * l, p),
        SupportCondition::Continuous | SupportCondition::Fixed => (p * l / 8.0, p / 2.0),
    };
    (mq + mp, vq + vp)
}

fn characteristic_effects(member: &RcMember, action: &LoadCaseActions) -> Eff {
    if action.source == semio_s_artifact_norm_en1992_model::ActionSource::External || member.use_fem {
        return (action.m_k, action.n_k, action.v_k, action.t_k, action.v_k_punch);
    }
    let q = match action.kind {
        semio_s_artifact_norm_en1992_model::ActionKind::Permanent | semio_s_artifact_norm_en1992_model::ActionKind::Prestress => action.g_k_line,
        _ => action.q_k_line,
    };
    let (m, v) = line_effects(member.support, member.span, q, action.point_force.abs());
    (m, action.n_k, v, action.t_k, action.v_k_punch)
}

fn add(a: Eff, f: f64, b: Eff) -> Eff {
    (a.0 + f * b.0, a.1 + f * b.1, a.2 + f * b.2, a.3 + f * b.3, a.4 + f * b.4)
}

fn make(e: Eff, id: String, situation: &str) -> DesignEffects {
    DesignEffects { m_ed: e.0, n_ed: e.1, v_ed: e.2, t_ed: e.3, v_ed_punch: e.4, combination_id: id, situation: situation.into() }
}

/// ⚖️ EN 1990 (+ DE NA) combinations — ULS 6.10a/b with unfavourable and favourable permanent actions, γ_P = 1.0 for prestress, accidental 6.11b with ψ_1,1 for the leading and ψ_2,i for the other variables, SLS characteristic, frequent and quasi-permanent.
pub fn combine_member_actions(member: &RcMember, factors: &ActionFactors) -> Vec<DesignEffects> {
    let factors = &factors.in_situation(DesignSituation::Persistent);
    let zero: Eff = (0.0, 0.0, 0.0, 0.0, 0.0);
    let mut g = zero;
    let mut pre = zero;
    let mut variables: Vec<(&LoadCaseActions, Eff)> = Vec::new();
    let mut accidentals: Vec<(&LoadCaseActions, Eff)> = Vec::new();
    for a in &member.actions {
        let eff = characteristic_effects(member, a);
        match a.kind {
            semio_s_artifact_norm_en1992_model::ActionKind::Permanent => g = add(g, 1.0, eff),
            semio_s_artifact_norm_en1992_model::ActionKind::Prestress => pre = add(pre, 1.0, eff),
            semio_s_artifact_norm_en1992_model::ActionKind::Accidental => accidentals.push((a, eff)),
            _ => variables.push((a, eff)),
        }
    }
    if member.udl != 0.0 && !member.use_fem {
        let (m, v) = line_effects(member.support, member.span, member.udl, 0.0);
        g = add(g, 1.0, (m, 0.0, v, 0.0, 0.0));
    }
    let (gamma_g, gamma_g_inf, gamma_q, xi) = (factors.gamma_g_sup(), factors.gamma_g_inf(), factors.gamma_q_building(), factors.xi());
    let mut out = Vec::new();
    let others = |lead: usize, f: &dyn Fn(&(f64, f64, f64)) -> f64, base: Eff| -> Eff {
        variables.iter().enumerate().filter(|(j, _)| *j != lead).fold(base, |acc, (_, (a, e))| add(acc, f(&psi_factors(factors, a.category)), *e))
    };
    if variables.is_empty() {
        out.push(make(add(add(zero, gamma_g, g), 1.0, pre), "ULS-6.10-G".into(), "uls"));
        out.push(make(add(add(zero, gamma_g_inf, g), 1.0, pre), "ULS-6.10-G-fav".into(), "uls"));
        out.push(make(add(g, 1.0, pre), "SLS-qp".into(), "sls_qp"));
        out.push(make(add(g, 1.0, pre), "SLS-char".into(), "sls_char"));
    } else {
        for (lead_i, (lead_a, lead_e)) in variables.iter().enumerate() {
            let base_a = add(add(add(zero, gamma_g, g), 1.0, pre), gamma_q, *lead_e);
            let base_b = add(add(add(zero, xi * gamma_g, g), 1.0, pre), gamma_q, *lead_e);
            let a_eff = others(lead_i, &|p| gamma_q * p.0, base_a);
            let b_eff = others(lead_i, &|p| gamma_q * p.0, base_b);
            let (e, tag) = if a_eff.0.abs() >= b_eff.0.abs() { (a_eff, "6.10a") } else { (b_eff, "6.10b") };
            out.push(make(e, format!("ULS-{}-{}", tag, lead_a.id), "uls"));
            let fav = others(lead_i, &|p| gamma_q * p.0, add(add(add(zero, gamma_g_inf, g), 1.0, pre), gamma_q, *lead_e));
            out.push(make(fav, format!("ULS-fav-{}", lead_a.id), "uls"));
            out.push(make(others(lead_i, &|p| p.0, add(add(add(zero, 1.0, g), 1.0, pre), 1.0, *lead_e)), format!("SLS-char-{}", lead_a.id), "sls_char"));
            let (_, psi1, _) = psi_factors(factors, lead_a.category);
            out.push(make(others(lead_i, &|p| p.2, add(add(add(zero, 1.0, g), 1.0, pre), psi1, *lead_e)), format!("SLS-freq-{}", lead_a.id), "sls_freq"));
        }
        let qp = variables.iter().fold(add(g, 1.0, pre), |acc, (a, e)| add(acc, psi_factors(factors, a.category).2, *e));
        out.push(make(qp, "SLS-qp".into(), "sls_qp"));
    }
    let lead_acc = (0..variables.len()).max_by(|&a, &b| variables[a].1 .0.abs().partial_cmp(&variables[b].1 .0.abs()).unwrap_or(std::cmp::Ordering::Equal));
    for (aa, ae) in &accidentals {
        let mut e = add(add(add(zero, 1.0, g), 1.0, pre), 1.0, *ae);
        for (j, (oa, oe)) in variables.iter().enumerate() {
            let (_, psi1, psi2) = psi_factors(factors, oa.category);
            e = add(e, if Some(j) == lead_acc { psi1 } else { psi2 }, *oe);
        }
        out.push(make(e, format!("ACC-6.11-{}", aa.id), "accidental"));
    }
    out
}

/// 🎯 Governing combination of a situation by the largest |M_Ed| (flexure with its coincident N_Ed).
pub fn governing<'a>(effects: &'a [DesignEffects], sit: &str) -> Option<&'a DesignEffects> {
    governing_by(effects, sit, |e| e.m_ed.abs())
}

/// 🎯 Governing combination of a situation by an arbitrary key such as |V_Ed|, |T_Ed| or the punching shear.
pub fn governing_by<'a>(effects: &'a [DesignEffects], sit: &str, key: impl Fn(&DesignEffects) -> f64) -> Option<&'a DesignEffects> {
    effects.iter().filter(|e| e.situation == sit).max_by(|a, b| key(a).partial_cmp(&key(b)).unwrap_or(std::cmp::Ordering::Equal))
}
