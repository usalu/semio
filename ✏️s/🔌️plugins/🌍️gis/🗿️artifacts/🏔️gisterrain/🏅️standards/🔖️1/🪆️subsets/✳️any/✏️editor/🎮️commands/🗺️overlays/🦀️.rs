//! 🗺️ Registered edits over the terrain document's complete intrinsic overlay map.

use crate::mutations::{ChangeImportedFeatures, GisTerrainMutation};
use crate::schema::imported_map::ImportedMap;
use crate::GisTerrainSnapshot;
use semio_framework_surface::tiled_map::projection::MAX_LAT as MAX_TILED_LATITUDE;
use semio_framework_value::{DslValue, ToValue};
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};
use semio_ui_locale::LocalizedLabel;

const MAX_TEXT_BYTES: usize = 256;
const MAX_WGS84_LATITUDE: f64 = 90.0;
pub(crate) const MAX_COORDINATES_JSON_BYTES: usize = 512;
pub type OverlayEdit = Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault>;

/// 🚫️ Overlay refusals under stable `gis.terrain.overlay.<name>` codes, localized through [`fault_notices`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OverlayFault { IdentifierOrLabel, Range, Duplicate, PositionInvalid, PositionMissing, PositionAmbiguous, CoordinatesTooLong, CoordinatesFormat, ShapeCoordinates, ShapeKind, ShapeMode, OtherKind, ShapeAmbiguous, ShapeMissing, ShapeStored, ShapeVertices, Identifier, IdentifierMissing, LabelInvalid, VerticesMissing, LongitudeInvalid, LatitudeInvalid, ElevationInvalid }

impl OverlayFault {
    const ALL: [Self; 23] = [Self::IdentifierOrLabel, Self::Range, Self::Duplicate, Self::PositionInvalid, Self::PositionMissing, Self::PositionAmbiguous, Self::CoordinatesTooLong, Self::CoordinatesFormat, Self::ShapeCoordinates, Self::ShapeKind, Self::ShapeMode, Self::OtherKind, Self::ShapeAmbiguous, Self::ShapeMissing, Self::ShapeStored, Self::ShapeVertices, Self::Identifier, Self::IdentifierMissing, Self::LabelInvalid, Self::VerticesMissing, Self::LongitudeInvalid, Self::LatitudeInvalid, Self::ElevationInvalid];

    fn message(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::IdentifierOrLabel => ("gis.terrain.overlay.identifier-or-label", "Invalid overlay identifier or label", "Ungültige Überlagerungs-ID oder ungültiger Name"),
            Self::Range => ("gis.terrain.overlay.range", "Overlay coordinates are outside the terrain display range", "Die Koordinaten der Überlagerung liegen außerhalb des Anzeigebereichs des Geländes"),
            Self::Duplicate => ("gis.terrain.overlay.duplicate", "An overlay already has that identifier", "Eine Überlagerung hat diese ID bereits"),
            Self::PositionInvalid => ("gis.terrain.overlay.position-invalid", "Invalid position identifier or coordinates", "Ungültige Positions-ID oder ungültige Koordinaten"),
            Self::PositionMissing => ("gis.terrain.overlay.position-missing", "Position was not found", "Die Position wurde nicht gefunden"),
            Self::PositionAmbiguous => ("gis.terrain.overlay.position-ambiguous", "More than one position has that identifier", "Mehrere Positionen haben diese ID"),
            Self::CoordinatesTooLong => ("gis.terrain.overlay.coordinates-too-long", "Coordinate entry exceeds 512 bytes", "Die Koordinateneingabe überschreitet 512 Byte"),
            Self::CoordinatesFormat => ("gis.terrain.overlay.coordinates-format", "Enter a JSON list of [longitude, latitude] pairs", "Eine JSON-Liste aus [Längengrad, Breitengrad]-Paaren eingeben"),
            Self::ShapeCoordinates => ("gis.terrain.overlay.shape-coordinates", "Shape contains no vertices or coordinates outside the terrain display range", "Die Form enthält keine Stützpunkte oder Koordinaten außerhalb des Anzeigebereichs des Geländes"),
            Self::ShapeKind => ("gis.terrain.overlay.shape-kind", "Unknown shape type", "Unbekannte Formart"),
            Self::ShapeMode => ("gis.terrain.overlay.shape-mode", "Unknown shape edit mode", "Unbekannter Formbearbeitungsmodus"),
            Self::OtherKind => ("gis.terrain.overlay.other-kind", "Another kind of overlay has that identifier", "Eine andere Art von Überlagerung hat diese ID"),
            Self::ShapeAmbiguous => ("gis.terrain.overlay.shape-ambiguous", "More than one shape has that identifier", "Mehrere Formen haben diese ID"),
            Self::ShapeMissing => ("gis.terrain.overlay.shape-missing", "Create the shape before appending vertices", "Die Form erstellen, bevor Stützpunkte hinzugefügt werden"),
            Self::ShapeStored => ("gis.terrain.overlay.shape-stored", "The stored shape contains invalid coordinates. Replace its vertices before appending.", "Die gespeicherte Form enthält ungültige Koordinaten. Ihre Stützpunkte ersetzen, bevor weitere hinzugefügt werden."),
            Self::ShapeVertices => ("gis.terrain.overlay.shape-vertices", "A route needs two distinct vertices and a region needs three", "Eine Route braucht zwei verschiedene Stützpunkte und eine Fläche drei"),
            Self::Identifier => ("gis.terrain.overlay.identifier", "Invalid overlay identifier", "Ungültige Überlagerungs-ID"),
            Self::IdentifierMissing => ("gis.terrain.overlay.identifier-missing", "Missing overlay identifier", "Die Überlagerungs-ID fehlt"),
            Self::LabelInvalid => ("gis.terrain.overlay.label-invalid", "Invalid overlay label", "Ungültiger Überlagerungsname"),
            Self::VerticesMissing => ("gis.terrain.overlay.vertices-missing", "Missing WGS84 vertices", "Die WGS84-Stützpunkte fehlen"),
            Self::LongitudeInvalid => ("gis.terrain.overlay.longitude-invalid", "Missing or invalid longitude", "Der Längengrad fehlt oder ist ungültig"),
            Self::LatitudeInvalid => ("gis.terrain.overlay.latitude-invalid", "Missing or invalid latitude", "Der Breitengrad fehlt oder ist ungültig"),
            Self::ElevationInvalid => ("gis.terrain.overlay.elevation-invalid", "Missing or invalid elevation", "Die Höhe fehlt oder ist ungültig"),
        }
    }

    pub(crate) fn fault(self) -> Fault {
        let (code, en, _) = self.message();
        Fault::new(FaultOrigin::App, FaultCode::new(code), en)
    }

    pub(crate) fn number(key: &str) -> Self {
        match key { "longitude" => Self::LongitudeInvalid, "latitude" => Self::LatitudeInvalid, _ => Self::ElevationInvalid }
    }
}

/// 🔔️ Registers the bilingual overlay refusal notices with the Terrain editor.
pub(crate) fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&'static str, LocalizedLabel); 23]> = std::sync::LazyLock::new(|| {
        OverlayFault::ALL.map(|kind| {
            let (code, en, de) = kind.message();
            (code, LocalizedLabel::native(en, de))
        })
    });
    NOTICES.as_slice()
}

fn replacement(map: ImportedMap) -> OverlayEdit {
    Ok(Emit::mutations(vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures {
        new_imported_map: Some(map),
    })]))
}

fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= MAX_TEXT_BYTES && id.trim() == id && !id.chars().any(char::is_control)
}

fn feature_id(record: &DslValue) -> Option<&str> {
    record.get("id").and_then(DslValue::as_str)
}

fn latitude_limit(snapshot: &GisTerrainSnapshot) -> f64 {
    if snapshot.elevation_source.as_ref().is_some_and(|source| source.is_valid()) { MAX_WGS84_LATITUDE } else { MAX_TILED_LATITUDE }
}

fn valid_coordinates(longitude: f64, latitude: f64, limit: f64) -> bool {
    (-180.0..=180.0).contains(&longitude) && (-limit..=limit).contains(&latitude)
}

fn coordinate_args() -> [semio_plugin::ActionArgDef; 2] {
    use semio_framework::ArgSchema;
    use semio_plugin::ActionArgDef;
    [
        ActionArgDef::with_schema("longitude", LocalizedLabel::native("Longitude", "Längengrad"), ArgSchema::number(Some(-180.0), Some(180.0), None, false)).required(),
        ActionArgDef::with_schema("latitude", LocalizedLabel::native("Latitude", "Breitengrad"), ArgSchema::number(Some(-MAX_WGS84_LATITUDE), Some(MAX_WGS84_LATITUDE), None, false)).required(),
    ]
}

fn replace_field(record: &mut DslValue, key: &str, value: DslValue) {
    if let DslValue::Object(fields) = record {
        if let Some((_, field)) = fields.iter_mut().find(|(name, _)| name == key) { *field = value; }
        else { fields.push((key.into(), value)); }
    }
}

pub mod add_overlay_position {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "add-overlay-position")]
    pub struct AddOverlayPosition {
        pub id: String,
        pub longitude: f64,
        pub latitude: f64,
        #[value(default)]
        pub label: Option<String>,
        #[value(default)]
        pub elevation: Option<f64>,
    }

    pub fn handle(payload: &AddOverlayPosition, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> OverlayEdit {
        if !valid_id(&payload.id) || payload.label.as_ref().is_some_and(|label| label.len() > MAX_TEXT_BYTES) {
            return Err(OverlayFault::IdentifierOrLabel.fault());
        }
        if !valid_coordinates(payload.longitude, payload.latitude, latitude_limit(doc.snapshot)) || payload.elevation.is_some_and(|value| !value.is_finite())
        {
            return Err(OverlayFault::Range.fault());
        }
        if doc.snapshot.imported_map.as_ref().is_some_and(|map| {
            map.positions.iter().chain(&map.routes).chain(&map.regions).any(|record| feature_id(record) == Some(payload.id.as_str()))
        }) {
            return Err(OverlayFault::Duplicate.fault());
        }
        let mut fields = vec![
            ("id".into(), payload.id.to_value()),
            ("lon".into(), payload.longitude.to_value()),
            ("lat".into(), payload.latitude.to_value()),
        ];
        if let Some(label) = &payload.label { fields.push(("label".into(), label.to_value())); }
        if let Some(elevation) = payload.elevation { fields.push(("alt".into(), elevation.to_value())); }
        let mut map = doc.snapshot.imported_map.clone().unwrap_or_default();
        map.positions.push(DslValue::Object(fields));
        replacement(map)
    }
}

pub mod move_overlay_position {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "move-overlay-position")]
    pub struct MoveOverlayPosition { pub id: String, pub longitude: f64, pub latitude: f64 }

    pub fn handle(payload: &MoveOverlayPosition, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> OverlayEdit {
        if !valid_id(&payload.id) || !valid_coordinates(payload.longitude, payload.latitude, latitude_limit(doc.snapshot)) {
            return Err(OverlayFault::PositionInvalid.fault());
        }
        let mut map = doc.snapshot.imported_map.clone().ok_or_else(|| OverlayFault::PositionMissing.fault())?;
        let mut matching = map.positions.iter_mut().filter(|record| feature_id(record) == Some(payload.id.as_str()));
        let feature = matching.next().ok_or_else(|| OverlayFault::PositionMissing.fault())?;
        if matching.next().is_some() { return Err(OverlayFault::PositionAmbiguous.fault()); }
        if feature.get("lon").and_then(DslValue::as_f64) == Some(payload.longitude)
            && feature.get("lat").and_then(DslValue::as_f64) == Some(payload.latitude)
        {
            return Ok(Emit::default());
        }
        replace_field(feature, "lon", payload.longitude.to_value());
        replace_field(feature, "lat", payload.latitude.to_value());
        replacement(map)
    }
}

pub mod edit_overlay_shape {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "edit-overlay-shape")]
    pub struct EditOverlayShape {
        pub id: String,
        pub kind: String,
        pub mode: String,
        pub coordinates: String,
        #[value(default)]
        pub label: Option<String>,
    }

    fn coordinates(text: &str, limit: f64) -> Result<Vec<DslValue>, Fault> {
        if text.len() > MAX_COORDINATES_JSON_BYTES { return Err(OverlayFault::CoordinatesTooLong.fault()); }
        let numbers: Vec<[f64; 2]> = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject)
            .map_err(|_| OverlayFault::CoordinatesFormat.fault())?;
        if numbers.is_empty() || !numbers.iter().all(|&[lon, lat]| valid_coordinates(lon, lat, limit)) {
            return Err(OverlayFault::ShapeCoordinates.fault());
        }
        Ok(numbers.iter().map(ToValue::to_value).collect())
    }

    fn coordinate_pair(value: &DslValue) -> Option<[f64; 2]> {
        let pair = value.as_array()?;
        Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?])
    }

    fn closed(vertices: &[DslValue]) -> bool {
        vertices.first().and_then(coordinate_pair).zip(vertices.last().and_then(coordinate_pair))
            .is_some_and(|(first, last)| first == last)
    }

    pub fn handle(payload: &EditOverlayShape, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> OverlayEdit {
        if !valid_id(&payload.id) || payload.label.as_ref().is_some_and(|label| label.len() > MAX_TEXT_BYTES) {
            return Err(OverlayFault::IdentifierOrLabel.fault());
        }
        let polygon = match payload.kind.as_str() {
            "route" => false,
            "region" => true,
            _ => return Err(OverlayFault::ShapeKind.fault()),
        };
        let append = match payload.mode.as_str() {
            "replace" => false,
            "append" => true,
            _ => return Err(OverlayFault::ShapeMode.fault()),
        };
        let limit = latitude_limit(doc.snapshot);
        let mut new_vertices = coordinates(&payload.coordinates, limit)?;
        let mut map = doc.snapshot.imported_map.clone().unwrap_or_default();
        let (collection, other_collection) = if polygon { (&mut map.regions, &map.routes) } else { (&mut map.routes, &map.regions) };
        let id = Some(payload.id.as_str());
        if map.positions.iter().chain(other_collection).any(|record| feature_id(record) == id) {
            return Err(OverlayFault::OtherKind.fault());
        }
        let mut matching = collection.iter_mut().filter(|record| feature_id(record) == id);
        let current = matching.next();
        if matching.next().is_some() { return Err(OverlayFault::ShapeAmbiguous.fault()); }
        let key = if polygon && current.as_ref().is_none_or(|record| record.get("ring").is_some() || record.get("points").is_none()) { "ring" } else { "points" };
        if append {
            let current = current.as_ref().and_then(|record| record.get(key)).and_then(DslValue::as_array)
                .ok_or_else(|| OverlayFault::ShapeMissing.fault())?;
            let mut vertices = current.to_vec();
            if polygon && closed(&vertices) { vertices.pop(); }
            vertices.append(&mut new_vertices);
            new_vertices = vertices;
        }
        if polygon && closed(&new_vertices) { new_vertices.pop(); }
        let minimum = if polygon { 3 } else { 2 };
        let mut distinct = Vec::with_capacity(minimum);
        for vertex in &new_vertices {
            let pair = coordinate_pair(vertex).filter(|&[lon, lat]| valid_coordinates(lon, lat, limit))
                .ok_or_else(|| OverlayFault::ShapeStored.fault())?;
            if distinct.len() < minimum && !distinct.contains(&pair) { distinct.push(pair); }
        }
        if distinct.len() < minimum { return Err(OverlayFault::ShapeVertices.fault()); }
        let vertices = DslValue::Array(new_vertices);
        if let Some(feature) = current {
            replace_field(feature, key, vertices);
            if let Some(label) = &payload.label { replace_field(feature, "label", label.to_value()); }
        } else {
            let mut feature = DslValue::Object(vec![("id".into(), payload.id.to_value()), (key.into(), vertices)]);
            if let Some(label) = &payload.label { replace_field(&mut feature, "label", label.to_value()); }
            collection.push(feature);
        }
        replacement(map)
    }
}

pub mod remove_overlay_feature {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "remove-overlay-feature")]
    pub struct RemoveOverlayFeature { pub id: String }

    pub fn handle(payload: &RemoveOverlayFeature, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> OverlayEdit {
        if !valid_id(&payload.id) { return Err(OverlayFault::Identifier.fault()); }
        let wanted = Some(payload.id.as_str());
        let Some(map) = doc.snapshot.imported_map.as_ref().filter(|map| {
            map.positions.iter().chain(&map.routes).chain(&map.regions).any(|record| feature_id(record) == wanted)
        }) else { return Ok(Emit::default()); };
        let remaining = |records: &[DslValue]| records.iter().filter(|record| feature_id(record) != wanted).cloned().collect();
        replacement(ImportedMap {
            positions: remaining(&map.positions), routes: remaining(&map.routes), regions: remaining(&map.regions),
            properties: map.properties.clone(),
        })
    }
}

pub mod clear_overlays {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "clear-overlays")]
    pub struct ClearOverlays {}

    pub fn handle(_: &ClearOverlays, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> OverlayEdit {
        match &doc.snapshot.imported_map {
            Some(map) if !(map.positions.is_empty() && map.routes.is_empty() && map.regions.is_empty()) =>
                replacement(ImportedMap { properties: map.properties.clone(), ..Default::default() }),
            _ => Ok(Emit::default()),
        }
    }
}

pub(crate) fn id_arg() -> semio_plugin::ActionArgDef {
    semio_plugin::ActionArgDef::text("id", LocalizedLabel::native("Feature ID", "Objekt-ID")).min_length(1).required()
}

pub(crate) fn add_args() -> Vec<semio_plugin::ActionArgDef> {
    use semio_plugin::ActionArgDef;
    let [longitude, latitude] = coordinate_args();
    vec![id_arg(), name_arg(), longitude, latitude, ActionArgDef::number("elevation", LocalizedLabel::native("Elevation (m)", "Höhe (m)"))]
}

fn name_arg() -> semio_plugin::ActionArgDef { semio_plugin::ActionArgDef::text("label", LocalizedLabel::native("Name", "Name")) }

pub(crate) fn move_args() -> Vec<semio_plugin::ActionArgDef> {
    let [longitude, latitude] = coordinate_args();
    vec![id_arg(), longitude, latitude]
}

pub(crate) fn shape_args() -> Vec<semio_plugin::ActionArgDef> {
    use semio_plugin::{ActionArgDef, ActionArgOption};
    use semio_framework::ArgSchema;
    let option = |id, en, de| ActionArgOption::new(id, LocalizedLabel::native(en, de));
    let mut coordinates = ActionArgDef::json_text("coordinates", LocalizedLabel::native("WGS84 Vertices", "WGS84-Stützpunkte")).required().describe(
        LocalizedLabel::native("Enter [[longitude, latitude], …] in up to 512 bytes. Append more vertices in another edit.", "Bis zu 512 Byte als [[Längengrad, Breitengrad], …] eingeben. Weitere Stützpunkte lassen sich anschließend hinzufügen."));
    if let ArgSchema::String { max_len, .. } = &mut coordinates.schema { *max_len = Some(MAX_COORDINATES_JSON_BYTES as u32); }
    vec![
        id_arg(),
        ActionArgDef::select("kind", LocalizedLabel::native("Shape", "Form"), vec![option("route", "Route", "Route"), option("region", "Region", "Fläche")]).default_value(&"route"),
        ActionArgDef::select("mode", LocalizedLabel::native("Edit", "Bearbeitung"), vec![option("replace", "Create or Replace", "Erstellen oder ersetzen"), option("append", "Append Vertices", "Stützpunkte hinzufügen")]).default_value(&"replace"),
        coordinates,
        name_arg(),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
