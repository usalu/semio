//! 🌍️ GIS map policy: the OpenStreetMap / OpenMapTiles level-of-detail bands, tile pyramid and cartography, the layer toggles, the positions/routes/regions overlay layers and the vector style presets this plugin's maps render by, as one framework [`MapPolicyDescriptor`].
//!
//! Moved from the framework tiled map (`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🌍️gis-policy`), which now only interprets a neutral policy. The editor's map host installs [`gis_map_extension`]; the editor and viewer map windows ship [`gis_map_scene_descriptor_json`], so the wgpu engine surface and the browser session render by the same policy.

use crate::standards::v1::subsets::any::io::text::snapshot::gis_map_descriptor_value;
use crate::GisMapSnapshot;
use semio_framework_surface::tiled_map::{
    map_layer_weight_slider_ids_json, map_lod_scale_json, CanvasExtension, DescribedMapPolicy, MapExtension, MapLayerRole, MapLayerTarget, MapLayerToggle, MapLodBand, MapOverlayGeometry, MapOverlayLayer, MapOverlayStyleFill, MapPaintTechnique, MapPaletteSlot, MapPolicy, MapPolicyDescriptor, MapStylePreset,
};
use serde_json::Value;
use std::sync::{Arc, OnceLock};

#[path = "🎨️vector-style/🦀️.rs"]
mod vector_style;

// #region 🔖️GisMapPolicy
// #region 🔖️MapLod
/// 🆔️ Id of the GIS map policy and of its map extension.
pub const GIS_MAP_POLICY_ID: &str = "gis.map/openmaptiles";

/// 🗂️ The GIS bands, coarse to fine: id, name, description and upper camera zoom (none for the finest).
const GIS_MAP_LODS: [(&str, &str, &str, Option<f64>); 8] = [
    ("world", "World", "Entire planet; coarsest OSM tiles.", Some(360.0)),
    ("continent", "Continent", "Multi-country overview.", Some(680.0)),
    ("country", "Country", "National extent.", Some(1_280.0)),
    ("region", "Region", "Regional detail.", Some(2_400.0)),
    ("city", "City", "Metropolitan area.", Some(4_400.0)),
    ("district", "District", "Neighbourhood streets.", Some(6_400.0)),
    ("street", "Street", "Block-level detail.", Some(7_200.0)),
    ("building", "Building", "Maximum map fidelity.", None),
];

/// 🔭️ Raster tile z per band, one entry per [`GIS_MAP_LODS`] band. It once carried a ninth, unreachable `18`, which is what led the oracle fixture to record a nonexistent band index 8 for `spanDeg = 0`.
const GIS_MAP_LOD_TILE_Z: [u32; 8] = [0, 1, 2, 3, 4, 5, 7, 10];

/// 🌐️ Upper visible longitude span (degrees) per band; a coarser band when the span exceeds it.
const GIS_MAP_LOD_MAX_SPAN_DEG: [f64; 8] = [100.0, 35.0, 12.0, 4.0, 1.2, 0.35, 0.1, 0.0];

/// 🎯️ Representative viewport longitude span (degrees) of a pinned band.
const GIS_MAP_LOD_REPRESENTATIVE_SPAN_DEG: [f64; 8] = [180.0, 50.0, 20.0, 8.0, 2.0, 0.6, 0.2, 0.05];

/// 🧱️ OSM raster pyramid ceiling for automatic viewport picking.
const GIS_MAP_RASTER_TILE_Z_MAX: u32 = 19;

/// 🧊️ OpenFreeMap planet vector tiles (OpenMapTiles schema, OSM) — max generated zoom.
const GIS_MAP_VECTOR_TILE_Z_MAX: u32 = 14;

/// 🧷️ World and continent: automatic raster tiles stay at the band's fixed coarse z, vector tiles follow the span cap alone.
const GIS_MAP_COARSE_BAND_COUNT: usize = 2;

/// 🏛️ Lowest administrative level a boundary paints at.
const GIS_MAP_BOUNDARY_ADMIN_MIN: u64 = 2;
// #endregion 🔖️MapLod

// #region 🔖️Layers
/// 🎚️ The GIS layer toggle keys in chrome order; the editor's layer stack (`GIS_MAP_LAYER_IDS`) names and decorates exactly these.
pub const GIS_MAP_LAYER_KEYS: [&str; 11] = ["raster", "water", "land", "roads", "buildings", "borders", "labels", "positions", "positionLabels", "routes", "regions"];

/// 📍️ Overlay layer id of the document's positions.
pub const GIS_MAP_OVERLAY_POSITIONS: &str = "positions";

/// 🛤️ Overlay layer id of the document's routes.
pub const GIS_MAP_OVERLAY_ROUTES: &str = "routes";

