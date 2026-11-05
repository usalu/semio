//! ⚖️ EN 1994 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

//#region 🔖️ComplianceHelpers
pub use semio_s_artifact_norm_en1994_checks_catalogue as catalogue;

#[path = "../../../🧭️spec-trace/🦀️.rs"]
pub mod spec_trace;
/// 📐️ Pure EN 1994 compliance helpers — geometry-derived resistances and clause checks.
use semio_s_plugin_norm_compliance::LocalizedCopy;
use semio_s_artifact_norm_en1994_model::{ActionKind, CompositeBeam, CompositeColumn, CompositeSlab};

/// 🇪️🇺️ National-annex NDPs for EN 1994.
/// Steel stability γ_M1: EN 1994-1-1 §2.4.1.2 → EN 1993-1-1 (EN recommended 1,00; DIN EN 1993-1-1/NA 1,10).
/// Bridge steel fatigue γ_Mf: EN 1994-2 §6.8.2 → EN 1993-1-9 Table 3.1 (EN recommended damage-tolerant
/// γ_Mf = 1,15); DIN EN 1993-1-9/NA adopts safe-life γ_Mf = 1,35 for bridges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub choice: AnnexChoice,
    pub gamma_v: f64,
    pub gamma_c: f64,
    pub gamma_s: f64,
    pub gamma_m0: f64,
    pub gamma_m1: f64,
    pub gamma_mf: f64,
}

impl AnnexParams {
    /// 🔖️ Registry ids (`ndp.en1993.*`, `ndp.en1994.*`) of the partial factors; a check cites the ones of the standard parts it rests on (see [`Self::ndps_of`]).
    pub const GAMMA_V: &'static str = "ndp.en1994.gamma-v";
    pub const GAMMA_C: &'static str = "ndp.en1994.gamma-c";
    pub const GAMMA_S: &'static str = "ndp.en1994.gamma-s";
    pub const GAMMA_M0: &'static str = "ndp.en1993.gamma-m0";
    pub const GAMMA_M1: &'static str = "ndp.en1993.gamma-m1";
    pub const GAMMA_MF: &'static str = "ndp.en1994.gamma-mf";

    /// 🔖️ Partial factors a check of the standard part `base` (for example `1992`, `1993-1-5`, `1994-1-1`, `1994-2`) rests on: EN 1990 and EN 1991 checks none, EN 1992 γ_C and γ_S, EN 1993 γ_M0 and γ_M1, the bridge part also γ_Mf, the composite part all but γ_Mf.
    pub fn ndps_of(base: &str) -> &'static [&'static str] {
        if base.starts_with("1990") || base.starts_with("1991") {
            &[]
        } else if base.starts_with("1992") {
            &[Self::GAMMA_C, Self::GAMMA_S]
        } else if base.starts_with("1993") {
            &[Self::GAMMA_M0, Self::GAMMA_M1]
        } else if base.starts_with("1994-2") {
            &[Self::GAMMA_V, Self::GAMMA_C, Self::GAMMA_S, Self::GAMMA_M0, Self::GAMMA_M1, Self::GAMMA_MF]
        } else {
            &[Self::GAMMA_V, Self::GAMMA_C, Self::GAMMA_S, Self::GAMMA_M0, Self::GAMMA_M1]
        }
    }

    /// 📖️ EN-recommended NDPs per EN 1994-1-1 §2.4.1.2 / EN 1994-2, read from the shared registry.
    pub fn en() -> Self {
        Self::for_annex(AnnexChoice::En)
    }

    /// 🇩️🇪️ DIN EN 1994-1-1/NA values of the shared registry; steel stability uses DIN EN 1993-1-1/NA γ_M1 = 1,10 and bridge fatigue the safe-life γ_Mf.
    pub fn de() -> Self {
        Self::for_annex(AnnexChoice::De)
    }

    pub fn for_annex(annex: AnnexChoice) -> Self {
        let n = |id: &str| semio_s_plugin_norm_compliance::ndp_number(id, annex);
        semio_s_plugin_norm_compliance::record_ndp_reads(|| Self { choice: annex, gamma_v: n(Self::GAMMA_V), gamma_c: n(Self::GAMMA_C), gamma_s: n(Self::GAMMA_S), gamma_m0: n(Self::GAMMA_M0), gamma_m1: n(Self::GAMMA_M1), gamma_mf: n(Self::GAMMA_MF) }).0
    }
}

fn lc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub mod part_en1990 {
    use crate::*;
    use crate::catalogue::continuous::{self, Load, Stiffness};
    use crate::catalogue::SpanPosition;
    use semio_s_artifact_norm_en1994_model::{CharacteristicAction, ColumnAction};

    /// 📋 Design effects from an EN 1990 combination.
    #[derive(Clone, Debug)]
    pub struct DesignEffects {
        pub m_nm: f64,
        pub m_hog_nm: f64,
        pub v_n: f64,
        pub n_n: f64,
        pub label_en: String,
        pub label_de: String,
    }

    impl Default for DesignEffects {
        fn default() -> Self {
            Self { m_nm: 0.0, m_hog_nm: 0.0, v_n: 0.0, n_n: 0.0, label_en: String::new(), label_de: String::new() }
        }
    }

    /// ⚖️ The EN 1990 partial and combination factors of the annex and the reliability class of the document from the registry (`ActionFactors`): K_FI scales γ_G and γ_Q.
    pub fn factors(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> semio_s_plugin_norm_compliance::ActionFactors {
        semio_s_plugin_norm_compliance::ActionFactors::new(annex, reliability_class, semio_s_plugin_norm_compliance::DesignSituation::Persistent)
    }

    /// 🎚️ ψ₀ / ψ₁ / ψ₂ of an action from the registry rows `ndp.en1990.psi.*` (Table A1.1, identical for the recommended values and DIN EN 1990/NA): snow and wind by kind, construction loads by `construction_load`, imposed actions by category letter A–H, anything else follows offices; permanent actions carry (1, 1, 1).
    pub fn psi_factors(kind: ActionKind, category: &str) -> (f64, f64, f64) {
        let key = match kind {
            ActionKind::Permanent => return (1.0, 1.0, 1.0),
            ActionKind::Snow => "snow",
            ActionKind::Wind => "wind",
            ActionKind::Construction => "construction_load",
            ActionKind::Imposed => category,
            ActionKind::Fatigue => "office",
        };
        let row = |annex: AnnexChoice| semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(annex, semio_s_plugin_norm_compliance::action_factors::canonical_category(key)).or_else(|| semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(annex, "office")).map_or((1.0, 1.0, 1.0), semio_s_plugin_norm_compliance::action_factors::PsiRow::as_tuple);
        let (en, de) = (row(AnnexChoice::En), row(AnnexChoice::De));
        debug_assert_eq!(en, de, "ψ rows of {key} differ between the annexes; pass the annex");
        en
    }

    /// 🔢️ γ_G,sup of the annex (registry row `ndp.en1990.gG_sup.str.A1` times K_FI).
    pub fn gamma_g(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> f64 {
        factors(annex, reliability_class).gamma_g_sup()
    }

    /// 🔢️ γ_Q of the annex (registry row `ndp.en1990.gQ.str.A1` times K_FI).
    pub fn gamma_q(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> f64 {
        factors(annex, reliability_class).gamma_q_building()
    }

    /// 🔥 ψ_fi for fire combination EN 1990 Eq. (6.11b) — EN/DE recommended 0.9 for offices (ψ₁) reduced; use 0.5 for category B ψ₂-like fire.
    pub fn psi_fi(kind: ActionKind, category: &str) -> f64 {
        let (_p0, p1, p2) = psi_factors(kind, category);
        if kind == ActionKind::Permanent { 1.0 } else { p1.max(p2) }
    }

    /// 📍️ Position of a concentrated force as a fraction of the span from the first support: `x_m / span` clamped to the span, 0.5 (mid-span) when `x_m` is unset.
    pub fn point_fraction(action: &CharacteristicAction, span_m: f64) -> f64 {
        if action.x_m > 0.0 && span_m > 1e-9 { (action.x_m / span_m).clamp(0.0, 1.0) } else { 0.5 }
    }

    /// 📐️ Characteristic internals (span moment, support moment, shear, axial force) of one action on the designed span of a simple or continuous beam; concentrated forces act at their position `xM` (0 = mid-span). Continuous spans (`continuous_2_span` / `end_span`, `interior_span` with the neighbour `adjacent_m`, `cantilever` whose `span_m` is the cantilever length) are solved by the shared mechanics statics (`catalogue::continuous::action_internals`), the equal-span end span reproducing 9qL²/128 / 49qL²/512, M_B = qL²/8, V_B = 5qL/8 and 5PL/32 / 13PL/64, 3PL/16, 11P/16 for mid-span forces.
    pub fn action_internals(action: &CharacteristicAction, span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64) -> (f64, f64, f64, f64) {
        let Some(load) = action_load(action, span_m, spacing_m) else {
            return (0.0, 0.0, 0.0, 0.0);
        };
        let position = crate::catalogue::span_position(support);
        if position == SpanPosition::Simple {
            return (load.simple_sagging(span_m), 0.0, load.simple_shear(span_m), 0.0);
        }
        let forces = continuous::action_internals(position, span_m, adjacent_m, load, action.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent, Stiffness::uniform(1.0));
        (forces.sagging, forces.hogging, forces.shear, 0.0)
    }

    /// 🏋️ The line load or concentrated force of an action on the designed span, `None` for an action without load.
    fn action_load(action: &CharacteristicAction, span_m: f64, spacing_m: f64) -> Option<Load> {
        let q = action.q_area_pa * spacing_m;
        if q.abs() > 1e-12 {
            Some(Load::Udl(q))
        } else if action.f_k_n.abs() > 1.0 {
            Some(Load::Point(action.f_k_n, point_fraction(action, span_m)))
        } else {
            None
        }
    }

    /// 🪜️ Design internals (span moment, support moment, shear, axial force) of a permanent action in the fundamental combination: on continuous spans it is individual per span and takes γG,sup where unfavourable and γG,inf on the other spans (EN 1990 A1.3.1, shared `continuous::permanent_internals`); a simple span has one factor γG,sup.
    pub fn design_permanent_internals(action: &CharacteristicAction, span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> (f64, f64, f64, f64) {
        let position = crate::catalogue::span_position(support);
        let Some(load) = action_load(action, span_m, spacing_m) else {
            return (0.0, 0.0, 0.0, 0.0);
        };
        let fac = factors(annex, reliability_class);
        if position == SpanPosition::Simple {
            let g = fac.gamma_g_sup();
            return (g * load.simple_sagging(span_m), 0.0, g * load.simple_shear(span_m), 0.0);
        }
        let span_factors = semio_s_plugin_norm_mechanics::SpanFactors { gamma_g: fac.gamma_g_sup(), gamma_g_inf: fac.gamma_g_inf(), gamma_q: fac.gamma_q_building() };
        let forces = continuous::permanent_internals(position, span_m, adjacent_m, load, Stiffness::uniform(1.0), &span_factors);
        (forces.sagging, forces.hogging, forces.shear, 0.0)
    }

    /// 🏛️ Characteristic N/M from a column analysis action.
    pub fn column_action_internals(action: &ColumnAction) -> (f64, f64, f64, f64) {
        (action.m_k_nm, 0.0, 0.0, action.n_k_n)
    }

    fn accumulate_column(actions: &[ColumnAction], stage: &str, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, mode: &str) -> DesignEffects {
        let g_g = gamma_g(annex, reliability_class);
        let g_q = gamma_q(annex, reliability_class);
        let staged: Vec<_> = actions.iter().filter(|a| a.stage == stage || a.stage == semio_s_artifact_norm_en1994_model::ActionStage::Any).collect();
        let permanents: Vec<_> = staged.iter().copied().filter(|a| a.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent).collect();
        let variables: Vec<_> = staged.iter().copied().filter(|a| matches!(a.kind, semio_s_artifact_norm_en1994_model::ActionKind::Imposed | semio_s_artifact_norm_en1994_model::ActionKind::Snow | semio_s_artifact_norm_en1994_model::ActionKind::Wind | semio_s_artifact_norm_en1994_model::ActionKind::Construction)).collect();
        let mut m = 0.0;
        let mut n = 0.0;
        let scale_perm = match mode {
            "uls" => g_g,
            _ => 1.0,
        };
        for a in &permanents {
            let (mi, _, _, ni) = column_action_internals(a);
            m += scale_perm * mi;
            n += scale_perm * ni;
        }
        match mode {
            "uls" => {
                let mut best = DesignEffects::default();
                if variables.is_empty() {
                    return DesignEffects {
                        m_nm: m, m_hog_nm: 0.0, v_n: 0.0, n_n: n,
                        label_en: format!("ULS 6.10 ({stage}, G only)"),
                        label_de: format!("GZT 6.10 ({stage}, nur G)"),
                    };
                }
                for (i, lead) in variables.iter().enumerate() {
                    let (lm, _, _, ln) = column_action_internals(lead);
                    let mut mm = m + g_q * lm;
                    let mut nn = n + g_q * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (p0, _, _) = psi_factors(acc.kind, &acc.category);
                        let (am, _, _, an) = column_action_internals(acc);
                        mm += g_q * p0 * am;
                        nn += g_q * p0 * an;
                    }
                    if mm.abs() + nn.abs() >= best.m_nm.abs() + best.n_n.abs() {
                        best = DesignEffects {
                            m_nm: mm, m_hog_nm: 0.0, v_n: 0.0, n_n: nn,
                            label_en: format!("ULS Eq. 6.10 ({stage}, leading {})", lead.id),
                            label_de: format!("GZT Gl. 6.10 ({stage}, führend {})", lead.id),
                        };
                    }
                }
                best
            }
            "fire" => {
                let mut best = DesignEffects { m_nm: m, m_hog_nm: 0.0, v_n: 0.0, n_n: n, label_en: "Fire Eq. 6.11b".into(), label_de: "Brand Gl. 6.11b".into() };
                for (i, lead) in variables.iter().enumerate() {
                    let (_, p1, _) = psi_factors(lead.kind, &lead.category);
                    let (lm, _, _, ln) = column_action_internals(lead);
                    let mut mm = m + p1 * lm;
                    let mut nn = n + p1 * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (_, _, p2) = psi_factors(acc.kind, &acc.category);
                        let (am, _, _, an) = column_action_internals(acc);
                        mm += p2 * am;
                        nn += p2 * an;
                    }
                    if mm.abs() + nn.abs() >= best.m_nm.abs() + best.n_n.abs() {
                        best = DesignEffects { m_nm: mm, m_hog_nm: 0.0, v_n: 0.0, n_n: nn, label_en: format!("Fire ψ₁,₁ (leading {})", lead.id), label_de: format!("Brand ψ₁,₁ (führend {})", lead.id) };
                    }
                }
                best
            }
            _ => DesignEffects { m_nm: m, m_hog_nm: 0.0, v_n: 0.0, n_n: n, label_en: mode.into(), label_de: mode.into() },
        }
    }

    pub fn uls_column(actions: &[ColumnAction], annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate_column(actions, "composite", annex, reliability_class, "uls")
    }

    pub fn fire_column(actions: &[ColumnAction], annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate_column(actions, "composite", annex, reliability_class, "fire")
    }

    fn accumulate(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, stage: &str, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, mode: &str) -> DesignEffects {
        let g_q = gamma_q(annex, reliability_class);
        let staged: Vec<_> = actions.iter().filter(|a| a.stage == stage || a.stage == semio_s_artifact_norm_en1994_model::ActionStage::Any).collect();
        let permanents: Vec<_> = staged.iter().copied().filter(|a| a.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent).collect();
        let variables: Vec<_> = staged.iter().copied().filter(|a| matches!(a.kind, semio_s_artifact_norm_en1994_model::ActionKind::Imposed | semio_s_artifact_norm_en1994_model::ActionKind::Snow | semio_s_artifact_norm_en1994_model::ActionKind::Wind | semio_s_artifact_norm_en1994_model::ActionKind::Construction)).collect();

        let mut m = 0.0;
        let mut m_hog = 0.0;
        let mut v = 0.0;
        let mut n = 0.0;
        let mut label_en = String::new();
        let mut label_de = String::new();

        for a in &permanents {
            if matches!(mode, "uls" | "uls_construction") {
                let (mi, hi, vi, ni) = design_permanent_internals(a, span_m, adjacent_m, support, spacing_m, annex, reliability_class);
                m += mi; m_hog += hi; v += vi; n += ni;
                continue;
            }
            let (mi, hi, vi, ni) = action_internals(a, span_m, adjacent_m, support, spacing_m);
            let s = if mode == "fire" { psi_fi(a.kind, &a.category) } else { 1.0 };
            m += s * mi; m_hog += s * hi; v += s * vi; n += s * ni;
        }

        match mode {
            "uls" | "uls_construction" => {
                if variables.is_empty() {
                    label_en = format!("ULS 6.10 ({stage}, G only)");
                    label_de = format!("GZT 6.10 ({stage}, nur G)");
                    return DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en, label_de };
                }
                let mut candidates = Vec::with_capacity(variables.len());
                for (i, lead) in variables.iter().enumerate() {
                    let (lm, lh, lv, ln) = action_internals(lead, span_m, adjacent_m, support, spacing_m);
                    let mut mm = m + g_q * lm;
                    let mut mh = m_hog + g_q * lh;
                    let mut vv = v + g_q * lv;
                    let mut nn = n + g_q * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (p0, _, _) = psi_factors(acc.kind, &acc.category);
                        let (am, ah, av, an) = action_internals(acc, span_m, adjacent_m, support, spacing_m);
                        mm += g_q * p0 * am; mh += g_q * p0 * ah; vv += g_q * p0 * av; nn += g_q * p0 * an;
                    }
                    candidates.push(DesignEffects {
                        m_nm: mm, m_hog_nm: mh, v_n: vv, n_n: nn,
                        label_en: format!("ULS Eq. 6.10 ({stage}, leading {})", lead.id),
                        label_de: format!("GZT Gl. 6.10 ({stage}, führend {})", lead.id),
                    });
                }
                let peak = |f: fn(&DesignEffects) -> f64| candidates.iter().map(|c| f(c).abs()).fold(0.0, f64::max).max(f64::MIN_POSITIVE);
                let (pm, ph, pv, pn) = (peak(|c| c.m_nm), peak(|c| c.m_hog_nm), peak(|c| c.v_n), peak(|c| c.n_n));
                let demand = |c: &DesignEffects| c.m_nm.abs() / pm + c.m_hog_nm.abs() / ph + c.v_n.abs() / pv + c.n_n.abs() / pn;
                let mut best = candidates.remove(0);
                for c in candidates {
                    if demand(&c) >= demand(&best) { best = c; }
                }
                best
            }
            "sls_char" => {
                let mut best = DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "SLS characteristic".into(), label_de: "GZG charakteristisch".into() };
                for (i, lead) in variables.iter().enumerate() {
                    let (lm, lh, lv, ln) = action_internals(lead, span_m, adjacent_m, support, spacing_m);
                    let mut mm = m + lm; let mut mh = m_hog + lh; let mut vv = v + lv; let mut nn = n + ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (p0, _, _) = psi_factors(acc.kind, &acc.category);
                        let (am, ah, av, an) = action_internals(acc, span_m, adjacent_m, support, spacing_m);
                        mm += p0 * am; mh += p0 * ah; vv += p0 * av; nn += p0 * an;
                    }
                    if mm.abs() >= best.m_nm.abs() {
                        best = DesignEffects { m_nm: mm, m_hog_nm: mh, v_n: vv, n_n: nn, label_en: format!("SLS characteristic (leading {})", lead.id), label_de: format!("GZG charakteristisch (führend {})", lead.id) };
                    }
                }
                best
            }
            "sls_freq" => {
                let mut best = DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "SLS frequent".into(), label_de: "GZG häufig".into() };
                for (i, lead) in variables.iter().enumerate() {
                    let (_, p1, _) = psi_factors(lead.kind, &lead.category);
                    let (lm, lh, lv, ln) = action_internals(lead, span_m, adjacent_m, support, spacing_m);
                    let mut mm = m + p1 * lm; let mut mh = m_hog + p1 * lh; let mut vv = v + p1 * lv; let mut nn = n + p1 * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (_, _, p2) = psi_factors(acc.kind, &acc.category);
                        let (am, ah, av, an) = action_internals(acc, span_m, adjacent_m, support, spacing_m);
                        mm += p2 * am; mh += p2 * ah; vv += p2 * av; nn += p2 * an;
                    }
                    if mm.abs() >= best.m_nm.abs() {
                        best = DesignEffects { m_nm: mm, m_hog_nm: mh, v_n: vv, n_n: nn, label_en: format!("SLS frequent ψ₁ (leading {})", lead.id), label_de: format!("GZG häufig ψ₁ (führend {})", lead.id) };
                    }
                }
                best
            }
            "sls_qp" => {
                for a in &variables {
                    let (_, _, p2) = psi_factors(a.kind, &a.category);
                    let (mi, hi, vi, ni) = action_internals(a, span_m, adjacent_m, support, spacing_m);
                    m += p2 * mi; m_hog += p2 * hi; v += p2 * vi; n += p2 * ni;
                }
                DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "SLS quasi-permanent".into(), label_de: "GZG quasi-ständig".into() }
            }
            "fire" => {
                let mut best = DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: "Fire Eq. 6.11b".into(), label_de: "Brand Gl. 6.11b".into() };
                for (i, lead) in variables.iter().enumerate() {
                    let (_, p1, _) = psi_factors(lead.kind, &lead.category);
                    let (lm, lh, lv, ln) = action_internals(lead, span_m, adjacent_m, support, spacing_m);
                    let mut mm = m + p1 * lm; let mut mh = m_hog + p1 * lh; let mut vv = v + p1 * lv; let mut nn = n + p1 * ln;
                    for (j, acc) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let (_, _, p2) = psi_factors(acc.kind, &acc.category);
                        let (am, ah, av, an) = action_internals(acc, span_m, adjacent_m, support, spacing_m);
                        mm += p2 * am; mh += p2 * ah; vv += p2 * av; nn += p2 * an;
                    }
                    if mm.abs() >= best.m_nm.abs() {
                        best = DesignEffects { m_nm: mm, m_hog_nm: mh, v_n: vv, n_n: nn, label_en: format!("Fire ψ₁,₁ (leading {})", lead.id), label_de: format!("Brand ψ₁,₁ (führend {})", lead.id) };
                    }
                }
                best
            }
            _ => DesignEffects { m_nm: m, m_hog_nm: m_hog, v_n: v, n_n: n, label_en: mode.into(), label_de: mode.into() },
        }
    }

    pub fn uls_composite(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate(actions, span_m, adjacent_m, support, spacing_m, "composite", annex, reliability_class, "uls")
    }
    pub fn uls_construction(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate(actions, span_m, adjacent_m, support, spacing_m, "construction", annex, reliability_class, "uls_construction")
    }
    pub fn sls_characteristic(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate(actions, span_m, adjacent_m, support, spacing_m, "composite", annex, reliability_class, "sls_char")
    }
    pub fn sls_frequent(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate(actions, span_m, adjacent_m, support, spacing_m, "composite", annex, reliability_class, "sls_freq")
    }
    pub fn sls_quasi_permanent(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate(actions, span_m, adjacent_m, support, spacing_m, "composite", annex, reliability_class, "sls_qp")
    }
    /// 🧮 Effects of one stage under an SLS or fire combination mode (`sls_char`, `sls_freq`, `sls_qp`, `fire`).
    pub fn stage_effects(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, stage: &str, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, mode: &str) -> DesignEffects {
        accumulate(actions, span_m, adjacent_m, support, spacing_m, stage, annex, reliability_class, mode)
    }
    pub fn fire_combination(actions: &[CharacteristicAction], span_m: f64, adjacent_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, spacing_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> DesignEffects {
        accumulate(actions, span_m, adjacent_m, support, spacing_m, "composite", annex, reliability_class, "fire")
    }

    /// 🌉️ EN 1991-2 FLM3 — simplified mid-span stress range for a simply-supported girder.
    pub fn flm3_delta_sigma_pa(span_m: f64, w_el_m3: f64) -> f64 {
        let m = 240e3 * (span_m / 4.0);
        m / w_el_m3.max(1e-9)
    }

    pub fn flm3_delta_tau_pa(stud_d_m: f64) -> f64 {
        let f = 60e3;
        let a = std::f64::consts::PI * (stud_d_m * 0.5).powi(2);
        f / a.max(1e-9)
    }
}

