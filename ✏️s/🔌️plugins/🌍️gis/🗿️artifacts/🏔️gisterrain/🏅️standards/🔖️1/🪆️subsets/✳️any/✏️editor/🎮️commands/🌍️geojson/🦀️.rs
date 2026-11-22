//! 🌍️ Exchanges GeoJSON overlays with a Terrain document through the Semio shell.

use super::local_file;
use crate::mutations::change_imported_features::ChangeImportedFeatures;
use crate::mutations::GisTerrainMutation;
use crate::schema::{imported_map::ImportedMap, snapshot::GisTerrainSnapshot};
use codec::Error;
use semio_framework_value::DslValue;
use semio_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};
use semio_ui_locale::LocalizedLabel;

pub const REQUEST_ACTION: &str = "importGeoJson";
pub const FILE_ACTION: &str = "importGeoJsonFile";

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-geojson")]
pub struct ImportGeoJson {}

pub fn handle(_: &ImportGeoJson, _: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    Ok(Emit::effect(local_file::open(0x47454f4a, ".geojson,.json,application/geo+json", FILE_ACTION, local_file::MAX_FILE_BYTES)))
}

pub mod file {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "import-geojson-file")]
    pub struct ImportGeoJsonFile {
        pub payload: String,
    }

    impl ImportGeoJsonFile {
        /// 📥️ Takes the original callback text into one typed import.
        pub fn receive(args: &mut Option<DslValue>) -> Result<Self, Fault> {
            local_file::receive(args, local_file::MAX_FILE_BYTES).map(|payload| Self { payload }).ok_or_else(|| Error::File.fault())
        }
    }

    pub fn handle(file: &ImportGeoJsonFile, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        let map = codec::decode(&file.payload).map_err(Error::fault)?;
        Ok(if doc.snapshot.imported_map.as_ref().is_some_and(|previous| previous.same(&map)) {
            Emit::default()
        } else {
            Emit::mutations(vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: Some(map) })])
        })
    }
}