/// 🗾️ Overlay layer id of the document's regions.
pub const GIS_MAP_OVERLAY_REGIONS: &str = "regions";

/// 🪜️ Pick and selection granularity of a position.
pub const GIS_MAP_POSITION_GRANULARITY: &str = "position";

/// 🪜️ Pick and selection granularity of a route.
pub const GIS_MAP_ROUTE_GRANULARITY: &str = "route";

/// 🖊️ Default route width (screen px at weight 1.0) and region outline width.
const GIS_MAP_ROUTE_WIDTH: f64 = 2.0;

/// ⭕️ Position marker radius (screen px at weight 1.0).
const GIS_MAP_POSITION_RADIUS: f64 = 8.0;

/// 🔘️ Position marker outline width (screen px at weight 1.0).
const GIS_MAP_POSITION_STROKE: f64 = 2.0;

/// 🏷️ Gap between a position marker and its caption (screen px).
const GIS_MAP_POSITION_LABEL_OFFSET: f64 = 6.0;

/// 🎯️ Position pick radius (screen px).
const GIS_MAP_POSITION_HIT_RADIUS: f64 = 14.0;

/// 🧭️ Every GIS layer toggle with the paint targets it switches and weights.
fn gis_map_layer_toggles() -> Vec<MapLayerToggle> {
    let roles = |roles: &[MapLayerRole]| -> Vec<MapLayerTarget> { roles.iter().map(|role| MapLayerTarget::Role(*role)).collect() };
    let targets: [Vec<MapLayerTarget>; 11] = [
        vec![MapLayerTarget::Raster],
        roles(&[MapLayerRole::WaterArea, MapLayerRole::Waterway, MapLayerRole::Coastline]),
        roles(&[MapLayerRole::LandCover, MapLayerRole::GreenArea, MapLayerRole::AdminArea]),
        roles(&[MapLayerRole::Road]),
        roles(&[MapLayerRole::Building]),
        roles(&[MapLayerRole::Boundary]),
        roles(&[MapLayerRole::RoadLabel, MapLayerRole::PointLabel, MapLayerRole::PlaceLabel]),
        vec![MapLayerTarget::Overlay(GIS_MAP_OVERLAY_POSITIONS.into())],
        vec![MapLayerTarget::OverlayLabels(GIS_MAP_OVERLAY_POSITIONS.into())],
        vec![MapLayerTarget::Overlay(GIS_MAP_OVERLAY_ROUTES.into())],
        vec![MapLayerTarget::Overlay(GIS_MAP_OVERLAY_REGIONS.into())],
    ];
    GIS_MAP_LAYER_KEYS.iter().zip(targets).map(|(key, targets)| MapLayerToggle { key: (*key).into(), targets }).collect()
}

/// 🗺️ The document's feature collections as overlay layers, drawn regions, routes, positions (topmost); positions and routes are pickable, a `donor` position fills with the route colour.
fn gis_map_overlay_layers() -> Vec<MapOverlayLayer> {
    let layer = |id: &str, geometry: MapOverlayGeometry, granularity: Option<&str>, fill: MapPaletteSlot, stroke: MapPaletteSlot| MapOverlayLayer {
        id: id.into(),
        geometry,
        granularity: granularity.map(str::to_string),
        hit_radius_px: GIS_MAP_POSITION_HIT_RADIUS,
        line_width: GIS_MAP_ROUTE_WIDTH,
        point_radius: GIS_MAP_POSITION_RADIUS,
        point_stroke: GIS_MAP_POSITION_STROKE,
        label_offset_px: GIS_MAP_POSITION_LABEL_OFFSET,
        fill,
        stroke,
        style_fills: Vec::new(),
    };
    let regions = layer(GIS_MAP_OVERLAY_REGIONS, MapOverlayGeometry::Area, None, MapPaletteSlot::RegionFill, MapPaletteSlot::RegionStroke);
    let routes = MapOverlayLayer { hit_radius_px: GIS_MAP_ROUTE_WIDTH * 4.0 + 6.0, ..layer(GIS_MAP_OVERLAY_ROUTES, MapOverlayGeometry::Line, Some(GIS_MAP_ROUTE_GRANULARITY), MapPaletteSlot::RouteStroke, MapPaletteSlot::RouteStroke) };
    let positions = MapOverlayLayer {
        style_fills: vec![MapOverlayStyleFill { style: "donor".into(), fill: MapPaletteSlot::RouteStroke }],
        ..layer(GIS_MAP_OVERLAY_POSITIONS, MapOverlayGeometry::Point, Some(GIS_MAP_POSITION_GRANULARITY), MapPaletteSlot::PositionFill, MapPaletteSlot::PositionStroke)
    };
    vec![regions, routes, positions]
}

