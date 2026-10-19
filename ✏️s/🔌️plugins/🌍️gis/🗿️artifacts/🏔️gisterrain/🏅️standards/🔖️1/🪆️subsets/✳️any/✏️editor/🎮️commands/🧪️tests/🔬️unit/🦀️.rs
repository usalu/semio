//! 🧪️ Aggregated unit tests of the `commands` leaves, one module per leaf.

mod example {
use crate::mutations::change_exaggeration::ChangeExaggeration;
use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
use super::super::example::*;
use crate::editor::gis3d::unit_tests::context::{app, close, dispatch, history_verb};
use crate::editor::gis3d::Gis3dCommand;
use semio_framework_plugin::PluginApp;

/// 🎬️ The catalogue is real content addressed by id, not an empty-vs-non-empty switch: the declared
/// `📚️examples/🎬️demo` facet resolves to the bundled Liège reuse terrain, and an id nobody declared
/// is a fault rather than a silent fallback onto whichever example happens to be bundled.
#[semio_framework_async_macros::async_test]
async fn the_example_catalogue_resolves_declared_ids_and_faults_on_the_rest() {
    let catalogue = example_catalogue();
    let ids: Vec<&str> = catalogue.iter().map(|source| source.id()).collect();
    assert_eq!(ids, [crate::examples::demo::ID], "the catalogue is exactly the subset's declared example facets");
    let none = example_document("").expect("the empty id is the catalogue's none arm");
    assert_eq!(none, crate::standards::v1::subsets::any::io::text::snapshot::empty_gis_terrain_snapshot(), "the none arm is the flat unimported terrain");
    let demo = example_document(crate::examples::demo::ID).expect("the declared example resolves");
    assert_eq!(demo, crate::schema::default_terrain_document(), "the demo id resolves to the bundled reuse terrain itself");
    assert_eq!(demo.exaggeration, 1.5, "the resolved example carries the authored relief, not the flat default");
    assert!(example_document("reuse-terrain").is_err(), "an id outside the catalogue faults instead of loading the demo");
}

/// 📝️ The palette's `exampleId` options are projected from the same catalogue, so a new
/// `📚️examples` facet can never be offered by one and rejected by the other.
#[semio_framework_async_macros::async_test]
async fn the_manifest_stages_exactly_the_catalogue_ids() {
    let definition = crate::editor::gis3d::create_gis3d_app();
    let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
    let arg = action.args.iter().find(|arg| arg.id == "exampleId").expect("the example choice is a declared arg");
    let semio_framework_plugin::ArgSchema::String { options, .. } = &arg.schema else { panic!("exampleId is staged as a string choice: {arg:?}") };
    let staged: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
    let catalogue = example_catalogue();
    let declared: Vec<&str> = catalogue.iter().map(|source| source.id()).collect();
    assert_eq!(staged, declared, "every staged option is a catalogue id and every catalogue id is staged");
    for value in staged {
        assert!(example_document(value).is_ok(), "the palette must never stage an id `set_active_example` rejects: {value}");
    }
}

/// 🧬️ `setActiveExample` replaces document content with the artifact's own authored leaves, so it
/// MUST be declared as a Mutation. Under the real registry the View/Shell → emits-operations guard
/// rejects a mis-declaration; this proves the declaration lets the document-replacing edit through.
/// It is ALSO the gate that makes the pane's navbar example picker exist at all: the React host's
/// `appSwitchesExamples` reads exactly this action, and without it `exampleOptions` is empty and
/// `resolveBootExampleId` announces nothing (ticket 26/09/19, gis3d's empty-picker defect).
#[semio_framework_async_macros::async_test]
async fn set_active_example_is_a_declared_mutation_with_a_staged_argument() {
    let definition = crate::editor::gis3d::create_gis3d_app();
    let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits document-mutating operations, so it is a Mutation");
    assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
}

/// ↩️ Empty → demo → undo round trips the document through real invertible leaves: the boot document
/// IS the demo, so clearing it publishes a real edit, reloading the demo restores the authored
/// relief, and ONE undo returns to the cleared document rather than to a mid-flight value.
#[semio_framework_async_macros::async_test]
async fn set_active_example_empty_then_demo_round_trips_and_undoes() {
    let mut app = app().await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5, "the terrain editor boots on its curated example");
    let cleared = dispatch(&mut app, Gis3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() })).await;
    assert!(cleared.lanes.iter().any(|lane| matches!(lane, semio_framework_plugin::app::TypedOperationResultLane::Artifact)), "clearing a curated document publishes on the artifact lane: {:?}", cleared.lanes);
    drop(cleared);
    assert_eq!(app.snapshot().expect("projection").exaggeration, crate::standards::v1::subsets::any::io::text::snapshot::empty_gis_terrain_snapshot().exaggeration);
    dispatch(&mut app, Gis3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEFAULT_EXAMPLE_ID.into() })).await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5, "the declared example restores the authored relief");
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, crate::standards::v1::subsets::any::io::text::snapshot::empty_gis_terrain_snapshot().exaggeration, "undo returns to the cleared document");
    close(&mut app);
}

