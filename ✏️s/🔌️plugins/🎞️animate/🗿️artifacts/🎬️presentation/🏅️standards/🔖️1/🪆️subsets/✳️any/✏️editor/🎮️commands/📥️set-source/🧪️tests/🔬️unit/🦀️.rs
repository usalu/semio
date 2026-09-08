
use super::*;
use crate::editor::animate::PresentationCommand;
use crate::editor::animate::commands::{set_active_example, set_frame};
use crate::editor::animate::testkit::{dispatch, presentation_app};
use crate::{FigureTileFrame, default_presentation_snapshot};

#[semio_framework_async_macros::async_test]
async fn set_source_replaces_source_and_clears_tiles_when_src_changes() {
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 4);
    let mut source = crate::default_figure_tile_source();
    source.src = "/new-figure.png".into();
    source.kind = "image".into();
    dispatch(&mut app, PresentationCommand::SetSource(SetSource { source })).await;
    let deck = app.snapshot().expect("projection");
    let (deck_source, deck_tiles) = crate::presentation_working_scene(&deck);
    assert_eq!(deck_source.src, "/new-figure.png");
    assert_eq!(deck_source.kind, "image");
    assert!(deck_tiles.is_empty(), "changing the source src clears stale tiles");
}

#[semio_framework_async_macros::async_test]
async fn set_source_with_same_src_keeps_existing_tiles() {
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
    let (mut source, _) = crate::presentation_working_scene(&app.snapshot().expect("projection"));
    source.kind = "figure".into();
    dispatch(&mut app, PresentationCommand::SetSource(SetSource { source })).await;
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 4, "unchanged src does not clear tiles");
}

#[semio_framework_async_macros::async_test]
async fn set_frame_updates_source_frame() {
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::SetFrame(set_frame::SetFrame { frame: FigureTileFrame { x: 0.1, y: 0.2, width: 0.3, height: 0.4 } })).await;
    let frame = crate::presentation_working_scene(&app.snapshot().expect("projection")).0.frame;
    assert_eq!(frame.x, 0.1);
    assert_eq!(frame.y, 0.2);
    assert_eq!(frame.width, 0.3);
    assert_eq!(frame.height, 0.4);
}

/// 🧬️ Whole-document replace is not an in-history mutation (a whole-snapshot variant is banned outright), so
/// `setActiveExample` now surfaces as a `Effect::LoadDocument` carrying the default document's
/// pack bytes rather than an `artifact_mutations` entry — `dispatch`'s in-process `VcsArtifactApp`
/// never applies `effects` to its own store (that's the real host's job), so this asserts directly
/// on the emitted effect rather than through `app.snapshot()`.
#[semio_framework_async_macros::async_test]
async fn set_active_example_demo_emits_a_reset_effect() {
    use semio_framework_plugin::Effect;
    let mut app = presentation_app().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
    let deck = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&deck, &history);
    let cfg_snapshot = PresentationConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot };
    let mut ctx = PresentationDispatchCtx { selected_ids: Vec::new() };
    let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "demo".into() }, &doc, &cfg, &mut ctx).expect("handle");
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <PresentationSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode loaded document pack");
    assert!(crate::presentation_working_scene(&loaded).1.is_empty(), "resetting to demo loads the default deck, which has no tiles");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_unknown_id_is_a_no_op() {
    let deck = default_presentation_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&deck, &history);
    let cfg_snapshot = PresentationConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot };
    let mut ctx = PresentationDispatchCtx { selected_ids: Vec::new() };
    let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "other".into() }, &doc, &cfg, &mut ctx).expect("handle");
    assert!(emit.effects.is_empty());
    assert!(emit.artifact_mutations.is_empty());
}