/// 🌈️ The window's vector styles: `colored` (default), `figureGround` and its inverse.
fn gis_map_style_presets() -> Vec<MapStylePreset> {
    let preset = |id: &str, technique: MapPaintTechnique, ink: MapPaletteSlot, paper: MapPaletteSlot, label_fill: MapPaletteSlot, label_halo: MapPaletteSlot| MapStylePreset { id: id.into(), technique, ink, paper, label_fill, label_halo };
    vec![
        preset("colored", MapPaintTechnique::Colored, MapPaletteSlot::LabelFill, MapPaletteSlot::SurfaceClear, MapPaletteSlot::LabelFill, MapPaletteSlot::LabelHalo),
        preset("figureGround", MapPaintTechnique::FigureGround, MapPaletteSlot::LabelFill, MapPaletteSlot::SurfaceClear, MapPaletteSlot::LabelFill, MapPaletteSlot::SurfaceClear),
        preset("invertedFigure", MapPaintTechnique::FigureGround, MapPaletteSlot::SurfaceClear, MapPaletteSlot::LabelFill, MapPaletteSlot::SurfaceClear, MapPaletteSlot::LabelFill),
    ]
}
// #endregion 🔖️Layers

// #region 🔖️Descriptor
/// 📋️ The complete GIS policy descriptor: bands, tile pyramid and the OpenMapTiles cartography of [`vector_style`].
pub fn gis_map_policy_descriptor() -> MapPolicyDescriptor {
    MapPolicyDescriptor {
        id: GIS_MAP_POLICY_ID.into(),
        bands: (0..GIS_MAP_LODS.len()).map(gis_map_band).collect(),
        coarse_band_count: GIS_MAP_COARSE_BAND_COUNT,
        raster_tile_z_max: GIS_MAP_RASTER_TILE_Z_MAX,
        vector_tile_z_max: GIS_MAP_VECTOR_TILE_Z_MAX,
        vector_zoom_steps: vector_style::gis_map_vector_zoom_steps(),
        boundary_admin_min: GIS_MAP_BOUNDARY_ADMIN_MIN,
        layers: vector_style::gis_map_source_layers(),
        default_draw_rank: vector_style::GIS_MAP_DEFAULT_DRAW_RANK,
        feature_keys: vector_style::gis_map_feature_keys(),
        road_spans: vector_style::gis_map_road_spans(),
        road_widths: vector_style::gis_map_road_widths(),
        boundary_widths: vector_style::gis_map_boundary_widths(),
        coastline_width: vector_style::gis_map_coastline_width(),
        line_scale: vector_style::gis_map_line_scale(),
        labels: vector_style::gis_map_label_rules(),
        layer_toggles: gis_map_layer_toggles(),
        overlays: gis_map_overlay_layers(),
        style_presets: gis_map_style_presets(),
    }
}

fn gis_map_band(index: usize) -> MapLodBand {
    let (id, name, description, max_zoom) = GIS_MAP_LODS[index];
    let (buildings_max_span_deg, buildings_min_tile_z) = vector_style::gis_map_band_buildings(index);
    MapLodBand {
        id: id.into(),
        name: name.into(),
        description: description.into(),
        max_zoom,
        max_span_deg: GIS_MAP_LOD_MAX_SPAN_DEG[index],
        raster_tile_z: GIS_MAP_LOD_TILE_Z[index],
        representative_span_deg: GIS_MAP_LOD_REPRESENTATIVE_SPAN_DEG[index],
        label_px: vector_style::gis_map_band_label_px(index),
        gates: vector_style::gis_map_band_gates(index),
        buildings_max_span_deg,
        buildings_min_tile_z,
        road_spans: vector_style::gis_map_band_road_spans(index),
        road_stroke_scale: vector_style::gis_map_band_road_stroke_scale(index),
        water_area_classes: vector_style::gis_map_band_water_area_classes(index),
        waterways: vector_style::gis_map_band_waterways(index),
        admin_area_holes: vector_style::gis_map_band_admin_area_holes(index),
        land_wash: vector_style::gis_map_band_land_wash(index),
        land_cover_over_admin_areas: vector_style::gis_map_band_land_cover_over_admin_areas(index),
    }
}
// #endregion 🔖️Descriptor

// #region 🔖️Extension
/// 🌍️ The GIS policy, built once.
pub fn gis_map_policy() -> &'static DescribedMapPolicy {
    static POLICY: OnceLock<DescribedMapPolicy> = OnceLock::new();
    POLICY.get_or_init(|| DescribedMapPolicy::new(gis_map_policy_descriptor()))
}

/// 🧩️ Web Mercator map extension rendering by [`gis_map_policy`].
#[derive(Clone, Copy, Debug, Default)]
pub struct GisMapExtension;

impl CanvasExtension for GisMapExtension {
    fn extension_id(&self) -> &str {
        GIS_MAP_POLICY_ID
    }
}

impl MapExtension for GisMapExtension {
    fn map_crs(&self) -> &str {
        "EPSG:3857"
    }

    fn policy(&self) -> &dyn MapPolicy {
        gis_map_policy()
    }
}

/// 🔌️ The extension a GIS `MapHost` is built with.
pub fn gis_map_extension() -> Arc<dyn MapExtension> {
    Arc::new(GisMapExtension)
}
// #endregion 🔖️Extension

// #region 🔖️JsonSurfaces
/// 📊️ JSON rows of the GIS bands for the LOD select and the palette arg schema.
pub fn gis_map_lod_scale_json() -> String {
    map_lod_scale_json(gis_map_policy())
}

/// 🎚️ JSON array of the layer ids with weight sliders at a GIS band and tile render mode.
pub fn gis_map_layer_weight_slider_ids_json(lod_id: &str, render_mode: &str) -> String {
    map_layer_weight_slider_ids_json(gis_map_policy(), lod_id, render_mode)
}

/// 📍️ One position payload (`id`, `lon`, `lat`, `name`/`label`, `kind`, `icon`, `source_url`) as a point overlay feature: the caption is `name` when present, else `label`, trimmed; `kind` becomes the style key.
fn gis_map_position_feature(payload: &Value) -> Option<Value> {
    let text = |key: &str| payload.get(key).and_then(Value::as_str);
    let mut feature = serde_json::json!({ "id": text("id")?, "coordinates": [[payload.get("lon")?.as_f64()?, payload.get("lat")?.as_f64()?]] });
    let optional = [("label", text("name").or(text("label")).map(str::trim).filter(|caption| !caption.is_empty())), ("style", text("kind")), ("icon", text("icon")), ("link", text("source_url").or(text("sourceUrl")))];
    for (key, value) in optional {
        if let Some(value) = value {
            feature[key] = value.into();
        }
    }
    Some(feature)
}

/// 🧮️ A `[[lon, lat], …]` payload array as overlay coordinates, or `None` when any entry is not a number pair.
fn gis_map_coordinates(value: Option<&Value>) -> Option<Value> {
    let rows: Option<Vec<Value>> = value?.as_array()?.iter().map(|row| Some(serde_json::json!([row.get(0)?.as_f64()?, row.get(1)?.as_f64()?]))).collect();
    rows.map(Value::Array)
}

/// 🛤️ One route payload (`id`, `points`, `stroke_width`) as a line overlay feature.
fn gis_map_route_feature(payload: &Value) -> Option<Value> {
    let mut feature = serde_json::json!({ "id": payload.get("id")?.as_str()?, "coordinates": gis_map_coordinates(payload.get("points"))? });
    if let Some(width) = payload.get("stroke_width").and_then(Value::as_f64) {
        feature["width"] = width.into();
    }
    Some(feature)
}

/// 🗾️ One region payload (`id`, `ring`) as an area overlay feature.
fn gis_map_region_feature(payload: &Value) -> Option<Value> {
    Some(serde_json::json!({ "id": payload.get("id")?.as_str()?, "coordinates": gis_map_coordinates(payload.get("ring"))? }))
}

/// 🗺️ The document's positions, routes and regions as the `overlays` member of a map descriptor; a malformed payload is skipped.
pub fn gis_map_overlays_value(document: &GisMapSnapshot) -> Value {
    let descriptor = gis_map_descriptor_value(document);
    let features = |collection: &str, convert: fn(&Value) -> Option<Value>| -> Value { Value::Array(descriptor.get(collection).and_then(Value::as_array).into_iter().flatten().filter_map(convert).collect()) };
    let mut overlays = serde_json::Map::new();
    overlays.insert(GIS_MAP_OVERLAY_POSITIONS.into(), features("positions", gis_map_position_feature));
    overlays.insert(GIS_MAP_OVERLAY_ROUTES.into(), features("routes", gis_map_route_feature));
    overlays.insert(GIS_MAP_OVERLAY_REGIONS.into(), features("regions", gis_map_region_feature));
    Value::Object(overlays)
}

/// 📨️ The scene map descriptor: the document's overlays plus the GIS policy under `policy`, which every tiled-map host installs on sync.
pub fn gis_map_scene_descriptor_json(document: &GisMapSnapshot) -> String {
    static POLICY: OnceLock<Value> = OnceLock::new();
    serde_json::json!({ "overlays": gis_map_overlays_value(document), "policy": POLICY.get_or_init(|| serde_json::to_value(gis_map_policy().descriptor()).unwrap_or_default()) }).to_string()
}
// #endregion 🔖️JsonSurfaces
// #endregion 🔖️GisMapPolicy

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
