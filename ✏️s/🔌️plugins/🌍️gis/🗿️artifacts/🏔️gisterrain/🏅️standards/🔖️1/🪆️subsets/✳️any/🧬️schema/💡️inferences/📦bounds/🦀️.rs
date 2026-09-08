//! 📦 `bounds` — one named inference: geographic bounding box + position count decoded from the
//! `map:in` overlay carried in `imported_features_json`'s `{positions:[{id,lon,lat,label?,icon?}]}`
//! descriptor JSON (mirrors `⚙️engine`'s private `imported_positions` decoder — kept independent
//! here per the schema-layer's own read of the snapshot, rather than reaching into engine
//! internals that aren't `pub`). Simple whole-snapshot scalar: no `InferredField` caching, the
//! overlay is small and re-decoding is O(positions).

use crate::GisTerrainSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📦Bounds
/// 📦 Geographic bounding box across every decoded `(lon, lat)` pair.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct GisTerrainBounds {
    pub lon_min: f64,
    pub lon_max: f64,
    pub lat_min: f64,
    pub lat_max: f64,
}

/// 🗺️ Decodes `imported_features_json`'s `positions` overlay into raw `(lon, lat)` pairs —
/// malformed/empty JSON (including the default empty string) contributes no positions.
pub(crate) fn imported_lon_lat_positions(snapshot: &GisTerrainSnapshot) -> Vec<(f64, f64)> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&snapshot.imported_features_json) else {
        return Vec::new();
    };
    let Some(positions) = value.get("positions").and_then(|value| value.as_array()) else {
        return Vec::new();
    };
    positions.iter().filter_map(|entry| Some((entry.get("lon")?.as_f64()?, entry.get("lat")?.as_f64()?))).collect()
}

/// 📦 Bounding box across every decoded `(lon, lat)` pair, or `None` for an empty overlay.
pub(crate) fn lon_lat_bounds(positions: &[(f64, f64)]) -> Option<GisTerrainBounds> {
    positions.iter().fold(None, |acc, &(lon, lat)| {
        Some(match acc {
            Some(bounds) => GisTerrainBounds { lon_min: bounds.lon_min.min(lon), lon_max: bounds.lon_max.max(lon), lat_min: bounds.lat_min.min(lat), lat_max: bounds.lat_max.max(lat) },
            None => GisTerrainBounds { lon_min: lon, lon_max: lon, lat_min: lat, lat_max: lat },
        })
    })
}
//#endregion 📦Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