pub mod part_1_1 {
    use crate::*;
    use crate::catalogue as cat;
    use semio_s_artifact_norm_en1994_model::SteelSection;

    /// 📐️ Effective width b_eff [m] per EN 1994-1-1 §5.4.1.2: `b0` is the distance between the outer connector rows, both half-widths equal half the beam spacing.
    pub fn effective_width_m(span_m: f64, b0_m: f64, beam_spacing_m: f64) -> f64 {
        cat::effective_width_m(span_m, b0_m, beam_spacing_m / 2.0, beam_spacing_m / 2.0, false)
    }

    /// 📐️ Uncapped width b_0 + L_e/4 [m] before the b_i = spacing/2 bound (§5.4.1.2).
    pub fn uncapped_effective_width_m(span_m: f64, b0_m: f64) -> f64 {
        b0_m + span_m / 4.0
    }

    /// 🧱 Plate model of a catalogue-resolved steel section.
    pub fn isection(steel: &SteelSection) -> cat::ISection {
        cat::ISection { h: steel.height_m, b: steel.width_m, tf: steel.tf_m, tw: steel.tw_m, a: steel.a_m2, i_y: steel.i_y_m4 }
    }

    /// 🧱 Concrete height above the ribs h_c [m] (at least 40 mm).
    pub fn concrete_height_m(beam: &CompositeBeam) -> f64 {
        (beam.slab_thickness_m - beam.sheeting.height_m).max(0.04)
    }

