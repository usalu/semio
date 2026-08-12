//! 🏔️ GIS 3D terrain descriptor — the `.gis.json` shape a `gis/3d` example is authored in, and the
//! `World3dScene.terrain_json` payload built from it for the `gis3d-play` app's terrain window.
//!
//! 🧭️ Relocated out of the generic `framework_surface_terrain` engine (audit finding A5: the
//! framework must not know ✏️s — these DTOs and `build_terrain_scene_json` name gis-specific
//! concepts). The DEM-tile decode/session/mesh engine itself
//! (`framework_surface::terrain::{tiles, projection, TerrainSessionCore}`) stays in the framework:
//! it is also path-mounted directly into `framework/os/infinite`'s `World3dState` (to dodge a
//! surface↔infinite cargo cycle) to drive the generic `World3d` terrain layer, so it is genuinely
//! shared rendering engine code, not gis-specific — only this descriptor/DTO layer belonged here.

use framework_surface::terrain::tiles;
use serde::{Deserialize, Serialize};

//#region TerrainDescriptor
/// 📄️ Terrain fixture DTO — the `.gis.json` shape a `gis/3d` example is authored in, mirroring
/// `framework_surface_tiled_map`'s `MapDescriptorJson`/`PositionData` pattern.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerrainProjectOrigin {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerrainPositionData {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerrainDescriptorJson {
    pub schema: String,
    pub project_origin: TerrainProjectOrigin,
    #[serde(default)]
    pub positions: Vec<TerrainPositionData>,
    #[serde(default = "default_exaggeration")]
    pub exaggeration: f64,
}

fn default_exaggeration() -> f64 {
    1.0
}

pub const GIS_3D_TERRAIN_TILE_URL_TEMPLATE: &str = "/dem/{z}/{x}/{y}.png";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TerrainSceneStyleJson<'a> {
    tile_url_template: &'a str,
    project_origin_lon: f64,
    project_origin_lat: f64,
    exaggeration: f64,
    color_ramp: &'a str,
    min_zoom: u32,
    max_zoom: u32,
}

/// 🏔️ Builds the `World3dScene.terrain_json` payload for a descriptor — the one place gis needs to
/// reach into `framework_surface::terrain` beyond the wasm session itself (for the generic engine's
/// tile zoom bounds).
pub fn build_terrain_scene_json(descriptor: &TerrainDescriptorJson) -> String {
    let style = TerrainSceneStyleJson {
        tile_url_template: GIS_3D_TERRAIN_TILE_URL_TEMPLATE,
        project_origin_lon: descriptor.project_origin.lon,
        project_origin_lat: descriptor.project_origin.lat,
        exaggeration: descriptor.exaggeration,
        color_ramp: "hypsometric",
        min_zoom: tiles::TERRAIN_TILE_MIN_ZOOM,
        max_zoom: tiles::TERRAIN_TILE_MAX_ZOOM,
    };
    serde_json::to_string(&style).unwrap_or_default()
}
//#endregion TerrainDescriptor

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_terrain_scene_json_roundtrips_descriptor_fields() {
        let descriptor = TerrainDescriptorJson {
            schema: "gis.terrain".to_string(),
            project_origin: TerrainProjectOrigin { lon: 9.7382, lat: 52.3759 },
            positions: vec![TerrainPositionData { id: "p1".to_string(), lon: 9.74, lat: 52.38, label: Some("Site".to_string()), icon: None }],
            exaggeration: 1.5,
        };
        let json = build_terrain_scene_json(&descriptor);
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(value["projectOriginLon"], 9.7382);
        assert_eq!(value["exaggeration"], 1.5);
        assert_eq!(value["tileUrlTemplate"], GIS_3D_TERRAIN_TILE_URL_TEMPLATE);
    }

    #[test]
    fn terrain_descriptor_json_defaults_exaggeration_and_positions_when_absent() {
        let json = r#"{"schema":"gis.terrain","projectOrigin":{"lon":1.0,"lat":2.0}}"#;
        let descriptor: TerrainDescriptorJson = serde_json::from_str(json).expect("valid descriptor json");
        assert_eq!(descriptor.exaggeration, 1.0);
        assert!(descriptor.positions.is_empty());
    }

    #[test]
    fn terrain_position_data_omits_none_fields_when_serialized() {
        let position = TerrainPositionData { id: "p2".to_string(), lon: 1.0, lat: 2.0, label: None, icon: Some("pin".to_string()) };
        let json = serde_json::to_string(&position).expect("serializes");
        assert!(!json.contains("label"));
        assert!(json.contains("\"icon\":\"pin\""));
    }
}
//#endregion 🧪️Tests
