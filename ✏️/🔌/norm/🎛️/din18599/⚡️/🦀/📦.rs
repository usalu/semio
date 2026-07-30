//! ⚡ DIN V 18599 app — document entities (constitutional: general).

use norm_core::ClimateZoneDe;
use serde::{Deserialize, Serialize};

// #region 🔖Types
/// 🏢 Building use class for energy reference area factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum UseClass {
    Residential,
    Office,
    School,
}

/// 📐 Monthly climate data for balancing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct MonthlyClimate {
    pub theta_e_c: [f64; 12],
    pub g_h_w_m2: [f64; 12],
}

impl MonthlyClimate {
    pub fn german_reference(zone: ClimateZoneDe) -> Self {
        let winter = zone.design_external_temperature_c();
        let summer = zone.summer_design_temperature_c();
        let mean = (winter + summer) / 2.0;
        let amplitude = (summer - winter) / 2.0;
        let mut theta_e = [0.0; 12];
        let g_h = [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0];
        for (i, t) in theta_e.iter_mut().enumerate() {
            let month = i as f64 + 1.0;
            *t = mean + amplitude * (2.0 * std::f64::consts::PI * (month - 7.0) / 12.0).cos();
        }
        Self { theta_e_c: theta_e, g_h_w_m2: g_h }
    }
}

/// 📋 Inputs for annual energy balancing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[dsl(extension = "din18599", layout = "lines")]
pub struct BalancingInputs {
    pub use_class: UseClass,
    pub heated_area_m2: f64,
    pub occupants: u32,
    pub h_t: f64,
    pub h_v: f64,
    #[dsl(block)]
    pub climate: MonthlyClimate,
    pub internal_gains_w_m2: f64,
    pub solar_gains_kwh: f64,
    pub system_losses_kwh: f64,
    pub renewable_kwh: f64,
    pub annual_limit_kwh: f64,
    pub energy_carrier: String,
    pub reference_q_p_kwh: f64,
}

pub type Document = BalancingInputs;

// 📌 Deviation from the original monolith: `BalancingInputs::reference_residential(..)` (the
// physically-computed reference-building constructor, needing `norm_din4108_engine`'s
// `total_resistance`/`u_value_from_resistance` and `norm_din16798_engine`'s
// `residential_ventilation_rate`) moved to `din18599_engine::reference_residential` — an inherent
// impl here would need those crates, but inherent impls must live in the crate that defines the
// type (orphan rule), and `rs` must not depend on `engine` (the reverse of every other
// constitutional dependency edge). `Default` has the same orphan-rule constraint, so — matching
// the plain-literal `Default` style `din4108`/`din16798` already use — this is the numeric result
// of `reference_residential(ClimateZoneDe::Zone2, 100.0)`, precomputed once and inlined; use
// `din18599_engine::reference_residential` directly for a live-computed reference building.
impl Default for Document {
    fn default() -> Self {
        Self {
            use_class: UseClass::Residential,
            heated_area_m2: 100.0,
            occupants: 4,
            h_t: 92.12124613902822,
            h_v: 40.800000000000004,
            climate: MonthlyClimate {
                theta_e_c: [-14.0, -11.186533479473212, -3.4999999999999964, 7.000000000000001, 17.5, 25.186533479473212, 28.0, 25.186533479473212, 17.5, 7.000000000000001, -3.4999999999999964, -11.186533479473212],
                g_h_w_m2: [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0],
            },
            internal_gains_w_m2: 3.5,
            solar_gains_kwh: 84.0,
            system_losses_kwh: 800.0,
            renewable_kwh: 1500.0,
            annual_limit_kwh: 7500.0,
            energy_carrier: "natural_gas".into(),
            reference_q_p_kwh: 10000.0,
        }
    }
}
// #endregion 🔖Types

// `impl store::DocumentDsl for BalancingInputs` and `UseClass`'s scalar-tag mapping are generated
// by `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslScalar)]` on the type definitions above.
