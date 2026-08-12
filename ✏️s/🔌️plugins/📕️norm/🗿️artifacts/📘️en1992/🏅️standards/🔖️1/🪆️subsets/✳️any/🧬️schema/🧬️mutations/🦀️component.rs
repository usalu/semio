//! 🧬️ En1992 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `En1992Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (thirty-five persistent scalar fields feeding the bending/shear,
//! fire, bridge-fatigue, liquid-retaining crack-width and anchor checks) — no id-keyed
//! collections, no name/identity field to `rename`. Every field becomes its own `change-<field>`
//! mutation per the rule's "change-<field> per remaining scalar" clause; none qualify for the
//! `update-<facet>` grouping exception (each check input is independently measured/entered in the
//! host UI, never validated as an atomic multi-field bundle). `SetSnapshot` — the pre-migration
//! whole-document replace — is gone: banned outright per `📓️taxonomy.md`/`📓️derivation-rules.md`
//! rule 6, with NO replacement mutation; file-open/import/load-example now goes through
//! `store::ArtifactStore::reset`, entirely outside this enum.
//!
//! `📄set-snapshot` keeps its pre-migration directory name — `📦️glue.rs` path-includes that exact
//! triad outside this facet's writable boundary, so it was repurposed in place (same path,
//! rewritten `🦠️mutation`/`🔺️diff`/`↩️inverse` content) to hold `ChangeAnnex` instead of being
//! renamed; see this ticket's wave2 report `sharedFileRequests` for the rename once a later pass
//! can touch `📦️glue.rs`. The other thirty-four triads have no pre-migration slot and are
//! self-wired directly below via nested `#[path = "."] pub mod <name> { ... }` blocks.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️NewLeaves
#[path = "."]
pub mod change_m_ed_knm {
    #[path = "🔧change-m-ed-knm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-m-ed-knm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-m-ed-knm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_v_ed_kn {
    #[path = "🔧change-v-ed-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-v-ed-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-v-ed-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_f_ck {
    #[path = "🔧change-f-ck/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-f-ck/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-f-ck/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_b_mm {
    #[path = "🔧change-b-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-b-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-b-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_d_mm {
    #[path = "🔧change-d-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-d-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-d-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_a_s_mm2 {
    #[path = "🔧change-as-mm2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-as-mm2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-as-mm2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_f_yk {
    #[path = "🔧change-f-yk/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-f-yk/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-f-yk/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_rho_l {
    #[path = "🔧change-rho-l/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-rho-l/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-rho-l/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_n_ed_kn {
    #[path = "🔧change-n-ed-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-n-ed-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-n-ed-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_p_kn {
    #[path = "🔧change-p-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-p-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-p-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_a_c_mm2 {
    #[path = "🔧change-ac-mm2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-ac-mm2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-ac-mm2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_use_fem {
    #[path = "🔧change-use-fem/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-use-fem/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-use-fem/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_span_m {
    #[path = "🔧change-span-m/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-span-m/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-span-m/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_udl_kn_m {
    #[path = "🔧change-udl-kn-m/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-udl-kn-m/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-udl-kn-m/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fire_rating {
    #[path = "🔧change-fire-rating/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-fire-rating/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-fire-rating/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_provided_axis_distance_mm {
    #[path = "🔧change-provided-axis-distance-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-provided-axis-distance-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-provided-axis-distance-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_bridge_sigma_c_mpa {
    #[path = "🔧change-bridge-sigma-c-mpa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-bridge-sigma-c-mpa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-bridge-sigma-c-mpa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_bridge_delta_sigma_s_mpa {
    #[path = "🔧change-bridge-delta-sigma-s-mpa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-bridge-delta-sigma-s-mpa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-bridge-delta-sigma-s-mpa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_tightness_class {
    #[path = "🔧change-tightness-class/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-tightness-class/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-tightness-class/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_hd_over_h {
    #[path = "🔧change-hd-over-h/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-hd-over-h/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-hd-over-h/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_liquid_sigma_s_mpa {
    #[path = "🔧change-liquid-sigma-s-mpa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-liquid-sigma-s-mpa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-liquid-sigma-s-mpa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_liquid_rho_p_eff {
    #[path = "🔧change-liquid-rho-p-eff/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-liquid-rho-p-eff/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-liquid-rho-p-eff/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_liquid_f_ct_eff_mpa {
    #[path = "🔧change-liquid-f-ct-eff-mpa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-liquid-f-ct-eff-mpa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-liquid-f-ct-eff-mpa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_liquid_e_s_mpa {
    #[path = "🔧change-liquid-es-mpa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-liquid-es-mpa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-liquid-es-mpa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_liquid_s_r_max_mm {
    #[path = "🔧change-liquid-sr-max-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-liquid-sr-max-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-liquid-sr-max-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_h_ef_mm {
    #[path = "🔧change-anchor-h-ef-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-h-ef-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-h-ef-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_cracked {
    #[path = "🔧change-anchor-cracked/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-cracked/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-cracked/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_f_uk_mpa {
    #[path = "🔧change-anchor-f-uk-mpa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-f-uk-mpa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-f-uk-mpa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_f_yk_mpa {
    #[path = "🔧change-anchor-f-yk-mpa/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-f-yk-mpa/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-f-yk-mpa/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_a_s_mm2 {
    #[path = "🔧change-anchor-as-mm2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-as-mm2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-as-mm2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_d_mm {
    #[path = "🔧change-anchor-d-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-d-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-d-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_c1_mm {
    #[path = "🔧change-anchor-c1-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-c1-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-c1-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_n_ed_kn {
    #[path = "🔧change-anchor-n-ed-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-n-ed-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-n-ed-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_anchor_v_ed_kn {
    #[path = "🔧change-anchor-v-ed-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-anchor-v-ed-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-anchor-v-ed-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
//#endregion 🔖️NewLeaves

//#region 🔖️RepurposedLeaves
// 🌱️ `set_snapshot` is declared by `📦️glue.rs` as a sibling of `component` (this file) under
// `pub mod mutations { ... }` — brought into this file's own scope the same way this ticket's
// din16798/vdi3805 precedents reach their own repurposed `set_snapshot` sibling.
use super::set_snapshot;
//#endregion 🔖️RepurposedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the En1992 document, derived per
/// `📓️derivation-rules.md` from `En1992Snapshot`'s flat scalar shape. `impl protocol::Mutation`/
/// `SemanticMutation` below are generated by `#[derive(dsl::Mutations)]` — never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = En1992Snapshot, diff = En1992Diff, schema = "s.norm.en1992")]
pub enum En1992Mutation {
    ChangeAnnex(set_snapshot::mutation::ChangeAnnex),
    ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm),
    ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn),
    ChangeFCk(change_f_ck::mutation::ChangeFCk),
    ChangeBMm(change_b_mm::mutation::ChangeBMm),
    ChangeDMm(change_d_mm::mutation::ChangeDMm),
    ChangeASMm2(change_a_s_mm2::mutation::ChangeASMm2),
    ChangeFYk(change_f_yk::mutation::ChangeFYk),
    ChangeRhoL(change_rho_l::mutation::ChangeRhoL),
    ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn),
    ChangePKn(change_p_kn::mutation::ChangePKn),
    ChangeACMm2(change_a_c_mm2::mutation::ChangeACMm2),
    ChangeUseFem(change_use_fem::mutation::ChangeUseFem),
    ChangeSpanM(change_span_m::mutation::ChangeSpanM),
    ChangeUdlKnM(change_udl_kn_m::mutation::ChangeUdlKnM),
    ChangeFireRating(change_fire_rating::mutation::ChangeFireRating),
    ChangeProvidedAxisDistanceMm(change_provided_axis_distance_mm::mutation::ChangeProvidedAxisDistanceMm),
    ChangeBridgeSigmaCMpa(change_bridge_sigma_c_mpa::mutation::ChangeBridgeSigmaCMpa),
    ChangeBridgeDeltaSigmaSMpa(change_bridge_delta_sigma_s_mpa::mutation::ChangeBridgeDeltaSigmaSMpa),
    ChangeTightnessClass(change_tightness_class::mutation::ChangeTightnessClass),
    ChangeHdOverH(change_hd_over_h::mutation::ChangeHdOverH),
    ChangeLiquidSigmaSMpa(change_liquid_sigma_s_mpa::mutation::ChangeLiquidSigmaSMpa),
    ChangeLiquidRhoPEff(change_liquid_rho_p_eff::mutation::ChangeLiquidRhoPEff),
    ChangeLiquidFCtEffMpa(change_liquid_f_ct_eff_mpa::mutation::ChangeLiquidFCtEffMpa),
    ChangeLiquidESMpa(change_liquid_e_s_mpa::mutation::ChangeLiquidESMpa),
    ChangeLiquidSRMaxMm(change_liquid_s_r_max_mm::mutation::ChangeLiquidSRMaxMm),
    ChangeAnchorHEfMm(change_anchor_h_ef_mm::mutation::ChangeAnchorHEfMm),
    ChangeAnchorCracked(change_anchor_cracked::mutation::ChangeAnchorCracked),
    ChangeAnchorFUkMpa(change_anchor_f_uk_mpa::mutation::ChangeAnchorFUkMpa),
    ChangeAnchorFYkMpa(change_anchor_f_yk_mpa::mutation::ChangeAnchorFYkMpa),
    ChangeAnchorASMm2(change_anchor_a_s_mm2::mutation::ChangeAnchorASMm2),
    ChangeAnchorDMm(change_anchor_d_mm::mutation::ChangeAnchorDMm),
    ChangeAnchorC1Mm(change_anchor_c1_mm::mutation::ChangeAnchorC1Mm),
    ChangeAnchorNEdKn(change_anchor_n_ed_kn::mutation::ChangeAnchorNEdKn),
    ChangeAnchorVEdKn(change_anchor_v_ed_kn::mutation::ChangeAnchorVEdKn),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    /// ⚖️ One value per `En1992Mutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring this ticket's din16798/vdi3805 precedents' own `every_mutation()` fixture.
    fn every_mutation() -> Vec<En1992Mutation> {
        vec![
        En1992Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1992Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 150.0 }),
        En1992Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 95.0 }),
        En1992Mutation::ChangeFCk(change_f_ck::mutation::ChangeFCk { new_f_ck: 35.0 }),
        En1992Mutation::ChangeBMm(change_b_mm::mutation::ChangeBMm { new_b_mm: 350.0 }),
        En1992Mutation::ChangeDMm(change_d_mm::mutation::ChangeDMm { new_d_mm: 500.0 }),
        En1992Mutation::ChangeASMm2(change_a_s_mm2::mutation::ChangeASMm2 { new_a_s_mm2: 1400.0 }),
        En1992Mutation::ChangeFYk(change_f_yk::mutation::ChangeFYk { new_f_yk: 550.0 }),
        En1992Mutation::ChangeRhoL(change_rho_l::mutation::ChangeRhoL { new_rho_l: 0.015 }),
        En1992Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 25.0 }),
        En1992Mutation::ChangePKn(change_p_kn::mutation::ChangePKn { new_p_kn: 50.0 }),
        En1992Mutation::ChangeACMm2(change_a_c_mm2::mutation::ChangeACMm2 { new_a_c_mm2: 150000.0 }),
        En1992Mutation::ChangeUseFem(change_use_fem::mutation::ChangeUseFem { new_use_fem: true }),
        En1992Mutation::ChangeSpanM(change_span_m::mutation::ChangeSpanM { new_span_m: 7.5 }),
        En1992Mutation::ChangeUdlKnM(change_udl_kn_m::mutation::ChangeUdlKnM { new_udl_kn_m: 24.0 }),
        En1992Mutation::ChangeFireRating(change_fire_rating::mutation::ChangeFireRating { new_fire_rating: crate::artifacts::en1992::part_1_2::FireRating::R90 }),
        En1992Mutation::ChangeProvidedAxisDistanceMm(change_provided_axis_distance_mm::mutation::ChangeProvidedAxisDistanceMm { new_provided_axis_distance_mm: 40.0 }),
        En1992Mutation::ChangeBridgeSigmaCMpa(change_bridge_sigma_c_mpa::mutation::ChangeBridgeSigmaCMpa { new_bridge_sigma_c_mpa: 14.0 }),
        En1992Mutation::ChangeBridgeDeltaSigmaSMpa(change_bridge_delta_sigma_s_mpa::mutation::ChangeBridgeDeltaSigmaSMpa { new_bridge_delta_sigma_s_mpa: 120.0 }),
        En1992Mutation::ChangeTightnessClass(change_tightness_class::mutation::ChangeTightnessClass { new_tightness_class: crate::artifacts::en1992::part_3::TightnessClass::Tc2 }),
        En1992Mutation::ChangeHdOverH(change_hd_over_h::mutation::ChangeHdOverH { new_hd_over_h: 12.0 }),
        En1992Mutation::ChangeLiquidSigmaSMpa(change_liquid_sigma_s_mpa::mutation::ChangeLiquidSigmaSMpa { new_liquid_sigma_s_mpa: 220.0 }),
        En1992Mutation::ChangeLiquidRhoPEff(change_liquid_rho_p_eff::mutation::ChangeLiquidRhoPEff { new_liquid_rho_p_eff: 0.012 }),
        En1992Mutation::ChangeLiquidFCtEffMpa(change_liquid_f_ct_eff_mpa::mutation::ChangeLiquidFCtEffMpa { new_liquid_f_ct_eff_mpa: 3.1 }),
        En1992Mutation::ChangeLiquidESMpa(change_liquid_e_s_mpa::mutation::ChangeLiquidESMpa { new_liquid_e_s_mpa: 205000.0 }),
        En1992Mutation::ChangeLiquidSRMaxMm(change_liquid_s_r_max_mm::mutation::ChangeLiquidSRMaxMm { new_liquid_s_r_max_mm: 275.0 }),
        En1992Mutation::ChangeAnchorHEfMm(change_anchor_h_ef_mm::mutation::ChangeAnchorHEfMm { new_anchor_h_ef_mm: 90.0 }),
        En1992Mutation::ChangeAnchorCracked(change_anchor_cracked::mutation::ChangeAnchorCracked { new_anchor_cracked: true }),
        En1992Mutation::ChangeAnchorFUkMpa(change_anchor_f_uk_mpa::mutation::ChangeAnchorFUkMpa { new_anchor_f_uk_mpa: 850.0 }),
        En1992Mutation::ChangeAnchorFYkMpa(change_anchor_f_yk_mpa::mutation::ChangeAnchorFYkMpa { new_anchor_f_yk_mpa: 680.0 }),
        En1992Mutation::ChangeAnchorASMm2(change_anchor_a_s_mm2::mutation::ChangeAnchorASMm2 { new_anchor_a_s_mm2: 94.3 }),
        En1992Mutation::ChangeAnchorDMm(change_anchor_d_mm::mutation::ChangeAnchorDMm { new_anchor_d_mm: 14.0 }),
        En1992Mutation::ChangeAnchorC1Mm(change_anchor_c1_mm::mutation::ChangeAnchorC1Mm { new_anchor_c1_mm: 120.0 }),
        En1992Mutation::ChangeAnchorNEdKn(change_anchor_n_ed_kn::mutation::ChangeAnchorNEdKn { new_anchor_n_ed_kn: 15.0 }),
        En1992Mutation::ChangeAnchorVEdKn(change_anchor_v_ed_kn::mutation::ChangeAnchorVEdKn { new_anchor_v_ed_kn: 8.0 }),
        ]
    }

    fn round_trip(base: &En1992Snapshot, mutation: &En1992Mutation) -> En1992Snapshot {
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
        assert_eq!(<En1992Mutation as protocol::SemanticMutation<En1992Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = En1992Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::testkit`), exercised against the three most structurally
    /// distinct variants: the enum-typed scalar (`change-annex`), a typical `f64` scalar
    /// (`change-m-ed-knm`), and a `bool` scalar (`change-use-fem`).
    #[test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = En1992Snapshot::default();
        let mutation = En1992Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1992Mutation::ChangeFireRating(change_fire_rating::mutation::ChangeFireRating { new_fire_rating: crate::artifacts::en1992::part_1_2::FireRating::R90 }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_m_ed_knm_satisfies_the_inverse_and_absorb_laws() {
        let base = En1992Snapshot::default();
        let mutation = En1992Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 150.0 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1992Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 95.0 }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_use_fem_satisfies_the_inverse_and_absorb_laws() {
        let base = En1992Snapshot::default();
        let mutation = En1992Mutation::ChangeUseFem(change_use_fem::mutation::ChangeUseFem { new_use_fem: true });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1992Mutation::ChangeAnchorCracked(change_anchor_cracked::mutation::ChangeAnchorCracked { new_anchor_cracked: true }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
