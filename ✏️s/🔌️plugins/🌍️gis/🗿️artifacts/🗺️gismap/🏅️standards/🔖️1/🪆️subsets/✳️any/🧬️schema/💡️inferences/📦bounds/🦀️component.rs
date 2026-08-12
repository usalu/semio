//! 📦 `bounds` — one named inference: geographic bounding box across every `positions`/`routes`/
//! `regions` feature. `MapFeature::data` is deliberately untyped (`dsl::DslValue`, the engine's
//! `Shape::Value` escape hatch — see `crate::artifacts::gismap`'s own docs), so the box is derived
//! by a generic coordinate-pair scan over each feature's raw value rather than by assuming a fixed
//! shape: any `{lon, lat}` object or `[number, number]` pair anywhere inside `data` counts as one
//! point. This uniformly covers `positions` (`{lon,lat}`), `routes` (`points: [[lon,lat], …]`) and
//! `regions` (`ring: [[lon,lat], …]`) without hard-coding any of those field names. Simple
//! whole-snapshot scalar: no `InferredField` caching, feature counts here are small.

use crate::artifacts::gismap::GisMapSnapshot;
use serde::{Deserialize, Serialize};

//#region 📦Bounds
/// 📦 Geographic bounding box across every scanned `(lon, lat)` pair.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapBounds {
    pub lon_min: f64,
    pub lon_max: f64,
    pub lat_min: f64,
    pub lat_max: f64,
}

/// 🗺️ Recursively collects every `{lon, lat}` object and `[number, number]` pair inside `value`.
pub(crate) fn scan_lon_lat_pairs(value: &dsl::DslValue, out: &mut Vec<(f64, f64)>) {
    match value {
        dsl::DslValue::Object(entries) => {
            let lon = entries.iter().find(|(key, _)| key == "lon").and_then(|(_, value)| value.as_f64());
            let lat = entries.iter().find(|(key, _)| key == "lat").and_then(|(_, value)| value.as_f64());
            if let (Some(lon), Some(lat)) = (lon, lat) {
                out.push((lon, lat));
            }
            for (_, value) in entries {
                scan_lon_lat_pairs(value, out);
            }
        }
        dsl::DslValue::Array(items) => {
            if let [a, b] = items.as_slice() {
                if let (Some(a), Some(b)) = (a.as_f64(), b.as_f64()) {
                    out.push((a, b));
                    return;
                }
            }
            for item in items {
                scan_lon_lat_pairs(item, out);
            }
        }
        _ => {}
    }
}

/// 📦 Bounding box across every scanned `(lon, lat)` pair, or `None` when nothing scanned.
pub(crate) fn lon_lat_bounds(pairs: &[(f64, f64)]) -> Option<GisMapBounds> {
    pairs.iter().fold(None, |acc, &(lon, lat)| {
        Some(match acc {
            Some(bounds) => GisMapBounds {
                lon_min: bounds.lon_min.min(lon),
                lon_max: bounds.lon_max.max(lon),
                lat_min: bounds.lat_min.min(lat),
                lat_max: bounds.lat_max.max(lat),
            },
            None => GisMapBounds { lon_min: lon, lon_max: lon, lat_min: lat, lat_max: lat },
        })
    })
}

/// 🗺️ Scans every feature across `positions`/`routes`/`regions` for coordinate pairs.
pub(crate) fn all_lon_lat_pairs(snapshot: &GisMapSnapshot) -> Vec<(f64, f64)> {
    let mut pairs = Vec::new();
    for feature in snapshot.positions.iter().chain(snapshot.routes.iter()).chain(snapshot.regions.iter()) {
        scan_lon_lat_pairs(&feature.data, &mut pairs);
    }
    pairs
}
//#endregion 📦Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::gismap::MapFeature;

    fn dsl_of(value: serde_json::Value) -> dsl::DslValue {
        dsl::to_dsl_value(&value).unwrap_or(dsl::DslValue::Null)
    }

    #[test]
    fn empty_snapshot_has_no_bounds() {
        assert!(lon_lat_bounds(&all_lon_lat_pairs(&GisMapSnapshot::default())).is_none());
    }

    #[test]
    fn positions_routes_and_regions_all_contribute_points() {
        let snapshot = GisMapSnapshot {
            positions: vec![MapFeature { id: "p1".into(), data: dsl_of(serde_json::json!({ "id": "p1", "lon": -0.1427, "lat": 51.5142 })) }],
            routes: vec![MapFeature { id: "r1".into(), data: dsl_of(serde_json::json!({ "id": "r1", "points": [[1.0, 2.0], [3.0, 4.0]] })) }],
            regions: vec![MapFeature { id: "g1".into(), data: dsl_of(serde_json::json!({ "id": "g1", "ring": [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] })) }],
        };
        let bounds = lon_lat_bounds(&all_lon_lat_pairs(&snapshot)).expect("features bound");
        assert_eq!(bounds, GisMapBounds { lon_min: -0.1427, lon_max: 3.0, lat_min: 0.0, lat_max: 51.5142 });
    }
}
//#endregion 🧪️Tests
