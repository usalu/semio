//! 🧮️ Load combinations of the EN 1995 timber checks: the action internals, the member and connection combination enumeration and the deflection. See [`super`].

use super::*;

/// ⚖️ Characteristic internals from loads or analysed forces.
#[derive(Clone, Copy, Debug, Default)]
pub struct ActionInternals {
    pub m_k_nm: f64,
    pub v_k_n: f64,
    pub n_k_n: f64,
    pub n_t_k_n: f64,
    pub f_c90_k_n: f64,
    pub q_line_n_per_m: f64,
    pub f_point_n: f64,
}

/// 🧮 Derive M/V/N from SupportType when loads are nonzero; else use analysed forces.
pub fn action_internals(m: &TimberMember, a: &CharacteristicAction) -> ActionInternals {
    let q = a.q_line_n_per_m;
    let f = a.f_point_n;
    let l = m.span_m.max(0.0);
    let loads = q.abs() > 0.0 || f.abs() > 0.0;
    if loads {
        let column = m.role == MemberRole::Column;
        let (q_t, f_t) = if column { (q, 0.0) } else { (q, f) };
        let (mk, vk) = match m.support {
            SupportType::SimplySupported => (q_t * l * l / 8.0 + f_t * l / 4.0, q_t * l / 2.0 + f_t / 2.0),
            SupportType::Cantilever => (q_t * l * l / 2.0 + f_t * l, q_t * l + f_t),
            SupportType::ContinuousTwoSpan => (q_t * l * l / 8.0 + 3.0 * f_t * l / 16.0, 5.0 * q_t * l / 8.0 + 11.0 * f_t / 16.0),
        };
        let nk = if column { f } else { 0.0 };
        let fc90 = vk.max(0.0);
        ActionInternals {
            m_k_nm: mk,
            v_k_n: vk,
            n_k_n: nk,
            n_t_k_n: 0.0,
            f_c90_k_n: fc90,
            q_line_n_per_m: q,
            f_point_n: f_t,
        }
    } else {
        ActionInternals {
            m_k_nm: a.m_k_nm,
            v_k_n: a.v_k_n,
            n_k_n: a.n_k_n,
            n_t_k_n: a.n_t_k_n,
            f_c90_k_n: a.f_c90_k_n,
            q_line_n_per_m: 0.0,
            f_point_n: 0.0,
        }
    }
}

pub(super) fn action_kind_family(kind: semio_s_artifact_norm_en1995_model::ActionKind) -> &'static str {
    use semio_s_artifact_norm_en1995_model::ActionKind as K;
    match kind {
        K::Permanent => "permanent",
        K::Accidental => "accidental",
        K::Snow => "snow",
        K::Wind => "wind",
        K::Imposed => "imposed",
    }
}

/// 📦 Limit-state combination kind (EN 1990 + DE NA eq 6.10 family).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboKind {
    Uls,
    SlsCharacteristic,
    SlsFrequent,
    SlsQuasiPermanent,
    Accidental,
}

/// 📦 Factored design internals for one load combination.
#[derive(Clone, Debug)]
pub struct LoadCombo {
    pub id: String,
    pub kind: ComboKind,
    pub duration: LoadDuration,
    pub m_ed_nm: f64,
    pub v_ed_n: f64,
    pub n_ed_n: f64,
    pub n_t_ed_n: f64,
    pub f_c90_ed_n: f64,
    pub q_ed_line: f64,
    pub f_ed_point: f64,
    pub psi2: f64,
}

fn scale_internals(i: &ActionInternals, factor: f64) -> ActionInternals {
    ActionInternals {
        m_k_nm: i.m_k_nm * factor,
        v_k_n: i.v_k_n * factor,
        n_k_n: i.n_k_n * factor,
        n_t_k_n: i.n_t_k_n * factor,
        f_c90_k_n: i.f_c90_k_n * factor,
        q_line_n_per_m: i.q_line_n_per_m * factor,
        f_point_n: i.f_point_n * factor,
    }
}

fn add_internals(a: &mut ActionInternals, b: &ActionInternals) {
    a.m_k_nm += b.m_k_nm;
    a.v_k_n += b.v_k_n;
    a.n_k_n += b.n_k_n;
    a.n_t_k_n += b.n_t_k_n;
    a.f_c90_k_n += b.f_c90_k_n;
    a.q_line_n_per_m += b.q_line_n_per_m;
    a.f_point_n += b.f_point_n;
}

