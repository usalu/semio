//! 🧱️ EN 1991 check parts: the unit and remedy helpers and the closed-form formulas of parts 1-1 to 4. See [`super`].

#![allow(async_fn_in_trait)]

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_plugin_norm_compliance::{CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

pub fn pressure_pa(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }
pub fn force_n(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
pub fn temp_k(k: f64) -> Quantity { Quantity::new(QuantityKind::Temperature, k) }
pub fn kn_m2_to_pa(kn_m2: f64) -> f64 { kn_m2 * 1000.0 }
pub fn pa_to_kn_m2(pa: f64) -> f64 { pa / 1000.0 }
pub fn kn_to_n(kn: f64) -> f64 { kn * 1000.0 }
pub fn ndp(id: &str) -> f64 { semio_s_plugin_norm_compliance::ndp_common(id) }
pub fn copy(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }
pub fn ratio(required: f64, assumed: f64) -> f64 { if required <= 0.0 { 0.0 } else { required / assumed.max(1e-9) } }

/// 🩹 Remedy raising an assumed pressure to the required value.
pub fn raise_pressure_remedy(path: &str, entity_id: &str, label_en: &str, label_de: &str, current_pa: f64, required_pa: f64) -> Remedy {
    let cur = pa_to_kn_m2(current_pa);
    let req = pa_to_kn_m2(required_pa);
    Remedy::at_least(
        SubjectRef::new(entity_id, path, copy(label_en, label_de)),
        pressure_pa(current_pa),
        pressure_pa(required_pa),
        copy(
            &format!("Raise assumed {label_en} from {cur:.3} kN/m² to at least {req:.3} kN/m²."),
            &format!("Angenommenen Wert ({label_de}) von {cur:.3} kN/m² auf mindestens {req:.3} kN/m² erhöhen."),
        ),
    )
}

/// 🪝 Remedy raising an assumed force to the required value.
pub fn raise_force_remedy(path: &str, entity_id: &str, label_en: &str, label_de: &str, current_n: f64, required_n: f64) -> Remedy {
    let cur = current_n / 1000.0;
    let req = required_n / 1000.0;
    Remedy::at_least(
        SubjectRef::new(entity_id, path, copy(label_en, label_de)),
        force_n(current_n),
        force_n(required_n),
        copy(
            &format!("Raise assumed {label_en} from {cur:.3} kN to at least {req:.3} kN."),
            &format!("Angenommenen Wert ({label_de}) von {cur:.3} kN auf mindestens {req:.3} kN erhöhen."),
        ),
    )
}

/// 🌡️ Remedy raising an assumed temperature to the required value.
pub fn raise_temp_remedy(path: &str, entity_id: &str, label_en: &str, label_de: &str, current_k: f64, required_k: f64) -> Remedy {
    Remedy::at_least(
        SubjectRef::new(entity_id, path, copy(label_en, label_de)),
        temp_k(current_k),
        temp_k(required_k),
        copy(
            &format!("Raise assumed {label_en} from {current_k:.1} K to at least {required_k:.1} K."),
            &format!("Angenommenen Wert ({label_de}) von {current_k:.1} K auf mindestens {required_k:.1} K erhöhen."),
        ),
    )
}

/// ♨️ Remedy raising an assumed heat flux to the required value.
pub fn raise_power_remedy(path: &str, entity_id: &str, label_en: &str, label_de: &str, current: f64, required: f64) -> Remedy {
    Remedy::at_least(
        SubjectRef::new(entity_id, path, copy(label_en, label_de)),
        Quantity::new(QuantityKind::Power, current),
        Quantity::new(QuantityKind::Power, required),
        copy(
            &format!("Raise assumed {label_en} from {current:.0} W/m² to at least {required:.0} W/m²."),
            &format!("Angenommenen Wert ({label_de}) von {current:.0} W/m² auf mindestens {required:.0} W/m² erhöhen."),
        ),
    )
}

/// 🔋 Remedy raising an assumed fire load density to the required value.
pub fn raise_energy_remedy(path: &str, entity_id: &str, label_en: &str, label_de: &str, current: f64, required: f64) -> Remedy {
    Remedy::at_least(
        SubjectRef::new(entity_id, path, copy(label_en, label_de)),
        Quantity::new(QuantityKind::Energy, current),
        Quantity::new(QuantityKind::Energy, required),
        copy(
            &format!("Raise assumed {label_en} from {:.0} MJ/m² to at least {:.0} MJ/m².", current / 1e6, required / 1e6),
            &format!("Angenommenen Wert ({label_de}) von {:.0} MJ/m² auf mindestens {:.0} MJ/m² erhöhen.", current / 1e6, required / 1e6),
        ),
    )
}

/// 🔩 Remedy raising an assumed moment to the required value.
pub fn raise_moment_remedy(path: &str, entity_id: &str, label_en: &str, label_de: &str, current: f64, required: f64) -> Remedy {
    Remedy::at_least(
        SubjectRef::new(entity_id, path, copy(label_en, label_de)),
        Quantity::new(QuantityKind::Moment, current),
        Quantity::new(QuantityKind::Moment, required),
        copy(
            &format!("Raise assumed {label_en} from {:.1} kNm to at least {:.1} kNm.", current / 1000.0, required / 1000.0),
            &format!("Angenommenen Wert ({label_de}) von {:.1} kNm auf mindestens {:.1} kNm erhöhen.", current / 1000.0, required / 1000.0),
        ),
    )
}

/// 🧮 Covering check: the assumed value must cover the required one; utilisation is required over assumed, so failing rows exceed 1 and Warning marks near-limit passes.
pub fn assess_covers(id: &str, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, assumed: Quantity, required: Quantity, explanation: LocalizedCopy, annex: AnnexChoice, remedy: Option<Remedy>) -> CheckResult {
    assess_covers_on(id, part, clause, subject, title, assumed, required, explanation, annex, remedy, &[])
}

/// 🔖 Covering check whose verdict rests on the given unverified NDP ids of the kernel registry, so it reports UnverifiedBasis instead of Pass.
pub fn assess_covers_on(id: &str, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, assumed: Quantity, required: Quantity, explanation: LocalizedCopy, annex: AnnexChoice, remedy: Option<Remedy>, ndps: &[&str]) -> CheckResult {
    let effective = if required.value <= assumed.value + 1e-9 { required.value.min(assumed.value) } else { required.value };
    let capacity = assumed.value.max(effective.abs() * 1e-6).max(f64::MIN_POSITIVE);
    let mut b = CheckResult::assess(id, part, clause, subject, title)
        .utilization(Quantity::new(required.kind, effective), Quantity::new(assumed.kind, capacity))
        .warn_near_limit()
        .annex(annex)
        .explanation(explanation);
    if let Some(r) = remedy { b = b.remedy(r); }
    for ndp in ndps { b = b.ndp(ndp); }
    b.build()
}

/// 🚫 Failing check for an input outside the normative domain, with the admissible values as remedy.
pub fn invalid_input(id: &str, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, explanation: LocalizedCopy, admissible: Vec<String>, annex: AnnexChoice) -> CheckResult {
    CheckResult::assess(id, part, clause, subject.clone(), title)
        .annex(annex)
        .explanation(explanation)
        .status(CheckStatus::Fail)
        .remedy(Remedy::one_of(subject, admissible, copy("Choose one of the admissible values.", "Einen der zulässigen Werte wählen.")))
        .build()
}

/// 🇩🇪 National-annex classifications whose lookup rejects unknown values instead of falling back.
pub mod na_de {
    /// ❄️ Snow load zones of DIN EN 1991-1-3/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SnowZone { Zone1, Zone1a, Zone2, Zone2a, Zone3 }
    impl SnowZone {
        pub const ADMISSIBLE: [&'static str; 5] = ["1", "1a", "2", "2a", "3"];
        pub fn parse(s: &str) -> Option<Self> {
            match s.trim().to_ascii_lowercase().as_str() {
                "1" => Some(Self::Zone1), "1a" => Some(Self::Zone1a), "2" => Some(Self::Zone2), "2a" => Some(Self::Zone2a), "3" => Some(Self::Zone3), _ => None,
            }
        }
    }
    /// 🌬️ Wind zones 1–4 of DIN EN 1991-1-4/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WindZone { Zone1, Zone2, Zone3, Zone4 }
    impl WindZone {
        pub const ADMISSIBLE: [&'static str; 4] = ["1", "2", "3", "4"];
        pub fn from_u8(z: u8) -> Option<Self> { match z { 1 => Some(Self::Zone1), 2 => Some(Self::Zone2), 3 => Some(Self::Zone3), 4 => Some(Self::Zone4), _ => None } }
        pub fn v_b0_m_s(self) -> f64 {
            semio_s_plugin_norm_compliance::ndp_number(match self { Self::Zone1 => "ndp.en1991.vb0.zone1", Self::Zone2 => "ndp.en1991.vb0.zone2", Self::Zone3 => "ndp.en1991.vb0.zone3", Self::Zone4 => "ndp.en1991.vb0.zone4" }, semio_s_plugin_norm_compliance::AnnexChoice::De)
        }
    }
}

/// 🧮 Shared numeric helpers of the clause formulas.
pub mod numeric {
    /// 📈 Linear interpolation of a sorted `(x, y)` table, clamped at both ends.
    pub fn table(rows: &[(f64, f64)], x: f64) -> f64 {
        if x <= rows[0].0 { return rows[0].1; }
        for w in rows.windows(2) {
            if x <= w[1].0 { return w[0].1 + (w[1].1 - w[0].1) * (x - w[0].0) / (w[1].0 - w[0].0); }
        }
        rows[rows.len() - 1].1
    }
}

pub mod part_1_1 {
    use crate::*;
    use semio_s_plugin_norm_compliance::ndp_number;

    /// 🧱 Characteristic bulk density (min, max, recommended) in kN/m³ — EN 1991-1-1 Annex A.
    pub fn density_kn_m3(material: &str, annex: AnnexChoice) -> Option<(f64, f64, f64)> {
        let row = match material {
            "concrete" => (24.0, 24.0, 24.0),
            "reinforced_concrete" => (25.0, 25.0, 25.0),
            "lightweight_concrete" => (9.0, 20.0, 18.0),
            "steel" => (77.0, 78.5, semio_s_plugin_norm_compliance::ndp_number("ndp.en1991.density.steel", annex)),
            "aluminium" => (27.0, 27.0, 27.0),
            "timber" | "softwood" => (3.5, 5.0, 5.0),
            "hardwood" => (6.0, 10.0, 8.0),
            "glulam" => (3.7, 4.4, 4.2),
            "plywood" => (4.5, 7.0, 6.0),
            "masonry" | "brick" => (7.0, 22.0, 18.0),
            "calcium_silicate" => (12.0, 22.0, 18.0),
            "aerated_concrete" => (4.0, 9.0, 6.0),
            "natural_stone" => (27.0, 27.0, 27.0),
            "mortar" => (19.0, 23.0, 21.0),
            "cement_screed" | "screed" => (22.0, 22.0, 22.0),
            "asphalt" => (23.0, 24.0, 23.0),
            "plaster" => (14.0, 14.0, 14.0),
            "glass" => (25.0, 25.0, 25.0),
            "water" => (10.0, 10.0, 10.0),
            "sand" | "loose_sand" => (17.0, 20.0, 18.0),
            "gravel" => (16.0, 20.0, 20.0),
            "clay" => (18.0, 20.0, 19.0),
            "soil_fill" => (16.0, 22.0, 20.0),
            _ => return None,
        };
        Some(row)
    }
    /// 🏷️ Admissible material names of the density table.
    pub const MATERIALS: [&str; 24] = ["concrete", "reinforced_concrete", "lightweight_concrete", "steel", "aluminium", "timber", "softwood", "hardwood", "glulam", "plywood", "masonry", "brick", "calcium_silicate", "aerated_concrete", "natural_stone", "mortar", "cement_screed", "asphalt", "plaster", "glass", "water", "sand", "gravel", "soil_fill"];

    /// 🧱 Recommended bulk density in N/m³, `None` for an unknown material.
    pub fn self_weight_n_m3(material: &str, annex: AnnexChoice) -> Option<f64> { density_kn_m3(material, annex).map(|d| d.2 * 1000.0) }
    /// 🧱 Self-weight pressure γ·t in Pa, `None` for an unknown material.
    pub fn self_weight_pa(material: &str, thickness_m: f64, annex: AnnexChoice) -> Option<f64> { self_weight_n_m3(material, annex).map(|g| g * thickness_m) }

    /// 🏢 (q_k kN/m², Q_k kN) per use category — EN 1991-1-1 Table 6.2 / DIN EN 1991-1-1/NA Table 6.1DE.
    pub fn imposed_row(category: &str, annex: AnnexChoice) -> Option<(f64, f64)> {
        let c = category.trim().to_ascii_uppercase();
        match annex {
            AnnexChoice::De => Some(match c.as_str() {
                "A1" => (1.0, 1.0), "A2" => (1.5, 1.0), "A3" => (2.0, 1.0),
                "B1" => (2.0, 1.0), "B2" => (3.0, 1.5), "B3" => (5.0, 4.0),
                "C1" => (3.0, 4.0), "C2" => (4.0, 4.0), "C3" | "C4" | "C5" => (5.0, 4.0), "C6" => (7.5, 10.0),
                "D1" | "D2" | "D3" => (5.0, 4.0),
                "E1" => (6.0, 7.0),
                "T1" => (2.0, 2.0), "T2" => (3.0, 3.0), "Z" => (4.0, 2.0),
                "F" => (2.5, 20.0), "G" => (5.0, 90.0), "H" => (0.0, 1.0),
                _ => return None,
            }),
            AnnexChoice::En => Some(match c.as_str() {
                "A" => (2.0, 2.0), "B" => (3.0, 4.5),
                "C1" => (3.0, 4.0), "C2" => (4.0, 4.0), "C3" => (4.0, 4.0), "C4" => (5.0, 4.0), "C5" => (5.0, 4.5),
                "D1" => (4.0, 4.0), "D2" => (5.0, 7.0),
                "E1" => (7.5, 7.0),
                "F" => (2.5, 20.0), "G" => (5.0, 90.0), "H" => (0.4, 1.0),
                _ => return None,
            }),
        }
    }
    /// 🏷️ Admissible use categories of the selected annex.
    pub fn categories(annex: AnnexChoice) -> Vec<String> {
        let rows: &[&str] = match annex {
            AnnexChoice::De => &["A1", "A2", "A3", "B1", "B2", "B3", "C1", "C2", "C3", "C4", "C5", "C6", "D1", "D2", "D3", "E1", "T1", "T2", "Z", "F", "G", "H"],
            AnnexChoice::En => &["A", "B", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "E1", "F", "G", "H"],
        };
        rows.iter().map(|s| s.to_string()).collect()
    }
    /// 🏢 Imposed q_k (Pa) of a use category, `None` for an unknown category.
    pub fn imposed_qk_pa(category: &str, annex: AnnexChoice) -> Option<f64> { imposed_row(category, annex).map(|r| kn_m2_to_pa(r.0)) }
    /// 📌 Concentrated Q_k (N) of a use category, `None` for an unknown category.
    pub fn imposed_qk_concentrated_n(category: &str, annex: AnnexChoice) -> Option<f64> { imposed_row(category, annex).map(|r| kn_to_n(r.1)) }

    /// 🗝️ The shared EN 1990 ψ registry key (`ndp.en1990.psi.<key>`) of an imposed-load category letter; unknown letters follow category B.
    pub fn psi_key(category: &str) -> &'static str {
        match category.trim().to_ascii_uppercase().chars().next() {
            Some('A' | 'T' | 'Z') => "residential",
            Some('C') => "congregation",
            Some('D') => "retail",
            Some('E') => "storage",
            Some('F') => "traffic_light",
            Some('G') => "traffic_heavy",
            Some('H') => "roof",
            _ => "office",
        }
    }

    /// 🎛️ The combination factors `(ψ₀, ψ₁, ψ₂)` of a registry key (EN 1990 Table A1.1), read from the one shared registry in `norm_compliance`.
    pub fn psi_factors_of_key(key: &str, annex: AnnexChoice) -> (f64, f64, f64) {
        let at = |index: usize| ndp_number(&format!("ndp.en1990.psi.{key}.{index}"), annex);
        (at(0), at(1), at(2))
    }

    /// 🎛️ The combination factors `(ψ₀, ψ₁, ψ₂)` of a use category in an annex.
    pub fn psi_factors(category: &str, annex: AnnexChoice) -> (f64, f64, f64) {
        psi_factors_of_key(psi_key(category), annex)
    }

    /// 🎛️ Combination factor ψ₀ of a use category (EN 1990 Table A1.1, shared registry, annex EN value).
    pub fn psi_0(category: &str) -> f64 {
        psi_factors(category, AnnexChoice::En).0
    }

    /// 📐 Area reduction α_A: DE `0.7 ≤ 0.5 + 10/√A ≤ 1.0` for C–E (repo NA value); EN Eq. (6.1) `5/7 ψ₀ + A₀/A ≤ 1.0` for A–D with a floor of 0.6 for C and D.
    pub fn alpha_a(category: &str, area_m2: f64, annex: AnnexChoice) -> f64 {
        let c = category.trim().to_ascii_uppercase();
        let first = c.chars().next();
        let a = area_m2.max(1.0);
        match annex {
            AnnexChoice::De => if matches!(first, Some('C' | 'D' | 'E')) { (ndp_number("ndp.en1991.alpha-a.de-c0", annex) + ndp_number("ndp.en1991.alpha-a.de-c1", annex) / a.sqrt()).clamp(ndp_number("ndp.en1991.alpha-a.de-min", annex), ndp_number("ndp.en1991.alpha-a.de-max", annex)) } else { 1.0 },
            AnnexChoice::En => match first {
                Some('A' | 'B') => (5.0 / 7.0 * psi_0(&c) + ndp_number("ndp.en1991.alpha-a.en-a0", annex) / a).min(1.0),
                Some('C' | 'D') => (5.0 / 7.0 * psi_0(&c) + ndp_number("ndp.en1991.alpha-a.en-a0", annex) / a).clamp(ndp_number("ndp.en1991.alpha-a.en-min-cd", annex), 1.0),
                _ => 1.0,
            },
        }
    }

    /// 🏬 Storey reduction α_n = (2 + (n−2)ψ₀)/n for n > 2 storeys, categories A–D (EN 1991-1-1 Eq. 6.2).
    pub fn alpha_n(category: &str, storey_count: u8, _annex: AnnexChoice) -> f64 {
        let c = category.trim().to_ascii_uppercase();
        if !matches!(c.chars().next(), Some('A' | 'B' | 'C' | 'D')) || storey_count <= 2 { return 1.0; }
        let n = storey_count as f64;
        ((2.0 + (n - 2.0) * psi_0(&c)) / n).min(1.0)
    }

    /// 🏢 Reduced imposed q_k (Pa), `None` for an unknown category.
    pub fn imposed_qk_reduced_pa(category: &str, annex: AnnexChoice, area_m2: f64, storey_count: u8) -> Option<f64> {
        imposed_qk_pa(category, annex).map(|q| q * alpha_a(category, area_m2, annex) * alpha_n(category, storey_count, annex))
    }

    /// 🧱 Allowance for movable partitions (Pa) from the wall line load g (kN/m), `None` when the wall must be applied individually.
    pub fn partition_allowance_pa(wall_kn_m: f64, annex: AnnexChoice) -> Option<f64> {
        (1..=3)
            .map(|band| (ndp_number(&format!("ndp.en1991.partition.band{band}.limit"), annex), ndp_number(&format!("ndp.en1991.partition.band{band}.q"), annex)))
            .find(|(limit, _)| wall_kn_m <= *limit)
            .map(|(_, q)| kn_m2_to_pa(q))
    }

    /// 🧱 Smallest partition allowance of the annex (Pa) used when the wall load is not given.
    pub fn partitions_allowance_pa(annex: AnnexChoice) -> f64 { partition_allowance_pa(0.0, annex).unwrap_or(0.0) }
}

pub mod part_1_3 {
    use crate::*;
    use semio_s_plugin_norm_compliance::ndp_number;
    use na_de::SnowZone;

    /// ❄️ Characteristic ground snow load s_k (Pa) of the NA-DE zones with altitude formula, `None` for an unknown zone.
    pub fn ground_snow_pa(zone: &str, altitude_m: f64) -> Option<f64> {
        let z = SnowZone::parse(zone)?;
        let t = ((altitude_m.max(0.0) + ndp_number("ndp.en1991.1-3.snow-altitude-offset", AnnexChoice::De)) / ndp_number("ndp.en1991.1-3.snow-altitude-scale", AnnexChoice::De)).powi(2);
        let key = match z { SnowZone::Zone1 => "z1", SnowZone::Zone1a => "z1a", SnowZone::Zone2 => "z2", SnowZone::Zone2a => "z2a", SnowZone::Zone3 => "z3" };
        let cell = |column: &str| ndp_number(&format!("ndp.en1991.1-3.snow-zone.{key}.{column}"), AnnexChoice::De);
        let kn = cell("factor") * (cell("a") + cell("b") * t).max(cell("min"));
        Some(kn_m2_to_pa(kn))
    }
    /// ❄️ Ground snow load of the selected annex (Pa): NA-DE zone formula or the explicit EN value.
    pub fn design_ground_snow_pa(annex: AnnexChoice, zone: &str, altitude_m: f64, en_sk_pa: f64) -> Option<f64> {
        match annex { AnnexChoice::De => ground_snow_pa(zone, altitude_m), AnnexChoice::En => Some(en_sk_pa) }
    }
    /// ❄️ Exceptional snow load s_Ad = 2.3·s_k of the North German lowland (Pa).
    pub fn exceptional_snow_pa(sk_pa: f64) -> f64 { ndp("ndp.en1991.1-3.exceptional-factor") * sk_pa }
    /// 🏔️ Shape coefficient μ₁(α) of EN 1991-1-3 Table 5.2.
    pub fn mu_1(alpha_deg: f64) -> f64 {
        let points = [(0.0, ndp("ndp.en1991.1-3.mu1.a0")), (30.0, ndp("ndp.en1991.1-3.mu1.a30")), (60.0, ndp("ndp.en1991.1-3.mu1.a60"))];
        numeric::table(&points, alpha_deg.max(0.0))
    }
    /// 🏔️ Shape coefficient μ₂(α) = 0.8 + 0.8α/30 for valleys of multi-span roofs.
    pub fn mu_2(alpha_deg: f64) -> f64 { ndp("ndp.en1991.1-3.mu2.base") + ndp("ndp.en1991.1-3.mu2.slope") * alpha_deg.max(0.0) / ndp("ndp.en1991.1-3.mu2.alpha-ref") }
    /// 🛢️ Upper bound μ₃ = 2.0 used for cylindrical roofs, whose rise and span the roof record does not carry.
    pub fn cylindrical_mu_3_cap() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-3.cylindrical-mu3-cap")
    }
    /// 🛢️ Shape coefficient μ₃ = 0.2 + 10 h/b ≤ 2.0 of cylindrical roofs.
    pub fn mu_3(h_m: f64, b_m: f64) -> f64 { (ndp("ndp.en1991.1-3.mu3.base") + ndp("ndp.en1991.1-3.mu3.slope") * h_m / b_m.max(1e-9)).min(cylindrical_mu_3_cap()) }
    /// 🧱 Drift behind an obstruction: (raw γh/s_k, capped μ₂, drift length l_s) with γ = 2 kN/m³, 0.8 ≤ μ₂ ≤ cap (2.0 NA-DE, 4.0 EN) and 5 m ≤ l_s ≤ 15 m.
    pub fn obstruction_drift(h_m: f64, sk_pa: f64, annex: AnnexChoice) -> (f64, f64, f64) {
        let raw = 2000.0 * h_m / sk_pa.max(1e-9);
        let cap = ndp_number("ndp.en1991.snow.drift-mu2-cap", annex);
        (raw, raw.clamp(0.8, cap), (2.0 * h_m).clamp(5.0, 15.0))
    }
    /// 🧱 Wind-induced drift coefficient μ_w = (b₁ + b₂)/(2h) of a roof step (EN 1991-1-3, 5.3.6), limited by γh/s_k and to 0.8 ≤ μ_w ≤ cap (2.0 NA-DE, 4.0 EN); the NDP `drift-mu-w-reading` selects whether the 0.8 floor (0, conservative, default) or γh/s_k (1) wins when γh/s_k < 0.8 (unverified standard reading).
    pub fn drift_mu_w(h_m: f64, b1_m: f64, b2_m: f64, sk_pa: f64, annex: AnnexChoice) -> f64 {
        drift_mu_w_reading(h_m, b1_m, b2_m, sk_pa, ndp_number("ndp.en1991.snow.drift-mu2-cap", annex), ndp("ndp.en1991.1-3.drift-mu-w-reading"))
    }
    /// 🔀 μ_w for an explicit reading of the 0.8 floor: below 0.5 the floor wins over γh/s_k, from 0.5 γh/s_k caps μ_w when it is below 0.8.
    pub fn drift_mu_w_reading(h_m: f64, b1_m: f64, b2_m: f64, sk_pa: f64, cap: f64, reading: f64) -> f64 {
        let (geometric, weight_limit) = ((b1_m + b2_m) / (2.0 * h_m.max(1e-9)), 2000.0 * h_m / sk_pa.max(1e-9));
        if reading < 0.5 { geometric.min(weight_limit).clamp(0.8, cap) } else { geometric.max(0.8).min(cap).min(weight_limit) }
    }
    /// 📈 Shape coefficient at a plan position along a drift triangle: linear between `base` and `peak` over `length` from `start` (rising towards an obstruction behind the drift, falling away from one), `base` outside (third-party oracle `en1991.snow.profile-mu`).
    pub fn drift_mu_at(base: f64, start: f64, length: f64, peak: f64, rising: bool, x: f64) -> f64 {
        if length <= 1e-9 || x < start - 1e-9 || x > start + length + 1e-9 { return base; }
        let t = ((x - start) / length).clamp(0.0, 1.0);
        base + (peak - base).max(0.0) * if rising { t } else { 1.0 - t }
    }
    /// 🛷 Sliding coefficient μ_s of a roof step: the registry share (unverified) of μ₁ of the upper roof when its pitch exceeds 15°, else 0.
    pub fn sliding_mu_s(upper_pitch_deg: f64) -> f64 { if upper_pitch_deg > 15.0 { ndp("ndp.en1991.1-3.sliding-share") * mu_1(upper_pitch_deg) } else { 0.0 } }
    /// 🪜 Height h (m) of the step a roof surface leaves behind: its rise over the extent, zero for flat roofs.
    pub fn roof_step_height_m(shape: &str, pitch_deg: f64, extent_m: f64) -> f64 { if shape.eq_ignore_ascii_case("flat") { 0.0 } else { extent_m.max(0.0) * pitch_deg.clamp(0.0, 60.0).to_radians().tan() } }
    /// 🧊 Eaves overhang line load s_e = k s²/γ (N/m) with γ = 3 kN/m³ and k = min(3/d, dγ).
    pub fn overhang_line_load_n_m(s_pa: f64) -> f64 {
        let s = s_pa / 1000.0;
        let gamma = 3.0;
        let d = s / gamma;
        let k = (3.0 / d.max(1e-9)).min(d * gamma);
        k * s * s / gamma * 1000.0
    }
    /// 🥅 Snow guard force F_s = s·b·sin α (N/m).
    pub fn snow_guard_force_n_m(s_pa: f64, b_m: f64, alpha_deg: f64) -> f64 { s_pa * b_m * alpha_deg.to_radians().sin() }
    /// 📆 Return-period factor s_n/s_k (EN 1991-1-3 Eq. D.1) for the coefficient of variation V and annual exceedance probability P.
    pub fn return_period_factor(v: f64, p: f64) -> f64 { (1.0 - v * (0.45 + 0.78 * (-(1.0 - p).ln()).ln())) / (1.0 + 2.59 * v) }
    /// 🏠 Shape coefficient of a roof area: pitched roofs μ₁(α), multi-span valleys μ₂(α), parapets and drift obstructions raise μ.
    pub fn shape_mu(roof_type: &str, pitch_deg: f64, has_parapet: bool, parapet_height: f64, drift_obstruction_height: f64, multi_span: bool, sk_pa: f64, annex: AnnexChoice) -> f64 {
        let mut mu = if roof_type.contains("cylind") { cylindrical_mu_3_cap() } else { mu_1(pitch_deg) };
        if multi_span || roof_type.contains("multi") { mu = mu.max(mu_2(pitch_deg).min(ndp("ndp.en1991.1-3.mu2.multi-span-cap"))); }
        let h = if has_parapet { parapet_height.max(drift_obstruction_height) } else { drift_obstruction_height };
        if h > 0.0 { mu = mu.max(obstruction_drift(h, sk_pa, annex).1); }
        mu
    }
    /// 🏠 Roof snow load s = μ·C_e·C_t·s_k (Pa).
    pub fn roof_snow_pa(s_k_pa: f64, mu: f64, c_e: f64, c_t: f64) -> f64 { mu * c_e * c_t * s_k_pa }
}

pub mod part_1_4 {
    use crate::*;
    /// 🏞️ Terrain categories 0, I–IV of EN 1991-1-4 Table 4.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TerrainCategory { Zero, I, II, III, IV }
    impl TerrainCategory {
        pub fn from_u8(v: u8) -> Option<Self> { match v { 0 => Some(Self::Zero), 1 => Some(Self::I), 2 => Some(Self::II), 3 => Some(Self::III), 4 => Some(Self::IV), _ => None } }
        fn key(self) -> &'static str { match self { Self::Zero => "zero", Self::I => "i", Self::II => "ii", Self::III => "iii", Self::IV => "iv" } }
        pub fn z_0_m(self) -> f64 { ndp(&format!("ndp.en1991.1-4.terrain.{}.z0", self.key())) }
        pub fn z_min_m(self) -> f64 { ndp(&format!("ndp.en1991.1-4.terrain.{}.zmin", self.key())) }
    }
    /// 🌬️ Basic wind velocity v_b (m/s), `None` for an unknown NA-DE wind zone.
    pub fn design_vb(annex: AnnexChoice, zone: u8, en_vb: f64) -> Option<f64> {
        match annex { AnnexChoice::De => na_de::WindZone::from_u8(zone).map(|z| z.v_b0_m_s()), AnnexChoice::En => Some(en_vb) }
    }
    /// 🌬️ Basic velocity pressure q_b = ½ρv_b² (Pa).
    pub fn basic_velocity_pressure_pa(rho: f64, vb: f64) -> f64 { 0.5 * rho * vb * vb }
    /// 📏 Terrain factor k_r = 0.19 (z₀/0.05)^0.07.
    pub fn k_r(terrain: TerrainCategory) -> f64 { ndp("ndp.en1991.1-4.k-r.coef") * (terrain.z_0_m() / ndp("ndp.en1991.1-4.k-r.z0-ref")).powf(ndp("ndp.en1991.1-4.k-r.exp")) }
    /// 📏 Roughness factor c_r(z) = k_r ln(max(z, z_min)/z₀).
    pub fn c_r(z_m: f64, terrain: TerrainCategory) -> f64 { k_r(terrain) * (z_m.max(terrain.z_min_m()) / terrain.z_0_m()).ln() }
    /// 🌬️ Mean wind velocity v_m(z) = c_r(z) c_o(z) v_b (Eq. 4.3) in m/s.
    pub fn mean_wind_velocity(z_m: f64, terrain: TerrainCategory, v_b: f64, c_o: f64) -> f64 { c_r(z_m, terrain) * c_o * v_b }
    /// 🌀 Turbulence intensity I_v(z) = k_I/(c_o ln(z/z₀)) with k_I = 1.
    pub fn turbulence_intensity(z_m: f64, terrain: TerrainCategory, c_o: f64) -> f64 { 1.0 / (c_o * (z_m.max(terrain.z_min_m()) / terrain.z_0_m()).ln()) }
    /// ⛰️ Orography factor c_o from the upwind slope Φ = H/L_u and the position factor s.
    pub fn orography_factor(phi: f64, s: f64) -> f64 { if phi < 0.05 { 1.0 } else if phi <= 0.3 { 1.0 + 2.0 * s * phi } else { 1.0 + 0.6 * s } }
    /// 💨 Exposure factor c_e(z) = (1 + 7 I_v)(c_r c_o)², so that q_p = c_e q_b.
    pub fn exposure_factor(z_m: f64, terrain: TerrainCategory, c_o: f64) -> f64 {
        let cr = c_r(z_m, terrain);
        (1.0 + 7.0 * turbulence_intensity(z_m, terrain, c_o)) * (cr * c_o).powi(2)
    }
    /// 💨 EN peak velocity pressure q_p(z) = (1 + 7 I_v)·½ρ(c_r c_o v_b)² (Pa).
    pub fn en_peak_pressure_pa(z_m: f64, terrain: TerrainCategory, vb: f64, rho: f64, c_o: f64) -> f64 { exposure_factor(z_m, terrain, c_o) * basic_velocity_pressure_pa(rho, vb) }
    /// 🇩🇪 NA-DE simplified profile of the inland (`coast == false`) or the Baltic coast and islands: q_p(z) (Pa) from q_b,0.
    pub fn na_profile_pa(qb0_pa: f64, z_m: f64, coast: bool) -> f64 {
        let z = z_m.max(1.0);
        let kind = if coast { "coast" } else { "inland" };
        let cell = |column: &str| ndp(&format!("ndp.en1991.1-4.na-profile.{kind}.{column}"));
        let f = if z <= cell("limit1") { cell("c1") } else if z <= cell("limit2") { cell("c2") * (z / 10.0).powf(cell("e2")) } else { cell("c3") * (z / 10.0).powf(cell("e3")) };
        f * qb0_pa
    }
    fn profile_pa(annex: AnnexChoice, zone: u8, terrain: u8, z_m: f64, vb: f64, rho: f64, c_o: f64, coast: bool) -> Option<f64> {
        let cat = TerrainCategory::from_u8(terrain)?;
        if annex == AnnexChoice::En || matches!(cat, TerrainCategory::Zero | TerrainCategory::I) {
            return Some(en_peak_pressure_pa(z_m, cat, vb, rho, c_o));
        }
        let qb0 = basic_velocity_pressure_pa(rho, na_de::WindZone::from_u8(zone)?.v_b0_m_s());
        Some(na_profile_pa(qb0, z_m, coast) * c_o * c_o)
    }
    /// 💨 Peak velocity pressure q_p(z_e) (Pa) with orography and the linear transition over `mixed_distance` from the upwind category; `None` for an unknown zone or category.
    pub fn peak_velocity_pressure_pa(annex: AnnexChoice, zone: u8, terrain: u8, z_m: f64, vb: f64, rho: f64, orography: f64, mixed_upwind: u8, mixed_distance: f64, coast: bool) -> Option<f64> {
        let c_o = orography.max(1.0);
        let here = profile_pa(annex, zone, terrain, z_m, vb, rho, c_o, coast)?;
        if mixed_distance > 0.0 && mixed_distance < 2000.0 {
            let t = (mixed_distance / 2000.0).clamp(0.0, 1.0);
            let upwind = profile_pa(annex, zone, mixed_upwind, z_m, vb, rho, c_o, coast)?;
            return Some(upwind * (1.0 - t) + here * t);
        }
        Some(here)
    }

    /// 🧭 Reference dimension e = min(b, 2h) (m) — EN 1991-1-4 Fig. 7.5.
    pub fn reference_e(b_m: f64, h_m: f64) -> f64 { b_m.min(2.0 * h_m) }

    /// 📏 Reference height z_e of the strip containing height z — EN 1991-1-4 Fig. 7.4.
    pub fn reference_height_ze(z_m: f64, h_m: f64, b_m: f64) -> f64 {
        let b = b_m.max(0.1);
        let h = h_m.max(0.1);
        if h <= b { h } else if h <= 2.0 * b { if z_m <= b { b } else { h } } else if z_m <= b { b } else if z_m >= h - b { h } else { z_m }
    }

    /// 📐 Area-dependent external pressure coefficient — EN 1991-1-4 Fig. 7.2: c_pe,1 for A ≤ 1 m², c_pe,10 for A ≥ 10 m², c_pe,1 − (c_pe,1 − c_pe,10) log₁₀A between.
    pub fn area_cpe(c_pe1: f64, c_pe10: f64, area_m2: f64) -> f64 {
        if area_m2 <= 1.0 { c_pe1 } else if area_m2 >= 10.0 { c_pe10 } else { c_pe1 - (c_pe1 - c_pe10) * area_m2.log10() }
    }

    /// 🧱 Widths of the wall zones A, B, C for e = min(b, 2h) and the along-wind depth d — Fig. 7.5 (zone C is 0 when e ≥ d).
    pub fn wall_zone_widths(e_m: f64, d_m: f64) -> (f64, f64, f64) {
        if e_m >= 5.0 * d_m { (d_m, 0.0, 0.0) } else if e_m >= d_m { (e_m / 5.0, d_m - e_m / 5.0, 0.0) } else { (e_m / 5.0, 4.0 * e_m / 5.0, d_m - e_m) }
    }

    /// 🧱 (c_pe,10, c_pe,1) of the vertical wall zones A–E, interpolated linearly in h/d between 0.25 and 5 — EN 1991-1-4 Table 7.1.
    pub fn wall_cpe(zone: &str, h_over_d: f64) -> Option<(f64, f64)> {
        let zone = zone.trim().to_ascii_lowercase();
        if !matches!(zone.as_str(), "a" | "b" | "c" | "d" | "e") { return None; }
        let x = h_over_d.clamp(0.25, 5.0);
        let prefix = format!("ndp.en1991.1-4.cpe.wall.{zone}");
        Some((ndp_grid(&prefix, &[("h0-25", 0.25), ("h1", 1.0), ("h5", 5.0)], "c10", x), ndp_grid(&prefix, &[("h0-25", 0.25), ("h1", 1.0), ("h5", 5.0)], "c1", x)))
    }

    fn ndp_grid(prefix: &str, axis: &[(&str, f64)], column: &str, x: f64) -> f64 {
        let points: Vec<(f64, f64)> = axis.iter().map(|(label, at)| (*at, ndp(&format!("{prefix}.{label}.{column}")))).collect();
        numeric::table(&points, x)
    }

    /// 🏢 Zone areas (F each, G, H, I) of a flat roof with e = min(b, 2h) — Fig. 7.6: F = e/4·e/10, G = (b − e/2)·e/10, H = (e/2 − e/10)·b, I = rest.
    pub fn flat_roof_zone_areas(e_m: f64, b_m: f64, d_m: f64) -> (f64, f64, f64, f64) {
        let f = e_m / 4.0 * e_m / 10.0;
        let g = (b_m - 2.0 * e_m / 4.0) * e_m / 10.0;
        let h = (e_m / 2.0 - e_m / 10.0) * b_m;
        (f, g, h, b_m * d_m - 2.0 * f - g - h)
    }

    fn flat_zone(zone: &str, kind: &str, axis: &[(&str, f64)], x: f64) -> Option<Vec<(f64, f64)>> {
        let zone = zone.trim().to_ascii_lowercase();
        if zone == "i" { return Some(vec![(ndp("ndp.en1991.1-4.cpe.flat.i.minus"), ndp("ndp.en1991.1-4.cpe.flat.i.minus")), (ndp("ndp.en1991.1-4.cpe.flat.i.plus"), ndp("ndp.en1991.1-4.cpe.flat.i.plus"))]); }
        if !matches!(zone.as_str(), "f" | "g" | "h") { return None; }
        let prefix = format!("ndp.en1991.1-4.cpe.flat.{kind}.{zone}");
        Some(vec![(ndp_grid(&prefix, axis, "c10", x), ndp_grid(&prefix, axis, "c1", x))])
    }

    /// 🏢 Flat-roof coefficients (c_pe,10, c_pe,1) per load case for sharp eaves with a parapet ratio h_p/h — EN 1991-1-4 Table 7.2.
    pub fn flat_roof_parapet_cpe(zone: &str, hp_over_h: f64) -> Option<Vec<(f64, f64)>> {
        flat_zone(zone, "parapet", &[("hp0", 0.0), ("hp0-025", 0.025), ("hp0-05", 0.05), ("hp0-10", 0.10)], hp_over_h.max(0.0))
    }
    /// 🌀 Flat-roof coefficients with curved eaves of radius ratio r/h — Table 7.2.
    pub fn flat_roof_curved_cpe(zone: &str, r_over_h: f64) -> Option<Vec<(f64, f64)>> {
        flat_zone(zone, "curved", &[("r0-05", 0.05), ("r0-10", 0.10), ("r0-20", 0.20)], r_over_h)
    }
    /// 🏘️ Flat-roof coefficients with mansard eaves of slope angle α (30°, 45°, 60°) — Table 7.2.
    pub fn flat_roof_mansard_cpe(zone: &str, alpha_deg: f64) -> Option<Vec<(f64, f64)>> {
        flat_zone(zone, "mansard", &[("a30", 30.0), ("a45", 45.0), ("a60", 60.0)], alpha_deg)
    }

    type Pair = (f64, f64);
    const PITCHED_ANGLES: [i32; 10] = [-45, -30, -15, -5, 5, 15, 30, 45, 60, 75];

    fn pitched_pair(zone: &str, series: &str, angle: i32) -> Option<Pair> {
        let key = if angle < 0 { format!("m{}", -angle) } else { format!("p{angle}") };
        let cell = |column: &str| semio_s_plugin_norm_compliance::ndp_cell_opt("ndp.en1991.1-4.cpe.pitched0", &[zone, series, &key, column], AnnexChoice::En);
        Some((cell("c10")?, cell("c1")?))
    }

    /// 🏠 Pitched-roof coefficients (c_pe,10, c_pe,1) per load case for wind θ = 0° and zones F–J: the suction and the pressure series are interpolated in the pitch angle separately, each only between rows that carry it — EN 1991-1-4 Tables 7.3a/7.4a.
    pub fn pitched_roof_cpe(zone: &str, pitch_deg: f64) -> Option<Vec<(f64, f64)>> {
        let zone = zone.trim().to_ascii_lowercase();
        if !matches!(zone.as_str(), "f" | "g" | "h" | "i" | "j") { return None; }
        let a = pitch_deg.clamp(-45.0, 75.0);
        let mut suction: Vec<(f64, Pair)> = Vec::new();
        let mut pressure: Vec<(f64, Pair)> = Vec::new();
        for angle in PITCHED_ANGLES {
            let primary = pitched_pair(&zone, "primary", angle)?;
            if primary.0 > 0.0 { pressure.push((f64::from(angle), primary)); } else { suction.push((f64::from(angle), primary)); }
            if let Some(p) = pitched_pair(&zone, "secondary", angle) { pressure.push((f64::from(angle), p)); }
        }
        let at = |series: &[(f64, Pair)]| -> Option<Pair> {
            let lower = series.iter().rev().find(|r| r.0 <= a)?;
            let upper = series.iter().find(|r| r.0 >= a)?;
            if (upper.0 - lower.0).abs() < 1e-12 { return Some(lower.1); }
            let t = (a - lower.0) / (upper.0 - lower.0);
            Some((lower.1.0 + (upper.1.0 - lower.1.0) * t, lower.1.1 + (upper.1.1 - lower.1.1) * t))
        };
        let out: Vec<Pair> = [at(&suction), at(&pressure)].into_iter().flatten().collect();
        if out.is_empty() { None } else { Some(out) }
    }

    /// 🛖️ The zones F to N of the hipped-roof partition (EN 1991-1-4 Figure 7.9); their coefficients (Table 7.5) are unverified registry rows `ndp.en1991.1-4.hipped-cpe10.<zone>` and `hipped-cpe1.<zone>` that stay unset until the table is supplied.
    pub const HIPPED_ZONES: [&str; 9] = ["F", "G", "H", "I", "J", "K", "L", "M", "N"];
    /// 🛖️ Whether the roof type is a hipped roof.
    pub fn is_hipped_roof(roof_type: &str) -> bool { roof_type.to_ascii_lowercase().contains("hip") }
    /// 🛖️ Whether the zone letter belongs to the hipped-roof partition.
    pub fn is_hipped_zone(zone: &str) -> bool { HIPPED_ZONES.contains(&zone.trim().to_ascii_uppercase().as_str()) }
    /// 🛖️ Registry id of the hipped-roof coefficient of a zone; `kind` is `cpe10` or `cpe1`.
    pub fn hipped_cpe_ndp_id(kind: &str, zone: &str) -> String { format!("ndp.en1991.1-4.hipped-{kind}.{}", zone.trim().to_ascii_lowercase()) }
    /// 🛖️ The registered (c_pe,10, c_pe,1) of a hipped-roof zone, `None` while either cell is unset.
    pub fn hipped_cpe(annex: semio_s_plugin_norm_compliance::AnnexChoice, zone: &str) -> Option<(f64, f64)> {
        Some((semio_s_plugin_norm_compliance::ndp_number_opt(&hipped_cpe_ndp_id("cpe10", zone), annex)?, semio_s_plugin_norm_compliance::ndp_number_opt(&hipped_cpe_ndp_id("cpe1", zone), annex)?))
    }

    /// 🏠 Admissible (c_pe,10, c_pe,1) pairs of a wind face: wall zones A–E from h/d, flat roofs from the parapet ratio, pitched roofs from the pitch angle.
    pub fn face_cpe_candidates(zone: &str, h_over_d: f64, roof_type: Option<&str>, pitch_deg: f64, hp_over_h: f64) -> Option<Vec<(f64, f64)>> {
        let z = zone.trim().to_ascii_uppercase();
        if matches!(z.as_str(), "A" | "B" | "C" | "D" | "E") { return wall_cpe(&z, h_over_d).map(|p| vec![p]); }
        let kind = roof_type?;
        if kind.contains("flat") { flat_roof_parapet_cpe(&z, hp_over_h) } else { pitched_roof_cpe(&z, pitch_deg) }
    }

    /// ⚖️ Internal-pressure alternatives c_pi: the two simplified cases +0.2 and −0.3 (EN 1991-1-4 7.2.9(6)).
    pub fn c_pi_cases() -> [f64; 2] {
        [semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-4.c-pi.1"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-4.c-pi.2")]
    }
    /// 🚪 Internal pressure of a dominant opening (EN 1991-1-4 7.2.9 (5)): 0.75·c_pe at an opening-area ratio of 2, 0.90·c_pe from 3, linear interpolation in between (Expressions 7.1, 7.2).
    pub fn dominant_opening_cpi(c_pe: f64, area_ratio: f64) -> Option<f64> { if area_ratio < 2.0 { None } else { Some(c_pe * (0.75 + 0.15 * (area_ratio.min(3.0) - 2.0))) } }
    /// 🌬️ Net wind pressure w = q_p (c_pe − c_pi) c_s c_d (Pa, signed: positive pressure, negative suction).
    pub fn wind_pressure_pa(qp_pa: f64, c_pe: f64, c_pi: f64, c_s: f64, c_d: f64) -> f64 { qp_pa * (c_pe - c_pi) * c_s * c_d }
    /// 🌬️ Governing net wind pressure magnitude (Pa) over the internal-pressure cases and the user-given c_pi.
    pub fn governing_wind_pressure_pa(qp_pa: f64, c_pe: f64, c_pi_user: f64, c_s: f64, c_d: f64) -> f64 {
        c_pi_cases().iter().chain(std::iter::once(&c_pi_user)).map(|c_pi| wind_pressure_pa(qp_pa, c_pe, *c_pi, c_s, c_d).abs()).fold(0.0, f64::max)
    }
    /// 🌀 Critical vortex-shedding velocity v_crit = b n/St (m/s).
    pub fn vortex_critical_velocity(b_m: f64, n_hz: f64, st: f64) -> f64 { b_m * n_hz / st }
    /// 🧵 Frictional drag F_fr = c_fr q_p A_fr (N).
    pub fn friction_force_n(c_fr: f64, qp_pa: f64, a_fr_m2: f64) -> f64 { c_fr * qp_pa * a_fr_m2 }
    /// 🌀 Dynamic factor pieces of EN 1991-1-4 Annex B for a building: (z_s, L, B², n₁, S_L, R², ν, k_p, c_s c_d).
    pub fn structural_factor(h_m: f64, b_m: f64, terrain: TerrainCategory, vb: f64, delta: f64, n1_hz: f64) -> [f64; 9] {
        let zs = (0.6 * h_m).max(terrain.z_min_m());
        let alpha = 0.67 + 0.05 * terrain.z_0_m().ln();
        let l = 300.0 * (zs / 200.0).powf(alpha);
        let b2 = 1.0 / (1.0 + 0.9 * ((b_m + h_m) / l).powf(0.63));
        let vm = c_r(zs, terrain) * vb;
        let f_l = n1_hz * l / vm;
        let s_l = 6.8 * f_l / (1.0 + 10.2 * f_l).powf(5.0 / 3.0);
        let admittance = |eta: f64| if eta <= 0.0 { 1.0 } else { 1.0 / eta - (1.0 - (-2.0 * eta).exp()) / (2.0 * eta * eta) };
        let r2 = std::f64::consts::PI.powi(2) / (2.0 * delta) * s_l * admittance(4.6 * h_m / l * f_l) * admittance(4.6 * b_m / l * f_l);
        let nu = (n1_hz * (r2 / (b2 + r2)).sqrt()).max(0.08);
        let root = (2.0 * (nu * 600.0).ln()).sqrt();
        let k_p = (root + 0.6 / root).max(3.0);
        let iv = turbulence_intensity(zs, terrain, 1.0);
        [zs, l, b2, n1_hz, s_l, r2, nu, k_p, (1.0 + 2.0 * k_p * iv * (b2 + r2).sqrt()) / (1.0 + 7.0 * iv)]
    }
    /// 🌉 Deck depth d_tot (m) of the bridge wind reference area (EN 1991-1-4, Table 8.1): the deck depth plus the larger of both parapets and the traffic (registry increments, unverified); `None` for an unknown parapet or traffic kind.
    pub fn bridge_depth_m(deck_depth_m: f64, parapet_kind: u8, parapet_height_m: f64, traffic_kind: u8) -> Option<f64> {
        let parapet = match parapet_kind { 0 => 0.0, 1 => 2.0 * ndp("ndp.en1991.8.parapet-open-add"), 2 => 2.0 * parapet_height_m.max(0.0), _ => return None };
        let traffic = match traffic_kind { 0 => 0.0, 1 => ndp("ndp.en1991.8.traffic-road-add"), 2 => ndp("ndp.en1991.8.traffic-rail-add"), _ => return None };
        Some(deck_depth_m + parapet.max(traffic))
    }
    /// 🌉 Force coefficient c_f,x0 of the bridge deck: the digitised plateaus of Fig. 8.3 (unverified) joined linearly between b/d_tot = 0.5 and 4.
    pub fn bridge_c_fx0(width_m: f64, depth_tot_m: f64) -> f64 {
        let (r, b0, b1) = (width_m / depth_tot_m.max(1e-9), ndp("ndp.en1991.8.cfx0-b-narrow"), ndp("ndp.en1991.8.cfx0-b-wide"));
        let (c0, c1) = (ndp("ndp.en1991.8.cfx0-narrow"), ndp("ndp.en1991.8.cfx0-wide"));
        if r <= b0 { c0 } else if r >= b1 { c1 } else { c0 + (c1 - c0) * (r - b0) / (b1 - b0) }
    }
    /// 🌉 Horizontal wind force per metre F_W,x = q_p c_f,x d_tot (N/m) — EN 1991-1-4 Eq. (8.1) with C = c_e c_f,x.
    pub fn bridge_force_x_n_m(qp_pa: f64, c_fx: f64, depth_tot_m: f64) -> f64 { qp_pa * c_fx * depth_tot_m }
    /// 🌉 Vertical wind force per metre F_W,z = q_p c_f,z b (N/m), with c_f,z from the registry (unverified).
    pub fn bridge_force_z_n_m(qp_pa: f64, width_m: f64) -> f64 { qp_pa * ndp("ndp.en1991.8.cfz") * width_m }
    /// 🌉 Longitudinal wind force per metre (N/m): the registry share (unverified) of F_W,x.
    pub fn bridge_force_y_n_m(force_x_n_m: f64) -> f64 { ndp("ndp.en1991.8.longitudinal") * force_x_n_m }
    /// 🌀 Whether the dynamic procedure can be skipped, c_s c_d = 1 (EN 1991-1-4, 6.2): heights below the registry limit.
    pub fn cscd_simplified(h_m: f64) -> bool { h_m < ndp("ndp.en1991.1-4.cscd-simple-height") }
    /// 🌪️ Reynolds number Re = b v/ν at the velocity v (EN 1991-1-4, E.1.3.1) with ν from the registry.
    pub fn reynolds(b_m: f64, v_m_s: f64) -> f64 { b_m * v_m_s / ndp("ndp.en1991.1-4.kinematic-viscosity") }
    /// 🌪️ Scruton number Sc = 2 δ_s m_e/(ρ b²) (EN 1991-1-4, E.1.3.3).
    pub fn scruton(delta_s: f64, m_e_kg_m: f64, rho: f64, b_m: f64) -> f64 { 2.0 * delta_s * m_e_kg_m / (rho * b_m * b_m) }
    /// 🌪️ Velocity 1.25 v_m below which the critical velocity makes vortex shedding relevant (E.1.3.1, registry factor).
    pub fn vortex_onset_velocity(v_m_s: f64) -> f64 { ndp("ndp.en1991.1-4.vortex-onset") * v_m_s }
}

pub mod part_1_2 {
    use crate::*;
    use semio_s_artifact_norm_en1991_model::part_1_2::FireCurve;
    use semio_s_artifact_norm_en1991_model::FireLoadBasis;

    /// 🔥 Thermal-radiation constants of Eq. (3.2): configuration factor, member and flame emissivity, Stefan–Boltzmann constant.
    pub fn configuration_factor() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-2.configuration-factor")
    }
    pub fn member_emissivity() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-2.member-emissivity")
    }
    pub fn flame_emissivity() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-2.flame-emissivity")
    }
    pub const STEFAN_BOLTZMANN: f64 = 5.67e-8;
    /// 🌡️ Ambient temperature of the unexposed side θ₀ = 20 °C in K.
    pub const AMBIENT_K: f64 = 293.15;
    /// 🔥 Combustion factor m of Annex E.
    pub fn combustion_factor_m() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-2.combustion-factor")
    }

    /// 🌡️ ISO 834 standard curve, Eq. (3.4): θ_g = 20 + 345·log₁₀(8t + 1) (°C, t in min).
    pub fn standard_gas_temp_c(t_min: f64) -> f64 { 20.0 + 345.0 * (8.0 * t_min.max(0.0) + 1.0).log10() }
    /// 🌡️ External fire curve, Eq. (3.5).
    pub fn external_gas_temp_c(t_min: f64) -> f64 { let t = t_min.max(0.0); 660.0 * (1.0 - 0.687 * (-0.32 * t).exp() - 0.313 * (-3.8 * t).exp()) + 20.0 }
    /// 🌡️ Hydrocarbon curve, Eq. (3.6).
    pub fn hydrocarbon_gas_temp_c(t_min: f64) -> f64 { let t = t_min.max(0.0); 1080.0 * (1.0 - 0.325 * (-0.167 * t).exp() - 0.675 * (-2.5 * t).exp()) + 20.0 }

    /// 🌡️ Nominal curve gas temperature (K) at t seconds, `None` for the parametric curve which needs compartment data.
    pub fn nominal_gas_temp_k(curve: FireCurve, t_s: f64) -> Option<f64> {
        let t_min = t_s / 60.0;
        let c = match curve {
            FireCurve::Standard => standard_gas_temp_c(t_min),
            FireCurve::External => external_gas_temp_c(t_min),
            FireCurve::Hydrocarbon => hydrocarbon_gas_temp_c(t_min),
            FireCurve::Parametric => return None,
        };
        Some(c + 273.15)
    }

    /// 🚪 Result of the Annex A parametric fire: opening factor O, Γ used in heating, t_max, θ_max, vent-controlled Γ and t*_max for cooling, the cooling shift x.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ParametricFire { pub o: f64, pub gamma_heating: f64, pub gamma_vent: f64, pub q_td_mj: f64, pub t_max_h: f64, pub t_lim_h: f64, pub fuel_controlled: bool, pub theta_max_c: f64, pub t_star_max_cooling: f64, pub x: f64 }

    fn heating_c(t_star: f64) -> f64 { 20.0 + 1325.0 * (1.0 - 0.324 * (-0.2 * t_star).exp() - 0.204 * (-1.7 * t_star).exp() - 0.472 * (-19.0 * t_star).exp()) }

    /// 🚪 Annex A parameters from A_f, A_t, A_v, h_eq (m², m), thermal absorptivity b (J/m²s^½K), q_f,d (MJ/m²) and t_lim (h).
    pub fn parametric_fire(af_m2: f64, at_m2: f64, av_m2: f64, heq_m: f64, b: f64, qfd_mj_m2: f64, t_lim_h: f64) -> ParametricFire {
        let o = (av_m2 * heq_m.sqrt() / at_m2).max(1e-9);
        let gamma = |o: f64| ((o / b) / (0.04 / 1160.0)).powi(2);
        let gamma_vent = gamma(o);
        let q_td = qfd_mj_m2 * af_m2 / at_m2;
        let t_max_vent = 0.2e-3 * q_td / o;
        let fuel = t_max_vent < t_lim_h;
        let (gamma_heating, t_max) = if fuel {
            let o_lim = 0.1e-3 * q_td / t_lim_h;
            let mut g = gamma(o_lim);
            if o > 0.04 && q_td < 75.0 && b < 1160.0 { g *= 1.0 + ((o - 0.04) / 0.04) * ((q_td - 75.0) / 75.0) * ((1160.0 - b) / 1160.0); }
            (g, t_lim_h)
        } else { (gamma_vent, t_max_vent) };
        let t_star_c = t_max_vent * gamma_vent;
        ParametricFire { o, gamma_heating, gamma_vent, q_td_mj: q_td, t_max_h: t_max, t_lim_h, fuel_controlled: fuel, theta_max_c: heating_c(t_max * gamma_heating), t_star_max_cooling: t_star_c, x: if fuel { t_lim_h * gamma_vent / t_star_c } else { 1.0 } }
    }

    /// 🚪 Parametric gas temperature θ_g (°C) at t hours: heating up to t_max, then the cooling branches of A.11.
    pub fn parametric_theta_c(p: &ParametricFire, t_h: f64) -> f64 {
        let t = t_h.max(0.0);
        if t <= p.t_max_h { return heating_c(t * p.gamma_heating); }
        let ts = p.t_star_max_cooling;
        let k = if ts <= 0.5 { 625.0 } else if ts < 2.0 { 250.0 * (3.0 - ts) } else { 250.0 };
        (p.theta_max_c - k * (t * p.gamma_vent - p.x * ts)).max(20.0)
    }

    /// 🚪 Opening factor and areas (A_t, A_v, h_eq) of a square compartment with 20 % wall openings and h_eq = H.
    pub fn compartment_geometry(af_m2: f64, height_m: f64) -> (f64, f64, f64) {
        let af = af_m2.max(1.0);
        let h = height_m.max(0.5);
        let wall = 4.0 * af.sqrt() * h;
        (2.0 * af + wall, 0.2 * wall, h)
    }
    /// 🚪 Opening factor O of the square compartment model (m^½).
    pub fn compartment_opening_factor(af_m2: f64, height_m: f64) -> f64 { let (at, av, h) = compartment_geometry(af_m2, height_m); av * h.sqrt() / at }
    /// 🔥 Design fire load related to the total surface q_t,d = q_f,d·A_f/A_t (J/m²).
    pub fn design_fire_load_qt_d_j_m2(qf_d_j_m2: f64, af_m2: f64, height_m: f64) -> f64 { qf_d_j_m2 * af_m2.max(1.0) / compartment_geometry(af_m2, height_m).0 }
    /// 🚪 Fire compartment time-temperature curve (K) of the square model for a given duration in seconds and q_f,d in J/m².
    pub fn parametric_gas_temp_k(t_s: f64, af_m2: f64, height_m: f64, thermal_inertia_b: f64, qf_d_j_m2: f64, opening_factor: Option<f64>, t_lim_h: f64) -> (f64, ParametricFire) {
        let (at, av, h) = compartment_geometry(af_m2, height_m);
        let av_eff = match opening_factor { Some(o) if o > 0.0 => o * at / h.sqrt(), _ => av };
        let p = parametric_fire(af_m2.max(1.0), at, av_eff, h, thermal_inertia_b, qf_d_j_m2 / 1e6, t_lim_h);
        (parametric_theta_c(&p, t_s / 3600.0) + 273.15, p)
    }

    /// 🔥 Net heat flux ĥ_net = α_c(θ_g−θ_m) + Φ ε_m ε_f σ(θ_g⁴−θ_m⁴) (W/m², K) — Eq. (3.1)/(3.2).
    pub fn h_net_w_m2(theta_g_k: f64, theta_m_k: f64, alpha_c: f64, phi: f64, epsilon_m: f64, epsilon_f: f64) -> f64 {
        alpha_c * (theta_g_k - theta_m_k) + phi * epsilon_m * epsilon_f * STEFAN_BOLTZMANN * (theta_g_k.powi(4) - theta_m_k.powi(4))
    }
    /// 🔥 Net heat flux with the default radiation constants against ambient temperature.
    pub fn h_net_default_w_m2(theta_g_k: f64, alpha_c: f64) -> f64 { h_net_w_m2(theta_g_k, AMBIENT_K, alpha_c, configuration_factor(), member_emissivity(), flame_emissivity()) }

    /// 📏 δ_q1 of Annex E by compartment floor area, linear in log A between 25 m² (1.10) and 10000 m² (2.13).
    pub fn delta_q1(area_m2: f64) -> f64 {
        let rows = [(25.0_f64, 1.10), (250.0, 1.50), (2500.0, 1.90), (5000.0, 2.00), (10000.0, 2.13)];
        let logs: Vec<(f64, f64)> = rows.iter().map(|(a, d)| (a.ln(), *d)).collect();
        numeric::table(&logs, area_m2.max(1.0).ln())
    }
    /// 🗂️ Registry key of an occupancy in the fire load table (Table E.4 rows), `None` for an unknown occupancy.
    pub fn occupancy_key(occupancy: &str) -> Option<&'static str> {
        match occupancy.trim().to_ascii_lowercase().as_str() {
            "dwelling" | "residential" => Some("dwelling"),
            "office" => Some("office"),
            "hospital" => Some("hospital"),
            "school" => Some("school"),
            "hotel" => Some("hotel"),
            "library" => Some("library"),
            "shopping" | "mall" => Some("shopping"),
            "theatre" => Some("theatre"),
            "transport" => Some("transport"),
            _ => None,
        }
    }
    /// 🗂️ Registry key of a Table E.4 occupancy row of the document.
    pub fn occupancy_name(occupancy: semio_s_artifact_norm_en1991_model::FireOccupancy) -> &'static str {
        use semio_s_artifact_norm_en1991_model::FireOccupancy as O;
        match occupancy {
            O::Dwelling => "dwelling",
            O::Office => "office",
            O::Hospital => "hospital",
            O::School => "school",
            O::Hotel => "hotel",
            O::Library => "library",
            O::Shopping => "shopping",
            O::Theatre => "theatre",
            O::Transport => "transport",
        }
    }
    /// 🧯 Characteristic fire load density q_f,k (J/m²) of a registry occupancy key (aliases accepted), the 80 % fractile of Table E.4; `None` for an unknown key.
    pub fn characteristic_fire_load_j_m2(annex: AnnexChoice, occupancy: &str) -> Option<f64> {
        occupancy_row(annex, occupancy).map(|row| row.0 * 1.0e6)
    }
    /// 🔖 Registry ids the design fire load rests on: the combustion factor m always, the occupancy row of Table E.4 only for the occupancy-table basis, so an unverified table never reports a compliant verdict.
    pub fn fire_load_ndp_ids(basis: FireLoadBasis, occupancy: semio_s_artifact_norm_en1991_model::FireOccupancy) -> Vec<String> {
        let mut ids = vec!["ndp.en1991.1-2.combustion-factor".to_owned()];
        if basis == FireLoadBasis::OccupancyTable {
            ids.push(format!("ndp.en1991.1-2.fire-load-fractile.{}", occupancy_name(occupancy)));
        }
        ids
    }
    /// ⚠️ 80 % fractile q_f,k (MJ/m²) of Table E.4 from the registry and δ_q2 by occupancy, `None` for an unknown occupancy.
    pub fn occupancy_row(annex: AnnexChoice, occupancy: &str) -> Option<(f64, f64)> {
        let fractile = semio_s_plugin_norm_compliance::ndp_cell_opt("ndp.en1991.1-2.fire-load-fractile", &[occupancy_key(occupancy)?], annex)?;
        Some((fractile, 1.0))
    }
    /// 🏷️ Admissible occupancies of the fire load tables.
    pub const OCCUPANCIES: [&str; 11] = ["dwelling", "residential", "office", "hospital", "school", "hotel", "library", "shopping", "mall", "theatre", "transport"];
    /// 🧯 Characteristic fire load density q_f,k (J/m²) of the chosen basis: the Table E.4 80 % fractile of the occupancy, or the user-specified value (which must be positive); `None` when the basis is not satisfiable.
    pub fn characteristic_fire_load_for_basis_j_m2(basis: FireLoadBasis, user_qf_k_j_m2: f64, annex: AnnexChoice, occupancy: semio_s_artifact_norm_en1991_model::FireOccupancy) -> Option<f64> {
        match basis {
            FireLoadBasis::OccupancyTable => characteristic_fire_load_j_m2(annex, occupancy_name(occupancy)),
            FireLoadBasis::UserSpecified => (user_qf_k_j_m2.is_finite() && user_qf_k_j_m2 > 0.0).then_some(user_qf_k_j_m2),
        }
    }
    /// 🏭 Design fire load q_f,d = q_f,k·m·δ_q1·δ_q2·δ_n (J/m²), Eq. (E.1), with q_f,k of exactly the chosen basis (no maximum of the two); `None` when the basis is not satisfiable (unknown occupancy class for the table, non-positive user value).
    pub fn design_fire_load_j_m2(basis: FireLoadBasis, user_qf_k_j_m2: f64, annex: AnnexChoice, occupancy: semio_s_artifact_norm_en1991_model::FireOccupancy, area_m2: f64, delta_n: f64) -> Option<f64> {
        let qf_k = characteristic_fire_load_for_basis_j_m2(basis, user_qf_k_j_m2, annex, occupancy)?;
        let dq2 = occupancy_row(annex, occupancy_name(occupancy)).map_or(1.0, |row| row.1);
        Some(qf_k * combustion_factor_m() * delta_q1(area_m2) * dq2 * delta_n)
    }
    /// 📜 The source of q_f,k as text for explanations: the Table E.4 row with its value, or the user value.
    pub fn fire_load_source(basis: FireLoadBasis, user_qf_k_j_m2: f64, annex: AnnexChoice, occupancy: semio_s_artifact_norm_en1991_model::FireOccupancy) -> Option<(String, String)> {
        let qf_k = characteristic_fire_load_for_basis_j_m2(basis, user_qf_k_j_m2, annex, occupancy)? / 1.0e6;
        Some(match basis {
            FireLoadBasis::OccupancyTable => (format!("Table E.4 row \"{}\", 80 % fractile q_f,k = {qf_k:.0} MJ/m²", occupancy_name(occupancy)), format!("Tabelle E.4 Zeile \"{}\", 80-%-Fraktile q_f,k = {qf_k:.0} MJ/m²", occupancy_name(occupancy))),
            FireLoadBasis::UserSpecified => (format!("user-specified q_f,k = {qf_k:.0} MJ/m²"), format!("vorgegebenes q_f,k = {qf_k:.0} MJ/m²")),
        })
    }
    /// ⏱️ Equivalent fire duration t_e,d = q_f,d·k_b·w_f (min, q_f,d in MJ/m²) — Annex F.
    pub fn equivalent_duration_min(qfd_mj_m2: f64, k_b: f64, height_m: f64, alpha_v: f64, alpha_h: f64) -> f64 {
        let b_v = (12.5 * (1.0 + 10.0 * alpha_v - alpha_v * alpha_v)).max(10.0);
        let w_f = ((6.0 / height_m).powf(0.3) * (0.62 + 90.0 * (0.4 - alpha_v).powi(4) / (1.0 + b_v * alpha_h))).max(0.5);
        qfd_mj_m2 * k_b * w_f
    }
    /// 🔥 Convection coefficient α_c of the curve: 50 W/m²K hydrocarbon, 25 otherwise.
    pub fn alpha_c_for_curve(curve: FireCurve) -> f64 { match curve { FireCurve::Hydrocarbon => 50.0, FireCurve::Parametric => 35.0, _ => 25.0 } }
    /// 🪔 Localised fire (Annex C): flame length L_f, virtual origin z₀ (m) and axis temperature θ(z) (°C, ≤ 900) for diameter D (m) and heat release Q (W).
    pub fn local_fire(d_m: f64, q_w: f64, z_m: f64) -> (f64, f64, f64) {
        let lf = -1.02 * d_m + 0.0148 * q_w.powf(0.4);
        let z0 = -1.02 * d_m + 0.00524 * q_w.powf(0.4);
        (lf, z0, (20.0 + 0.25 * (0.8 * q_w).powf(2.0 / 3.0) * (z_m - z0).powf(-5.0 / 3.0)).min(900.0))
    }
    /// 🔥 Dimensionless heat release rate Q* = Q/(1.11e6 L^2.5) for the length L: the fire diameter D gives Q_D*, the ceiling height H gives Q_H* (registry denominator, EN 1991-1-2, Annex C, unverified).
    pub fn local_fire_q_star(q_w: f64, length_m: f64) -> f64 { q_w / (ndp("ndp.en1991.1-2.local-q-denominator") * length_m.powf(2.5)) }
    /// 🔥 Horizontal flame length L_h = H (2.9 Q_H*^0.33 − 1) (m) of a fire whose flame reaches the ceiling at the height H (registry coefficient and exponent, EN 1991-1-2, Annex C, unverified).
    pub fn local_fire_horizontal_flame_m(q_w: f64, ceiling_m: f64) -> f64 { ceiling_m * (ndp("ndp.en1991.1-2.local-lh-coefficient") * local_fire_q_star(q_w, ceiling_m).powf(ndp("ndp.en1991.1-2.local-lh-exponent")) - 1.0) }
    /// 🔥 Height z' (m) of the virtual heat source: 2.4 D (Q_D*^(2/5) − Q_D*^(2/3)) below Q_D* = 1 and 2.4 D (1 − Q_D*^(2/5)) above it (registry factor, EN 1991-1-2, Annex C, unverified).
    pub fn local_fire_virtual_source_m(q_w: f64, d_m: f64) -> f64 {
        let q = local_fire_q_star(q_w, d_m);
        ndp("ndp.en1991.1-2.local-zprime-factor") * d_m * if q < 1.0 { q.powf(0.4) - q.powf(2.0 / 3.0) } else { 1.0 - q.powf(0.4) }
    }
    /// 🔥 Hasemi parameter y = (r + H + z')/(L_h + H + z') of the position of the member below the ceiling.
    pub fn hasemi_y(r_m: f64, ceiling_m: f64, z_prime_m: f64, l_h_m: f64) -> f64 { (r_m + ceiling_m + z_prime_m) / (l_h_m + ceiling_m + z_prime_m) }
    /// 🔥 Hasemi heat flux ĥ (W/m²) at the parameter y: plateau for y ≤ 0.30, linear to y = 1.0, then the power tail (registry rows, EN 1991-1-2, Annex C, unverified).
    pub fn hasemi_flux_w_m2(y: f64) -> f64 {
        if y <= 0.30 { ndp("ndp.en1991.1-2.local-flux-plateau") } else if y < 1.0 { ndp("ndp.en1991.1-2.local-flux-intercept") - ndp("ndp.en1991.1-2.local-flux-slope") * y } else { ndp("ndp.en1991.1-2.local-flux-tail") * y.powf(ndp("ndp.en1991.1-2.local-flux-tail-exponent")) }
    }
    /// 📐 Configuration factor Φ of a rectangle a = h/c, b = w/c facing a receiver at distance c (Annex G).
    pub fn rectangle_configuration_factor(a: f64, b: f64) -> f64 {
        (a / (1.0 + a * a).sqrt() * (b / (1.0 + a * a).sqrt()).atan() + b / (1.0 + b * b).sqrt() * (a / (1.0 + b * b).sqrt()).atan()) / (2.0 * std::f64::consts::PI)
    }
    /// 🏗️ Fire-to-ULS reduction η_fi = (G + ψ Q)/(γ_G G + γ_Q Q).
    pub fn eta_fi(g: f64, q: f64, psi: f64, gamma_g: f64, gamma_q: f64) -> f64 { (g + psi * q) / (gamma_g * g + gamma_q * q) }
}

pub mod part_1_5 {
    use crate::*;
    /// 🌉 Deck types 1 steel, 2 composite, 3 concrete with the shade-to-uniform temperature shifts (ΔT_min, ΔT_max) of Fig. 6.1.
    pub fn bridge_shift(bridge_type: u8) -> Option<(f64, f64)> { match bridge_type { 1 => Some((-3.0, 16.0)), 2 => Some((4.0, 4.0)), 3 => Some((8.0, 2.0)), _ => None } }
    /// 🌉 Contraction and expansion ranges (ΔT_N,con, ΔT_N,exp) from T_min, T_max, T_0 and the deck type.
    pub fn bridge_delta_t_n(bridge_type: u8, t_min: f64, t_max: f64, t_0: f64) -> Option<(f64, f64)> {
        let (dn, dx) = bridge_shift(bridge_type)?;
        Some((t_0 - (t_min + dn), t_max + dx - t_0))
    }
    /// 🏢 Building ranges (T_0 − T_min, T_max − T_0).
    pub fn building_delta_t(t_min: f64, t_max: f64, t_0: f64) -> (f64, f64) { (t_0 - t_min, t_max - t_0) }
    /// 🏷️ Admissible thermal element types.
    pub const ELEMENT_TYPES: [&str; 2] = ["building", "bridge"];
    /// 🌡️ Required ΔT (K): the governing combination of the uniform range and the linear gradient with ω_N = 0.35 and ω_M = 0.75; `None` for a bridge type outside 1–3 or an unknown element type.
    pub fn required_delta_t(element_type: &str, t_max_c: f64, t_min_c: f64, t_0_c: f64, bridge_type: u8, delta_t_m: f64) -> Option<f64> {
        let (con, exp) = match element_type { "bridge" => bridge_delta_t_n(bridge_type, t_min_c, t_max_c, t_0_c)?, "building" => building_delta_t(t_min_c, t_max_c, t_0_c), _ => return None };
        let n = con.max(exp);
        let m = delta_t_m.abs();
        Some(combined_delta_t(n, m))
    }
    /// 🌉 Deck types 1 steel, 2 composite, 3 concrete box, 4 concrete slab and 5 concrete beam mapped to the uniform-temperature class 1 to 3 of Fig. 6.1.
    pub fn uniform_class(deck_type: u8) -> Option<u8> { match deck_type { 1 | 2 => Some(deck_type), 3..=5 => Some(3), _ => None } }
    /// 🌉 Linear vertical temperature differences (ΔT_M,heat, ΔT_M,cool) in K: Table 6.1 base values times k_sur (steel decks only, other decks stay unmodified); `None` for an unknown deck type or surfacing.
    pub fn bridge_delta_t_m(deck_type: u8, surfacing: u8) -> Option<(f64, f64)> {
        if !(1..=5).contains(&deck_type) || surfacing > 4 { return None; }
        let (kh, kc) = if deck_type == 1 { (ndp(&format!("ndp.en1991.1-5.ksur-heat.{surfacing}")), ndp(&format!("ndp.en1991.1-5.ksur-cool.{surfacing}"))) } else { (ndp("ndp.en1991.1-5.ksur-other"), ndp("ndp.en1991.1-5.ksur-other")) };
        Some((kh * ndp(&format!("ndp.en1991.1-5.dtm-heat.{deck_type}")), kc * ndp(&format!("ndp.en1991.1-5.dtm-cool.{deck_type}"))))
    }
    /// 🌡️ Combination factors (ω_N, ω_M) of the uniform range and the linear difference.
    pub fn combination_factors() -> (f64, f64) { (ndp("ndp.en1991.1-5.omega-n"), ndp("ndp.en1991.1-5.omega-m")) }
    /// 🌡️ Governing combination max(ΔT_N + ω_M ΔT_M, ω_N ΔT_N + ΔT_M) in K.
    pub fn combined_delta_t(delta_t_n: f64, delta_t_m: f64) -> f64 { let (wn, wm) = combination_factors(); (delta_t_n + wm * delta_t_m).max(wn * delta_t_n + delta_t_m) }
    /// 📐 Curvature κ = α ΔT_M/h (1/m) of the linear temperature difference.
    pub fn curvature_per_m(alpha_per_k: f64, delta_t_m_k: f64, depth_m: f64) -> f64 { alpha_per_k * delta_t_m_k / depth_m.max(1e-9) }
    /// 📐 End rotation θ = α ΔT_M L/(2h) (rad) of a simply supported span.
    pub fn end_rotation_rad(alpha_per_k: f64, delta_t_m_k: f64, length_m: f64, depth_m: f64) -> f64 { curvature_per_m(alpha_per_k, delta_t_m_k, depth_m) * length_m / 2.0 }
    /// 📏 Thermal expansion ΔL = α ΔT L (m).
    pub fn expansion_m(alpha_per_k: f64, delta_t_k: f64, length_m: f64) -> f64 { alpha_per_k * delta_t_k * length_m }
    /// 🪢 Restraint stress σ = E α ΔT (Pa).
    pub fn restraint_stress_pa(e_pa: f64, alpha_per_k: f64, delta_t_k: f64) -> f64 { e_pa * alpha_per_k * delta_t_k }
}

