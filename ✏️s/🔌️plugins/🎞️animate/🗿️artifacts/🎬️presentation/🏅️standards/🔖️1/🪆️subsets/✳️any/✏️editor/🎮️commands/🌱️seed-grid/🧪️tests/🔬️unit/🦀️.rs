
use super::*;
use crate::editor::animate::PresentationCommand;
use crate::editor::animate::commands::clear_tiles;
use crate::editor::animate::testkit::{dispatch, presentation_app};

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
    let cfg = ConfigView { snapshot: &cfg_snapshot };
    let mut ctx = PresentationDispatchCtx { selected_ids: Vec::new() };
    let emit = crate::editor::animate::commands::set_active_example::handle(&crate::editor::animate::commands::set_active_example::SetActiveExample { example_id: "demo".into() }, &doc, &cfg, &mut ctx).expect("handle");
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <PresentationSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(crate::presentation_working_scene(&loaded).1.is_empty(), "resetting to demo loads the default deck, which has no seeded tiles");
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM); `clearTiles` clears the document, and its `interactionSelect` effect asks the
/// framework to clear the "tiles" domain's selection too (asserted directly on the effect — the
/// in-process test harness never applies `effects` to itself).
#[semio_framework_async_macros::async_test]
async fn clear_tiles_action_empties_tiles_and_requests_a_selection_clear() {
    use semio_framework_plugin::Effect;
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 })).await;
    let result = dispatch(&mut app, PresentationCommand::ClearTiles(clear_tiles::ClearTiles {})).await;
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty());
    assert!(matches!(result.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));
}
