//! 🧭️ GIS terrain descriptor and geographic overlay projections for World3d scenes.

use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🗺️overlays/🦀️.rs"]
pub mod overlays;

#[path = "🎞️scene/🦀️.rs"]
pub mod scene;

//#region 🔖️TerrainDescriptor
/// 🌐️ Local terrain coordinates use this geographic origin.
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
    pub name: String,
    pub coordinates: [f64; 2],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
}

#[derive(Clone, Debug, ToValue, FromValue)]
pub struct TerrainRouteData {
    pub id: String,
    pub name: String,
    pub coordinates: Vec<[f64; 2]>,
}

#[derive(Clone, Debug, ToValue, FromValue)]
pub struct TerrainRegionData {
    pub id: String,
    pub name: String,
    pub rings: Vec<Vec<[f64; 2]>>,
}

#[derive(Clone, Debug, Default, ToValue, FromValue)]
pub struct TerrainOverlays {
    pub positions: Vec<TerrainPositionData>,
    pub routes: Vec<TerrainRouteData>,
    pub regions: Vec<TerrainRegionData>,
}

impl TerrainOverlays {
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty() && self.routes.is_empty() && self.regions.is_empty()
    }

    pub fn coordinates(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.positions.iter().map(|position| position.coordinates)
            .chain(self.routes.iter().flat_map(|route| route.coordinates.iter().copied()))
            .chain(self.regions.iter().flat_map(|region| region.rings.iter().flatten().copied()))
    }
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainDescriptorJson {
    pub schema: String,
    pub project_origin: TerrainProjectOrigin,
    #[value(default)]
    pub overlays: TerrainOverlays,
    #[value(default = "default_exaggeration")]
    pub exaggeration: f64,
}

fn default_exaggeration() -> f64 {
    1.0
}

//#endregion 🔖️TerrainDescriptor
