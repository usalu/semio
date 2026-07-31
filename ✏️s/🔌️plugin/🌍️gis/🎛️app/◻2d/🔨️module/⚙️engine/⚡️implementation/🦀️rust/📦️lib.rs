//! ⚙️ GIS 2D app — headless compute (constitutional: engine).

use gis2d::{GisMapDocument, MapFeature};
use gis2d_dsl::REUSE_MAP_EXAMPLE_TEXT;
use semio_framework_plugin::{DwgDrawing, DwgGeometry};
use serde_json::{json, Value};

//#region 🔖️DocumentHelpers
pub fn empty_gis_map_projection() -> GisMapDocument {
    GisMapDocument::default()
}

/// 📥️ Parses a `{ positions, routes, regions }` map-descriptor JSON into a `GisMapDocument` — each
/// array entry becomes a `MapFeature` keyed by its `id`, keeping the full object as the payload.
pub fn gis_map_document_from_descriptor_json(json: &str) -> GisMapDocument {
    let value: Value = serde_json::from_str(json).unwrap_or_else(|_| serde_json::json!({}));
    let features = |key: &str| -> Vec<MapFeature> {
        value
            .get(key)
            .and_then(|entry| entry.as_array())
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|item| {
                        let id = item.get("id").and_then(|value| value.as_str())?.to_string();
                        Some(MapFeature { id, data: item.clone() })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    GisMapDocument { positions: features("positions"), routes: features("routes"), regions: features("regions") }
}

/// 📤️ Rebuilds the `{ positions, routes, regions }` map-descriptor JSON the `MapHost`/renderer consume,
/// emitting each feature's opaque payload.
pub fn gis_map_descriptor_json(document: &GisMapDocument) -> String {
    let payloads = |features: &[MapFeature]| -> Vec<Value> { features.iter().map(|feature| feature.data.clone()).collect() };
    serde_json::json!({
        "positions": payloads(&document.positions),
        "routes": payloads(&document.routes),
        "regions": payloads(&document.regions),
    })
    .to_string()
}

/// 🗺️ The default map document, seeded from the bundled reuse example (see `gis2d::GisMapDocument`'s
/// derive-generated `.gismap` DSL).
pub fn default_document() -> GisMapDocument {
    <GisMapDocument as store::DocumentDsl>::parse_dsl(REUSE_MAP_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_map_projection())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaExport
pub fn gis2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::map_points_svg(value, "GIS 2D")
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 🗺️ Imports a DWG drawing into a bare gis map document: entity vertices become position features.
/// Falls back to the default reuse-map document when the DWG carries no point-like geometry.
pub fn gis2d_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let points: Vec<[f64; 2]> = drawing
        .entities
        .iter()
        .flat_map(|entity| match &entity.geometry {
            DwgGeometry::Point { at } => vec![[at[0], at[1]]],
            DwgGeometry::Line { start, end } => vec![[start[0], start[1]], [end[0], end[1]]],
            DwgGeometry::LwPolyline { vertices, .. } => vertices.clone(),
            DwgGeometry::Polyline3d { vertices, .. } => vertices.iter().map(|v| [v[0], v[1]]).collect(),
            _ => Vec::new(),
        })
        .collect();
    if points.is_empty() {
        return serde_json::to_value(default_document()).map_err(|error| error.to_string());
    }
    let positions: Vec<MapFeature> = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let id = format!("dwg-{index}");
            MapFeature { id: id.clone(), data: json!({ "id": id, "lon": point[0], "lat": point[1] }) }
        })
        .collect();
    serde_json::to_value(GisMapDocument { positions, routes: Vec::new(), regions: Vec::new() }).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dwg_import_collects_point_and_line_vertices() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
        drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [3.0, 4.0, 0.0] } });
        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn dwg_import_falls_back_to_default_document_when_empty() {
        let drawing = DwgDrawing::default();
        let value = gis2d_document_json_from_dwg(&drawing).expect("import empty dwg");
        let document: GisMapDocument = serde_json::from_value(value).expect("document");
        assert!(!document.positions.is_empty(), "fallback seeds the reuse-map document");
    }
}
//#endregion 🧪️Tests
