//! 👁️ 👁️ Animate present app commands command — `set-selected-ids`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::valid_tile_ids;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selected-ids")]
pub struct SetSelectedIds {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelectedIds, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::config(vec![PresentConfigMutation::SetSelectedIds { ids: valid_tile_ids(doc.snapshot, payload.ids.clone()) }]))
}

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
        app.dispatch_typed(PresentCommand::AddTile(crate::apps::present::commands::add_tile::AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
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
        app.dispatch_typed(PresentCommand::SetSelectedIds(SetSelectedIds { ids: vec!["was-deleted".into()] }), &meta("local")).expect("select stale");
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.is_empty());
    }
}
//#endregion 🧪️Tests
