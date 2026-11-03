//! 🧭️ EN 1997 design approach: partial-factor sets A1/A2, M1/M2, R1-R4 (EN 1997-1 Annex A) and DIN 1054:2021-04 Tables A 2.1-A 2.3 (values carried over from the withdrawn 2010-12 edition, unverified against 2021-04) per design situation, plus the EQU/UPL/HYD factors.

use super::*;

/// ⚖️ Design approach / GEO combination per EN 1997-1 §2.4.7 and DIN 1054; `Geo3` is the overall-stability set (DIN 1054 GZ 1C, A2 + M2 + R3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignApproach {
    Da1Str,
    Da1Geo,
    Da2,
    Da2Star,
    Da3,
    Geo3,
}

/// 📊️ Partial factors resolved for one approach, annex and design situation; all values dimensionless.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub gamma_g: f64,
    pub gamma_q: f64,
    pub gamma_r_v: f64,
    pub gamma_r_h: f64,
    pub gamma_c: f64,
    pub gamma_phi: f64,
    pub gamma_gamma: f64,
    pub gamma_cu: f64,
    pub gamma_b: f64,
    pub gamma_s: f64,
    pub gamma_t: f64,
    pub gamma_st: f64,
    pub gamma_r_e: f64,
    pub gamma_a: f64,
}

/// 📚️ One row of the resistance partial-factor sets R1-R4 of EN 1997-1 Tables A.5-A.14 (recommended values) and DIN 1054 Table A 2.3.
#[derive(Clone, Copy)]
struct Resistance {
    r_v: f64,
    r_h: f64,
    r_e: f64,
    b: f64,
    s: f64,
    t: f64,
    st: f64,
    a: f64,
}

fn resistance(set: &str, annex: AnnexChoice) -> Resistance {
    let n = |symbol: &str| semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1997.{set}.{symbol}"));
    Resistance { r_v: n("rv"), r_h: n("rh"), r_e: n("re"), b: n("b"), s: n("s"), t: n("t"), st: n("st"), a: semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1997.{set}.a"), annex) }
}

#[derive(Clone, Copy)]
struct Material {
    phi: f64,
    c: f64,
    cu: f64,
    gamma: f64,
}

fn material(set: &str) -> Material {
    let n = |symbol: &str| semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1997.{set}.{symbol}"));
    Material { phi: n("phi"), c: n("c"), cu: n("cu"), gamma: n("gamma") }
}

fn action(set: &str) -> (f64, f64) {
    (semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1997.{set}.g")), semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1997.{set}.q")))
}

fn de(id: &str) -> f64 {
    semio_s_plugin_norm_compliance::ndp_number(id, AnnexChoice::De)
}

fn situation_key(sit: DesignSituation) -> char {
    match sit {
        DesignSituation::Persistent => 'p',
        DesignSituation::Transient => 't',
        DesignSituation::Accidental => 'a',
    }
}

fn assemble(g: f64, q: f64, m: Material, r: Resistance) -> AnnexParams {
    AnnexParams {
        gamma_g: g,
        gamma_q: q,
        gamma_r_v: r.r_v,
        gamma_r_h: r.r_h,
        gamma_c: m.c,
        gamma_phi: m.phi,
        gamma_gamma: m.gamma,
        gamma_cu: m.cu,
        gamma_b: r.b,
        gamma_s: r.s,
        gamma_t: r.t,
        gamma_st: r.st,
        gamma_r_e: r.r_e,
        gamma_a: r.a,
    }
}

fn de_actions(sit: DesignSituation) -> (f64, f64) {
    let k = situation_key(sit);
    (de(&format!("ndp.en1997.de.act.{k}.g")), de(&format!("ndp.en1997.de.act.{k}.q")))
}

fn de_resistance(sit: DesignSituation) -> Resistance {
    let k = situation_key(sit);
    let n = |symbol: &str| de(&format!("ndp.en1997.de.res.{symbol}"));
    Resistance { r_v: de(&format!("ndp.en1997.de.res.{k}.rv")), r_h: n("rh"), r_e: de(&format!("ndp.en1997.de.res.{k}.re")), b: n("b"), s: n("s"), t: n("t"), st: n("st"), a: n("a") }
}

fn de_soil(sit: DesignSituation) -> Material {
    let f = de(&format!("ndp.en1997.de.soil.{}.f", situation_key(sit)));
    Material { phi: f, c: f, cu: f, gamma: material("m1").gamma }
}

