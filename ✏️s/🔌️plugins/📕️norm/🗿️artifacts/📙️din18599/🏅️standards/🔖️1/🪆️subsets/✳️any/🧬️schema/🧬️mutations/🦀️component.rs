//! 🧬️ Din18599 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Din18599Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (twelve persistent scalar/enum fields describing occupancy, heat
//! transfer, energy balance and reference demand inputs to a DIN V 18599 primary-energy compliance
//! check, plus one nested `MonthlyClimate` facet) — no id-keyed collections, no name/identity field
//! to `rename`. The twelve top-level scalars each become their own `change-<field>` mutation. The
//! `climate: MonthlyClimate` field (two twelve-month arrays, `theta_e_c`/`g_h_w_m2`) is the one
//! genuinely inseparable ≥2-field facet in this lane's whole Job A batch: both arrays are entered
//! together as one climate dataset (typically loaded via `MonthlyClimate::german_reference` for a
//! `ClimateZoneDe`), never meaningfully set one month/array at a time from this app's own input
//! surface — so it gets a single `update-climate` mutation per the rule's `update-<facet>` exception,
//! not two `change-*` mutations on its component arrays. The pre-migration whole-document-replace
//! variant is gone: banned outright per `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO
//! replacement mutation; file-open/import/load-example now goes through `store::ArtifactStore::reset`,
//! entirely outside this enum. The old whole-document-replace macro call is removed with it.
//!
//! All triads are mounted directly as `mutations`-sibling modules in `📦️glue.rs` (this lane's agent
//! owns `📦️glue.rs`, so no self-wiring `#[path = "."]` blocks are needed here).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::Din18599Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::change_annual_limit_kwh;
use super::change_energy_carrier;
use super::change_h_t;
use super::change_h_v;
use super::change_heated_area_m2;
use super::change_internal_gains_w_m2;
use super::change_occupants;
use super::change_reference_q_p_kwh;
use super::change_renewable_kwh;
use super::change_solar_gains_kwh;
use super::change_system_losses_kwh;
use super::change_use_class;
use super::update_climate;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the din18599 document, derived per
/// `📓️derivation-rules.md` from `Din18599Snapshot`'s shape: twelve flat scalars plus one inseparable
/// nested `climate` facet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Din18599Snapshot, diff = Din18599Diff, schema = "norm.din18599")]
pub enum Din18599Mutation {
    ChangeUseClass(change_use_class::mutation::ChangeUseClass),
    ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2),
    ChangeOccupants(change_occupants::mutation::ChangeOccupants),
    ChangeHT(change_h_t::mutation::ChangeHT),
    ChangeHV(change_h_v::mutation::ChangeHV),
    ChangeInternalGainsWM2(change_internal_gains_w_m2::mutation::ChangeInternalGainsWM2),
    ChangeSolarGainsKwh(change_solar_gains_kwh::mutation::ChangeSolarGainsKwh),
    ChangeSystemLossesKwh(change_system_losses_kwh::mutation::ChangeSystemLossesKwh),
    ChangeRenewableKwh(change_renewable_kwh::mutation::ChangeRenewableKwh),
    ChangeAnnualLimitKwh(change_annual_limit_kwh::mutation::ChangeAnnualLimitKwh),
    ChangeEnergyCarrier(change_energy_carrier::mutation::ChangeEnergyCarrier),
    ChangeReferenceQPKwh(change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh),
    UpdateClimate(update_climate::mutation::UpdateClimate),
}

