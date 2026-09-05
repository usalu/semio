//! 🖼️ 🖼️ Animate presentation app commands command — `set-source`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::mutations::replace_source::mutation::ReplaceSource;
use crate::artifacts::presentation::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::{FigureTileSource, PresentationSnapshot};
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-source")]
pub struct SetSource {
    #[dsl(block)]
    pub source: FigureTileSource,
}

pub fn handle(payload: &SetSource, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (deck_source, _) = crate::artifacts::presentation::presentation_working_scene(deck);
    let replaced = payload.source.src != deck_source.src;
    let mut operations = vec![PresentationMutation::ReplaceSource(ReplaceSource { new_source: payload.source.clone() })];
    let mut emit_effects = Vec::new();
    if replaced {
        operations.push(PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: Vec::new() }));
        emit_effects.push(interaction_select_effect(&[], "replace"));
    }
    Ok(Emit { artifact_mutations: operations, effects: emit_effects, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::presentation::{default_presentation_snapshot, FigureTileFrame};
    use crate::editor::animate::commands::{set_active_example, set_frame};
    use crate::editor::animate::testkit::{dispatch, presentation_app};
    use crate::editor::animate::PresentationCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_source_replaces_source_and_clears_tiles_when_src_changes() {
        let mut app = presentation_app().await;
        dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection")).1.len(), 4);
        let mut source = crate::artifacts::presentation::default_figure_tile_source();
        source.src = "/new-figure.png".into();
        source.kind = "image".into();
        dispatch(&mut app, PresentationCommand::SetSource(SetSource { source })).await;
        let deck = app.snapshot().await.expect("projection");
        let (deck_source, deck_tiles) = crate::artifacts::presentation::presentation_working_scene(&deck);
        assert_eq!(deck_source.src, "/new-figure.png");
        assert_eq!(deck_source.kind, "image");
        assert!(deck_tiles.is_empty(), "changing the source src clears stale tiles");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_source_with_same_src_keeps_existing_tiles() {
        let mut app = presentation_app().await;
        dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
        let (mut source, _) = crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection"));
        source.kind = "figure".into();
        dispatch(&mut app, PresentationCommand::SetSource(SetSource { source })).await;
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection")).1.len(), 4, "unchanged src does not clear tiles");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_frame_updates_source_frame() {
        let mut app = presentation_app().await;
        dispatch(&mut app, PresentationCommand::SetFrame(set_frame::SetFrame { frame: FigureTileFrame { x: 0.1, y: 0.2, width: 0.3, height: 0.4 } })).await;
        let frame = crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection")).0.frame;
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
        let deck = app.snapshot().await.expect("projection");
        let history = semio_framework_plugin::HistoryView::empty().await;
        let doc = ArtifactView::new(&deck, &history).await;
        let cfg_snapshot = PresentationConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let mut ctx = PresentationDispatchCtx { selected_ids: Vec::new() };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "demo".into() }, &doc, &cfg, &mut ctx).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <PresentationSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(crate::artifacts::presentation::presentation_working_scene(&loaded).1.is_empty(), "resetting to demo loads the default deck, which has no tiles");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_unknown_id_is_a_no_op() {
        let deck = default_presentation_snapshot();
        let history = semio_framework_plugin::HistoryView::empty().await;
        let doc = ArtifactView::new(&deck, &history).await;
        let cfg_snapshot = PresentationConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let mut ctx = PresentationDispatchCtx { selected_ids: Vec::new() };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "other".into() }, &doc, &cfg, &mut ctx).expect("handle");
        assert!(emit.effects.is_empty());
        assert!(emit.artifact_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
