use super::*;
use crate::schema::mutations::{ChangeExaggeration, ChangeImportedFeatures};

#[semio_framework_async_macros::async_test]
async fn change_exaggeration_and_change_imported_features_invert_to_the_prior_field_value() {
    let snapshot = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into(), ..Default::default() };
    assert_eq!(GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 9.0 }).inverse(&snapshot), vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 1.5 })]);
    assert_eq!(
        GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "{}".into() }).inverse(&snapshot),
        vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "null".into() })]
    );
}

#[semio_framework_async_macros::async_test]
async fn change_exaggeration_obeys_the_inverse_and_diff_absorb_laws() {
    let base = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into(), ..Default::default() };
    let mutation = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 4.0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 8.0 }).diff(&base).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_imported_features_obeys_the_inverse_law() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: "null".into(), ..Default::default() };
    let mutation = GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "{}".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}