pub mod export {
    use super::*;
    use super::super::import_elevation::{self, ErrorNotice as _, ascii_grid};
    use crate::schema::snapshot::ELEVATION_MAX_SIDE;
    use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory};
    use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
    use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError, retirement::controlled::ControlledRetirement};
    use semio_plugin::{ArtifactOwnedToolJobFactory, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp};
    use semio_plugin::app::{ArtifactDownloadOutput, ArtifactOutputChunks, EphemeralEmit};
    use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};

    pub const ACTION: &str = "exportGeoJson";
    pub(crate) const MAX_BYTES: usize = 256 * 1024;
    const MAX_RAW_BYTES: usize = 8_192;
    const FILE_NAME: &str = "terrain-overlays.geojson";
    const MIME_TYPE: &str = "application/geo+json";
    const STAGE: &str = "terrain-geojson-export";
    const PREVIEW: &[u8] = "{\"en\":\"Writing GeoJSON overlays\",\"de\":\"GeoJSON-Overlays werden geschrieben\"}".as_bytes();
    type App = EditorApp<crate::editor::gis3d::Gis3dPlayApp>;
    type Step = ArtifactCommandWorkStep<App>;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "export-geojson")]
    pub struct ExportGeoJson {}

    pub fn handle(_: &ExportGeoJson, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
        let data = codec::encode(doc.snapshot.imported_map.as_ref()).map_err(Error::fault)?;
        Ok(inline(data))
    }

    fn inline(data: String) -> Emit<GisTerrainMutation, NoConfigMutation> {
        Emit::effect(Effect::DownloadMediaExport { filename: FILE_NAME.into(), mime_type: MIME_TYPE.into(), data, encoding: None })
    }

    pub(crate) struct LocalFileExportJobFactory([ToolFactoryKey; 2]);

    impl LocalFileExportJobFactory {
        pub(crate) fn new(controller: &str) -> Self {
            Self([ACTION, import_elevation::export::ACTION].map(|id| ToolFactoryKey::new(controller, id)))
        }

        pub(crate) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
            Self::bounded_first_step_tool_proofs()
        }

        semio_plugin::bounded_first_step_tool_proofs! {
            owner: App,
            owner_file: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
            controller: "s.gis.gisterrain@1/*#editor",
            artifact_schema: "gis.terrain",
            factory: "LocalFileExportJobFactory",
            factory_type: LocalFileExportJobFactory,
            tools: {
                "exportGeoJson" => ToolExecutionContract::resumable(MAX_RAW_BYTES, 32, 4096, MAX_BYTES, 7_500, 64, 1),
                "exportElevationGrid" => ToolExecutionContract::resumable(MAX_RAW_BYTES, 32, 4096, MAX_BYTES, 7_500, 64, 1),
            }
        }
    }

    impl ToolJobFactory for LocalFileExportJobFactory {
        type Payload = ArtifactRetainedCommandPayload<App>;
        type Job = ArtifactRetainedCommandJob<App>;

        fn keys(&self) -> &[ToolFactoryKey] { &self.0 }
        fn payload_schema_id(&self) -> &str { "gis.terrain.local-file-download.tool-command.v1" }
        fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::resumable(MAX_RAW_BYTES, 32, 4096, MAX_BYTES, 7_500, 64, 1)
        }
        semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: MAX_RAW_BYTES, checkpoint_resume: false);
    }

    impl ArtifactOwnedToolJobFactory for LocalFileExportJobFactory {
        type Owner = App;
        const TOOL_IDS: &'static [&'static str] = &[ACTION, import_elevation::export::ACTION];
        const DOCUMENT_SCHEMA: &'static str = crate::GIS_3D_TERRAIN_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
            ArtifactToolPublicationContract { tool_id: ACTION, lanes: &[ArtifactToolPublicationLane::HostOnly] },
            ArtifactToolPublicationContract { tool_id: import_elevation::export::ACTION, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ];
    }

    #[derive(Default, semio_framework_value::RetireOwned)]
    struct Owners {
        cursor: codec::FeatureCursor,
        rows_written: usize,
        chunks: Option<ArtifactOutputChunks>,
        page: Vec<u8>,
        pending_page: Option<Vec<u8>>,
        fragment: Option<String>,
        offset: usize,
        started: bool,
        footer: bool,
    }

    #[derive(Clone, Copy)]
    enum Format { GeoJson, Raster }

    impl Format {
        fn action(self) -> &'static str {
            match self { Self::GeoJson => ACTION, Self::Raster => import_elevation::export::ACTION }
        }

        fn file(self) -> (&'static str, &'static str, usize) {
            match self {
                Self::GeoJson => (FILE_NAME, MIME_TYPE, MAX_BYTES),
                Self::Raster => (import_elevation::export::FILE_NAME, import_elevation::export::MIME_TYPE, import_elevation::MAX_FILE_BYTES as usize),
            }
        }

        fn maximum_fragment(self) -> usize {
            match self {
                Self::GeoJson => 64 * 1024,
                Self::Raster => ascii_grid::MAX_HEADER_BYTES.max(ascii_grid::MAX_CELL_BYTES * ELEVATION_MAX_SIDE),
            }
        }

        fn status(self, pending: bool) -> Step {
            let (stage, preview) = match self {
                Self::GeoJson => (STAGE, PREVIEW),
                Self::Raster => ("terrain-elevation-export", r#"{"en":"Writing elevation rows","de":"Höhenrasterzeilen werden geschrieben"}"#.as_bytes()),
            };
            if pending { Step::Pending { stage, preview } } else { Step::Progress { stage, preview } }
        }
    }

    pub(crate) struct Work {
        format: Format,
        owners: ControlledRetirement<Owners>,
    }

    impl Work {
        pub(crate) fn for_action(action: &str) -> Self {
            let format = if action == import_elevation::export::ACTION { Format::Raster } else { Format::GeoJson };
            Self { format, owners: ControlledRetirement::retain(Owners::default()) }
        }
    }

    impl ArtifactCommandWork<App> for Work {
        fn tool_id(&self) -> &'static str { self.format.action() }

        fn extent(&self, command: &crate::editor::gis3d::Gis3dCommand, _: &GisTerrainSnapshot, _: &protocol::InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>) -> Option<usize> {
            (command.command_id() == self.format.action()).then_some(1)
        }

        fn receive_original_output_chunks(&mut self, chunks: &mut Option<ArtifactOutputChunks>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
            let owners = self.owners.original_mut().ok_or_else(|| ValueError::invariant("GeoJSON export is closing"))?;
            if owners.chunks.is_some() {
                return Ok(true);
            }
            let demand = RetirementDemand { copy_bytes: size_of::<Option<ArtifactOutputChunks>>() * 2, depth: 1, ..Default::default() };
            if cx.is_cancelled() || cx.should_yield() || !cx.retained_work_can_enter(demand) {
                return Ok(false);
            }
            owners.chunks = Some(chunks.take().ok_or_else(|| ValueError::invariant("GeoJSON export lost its output queue"))?);
            charge(cx, demand)?;
            Ok(true)
        }

        fn step(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut StepContext<'_>) -> Result<Step, OriginalTurnFailure> {
            let format = self.format;
            let (filename, mime_type, _) = format.file();
            let owners = self.owners.original_mut().ok_or_else(|| ValueError::invariant("GeoJSON export is closing"))?;
            if cx.is_cancelled() || cx.should_yield() {
                return Ok(format.status(true));
            }
            let chunks = owners.chunks.as_ref().ok_or_else(|| ValueError::invariant("GeoJSON export requires its supplied queue"))?;
            if owners.pending_page.is_some() {
                return Ok(if chunks.push_original(&mut owners.pending_page, None, cx)? { format.status(false) } else { format.status(true) });
            }
            if owners.fragment.is_some() {
                return copy_fragment(format, owners, cx);
            }
            if !owners.footer {
                return next_fragment(format, owners, input.snapshot, cx);
            }
            if !owners.page.is_empty() && chunks.bytes() == 0 {
                let demand = RetirementDemand { copy_bytes: size_of::<String>() + size_of::<Step>(), capacity_bytes: filename.len() + mime_type.len(), depth: 1, ..Default::default() };
                if !cx.retained_work_can_enter(demand) { return Ok(format.status(true)); }
                let data = String::from_utf8(std::mem::take(&mut owners.page)).map_err(|_| ValueError::invariant("GeoJSON writer produced invalid UTF-8"))?;
                charge(cx, demand)?;
                return Ok(Step::Complete(Emit::effect(Effect::DownloadMediaExport { filename: filename.into(), mime_type: mime_type.into(), data, encoding: None })));
            }
            if !owners.page.is_empty() {
                let demand = RetirementDemand { copy_bytes: 2 * size_of::<Vec<u8>>(), depth: 1, ..Default::default() };
                if !cx.retained_work_can_enter(demand) { return Ok(format.status(true)); }
                owners.pending_page = Some(std::mem::take(&mut owners.page));
                charge(cx, demand)?;
                return Ok(format.status(false));
            }
            if !chunks.is_sealed() {
                return Ok(if chunks.seal_original(cx)? { format.status(false) } else { format.status(true) });
            }
            let demand = RetirementDemand { copy_bytes: size_of::<ArtifactDownloadOutput>() + size_of::<Step>(), capacity_bytes: filename.len() + mime_type.len(), depth: 1, ..Default::default() };
            if !cx.retained_work_can_enter(demand) { return Ok(format.status(true)); }
            let chunks = owners.chunks.take().ok_or_else(|| ValueError::invariant("GeoJSON download lost its sealed queue"))?;
            let download = ArtifactDownloadOutput::new(filename, mime_type, None, chunks)?;
            charge(cx, demand)?;
            Ok(Step::CompleteDownload { download, ephemeral: EphemeralEmit::default() })
        }

        fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
            let body = cx.retained_grant().maximum_copy_bytes;
            let demand = self.owners.next_demand(body)?;
            semio_framework_job::close_original_effect_step(cx, demand, |effect| self.owners.step(body, effect))
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owners.next_demand(0)?.copy_bytes) }
        fn next_close_capacity_byte_demand(&self, size: usize) -> Result<usize, ValueError> { Ok(self.owners.next_demand(size)?.capacity_bytes) }
        fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owners.next_demand(0)?.release_bytes) }
        fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.owners.next_demand(0)?.depth) }
        fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
        fn terminal_frame_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
    }

    fn next_fragment(format: Format, owners: &mut Owners, snapshot: &GisTerrainSnapshot, cx: &mut StepContext<'_>) -> Result<Step, OriginalTurnFailure> {
        let mut demand = RetirementDemand { copy_bytes: size_of::<String>(), capacity_bytes: format.maximum_fragment(), depth: 1, ..Default::default() };
        if !cx.retained_work_can_enter(demand) { return Ok(format.status(true)); }
        let fragment = match format {
            Format::GeoJson => {
                if !owners.started {
                    owners.started = true;
                    String::from(codec::HEADER)
                } else if let Some(feature) = owners.cursor.next(snapshot.imported_map.as_ref()).map_err(Error::fault)? {
                    feature
                } else {
                    owners.footer = true;
                    String::from(codec::FOOTER)
                }
            }
            Format::Raster => {
                let source = snapshot.elevation_source.as_ref().ok_or_else(|| ascii_grid::Error::Missing.fault())?;
                if !owners.started {
                    owners.started = true;
                    ascii_grid::encode_header(source).map_err(|error| error.fault())?
                } else if let Some(row) = source.rows_millimetres.iter().rev().nth(owners.rows_written) {
                    let mut fragment = String::with_capacity(row.len() * ascii_grid::MAX_CELL_BYTES);
                    ascii_grid::write_row(&mut fragment, source, source.rows_millimetres.len() - 1 - owners.rows_written);
                    owners.rows_written += 1;
                    fragment
                } else {
                    owners.footer = true;
                    cx.consume_fuel(1);
                    return Ok(format.status(false));
                }
            }
        };
        let produced = owners.chunks.as_ref().map_or(0, ArtifactOutputChunks::bytes) + owners.page.len();
        if produced + fragment.len() > format.file().2 {
            return Err(match format {
                Format::GeoJson => Error::Extent.fault(),
                Format::Raster => ascii_grid::Error::FileSize.fault(),
            }.into());
        }
        demand.capacity_bytes = fragment.capacity();
        owners.offset = 0;
        owners.fragment = Some(fragment);
        charge(cx, demand)?;
        Ok(format.status(false))
    }

    fn copy_fragment(format: Format, owners: &mut Owners, cx: &mut StepContext<'_>) -> Result<Step, OriginalTurnFailure> {
        let fragment = owners.fragment.as_ref().ok_or_else(|| ValueError::invariant("GeoJSON fragment disappeared"))?;
        let remaining = &fragment.as_bytes()[owners.offset..];
        let carrier = 2 * size_of::<Vec<u8>>();
        let copy_grant = cx.retained_grant().maximum_copy_bytes.saturating_sub(carrier);
        let bytes = remaining.len().min(ArtifactOutputChunks::CHUNK_BYTES - owners.page.len()).min(copy_grant);
        if bytes == 0 { return Ok(format.status(true)); }
        let mut demand = RetirementDemand {
            copy_bytes: bytes + carrier,
            capacity_bytes: if owners.page.capacity() == 0 { ArtifactOutputChunks::CHUNK_BYTES } else { 0 },
            release_bytes: if bytes == remaining.len() { fragment.capacity() } else { 0 },
            depth: 1,
        };
        if !cx.retained_work_can_enter(demand) { return Ok(format.status(true)); }
        if demand.capacity_bytes != 0 {
            owners.page.reserve_exact(ArtifactOutputChunks::CHUNK_BYTES);
            demand.capacity_bytes = owners.page.capacity();
        }
        owners.page.extend_from_slice(&remaining[..bytes]);
        owners.offset += bytes;
        if owners.offset == fragment.len() {
            owners.fragment = None;
        }
        if owners.page.len() == ArtifactOutputChunks::CHUNK_BYTES {
            owners.pending_page = Some(std::mem::take(&mut owners.page));
        }
        charge(cx, demand)?;
        Ok(format.status(false))
    }

    fn charge(cx: &mut StepContext<'_>, demand: RetirementDemand) -> Result<(), OriginalTurnFailure> {
        cx.consume_retained(RetainedCloneProgress {
            copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes,
        })?;
        cx.consume_fuel(1);
        Ok(())
    }


}

