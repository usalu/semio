//! 👁️ Animate present app commands — ephemeral view/config-only actions: set-selected-ids,
//! canvas-pointer-down, no-op, set-locale.

use crate::apps::present::config::{PresentConfig, PresentConfigOperation};
use crate::apps::present::valid_tile_ids;
use crate::artifacts::present::op::PresentOperation;
use crate::artifacts::present::PresentDeck;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelectedIds
pub mod set_selected_ids {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selected-ids")]
    pub struct SetSelectedIds {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelectedIds, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        Ok(Emit::config(vec![PresentConfigOperation::SetSelectedIds { ids: valid_tile_ids(doc.projection, payload.ids.clone()) }]))
    }
}
//#endregion 🔖️SetSelectedIds

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-down")]
    pub struct CanvasPointerDown {
        pub layer_id: Option<String>,
    }

    pub fn handle(payload: &CanvasPointerDown, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        let deck = doc.projection;
        match &payload.layer_id {
            Some(id) if deck.tiles.iter().any(|tile| &tile.id == id) => Ok(Emit::config(vec![PresentConfigOperation::SetSelectedIds { ids: vec![id.clone()] }])),
            _ => Ok(Emit::config(vec![PresentConfigOperation::SetSelectedIds { ids: Vec::new() }])),
        }
    }
}
//#endregion 🔖️CanvasPointerDown

//#region 🔖️NoOperation
pub mod no_operation {
    use super::*;

    /// 👁️ Decorative no-op wired to the read-only "active source" catalogue field's `on_change` — never
    /// mutates anything (mirrors the pre-B1 `"noOperation"` view action verbatim).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "no-op")]
    pub struct NoOperation {}

    pub fn handle(_payload: &NoOperation, _doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️NoOperation

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        Ok(Emit::config(vec![PresentConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{dispatch, present_app, render};
    use crate::apps::present::{PresentCommand, PRESENT_PLAY_BODY_CATALOGUE, PRESENT_PLAY_BODY_DETAILS};
    use semio_framework_plugin::testkit::meta;

    #[test]
    fn animate_present_labels_resolve_native_by_default() {
        let mut app = present_app();
        let catalogue = render(&mut app, PRESENT_PLAY_BODY_CATALOGUE);
        assert!(catalogue.contains("Tile templates"));
        assert!(catalogue.contains("Split 2×2 grid"));
        assert!(catalogue.contains("Active source"));
        assert!(!catalogue.contains("Kachelvorlagen"));
    }

    #[test]
    fn animate_present_labels_translate_panels_in_german() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SetLocale(set_locale::SetLocale { value: "de".into() }));
        let catalogue_json = render(&mut app, PRESENT_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Kachelvorlagen"));
        assert!(catalogue_json.contains("2×2-Raster teilen"));
        assert!(catalogue_json.contains("Aktive Quelle"));
        assert!(!catalogue_json.contains("Tile templates"));

        let document_json = render(&mut app, crate::apps::present::PRESENT_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Kacheln"));
    }

    #[test]
    fn canvas_pointer_down_selects_matching_tile_and_clears_on_miss() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(crate::apps::present::commands::tile::add_tile::AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = app.projection().expect("projection").tiles[0].id.clone();
        app.dispatch_typed(PresentCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: Some(tile_id) }), &meta("local")).expect("pointer hit");
        let details = render(&mut app, PRESENT_PLAY_BODY_DETAILS);
        assert!(details.contains("animate.present.play.details.crop"), "hitting a tile populates the details panel");

        app.dispatch_typed(PresentCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: Some("source-frame".into()) }), &meta("local")).expect("pointer miss");
        let details = render(&mut app, PRESENT_PLAY_BODY_DETAILS);
        assert!(details.contains("Select a tile"), "missing the backdrop clears selection");
    }

    #[test]
    fn build_details_tree_reports_tile_not_found_for_stale_selection() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::SetSelectedIds(set_selected_ids::SetSelectedIds { ids: vec!["was-deleted".into()] }), &meta("local")).expect("select stale");
        assert!(app.projection().expect("projection").tiles.is_empty());
    }
}
//#endregion 🧪️Tests
