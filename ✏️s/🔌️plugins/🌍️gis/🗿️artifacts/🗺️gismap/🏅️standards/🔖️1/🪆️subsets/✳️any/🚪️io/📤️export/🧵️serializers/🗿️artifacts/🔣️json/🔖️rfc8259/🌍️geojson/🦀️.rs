//! 🌍️ gismap → GeoJSON (RFC 7946) — one FeatureCollection: every position a `Point`, every route a
//! `LineString`, every region a `Polygon` (its `ring`/`points` the exterior, its `holes` the interior
//! rings), in that order and each family in document order. The feature id is the map feature's id;
//! `properties` is its payload without the geometry members (`id`, `lon`, `lat`, `alt`, `points`,
//! `ring`, `holes`). A payload without geometry is a feature with `"geometry": null`. The map already
//! stores WGS 84 longitude/latitude, so no reprojection happens; `write_geojson` refuses anything
//! outside the WGS 84 range rather than write a non-RFC-7946 file.
//!
//! 🔖 `IoFidelity::Semantic`: every feature, id, payload member and coordinate survives the sibling
//! import; rings come back without the closing position RFC 7946 adds and wound by the right-hand rule,
//! a region's `points` member comes back as `ring`.
use crate::{GisMapSnapshot, MapFeature};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::geojson::schema::{write_geojson, GeoJsonFeature, GeoJsonGeometry, GeoJsonId, GeoJsonPosition};
use serde_json::Value;

/// 🧬️ The payload members that carry a map feature's geometry and identity, never its properties.
pub const GEOMETRY_MEMBERS: [&str; 7] = ["id", "lon", "lat", "alt", "points", "ring", "holes"];

pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn error(message: impl Into<String>) -> store::TextError {
    store::TextError::new(format!("gismap→geojson: {}", message.into()), dsl::TextSpan::at(1, 1))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chain(value: &Value, feature: &str) -> Result<Vec<GeoJsonPosition>, store::TextError> {
    let Some(items) = value.as_array() else { return Err(error(format!("`{feature}` carries a non-array chain"))) };
    items
        .iter()
        .map(|item| item.as_array().and_then(|numbers| numbers.iter().map(Value::as_f64).collect::<Option<Vec<f64>>>()).filter(|numbers| numbers.len() >= 2).ok_or_else(|| error(format!("`{feature}` carries a vertex that is not [lon, lat, …]"))))
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn feature(map_feature: &MapFeature, geometry: impl Fn(&serde_json::Map<String, Value>) -> Result<Option<GeoJsonGeometry>, store::TextError>) -> Result<GeoJsonFeature, store::TextError> {
    let payload = match Value::from(&map_feature.data) {
        Value::Object(members) => members,
        _ => return Err(error(format!("feature `{}` has a non-object payload", map_feature.id))),
    };
    let properties = payload.iter().filter(|(key, _)| !GEOMETRY_MEMBERS.contains(&key.as_str())).map(|(key, value)| (key.clone(), value.clone())).collect();
    Ok(GeoJsonFeature { id: Some(GeoJsonId::Text(map_feature.id.clone())), geometry: geometry(&payload)?, properties: Some(properties) })
}

/// 🗺️ The map's features in the GeoJSON model, positions → routes → regions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn geojson_features(snapshot: &GisMapSnapshot) -> Result<Vec<GeoJsonFeature>, store::TextError> {
    let mut features = Vec::with_capacity(snapshot.positions.len() + snapshot.routes.len() + snapshot.regions.len());
    for position in &snapshot.positions {
        features.push(feature(position, |payload| {
            let (Some(lon), Some(lat)) = (payload.get("lon").and_then(Value::as_f64), payload.get("lat").and_then(Value::as_f64)) else { return Ok(None) };
            Ok(Some(GeoJsonGeometry::Point(std::iter::once(lon).chain(std::iter::once(lat)).chain(payload.get("alt").and_then(Value::as_f64)).collect())))
        })?);
    }
    for route in &snapshot.routes {
        features.push(feature(route, |payload| payload.get("points").map(|points| chain(points, &route.id).map(GeoJsonGeometry::LineString)).transpose())?);
    }
    for region in &snapshot.regions {
        features.push(feature(region, |payload| {
            let Some(exterior) = payload.get("ring").or_else(|| payload.get("points")) else { return Ok(None) };
            let mut rings = vec![chain(exterior, &region.id)?];
            if let Some(holes) = payload.get("holes").and_then(Value::as_array) {
                for hole in holes {
                    rings.push(chain(hole, &region.id)?);
                }
            }
            Ok(Some(GeoJsonGeometry::Polygon(rings)))
        })?);
    }
    Ok(features)
}

pub fn serialize_bytes(snapshot: &GisMapSnapshot) -> Result<Vec<u8>, store::TextError> {
    let document = write_geojson(&geojson_features(snapshot)?).map_err(|failure| error(failure.to_string()))?;
    serde_json::to_vec_pretty(&document).map_err(|failure| error(failure.to_string()))
}
