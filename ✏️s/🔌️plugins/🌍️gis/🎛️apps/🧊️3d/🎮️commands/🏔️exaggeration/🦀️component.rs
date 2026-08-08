//! 🏔️ GIS 3D play app command — vertical exaggeration, the terrain's one editable document property.

use crate::apps::gis3d::config::{Gis3dConfig, Gis3dConfigMutation};
use crate::artifacts::gisterrain::op::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetExaggeration
/// 🧪️ A slider drag is many ticks sharing one coalesce key, so they fold into ONE undoable edit —
/// a single undo restores the pre-drag exaggeration rather than a mid-drag value.
pub mod set_exaggeration {
    use super::*;

    pub const GIS3D_EXAGGERATION_COALESCE_KEY: &str = "gis3d-exaggeration";

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "exaggeration")]
    pub struct SetExaggeration {
        pub exaggeration: f64,
    }

    pub fn handle(payload: &SetExaggeration, _doc: &DocumentView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation>, Fault> {
        Ok(Emit::amend(vec![GisTerrainMutation::SetExaggeration { exaggeration: payload.exaggeration }], GIS3D_EXAGGERATION_COALESCE_KEY))
    }
}
//#endregion 🔖️SetExaggeration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis3d::testkit::{app, dispatch};
    use crate::apps::gis3d::Gis3dCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn seeds_exaggeration_from_the_terrain_fixture() {
        let app = app();
        assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5);
    }

    /// 🧪️ A slider drag is many `setExaggeration` ticks sharing one coalesce key: they fold into ONE
    /// undoable edit, so a single undo restores the fixture's exaggeration rather than a mid-drag value.
    #[test]
    fn exaggeration_drag_coalesces_into_one_undo_step() {
        let mut app = app();
        for value in [2.0, 2.5, 3.0] {
            dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: value }));
        }
        assert_eq!(app.snapshot().expect("projection").exaggeration, 3.0);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5, "one coalesced edit: undo restores the fixture exaggeration");
    }

    #[test]
    fn set_exaggeration_is_a_document_operation_not_config_state() {
        let mut app = app();
        let result = dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 }));
        assert_eq!(result.mutations.len(), 1, "exaggeration is undoable document state");
    }
}
//#endregion 🧪️Tests
