//! 🌐️ 🌐️ Animate present app commands command — `seed-grid`.

use crate::artifacts::present::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "seed-grid")]
pub struct SeedGrid {
    pub rows: u32,
    pub columns: u32,
}

pub async fn handle(payload: &SeedGrid, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (deck_source, _) = crate::artifacts::present::present_working_scene(deck);
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck_source, rows: payload.rows, columns: payload.columns, gap: 0.0, key_prefix: "tile" });
    let selected: Vec<String> = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
    let mut emit = Emit::mutations(vec![PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })]);
    emit.effects.push(interaction_select_effect(&selected, "replace"));
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::commands::clear_tiles;
    use crate::editor::animate::testkit::{dispatch, present_app};
    use crate::editor::animate::PresentCommand;
    use semio_framework_plugin::testkit::meta;

    #[semio_framework_async_macros::async_test]
    async fn seed_grid_action_adds_tiles() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 }));
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 4);
    }

    /// 🧬️ Whole-document replace is not an in-history mutation (a whole-snapshot variant is banned outright), so
    /// `setActiveExample` now surfaces as a `Effect::LoadDocument` carrying the default document's
    /// pack bytes rather than an `artifact_mutations` entry — `dispatch`'s in-process `VcsArtifactApp`
    /// never applies `effects` to its own store (that's the real host's job), so this asserts directly
    /// on the emitted effect rather than through `app.snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn set_active_example_demo_emits_a_reset_effect_after_seed() {
        use semio_framework_plugin::Effect;
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 }));
        let deck = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&deck, &history);
        let cfg_snapshot = PresentConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let mut ctx = PresentDispatchCtx { selected_ids: Vec::new() };
        let emit = crate::editor::animate::commands::set_active_example::handle(&crate::editor::animate::commands::set_active_example::SetActiveExample { example_id: "demo".into() }, &doc, &cfg, &mut ctx).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <PresentSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(crate::artifacts::present::present_working_scene(&loaded).1.is_empty(), "resetting to demo loads the default deck, which has no seeded tiles");
    }

    /// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
    /// MECHANISM); `clearTiles` clears the document, and its `interactionSelect` effect asks the
    /// framework to clear the "tiles" domain's selection too (asserted directly on the effect — the
    /// in-process test harness never applies `effects` to itself).
    #[semio_framework_async_macros::async_test]
    async fn clear_tiles_action_empties_tiles_and_requests_a_selection_clear() {
        use semio_framework_plugin::Effect;
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 }));
        let result = dispatch(&mut app, PresentCommand::ClearTiles(clear_tiles::ClearTiles {}));
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.is_empty());
        assert!(matches!(result.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));
    }
}
//#endregion 🧪️Tests
