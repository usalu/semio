use super::*;
use crate::editor::animate::commands::clear_tiles;
use crate::editor::animate::unit_tests::context::{dispatch, presentation_app, presentation_app_with_registry};
use crate::editor::animate::PresentationCommand;

#[semio_framework_async_macros::async_test]
async fn seed_grid_action_adds_tiles() {
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 })).await;
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 4);
}

/// 🧬️ Whole-document replace is not an in-history mutation (a whole-snapshot variant is banned outright), so
/// `setActiveExample` now surfaces as a `Effect::LoadDocument` carrying the default document's
/// pack bytes rather than an `artifact_mutations` entry — `dispatch`'s in-process `VcsArtifactApp`
/// never applies `effects` to its own store (that's the real host's job), so this asserts directly
/// on the emitted effect rather than through `app.snapshot()`.
#[semio_framework_async_macros::async_test]
async fn set_active_example_demo_emits_a_reset_effect_after_seed() {
    use semio_framework_plugin::Effect;
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 })).await;
    let deck = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&deck, &history);
    let cfg_snapshot = PresentationConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let mut ctx = PresentationDispatchCtx { selected_ids: Vec::new() };
    let emit = crate::editor::animate::commands::set_active_example::handle(&crate::editor::animate::commands::set_active_example::SetActiveExample { example_id: "demo".into() }, &doc, &cfg, &mut ctx).expect("handle");
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <PresentationSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode loaded document pack");
    assert_eq!(loaded, crate::demo_presentation_snapshot(), "resetting to demo loads THE demo deck (source figure + its 3x5 tile crops), not the tile-less default document");
}

/// 🕹️ Selection is framework-owned (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM);
/// `clearTiles` clears the document and emits an empty-target `interactionSelect` so the "tiles"
/// domain's selection clears too. Since ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C the reactor
/// folds that verb inline in the typed-operation ladder, so the witness is the selection snapshot
/// once the retained publication settles (registry-backed app, bound instance, host settle
/// protocol), not the effect — which never reaches the host any more. The select-then-clear
/// transition itself is `👇️canvas-pointer-down`'s hit/miss law; here the clear lands on an empty
/// selection and must leave it empty without bouncing anything to the host.
#[semio_framework_async_macros::async_test]
async fn clear_tiles_action_empties_tiles_and_clears_the_selection_inline() {
    use crate::editor::animate::PRESENTATION_INTERACTION_DOMAIN;
    use semio_framework_plugin::{artifact_app_laws, Effect, PluginApp};
    let mut app = presentation_app_with_registry().await;
    let instance_id = artifact_app_laws::meta("local").instance_id;
    app.bind_instance_id(instance_id).await;
    dispatch(&mut app, PresentationCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 })).await;
    artifact_app_laws::settle_registered_typed_operation(&mut *app, instance_id).await.expect("seed settles");
    dispatch(&mut app, PresentationCommand::ClearTiles(clear_tiles::ClearTiles {})).await;
    let receipt = artifact_app_laws::settle_registered_typed_operation(&mut *app, instance_id).await.expect("clear settles");
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty());
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { .. })), "the clearing interactionSelect is folded in-reactor, never handed to the host: {:?}", receipt.effects);
    assert!(app.interaction_state().await.selection.get(PRESENTATION_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "the tiles selection is empty after the inline clear");
    artifact_app_laws::close_registered_fixture_app(&mut *app);
}
