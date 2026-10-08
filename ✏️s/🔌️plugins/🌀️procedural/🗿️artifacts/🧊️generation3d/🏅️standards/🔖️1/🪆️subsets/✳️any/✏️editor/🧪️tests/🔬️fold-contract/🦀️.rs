use super::*;
use crate::editor::generation3d::config::SetSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::generation3d_document_replacement;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::standards::v1::subsets::any::schema::{PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS};
use crate::standards::v1::subsets::any::io::text::snapshot::{default_snapshot, example_snapshot};

const BUNDLED_EXAMPLES: [&str; 8] = [PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_TORUS, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_BOX_SHELL];


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
fn set_active_example_artifact_gesture_fits_its_declared_fold_envelope_for_every_example() {
    let boot = Generation3dSnapshotRead::new(default_snapshot());
    let hex = Generation3dSnapshotRead::new(example_snapshot(PROCEDURAL_EXAMPLE_HEX_COLUMN).expect("bundled example snapshot"));
    assert!(
        generation3d_document_replacement(&boot, &hex).is_empty(),
        "the boot document IS the hex-column host_snapshot, so re-picking it authors no gesture — every OTHER pick in the cycle below is the one that publishes"
    );
    for (index, example_id) in BUNDLED_EXAMPLES.into_iter().enumerate() {
        let previous = BUNDLED_EXAMPLES[(index + BUNDLED_EXAMPLES.len() - 1) % BUNDLED_EXAMPLES.len()];
        let target = Generation3dSnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
        let mut base = Generation3dSnapshotRead::new(example_snapshot(previous).expect("bundled example snapshot"));
        let operations = generation3d_document_replacement(&base, &target);
        assert!(!operations.is_empty(), "example {example_id} authored an empty artifact gesture");
        let mut items = Vec::with_capacity(operations.len());
        for mutation in operations {
            let footprint = admit_generation3d_artifact_mutation(&mutation).expect("every authored example mutation is admissible");
            // 🧵️ `prepare_generation3d_artifact` IS the body of the store's one-item preparation, so
            // the inverse counted here is the very list `fold_batch_item` measures — never a
            // re-derivation that could disagree with the running post root the store folds against.
            let (post, inverse, mutation) = prepare_generation3d_artifact(&base, mutation).expect("every authored example mutation prepares against its running base");
            let inverse_rows = inverse.len();
            for row in inverse {
                row.retire_cold();
            }
            mutation.retire_cold();
            assert!(
                inverse_rows + 1 <= footprint.work_items,
                "example {example_id}: one item folds {} rows against a declared envelope of {}",
                inverse_rows + 1,
                footprint.work_items
            );
            items.push((inverse_rows, footprint.work_items));
            base = Generation3dSnapshotRead::new(post);
        }
        assert_eq!(base.host_snapshot.widgets, target.host_snapshot.widgets, "example {example_id}: replaying the authored gesture against the running post root does not reach the example's own widgets — in THIS order");
        assert_eq!(base.host_snapshot.synapses, target.host_snapshot.synapses, "example {example_id}: the replayed gesture does not reach the example's own synapses");
        assert_eq!(base.host_snapshot.layout, target.host_snapshot.layout, "example {example_id}: the replayed gesture leaves the PREVIOUS example's orphaned layout overrides behind");
        assert_eq!(base.host_snapshot.camera, Generation3dSnapshotRead::new(example_snapshot(previous).expect("bundled example snapshot")).host_snapshot.camera, "example {example_id}: the artifact lane must NOT author the camera (`mutations::tests::document_replacement_ignores_the_camera`) — `config_after_document_load` carries it on the Config lane");
        assert_eq!(base.host_snapshot.schema, target.host_snapshot.schema, "example {example_id}: the replayed gesture does not reach the example's own schema");
        let (rows, declared) = folded_rows_against_declaration(&items);
        assert!(rows <= declared, "example {example_id}: the staged gesture folds {rows} rows against a declared envelope of {declared}");
        eprintln!("fold envelope {previous} -> {example_id}: {} items, {rows} rows, {declared} declared", items.len());
    }
}

#[test]
fn set_active_example_config_gesture_fits_its_declared_fold_envelope() {
    let base = Generation3dConfig::default();
    let mutation = Generation3dConfigMutation::SetSnapshot(SetSnapshot { config: Generation3dConfig { sun_json: "{\"azimuth\":1.0}".into(), ..base.clone() } });
    let footprint = admit_generation3d_config_mutation(&mutation).expect("the config snapshot mutation is admissible");
    let inverse_rows = ::protocol::Mutation::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len();
    assert_eq!(inverse_rows, 1, "a config snapshot swap is point-invertible");
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
