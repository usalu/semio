use super::*;
use crate::editor::generation3d::config::SetSun;



/// 🧺️ Replays the store's OWN fold arithmetic over one lane's authored gesture: every item costs its
/// single forward row plus every row its inverse yields, and both the per-item gate in
/// `ArtifactStore::fold_batch_item` and the gesture-wide gate in `PreflightingCommit` compare that
/// against the MERGED footprint the preflights declared. Returns `(rows, declared)`.
fn folded_rows_against_declaration(items: &[(usize, usize)]) -> (usize, usize) {
    let declared = items.iter().map(|(_, work_items)| work_items).sum();
    let rows = items.iter().map(|(inverse_rows, _)| inverse_rows + 1).sum();
    (rows, declared)
}
#[test]
fn set_active_example_config_gesture_fits_its_declared_fold_envelope() {
    let base = Generation3dConfig::default();
    let mutation = Generation3dConfigMutation::SetSun(SetSun { json: "{\"azimuth\":1.0}".into() });
    let footprint = admit_generation3d_config_mutation(&mutation).expect("the config sun mutation is admissible");
    let inverse_rows = ::protocol::Mutation::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len();
    assert_eq!(inverse_rows, 1, "a config sun change is point-invertible");
    assert!(inverse_rows + 1 <= footprint.work_items, "one config item folds {} rows against a declared envelope of {}", inverse_rows + 1, footprint.work_items);
}

#[test]
fn a_one_work_item_declaration_cannot_carry_a_point_invertible_item() {
    let (rows, declared) = folded_rows_against_declaration(&[(1, 1)]);
    assert!(rows > declared, "a work_items: 1 declaration must not admit a forward plus an inverse row");
    let (rows, declared) = folded_rows_against_declaration(&[(1, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS)]);
    assert!(rows <= declared, "the invertible declaration must carry exactly one forward and one inverse row");
    assert_eq!(admit_generation3d_artifact_mutation(&Generation3dMutation::ChangeSchema(crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema { new_schema: "x".into() })).expect("admissible").work_items, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
}
