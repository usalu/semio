from hashlib import sha256
from pathlib import Path
import json
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated" / "gis-next"
generated.mkdir(exist_ok=True)
if sys.argv[1:] == ["--apply"]:
    index = json.loads((generated / "candidates.json").read_text())
    for row in index.values():
        path = Path(row["path"])
        assert (sha256(path.read_bytes()).hexdigest() if path.exists() else None) == row["base"], path
    for name, row in index.items():
        path = Path(row["path"])
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes((generated / name).read_bytes())
    sys.exit(0)

map_root = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
terrain_root = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any")
plugin = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/📤️structured")
surface = Path("🧰️framework/🔨️modules/🗺️surface/🏔️terrain")
index = {}

def proto(name):
    return (ticket / f"gis-{name}.rs").read_text()

def save(name, path, candidate):
    base = sha256(path.read_bytes()).hexdigest() if path.exists() else None
    index[name] = {"path": str(path), "base": base}
    (generated / name).write_text(candidate)

media = (plugin / "🦀️.rs").read_text()
media = media.replace("RetirementPollExit, ValueError,", "RetirementPollExit, NativeEncodeControl, ValueError, ValueRefusalKind,")
media = media.replace("OriginalJobReturnClaim, OriginalTurnFailure", "OriginalJobReturnClaim, OriginalSourceBinding, OriginalTurnFailure")
media = media.replace("use crate::media_contract", "use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonWriteSource};\nuse crate::media_contract")
start = media.index("/// 📦️ Type, schema")
end = media.index("/// 🎟️ Registers", start)
media = media[:start] + """const JSON_ACTIONS_PER_TURN: usize = 16;

/// 🎞️ Binds bounded borrowed media projection to an application's registered output ports.
pub trait StructuredMediaSource: 'static {
    type App: ArtifactApp;
    type Projection: Default + Send + semio_framework_value::RetireOwned;
    const TOOL_IDS: &'static [&'static str];
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract];
    const MAX_BYTES: usize;
    const PREPARATION_COPY_BYTES: usize;
    const OVERSIZE: &'static str = "media exceeds its bounded output size";
    fn descriptor(port: &str) -> Option<(MediaType, &'static str)>;
    fn prepare(port: &str, snapshot: &<Self::App as ArtifactApp>::Snapshot, projection: &mut Self::Projection) -> Result<bool, &'static str>;
    fn borrow<'a>(port: &str, snapshot: &'a <Self::App as ArtifactApp>::Snapshot, projection: &'a Self::Projection) -> Result<impl JsonWriteSource + 'a, ValueError>;
}

""" + media[end:]
media = media.replace("resumable(4_096, 1, 1,", "resumable(4_096, 1, JSON_ACTIONS_PER_TURN as u64,")
start = media.index("#[derive(Default, semio_framework_value::RetireOwned)]")
end = media.index("/// 🏗️ Advances", start)
buffers = proto("structured-media-buffers")
buffers, _ = buffers.split("\nfn advance_projection", 1)
buffers = buffers.replace("semio_framework_pack_json::JsonBorrowedWriteCursor", "JsonBorrowedWriteCursor")
buffers = buffers.replace("    projection: P,", "    projection: P,\n    source: Option<OriginalSourceBinding>,").replace("projection: P::default(), cursor,", "projection: P::default(), source: None, cursor,")
media = media[:start] + buffers + "\n" + media[end:]
media = media.replace("Option<Buffers>", "Option<Buffers<S::Projection>>").replace("::<Buffers>", "::<Buffers<S::Projection>>")
media = media.replace("                || !S::TOOL_IDS.contains(&request.tool_id.as_str())", "                || !S::TOOL_IDS.contains(&request.tool_id.as_str())\n                || S::descriptor(&request.port).is_none()")
media = media.replace("let bytes = size_of::<Request<S>>() + size_of::<Buffers<S::Projection>>();", """let original = request.as_ref().ok_or_else(|| ValueError::invariant("structured media export lost its original input"))?;
        let (_, schema) = S::descriptor(&original.port).ok_or_else(|| ValueError::invariant("structured media export received a foreign port"))?;
        let bytes = size_of::<Request<S>>() + size_of::<Buffers<S::Projection>>();""")
media = media.replace("owner.buffers = Some(Buffers::default());", "owner.buffers = Some(Buffers::new(schema.len(), S::MAX_BYTES));")
start = media.index("            cx.set_stage(\"structured-media-project\");")
end = media.index("        } else if buffers.page.is_some()", start)
media = media[:start] + "            advance_projection::<S>(request, buffers, cx)?;\n" + media[end:]
start = media.index("fn error_capacity")
end = media.index("impl<S: StructuredMediaSource>", start)
media = media[:start] + """#[path = "🔤️json/🦀️.rs"]
mod json;
use json::advance_projection;

""" + media[end:]
save("job.rs", plugin / "🦀️.rs", media)

turn = """//! 🔤️ Paid traversal of an original snapshot's borrowed JSON view.
use super::*;

pub(super) fn advance_projection<S: StructuredMediaSource>(
    request: &Request<S>,
    buffers: &mut Buffers<S::Projection>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    cx.set_stage("structured-media-project");
    let (media_type, schema) = S::descriptor(&request.port)
        .ok_or_else(|| ValueError::invariant("structured media export received a foreign port"))?;
    cx.original_source_parent()?.validate_borrowed_context(cx)?;
    if let Some(source) = buffers.source {
        if !source.matches(cx) {
            return Err(ValueError::invariant("structured media projection changed original Source").into());
        }
    } else {
        let bytes = size_of::<Option<OriginalSourceBinding>>();
        if !cx.retained_work_can_enter(header(bytes)) {
            return Ok(());
        }
        buffers.source = Some(OriginalSourceBinding::borrow(cx));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        return Ok(());
    }
    if let Some(message) = buffers.failure {
        let bytes = request.port.len() + message.len();
        let demand = RetirementDemand { capacity_bytes: bytes, ..header(size_of::<ResultPage>() + bytes) };
        if !cx.retained_work_can_enter(demand) {
            return Ok(());
        }
        buffers.result = Some(Err(MediaError::Payload(request.port.clone(), message.into())));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        return Ok(());
    }
    if !buffers.prepared {
        return prepare::<S>(request, buffers, cx);
    }
    if buffers.json.is_none() {
        return write::<S>(request, buffers, cx);
    }
    let demand = RetirementDemand {
        capacity_bytes: schema.len(),
        ..header(size_of::<(MediaType, String, Vec<u8>)>() + schema.len())
    };
    if !cx.retained_work_can_enter(demand) {
        return Ok(());
    }
    buffers.encoded = Some((media_type, schema.to_owned(), buffers.json.take().unwrap()));
    cx.consume_retained(RetainedCloneProgress {
        copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: schema.len(), ..Default::default()
    })?;
    cx.consume_fuel(1);
    Ok(())
}

fn prepare<S: StructuredMediaSource>(
    request: &Request<S>,
    buffers: &mut Buffers<S::Projection>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let bytes = S::PREPARATION_COPY_BYTES + size_of::<Option<&'static str>>() + size_of::<bool>();
    if !cx.retained_work_can_enter(header(bytes)) {
        return Ok(());
    }
    let grant = cx.retained_grant();
    let Some((result, performed)) = cx.with_original_source_effect(grant, |cx, effect| -> Result<_, ValueError> {
        let actions = JSON_ACTIONS_PER_TURN
            .min(effect.remaining_work_units()?.try_into().unwrap_or(usize::MAX))
            .min(grant.maximum_items)
            .min(grant.maximum_copy_bytes / bytes);
        for _ in 0..actions {
            if cx.should_yield() || !effect.performed(1)? {
                break;
            }
            match S::prepare(&request.port, request.snapshot_read.get(), &mut buffers.projection) {
                Ok(true) => { buffers.prepared = true; break; }
                Ok(false) => (),
                Err(message) => { buffers.failure = Some(message); break; }
            }
        }
        Ok(())
    })? else { return Ok(()) };
    cx.consume_retained(RetainedCloneProgress {
        copied_items: performed as usize, copied_bytes: bytes * performed as usize, ..Default::default()
    })?;
    cx.consume_fuel(performed as u64);
    Ok(result?)
}

fn write<S: StructuredMediaSource>(
    request: &Request<S>,
    buffers: &mut Buffers<S::Projection>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let projection = &buffers.projection;
    write_borrowed(
        || S::borrow(&request.port, request.snapshot_read.get(), projection),
        &mut buffers.cursor, &mut buffers.json, &mut buffers.failure, S::OVERSIZE, cx,
    )
}

fn write_borrowed<V: JsonWriteSource>(
    source: impl FnOnce() -> Result<V, ValueError>,
    cursor: &mut JsonBorrowedWriteCursor,
    output: &mut Option<Vec<u8>>,
    failure: &mut Option<&'static str>,
    oversize: &'static str,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let mut grant = cx.retained_grant();
    let metadata = size_of::<Option<Vec<u8>>>().max(size_of::<Option<&'static str>>());
    if cx.should_yield() || grant.maximum_copy_bytes < metadata {
        return Ok(());
    }
    grant.maximum_copy_bytes -= metadata;
    let total = cx.original_total_retained_grant();
    let Some((result, performed)) = cx.with_original_source_effect(grant, |cx, effect| {
        let source = source()?;
        let actions = JSON_ACTIONS_PER_TURN.min(effect.remaining_work_units()?.try_into().unwrap_or(usize::MAX));
        let mut live = |_| !cx.is_cancelled();
        let mut native = NativeEncodeControl::new_retained(&mut live);
        let result = cursor.step_with_demand(&source, actions, &mut native, effect.grant());
        let mut progress = cursor.normal_step_progress();
        if progress.retained_capacity_bytes != 0 {
            *effect.actual() = Some(progress.retained_capacity_bytes);
        }
        if !effect.performed(cursor.last_turn_work().max(1))? {
            return Err(ValueError::invariant("structured JSON exceeded its admitted work"));
        }
        let result = match result {
            Ok((Some(bytes), _)) => {
                *output = Some(bytes.into_bytes());
                progress.copied_bytes += size_of::<Option<Vec<u8>>>();
                Ok(())
            }
            Ok((None, Some(demand))) if demand.copy_bytes > total.maximum_copy_bytes.saturating_sub(metadata)
                || demand.capacity_bytes > total.maximum_capacity_bytes
                || demand.depth > total.maximum_depth => Err(ValueError::literal(ValueRefusalKind::WorkLimit, "structured JSON cannot fund its next source phase")),
            Ok(_) => Ok(()),
            Err(error) if error.kind == ValueRefusalKind::OwnershipLimit && error.message == "controlled JSON output exceeds caller limit" => {
                *failure = Some(oversize);
                progress.copied_items = progress.copied_items.max(1);
                progress.copied_bytes += size_of::<Option<&'static str>>();
                Ok(())
            }
            Err(error) => Err(error),
        };
        Ok::<_, ValueError>((result, progress))
    })? else { return Ok(()) };
    let (body, progress) = result?;
    cx.consume_retained(progress)?;
    cx.consume_fuel(performed as u64);
    Ok(body?)
}
"""
save("json.rs", plugin / "🔤️json/🦀️.rs", turn)

