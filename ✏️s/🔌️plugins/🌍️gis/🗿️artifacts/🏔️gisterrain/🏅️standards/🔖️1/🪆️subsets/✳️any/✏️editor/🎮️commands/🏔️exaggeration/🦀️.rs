//! 🏔️ GIS 3D play app command — vertical exaggeration, the terrain's one editable document property.

use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetExaggeration
/// 🎚️ The exaggeration slider's leaf constructor: the ABSOLUTE `change-exaggeration` of the value. A slider press is the
/// framework scrub (design §13.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): ticks preview, the release commits
/// ONE transaction, a cancel leaves zero trace — the handler never sees the press.
pub mod set_exaggeration {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "exaggeration")]
    pub struct SetExaggeration {
        pub exaggeration: f64,
    }

    pub fn handle(payload: &SetExaggeration, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        use crate::mutations::change_exaggeration::ChangeExaggeration;
        Ok(Emit::mutations(vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: payload.exaggeration })]))
    }
}
//#endregion 🔖️SetExaggeration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