trait ErrorNotice {
    fn details(&self) -> (&'static str, &'static str, &'static str);
    fn fault(self) -> Fault;
}

impl ErrorNotice for Error {
    fn details(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::File => ("gis.terrain.geojson.file", "Choose a complete GeoJSON file of 24 KiB or smaller.", "Wähle eine vollständige GeoJSON-Datei mit höchstens 24 KiB."),
            Self::Collection => ("gis.terrain.geojson.collection", "Choose a GeoJSON FeatureCollection.", "Wähle eine GeoJSON-FeatureCollection."),
            Self::Detail(_) => ("gis.terrain.geojson.invalid", "GeoJSON cannot be exchanged because its structure or coordinates are invalid. {detail}", "Die GeoJSON-Struktur oder Koordinaten sind ungültig. {detail}"),
            Self::Geometry => ("gis.terrain.geojson.geometry", "Use Point, LineString or Polygon features with two or three coordinate values per position.", "Verwende Punkte, Linien oder Polygone mit zwei oder drei Koordinatenwerten pro Position."),
            Self::Extent => ("gis.terrain.geojson.extent", "The feature data exceeds the local GeoJSON exchange limit.", "Die Objektdaten überschreiten die Grenze für den lokalen GeoJSON-Austausch."),
        }
    }

    fn fault(self) -> Fault {
        let (code, en, _) = self.details();
        match self {
            Self::Detail(detail) => Fault::new(FaultOrigin::App, FaultCode::new(code), en.replace("{detail}", &detail)).with_param("detail", detail),
            _ => Fault::new(FaultOrigin::App, FaultCode::new(code), en),
        }
    }
}

/// 🔔️ Registers bilingual GeoJSON exchange faults with the Terrain editor.
pub(crate) fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&'static str, LocalizedLabel); 5]> = std::sync::LazyLock::new(|| {
        [Error::File, Error::Collection, Error::Detail(String::new()), Error::Geometry, Error::Extent].map(|error| {
            let (code, en, de) = error.details();
            (code, LocalizedLabel::native(en, de))
        })
    });
    NOTICES.as_slice()
}

mod codec {
    pub(super) use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::text::geojson::{Error, FeatureCursor, FOOTER, HEADER, encode};
    pub(super) fn decode(text: &str) -> Result<super::ImportedMap, Error> {
        semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::text::geojson::decode(text, super::local_file::MAX_FILE_BYTES as usize)
    }
}

