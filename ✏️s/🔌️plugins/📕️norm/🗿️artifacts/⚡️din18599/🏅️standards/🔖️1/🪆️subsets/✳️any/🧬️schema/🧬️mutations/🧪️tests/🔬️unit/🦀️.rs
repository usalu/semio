
use super::*;
use protocol::Mutation;

fn every_mutation() -> Vec<Din18599Mutation> {
    vec![
        Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: crate::UseClass::Office }),
        Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 }),
        Din18599Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: 5 }),
        Din18599Mutation::ChangeHT(change_h_t::ChangeHT { new_h_t: 95.0 }),
        Din18599Mutation::ChangeHV(change_h_v::ChangeHV { new_h_v: 42.0 }),
        Din18599Mutation::ChangeInternalGainsWM2(change_internal_gains_w_m2::ChangeInternalGainsWM2 { new_internal_gains_w_m2: 4.0 }),
        Din18599Mutation::ChangeSolarGainsKwh(change_solar_gains_kwh::ChangeSolarGainsKwh { new_solar_gains_kwh: 90.0 }),
        Din18599Mutation::ChangeSystemLossesKwh(change_system_losses_kwh::ChangeSystemLossesKwh { new_system_losses_kwh: 850.0 }),
        Din18599Mutation::ChangeRenewableKwh(change_renewable_kwh::ChangeRenewableKwh { new_renewable_kwh: 1600.0 }),
        Din18599Mutation::ChangeAnnualLimitKwh(change_annual_limit_kwh::ChangeAnnualLimitKwh { new_annual_limit_kwh: 8000.0 }),
        Din18599Mutation::ChangeEnergyCarrier(change_energy_carrier::ChangeEnergyCarrier { new_energy_carrier: "district_heat".to_string() }),
        Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::ChangeReferenceQPKwh { new_reference_q_p_kwh: 10500.0 }),
        Din18599Mutation::UpdateClimate(update_climate::UpdateClimate {
            new_climate: crate::MonthlyClimate { theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0], g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0] },
        }),
    ]
}

fn round_trip(base: &Din18599Snapshot, mutation: &Din18599Mutation) -> Din18599Snapshot {
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
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
/// variants: the nested-facet `update-climate`, an enum scalar (`change-use-class`), and a plain
/// `f64` scalar (`change-heated-area-m2`).
#[semio_framework_async_macros::async_test]
async fn update_climate_satisfies_the_inverse_and_absorb_laws() {
    let base = Din18599Snapshot::default();
    let mutation = Din18599Mutation::UpdateClimate(update_climate::UpdateClimate {
        new_climate: crate::MonthlyClimate { theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0], g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0] },
    });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: crate::UseClass::Office }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_use_class_satisfies_the_inverse_and_absorb_laws() {
    let base = Din18599Snapshot::default();
    let mutation = Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: crate::UseClass::Office });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_heated_area_m2_satisfies_the_inverse_and_absorb_laws() {
    let base = Din18599Snapshot::default();
    let mutation = Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = Din18599Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: 5 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws
