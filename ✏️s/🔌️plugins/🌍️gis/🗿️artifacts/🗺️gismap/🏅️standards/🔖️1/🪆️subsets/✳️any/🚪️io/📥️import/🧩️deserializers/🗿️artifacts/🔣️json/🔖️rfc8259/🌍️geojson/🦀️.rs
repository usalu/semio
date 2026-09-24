//! 🌍️ gismap ← GeoJSON (RFC 7946, and GJ2008 files under stdio's CRS policy: CRS84/EPSG:4326 read as
//! lon/lat, spherical Web Mercator inverse projected, every other CRS refused) — `Point`s become
//! positions (`lon`, `lat`, and `alt` from a third coordinate), `LineString`s routes (`points`),
//! `Polygon`s regions (`ring` = the exterior without its closing position, `holes` = the interior rings
//! likewise). A multi-part geometry or GeometryCollection becomes one feature per part, id `<id>#<n>`.
//! Feature ids are kept (a number becomes its decimal text); a feature without one is `feature-<n>` by
//! its collection index; a repeated id within one family gains `#<n>`. `properties` become payload
//! members, and the geometry members win over properties of the same name.
//!
//! 🔖 `IoFidelity::Lossy`: features with `null` geometry carry no map feature and are dropped, as are
//! foreign members, `bbox`, the type of numeric ids, multi-part grouping, and property members named
//! like a geometry member.
use crate::io::export::serializers::artifacts::json::v_rfc8259::geojson::GEOMETRY_MEMBERS;
use crate::standards::v1::subsets::any::schema::value_to_dsl;
use crate::{gis_map_snapshot_with_derived_children, GisMapSnapshot, MapFeature};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::geojson::schema::{read_geojson_text, GeoJsonFeature, GeoJsonGeometry, GeoJsonId, GeoJsonPosition};
use serde_json::{json, Map, Value};
use std::collections::HashSet;

pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn error(message: impl Into<String>) -> store::TextError {
    store::TextError::new(format!("gismap←geojson: {}", message.into()), dsl::TextSpan::at(1, 1))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn open_ring(ring: &[GeoJsonPosition]) -> Value {
    let open = if ring.len() > 1 && ring.first() == ring.last() { &ring[..ring.len() - 1] } else { ring };
    json!(open)
}

/// 🧩️ The families a map feature lands in, and the id bookkeeping that keeps each family's ids unique.
#[derive(Default)]
struct Families {
    document: GisMapSnapshot,
    taken: [HashSet<String>; 3],
}

impl Families {
    fn push(&mut self, family: usize, id: String, geometry: Map<String, Value>, properties: &Map<String, Value>) {
        let mut unique = id.clone();
        let mut suffix = 1;
        while !self.taken[family].insert(unique.clone()) {
            unique = format!("{id}#{suffix}");
            suffix += 1;
        }
        let mut payload = properties.clone();
        payload.extend(geometry);
        payload.insert("id".into(), Value::String(unique.clone()));
        let feature = MapFeature { id: unique, data: value_to_dsl(&Value::Object(payload)) };
        match family {
            0 => self.document.positions.push(feature),
            1 => self.document.routes.push(feature),
            _ => self.document.regions.push(feature),
        }
    }

    fn geometry(&mut self, geometry: &GeoJsonGeometry, id: &str, properties: &Map<String, Value>) {
        let parts = |count: usize| -> Vec<String> { if count == 1 { vec![id.to_string()] } else { (0..count).map(|index| format!("{id}#{index}")).collect() } };
        match geometry {
            GeoJsonGeometry::Point(position) => self.push(0, id.into(), point(position), properties),
            GeoJsonGeometry::MultiPoint(positions) => positions.iter().zip(parts(positions.len())).for_each(|(position, part)| self.push(0, part, point(position), properties)),
            GeoJsonGeometry::LineString(line) => self.push(1, id.into(), line_string(line), properties),
            GeoJsonGeometry::MultiLineString(lines) => lines.iter().zip(parts(lines.len())).for_each(|(line, part)| self.push(1, part, line_string(line), properties)),
            GeoJsonGeometry::Polygon(rings) => self.push(2, id.into(), polygon(rings), properties),
            GeoJsonGeometry::MultiPolygon(polygons) => polygons.iter().zip(parts(polygons.len())).for_each(|(rings, part)| self.push(2, part, polygon(rings), properties)),
            GeoJsonGeometry::GeometryCollection(members) => members.iter().zip(parts(members.len())).for_each(|(member, part)| self.geometry(member, &part, properties)),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point(position: &GeoJsonPosition) -> Map<String, Value> {
    let mut members = Map::new();
    members.insert("lon".into(), json!(position[0]));
    members.insert("lat".into(), json!(position[1]));
    if let Some(alt) = position.get(2) {
        members.insert("alt".into(), json!(alt));
    }
    members
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn line_string(line: &[GeoJsonPosition]) -> Map<String, Value> {
    Map::from_iter([("points".to_string(), json!(line))])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn polygon(rings: &[Vec<GeoJsonPosition>]) -> Map<String, Value> {
    let mut members = Map::from_iter([("ring".to_string(), open_ring(&rings[0]))]);
    if rings.len() > 1 {
        members.insert("holes".into(), Value::Array(rings[1..].iter().map(|ring| open_ring(ring)).collect()));
    }
    members
}

/// 🗺️ Map features from GeoJSON features read under the stdio CRS policy.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gis_map_snapshot_from_geojson(features: &[GeoJsonFeature]) -> GisMapSnapshot {
    let mut families = Families::default();
    for (index, feature) in features.iter().enumerate() {
        let Some(geometry) = &feature.geometry else { continue };
        let id = match &feature.id {
            Some(GeoJsonId::Text(text)) => text.clone(),
            Some(GeoJsonId::Number(number)) => number.to_string(),
            None => format!("feature-{index}"),
        };
        let properties = feature.properties.clone().unwrap_or_default().into_iter().filter(|(key, _)| !GEOMETRY_MEMBERS.contains(&key.as_str())).collect();
        families.geometry(geometry, &id, &properties);
    }
    gis_map_snapshot_with_derived_children(families.document)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|failure| error(failure.to_string()))?;
    let read = read_geojson_text(text).map_err(|failure| error(failure.to_string()))?;
    Ok(gis_map_snapshot_from_geojson(&read.features))
}
