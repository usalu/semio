//! 🧬️ En1997 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `En1997Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (twenty-two persistent scalar/enum fields describing the EN 1997 geotechnical (shallow-footing, pile) design check's actions, resistances and ground parameters) — no id-keyed
//! collections, no name/identity field to `rename`. Every field becomes its own `change-<field>`
//! mutation per the rule's "change-<field> per remaining scalar" clause; none qualify for the
//! `update-<facet>` grouping exception (each parameter is independently entered on its own input row,
//! never validated as an atomic multi-field bundle). The pre-migration whole-document-replace variant
//! is gone: banned outright per `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement
//! mutation; file-open/import/load-example now goes through `store::ArtifactStore::reset`, entirely
//! outside this enum. The old whole-document-replace macro call is removed with it.
//!
//! All triads are mounted directly as `mutations`-sibling modules in `📦️glue.rs` (this lane's agent
//! owns `📦️glue.rs`, so no self-wiring `#[path = "."]` blocks are needed for the TRIADS).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::change_alpha_s;
use super::change_annex;
use super::change_b_m;
use super::change_c_kpa;
use super::change_d_f_m;
use super::change_design_approach;
use super::change_e_s_mpa;
use super::change_footing_area_m2;
use super::change_gamma_kn_m3;
use super::change_h_ed_kn;
use super::change_n_pile_ed_kn;
use super::change_nu;
use super::change_phi_deg;
use super::change_pile_base_area_m2;
use super::change_pile_d_m;
use super::change_pile_l_m;
use super::change_pile_n_profiles;
use super::change_q_b_kpa;
use super::change_q_s_kpa;
use super::change_settlement_limit_mm;
use super::change_v_ed_kn;
use super::change_z_investigated_m;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1997 document, derived per
/// `📓️derivation-rules.md` from `En1997Snapshot`'s flat scalar/enum shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1997Snapshot, diff = En1997Diff, schema = "norm.en1997")]
pub enum En1997Mutation {
    ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn),
    ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn),
    ChangeFootingAreaM2(change_footing_area_m2::mutation::ChangeFootingAreaM2),
    ChangePhiDeg(change_phi_deg::mutation::ChangePhiDeg),
    ChangeCKpa(change_c_kpa::mutation::ChangeCKpa),
    ChangeGammaKnM3(change_gamma_kn_m3::mutation::ChangeGammaKnM3),
    ChangeBM(change_b_m::mutation::ChangeBM),
    ChangeDFM(change_d_f_m::mutation::ChangeDFM),
    ChangeESMpa(change_e_s_mpa::mutation::ChangeESMpa),
    ChangeNu(change_nu::mutation::ChangeNu),
    ChangeDesignApproach(change_design_approach::mutation::ChangeDesignApproach),
    ChangeAnnex(change_annex::mutation::ChangeAnnex),
    ChangeSettlementLimitMm(change_settlement_limit_mm::mutation::ChangeSettlementLimitMm),
    ChangeNPileEdKn(change_n_pile_ed_kn::mutation::ChangeNPileEdKn),
    ChangeAlphaS(change_alpha_s::mutation::ChangeAlphaS),
    ChangePileDM(change_pile_d_m::mutation::ChangePileDM),
    ChangeQSKpa(change_q_s_kpa::mutation::ChangeQSKpa),
    ChangePileLM(change_pile_l_m::mutation::ChangePileLM),
    ChangeQBKpa(change_q_b_kpa::mutation::ChangeQBKpa),
    ChangePileBaseAreaM2(change_pile_base_area_m2::mutation::ChangePileBaseAreaM2),
    ChangePileNProfiles(change_pile_n_profiles::mutation::ChangePileNProfiles),
    ChangeZInvestigatedM(change_z_investigated_m::mutation::ChangeZInvestigatedM),
}

