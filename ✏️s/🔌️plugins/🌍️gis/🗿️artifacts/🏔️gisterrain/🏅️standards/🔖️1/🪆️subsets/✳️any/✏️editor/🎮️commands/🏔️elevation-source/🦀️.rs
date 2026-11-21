//! 🏔️ GIS Terrain source actions.

#[path = "➕️create-grid/🦀️.rs"]
pub mod create_elevation_grid;

pub mod clear_elevation_source {
    use crate::mutations::{GisTerrainMutation, ReplaceElevationSource};
    use crate::GisTerrainSnapshot;
    use semio_framework_value_derive::{FromValue, ToValue};
    use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "clear-elevation-source")]
    pub struct ClearElevationSource {}

    pub fn handle(_payload: &ClearElevationSource, doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        if doc.snapshot.elevation_source.is_none() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: None })]))
    }
}