/// 🚫️ Re-applying the example the document already holds emits nothing, and the none arm emits
/// exactly the fields that actually differ — the same `mutation.no-op` discipline
/// `change-exaggeration` states for itself, so a one-click re-select never writes an empty edit into
/// the history. Measured on the pure reducer, so it pins the emission rule itself rather than
/// whatever a store round trip would make of it.
#[semio_framework_async_macros::async_test]
async fn the_reducer_emits_only_the_fields_that_differ() {
    let document = crate::schema::default_terrain_document();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&document, &history);
    let config = semio_framework_plugin::NoConfig {};
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };

    let same = set_active_example::handle(&set_active_example::SetActiveExample { example_id: DEFAULT_EXAMPLE_ID.into() }, &doc, &cfg).expect("re-selecting the live example reduces");
    assert!(same.artifact_mutations.is_empty(), "the boot document already IS the demo, so re-selecting it changes no field");

    let cleared = set_active_example::handle(&set_active_example::SetActiveExample { example_id: String::new() }, &doc, &cfg).expect("the none arm reduces");
    assert_eq!(cleared.artifact_mutations.len(), 1, "only `exaggeration` differs between the demo and the flat terrain; `importedMap` is empty in both");
    assert!(matches!(cleared.artifact_mutations.first(), Some(crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation::ChangeExaggeration(_))), "the differing field is emitted through its own authored leaf: {:?}", cleared.artifact_mutations);
}
}

mod exaggeration {
use super::super::exaggeration::*;
use crate::editor::gis3d::modes::view::windows::terrain;
use crate::editor::gis3d::unit_tests::context::{app, close, dispatch, history_verb, main_window_view, render, Gis3dApp};
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use crate::editor::gis3d::Gis3dCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn seeds_exaggeration_from_the_terrain_fixture() {
    let mut app = app().await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5);
    close(&mut app);
}

//#region 🎚️Scrub
/// 🎚️ The dispatching Terrain window's meta, the window a slider press belongs to.
fn window_meta() -> semio_framework_plugin::ActionMeta {
    semio_framework_plugin::ActionMeta { view_state: Some(main_window_view()), ..meta("local") }
}

fn number(value: f64) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::float(value)
}

