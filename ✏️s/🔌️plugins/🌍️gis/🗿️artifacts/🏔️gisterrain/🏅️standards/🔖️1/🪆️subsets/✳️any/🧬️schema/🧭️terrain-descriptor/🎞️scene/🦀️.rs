//! 🎞️ Serializes the geographic terrain scene payload consumed by World3d.

use super::{TerrainDescriptorJson, TerrainOverlays};
use semio_framework_surface::terrain::TerrainElevationGrid;
use semio_framework_value::{DslValue, ToValue as _};
use semio_framework_value_derive::ToValue;

#[derive(ToValue)]
#[value(rename_all = "camelCase")]
struct TerrainSceneStyleJson<'a> {
    #[value(serialize_with = "elevation_grid_value")]
    elevation_grid: &'a TerrainElevationGrid,
    project_origin_lon: f64,
    project_origin_lat: f64,
    exaggeration: f64,
    color_ramp: &'a str,
    #[value(skip_serializing_if = "Option::is_none")]
    attribution: Option<TerrainAttribution>,
    #[value(serialize_with = "overlay_value")]
    overlays: &'a TerrainOverlays,
}

/// 🏷️ Provider credit shown beside bundled terrain: `label` is the provider's proper noun, identical in English and German, so the scene payload stays locale-free.
#[derive(ToValue)]
struct TerrainAttribution {
    label: &'static str,
    url: &'static str,
}

fn overlay_value(overlays: &&TerrainOverlays) -> DslValue {
    overlays.to_value()
}

fn elevation_grid_value(grid: &&TerrainElevationGrid) -> DslValue {
    grid.to_value()
}

/// 🏗️ Encodes the DEM source and its GIS overlays as a single terrain scene payload.
pub fn terrain_scene_json(descriptor: &TerrainDescriptorJson, elevation_grid: &TerrainElevationGrid, bundled: bool) -> String {
    let style = TerrainSceneStyleJson {
        elevation_grid,
        project_origin_lon: descriptor.project_origin.lon,
        project_origin_lat: descriptor.project_origin.lat,
        exaggeration: descriptor.exaggeration,
        color_ramp: "hypsometric",
        attribution: bundled.then_some(TerrainAttribution {
            label: "Mapzen Terrain Tiles",
            url: "https://github.com/tilezen/joerd/blob/master/docs/attribution.md",
        }),
        overlays: &descriptor.overlays,
    };
    semio_framework_pack_json::to_json_string(&style)
}

/// 🪧️ Presents a missing elevation source through the normal application UI.
pub fn terrain_source_required(view: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    let notice = semio_ui_locale::LocalizedLabel::native(
        "This terrain has no elevation grid. Connect a grid in Workflow or open a terrain with a saved grid.",
        "Dieses Gelände hat kein Höhenraster. Verbinden Sie ein Raster im Workflow oder öffnen Sie ein Gelände mit einem gespeicherten Raster.",
    );
    let label = semio_ui_locale::Label::data(notice.resolve(view.terminology, view.locale));
    semio_plugin::built_text_node(label).map_err(|_| semio_plugin::ui_assembly_error("terrain-source-required"))
}