fn combo_from(id: String, kind: ComboKind, duration: LoadDuration, i: ActionInternals, psi2: f64) -> LoadCombo {
    LoadCombo {
        id,
        kind,
        duration,
        m_ed_nm: i.m_k_nm,
        v_ed_n: i.v_k_n,
        n_ed_n: i.n_k_n,
        n_t_ed_n: i.n_t_k_n,
        f_c90_ed_n: i.f_c90_k_n,
        q_ed_line: i.q_line_n_per_m,
        f_ed_point: i.f_point_n,
        psi2,
    }
}

fn shortest_duration<'a>(actions: impl IntoIterator<Item = &'a CharacteristicAction>) -> LoadDuration {
    actions
        .into_iter()
        .map(|a| load_duration_of(a.load_duration))
        .max()
        .unwrap_or(LoadDuration::Medium)
}

/// 🔀 EN 1990 eq 6.10 ULS (γ_G,sup, γ_Q and K_FI of the reliability class, ψ₀ accompanying), SLS char/frequent(ψ₁)/QP, accidental, all factors from the registry through `params.factors`; `params.uls_ab` replaces equation 6.10 by the pair 6.10a (all variable actions with ψ₀) and 6.10b (ξ on the permanent actions, one leading variable action).
pub fn enumerate_combos(m: &TimberMember, params: &AnnexParams) -> Vec<LoadCombo> {
    let (ab, fac) = (params.uls_ab, params.factors);
    let mut out = Vec::new();
    if m.actions.is_empty() {
        return out;
    }
    let internals: Vec<ActionInternals> = m.actions.iter().map(|a| action_internals(m, a)).collect();
    let mut perm = Vec::new();
    let mut var = Vec::new();
    let mut acc = Vec::new();
    for (idx, a) in m.actions.iter().enumerate() {
        match action_kind_family(a.kind) {
            "permanent" => perm.push(idx),
            "accidental" => acc.push(idx),
            _ => var.push(idx),
        }
    }
    let (gamma_g, gamma_q) = (fac.gamma_g_sup(), fac.gamma_q_building());
    let gamma_g_lead = if ab { fac.gamma_g_sup_610b() } else { gamma_g };
    let office = psi_factors(&fac, semio_s_artifact_norm_en1995_model::ActionKind::Imposed, semio_s_artifact_norm_en1995_model::ImposedCategory::B);
    let eq_tag = if ab { "6.10b" } else { "6.10" };

    let sum_perm = |factor: f64| {
        let mut s = ActionInternals::default();
        for &i in &perm {
            add_internals(&mut s, &scale_internals(&internals[i], factor));
        }
        s
    };

    if var.is_empty() {
        let dur = shortest_duration(perm.iter().map(|&i| &m.actions[i]));
        out.push(combo_from(
            "uls.g".into(),
            ComboKind::Uls,
            dur,
            sum_perm(gamma_g),
            1.0,
        ));
    } else {
        for (li, &lead) in var.iter().enumerate() {
            let mut s = sum_perm(gamma_g_lead);
            add_internals(&mut s, &scale_internals(&internals[lead], gamma_q));
            let mut psi2_lead = office.2;
            for (vi, &v) in var.iter().enumerate() {
                let (psi0, _, psi2) = psi_factors(&fac, m.actions[v].kind, m.actions[v].category);
                if vi == li {
                    psi2_lead = psi2;
                    continue;
                }
                add_internals(&mut s, &scale_internals(&internals[v], gamma_q * psi0));
            }
            let mut acts: Vec<&CharacteristicAction> = perm.iter().map(|&i| &m.actions[i]).collect();
            acts.extend(var.iter().map(|&i| &m.actions[i]));
            let dur = shortest_duration(acts);
            out.push(combo_from(
                format!("uls.{eq_tag}.lead.{}", m.actions[lead].id),
                ComboKind::Uls,
                dur,
                s,
                psi2_lead,
            ));
        }
    }

    if ab && !var.is_empty() {
        let mut s = sum_perm(gamma_g);
        for &v in &var {
            let (psi0, _, _) = psi_factors(&fac, m.actions[v].kind, m.actions[v].category);
            add_internals(&mut s, &scale_internals(&internals[v], gamma_q * psi0));
        }
        let dur = shortest_duration(perm.iter().chain(var.iter()).map(|&i| &m.actions[i]));
        out.push(combo_from("uls.6.10a".into(), ComboKind::Uls, dur, s, 0.3));
    }

    if !var.is_empty() {
        let mut durs: Vec<LoadDuration> = m.actions.iter().filter(|a| action_kind_family(a.kind) != "accidental").map(|a| load_duration_of(a.load_duration)).collect();
        durs.sort();
        durs.dedup();
        for d in durs {
            let sub: Vec<usize> = var.iter().copied().filter(|&i| load_duration_of(m.actions[i].load_duration) <= d).collect();
            if sub.len() == var.len() {
                continue;
            }
            if sub.is_empty() {
                out.push(combo_from(format!("uls.dur.{d:?}.g"), ComboKind::Uls, d, sum_perm(gamma_g), 1.0));
                continue;
            }
            for &lead in &sub {
                let mut s = sum_perm(gamma_g_lead);
                add_internals(&mut s, &scale_internals(&internals[lead], gamma_q));
                let mut psi2_lead = office.2;
                for &v in &sub {
                    let (psi0, _, psi2) = psi_factors(&fac, m.actions[v].kind, m.actions[v].category);
                    if v == lead {
                        psi2_lead = psi2;
                        continue;
                    }
                    add_internals(&mut s, &scale_internals(&internals[v], gamma_q * psi0));
                }
                out.push(combo_from(format!("uls.dur.{d:?}.lead.{}", m.actions[lead].id), ComboKind::Uls, d, s, psi2_lead));
            }
        }
    }

    if !var.is_empty() {
        for (li, &lead) in var.iter().enumerate() {
            let mut s = sum_perm(1.0);
            add_internals(&mut s, &scale_internals(&internals[lead], 1.0));
            let mut psi2_lead = office.2;
            for (vi, &v) in var.iter().enumerate() {
                let (psi0, _, psi2) = psi_factors(&fac, m.actions[v].kind, m.actions[v].category);
                if vi == li {
                    psi2_lead = psi2;
                    continue;
                }
                add_internals(&mut s, &scale_internals(&internals[v], psi0));
            }
            let dur = shortest_duration(var.iter().chain(perm.iter()).map(|&i| &m.actions[i]));
            out.push(combo_from(
                format!("sls.char.lead.{}", m.actions[lead].id),
                ComboKind::SlsCharacteristic,
                dur,
                s,
                psi2_lead,
            ));
        }
        for (li, &lead) in var.iter().enumerate() {
            let mut s = sum_perm(1.0);
            let mut psi1_lead = office.1;
            for (vi, &v) in var.iter().enumerate() {
                let (_, psi1, psi2) = psi_factors(&fac, m.actions[v].kind, m.actions[v].category);
                if vi == li {
                    psi1_lead = psi1;
                    add_internals(&mut s, &scale_internals(&internals[v], psi1));
                    continue;
                }
                add_internals(&mut s, &scale_internals(&internals[v], psi2));
            }
            let dur = shortest_duration(var.iter().chain(perm.iter()).map(|&i| &m.actions[i]));
            out.push(combo_from(
                format!("sls.freq.lead.{}", m.actions[lead].id),
                ComboKind::SlsFrequent,
                dur,
                s,
                psi1_lead,
            ));
        }
        let mut s = sum_perm(1.0);
        let mut psi2_max: f64 = 0.0;
        for &v in &var {
            let (_, _, psi2) = psi_factors(&fac, m.actions[v].kind, m.actions[v].category);
            psi2_max = psi2_max.max(psi2);
            add_internals(&mut s, &scale_internals(&internals[v], psi2));
        }
        let dur = shortest_duration(var.iter().chain(perm.iter()).map(|&i| &m.actions[i]));
        out.push(combo_from(
            "sls.qp".into(),
            ComboKind::SlsQuasiPermanent,
            dur,
            s,
            psi2_max,
        ));
    } else {
        let dur = shortest_duration(perm.iter().map(|&i| &m.actions[i]));
        out.push(combo_from(
            "sls.char.g".into(),
            ComboKind::SlsCharacteristic,
            dur,
            sum_perm(1.0),
            1.0,
        ));
        out.push(combo_from(
            "sls.freq.g".into(),
            ComboKind::SlsFrequent,
            dur,
            sum_perm(1.0),
            1.0,
        ));
        out.push(combo_from(
            "sls.qp.g".into(),
            ComboKind::SlsQuasiPermanent,
            dur,
            sum_perm(1.0),
            1.0,
        ));
    }

    if !acc.is_empty() {
        let mut s = sum_perm(1.0);
        for &i in &acc {
            add_internals(&mut s, &internals[i]);
        }
        for &v in &var {
            let (_, _, psi2) = psi_factors(&fac, m.actions[v].kind, m.actions[v].category);
            add_internals(&mut s, &scale_internals(&internals[v], psi2));
        }
        let dur = LoadDuration::Instantaneous;
        out.push(combo_from("accidental".into(), ComboKind::Accidental, dur, s, 0.0));
    }
    out
}

