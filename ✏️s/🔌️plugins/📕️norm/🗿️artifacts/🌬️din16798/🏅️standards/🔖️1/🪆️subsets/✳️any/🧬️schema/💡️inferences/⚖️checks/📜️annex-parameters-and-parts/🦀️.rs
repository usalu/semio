//! 📜️ DIN EN 16798 annex parameters and the clause modules of parts 1, 3, 5-1, 7 and 17.

use super::*;

pub mod annex_params {
    use crate::*;
    pub struct AnnexParams {
        pub choice: AnnexChoice,
        pub acoustic_limit_residential_db: f64,
        pub co2_absolute_residential_ppm: f64,
        pub co2_absolute_classroom_ppm: f64,
        pub co2_absolute_other_ppm: f64,
    }
    impl AnnexParams {
        pub fn en() -> Self {
            Self { choice: AnnexChoice::En, acoustic_limit_residential_db: 30.0,
                co2_absolute_residential_ppm: 1500.0, co2_absolute_classroom_ppm: 1000.0, co2_absolute_other_ppm: 1000.0 }
        }
        pub fn de() -> Self {
            Self { choice: AnnexChoice::De, acoustic_limit_residential_db: 25.0,
                co2_absolute_residential_ppm: 1200.0, co2_absolute_classroom_ppm: 800.0, co2_absolute_other_ppm: 900.0 }
        }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice { AnnexChoice::En => Self::en(), AnnexChoice::De => Self::de() }
        }
    }
}

pub mod part_1 {
    pub use crate::{ComfortCategory, PollutionClass};

    use crate::*;
    use crate::annex_params::AnnexParams;

