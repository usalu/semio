//! 🏔️ GIS 3D play app command — vertical exaggeration, the terrain's one editable document property.

use crate::op::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetExaggeration
/// 🧪️ A slider drag is many ticks sharing one coalesce key, so they fold into ONE undoable edit —
/// a single undo restores the pre-drag exaggeration rather than a mid-drag value.
pub mod set_exaggeration {
    use super::*;

    pub const GIS3D_EXAGGERATION_COALESCE_KEY: &str = "gis3d-exaggeration";

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "exaggeration")]
    pub struct SetExaggeration {
        pub exaggeration: f64,
    }

    pub fn handle(payload: &SetExaggeration, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        use crate::mutations::change_exaggeration::ChangeExaggeration;
        Ok(Emit::amend(vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: payload.exaggeration })], GIS3D_EXAGGERATION_COALESCE_KEY))
    }
}
//#endregion 🔖️SetExaggeration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