impl DesignApproach {
    pub fn label(self, annex: AnnexChoice) -> &'static str {
        match (self, annex) {
            (Self::Da2 | Self::Da2Star, AnnexChoice::De) => "DA2*",
            (Self::Da2, AnnexChoice::En) => "DA2",
            (Self::Da2Star, AnnexChoice::En) => "DA2*",
            (Self::Geo3, _) => "GEO-3",
            (Self::Da1Str, _) => "DA1-C1",
            (Self::Da1Geo, _) => "DA1-C2",
            (Self::Da3, _) => "DA3",
        }
    }

    pub fn annex_params(self, annex: AnnexChoice) -> AnnexParams {
        self.resolve(annex, DesignSituation::Persistent)
    }

    /// 🛡️ The partial factors of the approach (EN 1997-1 2.4.7.3 and Annex A). The K_FI of EN 1990 Annex B does not scale them: EN 1997-1 differentiates by geotechnical category (2.1) and fixes γ_G/γ_Q in Tables A.3 and A.4; unverified against the clause text, which is not in the repository.
    pub fn resolve(self, annex: AnnexChoice, sit: DesignSituation) -> AnnexParams {
        if annex == AnnexChoice::En && sit == DesignSituation::Accidental {
            let (g, q) = action("a-acc");
            return assemble(g, q, material("m1"), resistance("r1", annex));
        }
        match (annex, self) {
            (AnnexChoice::De, Self::Da2 | Self::Da2Star) => {
                let (g, q) = de_actions(sit);
                assemble(g, q, material("m1"), de_resistance(sit))
            }
            (AnnexChoice::De, Self::Da3) => {
                let (g, q) = de_actions(sit);
                assemble(g, q, de_soil(sit), resistance("r3", annex))
            }
            (AnnexChoice::De, Self::Geo3) => {
                let q = de(&format!("ndp.en1997.de.geo3.{}.q", situation_key(sit)));
                assemble(action("a2").0, q, de_soil(sit), resistance("r3", annex))
            }
            (_, Self::Da1Str) => assemble(action("a1").0, action("a1").1, material("m1"), resistance("r1", annex)),
            (_, Self::Da1Geo) => {
                let r4 = resistance("r4", annex);
                assemble(action("a2").0, action("a2").1, material("m2"), Resistance { b: r4.b, s: r4.s, t: r4.t, st: r4.st, ..resistance("r1", annex) })
            }
            (_, Self::Da2 | Self::Da2Star) => assemble(action("a1").0, action("a1").1, material("m1"), resistance("r2", annex)),
            (_, Self::Da3) => assemble(action("a1").0, action("a1").1, material("m2"), resistance("r3", annex)),
            (_, Self::Geo3) => assemble(action("a2").0, action("a2").1, material("m2"), resistance("r3", annex)),
        }
    }
}

/// 🔖️ Registry ids of the partial-factor sets (`ndp.en1997.*`) an annex resolves its factors from; the DIN 1054 sets belong to the German annex only.
pub fn factor_ndp_ids(annex: AnnexChoice) -> Vec<&'static str> {
    const SETS: [&str; 11] = ["a1.", "a2.", "a-acc.", "m1.", "m2.", "r1.", "r2.", "r3.", "r4.", "upl.", "de."];
    semio_s_plugin_norm_compliance::NDP_TABLE
        .iter()
        .filter(|entry| SETS.iter().any(|set| entry.id.strip_prefix("ndp.en1997.").is_some_and(|rest| rest.starts_with(set))) && (annex == AnnexChoice::De || !entry.id.starts_with("ndp.en1997.de.")))
        .map(|entry| entry.id)
        .collect()
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
    match value.trim().to_ascii_lowercase().replace(['-', '_'], "").as_str() {
        "bst" | "transient" | "vorübergehend" | "voruebergehend" => DesignSituation::Transient,
        "bsa" | "accidental" | "aussergewöhnlich" | "außergewöhnlich" | "aussergewoehnlich" => DesignSituation::Accidental,
        _ => DesignSituation::Persistent,
    }
}

/// 🧭️ Parses a design-approach key (`da1str`/`da1c1`, `da1geo`/`da1c2`, `da2`/`geo2`, `da2star`/`da2s`/`da2*`, `da3`, `geo3`); unknown keys fall back to DA1 combination 1.
pub fn parse_design_approach(value: &str) -> DesignApproach {
    match value.trim().to_ascii_lowercase().replace(['-', '_', ' '], "").as_str() {
        "da1geo" | "da1c2" => DesignApproach::Da1Geo,
        "da2" | "geo2" => DesignApproach::Da2,
        "da2star" | "da2s" | "da2*" => DesignApproach::Da2Star,
        "da3" => DesignApproach::Da3,
        "geo3" => DesignApproach::Geo3,
        _ => DesignApproach::Da1Str,
    }
}

/// ⚖️ Partial factors for approach + annex + design situation (EN 1997-1 Annex A; DIN 1054:2021-04 Tables A 2.1-A 2.3 (values carried over from the withdrawn 2010-12 edition, unverified against 2021-04) for the German annex).
pub fn resolve_params(approach: DesignApproach, annex: AnnexChoice, situation: &str) -> AnnexParams {
    approach.resolve(annex, parse_design_situation(situation))
}

/// ⚖️ EQU, UPL and HYD partial factors (EN 1997-1 Table A.1, A.2, A.17; DIN 1054:2021-04 Table A 2.1 (2010-12 values, unverified)).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UplParams {
    pub gamma_g_stb: f64,
    pub gamma_g_dst: f64,
    pub gamma_q_dst: f64,
    pub gamma_hyd: f64,
    pub gamma_equ_stb: f64,
    pub gamma_equ_dst: f64,
    pub gamma_equ_q: f64,
    pub gamma_hyd_stb: f64,
}

pub fn resolve_upl_params(annex: AnnexChoice, situation: &str) -> UplParams {
    let k = situation_key(parse_design_situation(situation));
    let n = |symbol: &str| semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1997.upl.{k}.{symbol}"), annex);
    UplParams { gamma_g_stb: n("gstb"), gamma_g_dst: n("gdst"), gamma_q_dst: n("qdst"), gamma_hyd: n("hyd"), gamma_equ_stb: n("equstb"), gamma_equ_dst: n("equdst"), gamma_equ_q: n("equq"), gamma_hyd_stb: n("hydstb") }
}
