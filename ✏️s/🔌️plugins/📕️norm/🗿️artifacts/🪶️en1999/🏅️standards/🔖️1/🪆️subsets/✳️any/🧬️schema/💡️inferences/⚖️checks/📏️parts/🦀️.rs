//! 📏️ EN 1999 clause parts: the national annex parameters, the EN 1990 combinations and the clause tables of EN 1999-1-1 to 1-5.

use semio_s_artifact_norm_en1999_model::{AluminiumConnection, AluminiumShell, ColdFormedSheet, FatigueCategory, FatigueDetail, PatchLoad, RingStiffener, SupportCondition};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult};

//#region 🔖️ComplianceHelpers
pub mod na_de {
    use semio_s_plugin_norm_compliance::{ndp_number, AnnexChoice};
    pub const E_PA: f64 = 70_000.0e6;
    pub const G_PA: f64 = 27_000.0e6;
    pub const NU: f64 = 0.3;
    pub const STEFAN_BOLTZMANN: f64 = 5.67e-8;

    /// 🇩🇪️ Partial factors, combination factors and serviceability limits of one annex choice, read from the shared NDP registry (`ndp.en1999.*`; EN 1999-1-1 §2.4, EN 1990 A1.2, §7.2).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexParams {
        pub choice: AnnexChoice,
        pub gamma_m1: f64,
        pub gamma_m2: f64,
        pub gamma_mw: f64,
        pub gamma_m_fi: f64,
        pub gamma_g: f64,
        pub gamma_q: f64,
        pub xi: f64,
        pub gamma_mf: f64,
        pub gamma_ff: f64,
        pub deflection_ratio: f64,
        pub stress_ratio: f64,
    }

    impl AnnexParams {
        /// 🔖️ Registry ids of the parameters; a check cites exactly the ones its limits and resistances use.
        pub const GAMMA_M1: &'static str = "ndp.en1999.gamma-m1";
        pub const GAMMA_M2: &'static str = "ndp.en1999.gamma-m2";
        pub const GAMMA_MW: &'static str = "ndp.en1999.gamma-mw";
        pub const GAMMA_M_FI: &'static str = "ndp.en1999.gamma-m-fi";
        pub const GAMMA_MF: &'static str = "ndp.en1999.gamma-mf";
        pub const GAMMA_FF: &'static str = "ndp.en1999.gamma-ff";
        pub const DEFLECTION_RATIO: &'static str = "ndp.en1999.deflection-ratio";
        pub const STRESS_RATIO: &'static str = "ndp.en1999.stress-ratio";

        pub fn en() -> Self {
            Self::for_choice(AnnexChoice::En)
        }
        /// 🇩🇪️ DIN EN 1999-1-1/NA values of the registry.
        pub fn de() -> Self {
            Self::for_choice(AnnexChoice::De)
        }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            let n = |id: &str| ndp_number(id, choice);
            semio_s_plugin_norm_compliance::record_ndp_reads(|| Self { choice, gamma_m1: n(Self::GAMMA_M1), gamma_m2: n(Self::GAMMA_M2), gamma_mw: n(Self::GAMMA_MW), gamma_m_fi: n(Self::GAMMA_M_FI), gamma_g: n("ndp.en1999.gamma-g"), gamma_q: n("ndp.en1999.gamma-q"), xi: n("ndp.en1999.xi"), gamma_mf: n(Self::GAMMA_MF), gamma_ff: n(Self::GAMMA_FF), deflection_ratio: n(Self::DEFLECTION_RATIO), stress_ratio: n(Self::STRESS_RATIO) }).0
        }
    }
}

pub mod part_en1990 {
    use crate::*;
    use semio_s_artifact_norm_en1999_model::{AluminiumMember, MemberAction};

    #[derive(Clone, Copy, Debug)]
    pub struct PsiRow { pub psi_0: f64, pub psi_1: f64, pub psi_2: f64 }

    /// 🎚️ EN 1990 Table A1.1 combination factors read from the shared registry (`ndp.en1990.psi.*`, identical for the recommended values and DIN EN 1990/NA); fire and unknown categories use the EN 1999 entries.
    pub fn psi_row(category: semio_s_artifact_norm_en1999_model::ActionCategory) -> PsiRow {
        use semio_s_artifact_norm_en1999_model::ActionCategory as C;
        let key = match category {
            C::Office | C::SelfWeight => "ndp.en1990.psi.office",
            C::Snow => "ndp.en1990.psi.snow",
            C::Wind => "ndp.en1990.psi.wind",
        };
        let n = |index: u8| semio_s_plugin_norm_compliance::ndp_common(&format!("{key}.{index}"));
        PsiRow { psi_0: n(0), psi_1: n(1), psi_2: n(2) }
    }

