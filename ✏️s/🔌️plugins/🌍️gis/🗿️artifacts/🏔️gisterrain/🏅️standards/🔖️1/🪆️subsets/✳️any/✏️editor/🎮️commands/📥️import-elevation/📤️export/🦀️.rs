//! 📤️ Downloads an authored elevation source through the native shell.

use super::*;
use semio_plugin::Effect;

pub const ACTION: &str = "exportElevationGrid";

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "export-elevation-grid")]
pub struct ExportElevationGrid {}

pub fn handle(_: &ExportElevationGrid, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    let source = doc.snapshot.elevation_source.as_ref().ok_or_else(|| Error::Missing.fault())?;
    let data = ascii_grid::encode(source).map_err(Error::fault)?;
    Ok(Emit::effect(Effect::DownloadMediaExport {
        filename: "terrain-elevation.asc".into(),
        mime_type: "text/plain;charset=utf-8".into(),
        encoding: None,
        data,
    }))
}