/// 🎚️ One dispatch of the exaggeration slider as the hosts send it: `{value, gesture, commit}`, or a host cancel
/// `{gesture, abort}` that carries no value and is settled by the runtime itself.
async fn slide(app: &mut Gis3dApp, args: Vec<(&str, semio_framework_value::DslValue)>) {
    let aborting = args.iter().any(|(key, _)| *key == "abort");
    let args = semio_framework_value::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "setExaggeration", Some(args.clone()), &window_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("the slider dispatch is admitted");
    if !aborting {
        settle_registered_typed_operation(&mut app.0, window_meta().instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("the slider dispatch settles");
    }
}

async fn tick(app: &mut Gis3dApp, gesture: &str, value: f64) {
    slide(app, vec![("value", number(value)), ("gesture", semio_framework_value::DslValue::String(gesture.into())), ("commit", semio_framework_value::DslValue::Bool(false))]).await;
}

async fn release(app: &mut Gis3dApp, gesture: &str, value: f64) {
    slide(app, vec![("value", number(value)), ("gesture", semio_framework_value::DslValue::String(gesture.into())), ("commit", semio_framework_value::DslValue::Bool(true))]).await;
}

/// 🧾️ The history rows that carry a document edit, oldest first.
async fn edit_rows(app: &mut Gis3dApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    let mut rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

fn exaggeration(app: &Gis3dApp) -> f64 {
    app.snapshot().expect("projection").exaggeration
}

/// ⚖️ LAW: one slider press is ONE tool transaction — its ticks publish nothing (the committed document stays, the
/// render previews the value), the release publishes ONE edit whose one history row carries the press's
/// `TransactionRef` (tool `<appId>#setExaggeration`) and is labelled from the leaf in every language, and one undo
/// restores the value before the press.
#[semio_framework_async_macros::async_test]
async fn a_slider_press_is_one_transaction_one_edit_and_one_row() {
    let mut app = app().await;
    let rows_before = edit_rows(&mut app).await.len();
    let resting = render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await;
    tick(&mut app, "terrain.exaggeration:1", 2.0).await;
    tick(&mut app, "terrain.exaggeration:1", 2.5).await;
    assert_eq!(exaggeration(&app), 1.5, "ticks never touch the committed document");
    assert_eq!(edit_rows(&mut app).await.len(), rows_before, "ticks are no history");
    assert_ne!(render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await, resting, "the render previews the scrubbed value");
    release(&mut app, "terrain.exaggeration:1", 3.0).await;
    assert_eq!(exaggeration(&app), 3.0);
    let rows = edit_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 1, "one press, one edit, one row");
    let row = rows.last().expect("the press's row");
    let transaction = row.transaction.as_ref().expect("the row is the press's tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool.ends_with("#setExaggeration"), "{transaction:?}");
    assert_eq!(row.mutations.len(), 1, "one absolute leaf: the net value");
    assert_eq!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Change terrain exaggeration to 3");
    assert_eq!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Geländeüberhöhung auf 3 ändern");
    history_verb(&mut app, "undo").await;
    assert_eq!(exaggeration(&app), 1.5, "one undo restores the value before the press");
    close(&mut app);
}

/// ⚖️ LAW: a host cancel leaves zero trace — no edit, no row, the render back on the committed document — and a late
/// tick or release of the cancelled press stays silent.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_press_leaves_zero_trace() {
    let mut app = app().await;
    let rows_before = edit_rows(&mut app).await.len();
    let resting = render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await;
    tick(&mut app, "terrain.exaggeration:2", 4.0).await;
    slide(&mut app, vec![("gesture", semio_framework_value::DslValue::String("terrain.exaggeration:2".into())), ("abort", semio_framework_value::DslValue::String("blur".into()))]).await;
    assert_eq!(render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await, resting, "the preview is gone");
    release(&mut app, "terrain.exaggeration:2", 4.5).await;
    assert_eq!(exaggeration(&app), 1.5);
    assert_eq!(edit_rows(&mut app).await.len(), rows_before, "zero trace");
    close(&mut app);
}

/// ⚖️ LAW: two presses are two transactions — two edits, two rows, two distinct refs — and a dispatch without a
/// press (palette, agent) stays one plain edit.
#[semio_framework_async_macros::async_test]
async fn two_presses_are_two_transactions() {
    let mut app = app().await;
    let rows_before = edit_rows(&mut app).await.len();
    tick(&mut app, "terrain.exaggeration:3", 2.0).await;
    release(&mut app, "terrain.exaggeration:3", 2.2).await;
    tick(&mut app, "terrain.exaggeration:4", 3.0).await;
    release(&mut app, "terrain.exaggeration:4", 3.3).await;
    let rows = edit_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 2);
    let refs: Vec<_> = rows[rows_before..].iter().map(|row| row.transaction.clone().expect("a transaction row")).collect();
    assert_ne!(refs[0].id, refs[1].id, "two presses, two transactions");
    slide(&mut app, vec![("value", number(4.0))]).await;
    let rows = edit_rows(&mut app).await;
    assert_eq!((rows.len(), rows.last().and_then(|row| row.transaction.clone())), (rows_before + 3, None), "a dispatch without a press is a plain edit");
    assert_eq!(exaggeration(&app), 4.0);
    close(&mut app);
}