    #[derive(Clone, Debug)]
    pub struct GoverningEffects {
        pub action_id: String,
        pub combination: &'static str,
        pub n_ed: f64,
        pub v_y_ed: f64,
        pub v_z_ed: f64,
        pub m_y_ed: f64,
        pub m_z_ed: f64,
        pub t_ed: f64,
        pub f_loc_ed: f64,
    }

    type Fx = (f64, f64, f64, f64, f64, f64, f64);

    fn span_factor(support: SupportCondition) -> (f64, f64) {
        match support {
            SupportCondition::Cantilever => (0.5, 1.0),
            SupportCondition::Continuous => (1.0 / 12.0, 0.5),
            SupportCondition::SimplySupported => (1.0 / 8.0, 0.5),
        }
    }

    /// 📐 Characteristic internal forces of one action: `udl` spreads the line loads over the span, `external` reads them as a concentrated load at mid-span (tip of a cantilever); the given forces are added in both cases.
    pub fn characteristic_effects(member: &AluminiumMember, a: &MemberAction) -> Fx {
        if a.source == semio_s_artifact_norm_en1999_model::ActionSource::Udl {
            let l = member.length.max(1e-6);
            let w = a.g_k_line + a.q_k_line;
            let (km, kv) = span_factor(member.support);
            (a.n_k, a.v_y_k, w * l * kv + a.v_z_k, w * l * l * km + a.m_y_k, a.m_z_k, a.torsion_k, a.local_load_k)
        } else {
            let l = member.length.max(1e-6);
            let w = a.g_k_line + a.q_k_line;
            let (km, kv) = match member.support {
                SupportCondition::Cantilever => (1.0, 1.0),
                SupportCondition::Continuous => (1.0 / 8.0, 0.5),
                SupportCondition::SimplySupported => (1.0 / 4.0, 0.5),
            };
            (a.n_k, a.v_y_k, w * kv + a.v_z_k, w * l * km + a.m_y_k, a.m_z_k, a.torsion_k, a.local_load_k)
        }
    }

    fn scale(e: Fx, f: f64) -> Fx { (e.0 * f, e.1 * f, e.2 * f, e.3 * f, e.4 * f, e.5 * f, e.6 * f) }
    fn add(a: Fx, b: Fx) -> Fx { (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4, a.5 + b.5, a.6 + b.6) }

