//! 📤️ Downloads an authored elevation source through the native shell.

use super::*;
use semio_plugin::Effect;

pub const ACTION: &str = "exportElevationGrid";
pub(crate) const FILE_NAME: &str = "terrain-elevation.asc";
pub(crate) const MIME_TYPE: &str = "text/plain;charset=utf-8";

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "export-elevation-grid")]
pub struct ExportElevationGrid {}

pub fn handle(_: &ExportElevationGrid, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    let source = doc.snapshot.elevation_source.as_ref().ok_or_else(|| Error::Missing.fault())?;
    let data = ascii_grid::encode(source).map_err(Error::fault)?;
    Ok(Emit::effect(Effect::DownloadMediaExport {
        filename: FILE_NAME.into(),
        mime_type: MIME_TYPE.into(),
        encoding: None,
        data,
    }))
}
