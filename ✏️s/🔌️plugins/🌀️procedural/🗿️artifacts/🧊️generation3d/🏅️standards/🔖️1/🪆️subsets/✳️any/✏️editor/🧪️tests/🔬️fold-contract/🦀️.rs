use super::*;
use crate::editor::generation3d::config::SetSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::generation3d_fixture_operations;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::standards::v1::subsets::any::schema::{
    default_snapshot, example_snapshot, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry};
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
fn live_widget_ids(app: &context::Generation3dApp) -> std::collections::BTreeSet<String> {
    context::snapshot(app).fixture.widgets.iter().map(crate::widget_id).map(str::to_string).collect()
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
        assert_eq!(base.fixture.widgets, target.fixture.widgets, "example {example_id}: replaying the authored gesture against the running post root does not reach the example's own widgets — in THIS order");
        assert_eq!(base.fixture.synapses, target.fixture.synapses, "example {example_id}: the replayed gesture does not reach the example's own synapses");
        assert_eq!(base.fixture.layout, target.fixture.layout, "example {example_id}: the replayed gesture leaves the PREVIOUS example's orphaned layout overrides behind");
        assert_eq!(base.fixture.camera, Generation3dSnapshotRead::new(example_snapshot(previous).expect("bundled example snapshot")).fixture.camera, "example {example_id}: the artifact lane must NOT author the camera (`mutations::tests::fixture_ops_ignore_camera`) — `config_after_document_load` carries it on the Config lane");
        assert_eq!(base.fixture.schema, target.fixture.schema, "example {example_id}: the replayed gesture does not reach the example's own schema");
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

/// 🎬️ Boots ONE app, publishes `setActiveExample` through the real retained path (the host's bounded
/// publication/ACK protocol), and asserts the document actually became that example's fixture.
/// Before the fold-envelope fix every one of these retired without publishing
/// (`typed-operation publication turn=… operations=["27:Retiring:true:true"] latest_wins_empty=true`).
/// 🎯️ One law PER bundled example, and the body EXPANDED into each — never one walk over all eight
/// and never an awaited helper: a live `Generation3dApp` plus the host's publication machine is a
/// multi-megabyte future, so nesting even two of those states in one test future overflows the test
/// thread's 8 MiB stack. Each example therefore gets its own test thread holding exactly one, and the
/// picker's whole roster stays covered.
macro_rules! set_active_example_publishes {
    ($($name:ident => $example:expr,)+) => {
        $(
            #[semio_framework_async_macros::async_test]
            async fn $name() {
                let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
                let mut app = app_with_registry().await;
                let before = live_widget_ids(&app);
                app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": $example }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("setActiveExample dispatches");
                let receipt = context::settle(&mut app).await;
                assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "{} published a fault lane", $example);
                let after = live_widget_ids(&app);
                assert_eq!(after, bundled_widget_ids($example), "{} did not reach the store: the published document is not the example's fixture", $example);
                assert!(after != before || $example == PROCEDURAL_EXAMPLE_HEX_COLUMN, "{} left the document untouched", $example);
                eprintln!("[DEBUG] setActiveExample {} published: lanes={:?} effects={} widgets={}", $example, receipt.lanes, receipt.effects.len(), after.len());
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
            }
        )+
    };
}

set_active_example_publishes! {
    set_active_example_publishes_hex_column => PROCEDURAL_EXAMPLE_HEX_COLUMN,
    set_active_example_publishes_rect_extrude => PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    set_active_example_publishes_sphere_torus => PROCEDURAL_EXAMPLE_SPHERE_TORUS,
    set_active_example_publishes_box_fillet => PROCEDURAL_EXAMPLE_BOX_FILLET,
    set_active_example_publishes_sphere_box_fuse => PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    set_active_example_publishes_face_sweep_extrude => PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
    set_active_example_publishes_rectangle_wire => PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
    set_active_example_publishes_box_shell => PROCEDURAL_EXAMPLE_BOX_SHELL,
}

/// 🕹️ The framework-owned local-interaction route publishes on the same machine: `interactionSelect`
/// settles through the host's bounded publication protocol and the selection is readable afterwards.
/// It failed at runtime with the SAME fold-contract message as `setActiveExample`, because the whole
/// typed-operation lane of the instance was fail-closed behind the app's rejected candidate.
#[semio_framework_async_macros::async_test]
async fn interaction_select_publishes_through_the_retained_typed_path() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let node_id = context::snapshot(&app).fixture.widgets.first().map(crate::widget_id).expect("default fixture node").to_string();
    // 🧯️ `interactionSelect` is a framework-reserved TOOL JOB (`FrameworkInteractionSelectJob`), so
    // `handle_action` alone only admits its `Effect::SpawnJob` — the selection lands when the host
    // drives that job, which is what `select_graph` does. A `settle_registered_typed_operation`
    // cannot: it settles a typed operation, and the reserved spawn is not one.
    let settled = context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    eprintln!("[DEBUG] interactionSelect published: effects={}", settled.requested_effects.len());
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
