use super::*;
use crate::editor::gis2d::unit_tests::context::{app, close, dispatch};
use crate::editor::gis2d::Gis2dCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_the_document_without_history() {
    let mut app = app().await;
    assert!(!app.snapshot().expect("projection").positions.is_empty());
    dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() })).await;
    assert!(app.snapshot().expect("projection").positions.is_empty(), "the empty example loads an empty map");
    dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEFAULT_EXAMPLE_ID.into() })).await;
    assert!(!app.snapshot().expect("projection").positions.is_empty(), "the demo example loads its features");
    close(&mut app);
}

/// 🧬️ `setActiveExample` is a document load: it emits one load-document effect and no mutation rows, and an example the
/// open document already carries emits nothing (the shell replays the action on every boot).
#[semio_framework_async_macros::async_test]
async fn set_active_example_is_a_load_effect_and_idempotent() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
    assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    let demo = example_document(DEFAULT_EXAMPLE_ID).expect("the demo resolves");
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&demo, &history);
    let no_config = semio_framework_plugin::NoConfig {};
    let config = semio_framework_plugin::ConfigView { snapshot: &no_config, window: None };
    let same = set_active_example::handle(&set_active_example::SetActiveExample { example_id: DEFAULT_EXAMPLE_ID.into() }, &view, &config).expect("handles");
    assert!(same.artifact_mutations.is_empty() && same.effects.is_empty(), "an already loaded example emits nothing");
    let cleared = set_active_example::handle(&set_active_example::SetActiveExample { example_id: String::new() }, &view, &config).expect("handles");
    assert!(cleared.artifact_mutations.is_empty(), "an example switch writes no mutation rows");
    assert!(matches!(cleared.effects.as_slice(), [semio_framework_plugin::Effect::LoadDocument { .. }]), "it loads the document instead");
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
    assert_eq!(demo, crate::standards::v1::subsets::any::io::text::snapshot::default_document(), "the demo id resolves to the bundled reuse map itself");
    assert!(!demo.routes.is_empty(), "the resolved example carries real route content");
    assert!(example_document("reuse-map").is_err(), "an id outside the catalogue faults instead of loading the demo");
}

/// 📝️ The palette's `exampleId` options are projected from the same catalogue, so a new
/// `📚️examples` facet can never be offered by one and rejected by the other.
#[semio_framework_async_macros::async_test]
async fn the_manifest_stages_exactly_the_catalogue_ids() {
    let definition = crate::editor::gis2d::create_gis2d_app();
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
