use super::*;
use crate::editor::generation3d::config::SetSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::generation3d_fixture_operations;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::standards::v1::subsets::any::schema::{
    default_snapshot, example_snapshot, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use crate::editor::generation3d::testkit::{self, app_with_registry};
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::PluginApp;

/// 📚️ Every bundled example the picker offers — the same eight `each_example_loads_distinct_fixture_and_preview_geometry` walks.
const BUNDLED_EXAMPLES: [&str; 8] = [
    PROCEDURAL_EXAMPLE_HEX_COLUMN,
    PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    PROCEDURAL_EXAMPLE_SPHERE_TORUS,
    PROCEDURAL_EXAMPLE_BOX_FILLET,
    PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
    PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
    PROCEDURAL_EXAMPLE_BOX_SHELL,
];

/// 🪪️ The widget-id signature of a bundled example's authored fixture. A synchronous frame on
/// purpose: a `Generation3dSnapshotRead` carries the whole `FlowFixture`, and parking one in an
/// `async fn` that also holds a live app makes the test future large enough to overflow the test
/// thread's stack the moment the eight-example walk awaits inside it.
fn bundled_widget_ids(example_id: &str) -> std::collections::BTreeSet<String> {
    let read = Generation3dSnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
    read.fixture.widgets.iter().map(crate::widget_id).map(str::to_string).collect()
}

/// 🪪️ The same signature for the app's LIVE document — synchronous for the same reason.
fn live_widget_ids(app: &testkit::Generation3dApp) -> std::collections::BTreeSet<String> {
    testkit::snapshot(app).fixture.widgets.iter().map(crate::widget_id).map(str::to_string).collect()
}

/// 🧺️ Replays the store's OWN fold arithmetic over one lane's authored gesture: every item costs its
/// single forward row plus every row its inverse yields, and both the per-item gate in
/// `ArtifactStore::fold_batch_item` and the gesture-wide gate in `PreflightingCommit` compare that
/// against the MERGED footprint the preflights declared. Returns `(rows, declared)`.
fn folded_rows_against_declaration(items: &[(usize, usize)]) -> (usize, usize) {
    let declared = items.iter().map(|(_, work_items)| work_items).sum();
    let rows = items.iter().map(|(inverse_rows, _)| inverse_rows + 1).sum();
    (rows, declared)
}

/// 🧾️ The Artifact lane of `setActiveExample` declares a fold envelope that its own candidates fit —
/// per item AND for the whole gesture — for every hop of the picker cycle, so all eight bundled
/// fixtures appear as both a base and a target. The cycle, not eight diffs against the default: the
/// default document IS the hex-column fixture, so `default → hex-column` authors nothing at all, and
/// the second hop is exactly the boot-time gesture. This is the law the runtime broke:
/// a declared `work_items: 1` leaves ZERO inverse capacity, so `fold_batch_item` answered
/// `batched item candidate failed its exact fixed fold contract` for the very first action of the app
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn set_active_example_artifact_gesture_fits_its_declared_fold_envelope_for_every_example() {
    let boot = Generation3dSnapshotRead::new(default_snapshot());
    let hex = Generation3dSnapshotRead::new(example_snapshot(PROCEDURAL_EXAMPLE_HEX_COLUMN).expect("bundled example snapshot"));
    assert!(
        generation3d_fixture_operations(&boot.fixture, &hex.fixture).is_empty(),
        "the boot document IS the hex-column fixture, so re-picking it authors no gesture — every OTHER pick in the cycle below is the one that publishes"
    );
    for (index, example_id) in BUNDLED_EXAMPLES.into_iter().enumerate() {
        let previous = BUNDLED_EXAMPLES[(index + BUNDLED_EXAMPLES.len() - 1) % BUNDLED_EXAMPLES.len()];
        let target = Generation3dSnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
        let mut base = Generation3dSnapshotRead::new(example_snapshot(previous).expect("bundled example snapshot"));
        let operations = generation3d_fixture_operations(&base.fixture, &target.fixture);
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
        assert_eq!(base.fixture, target.fixture, "example {example_id}: replaying the authored gesture against the running post root does not reach the example's own fixture");
        let (rows, declared) = folded_rows_against_declaration(&items);
        assert!(rows <= declared, "example {example_id}: the staged gesture folds {rows} rows against a declared envelope of {declared}");
        eprintln!("[DEBUG] fold envelope {previous} -> {example_id}: {} items, {rows} rows, {declared} declared", items.len());
    }
}

/// 🧾️ The Config lane of the same command is the single-item case — the one that fail-closed FIRST at
/// runtime, because a one-mutation gesture leaves the per-item gate no slack at all.
#[test]
fn set_active_example_config_gesture_fits_its_declared_fold_envelope() {
    let base = Generation3dConfig::default();
    let mutation = Generation3dConfigMutation::SetSnapshot(SetSnapshot { config: Generation3dConfig { sun_json: "{\"azimuth\":1.0}".into(), ..base.clone() } });
    let footprint = admit_generation3d_config_mutation(&mutation).expect("the config snapshot mutation is admissible");
    let inverse_rows = ::protocol::Mutation::inverse(&mutation, &base).len();
    assert_eq!(inverse_rows, 1, "a config snapshot swap is point-invertible");
    assert!(inverse_rows + 1 <= footprint.work_items, "one config item folds {} rows against a declared envelope of {}", inverse_rows + 1, footprint.work_items);
}

/// 🚫️ The hostile control, and the exact shape of the defect: a declaration of ONE work item cannot
/// carry a point-invertible item, so every single-mutation durable gesture fails closed. Kept as a law
/// so the literal can never come back through either lane.
#[test]
fn a_one_work_item_declaration_cannot_carry_a_point_invertible_item() {
    let (rows, declared) = folded_rows_against_declaration(&[(1, 1)]);
    assert!(rows > declared, "a work_items: 1 declaration must not admit a forward plus an inverse row");
    let (rows, declared) = folded_rows_against_declaration(&[(1, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS)]);
    assert!(rows <= declared, "the invertible declaration must carry exactly one forward and one inverse row");
    assert_eq!(generation3d_one_item_footprint(64).work_items, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
    assert_eq!(admit_generation3d_artifact_mutation(&Generation3dMutation::ChangeSchema(crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema { new_schema: "x".into() })).expect("admissible").work_items, generation3d_one_item_footprint(0).work_items);
}

/// 🔒️ Both durable lanes declare through the ONE shared builder — not two literals that can drift
/// apart, and never a bare `ArtifactStoreOneItemFootprint { work_items: … }` struct literal.
#[test]
fn both_durable_lanes_declare_through_the_one_shared_footprint_builder() {
    let source = include_str!("../../🦀️.rs");
    assert_eq!(source.matches("fn generation3d_one_item_footprint(").count(), 1, "the shared fold-envelope builder is declared exactly once");
    assert_eq!(source.matches("Ok(generation3d_one_item_footprint(retained_bytes))").count(), 2, "both the artifact and the config preflight declare through the shared builder");
    assert!(!source.contains("ArtifactStoreOneItemFootprint { work_items"), "a durable lane regained a hand-written work-items literal");
}

/// 🎯️ `setActiveExample` PUBLISHES for every bundled example, driven through the real retained typed
/// path (`dispatch_typed` + the host's bounded publication/ACK protocol), and the document actually
/// swaps. Before the fold-envelope fix every one of these retired without publishing
/// (`typed-operation publication turn=… operations=["27:Retiring:true:true"] latest_wins_empty=true`).
#[semio_framework_async_macros::async_test]
async fn set_active_example_publishes_and_swaps_the_document_for_every_bundled_example() {
    let _serial = test_support::lock();
    let mut signatures = std::collections::BTreeSet::new();
    for example_id in BUNDLED_EXAMPLES {
        let mut app = app_with_registry().await;
        let before = live_widget_ids(&app);
        let receipt = testkit::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() })).await;
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "{example_id} published a fault lane");
        let after = live_widget_ids(&app);
        assert_eq!(after, bundled_widget_ids(example_id), "{example_id} did not reach the store: the published document is not the example's fixture");
        assert!(after != before || example_id == PROCEDURAL_EXAMPLE_HEX_COLUMN, "{example_id} left the document untouched");
        assert!(signatures.insert(format!("{after:?}")), "duplicate published fixture signature for {example_id}");
        eprintln!("[DEBUG] setActiveExample {example_id} published: lanes={:?} effects={} widgets={}", receipt.lanes, receipt.effects.len(), after.len());
    }
}

/// 🕹️ The framework-owned local-interaction route publishes on the same machine: `interactionSelect`
/// settles through the host's bounded publication protocol and the selection is readable afterwards.
/// It failed at runtime with the SAME fold-contract message as `setActiveExample`, because the whole
/// typed-operation lane of the instance was fail-closed behind the app's rejected candidate.
#[semio_framework_async_macros::async_test]
async fn interaction_select_publishes_through_the_retained_typed_path() {
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let node_id = testkit::snapshot(&app).fixture.widgets.first().map(crate::widget_id).expect("default fixture node").to_string();
    let targets = serde_json::to_string(&vec![semio_framework_plugin::InteractionTarget { granularity: "node".into(), id: node_id.clone() }]).expect("selection targets");
    let args: dsl::DslValue = serde_json::json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" }).into();
    app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&args), &semio_framework_plugin::testkit::meta("local")).await.expect("interactionSelect dispatches");
    let receipt = testkit::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "interactionSelect published a fault lane");
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    eprintln!("[DEBUG] interactionSelect published: lanes={:?} completions={}", receipt.lanes, receipt.completions);
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}