fn connection_action_internals(a: &ConnectionAction) -> f64 {
    a.f_k_n
}

/// 🔀 Connection force combinations mirroring member EN 1990 eq 6.10.
pub fn enumerate_connection_combos(c: &TimberConnection, params: &AnnexParams) -> Vec<(String, ComboKind, LoadDuration, f64)> {
    let fac = params.factors;
    let mut out = Vec::new();
    if c.actions.is_empty() {
        return out;
    }
    let mut perm = Vec::new();
    let mut var = Vec::new();
    let mut acc = Vec::new();
    for (idx, a) in c.actions.iter().enumerate() {
        match action_kind_family(a.kind) {
            "permanent" => perm.push(idx),
            "accidental" => acc.push(idx),
            _ => var.push(idx),
        }
    }
    let f = |i: usize| connection_action_internals(&c.actions[i]);
    let sum_perm = |factor: f64| perm.iter().map(|&i| f(i) * factor).sum::<f64>();
    let (gamma_g, gamma_q) = (fac.gamma_g_sup(), fac.gamma_q_building());
    let dur_of = |idxs: &[usize]| {
        idxs.iter()
            .map(|&i| load_duration_of(c.actions[i].load_duration))
            .max()
            .unwrap_or(LoadDuration::Medium)
    };
    if var.is_empty() {
        out.push(("uls.g".into(), ComboKind::Uls, dur_of(&perm), sum_perm(gamma_g)));
    } else {
        for &lead in &var {
            let mut fed = sum_perm(gamma_g) + gamma_q * f(lead);
            for &v in &var {
                if v == lead {
                    continue;
                }
                let (psi0, _, _) = psi_factors(&fac, c.actions[v].kind, semio_s_artifact_norm_en1995_model::ImposedCategory::NotImposed);
                fed += gamma_q * psi0 * f(v);
            }
            let mut idxs = perm.clone();
            idxs.extend_from_slice(&var);
            out.push((
                format!("uls.lead.{}", c.actions[lead].id),
                ComboKind::Uls,
                dur_of(&idxs),
                fed,
            ));
        }
    }
    if !acc.is_empty() {
        let mut fed = sum_perm(1.0);
        for &i in &acc {
            fed += f(i);
        }
        out.push((
            "accidental".into(),
            ComboKind::Accidental,
            LoadDuration::Instantaneous,
            fed,
        ));
    }
    out
}

pub fn deflection_m(m: &TimberMember, props: &TimberProperties, q: f64, f: f64) -> f64 {
    let ei = props.e_0_mean * i_m4(m);
    if ei <= 0.0 || m.span_m <= 0.0 {
        return 0.0;
    }
    let l = m.span_m;
    match m.support {
        SupportType::SimplySupported => 5.0 * q * l.powi(4) / (384.0 * ei) + f * l.powi(3) / (48.0 * ei),
        SupportType::Cantilever => q * l.powi(4) / (8.0 * ei) + f * l.powi(3) / (3.0 * ei),
        SupportType::ContinuousTwoSpan => {
            q * l.powi(4) / (185.0 * ei) + f * l.powi(3) / (100.0 * ei)
        }
    }
}