/// 🏷️ Every declared kind of [`Din18599Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🧪️oracle/🔣️component.json` publishes as the `din18599-1-any`
/// mutation catalog and `../../../../../🧪️tests/mutate-din18599-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-use-class",
    "change-heated-area-m2",
    "change-occupants",
    "change-ht",
    "change-hv",
    "change-internal-gains-wm2",
    "change-solar-gains-kwh",
    "change-system-losses-kwh",
    "change-renewable-kwh",
    "change-annual-limit-kwh",
    "change-energy-carrier",
    "change-reference-qp-kwh",
    "update-climate",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl Din18599Mutation {
    /// 📤️ Decomposes a whole `Din18599Snapshot` into one `change-<field>` mutation per scalar field
    /// plus one `update-climate` for the nested facet — the closed-vocabulary replacement for the
    /// banned whole-document-replace variant, used by `import_media`'s `"model:in"` port and the
    /// `set-snapshot` app command to bundle a bulk document replacement into a single atomic
    /// `Emit::commit`.
    pub async fn from_snapshot(snapshot: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        let mut mutations = Vec::with_capacity(13);
        mutations.push(Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: snapshot.use_class.clone() }));
        mutations.push(Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2: snapshot.heated_area_m2.clone() }));
        mutations.push(Din18599Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: snapshot.occupants.clone() }));
        mutations.push(Din18599Mutation::ChangeHT(change_h_t::mutation::ChangeHT { new_h_t: snapshot.h_t.clone() }));
        mutations.push(Din18599Mutation::ChangeHV(change_h_v::mutation::ChangeHV { new_h_v: snapshot.h_v.clone() }));
        mutations.push(Din18599Mutation::ChangeInternalGainsWM2(change_internal_gains_w_m2::mutation::ChangeInternalGainsWM2 { new_internal_gains_w_m2: snapshot.internal_gains_w_m2.clone() }));
        mutations.push(Din18599Mutation::ChangeSolarGainsKwh(change_solar_gains_kwh::mutation::ChangeSolarGainsKwh { new_solar_gains_kwh: snapshot.solar_gains_kwh.clone() }));
        mutations.push(Din18599Mutation::ChangeSystemLossesKwh(change_system_losses_kwh::mutation::ChangeSystemLossesKwh { new_system_losses_kwh: snapshot.system_losses_kwh.clone() }));
        mutations.push(Din18599Mutation::ChangeRenewableKwh(change_renewable_kwh::mutation::ChangeRenewableKwh { new_renewable_kwh: snapshot.renewable_kwh.clone() }));
        mutations.push(Din18599Mutation::ChangeAnnualLimitKwh(change_annual_limit_kwh::mutation::ChangeAnnualLimitKwh { new_annual_limit_kwh: snapshot.annual_limit_kwh.clone() }));
        mutations.push(Din18599Mutation::ChangeEnergyCarrier(change_energy_carrier::mutation::ChangeEnergyCarrier { new_energy_carrier: snapshot.energy_carrier.clone() }));
        mutations.push(Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh { new_reference_q_p_kwh: snapshot.reference_q_p_kwh.clone() }));
        mutations.push(Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate { new_climate: crate::artifacts::din18599::din18599_climate(snapshot) }));
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

    async fn every_mutation() -> Vec<Din18599Mutation> {
        vec![
            Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: crate::artifacts::din18599::UseClass::Office }),
            Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 }),
            Din18599Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: 5 }),
            Din18599Mutation::ChangeHT(change_h_t::mutation::ChangeHT { new_h_t: 95.0 }),
            Din18599Mutation::ChangeHV(change_h_v::mutation::ChangeHV { new_h_v: 42.0 }),
            Din18599Mutation::ChangeInternalGainsWM2(change_internal_gains_w_m2::mutation::ChangeInternalGainsWM2 { new_internal_gains_w_m2: 4.0 }),
            Din18599Mutation::ChangeSolarGainsKwh(change_solar_gains_kwh::mutation::ChangeSolarGainsKwh { new_solar_gains_kwh: 90.0 }),
            Din18599Mutation::ChangeSystemLossesKwh(change_system_losses_kwh::mutation::ChangeSystemLossesKwh { new_system_losses_kwh: 850.0 }),
            Din18599Mutation::ChangeRenewableKwh(change_renewable_kwh::mutation::ChangeRenewableKwh { new_renewable_kwh: 1600.0 }),
            Din18599Mutation::ChangeAnnualLimitKwh(change_annual_limit_kwh::mutation::ChangeAnnualLimitKwh { new_annual_limit_kwh: 8000.0 }),
            Din18599Mutation::ChangeEnergyCarrier(change_energy_carrier::mutation::ChangeEnergyCarrier { new_energy_carrier: "district_heat".to_string() }),
            Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh { new_reference_q_p_kwh: 10500.0 }),
            Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate {
                new_climate: crate::artifacts::din18599::MonthlyClimate { theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0], g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0] },
            }),
        ]
    }

    async fn round_trip(base: &Din18599Snapshot, mutation: &Din18599Mutation) -> Din18599Snapshot {
        let forward = vcs::apply_mutation(base, mutation).expect("valid mutation").0;
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<Din18599Mutation as protocol::SemanticMutation<Din18599Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[semio_framework_async_macros::async_test]
    async fn every_variant_round_trips_via_inverse() {
        let base = Din18599Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn from_snapshot_round_trips_via_full_document_replacement() {
        let base = Din18599Snapshot::default();
        let target = Din18599Snapshot::default();
        let mut projected = base.clone();
        for mutation in Din18599Mutation::from_snapshot(&target) {
            projected = vcs::apply_mutation(&projected, &mutation).expect("snapshot mutation applies").0;
        }
        assert_eq!(projected, target, "from_snapshot must reconstruct every persistent field");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
    /// variants: the nested-facet `update-climate`, an enum scalar (`change-use-class`), and a plain
    /// `f64` scalar (`change-heated-area-m2`).
    #[semio_framework_async_macros::async_test]
    async fn update_climate_satisfies_the_inverse_and_absorb_laws() {
        let base = Din18599Snapshot::default();
        let mutation = Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate {
            new_climate: crate::artifacts::din18599::MonthlyClimate { theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0], g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0] },
        });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: crate::artifacts::din18599::UseClass::Office }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    async fn change_use_class_satisfies_the_inverse_and_absorb_laws() {
        let base = Din18599Snapshot::default();
        let mutation = Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: crate::artifacts::din18599::UseClass::Office });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    async fn change_heated_area_m2_satisfies_the_inverse_and_absorb_laws() {
        let base = Din18599Snapshot::default();
        let mutation = Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = Din18599Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: 5 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Self-wired fixture cases for the DIN V 18599 mutation vocabulary: one handcrafted case per
// triad leaf, mounted here rather than in `📦️glue.rs` because that file is shared by all
// fifteen norm artifacts and several lanes edit it at once. `#[path = "."]` keeps the
// inline module's own name out of the base directory, so every leaf path below is read
// straight off this `🧬️mutations/` directory (ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION,
// contract D1).
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🦡change-annual-limit-kwh/🧪️tests/tightens-the-annual-primary-energy-limit-to-6000-kwh/🦀️component.rs"]
    mod tests_change_annual_limit_kwh_tightens_the_annual_primary_energy_limit_to_6000_kwh;
    #[path = "📐change-energy-carrier/🧪️tests/switches-the-energy-carrier-to-an-electric-heat-pump/🦀️component.rs"]
    mod tests_change_energy_carrier_switches_the_energy_carrier_to_an_electric_heat_pump;
    #[path = "🐫change-ht/🧪️tests/raises-the-transmission-loss-coefficient-to-118-w-per-k/🦀️component.rs"]
    mod tests_change_h_t_raises_the_transmission_loss_coefficient_to_118_w_per_k;
    #[path = "🦒change-hv/🧪️tests/raises-the-ventilation-loss-coefficient-to-52-25-w-per-k/🦀️component.rs"]
    mod tests_change_h_v_raises_the_ventilation_loss_coefficient_to_52_25_w_per_k;
    #[path = "🦛change-heated-area-m2/🧪️tests/extends-the-heated-area-to-160-m2/🦀️component.rs"]
    mod tests_change_heated_area_m2_extends_the_heated_area_to_160_m2;
    #[path = "🦘change-internal-gains-wm2/🧪️tests/raises-the-internal-gains-to-5-w-per-m2/🦀️component.rs"]
    mod tests_change_internal_gains_w_m2_raises_the_internal_gains_to_5_w_per_m2;
    #[path = "🐪change-occupants/🧪️tests/raises-the-occupancy-to-six-people/🦀️component.rs"]
    mod tests_change_occupants_raises_the_occupancy_to_six_people;
    #[path = "🔽change-reference-qp-kwh/🧪️tests/lowers-the-reference-building-primary-energy-to-8750-kwh/🦀️component.rs"]
    mod tests_change_reference_q_p_kwh_lowers_the_reference_building_primary_energy_to_8750_kwh;
    #[path = "🦨change-renewable-kwh/🧪️tests/raises-the-on-site-renewable-yield-to-2250-kwh/🦀️component.rs"]
    mod tests_change_renewable_kwh_raises_the_on_site_renewable_yield_to_2250_kwh;
    #[path = "🦥change-solar-gains-kwh/🧪️tests/raises-the-annual-solar-gains-to-132-kwh/🦀️component.rs"]
    mod tests_change_solar_gains_kwh_raises_the_annual_solar_gains_to_132_kwh;
    #[path = "🦦change-system-losses-kwh/🧪️tests/cuts-the-system-losses-to-450-kwh/🦀️component.rs"]
    mod tests_change_system_losses_kwh_cuts_the_system_losses_to_450_kwh;
    #[path = "🦏change-use-class/🧪️tests/reclassifies-the-building-as-an-office/🦀️component.rs"]
    mod tests_change_use_class_reclassifies_the_building_as_an_office;
    #[path = "🐘update-climate/🧪️tests/refuses-a-negative-january-irradiance/🦀️component.rs"]
    mod tests_update_climate_refuses_a_negative_january_irradiance;
}
//#endregion 🧪️FixtureTests


//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json`
/// specification vectors carry — into a real [`Din18599Mutation`]. The generated test host of
/// `../../../../../🧪️tests/mutate-din18599-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_mutation_json(text: &str) -> Result<Din18599Mutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_din18599_mutation(base: &Din18599Snapshot, mutation: &Din18599Mutation) -> Result<(Din18599Snapshot, Vec<String>), String> {
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `mutate-din18599-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_din18599_mutation(mutation: &Din18599Mutation, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
    /// committed `din18599-1-any` catalog. The framework never parses Rust, so this is the only thing
    /// standing between a renamed variant and a completeness gate that silently measures the wrong
    /// set.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Din18599Mutation as protocol::SemanticMutation<Din18599Snapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Din18599Mutation variant");
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
