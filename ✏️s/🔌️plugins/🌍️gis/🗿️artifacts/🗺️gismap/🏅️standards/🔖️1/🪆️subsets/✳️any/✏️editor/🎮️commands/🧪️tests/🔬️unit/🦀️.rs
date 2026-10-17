//! 🧪️ Aggregated unit tests of the `commands` leaves, one module per leaf.

mod example {
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
use super::super::example::*;
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
}

mod inference {
use semio_framework_plugin::kernel::Effect;
use super::super::inference::*;
use crate::editor::gis2d::unit_tests::context::{app, close, dispatch};
use crate::editor::gis2d::Gis2dCommand;

/// 💡️ Exactly one host-owned intent leaves the app, and it names nothing but the proposal kind.
#[semio_framework_async_macros::async_test]
async fn propose_bounds_region_emits_one_intent_and_no_document_state() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {})).await;
    assert_eq!(result.artifact_publication_count(), 0, "an inference intent never mutates the document");
    assert_eq!(result.effects().len(), 1);
    assert_eq!(result.effects()[0], Effect::RequestServiceOperation { owner: "gis".into(), service_id: "s.gis.gismap.inference".into(), action: "propose".into(), payload: semio_framework::DslValue::Object(Vec::new()) });
    drop(result);
    close(&mut app);
}

/// 🌐️ A Shell action never emits document operations — the registry's kind-discipline guard
/// rejects one that does.
#[semio_framework_async_macros::async_test]
async fn propose_bounds_region_is_a_shell_action_that_emits_no_operations() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == "proposeBoundsRegion").expect("proposeBoundsRegion declared");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Shell));
    let mut app = app().await;
    assert_eq!(dispatch(&mut app, Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {})).await.artifact_publication_count(), 0);
    close(&mut app);
}
}

mod shell {
use semio_framework_plugin::kernel::Effect;
use super::super::shell::*;
use crate::editor::gis2d::unit_tests::context::{app, close, dispatch};
use crate::editor::gis2d::Gis2dCommand;

#[semio_framework_async_macros::async_test]
async fn open_source_on_an_unknown_feature_emits_no_effect() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    assert!(!result.effects().iter().any(|effect| matches!(effect, Effect::OpenExternalUrl { .. })));
    drop(result);
    close(&mut app);
}

/// 🌐️ A Shell action never emits document operations — the registry's kind-discipline guard
/// rejects one that does.
#[semio_framework_async_macros::async_test]
async fn open_source_is_a_shell_action_that_emits_no_operations() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == "openSource").expect("openSource declared");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Shell));
    let mut app = app().await;
    assert_eq!(dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() })).await.artifact_publication_count(), 0);
    close(&mut app);
}
}

mod view {
use super::super::view::*;
use crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_BODY_COMPOSITE;
use crate::editor::gis2d::unit_tests::context::{app, close, dispatch, render_tiled_map_scene};
use crate::editor::gis2d::Gis2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_render_mode_is_view_state() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    assert_eq!(render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.render_mode, "vector");
    drop(result);
    close(&mut app);
}

/// 👁️ A representative View action mutates only config state, so under the real registry it
/// emits no operations and never trips the View → emits-operations guard.
#[semio_framework_async_macros::async_test]
async fn view_actions_emit_no_ops_under_registry_kind_discipline() {
    let mut app = app().await;
    let render_mode = dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() })).await;
    assert_eq!(render_mode.artifact_publication_count(), 0, "render mode is config state");
    let fit = dispatch(&mut app, Gis2dCommand::FitWorld(fit_world::FitWorld {})).await;
    assert_eq!(fit.artifact_publication_count(), 0, "framing the world only moves the config camera");
    drop(render_mode);
    drop(fit);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn toggling_a_layer_flips_its_visibility_in_the_rendered_scene() {
    let mut app = app().await;
    assert!(render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.layer_visibility_json.contains("\"water\":true"));
    dispatch(&mut app, Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() })).await;
    assert!(render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.layer_visibility_json.contains("\"water\":false"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn layer_stroke_scale_is_clamped_to_the_surface_crates_range() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 99.0 })).await;
    let strokes = render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.layer_stroke_scale_json;
    assert!(!strokes.contains("\"roads\":99"), "an out-of-range weight is clamped before it reaches the config");
    drop(strokes);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn camera_writes_straight_through_to_the_config() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":5,"y":6,"zoom":7}"#.into() })).await;
    assert!(render_tiled_map_scene(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.camera_json.contains("\"zoom\":7"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn focus_feature_on_an_unknown_id_emits_nothing() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: "nope".into(), feature_kind: "position".into() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    drop(result);
    close(&mut app);
}
}