    fn split(member: &AluminiumMember) -> (Vec<&MemberAction>, Vec<&MemberAction>) {
        (
            member.actions.iter().filter(|a| a.kind == semio_s_artifact_norm_en1999_model::ActionKind::Permanent).collect(),
            member.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1999_model::ActionKind::Permanent && a.kind != semio_s_artifact_norm_en1999_model::ActionKind::Fire).collect(),
        )
    }

    fn sum_permanent(member: &AluminiumMember, permanents: &[&MemberAction]) -> Fx {
        permanents.iter().fold((0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), |acc, a| add(acc, characteristic_effects(member, a)))
    }

    fn effects(action_id: String, combination: &'static str, e: Fx) -> GoverningEffects {
        GoverningEffects { action_id, combination, n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4, t_ed: e.5, f_loc_ed: e.6 }
    }

    fn permanent_id(permanents: &[&MemberAction]) -> String {
        permanents.first().map(|a| a.id.clone()).unwrap_or_else(|| "G".into())
    }

    /// ⚖️ EN 1990 (6.10a) and (6.10b) for every leading variable action; the checks keep the worst utilization over this set.
    pub fn uls_combinations(member: &AluminiumMember, params: &na_de::AnnexParams, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<GoverningEffects> {
        let k_fi = reliability_class.k_fi(params.choice);
        let (gamma_g, gamma_q) = (params.gamma_g * k_fi, params.gamma_q * k_fi);
        let (permanents, variables) = split(member);
        let g = sum_permanent(member, &permanents);
        if variables.is_empty() {
            return vec![effects(permanent_id(&permanents), "uls-g", scale(g, gamma_g))];
        }
        let mut out = Vec::new();
        for (i, lead) in variables.iter().enumerate() {
            let lead_e = characteristic_effects(member, lead);
            let mut companions = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            for (j, other) in variables.iter().enumerate() {
                if i != j {
                    companions = add(companions, scale(characteristic_effects(member, other), gamma_q * psi_row(other.category).psi_0));
                }
            }
            let psi_lead = psi_row(lead.category).psi_0;
            let a610 = add(add(scale(g, gamma_g), scale(lead_e, gamma_q * psi_lead)), companions);
            let b610 = add(add(scale(g, params.xi * gamma_g), scale(lead_e, gamma_q)), companions);
            out.push(effects(lead.id.clone(), "uls-610a", a610));
            out.push(effects(lead.id.clone(), "uls-610b", b610));
        }
        out
    }

    /// 🔥 Accidental combination of EN 1990 (6.11b): G + Σψ2·Q.
    pub fn fire_member_effects(member: &AluminiumMember) -> GoverningEffects {
        let (permanents, variables) = split(member);
        let mut e = sum_permanent(member, &permanents);
        let mut lead_id = permanent_id(&permanents);
        for a in &variables {
            e = add(e, scale(characteristic_effects(member, a), psi_row(a.category).psi_2));
            lead_id = a.id.clone();
        }
        effects(lead_id, "fire", e)
    }

    /// 🪟 EN 1990 serviceability combinations (`characteristic`, `frequent`, `quasiPermanent`) for every leading variable action.
    pub fn sls_combinations(member: &AluminiumMember, kind: &str) -> Vec<GoverningEffects> {
        let (permanents, variables) = split(member);
        let g = sum_permanent(member, &permanents);
        let name: &'static str = match kind { "frequent" => "sls-freq", "quasiPermanent" => "sls-qp", _ => "sls-char" };
        if variables.is_empty() {
            return vec![effects(permanent_id(&permanents), name, g)];
        }
        let mut out = Vec::new();
        for (i, lead) in variables.iter().enumerate() {
            let lead_e = characteristic_effects(member, lead);
            let psi = psi_row(lead.category);
            let mut e = g;
            e = match kind {
                "frequent" => add(e, scale(lead_e, psi.psi_1)),
                "quasiPermanent" => add(e, scale(lead_e, psi.psi_2)),
                _ => add(e, lead_e),
            };
            for (j, other) in variables.iter().enumerate() {
                if i == j { continue; }
                let po = psi_row(other.category);
                let f = match kind { "frequent" | "quasiPermanent" => po.psi_2, _ => po.psi_0 };
                e = add(e, scale(characteristic_effects(member, other), f));
            }
            out.push(effects(lead.id.clone(), name, e));
        }
        out
    }

    /// 🧰 A synthetic simply supported member that carries the actions of a connection, sheet or shell through the member combinations.
    pub fn proxy_member(id: &str, material_id: &str, length: f64, actions: &[MemberAction]) -> AluminiumMember {
        let l = length.max(1e-6);
        AluminiumMember {
            id: id.to_owned(),
            section_id: String::new(),
            material_id: material_id.to_owned(),
            length: l,
            support: SupportCondition::SimplySupported,
            buckling_length_y: l,
            buckling_length_z: l,
            buckling_length_t: l,
            ltb_length: l,
            c1: 1.0,
            restrained_ltb: true,
            bearing_length: 0.0,
            transverse_weld: false,
            transverse_weld_offset: 0.0,
            actions: actions.to_vec(),
        }
    }

    pub fn connection_proxy(conn: &AluminiumConnection) -> AluminiumMember {
        proxy_member(&conn.id, &conn.material_id, 1.0, &conn.actions)
    }

    pub fn sheet_proxy(sheet: &ColdFormedSheet) -> AluminiumMember {
        proxy_member(&sheet.id, &sheet.material_id, sheet.span, &sheet.actions)
    }

    pub fn shell_proxy(shell: &AluminiumShell) -> AluminiumMember {
        proxy_member(&shell.id, &shell.material_id, shell.length, &shell.actions)
    }
}


#[path = "🔩️alloys-and-sections/🦀️.rs"]
pub mod part_1_1;

#[path = "🔥️fire/🦀️.rs"]
pub mod part_1_2;

#[path = "🔄️fatigue/🦀️.rs"]
pub mod part_1_3;

#[path = "🧩️joints/🦀️.rs"]
pub mod joints;

#[path = "🫙️thin-wall/🦀️.rs"]
pub mod part_1_4_1_5;

#[path = "🦴️web-stiffener/🦀️.rs"]
pub mod web_stiffener;

/// 🩹 Keeps, per check id, the result with the highest utilization over the evaluated combinations (first-seen order).
pub fn worst_by_id(sets: Vec<Vec<CheckResult>>) -> Vec<CheckResult> {
    let mut out: Vec<CheckResult> = Vec::new();
    let mut index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for set in sets {
        for c in set {
            match index.get(&c.id) {
                Some(&i) => {
                    if c.utilization > out[i].utilization { out[i] = c; }
                }
                None => {
                    index.insert(c.id.clone(), out.len());
                    out.push(c);
                }
            }
        }
    }
    out
}
//#endregion 🔖️ComplianceHelpers
