use super::*;
use crate::editor::gis2d::unit_tests::context::{app, close, dispatch};
use crate::editor::gis2d::Gis2dCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn set_active_example_empty_then_reuse_round_trips_document() {
    let mut app = app().await;
    assert!(!app.snapshot().expect("projection").positions.is_empty());
    dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() })).await;
    assert!(app.snapshot().expect("projection").positions.is_empty());
    dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEFAULT_EXAMPLE_ID.into() })).await;
    assert!(!app.snapshot().expect("projection").positions.is_empty());
    let admitted = app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("undo");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app, admitted).await.expect("undo settles its reserved job");
    for _ in 0..10_000 {
        app.maintenance_step(1, 4_096).expect("undo maintenance");
        if app.snapshot().expect("projection").positions.is_empty() {
            break;
        }
        std::thread::yield_now();
    }
    assert!(app.snapshot().expect("projection").positions.is_empty(), "undo returns to the empty document");
    close(&mut app);
}

/// 🧬️ `setActiveExample` replaces document content with batched create/delete/replace-data
/// operations, so it MUST be declared as an Operation. Under the real registry the View/Shell →
/// emits-operations guard rejects a mis-declaration; this proves the corrected declaration lets
/// the document-replacing edit flow through without erroring.
#[semio_framework_async_macros::async_test]
async fn set_active_example_is_operation_under_registry_kind_discipline() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits document-mutating operations, so it is a Mutation");
    assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");

    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() })).await;
    assert!(result.artifact_publication_count() > 0, "clearing a non-empty example publishes at least one delete operation per removed feature");
    assert!(app.snapshot().expect("projection").positions.is_empty(), "the empty example clears every position feature");
    drop(result);
    close(&mut app);
}

/// 🎬️ The catalogue is real content addressed by id, not an empty-vs-non-empty switch: the declared
/// `📚️examples/🎬️demo` facet resolves to the bundled Liège reuse map, and an id nobody declared is a
/// fault rather than a silent fallback onto whichever example happens to be bundled.
#[semio_framework_async_macros::async_test]
async fn the_example_catalogue_resolves_declared_ids_and_faults_on_the_rest() {
    let catalogue = example_catalogue();
    let ids: Vec<&str> = catalogue.iter().map(|source| source.id()).collect();
    assert_eq!(ids, [crate::examples::demo::ID], "the catalogue is exactly the subset's declared example facets");
    assert!(example_document("").expect("the empty id is the catalogue's none arm").positions.is_empty());
    let demo = example_document(crate::examples::demo::ID).expect("the declared example resolves");
    assert_eq!(demo, crate::schema::default_document(), "the demo id resolves to the bundled reuse map itself");
    assert!(!demo.routes.is_empty(), "the resolved example carries real route content");
    assert!(example_document("reuse-map").is_err(), "an id outside the catalogue faults instead of loading the demo");
}

/// 📝️ The palette's `exampleId` options are projected from the same catalogue, so a new
/// `📚️examples` facet can never be offered by one and rejected by the other.
#[semio_framework_async_macros::async_test]
async fn the_manifest_stages_exactly_the_catalogue_ids() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
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
