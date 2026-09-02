//! 📦 `bounds` — one named inference: geographic bounding box + position count decoded from the
//! `map:in` overlay carried in `imported_features_json`'s `{positions:[{id,lon,lat,label?,icon?}]}`
//! descriptor JSON (mirrors `⚙️engine`'s private `imported_positions` decoder — kept independent
//! here per the schema-layer's own read of the snapshot, rather than reaching into engine
//! internals that aren't `pub`). Simple whole-snapshot scalar: no `InferredField` caching, the
//! overlay is small and re-decoding is O(positions).

use crate::artifacts::gisterrain::GisTerrainSnapshot;
use serde::{Deserialize, Serialize};

//#region 📦Bounds
/// 📦 Geographic bounding box across every decoded `(lon, lat)` pair.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_overlay_has_no_bounds() {
        let snapshot = GisTerrainSnapshot::default();
        assert!(lon_lat_bounds(&imported_lon_lat_positions(&snapshot)).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn malformed_overlay_json_contributes_no_positions() {
        let snapshot = GisTerrainSnapshot { exaggeration: 0.0, imported_features_json: "not json".into(), ..Default::default() };
        assert!(imported_lon_lat_positions(&snapshot).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn two_positions_produce_their_enclosing_box() {
        let snapshot = GisTerrainSnapshot {
            exaggeration: 1.0,
            imported_features_json: serde_json::json!({ "positions": [
                { "id": "a", "lon": 5.0, "lat": 50.0 },
                { "id": "b", "lon": 6.0, "lat": 51.0 },
            ] })
            .to_string(),
            ..Default::default()
        };
        let bounds = lon_lat_bounds(&imported_lon_lat_positions(&snapshot)).expect("two positions bound");
        assert_eq!(bounds, GisTerrainBounds { lon_min: 5.0, lon_max: 6.0, lat_min: 50.0, lat_max: 51.0 });
    }
}
//#endregion 🧪️Tests
