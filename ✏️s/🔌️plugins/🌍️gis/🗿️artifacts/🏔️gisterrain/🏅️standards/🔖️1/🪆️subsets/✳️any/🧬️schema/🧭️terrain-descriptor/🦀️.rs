//! 🧭️ Terrain descriptor DTOs of the gisterrain artifact: the `.gis.json` shape a `gis/3d` example is authored in and the `World3dScene.terrain_json` payload built from it. Kept below the schema root so the snapshot and the inferences can name it without reaching back into the root.

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️TerrainDescriptor
/// 🧭️ Relocated from `⚙️engine/terrain/🦀️.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): pure DTOs + a pure formatter, neither
/// snapshot-derived nor stateful — the `.gis.json` shape a `gis/3d` example is authored in, and the
/// `World3dScene.terrain_json` payload built from it for the `gis3d-play` app's terrain window.
///
/// 🧭️ Originally relocated out of the generic `framework_surface_terrain` engine (audit finding A5:
/// the framework must not know ✏️s — these DTOs and `build_terrain_scene_json` name gis-specific
/// concepts). The DEM-tile decode/session/mesh engine itself
/// (`semio_framework_surface::terrain::{tiles, projection, TerrainSessionCore}`) stays in the framework:
/// it is also path-mounted directly into `framework/os/infinite`'s `World3dState` (to dodge a
/// surface↔infinite cargo cycle) to drive the generic `World3d` terrain layer, so it is genuinely
/// shared rendering engine code, not gis-specific — only this descriptor/DTO layer belonged here.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainProjectOrigin {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainPositionData {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainDescriptorJson {
    pub schema: String,
    pub project_origin: TerrainProjectOrigin,
    #[value(default)]
    pub positions: Vec<TerrainPositionData>,
    #[value(default = "default_exaggeration")]
    pub exaggeration: f64,
}

fn default_exaggeration() -> f64 {
    1.0
}

pub const GIS_3D_TERRAIN_TILE_URL_TEMPLATE: &str = "/dem/{z}/{x}/{y}.png";




//#endregion 🔖️TerrainDescriptor