pub mod part_1_6 {
    use crate::*;
    /// 🏗️ Construction imposed load (Pa) by activity, `None` for an unknown activity.
    pub fn construction_qk_pa(activity: &str) -> Option<f64> {
        let kn = match activity { "storage" => 0.2, "machinery" | "concreting" => 3.0, "scaffolding" | "formwork" | "personnel" => 1.0, _ => return None };
        Some(kn_m2_to_pa(kn))
    }
    /// 🏷️ Admissible construction activities.
    pub const ACTIVITIES: [&str; 6] = ["storage", "machinery", "concreting", "scaffolding", "formwork", "personnel"];
    /// 📆 Return-period factor c_prob of the wind velocity for the annual probability p — EN 1991-1-6 / EN 1991-1-4 (4.2).
    pub fn c_prob(p: f64) -> f64 { ((1.0 - 0.2 * (-(1.0 - p).ln()).ln()) / (1.0 - 0.2 * (-(0.98_f64).ln()).ln())).powf(0.5) }
    /// 🗓️ Registry row of the duration class of a construction stage in EN 1991-1-6 Table 3.1: up to 3 days, up to 3 months (90 days), up to 1 year (365 days), longer.
    pub fn stage_return_period_ndp_id(duration_days: f64) -> &'static str {
        if duration_days <= 3.0 { "ndp.en1991.1-6.stage-return.days3" } else if duration_days <= 90.0 { "ndp.en1991.1-6.stage-return.months3" } else if duration_days <= 365.0 { "ndp.en1991.1-6.stage-return.year1" } else { "ndp.en1991.1-6.stage-return.longer" }
    }
    /// 🗓️ Return period (years) of the climatic actions of a construction stage by its duration — EN 1991-1-6 Table 3.1 (2, 5, 10, 50 years recommended); `None` while the annex holds no value (the modified DE NA table is not keyed).
    pub fn stage_return_period_years(annex: AnnexChoice, duration_days: f64) -> Option<f64> {
        semio_s_plugin_norm_compliance::ndp_number_opt(stage_return_period_ndp_id(duration_days), annex)
    }
    /// 🌬️ Ratio q_p(T)/q_p(50 a) of the peak velocity pressure for the return period T: `c_prob(1/T)²` (EN 1991-1-4 Eq. 4.2).
    pub fn wind_pressure_ratio(return_years: f64) -> f64 { c_prob(1.0 / return_years.max(1.01)).powi(2) }
    /// 🧱 Hydrostatic fresh-concrete pressure γ h (Pa).
    pub fn fresh_concrete_pressure_pa(gamma_n_m3: f64, h_m: f64) -> f64 { gamma_n_m3 * h_m }
    /// 🪨 Mean arrest force (N) of a mass m falling h and stopped over s: m g (h + s) / s (energy balance).
    pub fn falling_arrest_force_n(mass_kg: f64, drop_m: f64, stop_m: f64) -> f64 { mass_kg * 9.80665 * (drop_m + stop_m) / stop_m }
    /// 📐 Equivalent horizontal force (N) of an execution tolerance Δ on a member of height h under N: N Δ / h.
    pub fn imperfection_force_n(vertical_n: f64, tolerance_m: f64, height_m: f64) -> f64 { vertical_n * tolerance_m / height_m }
    /// 🧱 Combined construction load (Pa) of a stage: ψ₀ Σ Q_c,i over its loads (Pa).
    pub fn combined_construction_load_pa(magnitudes_pa: impl IntoIterator<Item = f64>) -> f64 { crate::ndp("ndp.en1991.1-6.combination-psi0") * magnitudes_pa.into_iter().sum::<f64>() }
}

pub mod part_1_7 {
    use crate::ndp;

    /// 🚗 Equivalent vehicle stiffness k = 300 kN/m of Annex C.
    pub fn vehicle_stiffness_n_m() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-7.vehicle-stiffness")
    }
    /// 🚗 Dynamic impact force F = v √(k m) (N) for an explicit equivalent stiffness k (N/m) — EN 1991-1-7 Annex C.
    pub fn dynamic_impact_force_n(mass_kg: f64, speed_m_s: f64, stiffness_n_m: f64) -> f64 { speed_m_s * (stiffness_n_m * mass_kg).sqrt() }
    /// 🚗 Dynamic impact force F = v √(k m) (N) with the registry vehicle stiffness — EN 1991-1-7 Annex C.
    pub fn vehicle_impact_force_n(mass_kg: f64, speed_m_s: f64) -> f64 { dynamic_impact_force_n(mass_kg, speed_m_s, vehicle_stiffness_n_m()) }
    /// ⚡ Kinetic impact energy ½mv² (J).
    pub fn impact_energy_j(mass_kg: f64, speed_m_s: f64) -> f64 { 0.5 * mass_kg * speed_m_s * speed_m_s }
    /// 💥 Incident blast overpressure (Pa) of a TNT-equivalent charge W (kg) at stand-off R (m): Mills' fit `1772/Z³ − 114/Z² + 108/Z` kPa with Z = R/W^⅓.
    pub fn explosion_pressure_pa(mass_kg: f64, distance_m: f64) -> f64 {
        if distance_m < f64::EPSILON || mass_kg <= 0.0 { return 0.0; }
        let z = distance_m / mass_kg.cbrt();
        (1772.0 / z.powi(3) - 114.0 / z.powi(2) + 108.0 / z).max(0.0) * 1000.0
    }
    /// 🏠 Internal gas explosion design pressure p_d (kN/m²) per Annex D.2: 3 + p_v/2 + 0.04/(A_v/V)².
    pub fn gas_explosion_pd_kn_m2(p_v: f64, av_over_v: f64) -> f64 { ndp("ndp.en1991.1-7.gas-base") + p_v / 2.0 + ndp("ndp.en1991.1-7.gas-vent-coefficient") / (av_over_v * av_over_v) }
    /// 🏠 Internal gas explosion design pressure p_d (kN/m²) per Annex D.1: 3 + p_v.
    pub fn gas_explosion_pd_static_kn_m2(p_v: f64) -> f64 { ndp("ndp.en1991.1-7.gas-base") + p_v }
    /// 🏠 Governing design pressure (kN/m²): the larger of D.1 and D.2 (selection rule unverified).
    pub fn gas_explosion_governing_kn_m2(p_v: f64, av_over_v: f64) -> f64 { gas_explosion_pd_static_kn_m2(p_v).max(gas_explosion_pd_kn_m2(p_v, av_over_v)) }
    /// 🏠 Whether the enclosure volume lies in the validity range of Annex D (registry bounds, unverified).
    pub fn gas_explosion_in_domain(volume_m3: f64) -> bool { (ndp("ndp.en1991.1-7.gas-volume-min")..=ndp("ndp.en1991.1-7.gas-volume-max")).contains(&volume_m3) }
    /// 🚗 Equivalent stiffness k (N/m): the given value or, when zero, the registry stiffness of road vehicles.
    pub fn impact_stiffness_n_m(given_n_m: f64) -> f64 { if given_n_m > 0.0 { given_n_m } else { vehicle_stiffness_n_m() } }
    /// 🚜 Equivalent static forklift impact force F = factor·m·g (N), registry factor (unverified).
    pub fn forklift_force_n(mass_kg: f64) -> f64 { ndp("ndp.en1991.1-7.forklift-factor") * mass_kg * 9.80665 }
    /// ↔️ Transverse impact force F_dy (N) as the registry share of F_dx (unverified).
    pub fn lateral_impact_force_n(force_x_n: f64) -> f64 { ndp("ndp.en1991.1-7.lateral-ratio") * force_x_n }
    /// 📏 Admissible application height range (m) of impacts on columns and walls (registry, unverified).
    pub fn impact_height_range_m() -> (f64, f64) { (ndp("ndp.en1991.1-7.impact-height-min"), ndp("ndp.en1991.1-7.impact-height-max")) }
    /// 🪢 Horizontal tie force T = factor·(g + ψ q)·s·L (kN), capped at 75 kN.
    pub fn tie_force_kn(factor: f64, g: f64, q: f64, psi: f64, s_m: f64, l_m: f64) -> (f64, f64) { let t = factor * (g + psi * q) * s_m * l_m; (t, t.min(75.0)) }
}

pub mod part_2 {
    use crate::*;

    /// 🌉 National axle factor α_Q of LM1 of lane 1 to 3, registry rows `ndp.en1991.2.alpha-q.<lane>` (interim value 1.0, unverified).
    pub fn alpha_q(annex: AnnexChoice, lane: u8) -> f64 { semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1991.2.alpha-q.{}", lane.clamp(1, 3)), annex) }
    /// 🌉 National UDL factor α_q of LM1 of lane 1 to 3 and of the remaining area (4), registry rows `ndp.en1991.2.alpha-q-udl.<lane>` (interim value 1.0, unverified).
    pub fn alpha_q_udl(annex: AnnexChoice, lane: u8) -> f64 { semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1991.2.alpha-q-udl.{}", lane.clamp(1, 4)), annex) }
    /// 🌉 LM1 tandem axle group Q_ak (N) of the lane.
    pub fn lm1_tandem_n(annex: AnnexChoice, lane: u8) -> f64 {
        let base_kn = match lane { 1 => 300.0, 2 => 200.0, 3 => 100.0, _ => 0.0 };
        kn_to_n(alpha_q(annex, lane) * base_kn)
    }
    /// 🌉 LM1 UDL q_ak (Pa) of the lane: 9 kN/m² on lane 1, 2.5 kN/m² elsewhere.
    pub fn lm1_udl_pa(annex: AnnexChoice, lane: u8) -> f64 { alpha_q_udl(annex, lane) * if lane == 1 { 9.0 } else { 2.5 } * 1000.0 }
    /// 🌉 LM2 single axle β_Q·400 kN (N).
    pub fn lm2_axle_n(annex: AnnexChoice) -> f64 { kn_to_n(alpha_q(annex, 1) * 400.0) }
    /// 🚶 Footway and cycle-track characteristic load (Pa): 5 kN/m².
    pub fn footway_pa(_annex: AnnexChoice) -> f64 { 5000.0 }
    /// 🚶 Reduced footway load of load group gr1a (Pa): 3 kN/m².
    pub fn footway_reduced_pa() -> f64 { 3000.0 }
    /// 🌉 Axle spacing of the LM1 tandem system (m), EN 1991-2 Fig. 4.2b.
    pub fn lm1_axle_spacing_m() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.lm1-axle-spacing")
    }
    /// 🌉 Maximum mid-span-governing bending moment (N·m) of two equal axles `axle_n` at [`lm1_axle_spacing_m`] on a simply supported span: P (L − s/2)² / (2L), or P·L/4 when only one axle fits.
    pub fn tandem_moment_nm(axle_n: f64, span_m: f64) -> f64 {
        if span_m <= 0.0 { return 0.0; }
        let single = axle_n * span_m / 4.0;
        if span_m < lm1_axle_spacing_m() { return single; }
        single.max(axle_n * (span_m - lm1_axle_spacing_m() / 2.0).powi(2) / (2.0 * span_m))
    }
    /// 🌉 LM1 tandem design moment (N·m): both axles α_Q·Q_k at 1.2 m in the worst position of a simply supported span (matches PyNite and anastruct).
    pub fn lm1_tandem_moment_nm(annex: AnnexChoice, lane: u8, span_m: f64) -> f64 { tandem_moment_nm(lm1_tandem_n(annex, lane), span_m) }
    /// 🏛️ Maximum support reaction (N) of a simply supported span under two axles `axle_n` at the LM1 spacing: the first axle on the support, the second at `L - s` (one axle only when L < s).
    pub fn tandem_reaction_n(axle_n: f64, span_m: f64) -> f64 {
        if span_m <= 0.0 { return 0.0; }
        if span_m < lm1_axle_spacing_m() { return axle_n; }
        axle_n * (2.0 - lm1_axle_spacing_m() / span_m)
    }
    /// 🚆 Mid-span moment (N·m) of a simply supported span under an axle group of `axles` loads `axle_n` at `spacing_m` centred at mid-span plus `udl_n_m` outside the group length `(axles - 1) spacing / 2 + lead_m` on both sides (influence ordinate min(x, L - x) / 2).
    pub fn rail_lm71_mid_moment_nm(axle_n: f64, udl_n_m: f64, axles: u32, spacing_m: f64, lead_m: f64, span_m: f64) -> f64 {
        if span_m <= 0.0 { return 0.0; }
        let eta = |x: f64| x.min(span_m - x) / 2.0;
        let centre = (f64::from(axles) - 1.0) / 2.0;
        let axle_sum: f64 = (0..axles).map(|k| eta(span_m / 2.0 + (f64::from(k) - centre) * spacing_m)).sum();
        let free_m = (span_m / 2.0 - (centre * spacing_m + lead_m)).max(0.0);
        axle_n * axle_sum + udl_n_m * free_m * free_m / 2.0
    }
    /// 📐 Influence ordinate of the mid-span moment of a simply supported span for a unit load at `x_m` (zero outside the span).
    pub fn eta_moment(x_m: f64, span_m: f64) -> f64 { if (0.0..=span_m).contains(&x_m) { x_m.min(span_m - x_m) / 2.0 } else { 0.0 } }
    /// 📐 Influence ordinate of the left support reaction for a unit load at `x_m` (zero outside the span).
    pub fn eta_reaction(x_m: f64, span_m: f64) -> f64 { if (0.0..=span_m).contains(&x_m) { 1.0 - x_m / span_m } else { 0.0 } }
    /// 🏗️ Mid-span moment (N·m) and left reaction (N) of two axles `axle_n` one `spacing_m` apart with the front axle at `front_m` on a simply supported span.
    pub fn tandem_effects(axle_n: f64, front_m: f64, span_m: f64, spacing_m: f64) -> (f64, f64) {
        (axle_n * (eta_moment(front_m, span_m) + eta_moment(front_m - spacing_m, span_m)), axle_n * (eta_reaction(front_m, span_m) + eta_reaction(front_m - spacing_m, span_m)))
    }
    /// 🌈️ Envelope over the travel of the tandem: the largest mid-span moment (N·m) and left reaction (N); both are piecewise linear in the front axle position, so the maximum sits at a breakpoint (third-party oracle `en1991.2.tandem-travel-envelope`).
    pub fn tandem_envelope(axle_n: f64, span_m: f64, spacing_m: f64) -> (f64, f64) {
        [0.0, spacing_m, span_m / 2.0, span_m / 2.0 + spacing_m, span_m, span_m + spacing_m].iter().map(|x| tandem_effects(axle_n, *x, span_m, spacing_m)).fold((0.0, 0.0), |a, e| (f64::max(a.0, e.0), f64::max(a.1, e.1)))
    }
    /// 📐 Influence ordinate of the bending moment at `section_m` of a simply supported span for a unit load at `x_m` (zero outside the span).
    pub fn eta_moment_at(section_m: f64, x_m: f64, span_m: f64) -> f64 {
        if !(0.0..=span_m).contains(&x_m) { 0.0 } else if x_m <= section_m { x_m * (span_m - section_m) / span_m } else { section_m * (span_m - x_m) / span_m }
    }
    /// 📐 Influence ordinate of the shear force at `section_m` (left reaction minus the loads left of the section) for a unit load at `x_m`; `right_limit` takes a load exactly at the section on its right side.
    pub fn eta_shear_at(section_m: f64, x_m: f64, span_m: f64, right_limit: bool) -> f64 {
        if !(0.0..=span_m).contains(&x_m) { 0.0 } else if x_m > section_m || (right_limit && x_m >= section_m) { (span_m - x_m) / span_m } else { -x_m / span_m }
    }
    /// 🌈️ Envelope of the LM1 tandem at one section of a simply supported span: the largest bending moment (N·m), the largest and the smallest shear force (N) of two axles `axle_n` one `spacing_m` apart. Both effects are piecewise linear in the front axle position, so the extremes sit where an axle stands over a support or the section (third-party oracle `mechanics.tandem-section-envelope`).
    pub fn tandem_section_envelope(axle_n: f64, span_m: f64, spacing_m: f64, section_m: f64) -> (f64, f64, f64) {
        let sum = |axles: (f64, f64), effect: &dyn Fn(f64) -> f64| axle_n * (effect(axles.0) + effect(axles.1));
        [0.0, section_m, span_m].iter().flat_map(|stand| [(*stand, stand - spacing_m), (stand + spacing_m, *stand)]).fold((0.0_f64, 0.0_f64, 0.0_f64), |(moment, shear_max, shear_min), axles| {
            let (right, left) = (sum(axles, &|x| eta_shear_at(section_m, x, span_m, true)), sum(axles, &|x| eta_shear_at(section_m, x, span_m, false)));
            (moment.max(sum(axles, &|x| eta_moment_at(section_m, x, span_m))), shear_max.max(right).max(left), shear_min.min(right).min(left))
        })
    }
    /// 🌈️ Envelope of a uniform load `udl_n_m` over the whole span at `section_m`: the bending moment w x (L − x) / 2 and the shear forces w (L − x)² / (2L) (loaded right of the section) and −w x² / (2L) (loaded left of it).
    pub fn udl_section_envelope(udl_n_m: f64, span_m: f64, section_m: f64) -> (f64, f64, f64) {
        (udl_n_m * section_m * (span_m - section_m) / 2.0, udl_n_m * (span_m - section_m).powi(2) / (2.0 * span_m), -udl_n_m * section_m.powi(2) / (2.0 * span_m))
    }
    /// 🚛 LM3 heaviest axle line of the Annex A standard vehicles (N): 200 kN.
    pub fn lm3_axle_n(_annex: AnnexChoice) -> f64 { kn_to_n(200.0) }
    /// 👥 LM4 crowd loading (Pa): 5 kN/m².
    pub fn lm4_crowd_pa(_annex: AnnexChoice) -> f64 { 5000.0 }

    /// 🚛 Designations of the standard special vehicles total/axle line in kN offered as options (Annex A, unverified).
    pub const SPECIAL_VEHICLES: [&str; 8] = ["600/150", "900/150", "1200/150", "1500/150", "1800/150", "2400/200", "3000/200", "3600/200"];
    /// 🚛 Standard special vehicle of the designation total/axle in kN → (total N, axle line N, axle lines): any integer number of 2 to 30 axle lines of 150 or 200 kN is accepted; `None` otherwise.
    pub fn special_vehicle(designation: &str) -> Option<(f64, f64, usize)> {
        let (total_text, axle_text) = designation.trim().split_once('/')?;
        let (total, axle): (f64, f64) = (total_text.trim().parse().ok()?, axle_text.trim().parse().ok()?);
        let lines = total / axle;
        if !(total > 0.0 && (axle == 150.0 || axle == 200.0) && (2.0..=30.0).contains(&lines) && (lines - lines.round()).abs() < 1e-9) { return None; }
        Some((kn_to_n(total), kn_to_n(axle), lines.round() as usize))
    }
    /// 🚛 Spacing of the axle lines (m), registry value (unverified).
    pub fn lm3_axle_spacing_m() -> f64 { ndp("ndp.en1991.2.lm3-axle-spacing") }
    /// 🚛 Width over the wheels (m), registry value (unverified).
    pub fn lm3_vehicle_width_m() -> f64 { ndp("ndp.en1991.2.lm3-vehicle-width") }
    /// 🚛 Length of a vehicle with `lines` axle lines (m).
    pub fn special_vehicle_length_m(lines: usize) -> f64 { (lines.max(1) - 1) as f64 * lm3_axle_spacing_m() }

    /// 📦 Load group constituents of EN 1991-2 Table 4.4a: LM1, LM2, LM3, LM4 and the footway (reduced in gr1a).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct GroupMembers { pub lm1: bool, pub lm2: bool, pub lm3: bool, pub lm4: bool, pub footway: bool }
    /// 📦 Constituents of a load group, `None` for an unknown group.
    pub fn group_members(group: &str) -> Option<GroupMembers> {
        let g = |lm1, lm2, lm3, lm4, footway| Some(GroupMembers { lm1, lm2, lm3, lm4, footway });
        match group { "gr1a" => g(true, false, false, false, true), "gr1b" => g(false, true, false, false, false), "gr2" => g(true, false, false, false, false), "gr3" => g(false, false, false, false, true), "gr4" => g(false, false, false, true, false), "gr5" => g(true, false, true, false, false), _ => None }
    }
    /// 🏷️ Admissible load groups.
    pub const GROUPS: [&str; 6] = ["gr1a", "gr1b", "gr2", "gr3", "gr4", "gr5"];

    /// 🛣️ Lane layout of a bridge: the authored lanes (count, mean lane width, remaining width of the carriageway) or, without any, the notional lanes of Table 4.1.
    pub fn lane_layout(carriageway_width_m: f64, lane_widths_m: &[f64]) -> (u8, f64, f64) {
        if lane_widths_m.is_empty() { return notional_lanes(carriageway_width_m); }
        let total: f64 = lane_widths_m.iter().sum();
        (lane_widths_m.len().min(255) as u8, total / lane_widths_m.len() as f64, (carriageway_width_m - total).max(0.0))
    }
    /// 🛣️ Notional lanes from the carriageway width w — Table 4.1 → (n₁, w_lane, w_remaining).
    pub fn notional_lanes(carriageway_width_m: f64) -> (u8, f64, f64) {
        let w = carriageway_width_m.max(0.0);
        if w < 5.4 { (1, 3.0, (w - 3.0).max(0.0)) } else if w < 6.0 { (2, w / 2.0, 0.0) } else { let n = (w / 3.0).floor() as u8; (n, 3.0, w - 3.0 * f64::from(n)) }
    }
    /// 🧮 Remaining-area UDL (Pa): 2.5 kN/m².
    pub fn remaining_area_udl_pa(annex: AnnexChoice) -> f64 { alpha_q_udl(annex, 4) * 2.5 * 1000.0 }
    /// 🛑 Bounds (kN) of the braking force Q_lk (EN 1991-2, 4.4.1 (2)): 180 α_Q1 and 900, registry rows (unverified).
    pub fn braking_bounds_kn(annex: AnnexChoice) -> (f64, f64) {
        let ndp = |id: &str| semio_s_plugin_norm_compliance::ndp_number(id, annex) / 1000.0;
        (ndp("ndp.en1991.2.braking-min-per-alpha") * alpha_q(annex, 1), ndp("ndp.en1991.2.braking-max"))
    }
    /// 🛑 Braking force Q_lk (kN) = 0.6 α_Q1 (2 Q_1k) + 0.10 α_q1 q_1k w_1 L within the bounds of [`braking_bounds_kn`]; `lane_width_m` is w_1 of lane 1 and `loaded_length_m` the loaded length L of the deck (EN 1991-2, 4.4.1 (2), registry factors unverified).
    pub fn braking_force_kn(annex: AnnexChoice, loaded_length_m: f64, lane_width_m: f64) -> f64 {
        let factor = |id: &str| semio_s_plugin_norm_compliance::ndp_number(id, annex);
        let (low, high) = braking_bounds_kn(annex);
        let n = factor("ndp.en1991.2.braking-q-factor") * 2.0 * lm1_tandem_n(annex, 1) + factor("ndp.en1991.2.braking-udl-factor") * lm1_udl_pa(annex, 1) * lane_width_m * loaded_length_m;
        (n / 1000.0).max(low).min(high)
    }
    /// 🛣️ Width w_1 (m) of the notional lane 1 (EN 1991-2, 4.4.1 (2)): the first authored lane `(lane number, width)` numbered 1, else the notional lane width of the carriageway (Table 4.1).
    pub fn lane_one_width_m(carriageway_width_m: f64, lanes: &[(u8, f64)]) -> f64 { lanes.iter().find(|(number, _)| *number == 1).map_or_else(|| notional_lanes(carriageway_width_m).1, |(_, width)| *width) }
    /// 🛑 Braking force Q_lk (kN) of a bridge of the carriageway width and authored lanes `(lane number, width)` over the loaded length (w_1 from [`lane_one_width_m`]).
    pub fn braking_force_of_deck_kn(annex: AnnexChoice, loaded_length_m: f64, carriageway_width_m: f64, lanes: &[(u8, f64)]) -> f64 { braking_force_kn(annex, loaded_length_m, lane_one_width_m(carriageway_width_m, lanes)) }
    /// ⚖️ Total weight Q_v (kN) of the LM1 tandem systems of the lanes 1 to `lanes`: Σ 2 α_Qi Q_ik (EN 1991-2, 4.4.2).
    pub fn tandem_weight_kn(annex: AnnexChoice, lanes: u8) -> f64 { (1..=lanes.min(3)).map(|lane| 2.0 * lm1_tandem_n(annex, lane)).sum::<f64>() / 1000.0 }
    /// 🌀 Centrifugal force Q_tk (kN) from Q_v (kN) and the horizontal radius r (m) of Table 4.3: 0.2 Q_v below the lower radius, 40 Q_v / r between, 0 above the upper one; a radius of zero or less is a straight deck (registry rows unverified).
    pub fn centrifugal_force_kn(annex: AnnexChoice, q_v_kn: f64, r_m: f64) -> f64 {
        let ndp = |id: &str| semio_s_plugin_norm_compliance::ndp_number(id, annex);
        if r_m <= 0.0 || r_m > ndp("ndp.en1991.2.centrifugal-r-high") { 0.0 } else if r_m < ndp("ndp.en1991.2.centrifugal-r-low") { ndp("ndp.en1991.2.centrifugal-factor-low") * q_v_kn } else { ndp("ndp.en1991.2.centrifugal-numerator") * q_v_kn / r_m }
    }
    /// 🚶 Footbridge uniform load q_fk = 2 + 120/(L + 30) within [2.5, 5.0] kN/m².
    pub fn footbridge_load_kn_m2(span_m: f64) -> f64 { (2.0 + 120.0 / (span_m + 30.0)).clamp(2.5, 5.0) }
    /// 🚆 Classification factors α of the rail load models (EN 1991-2, 6.3.2).
    pub fn rail_alphas() -> [f64; 8] {
        [semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.1"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.2"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.3"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.4"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.5"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.6"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.7"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.2.rail-alpha.8")]
    }
    /// 🚆 LM71 axle load α·Q_vk (N) with Q_vk = 250 kN (EN 1991-2, Fig. 6.1).
    pub fn rail_lm71_axle_n(alpha: f64) -> f64 { alpha * 250_000.0 }
    /// 🚆 LM71 distributed load α·q_vk (N/m) with q_vk = 80 kN/m (EN 1991-2, Fig. 6.1).
    pub fn rail_lm71_udl_n_m(alpha: f64) -> f64 { alpha * 80_000.0 }
    /// 🚆 SW/0 load α·q_vk (N/m) with q_vk = 133 kN/m for continuous girders (EN 1991-2, Table 6.1).
    pub fn rail_sw0_n_m(alpha: f64) -> f64 { alpha * 133_000.0 }
    /// 🚆 SW/2 load q_vk (N/m) = 150 kN/m, applied without α (EN 1991-2, Table 6.1).
    pub fn rail_sw2_n_m() -> f64 { 150_000.0 }
    /// 🚆 Dynamic factors (Φ₂, Φ₃) of the LM71 for the determinant length L_Φ (m).
    pub fn rail_dynamic_factors(l_phi_m: f64) -> (f64, f64) { let r = (l_phi_m.sqrt() - 0.2).max(0.01); ((1.44 / r + 0.82).clamp(1.0, 1.67), (2.16 / r + 0.73).clamp(1.0, 2.0)) }
    /// 🚆 Reduction factor f of the rail centrifugal force for speed v (km/h) and influence length L_f (m).
    pub fn rail_centrifugal_f(v_kmh: f64, l_f_m: f64) -> f64 { if v_kmh <= 120.0 || l_f_m <= 2.88 { 1.0 } else { 1.0 - (v_kmh - 120.0) / 1000.0 * (814.0 / v_kmh + 1.75) * (1.0 - (2.88 / l_f_m).sqrt()) } }
    /// 🚆 Rail centrifugal force Q_tk = v²/(127 r)·f·Q_vk (kN).
    pub fn rail_centrifugal_kn(v_kmh: f64, r_m: f64, f: f64, q_vk_kn: f64) -> f64 { v_kmh * v_kmh / (127.0 * r_m) * f * q_vk_kn }
    /// 🧱 Wheel contact pressure (Pa) of a wheel load (N) on a rectangle a×b (m).
    pub fn wheel_pressure_pa(load_n: f64, a_m: f64, b_m: f64) -> f64 { load_n / (a_m * b_m) }
    /// 🚶 Footbridge uniform load q_fk = 2.0 + 120/(L + 30) kN/m² (Pa) of the loaded length L (m), within the registry bounds 2.5 to 5.0 kN/m² (EN 1991-2, 5.3.2.1, unverified).
    pub fn footbridge_q_fk_pa(loaded_length_m: f64) -> f64 { (ndp("ndp.en1991.2.footbridge-q-base") + ndp("ndp.en1991.2.footbridge-q-slope") / (loaded_length_m + ndp("ndp.en1991.2.footbridge-q-offset"))).clamp(ndp("ndp.en1991.2.footbridge-q-min"), ndp("ndp.en1991.2.footbridge-q-max")) }
    /// 🚶 Horizontal footbridge force Q_flk = max(10 % of the total vertical load q_fk L b, 10 kN) (N), both from the registry (EN 1991-2, 5.4, unverified).
    pub fn footbridge_horizontal_n(loaded_length_m: f64, width_m: f64) -> f64 { (ndp("ndp.en1991.2.footbridge-horizontal-fraction") * footbridge_q_fk_pa(loaded_length_m) * loaded_length_m * width_m).max(ndp("ndp.en1991.2.footbridge-horizontal-min")) }
    /// 🚶 Whether a vertical natural frequency (Hz) lies inside the pedestrian walking range of the registry (1.7 to 2.1 Hz, EN 1991-2, 5.7, unverified).
    pub fn footbridge_walking_resonance(frequency_hz: f64) -> bool { (ndp("ndp.en1991.2.footbridge-walk-f-min")..=ndp("ndp.en1991.2.footbridge-walk-f-max")).contains(&frequency_hz) }
    /// 🚶 Whether the comfort verification of EN 1990 A2.4.3.2(1) is triggered by a vertical natural frequency (Hz): the frequency lies below the registry bound of 5 Hz (unverified).
    pub fn footbridge_comfort_required(frequency_hz: f64) -> bool { frequency_hz < ndp("ndp.en1991.2.footbridge-band-vertical-max") }
    /// 🚆 Dynamic factor of one rail span: Φ₂ for careful, Φ₃ for standard maintenance, `None` for any other maintenance.
    pub fn rail_phi_for(maintenance: &str, l_phi_m: f64) -> Option<f64> { let (p2, p3) = rail_dynamic_factors(l_phi_m); match maintenance { "careful" => Some(p2), "standard" => Some(p3), _ => None } }
    /// 📐 Dispersed wheel footprint side (m) of a contact side a through pavement t_p and half the slab t_s at the registry spreading ratio (45 degrees): a + 2 (t_p + t_s/2) tan α (EN 1991-2, 4.3.6, unverified).
    pub fn dispersed_side_m(contact_m: f64, pavement_m: f64, slab_m: f64) -> f64 { contact_m + 2.0 * (pavement_m + 0.5 * slab_m) * ndp("ndp.en1991.2.dispersal-ratio") }
    /// 🔁 Admissible fatigue load models of the document: FLM1 and FLM3 (FLM2, FLM4 and FLM5 need Table 4.7 and traffic records that are not available).
    pub const FATIGUE_MODELS: [&str; 2] = ["flm1", "flm3"];
    /// 🚛 Admissible struck parts of a bridge collision: the support at deck level and the soffit.
    pub const COLLISION_LOCATIONS: [&str; 2] = ["deck", "soffit"];
    /// 🔁 FLM1 axle load (N) of the lane: the factor 0.7 on the LM1 tandem axle Q_ik (EN 1991-2, 4.6.2, unverified registry factor).
    pub fn fatigue_flm1_axle_n(annex: AnnexChoice, lane: u8) -> f64 { ndp("ndp.en1991.2.fatigue-flm1-q-factor") * lm1_tandem_n(annex, lane) }
    /// 🔁 FLM1 distributed load (Pa) of the lane: the factor 0.3 on the LM1 UDL q_ik (EN 1991-2, 4.6.2, unverified registry factor).
    pub fn fatigue_flm1_udl_pa(annex: AnnexChoice, lane: u8) -> f64 { ndp("ndp.en1991.2.fatigue-flm1-udl-factor") * lm1_udl_pa(annex, lane) }
    /// 🔁 FLM3 axle load (N): four axles of the registry weight (EN 1991-2, 4.6.4, unverified).
    pub fn fatigue_flm3_axle_n() -> f64 { ndp("ndp.en1991.2.fatigue-flm3-axle") }
    /// 🔁 FLM3 axle spacings (m): 1.2, 6.0 and 1.2 (registry rows, EN 1991-2, 4.6.4, unverified).
    pub fn fatigue_flm3_spacings_m() -> [f64; 3] { [ndp("ndp.en1991.2.fatigue-flm3-spacing-1"), ndp("ndp.en1991.2.fatigue-flm3-spacing-2"), ndp("ndp.en1991.2.fatigue-flm3-spacing-3")] }
    /// 🔁 FLM3 wheel track (m) and square wheel contact side (m) (EN 1991-2, 4.6.4, unverified registry rows).
    pub fn fatigue_flm3_track_m() -> f64 { ndp("ndp.en1991.2.fatigue-flm3-track") }
    pub fn fatigue_flm3_wheel_m() -> f64 { ndp("ndp.en1991.2.fatigue-flm3-wheel") }
    /// 🔁 FLM3 vehicle length (m): the sum of the three axle spacings.
    pub fn fatigue_flm3_length_m() -> f64 { fatigue_flm3_spacings_m().iter().sum() }
    /// 🚛 Collision force (N) on a support at deck level (`false`) or on the soffit (`true`) in driving direction (EN 1991-2, 4.7.3.1, unverified registry rows).
    pub fn collision_force_x_n(soffit: bool) -> f64 { ndp(if soffit { "ndp.en1991.2.collision-soffit-fx" } else { "ndp.en1991.2.collision-deck-fx" }) }
    /// 🚛 Collision force (N) across the driving direction on a support at deck level (EN 1991-2, 4.7.3.1, unverified registry row); the soffit has no value.
    pub fn collision_force_y_n() -> f64 { ndp("ndp.en1991.2.collision-deck-fy") }
}

pub mod part_3 {
    use crate::*;

    /// 🏗️ Static wheel load Q_r (N) of the crane class, `None` for an unknown class.
    pub fn crane_vertical_wheel_n(crane_class: &str) -> Option<f64> {
        let kn = match crane_class { "HC1" => 50.0, "HC2" => 100.0, "HC3" => 160.0, "HC4" => 250.0, _ => return None };
        Some(kn_to_n(kn))
    }
    /// 🧩 (φ₂,min, β₂) of the hoisting class, `None` for an unknown class.
    pub fn phi_2_params(hoist_class: &str) -> Option<(f64, f64)> { match hoist_class { "HC1" => Some((1.05, 0.17)), "HC2" => Some((1.10, 0.34)), "HC3" => Some((1.15, 0.51)), "HC4" => Some((1.20, 0.68)), _ => None } }
    /// 🧩 Dynamic factor φ₂ = φ₂,min + β₂ v_h.
    pub fn phi_2(hoist_class: &str, v_h: f64) -> Option<f64> { phi_2_params(hoist_class).map(|(min, beta)| min + beta * v_h.max(0.0)) }
    /// 🧩 φ₃ = 1 − (Δm/m)(1 + β₃) for sudden release of a payload fraction.
    pub fn phi_3(dm_over_m: f64, beta_3: f64) -> f64 { 1.0 - dm_over_m * (1.0 + beta_3) }
    /// 🪝 Release factor β₃ of the release device (`grab` 0.5, `magnet` 1.0, both registry rows, unverified), `None` for any other device.
    pub fn beta_3(device: &str) -> Option<f64> {
        match device {
            "grab" => Some(semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.3.beta-3-grab")),
            "magnet" => Some(semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.3.beta-3-magnet")),
            _ => None,
        }
    }
    /// 🧪 Static test load Q_T = 1.1 Q_H (N) of the rated hoist load, the factor from the registry (spec line E3.04, unverified).
    pub fn test_load_static_n(rated_load_n: f64) -> f64 { semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.3.test-static") * rated_load_n }
    /// 🧪 Dynamic test load Q_T = 1.25 Q_H (N) of the rated hoist load, the factor from the registry (spec line E3.04, unverified).
    pub fn test_load_dynamic_n(rated_load_n: f64) -> f64 { semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.3.test-dynamic") * rated_load_n }
    /// 🏗️ Design wheel load φ₂·Q_r (N) of the hoist load.
    pub fn design_vertical_wheel_n(crane_class: &str, hoist_class: &str, v_h: f64) -> Option<f64> { Some(crane_vertical_wheel_n(crane_class)? * phi_2(hoist_class, v_h)?) }
    /// 🌈️ Moving-crane envelope of a runway girder on a simply supported span: the largest bending moment (N·m), the largest shear force and end reaction (N) and the governing section (m) over `sections` equidistant sections while the two wheel loads `wheel_n` one `wheel_base_m` apart travel along the girder (same influence-line sweep as the LM1 tandem, [`part_2::tandem_section_envelope`]).
    pub fn runway_envelope(wheel_n: f64, span_m: f64, wheel_base_m: f64, sections: usize) -> (f64, f64, f64, f64) {
        let at = |i: usize| span_m * i as f64 / sections.max(1) as f64;
        (0..=sections.max(1)).map(|i| (at(i), part_2::tandem_section_envelope(wheel_n, span_m, wheel_base_m, at(i)))).fold((0.0, 0.0, 0.0, 0.0), |(m, v, r, x), (section, (moment, shear, _))| {
            let (m_new, x_new) = if moment > m { (moment, section) } else { (m, x) };
            (m_new, v.max(shear), if section == 0.0 { r.max(shear) } else { r }, x_new)
        })
    }
    /// ↔️ Drive factor μ = 0.2 (steel on steel) and acceleration factor φ₅ = 1.5.
    pub fn drive_friction_mu() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.3.drive-friction-mu")
    }
    pub fn phi_5() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.3.phi-5")
    }
    /// ↔️ Longitudinal force H_L = φ₅ μ Q_r (N) per runway beam with one driven wheel pair.
    pub fn design_horizontal_force_n(crane_class: &str, _annex: AnnexChoice) -> Option<f64> { crane_vertical_wheel_n(crane_class).map(|q| phi_5() * drive_friction_mu() * q) }
    /// 🌀 Skewing factor f = 0.3(1 − e^{−250α}).
    pub fn skew_factor(alpha_rad: f64) -> f64 { 0.3 * (1.0 - (-250.0 * alpha_rad).exp()) }
    /// 🛑 Buffer force H_B = φ₇ v₁ √(m_c S_B) (N).
    pub fn buffer_force_n(phi_7: f64, v_1: f64, mass_kg: f64, s_b_n_m: f64) -> f64 { phi_7 * v_1 * (mass_kg * s_b_n_m).sqrt() }
    /// ⚙️ Equivalent static force F_dyn = m e ω² (N) of a rotating machine with unbalanced mass m (kg), eccentricity e (m) and rotation frequency f (Hz), ω = 2π f (EN 1991-3, 5, unverified spec line).
    pub fn machine_dynamic_force_n(mass_kg: f64, eccentricity_m: f64, frequency_hz: f64) -> f64 { let omega = 2.0 * std::f64::consts::PI * frequency_hz; mass_kg * eccentricity_m * omega * omega }
}

pub mod part_4 {
    use crate::*;

    /// 🫙 Janssen asymptote p_ho = γ K z_o = γ a/μ (Pa) with a = A/U.
    pub fn janssen_p_ho_pa(bulk_n_m3: f64, a_m: f64, mu: f64) -> f64 { bulk_n_m3 * a_m / mu.max(1e-9) }
    /// 🫙 Characteristic depth z_o = A/(K μ U) (m).
    pub fn janssen_z_o_m(a_m: f64, mu: f64, k: f64) -> f64 { a_m / (k * mu).max(1e-9) }
    /// 🫙 Janssen horizontal wall pressure p_hf(z) = p_ho (1 − e^{−z/z_o}) (Pa) — EN 1991-4 Eq. (5.1).
    pub fn janssen_horizontal_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 { janssen_p_ho_pa(bulk_n_m3, a_m, mu) * (1.0 - (-depth_m / janssen_z_o_m(a_m, mu, k)).exp()) }
    /// 🫙 Vertical pressure p_vf = p_hf/K (Pa).
    pub fn janssen_vertical_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 { janssen_horizontal_pa(bulk_n_m3, a_m, mu, k, depth_m) / k.max(1e-9) }
    /// 📦 Patch pressure p_pf = 0.2·p_hf (Pa) — EN 1991-4 §5.2.4 recommended basic patch factor.
    pub fn patch_factor() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.4.patch-factor")
    }
    pub fn patch_pressure_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64, _annex: AnnexChoice) -> f64 { patch_factor() * janssen_horizontal_pa(bulk_n_m3, a_m, mu, k, depth_m) }
    /// 🧱 Wall friction traction p_wf = μ p_hf (Pa) — EN 1991-4 Eq. (5.3).
    pub fn wall_friction_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 { mu * janssen_horizontal_pa(bulk_n_m3, a_m, mu, k, depth_m) }
    /// 🫙 Maximum horizontal pressure with K_u = 1.2 K and μ_l = μ/1.15 (Pa).
    pub fn horizontal_max_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 { janssen_horizontal_pa(bulk_n_m3, a_m, mu / 1.15, 1.2 * k, depth_m) }
    /// 🧱 Maximum wall friction traction with K_l = K/1.2 and μ_u = 1.15 μ (Pa).
    pub fn wall_friction_max_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 { 1.15 * mu * janssen_horizontal_pa(bulk_n_m3, a_m, 1.15 * mu, k / 1.2, depth_m) }
    /// ⏬ Discharge factor C_h of the horizontal pressure p_he = C_h p_hf (EN 1991-4 §5.3, registry row, unverified).
    pub fn discharge_c_h() -> f64 { semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.4.discharge-c-h") }
    /// ⏬ Discharge factor C_w of the wall friction p_we = C_w p_wf (EN 1991-4 §5.3, registry row, unverified).
    pub fn discharge_c_w() -> f64 { semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.4.discharge-c-w") }
    /// ⏬ Upper characteristic discharge horizontal pressure p_he = C_h p_hf,max (Pa).
    pub fn discharge_horizontal_max_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 { discharge_c_h() * horizontal_max_pa(bulk_n_m3, a_m, mu, k, depth_m) }
    /// ⏬ Upper characteristic discharge wall friction p_we = C_w p_wf,max (Pa).
    pub fn discharge_wall_friction_max_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 { discharge_c_w() * wall_friction_max_pa(bulk_n_m3, a_m, mu, k, depth_m) }
    /// ⚖️ Mass held (t) by a cylinder of diameter `dc_m` and height `hc_m` filled with bulk solid of unit weight `bulk_n_m3`.
    pub fn silo_mass_t(bulk_n_m3: f64, dc_m: f64, hc_m: f64) -> f64 { std::f64::consts::PI / 4.0 * dc_m * dc_m * hc_m * bulk_n_m3 / 9.81 / 1000.0 }
    /// 📏 Slenderness class of EN 1991-4 §2.2 by h_c/d_c: retaining (≤ 0.4), squat (≤ 1.0), intermediate (< 2.0), slender.
    pub fn slenderness_class(ratio: f64) -> &'static str { if ratio <= 0.4 { "retaining" } else if ratio <= 1.0 { "squat" } else if ratio < 2.0 { "intermediate" } else { "slender" } }
    /// 🛢️ Tank hydrostatic pressure γ z (Pa).
    pub fn tank_hydrostatic_pa(bulk_n_m3: f64, depth_m: f64) -> f64 { bulk_n_m3 * depth_m }
    /// 🌾 Mean lateral pressure ratio K_m = 1.1 (1 − sin φ_i) of a bulk solid with the angle of internal friction φ_i (rad); the factor from the registry (EN 1991-4, Annex E, unverified).
    pub fn k_mean(phi_i_rad: f64) -> f64 { ndp("ndp.en1991.4.k-mean-factor") * (1.0 - phi_i_rad.sin()) }
    /// 🧭 Rankine active coefficient K_a = (1 − sin φ)/(1 + sin φ).
    pub fn rankine_active(phi_deg: f64) -> f64 { let s = phi_deg.to_radians().sin(); (1.0 - s) / (1.0 + s) }
    /// 📊 Action assessment class (EN 1991-4 Table 2.1) by capacity in tonnes: 1 below, 3 above the registry capacities (100 t, 10 000 t), 2 otherwise, so both boundary values are class 2; unverified registry rows.
    pub fn action_assessment_class(capacity_t: f64) -> u8 { if capacity_t < ndp("ndp.en1991.4.aac-class1-below") { 1 } else if capacity_t > ndp("ndp.en1991.4.aac-class3-above") { 3 } else { 2 } }
}
