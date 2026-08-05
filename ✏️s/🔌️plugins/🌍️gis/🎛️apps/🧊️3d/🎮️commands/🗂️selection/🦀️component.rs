//! 🗂️ GIS 3D play app commands — world pin selection. Both rows are config-only: they emit
//! `config_operations`, never document operations.
//!
//! 🧷️ `setSelection` and `worldSelect` are two manifest actions with one behaviour (the pre-migration
//! `handle` matched them in a single `|` arm) — they stay two rows because they are two declared
//! actions with distinct wire keywords, and share one helper rather than duplicating the body.

use crate::apps::gis3d::config::{Gis3dConfig, Gis3dConfigOperation};
use crate::artifacts::gisterrain::op::Gis3dTerrainOperation;
use crate::artifacts::gisterrain::Gis3dTerrainDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SelectionHelpers
/// 👁️ The shared body of `setSelection`/`worldSelect`: replace the selected pin id set.
fn select_ids(ids: &[String]) -> Emit<Gis3dTerrainOperation, Gis3dConfigOperation> {
    Emit::config(vec![Gis3dConfigOperation::SetSelection { ids: ids.to_vec() }])
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

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, Gis3dTerrainDocument>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<Gis3dTerrainOperation, Gis3dConfigOperation>, Fault> {
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

    pub fn handle(payload: &WorldSelect, _doc: &DocumentView<'_, Gis3dTerrainDocument>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<Gis3dTerrainOperation, Gis3dConfigOperation>, Fault> {
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
        assert!(selection.operations.is_empty(), "selection is ephemeral config state");
    }

    #[test]
    fn both_rows_write_the_same_selection_and_reach_the_scene() {
        let mut app = app();
        dispatch(&mut app, Gis3dCommand::SetSelection(set_selection::SetSelection { ids: vec![PIN.into()] }));
        assert!(render(&mut app, GIS3D_PLAY_BODY_COMPOSITE).contains(PIN));
        dispatch(&mut app, Gis3dCommand::SetSelection(set_selection::SetSelection { ids: Vec::new() }));
        assert!(!render(&mut app, GIS3D_PLAY_BODY_COMPOSITE).contains(&format!("\\\"{PIN}\\\"")));
        dispatch(&mut app, Gis3dCommand::WorldSelect(world_select::WorldSelect { ids: vec![PIN.into()] }));
        assert!(render(&mut app, GIS3D_PLAY_BODY_COMPOSITE).contains(PIN));
    }
}
//#endregion 🧪️Tests
