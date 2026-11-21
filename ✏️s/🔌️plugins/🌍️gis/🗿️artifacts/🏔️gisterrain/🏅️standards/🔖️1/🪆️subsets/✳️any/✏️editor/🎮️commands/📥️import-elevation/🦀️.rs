//! 📂️ Opens a local WGS84 Esri ASCII Grid in the terrain editor.

use crate::mutations::{GisTerrainMutation, ReplaceElevationSource};
use crate::schema::snapshot::{ElevationSource, GisTerrainSnapshot};
use super::local_file;
use ascii_grid::Error;
pub use super::local_file::MAX_FILE_BYTES;
use semio_framework_value::DslValue;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};
use semio_ui_locale::LocalizedLabel;

pub const REQUEST_ACTION: &str = "importElevationGrid";
pub const FILE_ACTION: &str = "importElevationGridFile";

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-elevation-grid")]
pub struct ImportElevationGrid {}

pub fn handle(_: &ImportElevationGrid, _: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    Ok(Emit::effect(local_file::open(0x45534752, ".asc", FILE_ACTION)))
}

pub mod file {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "import-elevation-grid-file")]
    pub struct ImportElevationGridFile {
        pub payload: String,
    }

    impl ImportElevationGridFile {
        /// 📦️ Moves the bounded, complete picker text into the retained command.
        pub fn receive(args: &mut Option<DslValue>) -> Result<Self, Fault> {
            local_file::receive(args).map(|payload| Self { payload }).ok_or_else(|| Error::FileSize.fault())
        }
    }

    pub fn handle(file: &ImportElevationGridFile, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        let source = ascii_grid::decode(&file.payload).map_err(Error::fault)?;
        Ok(if doc.snapshot.elevation_source.as_ref() == Some(&source) {
            Emit::default()
        } else {
            Emit::mutations(vec![GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: Some(source) })])
        })
    }
}

trait ErrorNotice {
    fn details(self) -> (&'static str, &'static str, &'static str);
    fn fault(self) -> Fault;
}

impl ErrorNotice for Error {
    fn fault(self) -> Fault {
        let (code, en, _) = self.details();
        Fault::new(FaultOrigin::App, FaultCode::new(code), en)
    }

    fn details(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::FileSize => ("gis.terrain.import.size", "Choose a complete ASCII Grid file of 24 KiB or smaller.", "Wähle eine vollständige ASCII-Rasterdatei mit höchstens 24 KiB."),
            Self::Header => ("gis.terrain.import.header", "The ASCII Grid header requires dimensions, cell size, and a matching corner or centre origin.", "Der ASCII-Rasterkopf benötigt Abmessungen, Zellgrösse und zusammengehörige Eck- oder Mittelpunktkoordinaten."),
            Self::Bounds => ("gis.terrain.import.bounds", "The ASCII Grid extent must be in WGS84 longitude and latitude degrees.", "Die Ausdehnung des ASCII-Rasters muss in WGS84-Längen- und Breitengraden angegeben sein."),
            Self::Samples => ("gis.terrain.import.samples", "The ASCII Grid must contain the declared number of finite elevations in the range −32768 to 32767 metres.", "Das ASCII-Raster muss die angegebene Anzahl endlicher Höhenwerte zwischen −32768 und 32767 Metern enthalten."),
            Self::NoData => ("gis.terrain.import.nodata", "Fill the ASCII Grid’s NoData cells before importing it.", "Fülle die fehlenden Höhenwerte im ASCII-Raster vor dem Import aus."),
            Self::Missing => ("gis.terrain.export.missing", "This terrain has no saved elevation grid.", "Dieses Gelände hat kein gespeichertes Höhenraster."),
            Self::CellShape => ("gis.terrain.export.cell-shape", "Esri ASCII Grid requires the same longitude and latitude sample spacing.", "Esri-ASCII-Raster benötigen gleiche Abstände in Längen- und Breitengraden."),
        }
    }
}

/// 🔔️ Registers the native notifications for file admission and encoding faults.
pub(crate) fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&'static str, LocalizedLabel); 7]> = std::sync::LazyLock::new(|| {
        [Error::FileSize, Error::Header, Error::Bounds, Error::Samples, Error::NoData, Error::Missing, Error::CellShape].map(|error| {
            let (code, en, de) = error.details();
            (code, LocalizedLabel::native(en, de))
        })
    });
    NOTICES.as_slice()
}

mod ascii_grid {
    pub(super) use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::text::ascii_grid::{Error, encode};
    pub(super) fn decode(text: &str) -> Result<super::ElevationSource, Error> {
        semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::text::ascii_grid::decode(text, super::MAX_FILE_BYTES as usize)
    }
}

#[path = "📤️export/🦀️.rs"]
pub mod export;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
