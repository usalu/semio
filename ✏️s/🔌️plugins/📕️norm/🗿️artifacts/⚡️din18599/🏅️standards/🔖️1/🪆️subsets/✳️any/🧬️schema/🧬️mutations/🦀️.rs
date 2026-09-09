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
//! All triads are mounted directly as `mutations`-sibling modules in `🦀️.rs` (this lane's agent
//! owns `🦀️.rs`, so no self-wiring `#[path = "."]` blocks are needed here).

use crate::diff::Din18599Diff;
use crate::Din18599Snapshot;

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
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Din18599Snapshot, diff = Din18599Diff, schema = "norm.din18599")]
pub enum Din18599Mutation {
    ChangeUseClass(change_use_class::ChangeUseClass),
    ChangeHeatedAreaM2(change_heated_area_m2::ChangeHeatedAreaM2),
    ChangeOccupants(change_occupants::ChangeOccupants),
    ChangeHT(change_h_t::ChangeHT),
    ChangeHV(change_h_v::ChangeHV),
    ChangeInternalGainsWM2(change_internal_gains_w_m2::ChangeInternalGainsWM2),
    ChangeSolarGainsKwh(change_solar_gains_kwh::ChangeSolarGainsKwh),
    ChangeSystemLossesKwh(change_system_losses_kwh::ChangeSystemLossesKwh),
    ChangeRenewableKwh(change_renewable_kwh::ChangeRenewableKwh),
    ChangeAnnualLimitKwh(change_annual_limit_kwh::ChangeAnnualLimitKwh),
    ChangeEnergyCarrier(change_energy_carrier::ChangeEnergyCarrier),
    ChangeReferenceQPKwh(change_reference_q_p_kwh::ChangeReferenceQPKwh),
    UpdateClimate(update_climate::UpdateClimate),
}

/// 🏷️ Every declared kind of [`Din18599Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🔣️oracle.json` publishes as the `din18599-1-any`
/// mutation catalog and `../../../../../🧪️tests/⚡️mutate-din18599-1` registers its scenarios from. The
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
    pub fn from_snapshot(snapshot: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        let mutations = vec![
            Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: snapshot.use_class }),
            Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::ChangeHeatedAreaM2 { new_heated_area_m2: snapshot.heated_area_m2 }),
            Din18599Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: snapshot.occupants }),
            Din18599Mutation::ChangeHT(change_h_t::ChangeHT { new_h_t: snapshot.h_t }),
            Din18599Mutation::ChangeHV(change_h_v::ChangeHV { new_h_v: snapshot.h_v }),
            Din18599Mutation::ChangeInternalGainsWM2(change_internal_gains_w_m2::ChangeInternalGainsWM2 { new_internal_gains_w_m2: snapshot.internal_gains_w_m2 }),
            Din18599Mutation::ChangeSolarGainsKwh(change_solar_gains_kwh::ChangeSolarGainsKwh { new_solar_gains_kwh: snapshot.solar_gains_kwh }),
            Din18599Mutation::ChangeSystemLossesKwh(change_system_losses_kwh::ChangeSystemLossesKwh { new_system_losses_kwh: snapshot.system_losses_kwh }),
            Din18599Mutation::ChangeRenewableKwh(change_renewable_kwh::ChangeRenewableKwh { new_renewable_kwh: snapshot.renewable_kwh }),
            Din18599Mutation::ChangeAnnualLimitKwh(change_annual_limit_kwh::ChangeAnnualLimitKwh { new_annual_limit_kwh: snapshot.annual_limit_kwh }),
            Din18599Mutation::ChangeEnergyCarrier(change_energy_carrier::ChangeEnergyCarrier { new_energy_carrier: snapshot.energy_carrier.clone() }),
            Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::ChangeReferenceQPKwh { new_reference_q_p_kwh: snapshot.reference_q_p_kwh }),
            Din18599Mutation::UpdateClimate(update_climate::UpdateClimate { new_climate: crate::din18599_climate(snapshot) }),
        ];
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Self-wired fixture cases for the DIN V 18599 mutation vocabulary: one handcrafted case per
// triad leaf, mounted here rather than in `🦀️.rs` because that file is shared by all
// fifteen norm artifacts and several lanes edit it at once. `#[path = "."]` keeps the
// inline module's own name out of the base directory, so every leaf path below is read
// straight off this `🧬️mutations/` directory (ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION,
// contract D1).
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`Din18599Mutation`]. The generated test host of
/// `../../../../../🧪️tests/⚡️mutate-din18599-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_mutation_json(text: &str) -> Result<Din18599Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
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
/// `⚡️mutate-din18599-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_din18599_mutation(mutation: &Din18599Mutation, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