    pub fn pmv_limit(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 0.2, ComfortCategory::II => 0.5, ComfortCategory::III => 0.7, ComfortCategory::IV => 1.0 }
    }
    pub fn ppd_limit(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 6.0, ComfortCategory::II => 10.0, ComfortCategory::III => 15.0, ComfortCategory::IV => 25.0 }
    }
    pub fn operative_temp_band(cat: ComfortCategory, season: &str) -> (f64, f64) {
        match (cat, season) {
            (ComfortCategory::I, "winter") => (21.0, 23.0),
            (ComfortCategory::II, "winter") => (20.0, 24.0),
            (ComfortCategory::III, "winter") => (19.0, 25.0),
            (ComfortCategory::IV, "winter") => (18.0, 25.0),
            (ComfortCategory::I, _) => (23.5, 25.5),
            (ComfortCategory::II, _) => (23.0, 26.0),
            (ComfortCategory::III, _) => (22.0, 27.0),
            (ComfortCategory::IV, _) => (21.0, 28.0),
        }
    }
    pub fn humidity_band(cat: ComfortCategory) -> (f64, f64) {
        match cat {
            ComfortCategory::I => (30.0, 50.0),
            ComfortCategory::II => (25.0, 60.0),
            ComfortCategory::III => (20.0, 70.0),
            ComfortCategory::IV => (15.0, 75.0),
        }
    }
    pub fn co2_above_outdoor_ppm(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 550.0, ComfortCategory::II => 800.0, ComfortCategory::III => 1350.0, ComfortCategory::IV => 1350.0 }
    }
    /// 🫧 The CO₂ concentration above outdoor air [ppm] a category allows: the category value, under the German annex capped by the absolute limit of the usage minus the outdoor concentration.
    pub fn co2_limit_above_outdoor_ppm(annex: &AnnexParams, usage: OccupancyType, cat: ComfortCategory, outdoor_co2_ppm: f64) -> f64 {
        let category_limit = co2_above_outdoor_ppm(cat);
        if annex.choice != AnnexChoice::De {
            return category_limit;
        }
        let absolute = match usage {
            OccupancyType::Residential => annex.co2_absolute_residential_ppm,
            OccupancyType::Classroom => annex.co2_absolute_classroom_ppm,
            _ => annex.co2_absolute_other_ppm,
        };
        (absolute - outdoor_co2_ppm).min(category_limit)
    }
    pub fn illuminance_min_lx(usage: OccupancyType, cat: ComfortCategory) -> f64 {
        let base = match usage {
            OccupancyType::Office | OccupancyType::Meeting => 500.0,
            OccupancyType::Classroom => 300.0,
            OccupancyType::Retail => 300.0,
            OccupancyType::Kitchen => 500.0,
            OccupancyType::Corridor => 100.0,
            OccupancyType::Residential => 200.0,
        };
        match cat {
            ComfortCategory::I => base,
            ComfortCategory::II => base,
            ComfortCategory::III => base * 0.8,
            ComfortCategory::IV => base * 0.6,
        }
    }
    pub fn acoustic_limit_db(cat: ComfortCategory, usage: OccupancyType, annex: &AnnexParams) -> f64 {
        if matches!(usage, OccupancyType::Residential) {
            return annex.acoustic_limit_residential_db.min(match cat {
                ComfortCategory::I => 30.0, ComfortCategory::II => 35.0, ComfortCategory::III => 40.0, ComfortCategory::IV => 45.0,
            });
        }
        match cat { ComfortCategory::I => 30.0, ComfortCategory::II => 35.0, ComfortCategory::III => 40.0, ComfortCategory::IV => 45.0 }
    }
    pub fn adaptive_band_k(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 2.0, ComfortCategory::II => 3.0, ComfortCategory::III => 4.0, ComfortCategory::IV => 5.0 }
    }
    pub fn adaptive_comfort_temperature_c(theta_rm_c: f64) -> f64 { 0.33 * theta_rm_c + 18.8 }

    /// 😌️ ISO 7730:2005 Annex D predicted mean vote: the fixed-point iteration of the clothing surface temperature, `t_a = t_r = t_op`, no external work; `rh_percent` in %, `air_speed_m_s` in m/s, `metabolic_rate_met` in met and `clothing_clo` in clo.
    pub fn pmv_iso7730(t_op_c: f64, rh_percent: f64, air_speed_m_s: f64, metabolic_rate_met: f64, clothing_clo: f64) -> f64 {
        let (t_a, t_r) = (t_op_c, t_op_c);
        let p_a = rh_percent.clamp(0.0, 100.0) * 10.0 * (16.6536 - 4030.183 / (t_a + 235.0)).exp();
        let i_cl = 0.155 * clothing_clo.max(0.0);
        let m = metabolic_rate_met * 58.15;
        let f_cl = if i_cl <= 0.078 { 1.0 + 1.29 * i_cl } else { 1.05 + 0.645 * i_cl };
        let h_cf = 12.1 * air_speed_m_s.max(0.0).sqrt();
        let (t_aa, t_ra) = (t_a + 273.0, t_r + 273.0);
        let p1 = i_cl * f_cl;
        let (p2, p3, p4) = (p1 * 3.96, p1 * 100.0, p1 * t_aa);
        let p5 = 308.7 - 0.028 * m + p2 * (t_ra / 100.0).powi(4);
        let mut x_n = (t_aa + (35.5 - t_a) / (3.5 * i_cl + 0.1)) / 100.0;
        let mut x_f = x_n * 2.0;
        let mut h_c = h_cf;
        for _ in 0..150 {
            if (x_n - x_f).abs() <= 0.00015 {
                break;
            }
            x_f = (x_f + x_n) / 2.0;
            h_c = (2.38 * (100.0 * x_f - t_aa).abs().powf(0.25)).max(h_cf);
            x_n = (p5 + p4 * h_c - p2 * x_f.powi(4)) / (100.0 + p3 * h_c);
        }
        let t_cl = 100.0 * x_n - 273.0;
        let hl1 = 3.05e-3 * (5733.0 - 6.99 * m - p_a);
        let hl2 = if m > 58.15 { 0.42 * (m - 58.15) } else { 0.0 };
        let hl3 = 1.7e-5 * m * (5867.0 - p_a);
        let hl4 = 0.0014 * m * (34.0 - t_a);
        let hl5 = 3.96 * f_cl * (x_n.powi(4) - (t_ra / 100.0).powi(4));
        let hl6 = f_cl * h_c * (t_cl - t_a);
        let pmv = (0.303 * (-0.036 * m).exp() + 0.028) * (m - hl1 - hl2 - hl3 - hl4 - hl5 - hl6);
        if pmv.is_finite() { pmv.clamp(-3.0, 3.0) } else { 0.0 }
    }

    pub fn ppd_from_pmv(pmv: f64) -> f64 {
        let pmv_c = pmv.clamp(-3.0, 3.0);
        100.0 - 95.0 * (-0.03353 * pmv_c.powi(4) - 0.2179 * pmv_c.powi(2)).exp()
    }

    /// 🔍 Bisect operative temperature so |PMV| ≤ limit (summer side).
    pub fn invert_t_op_for_pmv(rh: f64, v: f64, met: f64, clo: f64, limit: f64, prefer_cool: bool) -> f64 {
        let (mut lo, mut hi) = if prefer_cool { (16.0, 30.0) } else { (16.0, 30.0) };
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let pmv = pmv_iso7730(mid, rh, v, met, clo);
            if prefer_cool {
                if pmv > limit { hi = mid; } else { lo = mid; }
            } else {
                if pmv < -limit { lo = mid; } else { hi = mid; }
            }
        }
        0.5 * (lo + hi)
    }

    // --- ventilation rate method Table B.6/B.7 ---
    pub fn outdoor_air_per_person_l_s(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 10.0, ComfortCategory::II => 7.0, ComfortCategory::III => 4.0, ComfortCategory::IV => 2.5 }
    }
    pub fn outdoor_air_per_area_l_s_m2(cat: ComfortCategory, pollution: PollutionClass) -> f64 {
        let (a, b, c, d) = match pollution {
            PollutionClass::VeryLow => (0.5, 0.35, 0.2, 0.15),
            PollutionClass::Low => (1.0, 0.7, 0.4, 0.3),
            PollutionClass::NonLow => (2.0, 1.4, 0.8, 0.6),
        };
        match cat { ComfortCategory::I => a, ComfortCategory::II => b, ComfortCategory::III => c, ComfortCategory::IV => d }
    }
    /// 📐️ Required outdoor airflow [m³/h] = (n·q_p + A·q_a) · 3.6
    pub fn required_outdoor_air_m3_h(occupants: u32, area_m2: f64, cat: ComfortCategory, pollution: PollutionClass) -> f64 {
        let q_l_s = occupants as f64 * outdoor_air_per_person_l_s(cat) + area_m2 * outdoor_air_per_area_l_s_m2(cat, pollution);
        q_l_s * 3.6
    }

    /// 🏠 EN 16798-1 Annex B predefined total outdoor-air intensity [L/(s·m²)] for method 3.
    pub fn predefined_outdoor_air_l_s_m2(cat: ComfortCategory) -> f64 {
        match cat {
            ComfortCategory::I => 1.4,
            ComfortCategory::II => 1.0,
            ComfortCategory::III => 0.6,
            ComfortCategory::IV => 0.4,
        }
    }

    /// 🏭️ Pollution multiplier applied to method-3 area intensity (EN 16798-1 Annex B building-emission classes).
    pub fn pollution_scale(pollution: PollutionClass) -> f64 {
        match pollution {
            PollutionClass::VeryLow => 0.8,
            PollutionClass::Low => 1.0,
            PollutionClass::NonLow => 1.25,
        }
    }

    /// 🧪 Method 2 metabolic CO₂ generation [L/s] ≈ 0.005 L/s per person at 1.2 met.
    pub fn metabolic_co2_generation_l_s(occupants: u32, metabolic_rate_met: f64) -> f64 {
        occupants as f64 * 0.005 * (metabolic_rate_met / 1.2).max(0.5)
    }

    /// 🌬️ Required outdoor air [m³/h] for the selected EN 16798-1 §6.3 design method.
    pub fn required_outdoor_air_for_method(
        method: VentMethod,
        occupants: u32,
        area_m2: f64,
        cat: ComfortCategory,
        pollution: PollutionClass,
        metabolic_rate_met: f64,
        annex_co2_limit_above_ppm: f64,
    ) -> f64 {
        match method {
            VentMethod::Method1PerceivedAirQuality => required_outdoor_air_m3_h(occupants, area_m2, cat, pollution),
            VentMethod::Method3PredefinedRates => {
                let q_area = area_m2 * predefined_outdoor_air_l_s_m2(cat) * pollution_scale(pollution) * 3.6;
                let q_person = occupants as f64 * outdoor_air_per_person_l_s(cat) * 3.6;
                q_area + q_person
            },
            VentMethod::Method2LimitConcentration => {
                let g = metabolic_co2_generation_l_s(occupants, metabolic_rate_met);
                let delta = annex_co2_limit_above_ppm.max(100.0);
                (1.0e6 * g / delta) * 3.6
            }
        }
    }

    /// 💨 ISO 7730 draught rate DR [%].
    pub fn draught_rate_percent(t_a_c: f64, air_speed_m_s: f64, turbulence_intensity_percent: f64) -> f64 {
        let v = air_speed_m_s.max(0.05);
        let tu = turbulence_intensity_percent.max(0.0);
        let dr = (34.0 - t_a_c) * (v - 0.05).max(0.0).powf(0.62) * (0.37 * v * tu + 3.14);
        dr.clamp(0.0, 100.0)
    }

    pub fn draught_limit_percent(cat: ComfortCategory) -> f64 {
        match cat {
            ComfortCategory::I => 10.0,
            ComfortCategory::II => 20.0,
            ComfortCategory::III => 30.0,
            ComfortCategory::IV => 40.0,
        }
    }
}