    /// 🧱 Stress-block resistance for a concrete force `nc_n` (cl. 6.2.1.3).
    pub fn stress_block(beam: &CompositeBeam, b_eff_m: f64, f_y_pa: f64, annex: AnnexChoice, nc_n: f64) -> cat::Plastic {
        let p = AnnexParams::for_annex(annex);
        cat::plastic_sagging(&isection(&beam.steel), f_y_pa / p.gamma_m0, beam.slab_thickness_m, b_eff_m, concrete_height_m(beam), cat::fcc_beam_pa(beam.concrete_f_ck_pa, p.gamma_c), nc_n)
    }

    /// 📐️ Steel plastic moment M_pl,a [N·m].
    pub fn steel_plastic_moment_nm(w_pl_y_m3: f64, f_y_pa: f64, gamma_m0: f64) -> f64 {
        w_pl_y_m3 * f_y_pa / gamma_m0
    }

    /// 📐️ Full composite plastic sagging moment M_pl,Rd [N·m] from the plastic stress blocks (cl. 6.2.1.2).
    pub fn full_plastic_moment_nm(beam: &CompositeBeam, b_eff_m: f64, f_y_pa: f64, annex: AnnexChoice) -> f64 {
        stress_block(beam, b_eff_m, f_y_pa, annex, f64::INFINITY).m_nm
    }

    /// 📐️ Full-connection concrete force N_cf = min(N_pl,a, 0.85·f_cd·b_eff·h_c) [N].
    pub fn n_cf_n(beam: &CompositeBeam, b_eff_m: f64, f_y_pa: f64, annex: AnnexChoice) -> f64 {
        stress_block(beam, b_eff_m, f_y_pa, annex, f64::INFINITY).nc_n
    }

    /// 📐️ Partial-connection plastic moment by linear interpolation M_a + η·(M_pl,Rd − M_a) (§6.2.1.3).
    pub fn plastic_moment_partial_nm(m_pla_nm: f64, m_pl_rd_nm: f64, eta: f64) -> f64 {
        cat::plastic_linear(m_pla_nm, m_pl_rd_nm, eta)
    }

    /// 📐️ Stud α per EN 1994-1-1 §6.6.3.1(1).
    pub fn stud_alpha(h_sc_m: f64, d_m: f64) -> f64 {
        let ratio = h_sc_m / d_m;
        if ratio > 4.0 { 1.0 } else { 0.2 * (ratio + 1.0) }
    }

    /// 📐️ Representative headed stud design resistance P_Rd [N] per EN 1994-1-1 §6.6.3.1 and §6.6.4: the mean over the studs inside the designed span (0 without studs).
    pub fn connector_resistance_n(studs: &crate::part_studs::BeamStuds) -> f64 {
        studs.mean_resistance_n()
    }

    /// 📐️ Required studs per half span for full shear connection (0 without studs).
    pub fn n_f_req(beam: &CompositeBeam, studs: &crate::part_studs::BeamStuds, b_eff_m: f64, f_y_pa: f64, annex: AnnexChoice) -> u32 {
        let p_rd = connector_resistance_n(studs);
        if p_rd <= 1.0 { 0 } else { (n_cf_n(beam, b_eff_m, f_y_pa, annex) / p_rd).ceil() as u32 }
    }

    /// 📐️ Degree of shear connection η = n_f / n_f,req.
    pub fn shear_connection_degree(n_f: u32, n_f_req: u32) -> f64 {
        if n_f_req == 0 { 1.0 } else { n_f as f64 / n_f_req as f64 }
    }

    /// 📐️ Degree of shear connection from the persisted studs: min(1, Σ P_Rd of the weaker half span / N_c,f), the single source of η for checks and scene.
    pub fn declared_shear_connection_degree(studs: &crate::part_studs::BeamStuds, n_cf_n: f64) -> f64 {
        degree_from_capacity(studs.capacity_n(), n_cf_n)
    }

    /// 📐️ η = min(1, Σ P_Rd / N_c,f) for the connection capacity `capacity_n` of a half span.
    pub fn degree_from_capacity(capacity_n: f64, n_cf_n: f64) -> f64 {
        (capacity_n / n_cf_n.max(1.0)).min(1.0)
    }

