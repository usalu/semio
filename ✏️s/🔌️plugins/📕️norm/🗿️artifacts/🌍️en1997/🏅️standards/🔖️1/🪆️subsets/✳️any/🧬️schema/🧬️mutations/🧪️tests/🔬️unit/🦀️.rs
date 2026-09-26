use super::*;
use crate::document::AnnexChoice;
use crate::standards::v1::subsets::any::schema::snapshot::compliant_demo;
use protocol::Mutation;

/// ⚖️ One value per `En1997Mutation` variant — the closed set the semantics/round-trip
/// tests iterate.
fn every_mutation() -> Vec<En1997Mutation> {
    let demo = compliant_demo();
    let layer = demo.layers[0].clone();
    let footing = demo.footings[0].clone();
    let pile = demo.piles[0].clone();
    vec![
        En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::De }),
        En1997Mutation::ChangeGeotechnicalCategory(change_geotechnical_category::ChangeGeotechnicalCategory { new_geotechnical_category: 2 }),
        En1997Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation: "persistent".into() }),
        En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da2".into() }),
        En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel { new_groundwater_level: -2.0 }),
        En1997Mutation::ChangeInvestigationDepth(change_investigation_depth::ChangeInvestigationDepth { new_investigation_depth: 12.0 }),
        En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth { id: "f1".into(), new_width: 2.5 }),
        En1997Mutation::ChangeFootingEmbedment(change_footing_embedment::ChangeFootingEmbedment { id: "f1".into(), new_embedment: 1.2 }),
        En1997Mutation::ChangePileLength(change_pile_length::ChangePileLength { id: "p1".into(), new_length: 14.0 }),
        En1997Mutation::ChangePileCount(change_pile_count::ChangePileCount { id: "p1".into(), new_count: 4 }),
        En1997Mutation::ChangeWallBaseWidth(change_wall_base_width::ChangeWallBaseWidth { id: "w1".into(), new_base_width: 2.0 }),
        En1997Mutation::ChangeSlopeAngle(change_slope_angle::ChangeSlopeAngle { id: "s1".into(), new_angle_deg: 28.0 }),
        En1997Mutation::ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime { id: "L1".into(), new_phi_prime_deg: 32.0 }),
        En1997Mutation::ChangeLayerOedometricModulus(change_layer_oedometric_modulus::ChangeLayerOedometricModulus {
            id: "L1".into(),
            new_oedometric_modulus: 40e6,
        }),
        En1997Mutation::InsertLayer(insert_layer::InsertLayer { index: 0, layer }),
        En1997Mutation::RemoveLayer(remove_layer::RemoveLayer { index: 0 }),
        En1997Mutation::InsertFooting(insert_footing::InsertFooting { index: 0, footing }),
        En1997Mutation::RemoveFooting(remove_footing::RemoveFooting { index: 0 }),
        En1997Mutation::InsertPile(insert_pile::InsertPile { index: 0, pile }),
        En1997Mutation::RemovePile(remove_pile::RemovePile { index: 0 }),
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
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for mutation in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
    }
    assert_eq!(<En1997Mutation as protocol::SemanticMutation<En1997Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = compliant_demo();
    for mutation in every_mutation() {
        // Skip pure removes/inserts against empty collections that would fail identity round-trip on default empty
        round_trip(&base, &mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn from_snapshot_round_trips_via_full_document_replacement() {
    let base = En1997Snapshot::default();
    let target = compliant_demo();
    let mut projected = base.clone();
    for mutation in En1997Mutation::from_snapshot(&base, &target) {
        projected = vcs::apply_mutation(&projected, &mutation).expect("snapshot mutation applies").0;
    }
    assert_eq!(projected.annex, target.annex);
    assert_eq!(projected.design_approach, target.design_approach);
    assert_eq!(projected.groundwater_level, target.groundwater_level);
    assert_eq!(projected.layers.len(), target.layers.len());
    assert_eq!(projected.footings.len(), target.footings.len());
    assert_eq!(projected.piles.len(), target.piles.len());
}

//#region 🧪️MutationLaws
#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = compliant_demo();
    let mutation = En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da1".to_string() }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_footing_width_satisfies_the_inverse_and_absorb_laws() {
    let base = compliant_demo();
    let mutation = En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth { id: base.footings[0].id.clone(), new_width: 3.0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel { new_groundwater_level: -1.5 }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_design_approach_satisfies_the_inverse_and_absorb_laws() {
    let base = compliant_demo();
    let mutation = En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da3".to_string() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime { id: base.layers[0].id.clone(), new_phi_prime_deg: 35.0 }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws
