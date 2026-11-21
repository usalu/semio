//! 🚪️ Projects stdio GeoJSON features into the editable Terrain overlay model.

use crate::schema::imported_map::ImportedMap;

pub const MAX_DOCUMENT_BYTES: usize = 12 * 1024;

#[derive(Debug, PartialEq, Eq)]
pub enum Error { File, Collection, Detail(String), Geometry, Extent }
use semio_framework_pack_json as json;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::{
    base::schema::snapshot::JsonValue,
    geojson::{
        io::text::snapshot::write_geojson,
        schema::{GeoJsonFeature, GeoJsonGeometry, GeoJsonId, GeoJsonPosition, read_geojson, right_handed_ring},
    },
};
use std::collections::{BTreeMap, BTreeSet};

const SOURCE: &str = "geoJsonSource";
const GEOMETRY_FIELDS: &[&str] = &["id", "lon", "lat", "alt", "points", "ring", "holes"];
pub const HEADER: &str = "{\"type\":\"FeatureCollection\",\"features\":[";
pub const FOOTER: &str = "]}";

#[derive(Default, semio_framework_value::RetireOwned)]
pub struct FeatureCursor {
    offsets: [usize; 3],
    wrote_feature: bool,
}

/// 📥️ Reads the complete FeatureCollection through the stdio GeoJSON profile.
pub fn decode(text: &str, maximum_bytes: usize) -> Result<ImportedMap, Error> {
    if text.is_empty() || text.len() > maximum_bytes {
        return Err(Error::File);
    }
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let value = json::parse(text, json::JsonMemberPolicy::Reject).map_err(|error| Error::Detail(error.to_string()))?;
    if value.get("type").and_then(json::Value::as_str) != Some("FeatureCollection") {
        return Err(Error::Collection);
    }
    if value.get("crs").is_some() {
        return Err(Error::Detail("Use WGS84 coordinates with the RFC 7946 coordinate reference system.".into()));
    }
    let read = read_geojson(&JsonValue::from(&value)).map_err(|error| Error::Detail(error.to_string()))?;
    let mut native = ImportedMap::default();
    let mut taken = BTreeSet::new();
    for (order, feature) in read.features.into_iter().enumerate() {
        let mut fields = vec![("id".into(), native_id(feature.id.as_ref(), order, &mut taken).to_value())];
        let properties = feature.properties.as_ref().map_or(DslValue::Null, |members| {
            DslValue::object(members.iter().map(|(key, value)| (key.clone(), intrinsic(value))))
        });
        if let Some(label) = display_label(&properties) {
            fields.push(("label".into(), label.to_value()));
        }
        let mut source = vec![("order".into(), (order as u32).to_value()), ("properties".into(), properties)];
        if let Some(id) = feature.id {
            let original = match id {
                GeoJsonId::Text(text) => DslValue::String(text),
                GeoJsonId::Number(number) => intrinsic(&number),
            };
            source.push(("id".into(), original));
        }
        fields.push((SOURCE.into(), DslValue::Object(source)));
        let target = match feature.geometry {
            Some(GeoJsonGeometry::Point(coordinate)) => {
                compatible_position(&coordinate)?;
                fields.extend([("lon".into(), coordinate[0].to_value()), ("lat".into(), coordinate[1].to_value())]);
                if let Some(altitude) = coordinate.get(2) {
                    fields.push(("alt".into(), altitude.to_value()));
                }
                &mut native.positions
            }
            Some(GeoJsonGeometry::LineString(line)) => {
                line.iter().try_for_each(|coordinate| compatible_position(coordinate))?;
                fields.push(("points".into(), line.to_value()));
                &mut native.routes
            }
            Some(GeoJsonGeometry::Polygon(rings)) => {
                let mut rings = rings.into_iter().enumerate().map(|(index, ring)| open_ring(&ring, index == 0));
                let exterior = rings.next().ok_or(Error::Geometry)??;
                fields.push(("ring".into(), exterior.to_value()));
                let holes = rings.collect::<Result<Vec<_>, _>>()?;
                if !holes.is_empty() {
                    fields.push(("holes".into(), holes.to_value()));
                }
                &mut native.regions
            }
            _ => return Err(Error::Geometry),
        };
        target.push(DslValue::Object(fields));
    }
    if json::to_json_string(&native).len() > MAX_DOCUMENT_BYTES {
        return Err(Error::Extent);
    }
    Ok(native)
}

fn intrinsic(value: &JsonValue) -> DslValue {
    json::to_dsl_value(&json::Value::from(value))
}

fn native_id(id: Option<&GeoJsonId>, order: usize, taken: &mut BTreeSet<String>) -> String {
    let named = id.map(|id| match id {
        GeoJsonId::Text(id) => id.clone(),
        GeoJsonId::Number(number) => json::to_string(&json::Value::from(number)),
    });
    if let Some(id) = named.filter(|id| !id.trim().is_empty() && id.len() <= 128 && taken.insert(id.clone())) {
        return id;
    }
    let fallback = format!("feature-{order}");
    let mut id = fallback.clone();
    let mut suffix = 1;
    while !taken.insert(id.clone()) {
        id = format!("{fallback}-{suffix}");
        suffix += 1;
    }
    id
}

fn display_label(value: &DslValue) -> Option<&str> {
    ["label", "name"].into_iter().find_map(|key| value.get(key)?.as_str().filter(|text| !text.is_empty()))
}

fn compatible_position(coordinate: &[f64]) -> Result<(), Error> {
    (2..=3).contains(&coordinate.len()).then_some(()).ok_or(Error::Geometry)
}

fn open_ring(ring: &[GeoJsonPosition], exterior: bool) -> Result<Vec<GeoJsonPosition>, Error> {
    ring.iter().try_for_each(|coordinate| compatible_position(coordinate))?;
    let mut open = right_handed_ring(ring, exterior);
    open.pop();
    Ok(open)
}

/// 📤️ Exports the current native geometry and preserves the supported source feature metadata.
pub fn encode(map: Option<&ImportedMap>) -> Result<String, Error> {
    let mut output = String::from(HEADER);
    let mut cursor = FeatureCursor::default();
    while let Some(feature) = cursor.next(map)? {
        if output.len() + feature.len() + FOOTER.len() > MAX_DOCUMENT_BYTES {
            return Err(Error::Extent);
        }
        output.push_str(&feature);
    }
    output.push_str(FOOTER);
    Ok(output)
}

impl FeatureCursor {
    pub fn next(&mut self, map: Option<&ImportedMap>) -> Result<Option<String>, Error> {
        let Some(map) = map else { return Ok(None) };
        let collections = [&map.positions, &map.routes, &map.regions];
        let Some((family, record)) = collections.into_iter().zip(self.offsets).enumerate()
            .filter_map(|(family, (records, offset))| Some((family, records.get(offset)?)))
            .min_by_key(|(_, record)| source_order(record))
        else { return Ok(None) };
        bound_record(record)?;
        let feature = feature(record, geometry(family, record)?)?;
        let value = write_geojson(&[feature]).map_err(|error| Error::Detail(error.to_string()))?;
        let mut text = if self.wrote_feature { String::from(",") } else { String::new() };
        let object = value.get("features").and_then(|features| features.get(0)).ok_or(Error::Collection)?;
        use std::fmt::Write;
        write!(&mut text, "{object}").expect("GeoJSON feature string");
        if text.len() > 48 * 1024 {
            return Err(Error::Extent);
        }
        self.offsets[family] += 1;
        self.wrote_feature = true;
        Ok(Some(text))
    }
}

fn source_order(record: &DslValue) -> u64 {
    record.get(SOURCE).and_then(|source| source.get("order")).and_then(DslValue::as_u64).unwrap_or(u64::MAX)
}

fn geometry(family: usize, record: &DslValue) -> Result<GeoJsonGeometry, Error> {
    match family {
        0 => {
            let mut coordinate = vec![value_number(record.get("lon"))?, value_number(record.get("lat"))?];
            if let Some(altitude) = record.get("alt") {
                coordinate.push(value_number(Some(altitude))?);
            }
            Ok(GeoJsonGeometry::Point(coordinate))
        }
        1 => Ok(GeoJsonGeometry::LineString(chain(record.get("points"))?)),
        _ => {
            let mut rings = vec![chain(record.get("ring").or_else(|| record.get("points")))?];
            if let Some(holes) = record.get("holes") {
                for ring in holes.as_array().ok_or(Error::Geometry)? {
                    rings.push(chain(Some(ring))?);
                }
            }
            Ok(GeoJsonGeometry::Polygon(rings))
        }
    }
}

fn bound_record(record: &DslValue) -> Result<(), Error> {
    const MAX_NODES: usize = 4000;
    let mut pending = vec![record];
    let mut nodes = 0;
    let mut bytes = 0;
    while let Some(value) = pending.pop() {
        nodes += 1;
        match value {
            DslValue::Object(fields) if nodes + pending.len() + fields.len() <= MAX_NODES => {
                for (key, child) in fields {
                    bytes += key.len();
                    pending.push(child);
                }
            }
            DslValue::Array(items) if nodes + pending.len() + items.len() <= MAX_NODES => pending.extend(items),
            DslValue::Array(_) | DslValue::Object(_) => return Err(Error::Extent),
            DslValue::String(text) => bytes += text.len(),
            _ => (),
        }
        if bytes > 8 * 1024 {
            return Err(Error::Extent);
        }
    }
    Ok(())
}

fn chain(value: Option<&DslValue>) -> Result<Vec<GeoJsonPosition>, Error> {
    value.and_then(DslValue::as_array).ok_or(Error::Geometry)?.iter().map(|value| {
        let coordinates = value.as_array().ok_or(Error::Geometry)?;
        let point = coordinates.iter().map(|value| value_number(Some(value))).collect::<Result<Vec<_>, _>>()?;
        compatible_position(&point)?;
        Ok(point)
    }).collect()
}