    /// 📐️ η_min per EN 1994-1-1 §6.6.1.2 for equal flanges and ductile studs.
    pub fn min_shear_connection_degree(span_m: f64, f_y_pa: f64) -> f64 {
        cat::eta_min(span_m, f_y_pa, true)
    }

    /// 📐️ Vertical shear resistance of the steel section V_pl,a,Rd [N] (§6.2.2).
    pub fn vertical_shear_resistance_n(beam: &CompositeBeam, f_y_pa: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        beam.steel.shear_area_from_plates_m2() * (f_y_pa / 3.0_f64.sqrt()) / p.gamma_m0
    }

    /// 📐️ Welded (non-catalogue) section: buckling curves c/d instead of a/b.
    pub fn is_welded(beam: &CompositeBeam) -> bool {
        SteelSection::from_catalogue(&beam.steel.designation).is_none()
    }

    /// 📐️ Section modulus for the LTB reference moment: W_pl for class 1/2, W_el for class 3.
    pub fn ltb_modulus_m3(beam: &CompositeBeam, f_y_pa: f64) -> f64 {
        if section_class(&beam.steel, f_y_pa) <= 2 { beam.steel.w_pl_y_m3 } else { isection(&beam.steel).w_el() }
    }

    /// 📐️ Elastic critical moment of the bare steel beam over the LTB length and the resulting M_b,Rd [N·m] (construction stage).
    pub fn ltb_moment_resistance_nm(beam: &CompositeBeam, f_y_pa: f64, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let sec = isection(&beam.steel);
        let mcr = cat::mcr_bare(&sec, beam.ltb_length_m.max(0.05));
        let w = ltb_modulus_m3(beam, f_y_pa);
        let (_, chi) = cat::chi_lt(w * f_y_pa, mcr, sec.h / sec.b, is_welded(beam));
        chi * w * f_y_pa / p.gamma_m1
    }

    /// 📐️ Cracked transverse slab stiffness I_2 per metre from the transverse reinforcement (lever 0.8·d).
    pub fn slab_i2_m4_per_m(beam: &CompositeBeam) -> f64 {
        let d = (concrete_height_m(beam) - 0.03).max(0.02);
        beam.transverse_as_m2_per_m * (0.8 * d).powi(2)
    }

    /// 📐️ Inverted U-frame M_cr and M_b,Rd [N·m] for the hogging region (EN 1994-1-1 §6.4.2): returns (M_cr, λ_LT, χ_LT, M_b,Rd).
    pub fn ltb_hogging(beam: &CompositeBeam, f_y_pa: f64, annex: AnnexChoice) -> (f64, f64, f64, f64) {
        let p = AnnexParams::for_annex(annex);
        let sec = isection(&beam.steel);
        let ks = cat::uframe_ks(slab_i2_m4_per_m(beam), beam.spacing_m, &sec, 2.0);
        let mcr = cat::mcr_uframe(&sec, beam.ltb_length_m.max(0.05), ks);
        let w = ltb_modulus_m3(beam, f_y_pa);
        let (lam, chi) = cat::chi_lt(w * f_y_pa, mcr, sec.h / sec.b, is_welded(beam));
        (mcr, lam, chi, chi * w * f_y_pa / p.gamma_m1)
    }

    /// 📐️ Modular ratio n_L = E_a/E_cm·(1 + ψ_L·φ) with ψ_L = 1.1 for permanent actions (§5.4.2.2).
    pub fn modular_ratio_long_term(e_cm_pa: f64, phi: f64) -> f64 {
        cat::modular_ratio(e_cm_pa, cat::constants::PSI_L_PERMANENT, phi)
    }

    /// 📐️ EN 1994-1-1 §6.6.5.5 — minimum stud spacing factor × diameter.
    pub const STUD_SPACING_MIN_DIAMETER_FACTOR: f64 = 5.0;
    /// 📐️ EN 1994-1-1 §6.6.5.5 — maximum stud spacing factor × concrete height h_c.
    pub const STUD_SPACING_MAX_HC_FACTOR: f64 = 6.0;
    /// 📐️ EN 1994-1-1 §6.6.5.5 — absolute maximum stud spacing [m].
    pub const STUD_SPACING_MAX_ABS_M: f64 = 0.800;

    /// 📐️ Minimum / maximum longitudinal stud spacing [m] per EN 1994-1-1 §6.6.5.5 for studs of the shank diameter `diameter_m`.
    pub fn stud_spacing_limits_m(beam: &CompositeBeam, diameter_m: f64) -> (f64, f64) {
        let h_c = concrete_height_m(beam);
        let s_max = (STUD_SPACING_MAX_HC_FACTOR * h_c).min(STUD_SPACING_MAX_ABS_M);
        (STUD_SPACING_MIN_DIAMETER_FACTOR * diameter_m, s_max)
    }

    /// 📐️ Section class (1–4) of the steel flange and web (EN 1993-1-1 Table 5.2 + EN 1994-1-1 §5.5), the outstand c = (b − t_w) / 2 − r and the web depth c = h − 2 t_f − 2 r without the root fillets.
    pub fn section_class(steel: &SteelSection, f_y_pa: f64) -> u8 {
        let eps = (235e6 / f_y_pa.max(1.0)).sqrt();
        let r = steel.root_radius_m();
        let c_flange = (steel.width_m / 2.0 - steel.tw_m / 2.0 - r).max(1e-6);
        let r_f = c_flange / steel.tf_m.max(1e-6);
        let class_f = if r_f <= 9.0 * eps { 1 } else if r_f <= 10.0 * eps { 2 } else if r_f <= 14.0 * eps { 3 } else { 4 };
        let r_w = (steel.h_w_m() - 2.0 * r).max(1e-6) / steel.tw_m.max(1e-6);
        let class_w = if r_w <= 72.0 * eps { 1 } else if r_w <= 83.0 * eps { 2 } else if r_w <= 124.0 * eps { 3 } else { 4 };
        class_f.max(class_w)
    }

    /// 📐️ Shear buckling utilisation h_w/t_w ≤ 72·ε/η with η = 1.2 for f_y ≤ 460 MPa (§6.2.2.3).
    pub fn shear_buckling_util(steel: &SteelSection, f_y_pa: f64) -> f64 {
        let eps = (235e6 / f_y_pa.max(1.0)).sqrt();
        let eta = if f_y_pa <= 460e6 { 1.2 } else { 1.0 };
        (steel.h_w_m() / steel.tw_m.max(1e-6)) / (72.0 * eps / eta)
    }

    /// 📐️ Plastic moment of the steel I-section from plate geometry [N·m].
    pub fn steel_mpl_from_plates_nm(steel: &SteelSection, f_y_pa: f64, gamma_m0: f64) -> f64 {
        isection(steel).w_pl() * f_y_pa / gamma_m0
    }

    /// 📐️ Sheeting construction-stage resistance per metre [N·m/m]: two idealised flanges A_p/2 at 0.9·h_p.
    pub fn sheeting_construction_mrd_nm_per_m(sheeting: &semio_s_artifact_norm_en1994_model::ProfiledSheeting, f_y_pa: f64) -> f64 {
        0.5 * sheeting.a_p_m2_per_m() * 0.9 * sheeting.height_m * f_y_pa
    }

    /// 📐️ Frequent SLS deflection limit [m]: EN 1990 Table A1.4 L/250; DIN EN 1990/NA L/300.
    pub fn deflection_limit_m(span_m: f64, annex: AnnexChoice) -> f64 {
        span_m / deflection_span_divisor(annex)
    }

    /// 📐️ Span divisor for [`deflection_limit_m`] (250 EN recommended, 300 DE NA).
    pub fn deflection_span_divisor(annex: AnnexChoice) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number("ndp.en1994.deflection-divisor", annex)
    }
}

pub mod part_1_2 {
    use crate::*;
    use semio_s_artifact_norm_en1994_model::FireRating;

    /// 🔥️ Required board insulation [m] — EN 1994-1-2 Table 4.2 (I-beams with composite slabs).
    /// Profile and section-height class select the row; DIN EN 1994-1-2/NA does not amend these values.
    pub fn insulation_thickness_m(rating: FireRating, deck_profile: semio_s_artifact_norm_en1994_model::SheetingProfile, steel_height_m: f64, _annex: AnnexChoice) -> f64 {
        let (r30, r60, r90, r120) = if deck_profile == semio_s_artifact_norm_en1994_model::SheetingProfile::ReEntrant {
            if steel_height_m >= 0.400 {
                (12.0, 22.0, 33.0, 48.0)
            } else if steel_height_m >= 0.300 {
                (12.0, 20.0, 32.0, 45.0)
            } else {
                (10.0, 18.0, 28.0, 40.0)
            }
        } else if steel_height_m >= 0.400 {
            (12.0, 20.0, 32.0, 45.0)
        } else if steel_height_m >= 0.300 {
            (10.0, 18.0, 28.0, 40.0)
        } else {
            (10.0, 15.0, 25.0, 35.0)
        };
        let base_mm = match rating {
            FireRating::Unrated => 0.0,
            FireRating::R30 => r30,
            FireRating::R60 => r60,
            FireRating::R90 => r90,
            FireRating::R120 => r120,
        };
        base_mm / 1000.0
    }

    /// 🔥️ Section factor A_p/V [1/m] of a beam with the top flange covered by the slab (three-sided exposure, 2h + b).
    pub fn section_factor_m_inv(steel: &semio_s_artifact_norm_en1994_model::SteelSection) -> f64 {
        (2.0 * steel.height_m + steel.width_m) / steel.a_m2.max(1e-9)
    }

    /// 🔥️ Required fire duration [min] for the rating.
    pub fn rating_minutes(rating: FireRating) -> f64 {
        match rating {
            FireRating::Unrated => 0.0,
            FireRating::R30 => 30.0,
            FireRating::R60 => 60.0,
            FireRating::R90 => 90.0,
            FireRating::R120 => 120.0,
        }
    }

    /// 🔥️ Protected steel temperature θ_a [°C] after the rating duration (EN 1993-1-2 Eq. 4.27, board data of [`catalogue::constants`]).
    pub fn steel_temperature_c(rating: FireRating, section_factor_m_inv: f64, insulation_m: f64) -> f64 {
        catalogue::protected_steel_temperature_c(rating_minutes(rating) * 60.0, section_factor_m_inv, insulation_m.max(1e-4), catalogue::constants::BOARD_LAMBDA, catalogue::constants::BOARD_C, catalogue::constants::BOARD_RHO)
    }

    /// 🧩 Effective thickness h_eff = h_c + h_p·b_0/pitch of the profiled deck of the beam (dovetail ribs 1.25·b_0/pitch): the solid slab the temperature field is computed for; the same idealisation as the slab insulation check, the closed formula of EN 1994-1-2 4.3.2 is not transcribed (`ndp.en1994.1-2.heff`).
    pub fn effective_thickness_m(beam: &CompositeBeam) -> f64 {
        let hp = beam.sheeting.height_m.min(beam.slab_thickness_m);
        let fill = (beam.sheeting.rib_width_m / catalogue::constants::RIB_PITCH_M).min(1.0);
        let fill = if beam.sheeting.profile == semio_s_artifact_norm_en1994_model::SheetingProfile::ReEntrant { (1.25 * fill).min(1.0) } else { fill };
        part_1_1::concrete_height_m(beam) + hp * fill
    }

    /// 🔥️ The fire inputs of one composite beam, (γ_M,fi = 1.0): the resolved profile, the effective width of the sagging region, the board of thickness `insulation_m`; the single source of the `mfi` check and of the temperature field of the scene.
    pub fn beam_fire(beam: &CompositeBeam, steel_f_y_pa: f64, insulation_m: f64) -> catalogue::beam_fire::Beam {
        let rebar = catalogue::column_fire::RebarProduct::from(beam.reinforcement_product);
        let steel = semio_s_artifact_norm_en1994_model::SteelSection::resolve(&beam.steel);
        let half = beam.spacing_m / 2.0;
        catalogue::beam_fire::Beam {
            sec: part_1_1::isection(&steel),
            fyd: steel_f_y_pa,
            ys: beam.slab_thickness_m,
            thermal_m: effective_thickness_m(beam),
            beff: catalogue::effective_width_m(catalogue::equivalent_span_m(beam.support, beam.span_m, false), 0.0, half, half, false),
            hc: part_1_1::concrete_height_m(beam),
            fcc: catalogue::fcc_beam_pa(beam.concrete_f_ck_pa, 1.0),
            am_v: section_factor_m_inv(&steel),
            board_m: insulation_m.max(1e-4),
            rebar,
        }
    }

    /// 🔥️ Critical steel temperature θ_cr [°C] for load level η_fi (EN 1993-1-2 §4.2.4).
    pub fn critical_temperature_c(eta_fi: f64) -> f64 {
        catalogue::critical_temperature_c(eta_fi)
    }
}

pub mod part_2 {
    use crate::*;
    use semio_s_artifact_norm_en1994_model::FatigueDetail;

    pub const STUD_DELTA_TAU_C_PA: f64 = 90e6;
    pub const STUD_N_REF: f64 = 2.0e6;
    pub const STUD_FATIGUE_SLOPE_M: f64 = 8.0;
    /// 🌉️ Detail category Δσ_c [Pa] at 2×10⁶ cycles (EN 1993-1-9 / EN 1994-2).
    pub fn steel_detail_category_pa(detail: FatigueDetail) -> f64 {
        match detail {
            FatigueDetail::StudWelded => 80e6,
            FatigueDetail::ShearConnector => 71e6,
            FatigueDetail::Reinforcement => 90e6,
            FatigueDetail::FlangeButtWeld => 71e6,
        }
    }

    /// 🌉️ S–N curve of the steel detail category `detail` (m = 3 from 2×10⁶ cycles) with the annex γ_Mf.
    pub fn steel_curve(detail: FatigueDetail, annex: AnnexChoice) -> catalogue::fatigue::Curve {
        catalogue::fatigue::Curve { delta_c_pa: steel_detail_category_pa(detail), n_ref: STUD_N_REF, m: 3.0, gamma: AnnexParams::for_annex(annex).gamma_mf }
    }

    /// 🌉️ S–N curve of the headed stud in shear (Δτ_c = 90 MPa, m = 8) with the annex γ_Mf,s.
    pub fn stud_curve(annex: AnnexChoice) -> catalogue::fatigue::Curve {
        catalogue::fatigue::Curve { delta_c_pa: STUD_DELTA_TAU_C_PA, n_ref: STUD_N_REF, m: STUD_FATIGUE_SLOPE_M, gamma: AnnexParams::for_annex(annex).gamma_mf }
    }

    pub fn steel_fatigue_resistance_pa(detail: FatigueDetail, n_cycles: f64, annex: AnnexChoice) -> f64 {
        steel_curve(detail, annex).resistance_pa(n_cycles)
    }

    /// 🌉️ Stud shear fatigue Δτ_R [Pa] with annex γ_Mf,s (EN 1994-2 §6.8 / EN 1993-1-9).
    pub fn stud_fatigue_resistance_pa(n_cycles: f64, annex: AnnexChoice) -> f64 {
        stud_curve(annex).resistance_pa(n_cycles)
    }
}

pub mod part_column {
    use crate::*;
    use crate::catalogue as cat;

    /// 📐️ N–M interaction vertex (N [N], M [N·m]) — EN 1994-1-1 Fig. 6.19 points A, C, D, B in that order.
    #[derive(Clone, Copy, Debug)]
    pub struct NmPoint {
        pub n_n: f64,
        pub m_nm: f64,
    }

    /// 🏛️ Equivalent-tube section: steel ring thickness from the stored steel area, 0.85·f_cd for encased kinds.
    pub fn section(col: &CompositeColumn, annex: AnnexChoice) -> cat::ColSection {
        let p = AnnexParams::for_annex(annex);
        let b = col.outer_size_m.max(1e-3);
        let t = ((b - (b * b - col.steel_a_m2).max(0.0).sqrt()) / 2.0).clamp(1e-4, b / 4.0);
        let factor = if col.kind == semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled { 1.0 } else { cat::constants::CONCRETE_BEAM_FACTOR };
        cat::ColSection { b, t, as_m2: col.reinforcement_as_m2, yr: (b / 2.0 - cat::constants::COLUMN_REBAR_EDGE_M).max(0.0), fyd: col.steel_f_y_pa / p.gamma_m0, fcd: factor * col.concrete_f_ck_pa / p.gamma_c, fsd: col.reinforcement_f_yk_pa / p.gamma_s }
    }

    /// 📐️ N_pl,Rd [N] from the stored areas (§6.7.3.2): filled A_a·f_yd + A_c·f_cd + A_s·f_sd; encased kinds with 0.85·f_cd.
    pub fn n_pl_rd_n(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let s = section(col, annex);
        col.steel_a_m2 * s.fyd + col.concrete_a_m2 * s.fcd + col.reinforcement_as_m2 * s.fsd
    }

    /// 📐️ N_pl,Rk [N] with all partial factors 1.0.
    pub fn n_pl_rk_n(col: &CompositeColumn) -> f64 {
        let factor = if col.kind == semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled { 1.0 } else { cat::constants::CONCRETE_BEAM_FACTOR };
        col.steel_a_m2 * col.steel_f_y_pa + factor * col.concrete_a_m2 * col.concrete_f_ck_pa + col.reinforcement_as_m2 * col.reinforcement_f_yk_pa
    }

    /// 📐️ Steel contribution ratio δ = A_a·f_yd/N_pl,Rd (§6.7.1(4)).
    pub fn steel_contribution(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        col.steel_a_m2 * section(col, annex).fyd / n_pl_rd_n(col, annex).max(1.0)
    }

    /// 📐️ Local buckling utilisation of Table 6.3: h/t ≤ 52·ε (filled), b/t ≤ 44·ε (partially encased), 40·ε otherwise.
    pub fn local_buckling_util(col: &CompositeColumn) -> f64 {
        let eps = (235e6 / col.steel_f_y_pa.max(1.0)).sqrt();
        let limit = match col.kind {
            semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled => 52.0 * eps,
            semio_s_artifact_norm_en1994_model::ColumnKind::PartiallyEncased => 44.0 * eps,
            _ => 40.0 * eps,
        };
        col.outer_size_m / col.wall_thickness_m.max(1e-6) / limit.max(1e-6)
    }

    /// 📐️ N_pm,Rd — concrete contribution (§6.7.3.2).
    pub fn n_pm_rd_n(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        col.concrete_a_m2 * section(col, annex).fcd
    }

    /// 🏛️ True encased or partially encased I-section geometry when the section leaves are set (review finding 31b).
    pub fn true_geometry(col: &CompositeColumn) -> Option<cat::encased::EncasedI> {
        let set = col.section_height_m > 1e-6 && col.section_width_m > 1e-6 && col.section_tw_m > 1e-6 && col.section_tf_m > 1e-6;
        let kind = match col.kind {
            semio_s_artifact_norm_en1994_model::ColumnKind::Encased => cat::encased::Encasement::Full,
            semio_s_artifact_norm_en1994_model::ColumnKind::PartiallyEncased => cat::encased::Encasement::BetweenFlanges,
            _ => return None,
        };
        set.then(|| cat::encased::EncasedI { kind, b_c: col.outer_size_m, h_c: col.outer_size_m, h: col.section_height_m, b: col.section_width_m, tw: col.section_tw_m, tf: col.section_tf_m, as_m2: col.reinforcement_as_m2, cover: if col.rebar_cover_m > 1e-6 { col.rebar_cover_m } else { cat::constants::COLUMN_REBAR_EDGE_M } })
    }

    fn polygon_abs(col: &CompositeColumn, annex: AnnexChoice) -> [(f64, f64); 4] {
        let s = section(col, annex);
        match true_geometry(col) {
            Some(g) => cat::encased::interaction_polygon(&g, &cat::encased::Strengths { fyd: s.fyd, fcd: s.fcd, fsd: s.fsd }),
            None => {
                let poly = cat::interaction_polygon(&s);
                let k = n_pl_rd_n(col, annex) / poly[0].0.max(1.0);
                poly.map(|(n, m)| (n * k, m))
            }
        }
    }

    /// 📐️ Interaction polygon A, C, D, B (Fig. 6.19) in absolute N [N] and M [N·m]: from the true section when its leaves are set, else the equivalent tube with the axial coordinates scaled to the stored areas.
    pub fn interaction_polygon(col: &CompositeColumn, annex: AnnexChoice) -> [NmPoint; 4] {
        polygon_abs(col, annex).map(|(n_n, m_nm)| NmPoint { n_n, m_nm })
    }

    /// 🌡️ Strain-compatibility reference curve `(N, M)` on the same axial scale as the polygon (the stored areas), never part of a verdict.
    pub fn strain_curve(col: &CompositeColumn, annex: AnnexChoice, points: usize) -> Vec<NmPoint> {
        let s = section(col, annex);
        let (strips, scale) = match true_geometry(col) {
            Some(g) => (g.strips(&cat::encased::Strengths { fyd: s.fyd, fcd: s.fcd, fsd: s.fsd }), 1.0),
            None => (cat::strain_curve::Strips::tube(&s), n_pl_rd_n(col, annex) / cat::interaction_polygon(&s)[0].0.max(1.0)),
        };
        strips.curve(points).into_iter().map(|(n, m)| NmPoint { n_n: n * scale, m_nm: m }).collect()
    }

    /// 📐️ M_pl,Rd at N = 0 (point B).
    pub fn m_pl_rd_nm(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        interaction_polygon(col, annex)[3].m_nm
    }

    /// 📐️ M_max,Rd (point D).
    pub fn m_max_rd_nm(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        interaction_polygon(col, annex)[2].m_nm
    }

    /// 📐️ α_M = 0.9 for S235..S355, 0.8 for S420/S460.
    pub fn alpha_m(steel_f_y_pa: f64) -> f64 {
        cat::alpha_m(steel_f_y_pa)
    }

    /// 📐️ μ_d(N_Ed) from the polygon.
    pub fn mu_d(col: &CompositeColumn, annex: AnnexChoice, n_ed_n: f64) -> f64 {
        cat::mu_d(&polygon_abs(col, annex), n_ed_n.max(0.0))
    }

    /// 📐️ Available M_Rd = α_M·μ_d·M_pl,Rd at axial force N_Ed.
    pub fn moment_resistance_at_n(col: &CompositeColumn, annex: AnnexChoice, n_ed_n: f64) -> f64 {
        alpha_m(col.steel_f_y_pa) * mu_d(col, annex, n_ed_n) * m_pl_rd_nm(col, annex)
    }

