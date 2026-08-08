//! ⚡️ DIN V 18599 app — document entities (constitutional: general).

use crate::document::ClimateZoneDe;
use serde::{Deserialize, Serialize};

// #region 🔖️Types
/// 🏢️ Building use class for energy reference area factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum UseClass {
    Residential,
    Office,
    School,
}

/// 📐️ Monthly climate data for balancing.
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

/// 📋️ Inputs for annual energy balancing.
pub type Document = BalancingInputs;

// 📌️ Deviation from the original monolith: `BalancingInputs::reference_residential(..)` (the
// physically-computed reference-building constructor, needing `norm_din4108_engine`'s
// `total_resistance`/`u_value_from_resistance` and `norm_din16798_engine`'s
// `residential_ventilation_rate`) moved to `crate::artifacts::din18599::engine::reference_residential` — an inherent
// impl here would need those crates, but inherent impls must live in the crate that defines the
// type (orphan rule), and `rs` must not depend on `engine` (the reverse of every other
// constitutional dependency edge). `Default` has the same orphan-rule constraint, so — matching
// the plain-literal `Default` style `din4108`/`din16798` already use — this is the numeric result
// of `reference_residential(ClimateZoneDe::Zone2, 100.0)`, precomputed once and inlined; use
// `crate::artifacts::din18599::engine::reference_residential` directly for a live-computed reference building.

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::din18599::snapshot::schema::Din18599Snapshot;
//#endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din18599", "DIN V 18599")
}
//#endregion 🔖️ArtifactKind
