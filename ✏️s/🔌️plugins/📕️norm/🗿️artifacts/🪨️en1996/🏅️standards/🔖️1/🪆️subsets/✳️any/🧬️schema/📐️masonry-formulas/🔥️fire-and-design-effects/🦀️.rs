//! ⚖️ Design effects of the EN 1996 masonry checks: the EN 1990 combinations of the wall load cases and their governing effects. See [`super`].

use super::*;

#[derive(Clone, Debug)]
/// 🧾 ULS design effects of one combination: normal force at top, mid-height and bottom [N], base shear [N], lateral pressure [Pa] and the combination labels.
pub struct DesignEffects {
    pub n_top: f64,
    pub n_mid: f64,
    pub n_bot: f64,
    pub v: f64,
    pub w_pa: f64,
    pub combo_en: String,
    pub combo_de: String,
}

/// ⚖️ The EN 1990 partial and combination factors of the annex from the registry (`ActionFactors`): persistent, or accidental when `exceptional`; the reliability class of the document scales γ by K_FI.
pub fn action_factors(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, exceptional: bool) -> semio_s_plugin_norm_compliance::ActionFactors {
    semio_s_plugin_norm_compliance::ActionFactors::new(annex, reliability_class, if exceptional { DesignSituation::Accidental } else { DesignSituation::Persistent })
}

/// ⚖️ Partial factor γ_Q of the variable action, 1.0 in the accidental and seismic situations.
pub fn gamma_q(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, exceptional: bool) -> f64 {
    action_factors(annex, reliability_class, exceptional).gamma_q_building()
}

/// ⚖️ Characteristic permanent and variable vertical actions of a load case [N]: slab and concentrated permanent, imposed, snow.
pub fn vertical_actions(lc: &WallLoadCase) -> (f64, f64, f64) {
    let g = lc.g_k_slab_n.max(0.0) + lc.concentrated.iter().map(|c| c.force_n.max(0.0)).sum::<f64>();
    (g, (lc.q_k_imposed_pa * lc.tributary_area_m2).max(0.0), (lc.q_k_snow_pa * lc.tributary_area_m2).max(0.0))
}

/// ⚖️ Form the EN 1990 ULS design effects from the characteristic actions and the self-weight.
pub fn design_effects(annex: AnnexChoice, wall: &MasonryWall, lc: &WallLoadCase, default_sit: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<DesignEffects> {
    let sit = DesignSituation::from(lc.design_situation);
    let exceptional = matches!(sit, DesignSituation::Accidental | DesignSituation::Seismic) || matches!(default_sit, DesignSituation::Accidental);
    let g_self = self_weight_n(wall);
    let (g_perm, q_imp, q_snow) = vertical_actions(lc);
    let w_k = (lc.q_p_wind_pa * lc.c_pe.abs()).max(0.0);
    let v_k = 0.5 * w_k * wall.height_m * wall.length_m + lc.h_k_earth_n.max(0.0);
    let factors = action_factors(annex, reliability_class, exceptional);
    let (yg_sup, yg_inf, yq) = (factors.gamma_g_sup(), factors.gamma_g_inf(), factors.gamma_q_building());
    let (psi0_snow, psi0_wind) = (factors.psi("snow").psi_0, factors.psi("wind").psi_0);
    let (psi_0, psi_1, psi_2) = psi_imposed(annex, lc.imposed_category.as_str()).unwrap_or((1.0, 1.0, 1.0));
    let lead_factor = match sit {
        DesignSituation::Accidental => psi_1,
        DesignSituation::Seismic => psi_2,
        _ if exceptional => psi_1,
        _ => 1.0,
    };
    let sit_label = lc.design_situation.as_str();
    let stations = |q_lead: f64, q_acc: f64| {
        let q = yq * (q_lead + q_acc);
        (
            yg_sup * g_perm + q,
            yg_sup * (g_perm + 0.5 * g_self) + q,
            yg_sup * (g_perm + g_self) + q,
        )
    };
    let mut out = Vec::new();
    let (nt, nm, nb) = stations(lead_factor * q_imp, psi0_snow * q_snow);
    out.push(DesignEffects {
        n_top: nt, n_mid: nm, n_bot: nb, v: yq * v_k, w_pa: yq * w_k,
        combo_en: format!("ULS max-N [{sit_label}/cat {}]: γ_G·G+γ_Q·Q_imposed+ψ₀,snow·Q_snow", lc.imposed_category),
        combo_de: format!("GZT max-N [{sit_label}/Kat. {}]: γ_G·G+γ_Q·Q_Nutz+ψ₀,Schnee·Q_Schnee", lc.imposed_category),
    });
    if q_snow > 0.0 {
        let (nt, nm, nb) = stations(lead_factor * q_snow, psi_0 * q_imp);
        out.push(DesignEffects {
            n_top: nt, n_mid: nm, n_bot: nb, v: yq * psi0_wind * v_k, w_pa: yq * psi0_wind * w_k,
            combo_en: "ULS max-N: γ_G·G+γ_Q·Q_snow+ψ₀·Q_imposed".into(),
            combo_de: "GZT max-N: γ_G·G+γ_Q·Q_Schnee+ψ₀·Q_Nutz".into(),
        });
    }
    out.push(DesignEffects {
        n_top: yg_inf * g_perm,
        n_mid: yg_inf * (g_perm + 0.5 * g_self),
        n_bot: yg_inf * (g_perm + g_self),
        v: yq * v_k,
        w_pa: yq * w_k,
        combo_en: "ULS min-N/max-H: γ_G,inf·G+γ_Q·(W+H_earth)".into(),
        combo_de: "GZT min-N/max-H: γ_G,inf·G+γ_Q·(W+H_Erde)".into(),
    });
    out
}

pub fn governing_compression(effects: &[DesignEffects]) -> &DesignEffects {
    effects.iter().max_by(|a, b| a.n_bot.total_cmp(&b.n_bot)).unwrap_or(&effects[0])
}

pub fn governing_shear(effects: &[DesignEffects]) -> &DesignEffects {
    effects.iter().max_by(|a, b| (a.v / a.n_mid.max(1.0)).total_cmp(&(b.v / b.n_mid.max(1.0)))).unwrap_or(&effects[0])
}

pub fn governing_lateral(effects: &[DesignEffects]) -> &DesignEffects {
    effects.iter().max_by(|a, b| a.w_pa.total_cmp(&b.w_pa)).unwrap_or(&effects[0])
}