/// 🏷️ Every declared kind of [`En1997Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🧪️oracle/🔣️component.json` publishes as the `en1997-1-any`
/// mutation catalog and `../../../../../🧪️tests/mutate-en1997-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-v-ed-kn",
    "change-h-ed-kn",
    "change-footing-area-m2",
    "change-phi-deg",
    "change-c-kpa",
    "change-gamma-kn-m3",
    "change-bm",
    "change-dfm",
    "change-es-mpa",
    "change-nu",
    "change-design-approach",
    "change-annex",
    "change-settlement-limit-mm",
    "change-n-pile-ed-kn",
    "change-alpha-s",
    "change-pile-dm",
    "change-qs-kpa",
    "change-pile-lm",
    "change-qb-kpa",
    "change-pile-base-area-m2",
    "change-pile-n-profiles",
    "change-z-investigated-m",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1997Mutation {
    /// 📤️ Decomposes a whole `En1997Snapshot` into one `change-<field>` mutation per
    /// persistent field — the closed-vocabulary replacement for the banned whole-document-replace
    /// variant, used by `import_media`'s `"model:in"` port and the `set-snapshot` app command to
    /// bundle a bulk document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &En1997Snapshot) -> Vec<En1997Mutation> {
        let mut mutations = Vec::with_capacity(22);
        mutations.push(En1997Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: snapshot.v_ed_kn.clone() }));
        mutations.push(En1997Mutation::ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn { new_h_ed_kn: snapshot.h_ed_kn.clone() }));
        mutations.push(En1997Mutation::ChangeFootingAreaM2(change_footing_area_m2::mutation::ChangeFootingAreaM2 { new_footing_area_m2: snapshot.footing_area_m2.clone() }));
        mutations.push(En1997Mutation::ChangePhiDeg(change_phi_deg::mutation::ChangePhiDeg { new_phi_deg: snapshot.phi_deg.clone() }));
        mutations.push(En1997Mutation::ChangeCKpa(change_c_kpa::mutation::ChangeCKpa { new_c_kpa: snapshot.c_kpa.clone() }));
        mutations.push(En1997Mutation::ChangeGammaKnM3(change_gamma_kn_m3::mutation::ChangeGammaKnM3 { new_gamma_kn_m3: snapshot.gamma_kn_m3.clone() }));
        mutations.push(En1997Mutation::ChangeBM(change_b_m::mutation::ChangeBM { new_b_m: snapshot.b_m.clone() }));
        mutations.push(En1997Mutation::ChangeDFM(change_d_f_m::mutation::ChangeDFM { new_d_f_m: snapshot.d_f_m.clone() }));
        mutations.push(En1997Mutation::ChangeESMpa(change_e_s_mpa::mutation::ChangeESMpa { new_e_s_mpa: snapshot.e_s_mpa.clone() }));
        mutations.push(En1997Mutation::ChangeNu(change_nu::mutation::ChangeNu { new_nu: snapshot.nu.clone() }));
        mutations.push(En1997Mutation::ChangeDesignApproach(change_design_approach::mutation::ChangeDesignApproach { new_design_approach: snapshot.design_approach.clone() }));
        mutations.push(En1997Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: snapshot.annex.clone() }));
        mutations.push(En1997Mutation::ChangeSettlementLimitMm(change_settlement_limit_mm::mutation::ChangeSettlementLimitMm { new_settlement_limit_mm: snapshot.settlement_limit_mm.clone() }));
        mutations.push(En1997Mutation::ChangeNPileEdKn(change_n_pile_ed_kn::mutation::ChangeNPileEdKn { new_n_pile_ed_kn: snapshot.n_pile_ed_kn.clone() }));
        mutations.push(En1997Mutation::ChangeAlphaS(change_alpha_s::mutation::ChangeAlphaS { new_alpha_s: snapshot.alpha_s.clone() }));
        mutations.push(En1997Mutation::ChangePileDM(change_pile_d_m::mutation::ChangePileDM { new_pile_d_m: snapshot.pile_d_m.clone() }));
        mutations.push(En1997Mutation::ChangeQSKpa(change_q_s_kpa::mutation::ChangeQSKpa { new_q_s_kpa: snapshot.q_s_kpa.clone() }));
        mutations.push(En1997Mutation::ChangePileLM(change_pile_l_m::mutation::ChangePileLM { new_pile_l_m: snapshot.pile_l_m.clone() }));
        mutations.push(En1997Mutation::ChangeQBKpa(change_q_b_kpa::mutation::ChangeQBKpa { new_q_b_kpa: snapshot.q_b_kpa.clone() }));
        mutations.push(En1997Mutation::ChangePileBaseAreaM2(change_pile_base_area_m2::mutation::ChangePileBaseAreaM2 { new_pile_base_area_m2: snapshot.pile_base_area_m2.clone() }));
        mutations.push(En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::mutation::ChangePileNProfiles { new_pile_n_profiles: snapshot.pile_n_profiles.clone() }));
        mutations.push(En1997Mutation::ChangeZInvestigatedM(change_z_investigated_m::mutation::ChangeZInvestigatedM { new_z_investigated_m: snapshot.z_investigated_m.clone() }));
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

    /// ⚖️ One value per `En1997Mutation` variant — the closed set the semantics/round-trip
    /// tests iterate.
    fn every_mutation() -> Vec<En1997Mutation> {
        vec![
            En1997Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 620.0 }),
            En1997Mutation::ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn { new_h_ed_kn: 95.0 }),
            En1997Mutation::ChangeFootingAreaM2(change_footing_area_m2::mutation::ChangeFootingAreaM2 { new_footing_area_m2: 2.4 }),
            En1997Mutation::ChangePhiDeg(change_phi_deg::mutation::ChangePhiDeg { new_phi_deg: 32.0 }),
            En1997Mutation::ChangeCKpa(change_c_kpa::mutation::ChangeCKpa { new_c_kpa: 5.0 }),
            En1997Mutation::ChangeGammaKnM3(change_gamma_kn_m3::mutation::ChangeGammaKnM3 { new_gamma_kn_m3: 19.0 }),
            En1997Mutation::ChangeBM(change_b_m::mutation::ChangeBM { new_b_m: 2.2 }),
            En1997Mutation::ChangeDFM(change_d_f_m::mutation::ChangeDFM { new_d_f_m: 1.8 }),
            En1997Mutation::ChangeESMpa(change_e_s_mpa::mutation::ChangeESMpa { new_e_s_mpa: 32_000.0 }),
            En1997Mutation::ChangeNu(change_nu::mutation::ChangeNu { new_nu: 0.32 }),
            En1997Mutation::ChangeDesignApproach(change_design_approach::mutation::ChangeDesignApproach { new_design_approach: "da2".to_string() }),
            En1997Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
            En1997Mutation::ChangeSettlementLimitMm(change_settlement_limit_mm::mutation::ChangeSettlementLimitMm { new_settlement_limit_mm: 20.0 }),
            En1997Mutation::ChangeNPileEdKn(change_n_pile_ed_kn::mutation::ChangeNPileEdKn { new_n_pile_ed_kn: 900.0 }),
            En1997Mutation::ChangeAlphaS(change_alpha_s::mutation::ChangeAlphaS { new_alpha_s: 0.75 }),
            En1997Mutation::ChangePileDM(change_pile_d_m::mutation::ChangePileDM { new_pile_d_m: 0.65 }),
            En1997Mutation::ChangeQSKpa(change_q_s_kpa::mutation::ChangeQSKpa { new_q_s_kpa: 90.0 }),
            En1997Mutation::ChangePileLM(change_pile_l_m::mutation::ChangePileLM { new_pile_l_m: 14.0 }),
            En1997Mutation::ChangeQBKpa(change_q_b_kpa::mutation::ChangeQBKpa { new_q_b_kpa: 2700.0 }),
            En1997Mutation::ChangePileBaseAreaM2(change_pile_base_area_m2::mutation::ChangePileBaseAreaM2 { new_pile_base_area_m2: 0.33 }),
            En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::mutation::ChangePileNProfiles { new_pile_n_profiles: 3 }),
            En1997Mutation::ChangeZInvestigatedM(change_z_investigated_m::mutation::ChangeZInvestigatedM { new_z_investigated_m: 10.0 }),
        ]
    }

    fn round_trip(base: &En1997Snapshot, mutation: &En1997Mutation) -> En1997Snapshot {
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
        assert_eq!(<En1997Mutation as protocol::SemanticMutation<En1997Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[semio_framework_async_macros::async_test]
    fn every_variant_round_trips_via_inverse() {
        let base = En1997Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    #[semio_framework_async_macros::async_test]
    fn from_snapshot_round_trips_via_full_document_replacement() {
        let base = En1997Snapshot::default();
        let mut target = En1997Snapshot::default();
        let _ = &mut target;
        let mut projected = base.clone();
        for mutation in En1997Mutation::from_snapshot(&target) {
            projected = vcs::apply_mutation(&projected, &mutation).expect("snapshot mutation applies").0;
        }
        assert_eq!(projected, target, "from_snapshot must reconstruct every persistent field");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
    /// variants.
    #[semio_framework_async_macros::async_test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = En1997Snapshot::default();
        let mutation = En1997Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1997Mutation::ChangeDesignApproach(change_design_approach::mutation::ChangeDesignApproach { new_design_approach: "da2".to_string() }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    fn change_v_ed_kn_satisfies_the_inverse_and_absorb_laws() {
        let base = En1997Snapshot::default();
        let mutation = En1997Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 620.0 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::mutation::ChangePileNProfiles { new_pile_n_profiles: 3 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    fn change_design_approach_satisfies_the_inverse_and_absorb_laws() {
        let base = En1997Snapshot::default();
        let mutation = En1997Mutation::ChangeDesignApproach(change_design_approach::mutation::ChangeDesignApproach { new_design_approach: "da2".to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1997Mutation::ChangePhiDeg(change_phi_deg::mutation::ChangePhiDeg { new_phi_deg: 32.0 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
/// 🧪️ Handcrafted mutation fixtures (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`),
/// self-wired here rather than in `📦️glue.rs`: that file is shared with the other artifact lanes
/// running concurrently, and a `#[path]` on a module declared at the top level of this non-mod-rs
/// file already resolves relative to this very directory.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🌿change-alpha-s/🧪️tests/lowers-the-shaft-resistance-factor-to-0-5/🦀️component.rs"]
    mod tests_change_alpha_s_lowers_the_shaft_resistance_factor_to_0_5;
    #[path = "🛏️change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/🦀️component.rs"]
    mod tests_change_annex_switches_from_the_german_na_to_the_recommended_en_annex;
    #[path = "🧹change-bm/🧪️tests/widens-the-footing-to-2-5-m/🦀️component.rs"]
    mod tests_change_bm_widens_the_footing_to_2_5_m;
    #[path = "🧽change-c-kpa/🧪️tests/gives-the-drained-sand-12-5-kpa-of-effective-cohesion/🦀️component.rs"]
    mod tests_change_c_kpa_gives_the_drained_sand_12_5_kpa_of_effective_cohesion;
    #[path = "🛋️change-design-approach/🧪️tests/switches-from-design-approach-1-to-design-approach-2/🦀️component.rs"]
    mod tests_change_design_approach_switches_from_design_approach_1_to_design_approach_2;
    #[path = "🧺change-dfm/🧪️tests/deepens-the-founding-level-to-2-m/🦀️component.rs"]
    mod tests_change_dfm_deepens_the_founding_level_to_2_m;
    #[path = "🪑change-es-mpa/🧪️tests/stiffens-the-soil-modulus-to-45-mpa/🦀️component.rs"]
    mod tests_change_es_mpa_stiffens_the_soil_modulus_to_45_mpa;
    #[path = "🧴change-footing-area-m2/🧪️tests/enlarges-the-footing-area-to-6-25-m2/🦀️component.rs"]
    mod tests_change_footing_area_m2_enlarges_the_footing_area_to_6_25_m2;
    #[path = "🪠change-gamma-kn-m3/🧪️tests/raises-the-soil-unit-weight-to-20-kn-m3/🦀️component.rs"]
    mod tests_change_gamma_kn_m3_raises_the_soil_unit_weight_to_20_kn_m3;
    #[path = "🪥change-h-ed-kn/🧪️tests/raises-the-design-horizontal-load-to-120-kn/🦀️component.rs"]
    mod tests_change_h_ed_kn_raises_the_design_horizontal_load_to_120_kn;
    #[path = "🛁change-n-pile-ed-kn/🧪️tests/raises-the-design-pile-axial-load-to-1200-kn/🦀️component.rs"]
    mod tests_change_n_pile_ed_kn_raises_the_design_pile_axial_load_to_1200_kn;
    #[path = "🪞change-nu/🧪️tests/raises-poissons-ratio-to-0-375/🦀️component.rs"]
    mod tests_change_nu_raises_poissons_ratio_to_0_375;
    #[path = "🧼change-phi-deg/🧪️tests/raises-the-friction-angle-to-35-degrees/🦀️component.rs"]
    mod tests_change_phi_deg_raises_the_friction_angle_to_35_degrees;
    #[path = "🌳change-pile-base-area-m2/🧪️tests/doubles-the-pile-base-area-to-0-5-m2/🦀️component.rs"]
    mod tests_change_pile_base_area_m2_doubles_the_pile_base_area_to_0_5_m2;
    #[path = "🍀change-pile-dm/🧪️tests/enlarges-the-pile-diameter-to-0-75-m/🦀️component.rs"]
    mod tests_change_pile_dm_enlarges_the_pile_diameter_to_0_75_m;
    #[path = "🌵change-pile-lm/🧪️tests/lengthens-the-pile-to-15-m/🦀️component.rs"]
    mod tests_change_pile_lm_lengthens_the_pile_to_15_m;
    #[path = "🌲change-pile-n-profiles/🧪️tests/adds-a-third-investigated-ground-profile/🦀️component.rs"]
    mod tests_change_pile_n_profiles_adds_a_third_investigated_ground_profile;
    #[path = "🌴change-qb-kpa/🧪️tests/raises-the-unit-base-resistance-to-3200-kpa/🦀️component.rs"]
    mod tests_change_qb_kpa_raises_the_unit_base_resistance_to_3200_kpa;
    #[path = "🌾change-qs-kpa/🧪️tests/raises-the-unit-shaft-resistance-to-120-kpa/🦀️component.rs"]
    mod tests_change_qs_kpa_raises_the_unit_shaft_resistance_to_120_kpa;
    #[path = "🚿change-settlement-limit-mm/🧪️tests/relaxes-the-settlement-limit-to-40-mm/🦀️component.rs"]
    mod tests_change_settlement_limit_mm_relaxes_the_settlement_limit_to_40_mm;
    #[path = "🪒change-v-ed-kn/🧪️tests/raises-the-design-vertical-load-to-750-kn/🦀️component.rs"]
    mod tests_change_v_ed_kn_raises_the_design_vertical_load_to_750_kn;
    #[path = "🍁change-z-investigated-m/🧪️tests/deepens-the-investigated-depth-to-12-m/🦀️component.rs"]
    mod tests_change_z_investigated_m_deepens_the_investigated_depth_to_12_m;
}
//#endregion 🧪️FixtureTests


//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json`
/// specification vectors carry — into a real [`En1997Mutation`]. The generated test host of
/// `../../../../../🧪️tests/mutate-en1997-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1997_mutation_json(text: &str) -> Result<En1997Mutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_en1997_mutation(base: &En1997Snapshot, mutation: &En1997Mutation) -> Result<(En1997Snapshot, Vec<String>), String> {
    let raised = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1997Diff as protocol::MutationDiff<En1997Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `mutate-en1997-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_en1997_mutation(mutation: &En1997Mutation, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
    /// committed `en1997-1-any` catalog. The framework never parses Rust, so this is the only thing
    /// standing between a renamed variant and a completeness gate that silently measures the wrong
    /// set.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <En1997Mutation as protocol::SemanticMutation<En1997Snapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared En1997Mutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsCatalog
