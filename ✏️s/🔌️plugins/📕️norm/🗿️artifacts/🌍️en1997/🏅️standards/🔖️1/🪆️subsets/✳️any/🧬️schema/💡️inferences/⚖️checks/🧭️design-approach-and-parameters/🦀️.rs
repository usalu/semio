//! 🧭️ EN 1997 design approach: national annex factors, design situations and the resolved partial-factor and uplift parameters.

use super::*;

pub mod na_de {
    pub fn gamma_c() -> f64 { 1.4 }
    pub fn gamma_phi() -> f64 { 1.25 }
    pub fn gamma_gamma() -> f64 { 1.0 }
}

/// ⚖️ Design approach / GEO combination per EN 1997-1 §2.4.7 and DIN 1054.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignApproach {
    Da1Str,
    Da1Geo,
    Da2,
    Da3,
    Geo2,
    Geo3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub gamma_g: f64,
    pub gamma_q: f64,
    pub gamma_r_v: f64,
    pub gamma_r_h: f64,
    pub gamma_c: f64,
    pub gamma_phi: f64,
    pub gamma_gamma: f64,
    pub gamma_b: f64,
    pub gamma_s: f64,
    pub gamma_st: f64,
    pub gamma_r_e: f64,
}

impl DesignApproach {
    pub fn label(self, annex: AnnexChoice) -> &'static str {
        match (self, annex) {
            (Self::Da2 | Self::Geo2, AnnexChoice::De) => "DA2*/GEO-2",
            (Self::Da2 | Self::Geo2, AnnexChoice::En) => "DA2",
            (Self::Geo3, _) => "GEO-3",
            (Self::Da1Str, _) => "DA1-C1",
            (Self::Da1Geo, _) => "DA1-C2",
            (Self::Da3, _) => "DA3",
        }
    }

    pub fn annex_params(self, annex: AnnexChoice) -> AnnexParams {
        match (self, annex) {
            (Self::Da2 | Self::Geo2, AnnexChoice::De) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.4, gamma_r_h: 1.1,
                gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Da2 | Self::Geo2, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Geo3, AnnexChoice::De) => AnnexParams {
                gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Geo3, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Da1Str, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.0, gamma_s: 1.0, gamma_st: 1.0, gamma_r_e: 1.0,
            },
            (Self::Da1Str, _) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(),
                gamma_b: 1.0, gamma_s: 1.0, gamma_st: 1.0, gamma_r_e: 1.0,
            },
            (Self::Da1Geo, _) => AnnexParams {
                gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.4, gamma_r_h: 1.1,
                gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Da3, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.15, gamma_s: 1.15, gamma_st: 1.15, gamma_r_e: 1.1,
            },
            (Self::Da3, _) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(),
                gamma_b: 1.15, gamma_s: 1.15, gamma_st: 1.15, gamma_r_e: 1.1,
            },
        }
    }
}

/// ⚖️ DIN 1054 design situation (BS-P / BS-T / BS-A).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum DesignSituation {
    Persistent,
    Transient,
    Accidental,
}

pub fn parse_design_situation(value: &str) -> DesignSituation {
    match value.trim().to_ascii_lowercase().replace('-', "").replace('_', "").as_str() {
        "bst" | "transient" | "vorübergehend" | "voruebergehend" => DesignSituation::Transient,
        "bsa" | "accidental" | "aussergewöhnlich" | "außergewöhnlich" | "aussergewoehnlich" => DesignSituation::Accidental,
        _ => DesignSituation::Persistent,
    }
}

