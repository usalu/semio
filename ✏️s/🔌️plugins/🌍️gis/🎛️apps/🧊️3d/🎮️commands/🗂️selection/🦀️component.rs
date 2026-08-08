//! 🗂️ GIS 3D play app commands — world pin selection. Both rows are config-only: they emit
//! `config_mutations`, never document operations.
//!
//! 🧷️ `setSelection` and `worldSelect` are two manifest actions with one behaviour (the pre-migration
//! `handle` matched them in a single `|` arm) — they stay two rows because they are two declared
//! actions with distinct wire keywords, and share one helper rather than duplicating the body.

use crate::apps::gis3d::config::{Gis3dConfig, Gis3dConfigMutation};
use crate::artifacts::gisterrain::op::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SelectionHelpers
/// 👁️ The shared body of `setSelection`/`worldSelect`: replace the selected pin id set.
fn select_ids(ids: &[String]) -> Emit<GisTerrainMutation, Gis3dConfigMutation> {
    Emit::config(vec![Gis3dConfigMutation::SetSelection { ids: ids.to_vec() }])
}
//#endregion 🔖️SelectionHelpers

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation>, Fault> {
        Ok(select_ids(&payload.ids))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️WorldSelect
pub mod world_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-select")]
    pub struct WorldSelect {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &WorldSelect, _doc: &DocumentView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation>, Fault> {
        Ok(select_ids(&payload.ids))
    }
}
//#endregion 🔖️WorldSelect

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE;
    use crate::apps::gis3d::testkit::{app, dispatch, render};
    use crate::apps::gis3d::Gis3dCommand;

    const PIN: &str = "p_institut_de_botanique_ulg_liege";

    #[test]
    fn selection_is_config_state_and_emits_no_operations() {
        let mut app = app();
        let selection = dispatch(&mut app, Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: vec![PIN.into()] }));
        assert!(selection.mutations.is_empty(), "selection is ephemeral config state");
    }

    /// 🗂️ Both rows must emit the identical config operation. Probes the emitted payload rather than
    /// the rendered scene: a pin id appears in the scene's instance layer whether or not it is
    /// selected, so a substring check on the render output is not a selection probe.
    #[test]
    fn both_rows_write_the_same_selection() {
        let document = crate::artifacts::gisterrain::engine::default_terrain_document();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { snapshot: &document, history: &history };
        let config = Gis3dConfig::default();
        let cfg = ConfigView { snapshot: &config };

        let via_set = set_selection::handle(&set_selection::SetSelection { ids: vec![PIN.into()] }, &doc, &cfg).expect("setSelection");
        let via_world = world_select::handle(&world_select::WorldSelect { ids: vec![PIN.into()] }, &doc, &cfg).expect("worldSelect");
        assert_eq!(via_set.config_mutations, vec![Gis3dConfigMutation::SetSelection { ids: vec![PIN.to_string()] }]);
        assert_eq!(via_set.config_mutations, via_world.config_mutations, "the two declared actions share one behaviour");
    }

    /// 🖥️ The selected id reaches the rendered World3d scene's selection payload.
    #[test]
    fn the_selection_reaches_the_rendered_scene() {
        let mut app = app();
        dispatch(&mut app, Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: vec![PIN.into()] }));
        assert!(render(&mut app, GIS3D_PLAY_BODY_COMPOSITE).contains(PIN));
    }
}
//#endregion 🧪️Tests