owners = proto("structured-projection-owners")
map_head, terrain_head = owners.split("\nmod terrain {", 1)
map_impl = map_head.split("    impl StructuredMediaSource", 1)[1].rsplit("\n}", 1)[0]
map_impl = "\n".join(line[4:] if line.startswith("    ") else line for line in ("    impl StructuredMediaSource" + map_impl).splitlines())
map_impl = map_impl.replace("    const PREPARATION_COPY_BYTES", '    const OVERSIZE: &\'static str = "map exceeds the bounded output size";\n    const PREPARATION_COPY_BYTES')
map_source = (map_root / "🧭️play-app-editor/🦀️.rs").read_text()
start = map_source.index("use semio_plugin::structured_media_export")
end = map_source.index("//#region", start)
map_source = map_source[:start] + """#[path = "../📤️map-export/🦀️.rs"]
mod map_export;
use map_export::{MapMedia, StructuredMediaExport, StructuredMediaExportFactory};

""" + map_source[end:]
save("map-app.rs", map_root / "🧭️play-app-editor/🦀️.rs", map_source)
map_owner = """//! 🗺️ The editor's mounted borrowed Map media projection.
use super::{EditorApp, Gis2dPlayApp, GisMapSnapshot, MapFeature, MediaClass, MediaForm, MediaType};
use semio_framework_pack_json::JsonWriteSource;
use semio_framework_value::ValueError;
use semio_plugin::{ArtifactToolPublicationContract, ArtifactToolPublicationLane, structured_media_export::StructuredMediaSource};
pub(super) use semio_plugin::structured_media_export::{StructuredMediaExport, StructuredMediaExportFactory};
use super::store;

#[path = "🪟️view/🦀️.rs"]
mod view;
#[path = "✅️validation/🦀️.rs"]
mod validation;
use view::MapJsonView;
use validation::MapJsonValidation;

pub(super) struct MapMedia;
""" + map_impl + "\n"
save("map.rs", map_root / "📤️map-export/🦀️.rs", map_owner)
save("map-view.rs", map_root / "📤️map-export/🪟️view/🦀️.rs", "use super::{GisMapSnapshot, MapFeature};\n" + proto("map-json-view"))
save("map-validation.rs", map_root / "📤️map-export/✅️validation/🦀️.rs", proto("map-json-validation"))
terrain_impl = terrain_head.split("    enum TerrainJsonView", 1)[1].rsplit("\n}", 1)[0]
terrain_impl = "\n".join(line[4:] if line.startswith("    ") else line for line in ("    enum TerrainJsonView" + terrain_impl).splitlines())
terrain_owner = """//! 📤️ GIS Terrain's borrowed structured workflow exports.
use super::{EditorApp, GIS_ELEVATION_GRID_SCHEMA, GIS_ELEVATION_MEDIA_TYPE, Gis3dPlayApp, GisTerrainSnapshot};
use semio_framework::{MediaClass, MediaForm, MediaType};
use semio_framework_pack_json::{JsonWriteNode, JsonWriteSource};
use semio_framework_value::ValueError;
use semio_plugin::{ArtifactToolPublicationContract, ArtifactToolPublicationLane, structured_media_export::{self, StructuredMediaSource}};
use crate::schema::imported_map::{TerrainGridMeshProjection, terrain_grid_origin};

#[path = "🏔️elevation/🦀️.rs"]
mod elevation;
#[path = "📐️scene/🦀️.rs"]
mod scene;
use elevation::ElevationJsonView;
use scene::{TerrainSceneRange, TerrainSceneView};

const TOOLS: &[&str] = &["export-media:elevation:out", "export-media:scene:out"];
pub(super) struct TerrainMedia;
""" + terrain_impl + """

pub(super) type Factory = structured_media_export::StructuredMediaExportFactory<TerrainMedia>;
pub(super) type Export = structured_media_export::StructuredMediaExport<TerrainMedia>;
"""
save("terrain.rs", terrain_root / "✏️editor/📤️media-export/🦀️.rs", terrain_owner.replace("snapshot.exaggeration, 33)", "snapshot.exaggeration)"))
save("elevation.rs", terrain_root / "✏️editor/📤️media-export/🏔️elevation/🦀️.rs", "use crate::schema::snapshot::ElevationSource;\n" + proto("elevation-json-view"))
save("scene.rs", terrain_root / "✏️editor/📤️media-export/📐️scene/🦀️.rs", "use super::TerrainGridMeshProjection;\n" + proto("terrain-json-view"))
imported_map = terrain_root / "🧬️schema/🗺️imported-map/🦀️.rs"
save("imported-map.rs", imported_map, imported_map.read_text() + "\npub use semio_framework_surface::terrain::TerrainGridMeshProjection;\n")
surface_file = surface / "🦀️.rs"
surface_source = surface_file.read_text()
start = surface_source.index("fn build_terrain_grid_mesh(")
end = surface_source.index("fn normalize3(", start)
surface_source = surface_source[:start] + """#[path = "📐️grid-projection/🦀️.rs"]
mod grid_projection;
pub use grid_projection::TerrainGridMeshProjection;

fn build_terrain_grid_mesh(heights: &[f32], bounds: [f64; 4], origin_lon: f64, origin_lat: f64, exaggeration: f64) -> TerrainTileMeshJson {
    let n = TERRAIN_GRID_RESOLUTION;
    let projection = TerrainGridMeshProjection::from_bounds(bounds, [origin_lon, origin_lat], exaggeration);
    let height = |row, column| heights[(row * n + column) as usize];
    let minimum = heights.iter().copied().fold(f32::INFINITY, f32::min);
    let maximum = heights.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let mut positions = Vec::with_capacity((n * n * 3) as usize);
    let mut normals = Vec::with_capacity((n * n * 3) as usize);
    let mut uvs = Vec::with_capacity((n * n * 2) as usize);
    let mut indices = Vec::with_capacity(((n - 1) * (n - 1) * 6) as usize);
    for row in 0..n {
        for column in 0..n {
            positions.extend(projection.position(row, column, height(row, column)));
            normals.extend(projection.normal(row, column, height));
            uvs.extend(TerrainGridMeshProjection::uv(height(row, column), minimum, maximum));
        }
    }
    for row in 0..n - 1 {
        for column in 0..n - 1 {
            indices.extend(projection.triangles(row, column));
        }
    }
    TerrainTileMeshJson { positions, normals, indices, uvs }
}

""" + surface_source[end:]
save("surface.rs", surface_file, surface_source)
grid = proto("terrain-mesh-projection").replace("    size: u32,\n", "").replace(", size: u32", "").replace("exaggeration, size,", "exaggeration,").replace("self.size", "TERRAIN_GRID_RESOLUTION")
save("grid.rs", surface / "📐️grid-projection/🦀️.rs", "use super::{normalize3, projection, TERRAIN_GRID_RESOLUTION};\n" + grid)
map_tests = map_root / "🧪️tests/🔬️unit/🦀️.rs"
map_laws = """
#[semio_framework_async_macros::async_test]
async fn mounted_map_export_preserves_the_original_document_descriptor() {
    use semio_plugin::ArtifactMediaExportPoll;
    use semio_plugin::artifact_app_laws::{poll_fixture_media_export, retain_fixture_media_export_result, submit_fixture_media_export, take_fixture_media_export_chunk};

    let mut app = app().await;
    let original = app.snapshot().unwrap();
    let MediaPayload::Intrinsic { schema, value } = gis2d_map_media(&original).payload else { unreachable!() };
    let expected = semio_framework_pack_json::to_json_string(&value);
    let loan = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close;
    let handle = submit_fixture_media_export(&mut app.0, "map:out", app.1.cx(), loan).await.unwrap();
    let mut result = None;
    let mut yields = 0;
    for _ in 0..10_000 {
        match poll_fixture_media_export(&mut app.0, &handle, app.1.cx(), loan).await.unwrap() {
            ArtifactMediaExportPoll::Running { .. } => yields += 1,
            ArtifactMediaExportPoll::Complete(media) => { result = Some(media); break; }
            other => panic!("Map export failed: {other:?}"),
        }
    }
    assert!(yields > 1);
    let result = result.expect("mounted Map export must complete");
    assert_eq!(result.schema, schema);
    retain_fixture_media_export_result(&mut app.0, &handle, result, app.1.cx(), loan).await.unwrap();
    let mut output = Vec::new();
    let mut terminal = false;
    while let Some(chunk) = take_fixture_media_export_chunk(&mut app.0, &handle, &mut terminal, app.1.cx(), loan).await.unwrap() {
        output.extend(chunk);
    }
    assert_eq!(output, expected.into_bytes());
    assert_eq!(app.snapshot().unwrap(), original);
    close(&mut app);
}
"""
map_test_source = map_tests.read_text().replace("//#endregion 🔖️Media", map_laws + "//#endregion 🔖️Media", 1)
save("map-test.rs", map_tests, map_test_source)
terrain_tests = terrain_root / "✏️editor/🧪️tests/🧩️mounted/🦀️.rs"
terrain_test_source = terrain_tests.read_text()
terrain_test_source = terrain_test_source.replace(
    '            assert_eq!(actual["mesh"]["positions"].as_array().unwrap().len(), 33 * 33 * 3);',
    '            let MediaPayload::Structured { json: expected, .. } = gis3d_scene_media(&initial).unwrap().payload else { unreachable!() };\n            assert_eq!(actual, serde_json::from_str::<Value>(&expected).unwrap());',
    1,
)
terrain_cancel = """
#[semio_framework_async_macros::async_test]
async fn mounted_terrain_export_can_cancel_a_live_projection_and_close() {
    use semio_plugin::{ArtifactMediaExportPoll, PluginApp};
    use semio_plugin::artifact_app_laws::{poll_fixture_media_export, submit_fixture_media_export};

    let mut app = app().await;
    let original = app.snapshot().unwrap();
    let loan = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close;
    let handle = submit_fixture_media_export(&mut *app.0, "scene:out", app.1.cx(), loan).await.unwrap();
    for _ in 0..24 {
        assert!(matches!(poll_fixture_media_export(&mut *app.0, &handle, app.1.cx(), loan).await.unwrap(), ArtifactMediaExportPoll::Running { .. }));
    }
    app.0.cancel_media_export(&handle).await.unwrap();
    assert_eq!(app.snapshot().unwrap(), original);
    close(&mut app);
}

"""
test_anchor = "#[semio_framework_async_macros::async_test]\nasync fn media_ports_declare"
assert test_anchor in terrain_test_source
save("terrain-test.rs", terrain_tests, terrain_test_source.replace(test_anchor, terrain_cancel + test_anchor, 1))
(generated / "candidates.json").write_text(json.dumps(index, ensure_ascii=False, indent=2))
