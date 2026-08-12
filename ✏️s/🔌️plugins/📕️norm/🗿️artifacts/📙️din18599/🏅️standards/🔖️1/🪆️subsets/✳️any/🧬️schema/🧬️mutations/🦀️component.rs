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
use super::change_use_class;
use super::change_heated_area_m2;
use super::change_occupants;
use super::change_h_t;
use super::change_h_v;
use super::change_internal_gains_w_m2;
use super::change_solar_gains_kwh;
use super::change_system_losses_kwh;
use super::change_renewable_kwh;
use super::change_annual_limit_kwh;
use super::change_energy_carrier;
use super::change_reference_q_p_kwh;
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
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl Din18599Mutation {
    /// 📤️ Decomposes a whole `Din18599Snapshot` into one `change-<field>` mutation per scalar field
    /// plus one `update-climate` for the nested facet — the closed-vocabulary replacement for the
    /// banned whole-document-replace variant, used by `import_media`'s `"model:in"` port and the
    /// `set-snapshot` app command to bundle a bulk document replacement into a single atomic
    /// `Emit::commit`.
    pub fn from_snapshot(snapshot: &Din18599Snapshot) -> Vec<Din18599Mutation> {
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
        mutations.push(Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate { new_climate: snapshot.climate.clone() }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    fn every_mutation() -> Vec<Din18599Mutation> {
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
        Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate { new_climate: crate::artifacts::din18599::MonthlyClimate {
            theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0],
            g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0],
        } }),
        ]
    }

    fn round_trip(base: &Din18599Snapshot, mutation: &Din18599Mutation) -> Din18599Snapshot {
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
        assert_eq!(<Din18599Mutation as protocol::SemanticMutation<Din18599Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = Din18599Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    #[test]
    fn from_snapshot_round_trips_via_full_document_replacement() {
        let base = Din18599Snapshot::default();
        let target = Din18599Snapshot::default();
        let mut projected = base.clone();
        for mutation in Din18599Mutation::from_snapshot(&target) {
            projected = vcs::apply_mutation(&projected, &mutation);
        }
        assert_eq!(projected, target, "from_snapshot must reconstruct every persistent field");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
    /// variants: the nested-facet `update-climate`, an enum scalar (`change-use-class`), and a plain
    /// `f64` scalar (`change-heated-area-m2`).
    #[test]
    fn update_climate_satisfies_the_inverse_and_absorb_laws() {
        let base = Din18599Snapshot::default();
        let mutation = Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate { new_climate: crate::artifacts::din18599::MonthlyClimate {
            theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0],
            g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0],
        } });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: crate::artifacts::din18599::UseClass::Office }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_use_class_satisfies_the_inverse_and_absorb_laws() {
        let base = Din18599Snapshot::default();
        let mutation = Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: crate::artifacts::din18599::UseClass::Office });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_heated_area_m2_satisfies_the_inverse_and_absorb_laws() {
        let base = Din18599Snapshot::default();
        let mutation = Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din18599Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: 5 }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
