//! ⚖️ EN 1991 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

//#region 🔖️ComplianceHelpers
/// 📐️ EN 1991 design-load formulas (DIN EN + NA) — assumed vs required verification.
use semio_s_plugin_norm_compliance::{CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

fn pressure_pa(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }
fn force_n(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
fn temp_k(k: f64) -> Quantity { Quantity::new(QuantityKind::Temperature, k) }
fn kn_m2_to_pa(kn_m2: f64) -> f64 { kn_m2 * 1000.0 }
fn pa_to_kn_m2(pa: f64) -> f64 { pa / 1000.0 }
fn kn_to_n(kn: f64) -> f64 { kn * 1000.0 }
fn copy(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

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

pub fn assess_covers(id: &str, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, assumed: Quantity, required: Quantity, explanation: LocalizedCopy, annex: AnnexChoice, remedy: Option<Remedy>) -> CheckResult {
    let mut b = CheckResult::assess(id, part, clause, subject, title).minimum(assumed, required).annex(annex).explanation(explanation);
    if let Some(r) = remedy { b = b.remedy(r); }
    b.build()
}

pub mod na_de {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SnowZone { Zone1, Zone1a, Zone2, Zone2a, Zone3 }
    impl SnowZone {
        pub fn parse(s: &str) -> Self {
            match s.trim().to_ascii_lowercase().as_str() {
                "1" => Self::Zone1, "1a" => Self::Zone1a, "2" => Self::Zone2, "2a" => Self::Zone2a, "3" => Self::Zone3, _ => Self::Zone2,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WindZone { Zone1, Zone2, Zone3, Zone4 }
    impl WindZone {
        pub fn from_u8(z: u8) -> Self { match z { 1 => Self::Zone1, 2 => Self::Zone2, 3 => Self::Zone3, _ => Self::Zone4 } }
        pub fn v_b0_m_s(self) -> f64 { match self { Self::Zone1 => 22.5, Self::Zone2 => 25.0, Self::Zone3 => 27.5, Self::Zone4 => 30.0 } }
    }
}

pub mod part_1_1 {
    use crate::*;

    /// 🧱 Characteristic bulk density (N/m³) — EN 1991-1-1 Annex A.
    pub fn self_weight_n_m3(material: &str) -> f64 {
        let kn = match material {
            "concrete" | "reinforced_concrete" => 25.0,
            "lightweight_concrete" => 18.0,
            "steel" => 78.5,
            "timber" | "softwood" => 5.0,
            "hardwood" => 8.0,
            "glulam" => 4.2,
            "masonry" | "brick" => 18.0,
            "natural_stone" => 27.0,
            "aluminium" => 27.0,
            "glass" => 25.0,
            "water" => 10.0,
            "sand" | "loose_sand" => 18.0,
            "gravel" => 20.0,
            "asphalt" => 23.0,
            "plaster" => 14.0,
            _ => 20.0,
        };
        kn * 1000.0
    }
    pub fn self_weight_pa(material: &str, thickness_m: f64) -> f64 { self_weight_n_m3(material) * thickness_m }

    /// 🏢 Imposed q_k (Pa) — EN 1991-1-1 Table 6.1 / DIN EN NA Table 6.1DE (incl. I/J/K).
    pub fn imposed_qk_pa(category: &str, annex: AnnexChoice) -> f64 {
        let c = category.trim().to_ascii_uppercase();
        let kn = match annex {
            AnnexChoice::De => match c.as_str() {
                "A" | "A1" => 1.5, "A2" => 3.0, "A3" => 4.0,
                "B" | "B1" => 2.0, "B2" => 3.0, "B3" => 5.0,
                "C" | "C1" => 3.0, "C2" => 4.0, "C3" | "C4" | "C5" => 5.0,
                "D" | "D1" => 4.0, "D2" => 5.0,
                "E" | "E1" => 5.0, "E2" => 7.5,
                "F" => 2.0, "G" => 5.0, "H" => 0.75,
                "I" => 10.0, "J" => 5.0, "K" => 5.0,
                _ => 2.0,
            },
            AnnexChoice::En => match c.chars().next().unwrap_or('B') {
                'A' => 2.0, 'B' => 3.0, 'C' => 4.0, 'D' => 5.0, 'E' => 7.5,
                'F' => 2.5, 'G' => 5.0, 'H' => 0.4,
                'I' => 5.0, 'J' => 5.0, 'K' => 5.0,
                _ => 2.0,
            },
        };
        kn_m2_to_pa(kn)
    }

    pub fn imposed_qk_concentrated_n(category: &str, annex: AnnexChoice) -> f64 {
        let c = category.trim().to_ascii_uppercase();
        let kn = match (annex, c.as_str()) {
            (AnnexChoice::De, "C" | "C1" | "C2" | "C3" | "C4" | "C5" | "D" | "D1" | "D2") => 4.0,
            (AnnexChoice::De, "E" | "E1" | "E2" | "I" | "J" | "K") => 7.0,
            (AnnexChoice::De, "G") => 4.5,
            (AnnexChoice::De, "F") => 7.0,
            (_, "H") => 1.0,
            _ => 2.0,
        };
        kn_to_n(kn)
    }

    /// 📐 α_A — DIN EN 1991-1-1/NA Eq. (6.1DE): α_A = 0.7 ≤ 0.5 + 10/√A ≤ 1.0 (A in m²), categories C–E only; else 1.0.
    pub fn alpha_a(category: &str, area_m2: f64, annex: AnnexChoice) -> f64 {
        let c = category.trim().to_ascii_uppercase();
        let eligible = matches!(c.chars().next(), Some('C' | 'D' | 'E' | 'I' | 'J' | 'K'));
        if !eligible { return 1.0; }
        let a = area_m2.max(1.0);
        match annex {
            AnnexChoice::De => (0.5 + 10.0 / a.sqrt()).clamp(0.7, 1.0),
            AnnexChoice::En => (0.5 + 10.0 / a.sqrt()).clamp(0.7, 1.0),
        }
    }

    /// 🏬 α_n — DIN EN 1991-1-1/NA Eq. (6.2DE): α_n = 0.7 ≤ (2 + (n-2)·0.3)/n ≤ 1.0 for n ≥ 2 storeys (cat. A–D); else 1.0.
    pub fn alpha_n(category: &str, storey_count: u8, annex: AnnexChoice) -> f64 {
        let c = category.trim().to_ascii_uppercase();
        let eligible = matches!(c.chars().next(), Some('A' | 'B' | 'C' | 'D'));
        if !eligible || storey_count < 2 { return 1.0; }
        let n = storey_count as f64;
        match annex {
            AnnexChoice::De | AnnexChoice::En => ((2.0 + (n - 2.0) * 0.3) / n).clamp(0.7, 1.0),
        }
    }

    pub fn imposed_qk_reduced_pa(category: &str, annex: AnnexChoice, area_m2: f64, storey_count: u8) -> f64 {
        imposed_qk_pa(category, annex) * alpha_a(category, area_m2, annex) * alpha_n(category, storey_count, annex)
    }

    pub fn partitions_allowance_pa(annex: AnnexChoice) -> f64 {
        match annex {
            AnnexChoice::De => kn_m2_to_pa(0.8),
            AnnexChoice::En => kn_m2_to_pa(1.0),
        }
    }
}

pub mod part_1_3 {
    use crate::*;
    pub fn ground_snow_pa(zone: &str, altitude_m: f64) -> f64 {
        let z = na_de::SnowZone::parse(zone);
        let a = altitude_m.max(0.0);
        let kn = match z {
            na_de::SnowZone::Zone1 | na_de::SnowZone::Zone1a => {
                let base = if a <= 400.0 { 0.65 } else { 0.19 + 0.91 * ((a + 140.0) / 760.0).powi(2) };
                if matches!(z, na_de::SnowZone::Zone1a) { base * 1.25 } else { base }
            }
            na_de::SnowZone::Zone2 | na_de::SnowZone::Zone2a => {
                let base = if a <= 285.0 { 0.85 } else { 0.25 + 1.91 * ((a + 140.0) / 760.0).powi(2) };
                if matches!(z, na_de::SnowZone::Zone2a) { base * 1.25 } else { base }
            }
            na_de::SnowZone::Zone3 => if a <= 255.0 { 1.10 } else { 0.31 + 2.91 * ((a + 140.0) / 760.0).powi(2) },
        };
        kn_m2_to_pa(kn)
    }
    pub fn design_ground_snow_pa(annex: AnnexChoice, zone: &str, altitude_m: f64, en_sk_pa: f64) -> f64 {
        match annex { AnnexChoice::De => ground_snow_pa(zone, altitude_m), AnnexChoice::En => en_sk_pa }
    }
    pub fn shape_mu(roof_type: &str, pitch_deg: f64, has_parapet: bool, parapet_height: f64, drift_obstruction_height: f64, multi_span: bool, exceptional_north: bool) -> f64 {
        let alpha = pitch_deg.max(0.0);
        let mut mu = if alpha <= 30.0 { 0.8 } else if alpha < 60.0 { 0.8 * (60.0 - alpha) / 30.0 } else { 0.0 };
        if roof_type.contains("duo") && (15.0..30.0).contains(&alpha) { mu = mu.max(0.8); }
        if multi_span || roof_type.contains("multi") { mu = mu.max(1.6); }
        let h = parapet_height.max(drift_obstruction_height);
        if has_parapet {
            let h_par = h.max(0.5);
            mu = mu.max((2.0 * h_par).min(2.0));
        } else if h > 0.0 {
            mu = mu.max((2.0 * h).min(2.0).max(0.8));
        }
        if exceptional_north { mu = (mu * 2.0).min(2.0); }
        mu
    }
    pub fn roof_snow_pa(s_k_pa: f64, mu: f64, c_e: f64, c_t: f64) -> f64 { mu * c_e * c_t * s_k_pa }
}

pub mod part_1_4 {
    use crate::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TerrainCategory { Zero, I, II, III, IV }
    impl TerrainCategory {
        pub fn from_u8(v: u8) -> Self { match v { 0 => Self::Zero, 1 => Self::I, 2 => Self::II, 3 => Self::III, _ => Self::IV } }
        pub fn z_0_m(self) -> f64 { match self { Self::Zero => 0.003, Self::I => 0.01, Self::II => 0.05, Self::III => 0.3, Self::IV => 1.0 } }
        pub fn z_min_m(self) -> f64 { match self { Self::Zero | Self::I => 1.0, Self::II => 2.0, Self::III => 5.0, Self::IV => 10.0 } }
    }
    pub fn design_vb(annex: AnnexChoice, zone: u8, en_vb: f64) -> f64 {
        match annex { AnnexChoice::De => na_de::WindZone::from_u8(zone).v_b0_m_s(), AnnexChoice::En => en_vb }
    }
    pub fn basic_velocity_pressure_pa(rho: f64, vb: f64) -> f64 { 0.5 * rho * vb * vb }
    pub fn exposure_factor(z_m: f64, terrain: TerrainCategory) -> f64 {
        let z = z_m.max(terrain.z_min_m());
        let z0 = terrain.z_0_m();
        let kr = 0.19 * (z0 / 0.05_f64).powf(0.07);
        let c0 = kr * (z / z0).ln();
        c0 * c0
    }
    pub fn na_b3_qp_pa(zone: u8, terrain: u8, z_m: f64) -> f64 {
        let z = z_m.max(1.0);
        let t = terrain.min(4);
        let zone = zone.clamp(1, 4);
        let heights = [10.0_f64, 20.0, 50.0, 100.0];
        let table = match (zone, t) {
            (1, 0) | (1, 1) => [0.50, 0.65, 0.90, 1.10],
            (1, 2) => [0.40, 0.55, 0.80, 1.00],
            (1, 3) => [0.30, 0.45, 0.70, 0.90],
            (1, _) => [0.25, 0.40, 0.60, 0.80],
            (2, 0) | (2, 1) => [0.65, 0.80, 1.10, 1.30],
            (2, 2) => [0.55, 0.70, 0.95, 1.15],
            (2, 3) => [0.40, 0.55, 0.80, 1.00],
            (2, _) => [0.35, 0.50, 0.70, 0.90],
            (3, 0) | (3, 1) => [0.80, 1.00, 1.30, 1.55],
            (3, 2) => [0.70, 0.85, 1.15, 1.40],
            (3, 3) => [0.55, 0.70, 0.95, 1.20],
            (3, _) => [0.45, 0.60, 0.85, 1.05],
            (4, 0) | (4, 1) => [0.95, 1.15, 1.50, 1.80],
            (4, 2) => [0.80, 1.00, 1.35, 1.60],
            (4, 3) => [0.65, 0.85, 1.15, 1.40],
            (4, _) => [0.55, 0.70, 1.00, 1.20],
            _ => [0.55, 0.70, 0.95, 1.15],
        };
        if z <= heights[0] { return kn_m2_to_pa(table[0]); }
        for i in 0..3 {
            if z <= heights[i + 1] {
                let t = (z - heights[i]) / (heights[i + 1] - heights[i]);
                return kn_m2_to_pa(table[i] + t * (table[i + 1] - table[i]));
            }
        }
        kn_m2_to_pa(table[3])
    }
    pub fn peak_velocity_pressure_pa(annex: AnnexChoice, zone: u8, terrain: u8, z_m: f64, vb: f64, rho: f64, orography: f64, mixed_upwind: u8, mixed_distance: f64, coast: bool) -> f64 {
        let co = orography.max(1.0);
        let terrain_eff = if coast && annex == AnnexChoice::De { terrain.min(1) } else { terrain };
        if mixed_distance > 0.0 && mixed_distance < 2000.0 {
            let t = (mixed_distance / 2000.0).clamp(0.0, 1.0);
            let upwind = if coast && annex == AnnexChoice::De { 0 } else { mixed_upwind };
            let ce_a = exposure_factor(z_m, TerrainCategory::from_u8(upwind));
            let ce_b = exposure_factor(z_m, TerrainCategory::from_u8(terrain_eff));
            let ce = ce_a * (1.0 - t) + ce_b * t;
            return basic_velocity_pressure_pa(rho, vb) * ce * co;
        }
        if annex == AnnexChoice::De { return na_b3_qp_pa(zone, terrain_eff, z_m) * co * (rho / 1.25); }
        let ce = exposure_factor(z_m, TerrainCategory::from_u8(terrain_eff));
        basic_velocity_pressure_pa(rho, vb) * ce * co
    }
    
    /// 🧭 Reference size e = min(b, 2h) (m) — EN 1991-1-4 §7.2.2.
    /// 🧭 Reference dimension e = min(b, 2h) — EN 1991-1-4 Fig. 7.5 / 7.6.
    pub fn reference_e(b_m: f64, h_m: f64) -> f64 { b_m.min(2.0 * h_m) }

    /// 📏 Reference height z_e for peak pressure (Fig. 7.4 simplified by e = min(b, 2h)).
    pub fn reference_height_ze(z_m: f64, h_m: f64, e_m: f64) -> f64 {
        let e = e_m.max(0.1);
        let h = h_m.max(0.1);
        if h <= e {
            h
        } else if h <= 2.0 * e {
            if z_m <= e { e } else { h }
        } else if z_m <= e {
            e
        } else if z_m >= h - e {
            h
        } else {
            z_m
        }
    }

    /// 📐 Area-dependent external pressure coefficient — EN 1991-1-4 Fig. 7.2 log interpolation.
    pub fn area_cpe(c_pe1: f64, c_pe10: f64, area_m2: f64) -> f64 {
        if area_m2 <= 1.0 {
            c_pe1
        } else if area_m2 >= 10.0 {
            c_pe10
        } else {
            let t = (area_m2.ln() - 1.0_f64.ln()) / (10.0_f64.ln() - 1.0_f64.ln());
            c_pe1 + t * (c_pe10 - c_pe1)
        }
    }

    /// 🧱 Wall zone letter for windward/side/leeward from h/d — EN 1991-1-4 Fig. 7.5 / Table 7.1.
    pub fn wall_zone_letter(h_over_d: f64, face: &str) -> &'static str {
        let f = face.trim().to_ascii_uppercase();
        match f.as_str() {
            "D" | "WINDWARD" => "D",
            "E" | "LEEWARD" => "E",
            "A" | "SIDE_A" => "A",
            "B" | "SIDE_B" => "B",
            "C" | "SIDE_C" => "C",
            _ => {
                if h_over_d >= 5.0 { "D" } else if h_over_d >= 1.0 { "D" } else { "D" }
            }
        }
    }

    /// 📊 External pressure coefficient c_pe,10 — EN 1991-1-4 Tables 7.1 (walls) / 7.2–7.4 (roofs), DE NA unchanged for |c_pe|.
    pub fn tabulated_cpe10(zone: &str, h_over_d: f64, is_roof: bool, pitch_deg: f64) -> f64 {
        let z = zone.trim().to_ascii_uppercase();
        if is_roof {
            let a = pitch_deg.abs();
            return match z.as_str() {
                "F" => if a <= 5.0 { -1.8 } else if a <= 15.0 { -1.6 } else { -1.3 },
                "G" => if a <= 5.0 { -1.2 } else if a <= 15.0 { -1.1 } else { -0.9 },
                "H" => if a <= 5.0 { -0.7 } else if a <= 15.0 { -0.6 } else { -0.4 },
                "I" => 0.2,
                "J" => 0.2,
                _ => -1.0,
            };
        }
        match z.as_str() {
            "A" => -1.2,
            "B" => -0.8,
            "C" => -0.5,
            "D" => if h_over_d >= 5.0 { 0.8 } else if h_over_d >= 1.0 { 0.8 } else { 0.7 },
            "E" => if h_over_d >= 5.0 { -0.7 } else if h_over_d >= 1.0 { -0.5 } else { -0.3 },
            _ => 0.8,
        }
    }

    pub fn tabulated_cpe1(zone: &str, h_over_d: f64, is_roof: bool, pitch_deg: f64) -> f64 {
        let c10 = tabulated_cpe10(zone, h_over_d, is_roof, pitch_deg);
        if c10 < 0.0 { (c10 * 1.25).max(-2.0) } else { (c10 * 1.25).min(1.0) }
    }

    pub fn wind_pressure_pa(qp_pa: f64, c_pe: f64, c_pi: f64, c_s: f64, c_d: f64) -> f64 {
        qp_pa * (c_pe - c_pi).abs() * c_s * c_d
    }
}

pub mod part_1_2 {
    use crate::*;
    use semio_s_artifact_norm_en1991_model::part_1_2::FireCurve;

    /// 🌡️ ISO 834 / EN 1991-1-2 Eq. (3.4) standard fire gas temperature (°C) at t minutes.
    pub fn standard_gas_temp_c(t_min: f64) -> f64 {
        20.0 + 345.0 * (1.0 + t_min.max(0.0) / 8.0).log10()
    }
    /// 🌡️ External fire curve Eq. (3.5).
    pub fn external_gas_temp_c(t_min: f64) -> f64 {
        660.0 * (1.0 - 0.687 * (-0.32 * t_min.max(0.0)).exp() - 0.313 * (-3.9 * t_min.max(0.0)).exp()) + 20.0
    }
    /// 🌡️ Hydrocarbon curve Eq. (3.6).
    pub fn hydrocarbon_gas_temp_c(t_min: f64) -> f64 {
        1080.0 * (1.0 - 0.325 * (-0.167 * t_min.max(0.0)).exp() - 0.675 * (-2.5 * t_min.max(0.0)).exp()) + 20.0
    }

    pub fn nominal_gas_temp_k(curve: FireCurve, t_s: f64) -> f64 {
        let t_min = t_s / 60.0;
        let c = match curve {
            FireCurve::Standard => standard_gas_temp_c(t_min),
            FireCurve::External => external_gas_temp_c(t_min),
            FireCurve::Hydrocarbon => hydrocarbon_gas_temp_c(t_min),
            FireCurve::Parametric => standard_gas_temp_c(t_min),
        };
        c + 273.15
    }

    /// 🌡️ Parametric fire Annex A (simplified heating): θ_g = 1325·(1−0.324e^(−0.2t*)−0.204e^(−1.7t*)−0.472e^(−19t*)) with t* = t·Γ; Γ from O, b.
    pub fn parametric_gas_temp_k(t_s: f64, opening_factor_m05: f64, thermal_inertia_b: f64) -> f64 {
        let o = opening_factor_m05.max(0.02);
        let b = thermal_inertia_b.max(100.0);
        let gamma = (o / b).powi(2) / (0.04f64 / 1160.0).powi(2);
        let t_star = (t_s / 3600.0) * gamma;
        let c = 1325.0 * (1.0 - 0.324 * (-0.2 * t_star).exp() - 0.204 * (-1.7 * t_star).exp() - 0.472 * (-19.0 * t_star).exp());
        c + 273.15
    }

    /// 🚪 Opening factor O from compartment floor area A_f and height H (square plan, 20% wall openings, h_eq = H).
    pub fn compartment_opening_factor(af_m2: f64, height_m: f64) -> f64 {
        let af = af_m2.max(1.0);
        let h = height_m.max(0.5);
        let wall = 4.0 * af.sqrt() * h;
        let av = 0.2 * wall;
        let at = 2.0 * af + wall;
        av * h.sqrt() / at.max(1.0)
    }

    /// 🔥 Design fire load related to total surface q_t,d = q_f,d · A_f / A_t (Annex E).
    pub fn design_fire_load_qt_d_j_m2(qf_d_j_m2: f64, af_m2: f64, height_m: f64) -> f64 {
        let af = af_m2.max(1.0);
        let h = height_m.max(0.5);
        let wall = 4.0 * af.sqrt() * h;
        let at = 2.0 * af + wall;
        qf_d_j_m2 * af / at.max(1.0)
    }

    /// ⏱️ Parametric t_max (hours) — Annex A: t_max = (0.2e-3 · q_t,d)/O with q_t,d in MJ/m².
    pub fn parametric_t_max_hours(qt_d_j_m2: f64, opening_factor_m05: f64) -> f64 {
        let o = opening_factor_m05.max(0.02);
        let qt_mj = qt_d_j_m2 / 1.0e6;
        (0.2e-3 * qt_mj) / o
    }

    /// 🌡️ Parametric θ_max at t_max (K) for the heating branch.
    pub fn parametric_theta_max_k(opening_factor_m05: f64, thermal_inertia_b: f64, qt_d_j_m2: f64) -> f64 {
        let t_max_h = parametric_t_max_hours(qt_d_j_m2, opening_factor_m05);
        parametric_gas_temp_k(t_max_h * 3600.0, opening_factor_m05, thermal_inertia_b)
    }

    /// 🔥 Net heat flux h_net (W/m²) — EN 1991-1-2 Eq. (3.1)/(3.2): α_c·(θ_g−θ_m) + Φ·ε_m·ε_f·σ·((θ_g)⁴−(θ_m)⁴) with θ in K.
    pub fn h_net_w_m2(theta_g_k: f64, theta_m_k: f64, alpha_c: f64, phi: f64, epsilon_m: f64, epsilon_f: f64) -> f64 {
        let sigma = 5.67e-8;
        let conv = alpha_c * (theta_g_k - theta_m_k);
        let rad = phi * epsilon_m * epsilon_f * sigma * (theta_g_k.powi(4) - theta_m_k.powi(4));
        conv + rad
    }

    /// 🏭 Design fire load density q_f,d (J/m²) — EN 1991-1-2 Annex E: q_f,d = q_f,k · m · δ_q1 · δ_q2 · δ_n · δ_ni…
    /// DE NA replaces Annex E table with NA values; δ factors from occupancy / active measures.
    pub fn design_fire_load_j_m2(qf_k_j_m2: f64, annex: AnnexChoice, occupancy: &str, combustion_factor_m: f64) -> f64 {
        let m = combustion_factor_m.clamp(0.5, 1.0);
        let (dq1, dq2, dn) = fire_delta_factors(annex, occupancy);
        qf_k_j_m2 * m * dq1 * dq2 * dn
    }

    fn fire_delta_factors(annex: AnnexChoice, occupancy: &str) -> (f64, f64, f64) {
        let o = occupancy.trim().to_ascii_lowercase();
        match annex {
            AnnexChoice::De => match o.as_str() {
                "dwelling" | "residential" => (1.0, 0.8, 1.0),
                "office" => (1.0, 1.0, 1.0),
                "hospital" | "school" => (1.05, 1.0, 1.0),
                "hotel" => (1.05, 1.0, 1.0),
                "library" | "museum" => (1.0, 1.0, 1.0),
                "shopping" | "mall" => (1.1, 1.0, 1.0),
                "industrial" | "warehouse" => (1.2, 1.2, 1.0),
                "parking" => (1.0, 0.8, 1.0),
                _ => (1.0, 1.0, 1.0),
            },
            AnnexChoice::En => match o.as_str() {
                "dwelling" | "residential" => (1.0, 0.8, 1.0),
                "office" => (1.0, 1.0, 1.0),
                "hospital" | "school" | "hotel" => (1.05, 1.0, 1.0),
                "shopping" | "mall" => (1.1, 1.0, 1.0),
                "industrial" | "warehouse" => (1.2, 1.2, 1.0),
                _ => (1.0, 1.0, 1.0),
            },
        }
    }

    /// 🇩🇪 DE NA characteristic fire load densities q_f,k (MJ/m² → J/m²) for common occupancies (DIN EN 1991-1-2/NA).
    pub fn characteristic_fire_load_j_m2(annex: AnnexChoice, occupancy: &str) -> f64 {
        let o = occupancy.trim().to_ascii_lowercase();
        let mj = match (annex, o.as_str()) {
            (_, "dwelling" | "residential") => 780.0,
            (_, "office") => 420.0,
            (_, "hospital" | "school") => 350.0,
            (_, "hotel") => 377.0,
            (_, "library") => 1500.0,
            (_, "shopping" | "mall") => 600.0,
            (_, "industrial") => 300.0,
            (_, "warehouse") => 1180.0,
            (_, "parking") => 200.0,
            _ => 500.0,
        };
        mj * 1.0e6
    }

    pub fn alpha_c_for_curve(curve: FireCurve) -> f64 {
        match curve {
            FireCurve::Hydrocarbon => 50.0,
            _ => 25.0,
        }
    }
}

pub mod part_1_5 {
    use crate::*;

    /// 🌡️ Shade air temperature extremes — DIN EN 1991-1-5/NA isotherm defaults (°C → stored as K via caller).
    pub fn t_max_c(annex: AnnexChoice, provided_c: f64) -> f64 {
        if provided_c.abs() > 0.01 { return provided_c; }
        match annex { AnnexChoice::De => 37.0, AnnexChoice::En => 40.0 }
    }
    pub fn t_min_c(annex: AnnexChoice, provided_c: f64) -> f64 {
        if provided_c.abs() > 0.01 { return provided_c; }
        match annex { AnnexChoice::De => -24.0, AnnexChoice::En => -20.0 }
    }

    /// 🌡️ Required uniform temperature difference ΔT_u (K) for building elements (EN 1991-1-5 §5 / NA).
    pub fn required_delta_t_building(annex: AnnexChoice, t_max_in_c: f64, t_min_in_c: f64, t0_c: f64) -> f64 {
        let tmax = t_max_c(annex, t_max_in_c);
        let tmin = t_min_c(annex, t_min_in_c);
        let heating = (tmax - t0_c).abs();
        let cooling = (t0_c - tmin).abs();
        heating.max(cooling)
    }

    /// 🌉 Bridge type 1–3 linear temperature differences ΔT_N,exp / ΔT_N,con (K) — EN 1991-1-5 Table 6.1 / DE NA.
    pub fn bridge_delta_t_n(annex: AnnexChoice, bridge_type: u8, expansive: bool) -> f64 {
        let t = bridge_type.clamp(1, 3);
        match (annex, t, expansive) {
            (AnnexChoice::De, 1, true) => 15.0,
            (AnnexChoice::De, 1, false) => 18.0,
            (AnnexChoice::De, 2, true) => 15.0,
            (AnnexChoice::De, 2, false) => 18.0,
            (AnnexChoice::De, 3, true) => 10.0,
            (AnnexChoice::De, 3, false) => 13.0,
            (_, 1, true) => 18.0,
            (_, 1, false) => 13.0,
            (_, 2, true) => 15.0,
            (_, 2, false) => 18.0,
            (_, _, true) => 10.0,
            (_, _, false) => 15.0,
        }
    }

    /// 🌡️ Combined required |ΔT| from element type: building uses T_max/T_min/T_0; bridges use ΔT_N; vertical gradient ΔT_M added when non-zero.
    pub fn required_delta_t(annex: AnnexChoice, element_type: &str, t_max_in_c: f64, t_min_in_c: f64, t0_c: f64, bridge_type: u8, delta_t_m: f64) -> f64 {
        let et = element_type.trim().to_ascii_lowercase();
        let base = if et.starts_with("bridge") {
            let bt = if et.contains('2') { 2 } else if et.contains('3') { 3 } else { bridge_type.max(1) };
            bridge_delta_t_n(annex, bt, true).max(bridge_delta_t_n(annex, bt, false))
        } else {
            required_delta_t_building(annex, t_max_in_c, t_min_in_c, t0_c)
        };
        base + delta_t_m.max(0.0)
    }
}

pub mod part_1_6 {
    use crate::*;
    pub fn construction_qk_pa(activity: &str) -> f64 {
        let kn = match activity { "storage" => 2.0, "machinery" | "concreting" => 3.0, "scaffolding" => 1.0, _ => 0.5 };
        kn_m2_to_pa(kn)
    }
}

pub mod part_1_7 {
    #[cfg(test)] use crate::*;
    pub fn vehicle_impact_force_n(mass_kg: f64, speed_m_s: f64) -> f64 {
        let delta = 0.1;
        0.5 * mass_kg * speed_m_s * speed_m_s / delta
    }
    pub fn explosion_pressure_pa(mass_kg: f64, distance_m: f64) -> f64 {
        if distance_m < f64::EPSILON { return 0.0; }
        (2.0 * mass_kg / (distance_m * distance_m)) * 1000.0
    }
}

pub mod part_2 {
    use crate::*;

    /// 🌉 LM1 tandem axle group Q_ak (N) with DE-NA α_Q (DIN EN 1991-2/NA: α_Q1 = 0.9).
    pub fn lm1_tandem_n(annex: AnnexChoice, lane: u8) -> f64 {
        let base_kn = match lane { 1 => 300.0, 2 => 200.0, _ => 100.0 };
        kn_to_n(alpha_q(annex, lane) * base_kn)
    }

    /// 🌉 National axle factor α_Q for LM1 (EN recommended 1.0; DE lane 1 = 0.9).
    pub fn alpha_q(annex: AnnexChoice, lane: u8) -> f64 {
        match (annex, lane) {
            (AnnexChoice::De, 1) => 0.9,
            (AnnexChoice::De, 2) => 0.9,
            (AnnexChoice::De, _) => 0.9,
            _ => 1.0,
        }
    }

    /// 🌉 National UDL factor α_q (EN 1.0; DE NA Table NA.2.1: α_q1 = 0.4, α_q2 = 0.4, α_q3 = 0.4).
    pub fn alpha_q_udl(annex: AnnexChoice, lane: u8) -> f64 {
        match (annex, lane) {
            (AnnexChoice::De, _) => 0.4,
            _ => 1.0,
        }
    }

    /// 🌉 LM1 UDL q_ak (Pa) with α_q · Table 4.2 (9 / 2.5 / 2.5 kN/m²).
    pub fn lm1_udl_pa(annex: AnnexChoice, lane: u8) -> f64 {
        let q_kn_m2 = match lane { 1 => 9.0, 2 => 2.5, _ => 2.5 };
        alpha_q_udl(annex, lane) * q_kn_m2 * 1000.0
    }

    /// 🌉 LM2 single axle Q_ak (N) — EN 400 kN; DE NA α_Q2 = 0.9 → 360 kN.
    pub fn lm2_axle_n(annex: AnnexChoice) -> f64 {
        let alpha = match annex { AnnexChoice::De => 0.9, _ => 1.0 };
        kn_to_n(alpha * 400.0)
    }

    /// 🚶 Footway / cycle-track characteristic (Pa) — EN 5.0 kN/m²; DE NA keeps 5.0 on carriageway bridges with footways.
    pub fn footway_pa(_annex: AnnexChoice) -> f64 {
        5000.0
    }

    /// 🌉 Characteristic LM1 tandem moment demand proxy (N·m) on simply-supported span: α_Q·Q_k · L/4.
    pub fn lm1_tandem_moment_nm(annex: AnnexChoice, lane: u8, span_m: f64) -> f64 {
        lm1_tandem_n(annex, lane) * span_m / 4.0
    }

    /// 🚛 LM3 special vehicle characteristic axle (N) — EN 1991-2 §4.3.4; DE NA 600 kN class.
    pub fn lm3_axle_n(annex: AnnexChoice) -> f64 {
        let kn = match annex { AnnexChoice::De => 600.0, _ => 600.0 };
        kn_to_n(kn)
    }

    /// 👥 LM4 crowd loading (Pa) — EN 1991-2 §4.3.5 / Table 4.5: 5 kN/m².
    pub fn lm4_crowd_pa(_annex: AnnexChoice) -> f64 { 5000.0 }

    /// 📦 Load group gr1a–gr5 governing pressure/force factors (1.0 = include).
    pub fn load_group_includes_lm1(group: &str) -> bool { matches!(group, "gr1a" | "") }
    pub fn load_group_includes_lm2(group: &str) -> bool { group == "gr1b" }
    pub fn load_group_includes_lm3(group: &str) -> bool { group == "gr5" }
    pub fn load_group_includes_lm4(group: &str) -> bool { matches!(group, "gr4" | "gr3") }
    pub fn load_group_includes_footway(group: &str) -> bool { matches!(group, "gr1a" | "gr3") }

    /// 🛣️ Notional lanes from carriageway width w — EN 1991-2 Table 4.1 → (n₁, w_lane, w_remaining).
    pub fn notional_lanes(carriageway_width_m: f64) -> (u8, f64, f64) {
        let w = carriageway_width_m.max(0.0);
        if w < 5.4 {
            (1, w.max(0.1), 0.0)
        } else if w < 6.0 {
            (2, w / 2.0, 0.0)
        } else {
            let n = ((w / 3.0).floor() as u8).max(1);
            let w_lane = 3.0;
            let remaining = (w - 3.0 * f64::from(n)).max(0.0);
            (n, w_lane, remaining)
        }
    }

    /// 🧮 Remaining area UDL (Pa) — Table 4.2 remaining area 2.5 kN/m² × α_q3.
    pub fn remaining_area_udl_pa(annex: AnnexChoice) -> f64 {
        alpha_q_udl(annex, 3) * 2.5 * 1000.0
    }

}

pub mod part_3 {
    use crate::*;

    pub fn crane_vertical_wheel_n(crane_class: &str) -> f64 {
        let kn = match crane_class { "HC1" => 50.0, "HC2" => 100.0, "HC3" => 160.0, "HC4" => 250.0, _ => 80.0 };
        kn_to_n(kn)
    }

    pub fn phi_2(hoist_class: &str, v_h: f64) -> f64 {
        let (phi_min, beta) = match hoist_class {
            "HC1" => (1.05, 0.17),
            "HC2" => (1.10, 0.34),
            "HC3" => (1.15, 0.51),
            _ => (1.20, 0.68),
        };
        phi_min + beta * v_h.max(0.0)
    }

    pub const PHI_1: f64 = 1.1;

    /// 🏗️ Design vertical wheel load φ · Q_c (N) — EN 1991-3 §2.3 / Table 2.4.
    pub fn design_vertical_wheel_n(crane_class: &str, hoist_class: &str, v_h: f64) -> f64 {
        crane_vertical_wheel_n(crane_class) * PHI_1.max(phi_2(hoist_class, v_h))
    }

    /// ↔️ Horizontal longitudinal / skewing force (N) — EN 1991-3 §2.5: H = ξ · φ_5 · Q_r (φ_5 = 1.5; DE ξ = 0.15, EN ξ = 0.10). H = ξ · φ_5 · Q_r where Q_r is the static wheel load (without φ_1/φ_2).
    pub fn design_horizontal_force_n(crane_class: &str, annex: AnnexChoice) -> f64 {
        let phi_5 = 1.5;
        let xi = match annex { AnnexChoice::De => 0.15, _ => 0.10 };
        crane_vertical_wheel_n(crane_class) * xi * phi_5
    }
}

pub mod part_4 {
    use crate::*;

    /// 🫙 Janssen horizontal wall pressure (Pa) — EN 1991-4 Eq. (5.1)/(5.2).
    pub fn janssen_horizontal_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 {
        let denom = (mu * k).max(1e-9);
        let asymptote = bulk_n_m3 * a_m / denom;
        asymptote * (1.0 - (-depth_m * mu * k / a_m.max(1e-9)).exp())
    }

    /// 📦 Patch-load eccentricity pressure (Pa) — EN 1991-4 §5.2.1.2: p_h,patch = C_p · p_hf with C_p = 0.4 (DE slender) / 0.25 (EN).
    pub fn patch_pressure_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64, annex: AnnexChoice) -> f64 {
        let c_p = match annex { AnnexChoice::De => 0.4, _ => 0.25 };
        c_p * janssen_horizontal_pa(bulk_n_m3, a_m, mu, k, depth_m)
    }

    /// 🧱 Vertical wall friction traction (Pa) — EN 1991-4 Eq. (5.3): p_w = μ · p_h.
    pub fn wall_friction_pa(bulk_n_m3: f64, a_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 {
        mu * janssen_horizontal_pa(bulk_n_m3, a_m, mu, k, depth_m)
    }

    /// 🛢️ Tank hydrostatic pressure (Pa).
    pub fn tank_hydrostatic_pa(bulk_n_m3: f64, depth_m: f64) -> f64 {
        bulk_n_m3 * depth_m
    }
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
