//! 🧬️ En1998 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `En1998Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (forty-nine persistent scalar/boolean fields describing the EN 1998 seismic design check across buildings, bridges, retrofit, silos/tanks, towers, foundations and retaining walls) — no id-keyed
//! collections, no name/identity field to `rename`. Every field becomes its own `change-<field>`
//! mutation per the rule's "change-<field> per remaining scalar" clause; none qualify for the
//! `update-<facet>` grouping exception (each parameter is independently entered on its own input row,
//! never validated as an atomic multi-field bundle). The pre-migration whole-document-replace variant
//! is gone: banned outright per `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement
//! mutation; file-open/import/load-example now goes through `store::ArtifactStore::reset`, entirely
//! outside this enum. The old whole-document-replace macro call is removed with it.
//!
//! All triads are mounted directly as `mutations`-sibling modules in `📦️glue.rs` (this lane's agent
//! owns `📦️glue.rs`). The one exception is the `🧪️FixtureTests` region at the bottom of this file:
//! the per-mutation fixture cases self-wire from here, because `📦️glue.rs` is shared across all
//! fifteen norm artifacts and is under concurrent edit.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::change_annex;
use super::change_bearing_d_ed_mm;
use super::change_bearing_d_rd_mm;
use super::change_bridge_v_rd_kn;
use super::change_drift_mm;
use super::change_en_a_gr;
use super::change_en_ground_type;
use super::change_en_spectrum_type;
use super::change_foundation_area_m2;
use super::change_foundation_h_ed_kn;
use super::change_foundation_h_rd_kn;
use super::change_foundation_p_rd_kpa;
use super::change_ground_type;
use super::change_height_m;
use super::change_importance_class;
use super::change_k_foundation;
use super::change_k_soil;
use super::change_mass_t;
use super::change_multiple_resisting_systems;
use super::change_period_ratio;
use super::change_retrofit_e_d_kn;
use super::change_retrofit_gamma_el;
use super::change_retrofit_knowledge_level;
use super::change_retrofit_limit_state;
use super::change_retrofit_r_k_kn;
use super::change_seismic_zone;
use super::change_silo_height_m;
use super::change_silo_n_rd_kn;
use super::change_silo_q_nominal;
use super::change_silo_radius_m;
use super::change_silo_v_ed_kn;
use super::change_silo_v_rd_kn;
use super::change_structural_system;
use super::change_t1_s;
use super::change_tank_height_m;
use super::change_tank_mass_t;
use super::change_tank_radius_m;
use super::change_tank_v_rd_kn;
use super::change_tower_is_chimney;
use super::change_tower_m_ed_knm;
use super::change_tower_m_rd_knm;
use super::change_tower_mass_t;
use super::change_tower_q_nominal;
use super::change_v_rd_kn;
use super::change_wall_h_rd_kn;
use super::change_wall_height_m;
use super::change_wall_phi_deg;
use super::change_wall_r;
use super::change_wall_soil_gamma_kn_m3;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1998 document, derived per
/// `📓️derivation-rules.md` from `En1998Snapshot`'s flat scalar/enum shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1998Snapshot, diff = En1998Diff, schema = "norm.en1998")]
pub enum En1998Mutation {
    ChangeSeismicZone(change_seismic_zone::ChangeSeismicZone),
    ChangeGroundType(change_ground_type::ChangeGroundType),
    ChangeImportanceClass(change_importance_class::ChangeImportanceClass),
    ChangeStructuralSystem(change_structural_system::ChangeStructuralSystem),
    ChangeT1S(change_t1_s::ChangeT1S),
    ChangeMassT(change_mass_t::ChangeMassT),
    ChangeVRdKn(change_v_rd_kn::ChangeVRdKn),
    ChangeDriftMm(change_drift_mm::ChangeDriftMm),
    ChangeHeightM(change_height_m::ChangeHeightM),
    ChangeMultipleResistingSystems(change_multiple_resisting_systems::ChangeMultipleResistingSystems),
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeEnAGr(change_en_a_gr::ChangeEnAGr),
    ChangeEnGroundType(change_en_ground_type::ChangeEnGroundType),
    ChangeEnSpectrumType(change_en_spectrum_type::ChangeEnSpectrumType),
    ChangePeriodRatio(change_period_ratio::ChangePeriodRatio),
    ChangeBridgeVRdKn(change_bridge_v_rd_kn::ChangeBridgeVRdKn),
    ChangeBearingDEdMm(change_bearing_d_ed_mm::ChangeBearingDEdMm),
    ChangeBearingDRdMm(change_bearing_d_rd_mm::ChangeBearingDRdMm),
    ChangeRetrofitKnowledgeLevel(change_retrofit_knowledge_level::ChangeRetrofitKnowledgeLevel),
    ChangeRetrofitLimitState(change_retrofit_limit_state::ChangeRetrofitLimitState),
    ChangeRetrofitEDKn(change_retrofit_e_d_kn::ChangeRetrofitEDKn),
    ChangeRetrofitRKKn(change_retrofit_r_k_kn::ChangeRetrofitRKKn),
    ChangeRetrofitGammaEl(change_retrofit_gamma_el::ChangeRetrofitGammaEl),
    ChangeSiloHeightM(change_silo_height_m::ChangeSiloHeightM),
    ChangeSiloRadiusM(change_silo_radius_m::ChangeSiloRadiusM),
    ChangeSiloNRdKn(change_silo_n_rd_kn::ChangeSiloNRdKn),
    ChangeSiloVEdKn(change_silo_v_ed_kn::ChangeSiloVEdKn),
    ChangeSiloVRdKn(change_silo_v_rd_kn::ChangeSiloVRdKn),
    ChangeSiloQNominal(change_silo_q_nominal::ChangeSiloQNominal),
    ChangeTankHeightM(change_tank_height_m::ChangeTankHeightM),
    ChangeTankRadiusM(change_tank_radius_m::ChangeTankRadiusM),
    ChangeTankMassT(change_tank_mass_t::ChangeTankMassT),
    ChangeTankVRdKn(change_tank_v_rd_kn::ChangeTankVRdKn),
    ChangeTowerMEdKnm(change_tower_m_ed_knm::ChangeTowerMEdKnm),
    ChangeTowerMRdKnm(change_tower_m_rd_knm::ChangeTowerMRdKnm),
    ChangeTowerIsChimney(change_tower_is_chimney::ChangeTowerIsChimney),
    ChangeTowerQNominal(change_tower_q_nominal::ChangeTowerQNominal),
    ChangeTowerMassT(change_tower_mass_t::ChangeTowerMassT),
    ChangeFoundationAreaM2(change_foundation_area_m2::ChangeFoundationAreaM2),
    ChangeFoundationPRdKpa(change_foundation_p_rd_kpa::ChangeFoundationPRdKpa),
    ChangeFoundationHEdKn(change_foundation_h_ed_kn::ChangeFoundationHEdKn),
    ChangeFoundationHRdKn(change_foundation_h_rd_kn::ChangeFoundationHRdKn),
    ChangeKFoundation(change_k_foundation::ChangeKFoundation),
    ChangeKSoil(change_k_soil::ChangeKSoil),
    ChangeWallHeightM(change_wall_height_m::ChangeWallHeightM),
    ChangeWallPhiDeg(change_wall_phi_deg::ChangeWallPhiDeg),
    ChangeWallSoilGammaKnM3(change_wall_soil_gamma_kn_m3::ChangeWallSoilGammaKnM3),
    ChangeWallR(change_wall_r::ChangeWallR),
    ChangeWallHRdKn(change_wall_h_rd_kn::ChangeWallHRdKn),
}