    /// 📐️ Utilisation M_Ed/(α_M·μ_d·M_pl,Rd) of §6.7.3.6 (axial force beyond N_pl,Rd counts as full utilisation).
    pub fn interaction_utilization(col: &CompositeColumn, annex: AnnexChoice, n_ed_n: f64, m_ed_nm: f64) -> f64 {
        let m_ed = m_ed_nm.abs();
        let m_rd = moment_resistance_at_n(col, annex, n_ed_n);
        if m_ed < 1.0 { return n_ed_n.max(0.0) / n_pl_rd_n(col, annex).max(1.0); }
        if m_rd < 1.0 { return f64::INFINITY.min(1.0e6); }
        m_ed / m_rd
    }

    /// 📐️ Second moment of the concrete part I_c [m⁴].
    pub fn i_concrete_m4(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let s = section(col, annex);
        if col.kind == semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled { (s.b - 2.0 * s.t).powi(4) / 12.0 } else { (s.b.powi(4) / 12.0 - col.i_m4).max(1e-9) }
    }

    /// 📐️ Second moment of the reinforcement I_s [m⁴].
    pub fn i_rebar_m4(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        let s = section(col, annex);
        col.reinforcement_as_m2 * s.yr * s.yr
    }

    /// 📐️ Effective stiffness (EI)_eff (§6.7.3.4(2)) with concrete modulus `ecm`.
    pub fn ei_eff_nmm2(col: &CompositeColumn, annex: AnnexChoice, ecm: f64) -> f64 {
        cat::ei_eff(col.i_m4, i_rebar_m4(col, annex), i_concrete_m4(col, annex), ecm, 0.6)
    }

    /// 📐️ N_cr [N] with buckling length L and concrete modulus `ecm`.
    pub fn n_cr_n(col: &CompositeColumn, annex: AnnexChoice, ecm: f64) -> f64 {
        std::f64::consts::PI.powi(2) * ei_eff_nmm2(col, annex, ecm) / col.length_m.max(0.1).powi(2)
    }

    /// 📐️ Relative slenderness λ̄ = √(N_pl,Rk/N_cr).
    pub fn relative_slenderness(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        (n_pl_rk_n(col) / n_cr_n(col, annex, cat::ecm_pa(col.concrete_f_ck_pa)).max(1.0)).sqrt()
    }

    /// 📐️ Imperfection α of the buckling curve (Table 6.5).
    pub fn alpha_curve(curve: &str) -> f64 {
        match curve {
            "a" => 0.21,
            "b" => 0.34,
            "c" => 0.49,
            _ => 0.76,
        }
    }

    /// 📐️ Reduction factor χ (§6.7.3.5).
    pub fn chi(col: &CompositeColumn, annex: AnnexChoice) -> f64 {
        cat::chi_column(relative_slenderness(col, annex), alpha_curve(col.buckling_curve.as_str()))
    }
}

pub mod part_slab {
    use crate::*;
    use crate::catalogue as cat;

    /// 🧩 Composite slab section per metre: sheeting plastic axis and centroid at half the profile depth.
    pub fn section(slab: &CompositeSlab, annex: AnnexChoice) -> cat::SlabSection {
        let p = AnnexParams::for_annex(annex);
        let hp = slab.sheeting.height_m;
        cat::SlabSection { h: slab.concrete_thickness_m, hp, ap: sheeting_area_m2_per_m(slab), ep: hp / 2.0, e: hp / 2.0, fyp_d: cat::constants::SHEETING_FYP / cat::constants::gamma_ap(), fcd: slab.f_ck_pa / p.gamma_c }
    }

    /// 🧩 Sheeting steel area per metre A_p [m²/m].
    pub fn sheeting_area_m2_per_m(slab: &CompositeSlab) -> f64 {
        slab.sheeting.a_p_m2_per_m()
    }

    /// 🧩 Sheeting-only plastic moment M_pa = A_p·f_yp,d·0.9·h_p/2 [N·m/m].
    pub fn m_pa_nm_per_m(slab: &CompositeSlab) -> f64 {
        0.5 * sheeting_area_m2_per_m(slab) * 0.9 * slab.sheeting.height_m * cat::constants::SHEETING_FYP / cat::constants::gamma_ap()
    }

    /// 🧩 Shear span L_s = L/4 for a uniformly loaded slab.
    pub fn shear_span_m(slab: &CompositeSlab) -> f64 {
        slab.span_m / 4.0
    }

    /// 🧩 m–k resistance V_l,Rd [N/m] (§9.7.3(4)).
    pub fn longitudinal_shear_mk_n_per_m(slab: &CompositeSlab) -> f64 {
        let s = section(slab, AnnexChoice::En);
        cat::slab_mk(s.dp(), s.ap, shear_span_m(slab), slab.m_factor * 1e6, slab.k_factor * 1e6, cat::constants::gamma_vs())
    }

    /// 🧩 Equivalent bond force N_c at section L_x [N/m] from the m–k test values (partial connection, §9.7.2).
    pub fn bond_force_n_per_m(slab: &CompositeSlab, annex: AnnexChoice, lx_m: f64) -> f64 {
        if slab.sheeting.ribs_parallel_to_beam { return 0.0; }
        let s = section(slab, annex);
        let z = cat::slab_bending(&s, m_pa_nm_per_m(slab), f64::INFINITY).2;
        cat::slab_tau_u(longitudinal_shear_mk_n_per_m(slab), z) * lx_m
    }

    /// 🧩 Sagging resistance m_Rd [N·m/m] at mid-span with partial connection and the bar reinforcement acting at depth d.
    pub fn bending_resistance_nm_per_m(slab: &CompositeSlab, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let s = section(slab, annex);
        let nc = bond_force_n_per_m(slab, annex, slab.span_m / 2.0);
        let (m_sheet, _, _) = cat::slab_bending(&s, m_pa_nm_per_m(slab), nc);
        let d = (slab.concrete_thickness_m - 0.03).max(0.05);
        let f_sd = cat::constants::REBAR_FSK / p.gamma_s;
        m_sheet + slab.as_m2_per_m * f_sd * 0.9 * d
    }

    /// 🧩 Rib width sum b_o per metre from the rib width and the assumed pitch.
    pub fn rib_width_sum_m_per_m(slab: &CompositeSlab) -> f64 {
        (slab.sheeting.rib_width_m / cat::constants::RIB_PITCH_M).min(1.0)
    }

    /// 🧩 Share of the rib depth filled with concrete: b_0/pitch for trapezoidal, 1.25·b_0/pitch (dovetail) for re-entrant sheeting.
    pub fn rib_fill_fraction(slab: &CompositeSlab) -> f64 {
        let f = rib_width_sum_m_per_m(slab);
        if slab.sheeting.profile == semio_s_artifact_norm_en1994_model::SheetingProfile::ReEntrant { (1.25 * f).min(1.0) } else { f }
    }

    /// 🧩 Vertical shear resistance V_v,Rd [N/m] (§9.7.5).
    pub fn vertical_shear_resistance_n_per_m(slab: &CompositeSlab, annex: AnnexChoice) -> f64 {
        let p = AnnexParams::for_annex(annex);
        let s = section(slab, annex);
        cat::slab_vertical_shear(rib_width_sum_m_per_m(slab), s.dp(), s.ap, slab.f_ck_pa, p.gamma_c)
    }
}
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;

#[cfg(any(test, feature = "oracle-wiring"))]
#[path = "🧪️tests/🎯️oracle-wiring/🦀️.rs"]
pub mod oracle_wiring;
//#endregion 🧪️ComplianceTests

#[path = "📇️catalogue/🦀️.rs"]
pub mod case_catalogue;

#[path = "🔩️studs/🦀️.rs"]
pub mod part_studs;
