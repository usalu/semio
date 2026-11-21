//! 🗺️ Projects GIS map features onto the geographic terrain overlay schema.

use super::{TerrainOverlays, TerrainPositionData, TerrainProjectOrigin, TerrainRegionData, TerrainRouteData};
use semio_framework_value::DslValue;

const WEB_MERCATOR_MAX_LATITUDE: f64 = 85.051_128_78;

fn coordinate(lon: &DslValue, lat: &DslValue, maximum_latitude: f64) -> Option<[f64; 2]> {
    let lon = lon.as_f64()?;
    let lat = lat.as_f64()?;
    ((-180.0..=180.0).contains(&lon) && (-maximum_latitude..=maximum_latitude).contains(&lat)).then_some([lon, lat])
}

/// 🔎️ Reads an intrinsic point in the Web Mercator tile extent.
pub fn map_point(value: &DslValue) -> Option<[f64; 2]> {
    coordinate(value.get("lon")?, value.get("lat")?, WEB_MERCATOR_MAX_LATITUDE)
}

fn feature_identity(feature: &DslValue) -> Option<(String, String)> {
    let id = feature.get("id")?.as_str().filter(|id| !id.is_empty())?;
    let name = ["label", "name"].into_iter()
        .find_map(|key| feature.get(key)?.as_str().filter(|name| !name.is_empty()))
        .unwrap_or(id);
    Some((id.into(), name.into()))
}

fn position(feature: &DslValue, maximum_latitude: f64) -> Option<TerrainPositionData> {
    let (id, name) = feature_identity(feature)?;
    let coordinates = coordinate(feature.get("lon")?, feature.get("lat")?, maximum_latitude)?;
    let elevation = feature.get("alt").and_then(DslValue::as_f64).filter(|value| value.is_finite());
    Some(TerrainPositionData { id, name, coordinates, elevation })
}

fn coordinate_chain(value: &DslValue, maximum_latitude: f64) -> Option<Vec<[f64; 2]>> {
    value.as_array()?.iter().map(|point| {
        let [lon, lat, ..] = point.as_array()? else { return None };
        coordinate(lon, lat, maximum_latitude)
    }).collect()
}

fn route(feature: &DslValue, maximum_latitude: f64) -> Option<TerrainRouteData> {
    let (id, name) = feature_identity(feature)?;
    let coordinates = coordinate_chain(feature.get("points")?, maximum_latitude)?;
    (coordinates.len() >= 2).then_some(TerrainRouteData { id, name, coordinates })
}

fn ring(value: &DslValue, maximum_latitude: f64) -> Option<Vec<[f64; 2]>> {
    let mut coordinates = coordinate_chain(value, maximum_latitude)?;
    if coordinates.len() > 1 && coordinates.first() == coordinates.last() {
        coordinates.pop();
    }
    (coordinates.len() >= 3).then_some(coordinates)
}

fn region(feature: &DslValue, maximum_latitude: f64) -> Option<TerrainRegionData> {
    let (id, name) = feature_identity(feature)?;
    let exterior = ring(feature.get("ring").or_else(|| feature.get("points"))?, maximum_latitude)?;
    let mut rings = vec![exterior];
    if let Some(holes) = feature.get("holes") {
        rings.extend(holes.as_array()?.iter().map(|hole| ring(hole, maximum_latitude)).collect::<Option<Vec<_>>>()?);
    }
    Some(TerrainRegionData { id, name, rings })
}

impl TerrainOverlays {
    /// 🧭️ Reads WGS84 map geometry for an authored terrain grid.
    pub fn from_map_features(positions: &[DslValue], routes: &[DslValue], regions: &[DslValue]) -> Self {
        Self::within_latitude_limit(positions, routes, regions, 90.0)
    }

    /// 🗾️ Reads map geometry representable by the tiled Web Mercator source.
    pub fn from_tiled_map_features(positions: &[DslValue], routes: &[DslValue], regions: &[DslValue]) -> Self {
        Self::within_latitude_limit(positions, routes, regions, WEB_MERCATOR_MAX_LATITUDE)
    }

    fn within_latitude_limit(positions: &[DslValue], routes: &[DslValue], regions: &[DslValue], maximum_latitude: f64) -> Self {
        Self {
            positions: positions.iter().filter_map(|feature| position(feature, maximum_latitude)).collect(),
            routes: routes.iter().filter_map(|feature| route(feature, maximum_latitude)).collect(),
            regions: regions.iter().filter_map(|feature| region(feature, maximum_latitude)).collect(),
        }
    }

    /// 📍️ Computes the center of the geographic extent covered by the rendered features.
    pub fn center(&self) -> Option<TerrainProjectOrigin> {
        let mut coordinates = self.coordinates();
        let [lon, lat] = coordinates.next()?;
        let (low, high) = coordinates.fold(([lon, lat], [lon, lat]), |(low, high), [lon, lat]| {
            ([low[0].min(lon), low[1].min(lat)], [high[0].max(lon), high[1].max(lat)])
        });
        Some(TerrainProjectOrigin { lon: low[0].midpoint(high[0]), lat: low[1].midpoint(high[1]) })
    }
}