/// ⚖️ Resolve partial factors for approach + annex + DIN 1054 design situation.
///
/// DA2* (DE + DA2/GEO-2): characteristic soil (γ_φ′=γ_c′=1), factors on actions (γ_G, γ_Q)
/// and resistances (γ_R,·) — single coupled verification, not DA1 Combination 1+2.
pub fn resolve_params(approach: DesignApproach, annex: AnnexChoice, situation: &str) -> AnnexParams {
    let mut p = approach.annex_params(annex);
    let sit = parse_design_situation(situation);
    let da2_star = annex == AnnexChoice::De && matches!(approach, DesignApproach::Da2 | DesignApproach::Geo2);
    if da2_star {
        p.gamma_c = 1.0;
        p.gamma_phi = 1.0;
        p.gamma_gamma = 1.0;
    }
    match sit {
        DesignSituation::Persistent => {
            if annex == AnnexChoice::De {
                match approach {
                    DesignApproach::Da2 | DesignApproach::Geo2 => {
                        p.gamma_g = 1.35;
                        p.gamma_q = 1.50;
                        p.gamma_r_v = 1.40;
                        p.gamma_r_h = 1.10;
                    }
                    DesignApproach::Geo3 | DesignApproach::Da1Geo => {
                        p.gamma_g = 1.00;
                        p.gamma_q = 1.30;
                    }
                    _ => {}
                }
            }
        }
        DesignSituation::Transient => {
            if annex == AnnexChoice::De {
                p.gamma_g = 1.20;
                p.gamma_q = 1.30;
                if matches!(approach, DesignApproach::Da2 | DesignApproach::Geo2) {
                    p.gamma_r_v = 1.30;
                    p.gamma_r_h = 1.10;
                    p.gamma_c = 1.0;
                    p.gamma_phi = 1.0;
                }
                if matches!(approach, DesignApproach::Geo3 | DesignApproach::Da3) {
                    p.gamma_phi = (p.gamma_phi * 0.96).max(1.0);
                    p.gamma_c = (p.gamma_c * 0.96).max(1.0);
                }
            } else {
                p.gamma_g = 1.20;
                p.gamma_q = 1.30;
            }
        }
        DesignSituation::Accidental => {
            p.gamma_g = 1.00;
            p.gamma_q = 1.00;
            if annex == AnnexChoice::De && matches!(approach, DesignApproach::Da2 | DesignApproach::Geo2) {
                p.gamma_r_v = 1.20;
                p.gamma_r_h = 1.10;
                p.gamma_c = 1.0;
                p.gamma_phi = 1.0;
            }
        }
    }
    p
}

/// ⚖️ UPL/HYD partial factors (DIN 1054 Table A.2.1 / EN 1997-1 §10).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UplParams {
    pub gamma_g_stb: f64,
    pub gamma_g_dst: f64,
    pub gamma_q_dst: f64,
    pub gamma_hyd: f64,
}

pub fn resolve_upl_params(annex: AnnexChoice, situation: &str) -> UplParams {
    let sit = parse_design_situation(situation);
    match (annex, sit) {
        (AnnexChoice::De, DesignSituation::Transient) => UplParams {
            gamma_g_stb: 0.95, gamma_g_dst: 1.05, gamma_q_dst: 1.30, gamma_hyd: 1.30,
        },
        (AnnexChoice::De, DesignSituation::Accidental) => UplParams {
            gamma_g_stb: 1.00, gamma_g_dst: 1.00, gamma_q_dst: 1.00, gamma_hyd: 1.00,
        },
        (AnnexChoice::De, _) => UplParams {
            gamma_g_stb: 0.90, gamma_g_dst: 1.10, gamma_q_dst: 1.50, gamma_hyd: 1.50,
        },
        (_, DesignSituation::Transient) => UplParams {
            gamma_g_stb: 0.95, gamma_g_dst: 1.05, gamma_q_dst: 1.30, gamma_hyd: 1.30,
        },
        (_, DesignSituation::Accidental) => UplParams {
            gamma_g_stb: 1.00, gamma_g_dst: 1.00, gamma_q_dst: 1.00, gamma_hyd: 1.00,
        },
        _ => UplParams {
            gamma_g_stb: 0.90, gamma_g_dst: 1.10, gamma_q_dst: 1.50, gamma_hyd: 1.50,
        },
    }
}

pub fn parse_design_approach(value: &str) -> DesignApproach {
    match value.to_ascii_lowercase().as_str() {
        "da1geo" => DesignApproach::Da1Geo,
        "da2" | "geo2" => DesignApproach::Da2,
        "da3" => DesignApproach::Da3,
        "geo3" => DesignApproach::Geo3,
        _ => DesignApproach::Da1Str,
    }
}
