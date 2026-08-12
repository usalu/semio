//! 🧬️ Din16798 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Din16798Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (sixty-two persistent scalar fields describing occupancy,
//! ventilation, comfort, heat-recovery, infiltration, cooling, storage and duct-leakage inputs to
//! a DIN EN 16798-1 compliance check) — no id-keyed collections, no name/identity field to
//! `rename`. Every field becomes its own `change-<field>` mutation per the rule's "change-<field>
//! per remaining scalar" clause; none qualify for the `update-<facet>` grouping exception (each
//! parameter is independently measured/entered, never validated as an atomic multi-field bundle).
//! `SetSnapshot` — the pre-migration whole-document replace — is gone: banned outright per
//! `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement mutation; file-open/import/
//! load-example now goes through `store::ArtifactStore::reset`, entirely outside this enum.
//!
//! `📄set-snapshot` keeps its pre-migration directory name — `📦️glue.rs` path-includes that exact
//! triad outside this facet's writable boundary, so it was repurposed in place (same path,
//! rewritten `🦠️mutation`/`🔺️diff`/`↩️inverse` content) to hold `ChangeAnnex` instead of being
//! renamed; see the migration report's `sharedFileRequests` for the rename once a later pass can
//! touch `📦️glue.rs`. The other sixty-one triads have no pre-migration slot and are self-wired
//! directly below via nested `#[path = "."] pub mod <name> { ... }` blocks (mirrors this ticket's
//! `process`/`process3d` precedent — `#[path]` resolves per physical file, not per logical mod
//! nesting, so this works without touching `📦️glue.rs`).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️NewLeaves
#[path = "."]
pub mod change_occupancy {
    #[path = "🔧change-occupancy/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-occupancy/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-occupancy/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_comfort_category {
    #[path = "🔧change-comfort-category/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-comfort-category/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-comfort-category/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_t_op_c {
    #[path = "🔧change-t-op-c/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-t-op-c/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-t-op-c/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_rh_percent {
    #[path = "🔧change-rh-percent/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-rh-percent/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-rh-percent/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_air_speed_m_s {
    #[path = "🔧change-air-speed-ms/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-air-speed-ms/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-air-speed-ms/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_theta_rm_c {
    #[path = "🔧change-theta-rm-c/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-theta-rm-c/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-theta-rm-c/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_co2_ppm {
    #[path = "🔧change-co2-ppm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-co2-ppm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-co2-ppm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_df_percent {
    #[path = "🔧change-df-percent/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-df-percent/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-df-percent/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_l_aeq_db {
    #[path = "🔧change-l-aeq-db/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-l-aeq-db/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-l-aeq-db/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_persons {
    #[path = "🔧change-persons/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-persons/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-persons/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ida_class {
    #[path = "🔧change-ida-class/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-ida-class/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-ida-class/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ventilation_m3_h {
    #[path = "🔧change-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-ventilation-m3-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-ventilation-m3-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_floor_area_m2 {
    #[path = "🔧change-floor-area-m2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-floor-area-m2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-floor-area-m2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_bedrooms {
    #[path = "🔧change-bedrooms/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-bedrooms/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-bedrooms/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_dwelling_ventilation_m3_h {
    #[path = "🔧change-dwelling-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-dwelling-ventilation-m3-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-dwelling-ventilation-m3-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_occupants {
    #[path = "🔧change-occupants/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-occupants/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-occupants/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_residential_ventilation_m3_h {
    #[path = "🔧change-residential-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-residential-ventilation-m3-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-residential-ventilation-m3-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_sfp_w_m3_s {
    #[path = "🔧change-sfp-wm3-s/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-sfp-wm3-s/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-sfp-wm3-s/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_sfp_required_class {
    #[path = "🔧change-sfp-required-class/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-sfp-required-class/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-sfp-required-class/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_heat_recovery_eta {
    #[path = "🔧change-heat-recovery-eta/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-heat-recovery-eta/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-heat-recovery-eta/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_heat_recovery_eta_min {
    #[path = "🔧change-heat-recovery-eta-min/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-heat-recovery-eta-min/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-heat-recovery-eta-min/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_system_type {
    #[path = "🔧change-system-type/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-system-type/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-system-type/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_years_since_inspection {
    #[path = "🔧change-years-since-inspection/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-years-since-inspection/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-years-since-inspection/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_humidification_required_kg_h {
    #[path = "🔧change-humidification-required-kg-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-humidification-required-kg-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-humidification-required-kg-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_humidification_provided_kg_h {
    #[path = "🔧change-humidification-provided-kg-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-humidification-provided-kg-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-humidification-provided-kg-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fan_q_v_m3_s {
    #[path = "🔧change-fan-qvm3-s/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-fan-qvm3-s/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-fan-qvm3-s/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fan_t_run_h {
    #[path = "🔧change-fan-t-run-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-fan-t-run-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-fan-t-run-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fan_energy_reference_kwh {
    #[path = "🔧change-fan-energy-reference-kwh/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-fan-energy-reference-kwh/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-fan-energy-reference-kwh/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_night_setback_k {
    #[path = "🔧change-night-setback-k/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-night-setback-k/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-night-setback-k/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_hr_m_dot_kg_s {
    #[path = "🔧change-hr-m-dot-kg-s/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-hr-m-dot-kg-s/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-hr-m-dot-kg-s/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_hr_cp_j_kgk {
    #[path = "🔧change-hr-cp-j-kgk/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-hr-cp-j-kgk/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-hr-cp-j-kgk/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_hr_delta_t_c {
    #[path = "🔧change-hr-delta-tc/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-hr-delta-tc/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-hr-delta-tc/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_hr_t_h {
    #[path = "🔧change-hr-th/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-hr-th/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-hr-th/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_hr_savings_reference_kwh {
    #[path = "🔧change-hr-savings-reference-kwh/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-hr-savings-reference-kwh/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-hr-savings-reference-kwh/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_n50_h_inv {
    #[path = "🔧change-n50-h-inv/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-n50-h-inv/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-n50-h-inv/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_volume_m3 {
    #[path = "🔧change-volume-m3/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-volume-m3/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-volume-m3/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_allowance_m3_h {
    #[path = "🔧change-infiltration-allowance-m3-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-infiltration-allowance-m3-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-infiltration-allowance-m3-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_cellar_area_m2 {
    #[path = "🔧change-cellar-area-m2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-cellar-area-m2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-cellar-area-m2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_cellar_ventilation_m3_h {
    #[path = "🔧change-cellar-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-cellar-ventilation-m3-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-cellar-ventilation-m3-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_h_tr_w_k {
    #[path = "🔧change-h-tr-wk/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-h-tr-wk/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-h-tr-wk/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_h_ve_w_k {
    #[path = "🔧change-h-ve-wk/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-h-ve-wk/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-h-ve-wk/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_theta_e_c {
    #[path = "🔧change-theta-ec/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-theta-ec/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-theta-ec/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_theta_set_c {
    #[path = "🔧change-theta-set-c/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-theta-set-c/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-theta-set-c/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_cooling_delta_t_h {
    #[path = "🔧change-cooling-delta-th/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-cooling-delta-th/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-cooling-delta-th/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_cooling_gains_kwh {
    #[path = "🔧change-cooling-gains-kwh/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-cooling-gains-kwh/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-cooling-gains-kwh/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_cooling_utilization_factor {
    #[path = "🔧change-cooling-utilization-factor/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-cooling-utilization-factor/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-cooling-utilization-factor/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_cooling_reference_kwh {
    #[path = "🔧change-cooling-reference-kwh/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-cooling-reference-kwh/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-cooling-reference-kwh/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_chiller_type {
    #[path = "🔧change-chiller-type/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-chiller-type/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-chiller-type/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_eer_actual {
    #[path = "🔧change-eer-actual/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-eer-actual/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-eer-actual/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_q_c_kwh {
    #[path = "🔧change-qc-kwh/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-qc-kwh/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-qc-kwh/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_generation_reference_kwh {
    #[path = "🔧change-generation-reference-kwh/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-generation-reference-kwh/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-generation-reference-kwh/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_data_center_supply_c {
    #[path = "🔧change-data-center-supply-c/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-data-center-supply-c/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-data-center-supply-c/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_h_st_w_k {
    #[path = "🔧change-h-st-wk/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-h-st-wk/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-h-st-wk/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_theta_st_c {
    #[path = "🔧change-theta-st-c/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-theta-st-c/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-theta-st-c/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_theta_amb_c {
    #[path = "🔧change-theta-amb-c/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-theta-amb-c/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-theta-amb-c/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_storage_t_h {
    #[path = "🔧change-storage-th/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-storage-th/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-storage-th/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_storage_allowance_kwh {
    #[path = "🔧change-storage-allowance-kwh/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-storage-allowance-kwh/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-storage-allowance-kwh/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_dhw_delivery_c {
    #[path = "🔧change-dhw-delivery-c/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-dhw-delivery-c/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-dhw-delivery-c/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_duct_class {
    #[path = "🔧change-duct-class/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-duct-class/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-duct-class/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_duct_test_pressure_pa {
    #[path = "🔧change-duct-test-pressure-pa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-duct-test-pressure-pa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-duct-test-pressure-pa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_duct_leakage_m3_s_m2 {
    #[path = "🔧change-duct-leakage-m3-sm2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-duct-leakage-m3-sm2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-duct-leakage-m3-sm2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
//#endregion 🔖️NewLeaves

//#region 🔖️RepurposedLeaves
// 🌱️ `set_snapshot` is declared by `📦️glue.rs` as a sibling of `component` (this file) under
// `pub mod mutations { ... }` — brought into this file's own scope the same way `process3d`'s
// already-migrated `🧬️mutations/🦀️component.rs` reaches its own repurposed siblings.
use super::set_snapshot;
//#endregion 🔖️RepurposedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the din16798 document, derived per
/// `📓️derivation-rules.md` from `Din16798Snapshot`'s flat scalar shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Din16798Snapshot, diff = Din16798Diff, schema = "norm.din16798")]
pub enum Din16798Mutation {
    ChangeAnnex(set_snapshot::mutation::ChangeAnnex),
    ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy),
    ChangeComfortCategory(change_comfort_category::mutation::ChangeComfortCategory),
    ChangeTOpC(change_t_op_c::mutation::ChangeTOpC),
    ChangeRhPercent(change_rh_percent::mutation::ChangeRhPercent),
    ChangeAirSpeedMS(change_air_speed_m_s::mutation::ChangeAirSpeedMS),
    ChangeThetaRmC(change_theta_rm_c::mutation::ChangeThetaRmC),
    ChangeCo2Ppm(change_co2_ppm::mutation::ChangeCo2Ppm),
    ChangeDfPercent(change_df_percent::mutation::ChangeDfPercent),
    ChangeLAeqDb(change_l_aeq_db::mutation::ChangeLAeqDb),
    ChangePersons(change_persons::mutation::ChangePersons),
    ChangeIdaClass(change_ida_class::mutation::ChangeIdaClass),
    ChangeVentilationM3H(change_ventilation_m3_h::mutation::ChangeVentilationM3H),
    ChangeFloorAreaM2(change_floor_area_m2::mutation::ChangeFloorAreaM2),
    ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms),
    ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H),
    ChangeOccupants(change_occupants::mutation::ChangeOccupants),
    ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H),
    ChangeSfpWM3S(change_sfp_w_m3_s::mutation::ChangeSfpWM3S),
    ChangeSfpRequiredClass(change_sfp_required_class::mutation::ChangeSfpRequiredClass),
    ChangeHeatRecoveryEta(change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta),
    ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin),
    ChangeSystemType(change_system_type::mutation::ChangeSystemType),
    ChangeYearsSinceInspection(change_years_since_inspection::mutation::ChangeYearsSinceInspection),
    ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH),
    ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH),
    ChangeFanQVM3S(change_fan_q_v_m3_s::mutation::ChangeFanQVM3S),
    ChangeFanTRunH(change_fan_t_run_h::mutation::ChangeFanTRunH),
    ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh),
    ChangeNightSetbackK(change_night_setback_k::mutation::ChangeNightSetbackK),
    ChangeHrMDotKgS(change_hr_m_dot_kg_s::mutation::ChangeHrMDotKgS),
    ChangeHrCpJKgk(change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk),
    ChangeHrDeltaTC(change_hr_delta_t_c::mutation::ChangeHrDeltaTC),
    ChangeHrTH(change_hr_t_h::mutation::ChangeHrTH),
    ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh),
    ChangeN50HInv(change_n50_h_inv::mutation::ChangeN50HInv),
    ChangeVolumeM3(change_volume_m3::mutation::ChangeVolumeM3),
    ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H),
    ChangeCellarAreaM2(change_cellar_area_m2::mutation::ChangeCellarAreaM2),
    ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::mutation::ChangeCellarVentilationM3H),
    ChangeHTrWK(change_h_tr_w_k::mutation::ChangeHTrWK),
    ChangeHVeWK(change_h_ve_w_k::mutation::ChangeHVeWK),
    ChangeThetaEC(change_theta_e_c::mutation::ChangeThetaEC),
    ChangeThetaSetC(change_theta_set_c::mutation::ChangeThetaSetC),
    ChangeCoolingDeltaTH(change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH),
    ChangeCoolingGainsKwh(change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh),
    ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor),
    ChangeCoolingReferenceKwh(change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh),
    ChangeChillerType(change_chiller_type::mutation::ChangeChillerType),
    ChangeEerActual(change_eer_actual::mutation::ChangeEerActual),
    ChangeQCKwh(change_q_c_kwh::mutation::ChangeQCKwh),
    ChangeGenerationReferenceKwh(change_generation_reference_kwh::mutation::ChangeGenerationReferenceKwh),
    ChangeDataCenterSupplyC(change_data_center_supply_c::mutation::ChangeDataCenterSupplyC),
    ChangeHStWK(change_h_st_w_k::mutation::ChangeHStWK),
    ChangeThetaStC(change_theta_st_c::mutation::ChangeThetaStC),
    ChangeThetaAmbC(change_theta_amb_c::mutation::ChangeThetaAmbC),
    ChangeStorageTH(change_storage_t_h::mutation::ChangeStorageTH),
    ChangeStorageAllowanceKwh(change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh),
    ChangeDhwDeliveryC(change_dhw_delivery_c::mutation::ChangeDhwDeliveryC),
    ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass),
    ChangeDuctTestPressurePa(change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa),
    ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::mutation::ChangeDuctLeakageM3SM2),
}
//#endregion 🔖️Mutations


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    /// ⚖️ One value per `Din16798Mutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring `process3d`'s own `every_mutation()` fixture.
    fn every_mutation() -> Vec<Din16798Mutation> {
        vec![
        Din16798Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: "office".to_string() }),
        Din16798Mutation::ChangeComfortCategory(change_comfort_category::mutation::ChangeComfortCategory { new_comfort_category: "I".to_string() }),
        Din16798Mutation::ChangeTOpC(change_t_op_c::mutation::ChangeTOpC { new_t_op_c: 24.5 }),
        Din16798Mutation::ChangeRhPercent(change_rh_percent::mutation::ChangeRhPercent { new_rh_percent: 45.0 }),
        Din16798Mutation::ChangeAirSpeedMS(change_air_speed_m_s::mutation::ChangeAirSpeedMS { new_air_speed_m_s: 0.15 }),
        Din16798Mutation::ChangeThetaRmC(change_theta_rm_c::mutation::ChangeThetaRmC { new_theta_rm_c: 18.0 }),
        Din16798Mutation::ChangeCo2Ppm(change_co2_ppm::mutation::ChangeCo2Ppm { new_co2_ppm: 900.0 }),
        Din16798Mutation::ChangeDfPercent(change_df_percent::mutation::ChangeDfPercent { new_df_percent: 3.0 }),
        Din16798Mutation::ChangeLAeqDb(change_l_aeq_db::mutation::ChangeLAeqDb { new_l_aeq_db: 28.0 }),
        Din16798Mutation::ChangePersons(change_persons::mutation::ChangePersons { new_persons: 12 }),
        Din16798Mutation::ChangeIdaClass(change_ida_class::mutation::ChangeIdaClass { new_ida_class: "1".to_string() }),
        Din16798Mutation::ChangeVentilationM3H(change_ventilation_m3_h::mutation::ChangeVentilationM3H { new_ventilation_m3_h: 320.0 }),
        Din16798Mutation::ChangeFloorAreaM2(change_floor_area_m2::mutation::ChangeFloorAreaM2 { new_floor_area_m2: 110.0 }),
        Din16798Mutation::ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms { new_bedrooms: 4 }),
        Din16798Mutation::ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: 70.0 }),
        Din16798Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: 4 }),
        Din16798Mutation::ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: 90.0 }),
        Din16798Mutation::ChangeSfpWM3S(change_sfp_w_m3_s::mutation::ChangeSfpWM3S { new_sfp_w_m3_s: 1600.0 }),
        Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::mutation::ChangeSfpRequiredClass { new_sfp_required_class: 3 }),
        Din16798Mutation::ChangeHeatRecoveryEta(change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta { new_heat_recovery_eta: 0.8 }),
        Din16798Mutation::ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: 0.72 }),
        Din16798Mutation::ChangeSystemType(change_system_type::mutation::ChangeSystemType { new_system_type: "decentral_mech".to_string() }),
        Din16798Mutation::ChangeYearsSinceInspection(change_years_since_inspection::mutation::ChangeYearsSinceInspection { new_years_since_inspection: 2 }),
        Din16798Mutation::ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: 2.5 }),
        Din16798Mutation::ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: 2.5 }),
        Din16798Mutation::ChangeFanQVM3S(change_fan_q_v_m3_s::mutation::ChangeFanQVM3S { new_fan_q_v_m3_s: 1.2 }),
        Din16798Mutation::ChangeFanTRunH(change_fan_t_run_h::mutation::ChangeFanTRunH { new_fan_t_run_h: 10.0 }),
        Din16798Mutation::ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: 18.0 }),
        Din16798Mutation::ChangeNightSetbackK(change_night_setback_k::mutation::ChangeNightSetbackK { new_night_setback_k: 4.0 }),
        Din16798Mutation::ChangeHrMDotKgS(change_hr_m_dot_kg_s::mutation::ChangeHrMDotKgS { new_hr_m_dot_kg_s: 0.6 }),
        Din16798Mutation::ChangeHrCpJKgk(change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk { new_hr_cp_j_kgk: 1006.0 }),
        Din16798Mutation::ChangeHrDeltaTC(change_hr_delta_t_c::mutation::ChangeHrDeltaTC { new_hr_delta_t_c: 16.0 }),
        Din16798Mutation::ChangeHrTH(change_hr_t_h::mutation::ChangeHrTH { new_hr_t_h: 12.0 }),
        Din16798Mutation::ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: 55.0 }),
        Din16798Mutation::ChangeN50HInv(change_n50_h_inv::mutation::ChangeN50HInv { new_n50_h_inv: 1.2 }),
        Din16798Mutation::ChangeVolumeM3(change_volume_m3::mutation::ChangeVolumeM3 { new_volume_m3: 540.0 }),
        Din16798Mutation::ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: 50.0 }),
        Din16798Mutation::ChangeCellarAreaM2(change_cellar_area_m2::mutation::ChangeCellarAreaM2 { new_cellar_area_m2: 55.0 }),
        Din16798Mutation::ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::mutation::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: 18.0 }),
        Din16798Mutation::ChangeHTrWK(change_h_tr_w_k::mutation::ChangeHTrWK { new_h_tr_w_k: 220.0 }),
        Din16798Mutation::ChangeHVeWK(change_h_ve_w_k::mutation::ChangeHVeWK { new_h_ve_w_k: 110.0 }),
        Din16798Mutation::ChangeThetaEC(change_theta_e_c::mutation::ChangeThetaEC { new_theta_e_c: 33.0 }),
        Din16798Mutation::ChangeThetaSetC(change_theta_set_c::mutation::ChangeThetaSetC { new_theta_set_c: 25.0 }),
        Din16798Mutation::ChangeCoolingDeltaTH(change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH { new_cooling_delta_t_h: 12.0 }),
        Din16798Mutation::ChangeCoolingGainsKwh(change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh { new_cooling_gains_kwh: 6.0 }),
        Din16798Mutation::ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: 0.85 }),
        Din16798Mutation::ChangeCoolingReferenceKwh(change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh { new_cooling_reference_kwh: 24.0 }),
        Din16798Mutation::ChangeChillerType(change_chiller_type::mutation::ChangeChillerType { new_chiller_type: "water_cooled".to_string() }),
        Din16798Mutation::ChangeEerActual(change_eer_actual::mutation::ChangeEerActual { new_eer_actual: 3.4 }),
        Din16798Mutation::ChangeQCKwh(change_q_c_kwh::mutation::ChangeQCKwh { new_q_c_kwh: 1200.0 }),
        Din16798Mutation::ChangeGenerationReferenceKwh(change_generation_reference_kwh::mutation::ChangeGenerationReferenceKwh { new_generation_reference_kwh: 420.0 }),
        Din16798Mutation::ChangeDataCenterSupplyC(change_data_center_supply_c::mutation::ChangeDataCenterSupplyC { new_data_center_supply_c: 24.0 }),
        Din16798Mutation::ChangeHStWK(change_h_st_w_k::mutation::ChangeHStWK { new_h_st_w_k: 6.0 }),
        Din16798Mutation::ChangeThetaStC(change_theta_st_c::mutation::ChangeThetaStC { new_theta_st_c: 62.0 }),
        Din16798Mutation::ChangeThetaAmbC(change_theta_amb_c::mutation::ChangeThetaAmbC { new_theta_amb_c: 21.0 }),
        Din16798Mutation::ChangeStorageTH(change_storage_t_h::mutation::ChangeStorageTH { new_storage_t_h: 20.0 }),
        Din16798Mutation::ChangeStorageAllowanceKwh(change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh { new_storage_allowance_kwh: 7.0 }),
        Din16798Mutation::ChangeDhwDeliveryC(change_dhw_delivery_c::mutation::ChangeDhwDeliveryC { new_dhw_delivery_c: 60.0 }),
        Din16798Mutation::ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass { new_duct_class: "B".to_string() }),
        Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: 450.0 }),
        Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::mutation::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: 0.08 }),
        ]
    }

    fn round_trip(base: &Din16798Snapshot, mutation: &Din16798Mutation) -> Din16798Snapshot {
        let forward = vcs::apply_mutation(base, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<Din16798Mutation as protocol::SemanticMutation<Din16798Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = Din16798Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit` — the bare `protocol::testkit` path is ambiguous crate-wide because `os_pack` also re-exports a `testkit` module), exercised against the three most structurally
    /// distinct variants: the repurposed enum-typed slot (`change-annex`), a typical `f64` scalar
    /// (`change-t-op-c`), and a `String` scalar (`change-occupancy`).

    #[test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: "office".to_string() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_t_op_c_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeTOpC(change_t_op_c::mutation::ChangeTOpC { new_t_op_c: 24.5 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din16798Mutation::ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms { new_bedrooms: 4 }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_occupancy_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: "office".to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din16798Mutation::ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass { new_duct_class: "B".to_string() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