/// 🏷️ Every declared kind of [`En1998Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🧪️oracle/🔣️.json` publishes as the `en1998-1-any`
/// mutation catalog and `../../../../../🧪️tests/mutate-en1998-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-seismic-zone",
    "change-ground-type",
    "change-importance-class",
    "change-structural-system",
    "change-t1-s",
    "change-mass-t",
    "change-v-rd-kn",
    "change-drift-mm",
    "change-height-m",
    "change-multiple-resisting-systems",
    "change-annex",
    "change-en-a-gr",
    "change-en-ground-type",
    "change-en-spectrum-type",
    "change-period-ratio",
    "change-bridge-v-rd-kn",
    "change-bearing-d-ed-mm",
    "change-bearing-d-rd-mm",
    "change-retrofit-knowledge-level",
    "change-retrofit-limit-state",
    "change-retrofit-ed-kn",
    "change-retrofit-rk-kn",
    "change-retrofit-gamma-el",
    "change-silo-height-m",
    "change-silo-radius-m",
    "change-silo-n-rd-kn",
    "change-silo-v-ed-kn",
    "change-silo-v-rd-kn",
    "change-silo-q-nominal",
    "change-tank-height-m",
    "change-tank-radius-m",
    "change-tank-mass-t",
    "change-tank-v-rd-kn",
    "change-tower-m-ed-knm",
    "change-tower-m-rd-knm",
    "change-tower-is-chimney",
    "change-tower-q-nominal",
    "change-tower-mass-t",
    "change-foundation-area-m2",
    "change-foundation-p-rd-kpa",
    "change-foundation-h-ed-kn",
    "change-foundation-h-rd-kn",
    "change-k-foundation",
    "change-k-soil",
    "change-wall-height-m",
    "change-wall-phi-deg",
    "change-wall-soil-gamma-kn-m3",
    "change-wall-r",
    "change-wall-h-rd-kn",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1998Mutation {
    /// 📤️ Decomposes a whole `En1998Snapshot` into one `change-<field>` mutation per
    /// persistent field — the closed-vocabulary replacement for the banned whole-document-replace
    /// variant, used by `import_media`'s `"model:in"` port and the `set-snapshot` app command to
    /// bundle a bulk document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &En1998Snapshot) -> Vec<En1998Mutation> {
        let mut mutations = Vec::with_capacity(49);
        mutations.push(En1998Mutation::ChangeSeismicZone(change_seismic_zone::ChangeSeismicZone { new_seismic_zone: snapshot.seismic_zone.clone() }));
        mutations.push(En1998Mutation::ChangeGroundType(change_ground_type::ChangeGroundType { new_ground_type: snapshot.ground_type.clone() }));
        mutations.push(En1998Mutation::ChangeImportanceClass(change_importance_class::ChangeImportanceClass { new_importance_class: snapshot.importance_class.clone() }));
        mutations.push(En1998Mutation::ChangeStructuralSystem(change_structural_system::ChangeStructuralSystem { new_structural_system: snapshot.structural_system.clone() }));
        mutations.push(En1998Mutation::ChangeT1S(change_t1_s::ChangeT1S { new_t1_s: snapshot.t1_s.clone() }));
        mutations.push(En1998Mutation::ChangeMassT(change_mass_t::ChangeMassT { new_mass_t: snapshot.mass_t.clone() }));
        mutations.push(En1998Mutation::ChangeVRdKn(change_v_rd_kn::ChangeVRdKn { new_v_rd_kn: snapshot.v_rd_kn.clone() }));
        mutations.push(En1998Mutation::ChangeDriftMm(change_drift_mm::ChangeDriftMm { new_drift_mm: snapshot.drift_mm.clone() }));
        mutations.push(En1998Mutation::ChangeHeightM(change_height_m::ChangeHeightM { new_height_m: snapshot.height_m.clone() }));
        mutations.push(En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::ChangeMultipleResistingSystems { new_multiple_resisting_systems: snapshot.multiple_resisting_systems.clone() }));
        mutations.push(En1998Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex.clone() }));
        mutations.push(En1998Mutation::ChangeEnAGr(change_en_a_gr::ChangeEnAGr { new_en_a_gr: snapshot.en_a_gr.clone() }));
        mutations.push(En1998Mutation::ChangeEnGroundType(change_en_ground_type::ChangeEnGroundType { new_en_ground_type: snapshot.en_ground_type.clone() }));
        mutations.push(En1998Mutation::ChangeEnSpectrumType(change_en_spectrum_type::ChangeEnSpectrumType { new_en_spectrum_type: snapshot.en_spectrum_type.clone() }));
        mutations.push(En1998Mutation::ChangePeriodRatio(change_period_ratio::ChangePeriodRatio { new_period_ratio: snapshot.period_ratio.clone() }));
        mutations.push(En1998Mutation::ChangeBridgeVRdKn(change_bridge_v_rd_kn::ChangeBridgeVRdKn { new_bridge_v_rd_kn: snapshot.bridge_v_rd_kn.clone() }));
        mutations.push(En1998Mutation::ChangeBearingDEdMm(change_bearing_d_ed_mm::ChangeBearingDEdMm { new_bearing_d_ed_mm: snapshot.bearing_d_ed_mm.clone() }));
        mutations.push(En1998Mutation::ChangeBearingDRdMm(change_bearing_d_rd_mm::ChangeBearingDRdMm { new_bearing_d_rd_mm: snapshot.bearing_d_rd_mm.clone() }));
        mutations.push(En1998Mutation::ChangeRetrofitKnowledgeLevel(change_retrofit_knowledge_level::ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level: snapshot.retrofit_knowledge_level.clone() }));
        mutations.push(En1998Mutation::ChangeRetrofitLimitState(change_retrofit_limit_state::ChangeRetrofitLimitState { new_retrofit_limit_state: snapshot.retrofit_limit_state.clone() }));
        mutations.push(En1998Mutation::ChangeRetrofitEDKn(change_retrofit_e_d_kn::ChangeRetrofitEDKn { new_retrofit_e_d_kn: snapshot.retrofit_e_d_kn.clone() }));
        mutations.push(En1998Mutation::ChangeRetrofitRKKn(change_retrofit_r_k_kn::ChangeRetrofitRKKn { new_retrofit_r_k_kn: snapshot.retrofit_r_k_kn.clone() }));
        mutations.push(En1998Mutation::ChangeRetrofitGammaEl(change_retrofit_gamma_el::ChangeRetrofitGammaEl { new_retrofit_gamma_el: snapshot.retrofit_gamma_el.clone() }));
        mutations.push(En1998Mutation::ChangeSiloHeightM(change_silo_height_m::ChangeSiloHeightM { new_silo_height_m: snapshot.silo_height_m.clone() }));
        mutations.push(En1998Mutation::ChangeSiloRadiusM(change_silo_radius_m::ChangeSiloRadiusM { new_silo_radius_m: snapshot.silo_radius_m.clone() }));
        mutations.push(En1998Mutation::ChangeSiloNRdKn(change_silo_n_rd_kn::ChangeSiloNRdKn { new_silo_n_rd_kn: snapshot.silo_n_rd_kn.clone() }));
        mutations.push(En1998Mutation::ChangeSiloVEdKn(change_silo_v_ed_kn::ChangeSiloVEdKn { new_silo_v_ed_kn: snapshot.silo_v_ed_kn.clone() }));
        mutations.push(En1998Mutation::ChangeSiloVRdKn(change_silo_v_rd_kn::ChangeSiloVRdKn { new_silo_v_rd_kn: snapshot.silo_v_rd_kn.clone() }));
        mutations.push(En1998Mutation::ChangeSiloQNominal(change_silo_q_nominal::ChangeSiloQNominal { new_silo_q_nominal: snapshot.silo_q_nominal.clone() }));
        mutations.push(En1998Mutation::ChangeTankHeightM(change_tank_height_m::ChangeTankHeightM { new_tank_height_m: snapshot.tank_height_m.clone() }));
        mutations.push(En1998Mutation::ChangeTankRadiusM(change_tank_radius_m::ChangeTankRadiusM { new_tank_radius_m: snapshot.tank_radius_m.clone() }));
        mutations.push(En1998Mutation::ChangeTankMassT(change_tank_mass_t::ChangeTankMassT { new_tank_mass_t: snapshot.tank_mass_t.clone() }));
        mutations.push(En1998Mutation::ChangeTankVRdKn(change_tank_v_rd_kn::ChangeTankVRdKn { new_tank_v_rd_kn: snapshot.tank_v_rd_kn.clone() }));
        mutations.push(En1998Mutation::ChangeTowerMEdKnm(change_tower_m_ed_knm::ChangeTowerMEdKnm { new_tower_m_ed_knm: snapshot.tower_m_ed_knm.clone() }));
        mutations.push(En1998Mutation::ChangeTowerMRdKnm(change_tower_m_rd_knm::ChangeTowerMRdKnm { new_tower_m_rd_knm: snapshot.tower_m_rd_knm.clone() }));
        mutations.push(En1998Mutation::ChangeTowerIsChimney(change_tower_is_chimney::ChangeTowerIsChimney { new_tower_is_chimney: snapshot.tower_is_chimney.clone() }));
        mutations.push(En1998Mutation::ChangeTowerQNominal(change_tower_q_nominal::ChangeTowerQNominal { new_tower_q_nominal: snapshot.tower_q_nominal.clone() }));
        mutations.push(En1998Mutation::ChangeTowerMassT(change_tower_mass_t::ChangeTowerMassT { new_tower_mass_t: snapshot.tower_mass_t.clone() }));
        mutations.push(En1998Mutation::ChangeFoundationAreaM2(change_foundation_area_m2::ChangeFoundationAreaM2 { new_foundation_area_m2: snapshot.foundation_area_m2.clone() }));
        mutations.push(En1998Mutation::ChangeFoundationPRdKpa(change_foundation_p_rd_kpa::ChangeFoundationPRdKpa { new_foundation_p_rd_kpa: snapshot.foundation_p_rd_kpa.clone() }));
        mutations.push(En1998Mutation::ChangeFoundationHEdKn(change_foundation_h_ed_kn::ChangeFoundationHEdKn { new_foundation_h_ed_kn: snapshot.foundation_h_ed_kn.clone() }));
        mutations.push(En1998Mutation::ChangeFoundationHRdKn(change_foundation_h_rd_kn::ChangeFoundationHRdKn { new_foundation_h_rd_kn: snapshot.foundation_h_rd_kn.clone() }));
        mutations.push(En1998Mutation::ChangeKFoundation(change_k_foundation::ChangeKFoundation { new_k_foundation: snapshot.k_foundation.clone() }));
        mutations.push(En1998Mutation::ChangeKSoil(change_k_soil::ChangeKSoil { new_k_soil: snapshot.k_soil.clone() }));
        mutations.push(En1998Mutation::ChangeWallHeightM(change_wall_height_m::ChangeWallHeightM { new_wall_height_m: snapshot.wall_height_m.clone() }));
        mutations.push(En1998Mutation::ChangeWallPhiDeg(change_wall_phi_deg::ChangeWallPhiDeg { new_wall_phi_deg: snapshot.wall_phi_deg.clone() }));
        mutations.push(En1998Mutation::ChangeWallSoilGammaKnM3(change_wall_soil_gamma_kn_m3::ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3: snapshot.wall_soil_gamma_kn_m3.clone() }));
        mutations.push(En1998Mutation::ChangeWallR(change_wall_r::ChangeWallR { new_wall_r: snapshot.wall_r.clone() }));
        mutations.push(En1998Mutation::ChangeWallHRdKn(change_wall_h_rd_kn::ChangeWallHRdKn { new_wall_h_rd_kn: snapshot.wall_h_rd_kn.clone() }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;
    use protocol::SemanticMutation;

    /// ⚖️ One value per `En1998Mutation` variant — the closed set the semantics/round-trip
    /// tests iterate.
    fn every_mutation() -> Vec<En1998Mutation> {
        vec![
            En1998Mutation::ChangeSeismicZone(change_seismic_zone::ChangeSeismicZone { new_seismic_zone: 3 }),
            En1998Mutation::ChangeGroundType(change_ground_type::ChangeGroundType { new_ground_type: "c".to_string() }),
            En1998Mutation::ChangeImportanceClass(change_importance_class::ChangeImportanceClass { new_importance_class: "cc3".to_string() }),
            En1998Mutation::ChangeStructuralSystem(change_structural_system::ChangeStructuralSystem { new_structural_system: "wall_dcm".to_string() }),
            En1998Mutation::ChangeT1S(change_t1_s::ChangeT1S { new_t1_s: 0.35 }),
            En1998Mutation::ChangeMassT(change_mass_t::ChangeMassT { new_mass_t: 550.0 }),
            En1998Mutation::ChangeVRdKn(change_v_rd_kn::ChangeVRdKn { new_v_rd_kn: 850.0 }),
            En1998Mutation::ChangeDriftMm(change_drift_mm::ChangeDriftMm { new_drift_mm: 22.0 }),
            En1998Mutation::ChangeHeightM(change_height_m::ChangeHeightM { new_height_m: 14.0 }),
            En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::ChangeMultipleResistingSystems { new_multiple_resisting_systems: false }),
            En1998Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: "en".to_string() }),
            En1998Mutation::ChangeEnAGr(change_en_a_gr::ChangeEnAGr { new_en_a_gr: 0.2 }),
            En1998Mutation::ChangeEnGroundType(change_en_ground_type::ChangeEnGroundType { new_en_ground_type: "c".to_string() }),
            En1998Mutation::ChangeEnSpectrumType(change_en_spectrum_type::ChangeEnSpectrumType { new_en_spectrum_type: "type2".to_string() }),
            En1998Mutation::ChangePeriodRatio(change_period_ratio::ChangePeriodRatio { new_period_ratio: 1.8 }),
            En1998Mutation::ChangeBridgeVRdKn(change_bridge_v_rd_kn::ChangeBridgeVRdKn { new_bridge_v_rd_kn: 650.0 }),
            En1998Mutation::ChangeBearingDEdMm(change_bearing_d_ed_mm::ChangeBearingDEdMm { new_bearing_d_ed_mm: 130.0 }),
            En1998Mutation::ChangeBearingDRdMm(change_bearing_d_rd_mm::ChangeBearingDRdMm { new_bearing_d_rd_mm: 260.0 }),
            En1998Mutation::ChangeRetrofitKnowledgeLevel(change_retrofit_knowledge_level::ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level: "kl3".to_string() }),
            En1998Mutation::ChangeRetrofitLimitState(change_retrofit_limit_state::ChangeRetrofitLimitState { new_retrofit_limit_state: "near_collapse".to_string() }),
            En1998Mutation::ChangeRetrofitEDKn(change_retrofit_e_d_kn::ChangeRetrofitEDKn { new_retrofit_e_d_kn: 270.0 }),
            En1998Mutation::ChangeRetrofitRKKn(change_retrofit_r_k_kn::ChangeRetrofitRKKn { new_retrofit_r_k_kn: 420.0 }),
            En1998Mutation::ChangeRetrofitGammaEl(change_retrofit_gamma_el::ChangeRetrofitGammaEl { new_retrofit_gamma_el: 1.15 }),
            En1998Mutation::ChangeSiloHeightM(change_silo_height_m::ChangeSiloHeightM { new_silo_height_m: 11.0 }),
            En1998Mutation::ChangeSiloRadiusM(change_silo_radius_m::ChangeSiloRadiusM { new_silo_radius_m: 5.5 }),
            En1998Mutation::ChangeSiloNRdKn(change_silo_n_rd_kn::ChangeSiloNRdKn { new_silo_n_rd_kn: 520.0 }),
            En1998Mutation::ChangeSiloVEdKn(change_silo_v_ed_kn::ChangeSiloVEdKn { new_silo_v_ed_kn: 190.0 }),
            En1998Mutation::ChangeSiloVRdKn(change_silo_v_rd_kn::ChangeSiloVRdKn { new_silo_v_rd_kn: 320.0 }),
            En1998Mutation::ChangeSiloQNominal(change_silo_q_nominal::ChangeSiloQNominal { new_silo_q_nominal: 2.2 }),
            En1998Mutation::ChangeTankHeightM(change_tank_height_m::ChangeTankHeightM { new_tank_height_m: 9.0 }),
            En1998Mutation::ChangeTankRadiusM(change_tank_radius_m::ChangeTankRadiusM { new_tank_radius_m: 4.5 }),
            En1998Mutation::ChangeTankMassT(change_tank_mass_t::ChangeTankMassT { new_tank_mass_t: 320.0 }),
            En1998Mutation::ChangeTankVRdKn(change_tank_v_rd_kn::ChangeTankVRdKn { new_tank_v_rd_kn: 420.0 }),
            En1998Mutation::ChangeTowerMEdKnm(change_tower_m_ed_knm::ChangeTowerMEdKnm { new_tower_m_ed_knm: 1300.0 }),
            En1998Mutation::ChangeTowerMRdKnm(change_tower_m_rd_knm::ChangeTowerMRdKnm { new_tower_m_rd_knm: 2600.0 }),
            En1998Mutation::ChangeTowerIsChimney(change_tower_is_chimney::ChangeTowerIsChimney { new_tower_is_chimney: false }),
            En1998Mutation::ChangeTowerQNominal(change_tower_q_nominal::ChangeTowerQNominal { new_tower_q_nominal: 2.8 }),
            En1998Mutation::ChangeTowerMassT(change_tower_mass_t::ChangeTowerMassT { new_tower_mass_t: 85.0 }),
            En1998Mutation::ChangeFoundationAreaM2(change_foundation_area_m2::ChangeFoundationAreaM2 { new_foundation_area_m2: 110.0 }),
            En1998Mutation::ChangeFoundationPRdKpa(change_foundation_p_rd_kpa::ChangeFoundationPRdKpa { new_foundation_p_rd_kpa: 520.0 }),
            En1998Mutation::ChangeFoundationHEdKn(change_foundation_h_ed_kn::ChangeFoundationHEdKn { new_foundation_h_ed_kn: 160.0 }),
            En1998Mutation::ChangeFoundationHRdKn(change_foundation_h_rd_kn::ChangeFoundationHRdKn { new_foundation_h_rd_kn: 420.0 }),
            En1998Mutation::ChangeKFoundation(change_k_foundation::ChangeKFoundation { new_k_foundation: 520_000.0 }),
            En1998Mutation::ChangeKSoil(change_k_soil::ChangeKSoil { new_k_soil: 210_000.0 }),
            En1998Mutation::ChangeWallHeightM(change_wall_height_m::ChangeWallHeightM { new_wall_height_m: 4.5 }),
            En1998Mutation::ChangeWallPhiDeg(change_wall_phi_deg::ChangeWallPhiDeg { new_wall_phi_deg: 32.0 }),
            En1998Mutation::ChangeWallSoilGammaKnM3(change_wall_soil_gamma_kn_m3::ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3: 19.0 }),
            En1998Mutation::ChangeWallR(change_wall_r::ChangeWallR { new_wall_r: 1.4 }),
            En1998Mutation::ChangeWallHRdKn(change_wall_h_rd_kn::ChangeWallHRdKn { new_wall_h_rd_kn: 160.0 }),
        ]
    }

    fn round_trip(base: &En1998Snapshot, mutation: &En1998Mutation) -> En1998Snapshot {
        let forward = vcs::apply_mutation(base, mutation).expect("valid mutation").0;
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[semio_framework_async_macros::async_test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<En1998Mutation as protocol::SemanticMutation<En1998Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[semio_framework_async_macros::async_test]
    fn every_variant_round_trips_via_inverse() {
        let base = En1998Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    #[semio_framework_async_macros::async_test]
    fn from_snapshot_round_trips_via_full_document_replacement() {
        let base = En1998Snapshot::default();
        let mut target = En1998Snapshot::default();
        let _ = &mut target;
        let mut projected = base.clone();
        for mutation in En1998Mutation::from_snapshot(&target) {
            projected = vcs::apply_mutation(&projected, &mutation).expect("snapshot mutation applies").0;
        }
        assert_eq!(projected, target, "from_snapshot must reconstruct every persistent field");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
    /// variants.
    #[semio_framework_async_macros::async_test]
    fn change_seismic_zone_satisfies_the_inverse_and_absorb_laws() {
        let base = En1998Snapshot::default();
        let mutation = En1998Mutation::ChangeSeismicZone(change_seismic_zone::ChangeSeismicZone { new_seismic_zone: 3 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1998Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: "en".to_string() }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    fn change_multiple_resisting_systems_satisfies_the_inverse_and_absorb_laws() {
        let base = En1998Snapshot::default();
        let mutation = En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::ChangeMultipleResistingSystems { new_multiple_resisting_systems: false });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1998Mutation::ChangeT1S(change_t1_s::ChangeT1S { new_t1_s: 0.35 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    fn change_ground_type_satisfies_the_inverse_and_absorb_laws() {
        let base = En1998Snapshot::default();
        let mutation = En1998Mutation::ChangeGroundType(change_ground_type::ChangeGroundType { new_ground_type: "c".to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1998Mutation::ChangeMassT(change_mass_t::ChangeMassT { new_mass_t: 550.0 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
/// 🧪️ The 49 handcrafted mutation fixtures (contract D1, ticket
/// `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`) are self-wired from here rather than from
/// `📦️glue.rs`: that file is shared by all fifteen norm artifacts and is being edited concurrently,
/// so each artifact mounts its own `🧪️tests` leaves. `#[path = "."]` re-bases the nested `#[path]`
/// attributes onto this file's own directory, which is this `🧬️mutations/` tree.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🗻change-annex/🧪️tests/switches-annex-to-en/🦀️component.rs"]
    mod tests_change_annex_switches;
    #[path = "🏝️change-bearing-d-ed-mm/🧪️tests/raises-bearing-d-ed-mm-to-165-5/🦀️component.rs"]
    mod tests_change_bearing_d_ed_mm_raises;
    #[path = "🏞️change-bearing-d-rd-mm/🧪️tests/raises-bearing-d-rd-mm-to-312-5/🦀️component.rs"]
    mod tests_change_bearing_d_rd_mm_raises;
    #[path = "🏜️change-bridge-v-rd-kn/🧪️tests/raises-bridge-v-rd-kn-to-725-0/🦀️component.rs"]
    mod tests_change_bridge_v_rd_kn_raises;
    #[path = "🌎️change-drift-mm/🧪️tests/raises-drift-mm-to-33-5/🦀️component.rs"]
    mod tests_change_drift_mm_raises;
    #[path = "🏔️change-en-a-gr/🧪️tests/raises-en-a-gr-to-0-25/🦀️component.rs"]
    mod tests_change_en_a_gr_raises;
    #[path = "⛰️change-en-ground-type/🧪️tests/switches-en-ground-type-to-e/🦀️component.rs"]
    mod tests_change_en_ground_type_switches;
    #[path = "🏕️change-en-spectrum-type/🧪️tests/switches-en-spectrum-type-to-type2/🦀️component.rs"]
    mod tests_change_en_spectrum_type_switches;
    #[path = "🦇change-foundation-area-m2/🧪️tests/raises-foundation-area-m2-to-144-0/🦀️component.rs"]
    mod tests_change_foundation_area_m2_raises;
    #[path = "🐴change-foundation-h-ed-kn/🧪️tests/raises-foundation-h-ed-kn-to-212-5/🦀️component.rs"]
    mod tests_change_foundation_h_ed_kn_raises;
    #[path = "🐎change-foundation-h-rd-kn/🧪️tests/raises-foundation-h-rd-kn-to-475-0/🦀️component.rs"]
    mod tests_change_foundation_h_rd_kn_raises;
    #[path = "🦉change-foundation-p-rd-kpa/🧪️tests/raises-foundation-p-rd-kpa-to-625-0/🦀️component.rs"]
    mod tests_change_foundation_p_rd_kpa_raises;
    #[path = "🍄change-ground-type/🧪️tests/switches-ground-type-to-c/🦀️component.rs"]
    mod tests_change_ground_type_switches;
    #[path = "🌏️change-height-m/🧪️tests/raises-height-m-to-18-75/🦀️component.rs"]
    mod tests_change_height_m_raises;
    #[path = "🌰change-importance-class/🧪️tests/switches-importance-class-to-cc3/🦀️component.rs"]
    mod tests_change_importance_class_switches;
    #[path = "🦄change-k-foundation/🧪️tests/raises-k-foundation-to-640000-0/🦀️component.rs"]
    mod tests_change_k_foundation_raises;
    #[path = "🐑change-k-soil/🧪️tests/raises-k-soil-to-262500-0/🦀️component.rs"]
    mod tests_change_k_soil_raises;
    #[path = "🪨change-mass-t/🧪️tests/raises-mass-t-to-812-5/🦀️component.rs"]
    mod tests_change_mass_t_raises;
    #[path = "🌐change-multiple-resisting-systems/🧪️tests/turns-multiple-resisting-systems-off/🦀️component.rs"]
    mod tests_change_multiple_resisting_systems_turns_off;
    #[path = "🏖️change-period-ratio/🧪️tests/raises-period-ratio-to-3-5/🦀️component.rs"]
    mod tests_change_period_ratio_raises;
    #[path = "🐝change-retrofit-ed-kn/🧪️tests/raises-retrofit-e-d-kn-to-337-5/🦀️component.rs"]
    mod tests_change_retrofit_e_d_kn_raises;
    #[path = "🦋change-retrofit-gamma-el/🧪️tests/raises-retrofit-gamma-el-to-1-25/🦀️component.rs"]
    mod tests_change_retrofit_gamma_el_raises;
    #[path = "🏟️change-retrofit-knowledge-level/🧪️tests/switches-retrofit-knowledge-level-to-kl3/🦀️component.rs"]
    mod tests_change_retrofit_knowledge_level_switches;
    #[path = "🪵change-retrofit-limit-state/🧪️tests/switches-retrofit-limit-state-to-near-collapse/🦀️component.rs"]
    mod tests_change_retrofit_limit_state_switches;
    #[path = "🐞change-retrofit-rk-kn/🧪️tests/raises-retrofit-r-k-kn-to-512-5/🦀️component.rs"]
    mod tests_change_retrofit_r_k_kn_raises;
    #[path = "🌼change-seismic-zone/🧪️tests/raises-seismic-zone-to-4/🦀️component.rs"]
    mod tests_change_seismic_zone_raises;
    #[path = "🐌change-silo-height-m/🧪️tests/raises-silo-height-m-to-14-5/🦀️component.rs"]
    mod tests_change_silo_height_m_raises;
    #[path = "🐬change-silo-n-rd-kn/🧪️tests/raises-silo-n-rd-kn-to-640-0/🦀️component.rs"]
    mod tests_change_silo_n_rd_kn_raises;
    #[path = "🦭change-silo-q-nominal/🧪️tests/raises-silo-q-nominal-to-2-75/🦀️component.rs"]
    mod tests_change_silo_q_nominal_raises;
    #[path = "🐢change-silo-radius-m/🧪️tests/raises-silo-radius-m-to-6-25/🦀️component.rs"]
    mod tests_change_silo_radius_m_raises;
    #[path = "🐳change-silo-v-ed-kn/🧪️tests/raises-silo-v-ed-kn-to-225-5/🦀️component.rs"]
    mod tests_change_silo_v_ed_kn_raises;
    #[path = "🦈change-silo-v-rd-kn/🧪️tests/raises-silo-v-rd-kn-to-412-5/🦀️component.rs"]
    mod tests_change_silo_v_rd_kn_raises;
    #[path = "🌊change-structural-system/🧪️tests/switches-structural-system-to-wall-dcm/🦀️component.rs"]
    mod tests_change_structural_system_switches;
    #[path = "🐚change-t1-s/🧪️tests/raises-t1-s-to-0-75/🦀️component.rs"]
    mod tests_change_t1_s_raises;
    #[path = "🐊change-tank-height-m/🧪️tests/raises-tank-height-m-to-11-5/🦀️component.rs"]
    mod tests_change_tank_height_m_raises;
    #[path = "🐍change-tank-mass-t/🧪️tests/raises-tank-mass-t-to-425-0/🦀️component.rs"]
    mod tests_change_tank_mass_t_raises;
    #[path = "🦎change-tank-radius-m/🧪️tests/raises-tank-radius-m-to-5-75/🦀️component.rs"]
    mod tests_change_tank_radius_m_raises;
    #[path = "🦂change-tank-v-rd-kn/🧪️tests/raises-tank-v-rd-kn-to-537-5/🦀️component.rs"]
    mod tests_change_tank_v_rd_kn_raises;
    #[path = "🕷️change-tower-is-chimney/🧪️tests/turns-tower-is-chimney-off/🦀️component.rs"]
    mod tests_change_tower_is_chimney_turns_off;
    #[path = "🦟change-tower-m-ed-knm/🧪️tests/raises-tower-m-ed-knm-to-1562-5/🦀️component.rs"]
    mod tests_change_tower_m_ed_knm_raises;
    #[path = "🦗change-tower-m-rd-knm/🧪️tests/raises-tower-m-rd-knm-to-2812-5/🦀️component.rs"]
    mod tests_change_tower_m_rd_knm_raises;
    #[path = "🦔change-tower-mass-t/🧪️tests/raises-tower-mass-t-to-112-5/🦀️component.rs"]
    mod tests_change_tower_mass_t_raises;
    #[path = "🐜change-tower-q-nominal/🧪️tests/raises-tower-q-nominal-to-3-25/🦀️component.rs"]
    mod tests_change_tower_q_nominal_raises;
    #[path = "🌍️change-v-rd-kn/🧪️tests/raises-v-rd-kn-to-925-0/🦀️component.rs"]
    mod tests_change_v_rd_kn_raises;
    #[path = "🦌change-wall-h-rd-kn/🧪️tests/raises-wall-h-rd-kn-to-187-5/🦀️component.rs"]
    mod tests_change_wall_h_rd_kn_raises;
    #[path = "🐐change-wall-height-m/🧪️tests/raises-wall-height-m-to-5-5/🦀️component.rs"]
    mod tests_change_wall_height_m_raises;
    #[path = "🐮change-wall-phi-deg/🧪️tests/raises-wall-phi-deg-to-37-5/🦀️component.rs"]
    mod tests_change_wall_phi_deg_raises;
    #[path = "🐗change-wall-r/🧪️tests/raises-wall-r-to-2-25/🦀️component.rs"]
    mod tests_change_wall_r_raises;
    #[path = "🐷change-wall-soil-gamma-kn-m3/🧪️tests/raises-wall-soil-gamma-kn-m3-to-20-5/🦀️component.rs"]
    mod tests_change_wall_soil_gamma_kn_m3_raises;
}
//#endregion 🧪️FixtureTests


//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json`
/// specification vectors carry — into a real [`En1998Mutation`]. The generated test host of
/// `../../../../../🧪️tests/mutate-en1998-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1998_mutation_json(text: &str) -> Result<En1998Mutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_en1998_mutation(base: &En1998Snapshot, mutation: &En1998Mutation) -> Result<(En1998Snapshot, Vec<String>), String> {
    let raised = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `mutate-en1998-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_en1998_mutation(mutation: &En1998Mutation, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
    /// committed `en1998-1-any` catalog. The framework never parses Rust, so this is the only thing
    /// standing between a renamed variant and a completeness gate that silently measures the wrong
    /// set.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <En1998Mutation as protocol::SemanticMutation<En1998Snapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared En1998Mutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsCatalog
