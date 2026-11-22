"""📤️ Stage Esri export through the same acknowledged paging as GeoJSON."""

from difflib import unified_diff
from pathlib import Path


TICKET = Path(__file__).resolve().parent
APP = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
COMMANDS = APP / "🎮️commands"
GEOJSON = COMMANDS / "🌍️geojson/🦀️.rs"
RASTER = COMMANDS / "📥️import-elevation/🦀️.rs"
EXPORT = RASTER.parent / "📤️export/🦀️.rs"
EDITOR = APP / "🦀️.rs"
LIMITS = APP / "🧪️tests/🧫️fixtures/🧫️retained-command-limits/🔣️.json"


def change(source, before, after):
    if source.count(before) != 1:
        raise ValueError(before)
    return source.replace(before, after)


def diff(path, before, after):
    return "".join(unified_diff(before.splitlines(True), after.splitlines(True), "a/" + str(path), "b/" + str(path)))


def transform_geojson(source):
    source = source.replace("GeoJsonExportFactory", "LocalFileExportJobFactory")
    source = change(source, "    use super::*;\n    use semio_framework::{InteractiveJobClassification",
                    "    use super::*;\n    use super::super::import_elevation::{self, ErrorNotice, ascii_grid};\n    use crate::schema::snapshot::ELEVATION_MAX_SIDE;\n    use semio_framework::{InteractiveJobClassification")
    source = change(source, "pub(crate) struct LocalFileExportJobFactory([ToolFactoryKey; 1]);",
                    "pub(crate) struct LocalFileExportJobFactory([ToolFactoryKey; 2]);")
    source = change(source, "Self([ToolFactoryKey::new(controller, ACTION)])",
                    "Self([ACTION, import_elevation::export::ACTION].map(|id| ToolFactoryKey::new(controller, id)))")
    source = change(source, 'tools: { "exportGeoJson" => ToolExecutionContract::resumable(MAX_RAW_BYTES, 32, 4096, MAX_BYTES, 7_500, 64, 1) }',
                    '''tools: {
                "exportGeoJson" => ToolExecutionContract::resumable(MAX_RAW_BYTES, 32, 4096, MAX_BYTES, 7_500, 64, 1),
                "exportElevationGrid" => ToolExecutionContract::resumable(MAX_RAW_BYTES, 32, 4096, MAX_BYTES, 7_500, 64, 1),
            }''')
    source = change(source, "const TOOL_IDS: &'static [&'static str] = &[ACTION];",
                    "const TOOL_IDS: &'static [&'static str] = &[ACTION, import_elevation::export::ACTION];")
    source = change(source, "            ArtifactToolPublicationContract { tool_id: ACTION, lanes: &[ArtifactToolPublicationLane::HostOnly] },",
                    '''            ArtifactToolPublicationContract { tool_id: ACTION, lanes: &[ArtifactToolPublicationLane::HostOnly] },
            ArtifactToolPublicationContract { tool_id: import_elevation::export::ACTION, lanes: &[ArtifactToolPublicationLane::HostOnly] },''')
    source = change(source, "gis.terrain.geojson-download.tool-command.v1", "gis.terrain.local-file-download.tool-command.v1")
    source = change(source, "        cursor: codec::FeatureCursor,", "        cursor: codec::FeatureCursor,\n        rows_written: usize,")
    start = source.index("    pub(crate) struct Work {")
    end = source.index("        fn receive_original_output_chunks", start)
    source = source[:start] + '''    #[derive(Clone, Copy)]
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

''' + source[end:]
    start = source.index("        fn step(&mut self, input:")
    end = source.index("        fn close_step(", start)
    step = source[start:end].replace('            let owners =', '            let format = self.format;\n            let (filename, mime_type, _) = format.file();\n            let owners =', 1)
    step = step.replace("pending()", "format.status(true)").replace("progress()", "format.status(false)")
    step = step.replace("copy_fragment(owners, cx)", "copy_fragment(format, owners, cx)").replace("next_fragment(owners, input.snapshot, cx)", "next_fragment(format, owners, input.snapshot, cx)")
    step = step.replace("FILE_NAME.len() + MIME_TYPE.len()", "filename.len() + mime_type.len()").replace("ArtifactDownloadOutput::new(FILE_NAME, MIME_TYPE,", "ArtifactDownloadOutput::new(filename, mime_type,")
    step = step.replace("Step::Complete(inline(data))", "Step::Complete(Emit::effect(Effect::DownloadMediaExport { filename: filename.into(), mime_type: mime_type.into(), data, encoding: None }))")
    source = source[:start] + step + source[end:]
    start = source.index("    fn next_fragment(")
    end = source.index("    fn copy_fragment(", start)
    source = source[:start] + '''    fn next_fragment(format: Format, owners: &mut Owners, snapshot: &GisTerrainSnapshot, cx: &mut StepContext<'_>) -> Result<Step, OriginalTurnFailure> {
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
                    ascii_grid::encode_header(source).map_err(ErrorNotice::fault)?
                } else if let Some(row) = source.rows_millimetres.iter().rev().nth(owners.rows_written) {
                    let mut fragment = String::with_capacity(row.len() * ascii_grid::MAX_CELL_BYTES);
                    ascii_grid::write_row(&mut fragment, row);
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

''' + source[end:]
    start = source.index("    fn copy_fragment(")
    end = source.index("    fn charge(", start)
    body = source[start:end].replace("fn copy_fragment(owners", "fn copy_fragment(format: Format, owners").replace("pending()", "format.status(true)").replace("progress()", "format.status(false)")
    source = source[:start] + body + source[end:]
    start = source.index("    fn progress() -> Step")
    end = source.index("\n}", start)
    source = source[:start] + source[end:]
    return source


def transform_editor(source):
    source = source.replace("GeoJsonExportFactory", "LocalFileExportJobFactory")
    assert source.count('"exportElevationGrid", "gradeTerrainRegion"') == 2
    source = source.replace('"exportElevationGrid", "gradeTerrainRegion"', '"gradeTerrainRegion"')
    source = change(source, '        ArtifactToolPublicationContract { tool_id: "exportElevationGrid", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n', "")
    source = change(source, "let geojson_export = original.tool_id == export_geojson::ACTION;",
                    "let file_export = matches!(original.tool_id.as_str(), export_geojson::ACTION | export_elevation_grid::ACTION);")
    source = change(source, "!file_import && !geojson_export", "!file_import && !file_export")
    source = change(source, "if geojson_export {", "if file_export {")
    source = change(source, "|_| export_geojson::Work::default()", "move |_| export_geojson::Work::for_action(tool_id)")
    return source


before = EXPORT.read_text()
after = change(before, 'pub const ACTION: &str = "exportElevationGrid";',
               'pub const ACTION: &str = "exportElevationGrid";\npub(crate) const FILE_NAME: &str = "terrain-elevation.asc";\npub(crate) const MIME_TYPE: &str = "text/plain;charset=utf-8";')
after = after.replace('filename: "terrain-elevation.asc".into()', 'filename: FILE_NAME.into()').replace('mime_type: "text/plain;charset=utf-8".into()', 'mime_type: MIME_TYPE.into()')
patch = [diff(EXPORT, before, after)]
before = LIMITS.read_text()
after = "".join(line for line in before.splitlines(keepends=True) if '"exportElevationGrid"' not in line)
assert len(after) < len(before)
patch.append(diff(LIMITS, before, after))
for path, transform in [(GEOJSON, transform_geojson), (EDITOR, transform_editor)]:
    before = path.read_text()
    patch.append(diff(path, before, transform(before)))
(TICKET / "terrain-rectangular-export-work.patch").write_text("".join(patch))
