use super::*;
use crate::document::ClimateZoneDe;
use crate::LayerDocument;
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn round_trip(base: &Din4108Snapshot, operation: &Din4108Mutation) -> Din4108Snapshot {
    let forward = operation.diff(base).diff().apply(base).expect("valid mutation diff");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(base).diff().apply(&restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

#[semio_framework_async_macros::async_test]
async fn every_scalar_change_round_trips() {
    let base = Din4108Snapshot::default();

    let after = round_trip(&base, &Din4108Mutation::ChangeCategory(change_category::ChangeCategory { new_category: "office".into() }));
    assert_eq!(after.category, "office");

    let after = round_trip(&base, &Din4108Mutation::ChangeClimate(change_climate::ChangeClimate { new_climate: ClimateZoneDe::Zone4 }));
    assert_eq!(after.climate, ClimateZoneDe::Zone4);

    let after = round_trip(&base, &Din4108Mutation::ChangeAirtightnessN50(change_airtightness_n50::ChangeAirtightnessN50 { new_airtightness_n50: 4.0 }));
    assert_eq!(after.airtightness_n50, 4.0);

    let after = round_trip(&base, &Din4108Mutation::ChangePsiTimesLSum(change_psi_times_l_sum::ChangePsiTimesLSum { new_psi_times_l_sum: 0.5 }));
    assert_eq!(after.psi_times_l_sum, 0.5);

    let after = round_trip(&base, &Din4108Mutation::ChangeRhInt(change_rh_int::ChangeRhInt { new_rh_int: 0.65 }));
    assert_eq!(after.rh_int, 0.65);

    let after = round_trip(&base, &Din4108Mutation::ChangeCatalogId(change_catalog_id::ChangeCatalogId { new_catalog_id: "AW-02".into() }));
    assert_eq!(after.catalog_id, "AW-02");

    let after = round_trip(&base, &Din4108Mutation::ChangeMaterialId(change_material_id::ChangeMaterialId { new_material_id: "eps".into() }));
    assert_eq!(after.material_id, "eps");

    let after = round_trip(&base, &Din4108Mutation::ChangeAirtightnessClass(change_airtightness_class::ChangeAirtightnessClass { new_airtightness_class: "class1".into() }));
    assert_eq!(after.airtightness_class, "class1");

    let after = round_trip(&base, &Din4108Mutation::ChangeTIntC(change_t_int_c::ChangeTIntC { new_t_int_c: 22.0 }));
    assert_eq!(after.t_int_c, 22.0);

    let after = round_trip(&base, &Din4108Mutation::ChangeSolarAbsorptance(change_solar_absorptance::ChangeSolarAbsorptance { new_solar_absorptance: 0.8 }));
    assert_eq!(after.solar_absorptance, 0.8);

    let after = round_trip(&base, &Din4108Mutation::ChangeIrradianceWM2(change_irradiance_w_m2::ChangeIrradianceWM2 { new_irradiance_w_m2: 700.0 }));
    assert_eq!(after.irradiance_w_m2, 700.0);

    let after = round_trip(&base, &Din4108Mutation::ChangeMoistureMuExterior(change_moisture_mu_exterior::ChangeMoistureMuExterior { new_moisture_mu_exterior: 20.0 }));
    assert_eq!(after.moisture_mu_exterior, 20.0);

    let after = round_trip(&base, &Din4108Mutation::ChangeMoistureMuInterior(change_moisture_mu_interior::ChangeMoistureMuInterior { new_moisture_mu_interior: 2.0 }));
    assert_eq!(after.moisture_mu_interior, 2.0);

    let after = round_trip(&base, &Din4108Mutation::ChangeEnvelopeAreaM2(change_envelope_area_m2::ChangeEnvelopeAreaM2 { new_envelope_area_m2: 150.0 }));
    assert_eq!(after.envelope_area_m2, 150.0);

    let after = round_trip(&base, &Din4108Mutation::ChangeBb2DetailsConform(change_bb2_details_conform::ChangeBb2DetailsConform { new_bb2_details_conform: false }));
    assert!(!after.bb2_details_conform);

    let after = round_trip(&base, &Din4108Mutation::ChangeApplicationType(change_application_type::ChangeApplicationType { new_application_type: "NDEO".into() }));
    assert_eq!(after.application_type, "NDEO");

    let after = round_trip(&base, &Din4108Mutation::ChangeDeclaredApplicationClass(change_declared_application_class::ChangeDeclaredApplicationClass { new_declared_application_class: "kh".into() }));
    assert_eq!(after.declared_application_class, "kh");
}

#[semio_framework_async_macros::async_test]
async fn insert_remove_layer_round_trips() {
    let base = Din4108Snapshot::default();
    let new_layer = LayerDocument { thickness_m: 0.05, lambda_w_mk: 0.04 };

    let insert = Din4108Mutation::InsertLayer(insert_layer::InsertLayer { index: 1, layer: new_layer.clone() });
    let after_insert = round_trip(&base, &insert);
    assert_eq!(after_insert.layers.len(), base.layers.len() + 1);
    assert_eq!(after_insert.layers[1], new_layer);

    let undo = insert.inverse(&base);
    assert_eq!(undo, vec![Din4108Mutation::RemoveLayer(remove_layer::RemoveLayer { index: 1 })]);

    let remove = Din4108Mutation::RemoveLayer(remove_layer::RemoveLayer { index: 0 });
    let after_remove = round_trip(&base, &remove);
    assert_eq!(after_remove.layers.len(), base.layers.len() - 1);
    assert_eq!(after_remove.layers[0], base.layers[1]);
}

#[semio_framework_async_macros::async_test]
async fn remove_layer_of_an_out_of_range_index_is_rejected() {
    let base = Din4108Snapshot::default();
    let remove = Din4108Mutation::RemoveLayer(remove_layer::RemoveLayer { index: 99 });
    assert!(remove.inverse(&base).is_empty(), "removing an absent index has nothing to undo");
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &remove).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_layers_round_trips() {
    let base = Din4108Snapshot::default();
    assert!(base.layers.len() >= 2, "fixture must have at least two layers to exercise reorder");

    let reorder = Din4108Mutation::ReorderLayers(reorder_layers::ReorderLayers { from: 0, to: 1 });
    let after = round_trip(&base, &reorder);
    assert_eq!(after.layers[0], base.layers[1]);
    assert_eq!(after.layers[1], base.layers[0]);
}

#[semio_framework_async_macros::async_test]
async fn change_layer_thickness_and_lambda_round_trip() {
    let base = Din4108Snapshot::default();

    let thickness = Din4108Mutation::ChangeLayerThickness(change_layer_thickness::ChangeLayerThickness { index: 0, new_thickness_m: 0.3 });
    let after = round_trip(&base, &thickness);
    assert_eq!(after.layers[0].thickness_m, 0.3);
    assert_eq!(after.layers[0].lambda_w_mk, base.layers[0].lambda_w_mk);

    let lambda = Din4108Mutation::ChangeLayerLambda(change_layer_lambda::ChangeLayerLambda { index: 0, new_lambda_w_mk: 0.9 });
    let after = round_trip(&base, &lambda);
    assert_eq!(after.layers[0].lambda_w_mk, 0.9);

    let missing = Din4108Mutation::ChangeLayerThickness(change_layer_thickness::ChangeLayerThickness { index: 99, new_thickness_m: 1.0 });
    assert!(missing.inverse(&base).is_empty(), "changing an absent index has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(Din4108Mutation::kinds().len(), 22);
    let mutation = Din4108Mutation::ChangeCategory(change_category::ChangeCategory { new_category: "x".into() });
    assert_eq!(mutation.semantics().kind, "change-category");
    assert_eq!(mutation.semantics().record, "ChangedCategory");
    assert!(mutation.target().is_empty());

    let indexed = Din4108Mutation::RemoveLayer(remove_layer::RemoveLayer { index: 2 });
    assert_eq!(indexed.semantics().kind, "remove-layer");
    assert_eq!(indexed.target(), vec!["2".to_string()]);
}
