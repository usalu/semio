//! 🖼️ Animate present app commands — source media: set-source, set-frame, set-active-example.

use crate::apps::present::config::{PresentConfig, PresentConfigOperation};
use crate::artifacts::present::op::PresentOperation;
use crate::artifacts::present::{default_present_deck, FigureTileFrame, FigureTileSource, PresentDeck};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
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

    pub fn handle(payload: &SetSource, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        let deck = doc.projection;
        let replaced = payload.source.src != deck.source.src;
        let mut operations = vec![PresentOperation::SetSource { source: payload.source.clone() }];
        let mut config_operations = Vec::new();
        if replaced {
            operations.push(PresentOperation::SetTiles { tiles: Vec::new() });
            config_operations.push(PresentConfigOperation::SetSelectedIds { ids: Vec::new() });
        }
        Ok(Emit { document_operations: operations, config_operations, ..Default::default() })
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

    pub fn handle(payload: &SetFrame, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        let deck = doc.projection;
        let mut source = deck.source.clone();
        source.frame = payload.frame.clone();
        Ok(Emit::operations(vec![PresentOperation::SetSource { source }]))
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

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        if payload.example_id == "demo" || payload.example_id.is_empty() {
            Ok(Emit { document_operations: vec![PresentOperation::SetDeck { deck: default_present_deck() }], config_operations: vec![PresentConfigOperation::SetSelectedIds { ids: Vec::new() }], ..Default::default() })
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
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
        let mut source = crate::artifacts::present::default_figure_tile_source();
        source.src = "/new-figure.png".into();
        source.kind = "image".into();
        dispatch(&mut app, PresentCommand::SetSource(set_source::SetSource { source }));
        let deck = app.projection().expect("projection");
        assert_eq!(deck.source.src, "/new-figure.png");
        assert_eq!(deck.source.kind, "image");
        assert!(deck.tiles.is_empty(), "changing the source src clears stale tiles");
    }

    #[test]
    fn set_source_with_same_src_keeps_existing_tiles() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }));
        let mut source = app.projection().expect("projection").source;
        source.kind = "figure".into();
        dispatch(&mut app, PresentCommand::SetSource(set_source::SetSource { source }));
        assert_eq!(app.projection().expect("projection").tiles.len(), 4, "unchanged src does not clear tiles");
    }

    #[test]
    fn set_frame_updates_source_frame() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SetFrame(set_frame::SetFrame { frame: FigureTileFrame { x: 0.1, y: 0.2, width: 0.3, height: 0.4 } }));
        let frame = app.projection().expect("projection").source.frame;
        assert_eq!(frame.x, 0.1);
        assert_eq!(frame.y, 0.2);
        assert_eq!(frame.width, 0.3);
        assert_eq!(frame.height, 0.4);
    }

    #[test]
    fn set_active_example_demo_resets_to_default_deck() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }));
        dispatch(&mut app, PresentCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }));
        assert!(app.projection().expect("projection").tiles.is_empty(), "resetting to demo clears seeded tiles");
    }

    #[test]
    fn set_active_example_unknown_id_is_a_no_op() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }));
        dispatch(&mut app, PresentCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "other".into() }));
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
    }
}
//#endregion 🧪️Tests
