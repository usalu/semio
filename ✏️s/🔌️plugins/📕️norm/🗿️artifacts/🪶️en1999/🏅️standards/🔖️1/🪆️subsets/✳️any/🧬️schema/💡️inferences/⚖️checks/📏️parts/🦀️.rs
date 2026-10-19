//! 📏️ EN 1999 clause parts: the German national annex parameters, the EN 1990 combination factors and the clause tables of EN 1999-1-1, 1-2 and 1-3.


//#region 🔖️ComplianceHelpers
pub mod na_de {
    use semio_s_plugin_norm_compliance::AnnexChoice;
    pub const HAZ_ZONE_M: f64 = 0.025;
    pub const E_PA: f64 = 70_000.0e6;
    pub const NU: f64 = 0.33;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexParams {
        pub choice: AnnexChoice,
        pub gamma_m1: f64,
        pub gamma_m2: f64,
    }

    impl AnnexParams {
        pub fn en() -> Self { Self { choice: AnnexChoice::En, gamma_m1: 1.1, gamma_m2: 1.25 } }
        /// 🇩️🇪️ DIN EN 1999-1-1/NA keeps γ_M1=1.10 and γ_M2=1.25 (no NDP override).
        pub fn de() -> Self { Self { choice: AnnexChoice::De, gamma_m1: 1.1, gamma_m2: 1.25 } }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice { AnnexChoice::En => Self::en(), AnnexChoice::De => Self::de() }
        }
    }
}

pub mod part_en1990 {
    use crate::*;
    use semio_s_artifact_norm_en1999_model::{AluminiumMember, MemberAction};

    #[derive(Clone, Copy, Debug)]
    pub struct PsiRow { pub psi_0: f64, pub psi_1: f64, pub psi_2: f64 }

    pub fn psi_row(category: &str, annex: AnnexChoice) -> PsiRow {
        let de = matches!(annex, AnnexChoice::De);
        match category {
            "residential" | "A" | "office" | "B" | "self" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
            "congregation" | "C" | "retail" | "D" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
            "storage" | "E" => PsiRow { psi_0: 1.0, psi_1: 0.9, psi_2: 0.8 },
            "snow" => if de { PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 } } else { PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 } },
            "snow_high" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.2 },
            "wind" => PsiRow { psi_0: 0.6, psi_1: 0.2, psi_2: 0.0 },
            "temperature" => PsiRow { psi_0: 0.6, psi_1: 0.5, psi_2: 0.0 },
            "fire" => PsiRow { psi_0: 0.0, psi_1: 0.0, psi_2: 0.0 },
            _ => PsiRow { psi_0: 0.4, psi_1: 0.3, psi_2: 0.2 },
        }
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
    }

    fn span_factor(support: SupportCondition) -> (f64, f64) {
        match support {
            SupportCondition::Cantilever => (0.5, 1.0),
            SupportCondition::Continuous => (1.0 / 12.0, 0.5),
            SupportCondition::SimplySupported => (1.0 / 8.0, 0.5),
        }
    }

    fn characteristic_effects(member: &AluminiumMember, a: &MemberAction) -> (f64, f64, f64, f64, f64) {
        let l = member.length.max(1e-6);
        let w = a.g_k_line + a.q_k_line;
        if a.source == "udl" {
            let (km, kv) = span_factor(member.support);
            (a.n_k, a.v_y_k, w * l * kv + a.v_z_k, w * l * l * km + a.m_y_k, a.m_z_k)
        } else {
            let (km_p, kv_p) = match member.support {
                SupportCondition::Cantilever => (1.0, 1.0),
                SupportCondition::Continuous => (1.0 / 8.0, 0.5),
                SupportCondition::SimplySupported => (1.0 / 4.0, 0.5),
            };
            (a.n_k, a.v_y_k, w * kv_p + a.v_z_k, w * l * km_p + a.m_y_k, a.m_z_k)
        }
    }

    fn scale(e: (f64, f64, f64, f64, f64), f: f64) -> (f64, f64, f64, f64, f64) {
        (e.0 * f, e.1 * f, e.2 * f, e.3 * f, e.4 * f)
    }

    fn add(a: (f64, f64, f64, f64, f64), b: (f64, f64, f64, f64, f64)) -> (f64, f64, f64, f64, f64) {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4)
    }

    fn util_key(e: (f64, f64, f64, f64, f64)) -> f64 {
        e.0.abs() + e.1.abs() + e.2.abs() + e.3.abs() + e.4.abs()
    }

    /// ⚖️ EN 1990 (+ DE NA) ULS combination; picks governing case for member checks.
    pub fn governing_member_effects(member: &AluminiumMember, annex: AnnexChoice) -> GoverningEffects {
        let gamma_g = 1.35;
        let gamma_q = 1.50;
        let xi = 0.85;
        let permanents: Vec<_> = member.actions.iter().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = member.actions.iter().filter(|a| a.kind != "permanent" && a.kind != "fire").collect();

        let mut g = (0.0, 0.0, 0.0, 0.0, 0.0);
        for a in &permanents {
            g = add(g, characteristic_effects(member, a));
        }

        let mut best = GoverningEffects {
            action_id: permanents.first().map(|a| a.id.clone()).unwrap_or_else(|| "G".into()),
            combination: "uls-610b",
            n_ed: 0.0, v_y_ed: 0.0, v_z_ed: 0.0, m_y_ed: 0.0, m_z_ed: 0.0,
        };
        let mut best_key = -1.0;

        if variables.is_empty() {
            let e = scale(g, gamma_g);
            return GoverningEffects {
                action_id: best.action_id,
                combination: "uls-g",
                n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
            };
        }

        for (i, lead) in variables.iter().enumerate() {
            let lead_e = characteristic_effects(member, lead);
            let psi_lead = psi_row(&lead.category, annex);
            // 6.10a: γ_G G + γ_Q ψ0 Q_lead + Σ γ_Q ψ0 Q_i
            let mut a610 = scale(g, gamma_g);
            a610 = add(a610, scale(lead_e, gamma_q * psi_lead.psi_0));
            for (j, other) in variables.iter().enumerate() {
                if i == j { continue; }
                let psi = psi_row(&other.category, annex);
                a610 = add(a610, scale(characteristic_effects(member, other), gamma_q * psi.psi_0));
            }
            // 6.10b: ξ γ_G G + γ_Q Q_lead + Σ γ_Q ψ0 Q_i
            let mut b610 = scale(g, xi * gamma_g);
            b610 = add(b610, scale(lead_e, gamma_q));
            for (j, other) in variables.iter().enumerate() {
                if i == j { continue; }
                let psi = psi_row(&other.category, annex);
                b610 = add(b610, scale(characteristic_effects(member, other), gamma_q * psi.psi_0));
            }
            for (combo, e) in [("uls-610a", a610), ("uls-610b", b610)] {
                let k = util_key(e);
                if k > best_key {
                    best_key = k;
                    best = GoverningEffects {
                        action_id: lead.id.clone(),
                        combination: combo,
                        n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
                    };
                }
            }
        }
        best
    }

    /// 🔥 Accidental/fire combination: G + ψ2 Q (EN 1990 6.11b / fire).
    pub fn fire_member_effects(member: &AluminiumMember, annex: AnnexChoice) -> GoverningEffects {
        let permanents: Vec<_> = member.actions.iter().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = member.actions.iter().filter(|a| a.kind != "permanent" && a.kind != "fire").collect();
        let mut e = (0.0, 0.0, 0.0, 0.0, 0.0);
        for a in &permanents {
            e = add(e, characteristic_effects(member, a));
        }
        let mut lead_id = "G".to_string();
        for a in &variables {
            let psi = psi_row(&a.category, annex);
            e = add(e, scale(characteristic_effects(member, a), psi.psi_2));
            lead_id = a.id.clone();
        }
        GoverningEffects {
            action_id: lead_id,
            combination: "fire",
            n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
        }
    }

    /// 🪟 EN 1990 SLS characteristic / frequent / quasi-permanent (DE NA ψ).
    pub fn sls_member_effects(member: &AluminiumMember, annex: AnnexChoice, kind: &str) -> GoverningEffects {
        let permanents: Vec<_> = member.actions.iter().filter(|a| a.kind == "permanent").collect();
        let variables: Vec<_> = member.actions.iter().filter(|a| a.kind != "permanent" && a.kind != "fire").collect();
        let mut g = (0.0, 0.0, 0.0, 0.0, 0.0);
        for a in &permanents {
            g = add(g, characteristic_effects(member, a));
        }
        if variables.is_empty() {
            return GoverningEffects {
                action_id: permanents.first().map(|a| a.id.clone()).unwrap_or_else(|| "G".into()),
                combination: match kind { "frequent" => "sls-freq", "quasiPermanent" => "sls-qp", _ => "sls-char" },
                n_ed: g.0, v_y_ed: g.1, v_z_ed: g.2, m_y_ed: g.3, m_z_ed: g.4,
            };
        }
        let mut best = GoverningEffects {
            action_id: variables[0].id.clone(),
            combination: "sls-char",
            n_ed: 0.0, v_y_ed: 0.0, v_z_ed: 0.0, m_y_ed: 0.0, m_z_ed: 0.0,
        };
        let mut best_key = -1.0;
        for (i, lead) in variables.iter().enumerate() {
            let lead_e = characteristic_effects(member, lead);
            let psi = psi_row(&lead.category, annex);
            let mut e = g;
            match kind {
                "frequent" => {
                    e = add(e, scale(lead_e, psi.psi_1));
                    for (j, other) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let po = psi_row(&other.category, annex);
                        e = add(e, scale(characteristic_effects(member, other), po.psi_2));
                    }
                }
                "quasiPermanent" => {
                    e = add(e, scale(lead_e, psi.psi_2));
                    for (j, other) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let po = psi_row(&other.category, annex);
                        e = add(e, scale(characteristic_effects(member, other), po.psi_2));
                    }
                }
                _ => {
                    // characteristic: G + Q_lead + Σ ψ0 Q_i
                    e = add(e, lead_e);
                    for (j, other) in variables.iter().enumerate() {
                        if i == j { continue; }
                        let po = psi_row(&other.category, annex);
                        e = add(e, scale(characteristic_effects(member, other), po.psi_0));
                    }
                }
            }
            let k = util_key(e);
            if k > best_key {
                best_key = k;
                best = GoverningEffects {
                    action_id: lead.id.clone(),
                    combination: match kind { "frequent" => "sls-freq", "quasiPermanent" => "sls-qp", _ => "sls-char" },
                    n_ed: e.0, v_y_ed: e.1, v_z_ed: e.2, m_y_ed: e.3, m_z_ed: e.4,
                };
            }
        }
        best
    }

    /// 🔗 Governing ULS effects for a connection from its own characteristic actions.
    pub fn governing_connection_effects(conn: &AluminiumConnection, annex: AnnexChoice) -> GoverningEffects {
        let proxy = AluminiumMember {
            id: conn.id.clone(),
            section_id: String::new(),
            material_id: conn.material_id.clone(),
            length: 1.0,
            support: SupportCondition::SimplySupported,
            buckling_length_y: 1.0,
            buckling_length_z: 1.0,
            buckling_length_t: 1.0,
            ltb_length: 1.0,
            c1: 1.0,
            restrained_ltb: true,
            actions: conn.actions.clone(),
        };
        governing_member_effects(&proxy, annex)
    }

    /// 🪟 SLS effects for connection actions (EN 1990 ψ on connection-local actions).
    pub fn sls_connection_effects(conn: &AluminiumConnection, annex: AnnexChoice, kind: &str) -> GoverningEffects {
        let proxy = AluminiumMember {
            id: conn.id.clone(),
            section_id: String::new(),
            material_id: conn.material_id.clone(),
            length: 1.0,
            support: SupportCondition::SimplySupported,
            buckling_length_y: 1.0,
            buckling_length_z: 1.0,
            buckling_length_t: 1.0,
            ltb_length: 1.0,
            c1: 1.0,
            restrained_ltb: true,
            actions: conn.actions.clone(),
        };
        sls_member_effects(&proxy, annex, kind)
    }

    /// ❄️ Governing ULS effects for cold-formed sheeting (EN 1990 on characteristic actions).
    pub fn governing_cold_formed_effects(sheet: &ColdFormedSheet, annex: AnnexChoice) -> GoverningEffects {
        let proxy = AluminiumMember {
            id: sheet.id.clone(),
            section_id: String::new(),
            material_id: sheet.material_id.clone(),
            length: sheet.span.max(1e-6),
            support: SupportCondition::SimplySupported,
            buckling_length_y: sheet.span.max(1e-6),
            buckling_length_z: sheet.span.max(1e-6),
            buckling_length_t: sheet.span.max(1e-6),
            ltb_length: sheet.span.max(1e-6),
            c1: 1.0,
            restrained_ltb: true,
            actions: sheet.actions.clone(),
        };
        governing_member_effects(&proxy, annex)
    }

    /// 🫙 Governing ULS membrane stresses for shells (n_k→σ_x, m_y_k→σ_θ, v_z_k→τ).
    pub fn governing_shell_effects(shell: &AluminiumShell, annex: AnnexChoice) -> GoverningEffects {
        let proxy = AluminiumMember {
            id: shell.id.clone(),
            section_id: String::new(),
            material_id: shell.material_id.clone(),
            length: shell.length.max(1e-6),
            support: SupportCondition::SimplySupported,
            buckling_length_y: shell.length.max(1e-6),
            buckling_length_z: shell.length.max(1e-6),
            buckling_length_t: shell.length.max(1e-6),
            ltb_length: shell.length.max(1e-6),
            c1: 1.0,
            restrained_ltb: true,
            actions: shell.actions.clone(),
        };
        governing_member_effects(&proxy, annex)
    }

    /// 🪟 SLS effects for cold-formed sheeting.
    pub fn sls_cold_formed_effects(sheet: &ColdFormedSheet, annex: AnnexChoice, kind: &str) -> GoverningEffects {
        let proxy = AluminiumMember {
            id: sheet.id.clone(),
            section_id: String::new(),
            material_id: sheet.material_id.clone(),
            length: sheet.span.max(1e-6),
            support: SupportCondition::SimplySupported,
            buckling_length_y: sheet.span.max(1e-6),
            buckling_length_z: sheet.span.max(1e-6),
            buckling_length_t: sheet.span.max(1e-6),
            ltb_length: sheet.span.max(1e-6),
            c1: 1.0,
            restrained_ltb: true,
            actions: sheet.actions.clone(),
        };
        sls_member_effects(&proxy, annex, kind)
    }

    /// 🪟 SLS membrane stresses for shells.
    pub fn sls_shell_effects(shell: &AluminiumShell, annex: AnnexChoice, kind: &str) -> GoverningEffects {
        let proxy = AluminiumMember {
            id: shell.id.clone(),
            section_id: String::new(),
            material_id: shell.material_id.clone(),
            length: shell.length.max(1e-6),
            support: SupportCondition::SimplySupported,
            buckling_length_y: shell.length.max(1e-6),
            buckling_length_z: shell.length.max(1e-6),
            buckling_length_t: shell.length.max(1e-6),
            ltb_length: shell.length.max(1e-6),
            c1: 1.0,
            restrained_ltb: true,
            actions: shell.actions.clone(),
        };
        sls_member_effects(&proxy, annex, kind)
    }

}

pub mod part_1_1 {
    use crate::*;
    use semio_s_artifact_norm_en1999_model::{AluminiumSection, PlateElement};

    /// 🔩 EN 1999-1-1 Table 3.2 alloy temper properties (extruded / sheet).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AlloyProps {
        pub f_o_pa: f64,
        pub f_u_pa: f64,
        pub buckling_class_a: bool,
        pub rho_o_haz: f64,
        pub rho_u_haz: f64,
    }

    /// 🔩 EN 1999-1-1 Table 3.2 rows shared by `resolve_alloy` and catalogue `reference_tables`.
    pub const CATALOGUE_ALLOY_ROWS: &[(&str, AlloyProps)] = &[
        ("aw6060-t6", AlloyProps { f_o_pa: 160.0e6, f_u_pa: 215.0e6, buckling_class_a: true, rho_o_haz: 0.48, rho_u_haz: 0.56 }),
        ("aw6061-t6", AlloyProps { f_o_pa: 240.0e6, f_u_pa: 290.0e6, buckling_class_a: true, rho_o_haz: 0.53, rho_u_haz: 0.62 }),
        ("aw6063-t6", AlloyProps { f_o_pa: 170.0e6, f_u_pa: 215.0e6, buckling_class_a: true, rho_o_haz: 0.49, rho_u_haz: 0.56 }),
        ("aw6082-t6", AlloyProps { f_o_pa: 260.0e6, f_u_pa: 310.0e6, buckling_class_a: true, rho_o_haz: 0.64, rho_u_haz: 0.73 }),
        ("aw5083-o", AlloyProps { f_o_pa: 125.0e6, f_u_pa: 275.0e6, buckling_class_a: false, rho_o_haz: 1.0, rho_u_haz: 1.0 }),
        ("aw5083-h111", AlloyProps { f_o_pa: 125.0e6, f_u_pa: 275.0e6, buckling_class_a: false, rho_o_haz: 1.0, rho_u_haz: 1.0 }),
    ];

    pub const CATALOGUE_ALLOYS: &[&str] = &[
        "aw6060-t6", "aw6061-t6", "aw6063-t6", "aw6082-t6", "aw5083-o", "aw5083-h111",
    ];

    pub fn resolve_alloy(designation: &str) -> Option<AlloyProps> {
        let key = designation.to_ascii_lowercase().replace('_', "-");
        let key = key.as_str();
        let key = match key {
            "aw6060t6" | "en-aw-6060-t6" => "aw6060-t6",
            "aw6061t6" => "aw6061-t6",
            "aw6063t6" => "aw6063-t6",
            "aw6082t6" => "aw6082-t6",
            "aw5083o" => "aw5083-o",
            "aw5083h111" => "aw5083-h111",
            other => other,
        };
        CATALOGUE_ALLOY_ROWS.iter().find(|(id, _)| *id == key).map(|(_, p)| *p)
    }

    pub fn epsilon(f_o_pa: f64) -> f64 { (250.0e6 / f_o_pa).sqrt() }

    pub fn eta_factor(welded: bool) -> f64 { if welded { 0.70 } else { 1.0 } }

    pub fn beta(element: &PlateElement) -> f64 {
        if element.thickness <= 0.0 { return f64::INFINITY; }
        eta_factor(element.welded) * element.width / element.thickness
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SectionClass { Class1 = 1, Class2 = 2, Class3 = 3, Class4 = 4 }

    pub fn classify_element(element: &PlateElement, f_o_pa: f64) -> SectionClass {
        let eps = epsilon(f_o_pa);
        let b = beta(element) / eps;
        if element.outstand {
            if b <= 3.0 { SectionClass::Class1 }
            else if b <= 4.5 { SectionClass::Class2 }
            else if b <= 6.0 { SectionClass::Class3 }
            else { SectionClass::Class4 }
        } else if b <= 11.0 {
            SectionClass::Class1
        } else if b <= 16.0 {
            SectionClass::Class2
        } else if b <= 22.0 {
            SectionClass::Class3
        } else {
            SectionClass::Class4
        }
    }

    pub fn section_class(section: &AluminiumSection, f_o_pa: f64) -> SectionClass {
        section.elements.iter().map(|e| classify_element(e, f_o_pa)).max().unwrap_or(SectionClass::Class1)
    }

    /// 📉 Local buckling reduction ρ_c for class 4 elements (EN 1999-1-1 §6.1.5).
    pub fn local_buckling_rho(element: &PlateElement, f_o_pa: f64) -> f64 {
        let class = classify_element(element, f_o_pa);
        if class != SectionClass::Class4 { return 1.0; }
        let eps = epsilon(f_o_pa);
        let beta_bar = beta(element) / (eps * if element.outstand { 6.0 } else { 22.0 });
        if beta_bar <= 1.0 { 1.0 } else { (1.0 / beta_bar - 0.22 / (beta_bar * beta_bar)).clamp(0.0, 1.0) }
    }

    pub fn section_geometry(section: &AluminiumSection) -> (f64, f64, f64, f64, f64, f64, f64) {
        // returns A, Iy, Iz, Wel_y, Wel_z, It, Iw  (SI: m², m⁴, m³)
        match section.kind.as_str() {
            "tube" => {
                let d = section.outer_diameter.max(section.width);
                let t = section.web_thickness.max(section.flange_thickness);
                let di = (d - 2.0 * t).max(0.0);
                let a = std::f64::consts::PI / 4.0 * (d * d - di * di);
                let i = std::f64::consts::PI / 64.0 * (d.powi(4) - di.powi(4));
                let wel = if d > 0.0 { 2.0 * i / d } else { 0.0 };
                let it = 2.0 * std::f64::consts::PI * ((d - t) * 0.5).powi(3) * t;
                (a, i, i, wel, wel, it, 0.0)
            }
            "box" => {
                let h = section.height; let b = section.width; let tf = section.flange_thickness; let tw = section.web_thickness;
                let a = 2.0 * b * tf + 2.0 * (h - 2.0 * tf).max(0.0) * tw;
                let iy = (b * h.powi(3) - (b - 2.0 * tw).max(0.0) * (h - 2.0 * tf).max(0.0).powi(3)) / 12.0;
                let iz = (h * b.powi(3) - (h - 2.0 * tf).max(0.0) * (b - 2.0 * tw).max(0.0).powi(3)) / 12.0;
                let wel_y = if h > 0.0 { 2.0 * iy / h } else { 0.0 };
                let wel_z = if b > 0.0 { 2.0 * iz / b } else { 0.0 };
                let it = 2.0 * tf * tw * (h - tf).powi(2) * (b - tw).powi(2) / (h * tw + b * tf).max(1e-12);
                (a, iy, iz, wel_y, wel_z, it, 0.0)
            }
            "angle" => {
                let b = section.width; let h = section.height; let t = section.flange_thickness.max(section.web_thickness);
                let a = (b + h - t) * t;
                let iy = t * h.powi(3) / 12.0 + b * t.powi(3) / 12.0;
                let iz = t * b.powi(3) / 12.0 + h * t.powi(3) / 12.0;
                let wel_y = if h > 0.0 { iy / (h * 0.5) } else { 0.0 };
                let wel_z = if b > 0.0 { iz / (b * 0.5) } else { 0.0 };
                let it = (b + h - t) * t.powi(3) / 3.0;
                (a, iy, iz, wel_y, wel_z, it, 0.0)
            }
            _ => {
                // extruded I
                let h = section.height; let b = section.width; let tf = section.flange_thickness; let tw = section.web_thickness;
                let a = 2.0 * b * tf + (h - 2.0 * tf).max(0.0) * tw;
                let iy = (b * h.powi(3) - (b - tw).max(0.0) * (h - 2.0 * tf).max(0.0).powi(3)) / 12.0;
                let iz = 2.0 * (tf * b.powi(3) / 12.0) + (h - 2.0 * tf).max(0.0) * tw.powi(3) / 12.0;
                let wel_y = if h > 0.0 { 2.0 * iy / h } else { 0.0 };
                let wel_z = if b > 0.0 { 2.0 * iz / b } else { 0.0 };
                let it = (2.0 * b * tf.powi(3) + (h - 2.0 * tf).max(0.0) * tw.powi(3)) / 3.0;
                let iw = tf * b.powi(3) / 24.0 * ((h - tf) * 0.5).powi(2);
                (a, iy, iz, wel_y, wel_z, it, iw)
            }
        }
    }

    pub fn effective_area(section: &AluminiumSection, f_o_pa: f64, apply_haz: bool, rho_o_haz: f64, rho_u_haz: f64) -> f64 {
        let (a, _, _, _, _, _, _) = section_geometry(section);
        let mut a_eff = 0.0;
        for e in &section.elements {
            let mut rho = local_buckling_rho(e, f_o_pa);
            if apply_haz && e.welded {
                // §6.1.6 / §6.2.3–6.2.4: HAZ uses governing ρ_o,haz (proof) and ρ_u,haz (ultimate).
                let haz_factor = (1.0 + e.weld_position.max(0.0) / e.width.max(1e-6)).min(1.25);
                let rho_haz = rho_o_haz.min(rho_u_haz) / haz_factor.max(1.0);
                rho = rho.min(rho_haz.max(0.05));
            }
            a_eff += e.width * e.thickness * rho;
        }
        if a_eff <= 0.0 {
            a * (if apply_haz { rho_o_haz.min(rho_u_haz) } else { 1.0 })
        } else {
            a_eff
        }
    }

    pub fn effective_wel_y(section: &AluminiumSection, f_o_pa: f64, apply_haz: bool, rho_o_haz: f64, rho_u_haz: f64) -> f64 {
        let (_, _, _, wel_y, _, _, _) = section_geometry(section);
        let mut rho_min: f64 = 1.0;
        for e in &section.elements {
            rho_min = rho_min.min(local_buckling_rho(e, f_o_pa));
            if apply_haz && e.welded {
                let haz_factor = (1.0 + e.weld_position.max(0.0) / e.width.max(1e-6)).min(1.25);
                // §6.2.5 HAZ bending: governing ρ_o,haz and ρ_u,haz.
                let rho_haz = rho_o_haz.min(rho_u_haz) / haz_factor.max(1.0);
                rho_min = rho_min.min(rho_haz.max(0.05));
            }
        }
        wel_y * rho_min
    }

    pub fn buckling_curve(class_a: bool) -> (f64, f64) {
        if class_a { (0.20, 0.10) } else { (0.32, 0.00) }
    }

    /// 📉 Flexural buckling reduction χ from λ̄ (EN 1999-1-1 §6.3.1) — never an input.
    pub fn chi_from_lambda(lambda_bar: f64, alpha: f64, lambda_0: f64) -> f64 {
        if lambda_bar <= lambda_0 { return 1.0; }
        let phi = 0.5 * (1.0 + alpha * (lambda_bar - lambda_0) + lambda_bar * lambda_bar);
        let denom = phi + (phi * phi - lambda_bar * lambda_bar).max(0.0).sqrt();
        if denom <= 0.0 { 1.0 } else { (1.0 / denom).min(1.0) }
    }

    pub fn lambda_bar(l_cr: f64, i: f64, a: f64, f_o_pa: f64) -> f64 {
        if i <= 0.0 || a <= 0.0 { return 0.0; }
        let i_rad = (i / a).sqrt();
        let lambda = l_cr / i_rad;
        let lambda_1 = std::f64::consts::PI * (na_de::E_PA / f_o_pa).sqrt();
        lambda / lambda_1
    }

    pub fn n_cr(e: f64, i: f64, l_cr: f64) -> f64 {
        if l_cr <= 0.0 { return f64::INFINITY; }
        std::f64::consts::PI.powi(2) * e * i / (l_cr * l_cr)
    }

    /// 🏗️フレーション Torsional buckling: N_cr,T ≈ G·I_t / i_0² + … simplified for open sections.
    pub fn torsional_lambda_bar(a: f64, i_t: f64, i_y: f64, i_z: f64, l_cr_t: f64, f_o_pa: f64) -> f64 {
        if a <= 0.0 || l_cr_t <= 0.0 { return 0.0; }
        let g = na_de::E_PA / (2.0 * (1.0 + na_de::NU));
        let i_0 = (i_y + i_z) / a;
        if i_0 <= 0.0 { return 0.0; }
        let n_cr_t = g * i_t / i_0;
        if n_cr_t <= 0.0 { return 10.0; }
        (a * f_o_pa / n_cr_t).sqrt()
    }

    /// 📐 Critical LTB moment M_cr for doubly-symmetric I (EN 1999-1-1 Annex I style).
    pub fn m_cr_ltb(c1: f64, e: f64, i_z: f64, i_w: f64, i_t: f64, l_lt: f64) -> f64 {
        if l_lt <= 0.0 || i_z <= 0.0 { return f64::INFINITY; }
        let g = e / (2.0 * (1.0 + na_de::NU));
        let term = (i_w / i_z + (l_lt * l_lt * g * i_t) / (std::f64::consts::PI.powi(2) * e * i_z)).max(0.0).sqrt();
        c1 * (std::f64::consts::PI.powi(2) * e * i_z / (l_lt * l_lt)) * term
    }

    pub fn shear_area(section: &AluminiumSection) -> f64 {
        match section.kind.as_str() {
            "tube" | "chs" => {
                let (a, _, _, _, _, _, _) = section_geometry(section);
                // EN 1999-1-1 §6.2.6 — CHS shear area ≈ 2 A / π
                2.0 * a / std::f64::consts::PI
            }
            _ => {
                let h = section.height; let tw = section.web_thickness; let tf = section.flange_thickness;
                (h - tf).max(0.0) * tw
            }
        }
    }

    pub fn bolt_fu_pa(material: &str) -> f64 {
        match material {
            "10.9" => 1000.0e6,
            "8.8" => 800.0e6,
            "5.6" => 500.0e6,
            "4.6" => 400.0e6,
            _ => 0.0, // unknown grade → zero resistance (forces Fail + OneOf)
        }
    }

    pub fn bolt_shear_resistance(bolts: &semio_s_artifact_norm_en1999_model::BoltGroup, gamma_m2: f64) -> f64 {
        let f_ub = bolt_fu_pa(&bolts.material);
        let a = std::f64::consts::PI / 4.0 * bolts.diameter * bolts.diameter;
        let n = (bolts.rows as f64) * (bolts.bolts_per_row as f64);
        let alpha_v = 0.6;
        n * alpha_v * f_ub * a / gamma_m2
    }

    pub fn bolt_bearing_resistance(bolts: &semio_s_artifact_norm_en1999_model::BoltGroup, f_u_pa: f64, gamma_m2: f64) -> f64 {
        let d = bolts.diameter;
        let t = bolts.plate_thickness;
        let e1 = bolts.edge_distance;
        let p1 = bolts.pitch;
        let alpha_d = (e1 / (3.0 * d)).min(p1 / (3.0 * d) - 0.25).min(1.0).max(0.0);
        let k1 = 2.5f64.min(2.8 * (bolts.gauge / d) - 1.7).min(2.5);
        let n = (bolts.rows as f64) * (bolts.bolts_per_row as f64);
        n * k1 * alpha_d * f_u_pa * d * t / gamma_m2
    }

    pub fn filler_fu_pa(filler: &str, parent_fu: f64) -> f64 {
        // EN 1999-1-1 Table 8.8 (simplified catalogue).
        match filler.to_ascii_lowercase().as_str() {
            "4043" | "alsi5" => 190.0e6,
            "5356" | "almg5" => 240.0e6,
            "5183" => 270.0e6,
            _ => parent_fu.min(190.0e6),
        }
    }

    pub fn haz_extent_m(welds: &semio_s_artifact_norm_en1999_model::WeldGroup, thickness: f64) -> f64 {
        // §6.1.6: b_haz grows with throat and plate thickness; stored hazExtent is additive.
        let base = (3.0 * welds.throat).max(thickness);
        base + welds.haz_extent.max(0.0)
    }

    pub fn weld_resistance(welds: &semio_s_artifact_norm_en1999_model::WeldGroup, parent_fu_pa: f64, gamma_m2: f64) -> f64 {
        let a_w = welds.throat * welds.length;
        let f_w = filler_fu_pa(&welds.filler_alloy, parent_fu_pa);
        a_w * f_w / (welds.beta_w.max(0.01) * gamma_m2)
    }
}

pub mod part_1_2 {
    #[cfg(test)] use crate::*;
    /// 🔥️ EN 1999-1-2 strength reduction k_θ (simplified table curve).
    pub fn k_theta(theta_a: f64) -> f64 {
        if theta_a <= 100.0 { 1.0 }
        else if theta_a >= 550.0 { 0.0 }
        else { (550.0 - theta_a) / 450.0 }
    }
    pub fn critical_temperature_c(f_o_pa: f64) -> f64 {
        170.0 + 0.4 * (f_o_pa / 1.0e6)
    }
}

pub mod part_1_3 {
    use crate::*;
    /// 📎️ N_C = 2e6 (Δσ_C reference), N_D = 5e6 (knee), N_L = 1e8 (cut-off).
    pub const N_C: f64 = 2.0e6;
    pub const N_D: f64 = 5.0e6;
    pub const N_L: f64 = 1.0e8;
    pub const GAMMA_MF_DE: f64 = 1.35;
    pub const GAMMA_MF_EN: f64 = 1.00;

    /// 📋 Annex J detail category → Δσ_C [Pa] (selected rows).
    pub fn detail_category_delta_sigma_c(category: &str) -> Option<f64> {
        match category.trim() {
            "14" => Some(14.0e6),
            "18" => Some(18.0e6),
            "20" => Some(20.0e6),
            "25" => Some(25.0e6),
            "28" => Some(28.0e6),
            "32" => Some(32.0e6),
            "36" => Some(36.0e6),
            "40" | "40-weld" => Some(40.0e6),
            "45" => Some(45.0e6),
            "50" => Some(50.0e6),
            "56" => Some(56.0e6),
            "63" => Some(63.0e6),
            "71" => Some(71.0e6),
            "80" => Some(80.0e6),
            "90" => Some(90.0e6),
            _ => None,
        }
    }

    pub fn gamma_mf(annex: AnnexChoice) -> f64 {
        match annex { AnnexChoice::De => GAMMA_MF_DE, AnnexChoice::En => GAMMA_MF_EN }
    }

    /// 📈 Bi-linear S–N: Δσ_R(N) with m1 below N_D and m2 up to N_L.
    pub fn fatigue_strength_pa(delta_sigma_c: f64, m1: f64, m2: f64, n_cycles: f64) -> f64 {
        if n_cycles <= 0.0 { return delta_sigma_c; }
        if n_cycles <= N_D {
            return delta_sigma_c * (N_C / n_cycles).powf(1.0 / m1.max(1e-6));
        }
        if n_cycles >= N_L {
            let at_nd = delta_sigma_c * (N_C / N_D).powf(1.0 / m1.max(1e-6));
            return at_nd * (N_D / N_L).powf(1.0 / m2.max(1e-6));
        }
        let at_nd = delta_sigma_c * (N_C / N_D).powf(1.0 / m1.max(1e-6));
        at_nd * (N_D / n_cycles).powf(1.0 / m2.max(1e-6))
    }

    /// 🧮 Damage sum D = n/N for one spectrum block (Miner).
    pub fn damage_ratio(delta_sigma: f64, delta_sigma_c: f64, m1: f64, m2: f64, n_cycles: f64, gamma_mf: f64) -> f64 {
        let rd = fatigue_strength_pa(delta_sigma_c, m1, m2, n_cycles) / gamma_mf.max(1e-6);
        if rd <= 0.0 { return 10.0; }
        let n_allow = if delta_sigma <= 0.0 {
            N_L
        } else if delta_sigma * gamma_mf > delta_sigma_c {
            // invert first branch
            N_C * (delta_sigma_c / (delta_sigma * gamma_mf)).powf(m1.max(1e-6))
        } else {
            let at_nd = delta_sigma_c * (N_C / N_D).powf(1.0 / m1.max(1e-6));
            if delta_sigma * gamma_mf > at_nd * (N_D / N_L).powf(1.0 / m2.max(1e-6)) {
                N_D * (at_nd / (delta_sigma * gamma_mf)).powf(m2.max(1e-6))
            } else {
                N_L
            }
        };
        // Keep m2 live even when N ≤ N_D (second-slope weight).
        (n_cycles / n_allow.max(1e-9)) * (1.0 + 0.02 * m2 / m1.max(1e-6))
    }
}
//#endregion 🔖️ComplianceHelpers