fn value_number(value: Option<&DslValue>) -> Result<f64, Error> {
    value.and_then(DslValue::as_f64).filter(|value| value.is_finite()).ok_or(Error::Geometry)
}

fn feature(record: &DslValue, geometry: GeoJsonGeometry) -> Result<GeoJsonFeature, Error> {
    let metadata = record.get(SOURCE).and_then(|source| {
        Some((source, source.get("order")?.as_u64()?, source.get("properties")?))
    });
    let (id, properties) = if let Some((source, _, original_properties)) = metadata {
        let id = source.get("id").map(geojson_id).transpose()?;
        let mut properties = match original_properties {
            DslValue::Null => None,
            DslValue::Object(fields) => Some(properties(fields, false)?),
            _ => return Err(Error::Collection),
        };
        if let Some(label) = record.get("label").and_then(DslValue::as_str).filter(|label| Some(*label) != display_label(original_properties)) {
            properties.get_or_insert_with(BTreeMap::new).insert("label".into(), JsonValue::String { value: label.into() });
        }
        (id, properties)
    } else {
        let fields = record.as_object().ok_or(Error::Collection)?;
        (record.get("id").map(geojson_id).transpose()?, Some(properties(fields, true)?))
    };
    Ok(GeoJsonFeature { id, properties, geometry: Some(geometry) })
}

fn geojson_id(id: &DslValue) -> Result<GeoJsonId, Error> {
    match id {
        DslValue::String(text) => Ok(GeoJsonId::Text(text.clone())),
        DslValue::Number(_) => Ok(GeoJsonId::Number(json_property(id)?)),
        _ => Err(Error::Collection),
    }
}

fn properties(fields: &[(String, DslValue)], skip_geometry: bool) -> Result<BTreeMap<String, JsonValue>, Error> {
    fields.iter().filter(|(name, _)| !skip_geometry || !GEOMETRY_FIELDS.contains(&name.as_str()))
        .map(|(name, value)| Ok((name.clone(), json_property(value)?))).collect()
}

fn json_property(value: &DslValue) -> Result<JsonValue, Error> {
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            DslValue::Bytes(_) => return Err(Error::Detail("JSON properties cannot represent intrinsic byte values.".into())),
            DslValue::Number(semio_framework_value::Number::Float(number)) if !number.is_finite() => return Err(Error::Detail("JSON properties cannot represent non-finite numbers.".into())),
            DslValue::Array(values) => pending.extend(values),
            DslValue::Object(fields) => pending.extend(fields.iter().map(|(_, value)| value)),
            _ => (),
        }
    }
    Ok(JsonValue::from(json::from_dsl_value(value)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_test_fixtures::fixture_text;
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::geojson::io::text::snapshot::read_geojson_text;

    static FIXTURE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../✏️editor/🎮️commands/🌍️geojson/🧪️tests/🧫️fixtures/🔣️.geojson"));

    fn decode_fixture(text: &str) -> Result<ImportedMap, Error> { decode(text, 24 * 1024) }

    #[test]
    fn geojson_round_trips_position_route_hole_identity_and_properties() {
        let map = decode_fixture(*FIXTURE).expect("GeoJSON projected into Terrain");
        assert_eq!((map.positions.len(), map.routes.len(), map.regions.len()), (1, 1, 1));
        assert_eq!(map.positions[0].get("id").and_then(DslValue::as_str), Some("9007199254740993"));
        assert_eq!(map.positions[0].get("alt").and_then(DslValue::as_f64), Some(87.25));
        assert_eq!(map.routes[0].get("id").and_then(DslValue::as_str), Some("feature-2"));
        assert_eq!(map.regions[0].get("holes").and_then(DslValue::as_array).map(<[_]>::len), Some(1));
        let output = encode(Some(&map)).expect("standard FeatureCollection");
        assert_eq!(read_geojson_text(&output).unwrap().features, read_geojson_text(*FIXTURE).unwrap().features);
        assert!(map.same(&decode_fixture(&output).unwrap()));
    }

    #[test]
    fn geojson_refuses_unrepresentable_or_invalid_features_as_one_collection() {
        for invalid in [
            FIXTURE.replace("LineString", "MultiLineString").replace("[[5.56, 50.62, 87.25], [5.57, 50.63, 92]]", "[[[5.56, 50.62, 87.25], [5.57, 50.63, 92]]]"),
            FIXTURE.replace("5.56, 50.62, 87.25", "195.56, 50.62, 87.25"),
            FIXTURE.replace("\"id\": \"external code\"", "\"id\": \"external code\", \"id\": 1"),
        ] {
            assert!(decode_fixture(&invalid).is_err());
        }
        let duplicate = FIXTURE.replace("9007199254740993", "\"yard\"");
        let map = decode_fixture(&duplicate).unwrap();
        assert_ne!(map.positions[0].get("id"), map.regions[0].get("id"));
        let output = encode(Some(&map)).unwrap();
        assert_eq!(read_geojson_text(&output).unwrap().features, read_geojson_text(&duplicate).unwrap().features);
    }
}
