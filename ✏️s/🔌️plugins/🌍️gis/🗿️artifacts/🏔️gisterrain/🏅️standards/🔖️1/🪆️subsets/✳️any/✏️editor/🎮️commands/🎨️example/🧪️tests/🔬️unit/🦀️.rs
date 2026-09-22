use super::*;
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
    assert_eq!(none, crate::schema::empty_gis_terrain_snapshot(), "the none arm is the flat unimported terrain");
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
    assert_eq!(app.snapshot().expect("projection").exaggeration, crate::schema::empty_gis_terrain_snapshot().exaggeration);
    dispatch(&mut app, Gis3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEFAULT_EXAMPLE_ID.into() })).await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5, "the declared example restores the authored relief");
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, crate::schema::empty_gis_terrain_snapshot().exaggeration, "undo returns to the cleared document");
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
    assert_eq!(cleared.artifact_mutations.len(), 1, "only `exaggeration` differs between the demo and the flat terrain; `importedFeaturesJson` is empty in both");
    assert!(matches!(cleared.artifact_mutations.first(), Some(crate::op::GisTerrainMutation::ChangeExaggeration(_))), "the differing field is emitted through its own authored leaf: {:?}", cleared.artifact_mutations);
}
