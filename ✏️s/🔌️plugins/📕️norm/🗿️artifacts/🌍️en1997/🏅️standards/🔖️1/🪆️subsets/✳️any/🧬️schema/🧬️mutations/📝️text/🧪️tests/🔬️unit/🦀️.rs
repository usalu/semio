use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::compliant_demo;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_annex() {
    store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_design_approach() {
    store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach {
        new_design_approach: "da2".to_string(),
    }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_footing_width() {
    store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth {
        id: "f1".into(),
        new_width: 2.4,
    }));
}

/// ⚖️ Every variant — full-coverage `OpText` round trip over the closed vocabulary.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

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
