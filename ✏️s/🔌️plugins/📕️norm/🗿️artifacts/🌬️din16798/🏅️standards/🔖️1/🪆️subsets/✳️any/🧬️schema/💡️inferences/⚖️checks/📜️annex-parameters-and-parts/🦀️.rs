//! 📜️ DIN EN 16798 annex parameters and the clause modules of parts 1, 3, 5-1, 7 and 17.

use super::*;

pub mod annex_params {
    use crate::*;
    pub struct AnnexParams {
        pub choice: AnnexChoice,
    }
    impl AnnexParams {
        pub fn en() -> Self {
            Self { choice: AnnexChoice::En }
        }
        pub fn de() -> Self {
            Self { choice: AnnexChoice::De }
        }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice { AnnexChoice::En => Self::en(), AnnexChoice::De => Self::de() }
        }
    }
}

pub mod part_1 {
    pub use crate::{ComfortCategory, PollutionClass};

    use crate::*;

    pub fn pmv_limit(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 0.2, ComfortCategory::II => 0.5, ComfortCategory::III => 0.7, ComfortCategory::IV => 1.0 }
    }
    pub fn ppd_limit(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 6.0, ComfortCategory::II => 10.0, ComfortCategory::III => 15.0, ComfortCategory::IV => 25.0 }
    }
    /// 🌡️ Editable occupied-zone design references from Tables B.2 and NA.3: winter heating minimum and summer cooling maximum.
    pub fn seasonal_design_temperatures(annex: AnnexChoice, usage: OccupancyType, cat: ComfortCategory) -> Option<(f64, f64)> {
        let is_living_room = match (usage, annex) {
            (OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom, _) => false,
            (OccupancyType::Residential, AnnexChoice::En) => true,
            _ => return None,
        };
        Some(match (cat, is_living_room) {
            (ComfortCategory::I, _) => (21.0, 25.5),
            (ComfortCategory::II, _) => (20.0, 26.0),
            (ComfortCategory::III, false) => (19.0, 27.0),
            (ComfortCategory::III, true) => (18.0, 27.0),
            (ComfortCategory::IV, false) => (18.0, 28.0),
            (ComfortCategory::IV, true) => (16.0, 28.0),
        })
    }
    /// 💧 Installed humidity-control design references from Tables B.16 and NA.10.
    pub fn humidity_band(annex: AnnexChoice, cat: ComfortCategory) -> Option<(f64, f64)> {
        match (annex, cat) {
            (AnnexChoice::En, ComfortCategory::I) => Some((30.0, 50.0)),
            (AnnexChoice::En, ComfortCategory::II) => Some((25.0, 60.0)),
            (AnnexChoice::De, ComfortCategory::I) => Some((40.0, 50.0)),
            (AnnexChoice::De, ComfortCategory::II) => Some((30.0, 60.0)),
            (_, ComfortCategory::III) => Some((20.0, 70.0)),
            (_, ComfortCategory::IV) => None,
        }
    }
    /// 🫧 Standard design CO₂ concentration above outdoor air [ppm], Table B.9 or NA.9.
    pub fn co2_above_outdoor_ppm(annex: AnnexChoice, cat: ComfortCategory) -> f64 {
        match (annex, cat) {
            (AnnexChoice::De, ComfortCategory::I) => 350.0,
            (AnnexChoice::De, ComfortCategory::II) => 550.0,
            (AnnexChoice::De, ComfortCategory::III) => 900.0,
            (AnnexChoice::En, ComfortCategory::I) => 550.0,
            (AnnexChoice::En, ComfortCategory::II) => 800.0,
            _ => 1350.0,
        }
    }
    /// 💡 Maintained illuminance for the modelled office, meeting and teaching tasks.
    pub fn illuminance_min_lx(usage: OccupancyType) -> Option<f64> {
        match usage {
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => Some(500.0),
            _ => None,
        }
    }

    /// 🔊 Equipment sound level in the modelled room under Table B.20 or NA.11.
    pub fn acoustic_limit_db(annex: AnnexChoice, usage: OccupancyType, cat: ComfortCategory) -> Option<f64> {
        let levels = match (annex, usage) {
            (AnnexChoice::De, OccupancyType::Office | OccupancyType::Residential) => [30.0, 35.0, 35.0],
            (AnnexChoice::En, OccupancyType::Office | OccupancyType::Residential)
            | (_, OccupancyType::Meeting) => [30.0, 35.0, 40.0],
            (AnnexChoice::De, OccupancyType::Classroom) => [30.0, 35.0, 35.0],
            (AnnexChoice::En, OccupancyType::Classroom) => [30.0, 34.0, 38.0],
            (AnnexChoice::De, OccupancyType::Retail) => [40.0, 45.0, 50.0],
            (AnnexChoice::En, OccupancyType::Retail) | (_, OccupancyType::Corridor) => [35.0, 40.0, 45.0],
            (AnnexChoice::De, OccupancyType::Kitchen) => [50.0, 55.0, 60.0],
            (AnnexChoice::En, OccupancyType::Kitchen) => [45.0, 50.0, 55.0],
        };
        match cat {
            ComfortCategory::I => Some(levels[0]),
            ComfortCategory::II => Some(levels[1]),
            ComfortCategory::III => Some(levels[2]),
            ComfortCategory::IV => None,
        }
    }

    pub const ADAPTIVE_RUNNING_MEAN_RANGE_C: (f64, f64) = (10.0, 30.0);

    pub fn adaptive_comfort_temperature_c(theta_rm_c: f64) -> f64 { 0.33 * theta_rm_c + 18.8 }

    /// 📈️ Figure B.1 adaptive lower and upper operative design limits for the valid running-mean temperature range.
    pub fn adaptive_operative_bounds(theta_rm_c: f64, cat: ComfortCategory) -> Option<(f64, f64)> {
        let (min, max) = ADAPTIVE_RUNNING_MEAN_RANGE_C;
        if !(min < theta_rm_c && theta_rm_c < max) {
            return None;
        }
        let upper_offset = match cat {
            ComfortCategory::I => 2.0,
            ComfortCategory::II => 3.0,
            ComfortCategory::III => 4.0,
            ComfortCategory::IV => return None,
        };
        let centre = adaptive_comfort_temperature_c(theta_rm_c);
        Some((centre - upper_offset - 1.0, centre + upper_offset))
    }

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

    /// 🔍 The operative temperature [°C] at which the ISO 7730 PMV equals `+limit` (`prefer_cool`) or `-limit`: the bracket [16, 30] °C is doubled outwards until the PMV crosses the target (within [-10, 60] °C), then bisected; `None` when no sign change exists.
    pub fn invert_t_op_for_pmv(rh: f64, v: f64, met: f64, clo: f64, limit: f64, prefer_cool: bool) -> Option<f64> {
        let target = if prefer_cool { limit } else { -limit };
        let g = |t: f64| pmv_iso7730(t, rh, v, met, clo) - target;
        let (mut lo, mut hi) = (16.0_f64, 30.0_f64);
        let mut width = hi - lo;
        while g(lo) * g(hi) > 0.0 && (lo > -10.0 || hi < 60.0) {
            lo = (lo - width).max(-10.0);
            hi = (hi + width).min(60.0);
            width *= 2.0;
        }
        let g_lo = g(lo);
        if g_lo * g(hi) > 0.0 {
            return None;
        }
        for _ in 0..80 {
            let mid = 0.5 * (lo + hi);
            if g(mid) * g_lo > 0.0 { lo = mid; } else { hi = mid; }
        }
        Some(0.5 * (lo + hi))
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

    /// 🫁 Minimum design outdoor airflow in m³/h for non-residential occupancy with complete mixing, DIN EN 16798-1:2022-03 NA.5.1.1 and EN 16798-1:2019 B.3.1.1.
    pub fn hygienic_outdoor_air_minimum_m3_h(occupants: u32, usage: OccupancyType) -> f64 {
        match usage {
            OccupancyType::Residential => 0.0,
            _ => f64::from(occupants) * 4.0 * 3.6,
        }
    }

    /// 🏠 EN 16798-1:2019 Table B.10 default total rates for an office with non-adapted people.
    pub fn predefined_office_rates(cat: ComfortCategory) -> (f64, f64) {
        match cat {
            ComfortCategory::I => (20.0, 2.0),
            ComfortCategory::II => (14.0, 1.4),
            ComfortCategory::III => (8.0, 0.8),
            ComfortCategory::IV => (5.5, 0.55),
        }
    }

    /// 🏘️ EN 16798-1:2019 Table B.11 adapted-person and building rates for residential supply.
    pub fn residential_perceived_rates(cat: ComfortCategory) -> Option<(f64, f64)> {
        match cat {
            ComfortCategory::I => Some((3.5, 0.25)),
            ComfortCategory::II => Some((2.5, 0.15)),
            ComfortCategory::III => Some((1.5, 0.1)),
            ComfortCategory::IV => None,
        }
    }

    /// 📋️ An airflow design basis that cannot supply a numerical target for the given inputs.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OutdoorAirBasisGap {
        OutsideDefaultTable,
        GermanMethod3,
        GermanResidentialBasis,
        GermanVeryLowPollutionEvidence,
        NoPositiveCo2Gradient,
    }

    /// 🌬️ Required outdoor air [m³/h] for EN 16798-1:2019 design methods with complete mixing.
    #[allow(clippy::too_many_arguments, reason = "the design basis uses the annex, use, category and physical zone inputs")]
    pub fn required_outdoor_air_for_method(
        annex: AnnexChoice,
        method: VentMethod,
        occupants: u32,
        area_m2: f64,
        cat: ComfortCategory,
        usage: OccupancyType,
        pollution: PollutionClass,
        co2_limit_above_ppm: f64,
    ) -> Result<f64, OutdoorAirBasisGap> {
        if annex == AnnexChoice::De {
            if method == VentMethod::Method3PredefinedRates {
                return Err(OutdoorAirBasisGap::GermanMethod3);
            }
            if usage == OccupancyType::Residential {
                return Err(OutdoorAirBasisGap::GermanResidentialBasis);
            }
            if method == VentMethod::Method1PerceivedAirQuality && pollution == PollutionClass::VeryLow {
                return Err(OutdoorAirBasisGap::GermanVeryLowPollutionEvidence);
            }
        }
        let people = f64::from(occupants);
        let outside = OutdoorAirBasisGap::OutsideDefaultTable;
        match (method, usage) {
            (VentMethod::Method1PerceivedAirQuality, OccupancyType::Residential) => {
                let (person, area) = residential_perceived_rates(cat).ok_or(outside)?;
                Ok((people * person + area_m2 * area) * 3.6)
            }
            (VentMethod::Method1PerceivedAirQuality, _) => Ok(required_outdoor_air_m3_h(occupants, area_m2, cat, pollution)),
            (VentMethod::Method3PredefinedRates, OccupancyType::Residential) => {
                residential_perceived_rates(cat).ok_or(outside)?;
                Ok(people * outdoor_air_per_person_l_s(cat) * 3.6)
            }
            (VentMethod::Method3PredefinedRates, OccupancyType::Office) if pollution == PollutionClass::Low => {
                let (person, area) = predefined_office_rates(cat);
                Ok((people * person).max(area_m2 * area) * 3.6)
            }
            (VentMethod::Method3PredefinedRates, _) => Err(outside),
            (VentMethod::Method2LimitConcentration, _) if occupants == 0 => Ok(0.0),
            (VentMethod::Method2LimitConcentration, _) if co2_limit_above_ppm.is_finite() && co2_limit_above_ppm > 0.0 => {
                Ok(people * 20_000.0 / co2_limit_above_ppm)
            }
            (VentMethod::Method2LimitConcentration, _) => Err(OutdoorAirBasisGap::NoPositiveCo2Gradient),
        }
    }

    /// 💨 ISO 7730 draught rate DR [%] in the validated local-air input range.
    pub fn draught_rate_percent(t_a_c: f64, air_speed_m_s: f64, turbulence_intensity_percent: f64) -> Option<f64> {
        if !(20.0..=26.0).contains(&t_a_c) || !(0.0..0.5).contains(&air_speed_m_s)
            || !(10.0..=60.0).contains(&turbulence_intensity_percent)
        {
            return None;
        }
        Some(draught_prediction(t_a_c, air_speed_m_s, turbulence_intensity_percent))
    }

    fn draught_prediction(t_a_c: f64, air_speed_m_s: f64, turbulence_intensity_percent: f64) -> f64 {
        let speed = air_speed_m_s.max(0.05);
        let dr = (34.0 - t_a_c) * (speed - 0.05).powf(0.62) * (0.37 * speed * turbulence_intensity_percent + 3.14);
        dr.min(100.0)
    }

    /// 🔧 Maximum speed at the selected draught rate, bracketed by an already failing speed.
    pub fn maximum_air_speed_for_draught(t_a_c: f64, speed_m_s: f64, turbulence: f64, limit: f64) -> f64 {
        let (mut passing, mut failing) = (0.05, speed_m_s);
        for _ in 0..32 {
            let trial = (passing + failing) * 0.5;
            if draught_prediction(t_a_c, trial, turbulence) <= limit {
                passing = trial;
            } else {
                failing = trial;
            }
        }
        (passing * 1000.0).floor() / 1000.0
    }

    /// 🧊️ Draught rate design limits from Table B.3 or NA.4 for categories I to III.
    pub fn draught_limit_percent(annex: AnnexChoice, cat: ComfortCategory) -> Option<f64> {
        match (annex, cat) {
            (_, ComfortCategory::I) => Some(10.0),
            (AnnexChoice::De, ComfortCategory::II) => Some(15.0),
            (AnnexChoice::En, ComfortCategory::II) => Some(20.0),
            (_, ComfortCategory::III) => Some(30.0),
            (_, ComfortCategory::IV) => None,
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
