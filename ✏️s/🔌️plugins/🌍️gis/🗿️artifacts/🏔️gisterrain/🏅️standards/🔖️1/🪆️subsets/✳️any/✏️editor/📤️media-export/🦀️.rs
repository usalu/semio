//! 📤️ GIS Terrain's borrowed structured workflow exports.
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
enum TerrainJsonView<'a> {
    Elevation(ElevationJsonView<'a>),
    Scene(TerrainSceneView<'a>),
}

impl JsonWriteSource for TerrainJsonView<'_> {
    fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> {
        match self {
            Self::Elevation(view) => view.node_at_path(path),
            Self::Scene(view) => view.node_at_path(path),
        }
    }

    fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        match self {
            Self::Elevation(view) => view.object_key_at_path(path, index),
            Self::Scene(view) => view.object_key_at_path(path, index),
        }
    }
}

impl StructuredMediaSource for TerrainMedia {
    type App = EditorApp<Gis3dPlayApp>;
    type Projection = TerrainSceneRange;
    const TOOL_IDS: &'static [&'static str] = TOOLS;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: TOOLS[0], lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: TOOLS[1], lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
    const MAX_BYTES: usize = 1_048_576;
    const PREPARATION_COPY_BYTES: usize = 33 * size_of::<i16>() + size_of::<TerrainSceneRange>();

    fn descriptor(port: &str) -> Option<(MediaType, &'static str)> {
        match port {
            "elevation:out" => Some((GIS_ELEVATION_MEDIA_TYPE, GIS_ELEVATION_GRID_SCHEMA)),
            "scene:out" => Some((MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, "mesh.document")),
            _ => None,
        }
    }

    fn prepare(port: &str, snapshot: &GisTerrainSnapshot, state: &mut TerrainSceneRange) -> Result<bool, &'static str> {
        let Some(source) = snapshot.elevation_source.as_ref() else {
            return Err(if port == "elevation:out" { "no authored elevation source" } else { "This terrain has no elevation source" });
        };
        let [west, south, east, north] = source.bounds;
        if !((-180.0..east).contains(&west) && east <= 180.0 && (-90.0..north).contains(&south) && north <= 90.0) {
            return Err(if port == "elevation:out" { "no authored elevation source" } else { "The authored terrain elevation grid is invalid" });
        }
        let complete = state.step(&source.rows).map_err(|_| if port == "elevation:out" { "no authored elevation source" } else { "The authored terrain elevation grid is invalid" })?;
        if port == "scene:out" && complete {
            if !(0.0..=f64::from(f32::MAX) / f64::from(i16::MIN.unsigned_abs())).contains(&snapshot.exaggeration) {
                return Err("Terrain elevation cannot be projected in the defined world coordinates");
            }
            let origin = terrain_grid_origin(source.bounds);
            state.geometry = Some(TerrainGridMeshProjection::from_bounds(source.bounds, [origin.lon, origin.lat], snapshot.exaggeration));
        }
        Ok(complete)
    }

    fn borrow<'a>(port: &str, snapshot: &'a GisTerrainSnapshot, range: &'a TerrainSceneRange) -> Result<impl JsonWriteSource + 'a, ValueError> {
        let source = snapshot.elevation_source.as_ref()
            .ok_or_else(|| ValueError::invariant("the prepared terrain projection has no elevation source"))?;
        if port == "elevation:out" {
            return Ok(TerrainJsonView::Elevation(ElevationJsonView(source)));
        }
        let projection = range.geometry.ok_or_else(|| ValueError::invariant("the prepared terrain mesh has no geometry"))?;
        Ok(TerrainJsonView::Scene(TerrainSceneView { rows: &source.rows, projection, range: *range }))
    }
}

pub(super) type Factory = structured_media_export::StructuredMediaExportFactory<TerrainMedia>;
pub(super) type Export = structured_media_export::StructuredMediaExport<TerrainMedia>;