pub mod part_3 {
    use crate::*;

    pub const SFP_CLASS_BOUNDS_W_M3_S: [f64; 8] = [300.0, 500.0, 750.0, 1250.0, 2000.0, 3000.0, 4500.0, 6500.0]; // SFP 0..7

    pub fn sfp_bound(class: u8) -> f64 {
        let idx = (class as usize).min(7);
        SFP_CLASS_BOUNDS_W_M3_S[idx]
    }

    /// 🎯 Minimum heat-recovery temperature efficiency for balanced mechanical systems (EN 16798-3 / Ecodesign).
    pub fn heat_recovery_eta_min(system_type: &str) -> Option<f64> {
        match VentSystemType::parse(system_type) {
            VentSystemType::CentralMech | VentSystemType::DecentralMech => Some(0.73),
            VentSystemType::Natural => None,
        }
    }

    pub fn inspection_interval_years(system_type: &str) -> u32 {
        match VentSystemType::parse(system_type) {
            VentSystemType::DecentralMech => 5,
            _ => 3,
        }
    }

    /// 🧽 Required SUP filter class rank for ODA class (EN 16798-3 §7.2 / ISO 16890-1 ODA). Higher = finer.
    pub fn filter_rank(class: &str) -> u8 {
        match class {
            "ePM1_80_G" | "ePM1≥80+G" => 5,
            "ePM1_80" | "ePM1≥80" => 4,
            "ePM1_55" | "ePM1≥55" | "ePM1_50" => 3,
            "ePM2_5_65" | "ePM2.5_65" => 2,
            "ePM10_50" | "ePM10≥50" | "G4" => 1,
            _ => 0,
        }
    }
    pub fn required_filter_for_oda(oda: &str) -> (&'static str, u8) {
        match oda.to_ascii_uppercase().as_str() {
            "ODA1" => ("ePM10_50", 1),
            "ODA3" => ("ePM1_80", 4),
            "ODA4" => ("ePM1_80_G", 5),
            _ => ("ePM1_55", 3),
        }
    }

    pub fn duct_leakage_limit(class: &str, pressure_pa: f64) -> f64 {
        let c = match class.to_ascii_uppercase().as_str() {
            "A" => 0.027, "B" => 0.009, "C" => 0.003, "D" => 0.001, _ => 0.003,
        };
        c * pressure_pa.powf(0.65)
    }
}

pub mod part_5_1 {
    use crate::*;
    pub fn fan_energy_kwh(sfp: f64, q_v: f64, t_run: f64) -> f64 { sfp * q_v * t_run / 1000.0 }
    pub fn night_setback_min_k(usage: OccupancyType) -> f64 {
        match usage {
            OccupancyType::Residential => 3.0,
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => 4.0,
            _ => 2.0,
        }
    }
}

pub mod part_7 {
    #[cfg(test)] use crate::*;
    /// 🌬️ Infiltration volume flow [m³/h] ≈ n50 · V / 20 (shielding-corrected rule of thumb used with n50 limit).
    pub fn infiltration_m3_h(n50: f64, volume_m3: f64) -> f64 { n50 * volume_m3 / 20.0 }
    pub fn n50_max_mechanical() -> f64 { 3.0 }
    pub fn cellar_ventilation_required_m3_h(area_m2: f64) -> f64 { 0.3 * area_m2 }
}

pub mod part_17 {
    use crate::part_3;
    pub use part_3::duct_leakage_limit;
}
