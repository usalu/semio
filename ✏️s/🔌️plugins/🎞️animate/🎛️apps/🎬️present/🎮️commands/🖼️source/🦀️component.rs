//! 🖼️ Animate present app commands — source media: set-source, set-frame, set-active-example.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::artifacts::present::mutations::replace_source::mutation::ReplaceSource;
use crate::artifacts::present::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::present::mutations::resize_source_frame::mutation::ResizeSourceFrame;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{default_present_snapshot, FigureTileFrame, FigureTileSource, PresentSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSource
pub mod set_source {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-source")]
    pub struct SetSource {
        #[dsl(block)]
        pub source: FigureTileSource,
    }

    pub fn handle(payload: &SetSource, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        let deck = doc.snapshot;
        let replaced = payload.source.src != deck.source.src;
        let mut operations = vec![PresentMutation::ReplaceSource(ReplaceSource { new_source: payload.source.clone() })];
        let mut config_mutations = Vec::new();
        if replaced {
            operations.push(PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: Vec::new() }));
            config_mutations.push(PresentConfigMutation::SetSelectedIds { ids: Vec::new() });
        }
        Ok(Emit { artifact_mutations: operations, config_mutations, ..Default::default() })
    }
}
//#endregion 🔖️SetSource

//#region 🔖️SetFrame
pub mod set_frame {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-frame")]
    pub struct SetFrame {
        #[dsl(block)]
        pub frame: FigureTileFrame,
    }

    pub fn handle(payload: &SetFrame, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![PresentMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: payload.frame.clone() })]))
    }
}
//#endregion 🔖️SetFrame

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see
    /// `📓️taxonomy.md`'s forbidden vocabulary), so "reset to demo" builds
    /// `apps::present::reset_present_document_effect` (a `HostEffect::LoadDocument`, outside undo
    /// history) instead of an `artifact_mutations` entry.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        if payload.example_id == "demo" || payload.example_id.is_empty() {
            Ok(Emit {
                effects: vec![crate::apps::present::reset_present_document_effect(&default_present_snapshot())],
                config_mutations: vec![PresentConfigMutation::SetSelectedIds { ids: Vec::new() }],
                ..Default::default()
            })
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{dispatch, present_app};
    use crate::apps::present::PresentCommand;

    #[test]
    fn set_source_replaces_source_and_clears_tiles_when_src_changes() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }));
        assert_eq!(app.snapshot().expect("projection").tiles.len(), 4);
        let mut source = crate::artifacts::present::default_figure_tile_source();
        source.src = "/new-figure.png".into();
        source.kind = "image".into();
        dispatch(&mut app, PresentCommand::SetSource(set_source::SetSource { source }));
        let deck = app.snapshot().expect("projection");
        assert_eq!(deck.source.src, "/new-figure.png");
        assert_eq!(deck.source.kind, "image");
        assert!(deck.tiles.is_empty(), "changing the source src clears stale tiles");
    }

    #[test]
    fn set_source_with_same_src_keeps_existing_tiles() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }));
        let mut source = app.snapshot().expect("projection").source;
        source.kind = "figure".into();
        dispatch(&mut app, PresentCommand::SetSource(set_source::SetSource { source }));
        assert_eq!(app.snapshot().expect("projection").tiles.len(), 4, "unchanged src does not clear tiles");
    }

    #[test]
    fn set_frame_updates_source_frame() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SetFrame(set_frame::SetFrame { frame: FigureTileFrame { x: 0.1, y: 0.2, width: 0.3, height: 0.4 } }));
        let frame = app.snapshot().expect("projection").source.frame;
        assert_eq!(frame.x, 0.1);
        assert_eq!(frame.y, 0.2);
        assert_eq!(frame.width, 0.3);
        assert_eq!(frame.height, 0.4);
    }

    /// 🧬️ Whole-document replace is not an in-history mutation (a whole-snapshot variant is banned outright), so
    /// `setActiveExample` now surfaces as a `HostEffect::LoadDocument` carrying the default document's
    /// pack bytes rather than an `artifact_mutations` entry — `dispatch`'s in-process `VcsArtifactApp`
    /// never applies `effects` to its own store (that's the real host's job), so this asserts directly
    /// on the emitted effect rather than through `app.snapshot()`.
    #[test]
    fn set_active_example_demo_emits_a_reset_effect() {
        use semio_framework_plugin::HostEffect;
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }));
        let deck = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&deck, &history);
        let cfg_snapshot = PresentConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "demo".into() }, &doc, &cfg).expect("handle");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <PresentSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(loaded.tiles.is_empty(), "resetting to demo loads the default deck, which has no tiles");
    }

    #[test]
    fn set_active_example_unknown_id_is_a_no_op() {
        let deck = default_present_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&deck, &history);
        let cfg_snapshot = PresentConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "other".into() }, &doc, &cfg).expect("handle");
        assert!(emit.effects.is_empty());
        assert!(emit.artifact_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