async fn history_edit(app: &mut Gis3dApp, verb: &str, args: Vec<(&str, semio_framework_value::DslValue)>) {
    let args = semio_framework_value::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let result = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, verb, Some(args.clone()), &window_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn time_travel_stage(app: &mut Gis3dApp) -> Option<semio_framework::kernel::HistoryTimeTravelStage> {
    app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
}

async fn pump_time_travel(app: &mut Gis3dApp, done: impl Fn(Option<semio_framework::kernel::HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel_stage(app).await) {
            return;
        }
        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel_stage(app).await);
}

/// ⚖️ LAW: time travel edits the press's committed value, never the slider: the absolute leaf superseded with another
/// value replays deterministically — the later press, itself absolute, still wins at the head, and withdrawing nothing
/// else the overwrite leaves exactly the head a fresh run of the edited presses reaches.
#[semio_framework_async_macros::async_test]
async fn a_committed_press_edited_in_history_replays_deterministically() {
    let mut app = app().await;
    release(&mut app, "terrain.exaggeration:5", 3.0).await;
    let edited = edit_rows(&mut app).await.last().and_then(|row| row.mutations.first().map(|mutation| mutation.mutation_id.clone())).expect("the press's mutation");
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", semio_framework_value::DslValue::String(edited))]).await;
    history_edit(&mut app, "historyEditInput", vec![("path", semio_framework_value::DslValue::String("/newExaggeration".into())), ("value", number(2.0))]).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(&mut app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
    assert_eq!(exaggeration(&app), 3.0, "reviewing never touches the committed document");
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    history_edit(&mut app, "historyEditCommit", vec![("choice", semio_framework_value::DslValue::String("overwrite".into()))]).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(exaggeration(&app), 2.0, "the overwrite folds the edited value");
    let mut fresh = crate::editor::gis3d::unit_tests::context::app().await;
    release(&mut fresh, "terrain.exaggeration:6", 2.0).await;
    assert_eq!(app.snapshot().expect("edited head"), fresh.snapshot().expect("fresh head"), "the edited log equals a fresh run of the edited press");
    close(&mut app);
    close(&mut fresh);
}
//#endregion 🎚️Scrub

#[semio_framework_async_macros::async_test]
async fn set_exaggeration_is_a_document_operation_not_config_state() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 })).await;
    assert_eq!(result.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Artifact).count(), 1, "exaggeration publishes exactly one undoable document operation");
    close(&mut app);
}
}

mod view {
use super::super::view::*;
use crate::editor::gis3d::unit_tests::context::{app, close, dispatch};
use crate::editor::gis3d::Gis3dCommand;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn camera_is_exact_window_config_and_emits_no_document_operation() {
    let mut app = app().await;
    let camera = dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: json!({ "position": [1.0, 1.0, 1.0] }).to_string() })).await;
    assert_eq!(camera.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count(), 1, "camera publishes exactly one window-config operation");
    assert!(!camera.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "camera never publishes document state");
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_camera_reaches_the_rendered_scene() {
    let mut app = app().await;
    dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: json!({ "position": [123.0, 1.0, 1.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string() })).await;
    assert!(crate::editor::gis3d::unit_tests::context::render(&mut app, crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE).await.contains("123"));
    close(&mut app);
}
}
