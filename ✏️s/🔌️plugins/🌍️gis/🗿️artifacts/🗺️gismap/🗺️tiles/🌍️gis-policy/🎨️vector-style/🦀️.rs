//! 🎨️ GIS vector-tile cartography over the OpenMapTiles schema: source-layer roles and draw order, road and label class tables, per-band paint gates, stroke widths and line damping.
//!
//! Moved from the framework tiled map (`🗺️tiled-map/🌍️gis-policy/🎨️vector-style`). Each former rule is a row of the [`semio_framework_surface::tiled_map::MapPolicyDescriptor`] the parent assembles; the framework's neutral `MapPolicy` interprets it.

use super::GIS_MAP_LOD_MAX_SPAN_DEG;
use semio_framework_surface::tiled_map::{MapClassRank, MapClassRanks, MapClassSpan, MapClassSpanRange, MapClassSpans, MapClassWidth, MapClassWidths, MapDampingCurve, MapFeatureKeys, MapLabelRules, MapLayerGates, MapLayerRole, MapLevelWidth, MapLevelWidths, MapLineDamping, MapLineScale, MapSourceLayer, MapSpanZoom, MapWidth};

// #region 🔖️VectorStyle
/// 🔢️ Draw rank of an OpenMapTiles source layer missing from [`gis_map_source_layers`].
pub(super) const GIS_MAP_DEFAULT_DRAW_RANK: u8 = 15;

/// 🛣️ Road stroke multiplier of the region band.
const GIS_MAP_ROAD_REGION_SCALE: f64 = 0.4;
/// 🏙️ Road stroke multiplier of the city band.
const GIS_MAP_ROAD_CITY_SCALE: f64 = 0.3;
/// 🔤️ Label screen px at each band's coarse edge.
const GIS_MAP_LABEL_PX_BANDS: [f64; 8] = [26.0, 26.0, 20.0, 14.0, 12.5, 12.0, 10.5, 10.5];
/// 🚗️ Motorway base stroke width.
const GIS_MAP_ROAD_MOTORWAY_WIDTH: f64 = 2.4;
/// 🥾️ Path base stroke width.
const GIS_MAP_ROAD_PATH_WIDTH: f64 = 0.55;
/// ➖️ Narrowest road stroke.
const GIS_MAP_ROAD_WIDTH_MIN: f64 = 0.35;
/// ➕️ Widest road stroke.
const GIS_MAP_ROAD_WIDTH_MAX: f64 = 2.8;
/// 🏳️ National border base stroke width.
const GIS_MAP_BOUNDARY_ADMIN2_WIDTH: f64 = 1.75;
/// 🚩️ Base stroke width of an unlisted administrative level.
const GIS_MAP_BOUNDARY_DEFAULT_WIDTH: f64 = 0.65;
/// 🔽️ Narrowest boundary stroke.
const GIS_MAP_BOUNDARY_WIDTH_MIN: f64 = 0.45;
/// 🔼️ Widest boundary stroke.
const GIS_MAP_BOUNDARY_WIDTH_MAX: f64 = 2.2;
/// 🏖️ Coastline base stroke width.
const GIS_MAP_COASTLINE_WIDTH: f64 = 1.35;
/// 🐚️ Narrowest coastline stroke.
const GIS_MAP_COASTLINE_WIDTH_MIN: f64 = 0.85;
/// 🌅️ Widest coastline stroke.
const GIS_MAP_COASTLINE_WIDTH_MAX: f64 = 2.8;
/// 📐️ Line scale coefficient over the viewport span.
const GIS_MAP_LINE_SCALE_COEFFICIENT: f64 = 280.0;
/// 🧢️ Line scale ceiling.
const GIS_MAP_LINE_SCALE_CAP: f64 = 1.38;
/// 📉️ Line damping at street zoom.
const GIS_MAP_LINE_DAMP_MIN: f64 = 0.44;

/// 🌊️ Water classes that stay visible at continent zoom; inland water is dropped there.
const GIS_MAP_OPEN_SEA_CLASSES: &[&str] = &["ocean", "sea", "bay", "strait", "fjord", "lagoon", "sound", "gulf"];

fn names(classes: &[&str]) -> Vec<String> {
    classes.iter().map(|class| (*class).to_string()).collect()
}

/// 🧱️ OpenMapTiles and demotiles source layers: paint or label role, draw rank and fixed label rank.
pub(super) fn gis_map_source_layers() -> Vec<MapSourceLayer> {
    let layer = |name: &str, role: Option<MapLayerRole>, draw_rank: u8, label_rank: Option<u16>| MapSourceLayer { name: name.into(), role, draw_rank, label_rank };
    vec![
        layer("water", Some(MapLayerRole::WaterArea), 0, None),
        layer("landcover", Some(MapLayerRole::LandCover), 1, None),
        layer("landuse", Some(MapLayerRole::LandCover), 2, None),
        layer("park", Some(MapLayerRole::GreenArea), 3, None),
        layer("transportation", Some(MapLayerRole::Road), 4, None),
        layer("building", Some(MapLayerRole::Building), 5, None),
        layer("boundary", Some(MapLayerRole::Boundary), 6, None),
        layer("waterway", Some(MapLayerRole::Waterway), 7, None),
        layer("aeroway", None, 8, None),
        layer("countries", Some(MapLayerRole::AdminArea), 9, None),
        layer("geolines", Some(MapLayerRole::Coastline), 10, None),
        layer("place", Some(MapLayerRole::PlaceLabel), 20, None),
        layer("transportation_name", Some(MapLayerRole::RoadLabel), 21, None),
        layer("water_name", Some(MapLayerRole::PlaceLabel), 22, Some(40)),
        layer("poi", Some(MapLayerRole::PointLabel), 24, Some(48)),
        layer("centroids", Some(MapLayerRole::PlaceLabel), 25, Some(8)),
    ]
}

/// 🔑️ OpenMapTiles property keys: `class`, `admin_level`, `maritime`, and the label candidates in priority order.
pub(super) fn gis_map_feature_keys() -> MapFeatureKeys {
    MapFeatureKeys { class: "class".into(), admin_level: "admin_level".into(), maritime: "maritime".into(), label: names(&["name:en", "name_en", "name_int", "name", "name:latin", "NAME", "Name", "title", "TITLE", "ref"]) }
}

/// 🗺️ Vector tile z cap from the viewport span (finer than the raster band floor so oceans stay separated).
pub(super) fn gis_map_vector_zoom_steps() -> Vec<MapSpanZoom> {
    let caps = GIS_MAP_LOD_MAX_SPAN_DEG;
    vec![MapSpanZoom { above_span_deg: caps[0], tile_z: 3 }, MapSpanZoom { above_span_deg: caps[1], tile_z: 5 }, MapSpanZoom { above_span_deg: caps[2], tile_z: 7 }, MapSpanZoom { above_span_deg: caps[3], tile_z: 6 }]
}

/// 🔭️ Per-band vector draw gates (span + tile z) matched to the OSM raster bands.
pub(super) fn gis_map_band_gates(band: usize) -> MapLayerGates {
    match band {
        0 | 1 => MapLayerGates { water: true, land_backdrop: false, land_cover: true, roads: false, buildings: false, boundaries: false, coastline: true, max_admin_level: 0 },
        2 => MapLayerGates { water: true, land_backdrop: true, land_cover: false, roads: false, buildings: false, boundaries: true, coastline: false, max_admin_level: 2 },
        3 | 4 => MapLayerGates { water: true, land_backdrop: true, land_cover: true, roads: true, buildings: false, boundaries: true, coastline: false, max_admin_level: 6 },
        5 | 6 => MapLayerGates { water: true, land_backdrop: true, land_cover: true, roads: true, buildings: false, boundaries: true, coastline: false, max_admin_level: 8 },
        _ => MapLayerGates { water: true, land_backdrop: true, land_cover: true, roads: true, buildings: true, boundaries: true, coastline: false, max_admin_level: 8 },
    }
}

/// 🏢️ Buildings paint only in the finest band, below city span and from tile z 13.
pub(super) fn gis_map_band_buildings(band: usize) -> (Option<f64>, u32) {
    if band >= 7 {
        (Some(GIS_MAP_LOD_MAX_SPAN_DEG[4]), 13)
    } else {
        (None, 0)
    }
}

fn class_spans(rows: &[(&[&str], f64)], fallback: f64) -> MapClassSpans {
    MapClassSpans { rows: rows.iter().map(|(classes, below)| MapClassSpan { classes: names(classes), below_span_deg: Some(*below) }).collect(), fallback_below_span_deg: Some(fallback) }
}

/// 🛣️ Road classes and the span they stay visible below.
pub(super) fn gis_map_road_spans() -> MapClassSpans {
    class_spans(
        &[
            (&["motorway", "trunk"], 22.0),
            (&["primary"], 12.0),
            (&["secondary", "tertiary"], 6.0),
            (&["minor", "street", "bus_guideway"], 2.8),
            (&["residential", "service", "living_street", "unclassified", "pedestrian"], 1.2),
            (&["path", "track", "footway", "cycleway", "steps"], 0.45),
        ],
        4.0,
    )
}

/// 🚧️ The region band shows only motorway, trunk and primary roads, below country span.
pub(super) fn gis_map_band_road_spans(band: usize) -> Option<MapClassSpans> {
    (band == 3).then(|| class_spans(&[(&["motorway", "trunk", "primary"], 12.0)], 0.0))
}

/// 📏️ Per-band road stroke multiplier (city 30% of the default).
pub(super) fn gis_map_band_road_stroke_scale(band: usize) -> f64 {
    match band {
        3 => GIS_MAP_ROAD_REGION_SCALE,
        4 => GIS_MAP_ROAD_CITY_SCALE,
        _ => 1.0,
    }
}

/// 🌊️ Continent zoom keeps open sea only; every other band paints all water classes.
pub(super) fn gis_map_band_water_area_classes(band: usize) -> Option<Vec<String>> {
    (band == 1).then(|| names(GIS_MAP_OPEN_SEA_CLASSES))
}

pub(super) fn gis_map_band_waterways(band: usize) -> bool {
    band >= 2
}

/// 🗺️ Country-polygon lake holes are suppressed only at continent zoom.
pub(super) fn gis_map_band_admin_area_holes(band: usize) -> bool {
    band != 1
}

/// 🖌️ World, continent and country paint a land wash under the vector tiles.
pub(super) fn gis_map_band_land_wash(band: usize) -> bool {
    band <= 2
}

/// 🌿️ Continent zoom draws land cover even over country polygons.
pub(super) fn gis_map_band_land_cover_over_admin_areas(band: usize) -> bool {
    band == 1
}

/// 🔤️ Label screen px at each band's coarse edge.
pub(super) fn gis_map_band_label_px(band: usize) -> f64 {
    GIS_MAP_LABEL_PX_BANDS.get(band).copied().unwrap_or(10.5)
}

/// 🖊️ Road stroke base width per class.
pub(super) fn gis_map_road_widths() -> MapClassWidths {
    let row = |classes: &[&str], width: f64| MapClassWidth { classes: names(classes), width };
    MapClassWidths {
        rows: vec![
            row(&["motorway"], GIS_MAP_ROAD_MOTORWAY_WIDTH),
            row(&["trunk"], 2.1),
            row(&["primary"], 1.8),
            row(&["secondary"], 1.45),
            row(&["tertiary"], 1.2),
            row(&["minor", "street"], 0.95),
            row(&["residential", "service", "living_street"], 0.78),
            row(&["path", "track", "footway", "cycleway", "steps"], GIS_MAP_ROAD_PATH_WIDTH),
        ],
        fallback: 0.9,
        min: GIS_MAP_ROAD_WIDTH_MIN,
        max: GIS_MAP_ROAD_WIDTH_MAX,
    }
}

/// 🏛️ Boundary stroke base width per administrative level.
pub(super) fn gis_map_boundary_widths() -> MapLevelWidths {
    MapLevelWidths {
        rows: vec![MapLevelWidth { min_level: 2, max_level: 2, width: GIS_MAP_BOUNDARY_ADMIN2_WIDTH }, MapLevelWidth { min_level: 3, max_level: 4, width: 1.3 }, MapLevelWidth { min_level: 5, max_level: 6, width: 1.0 }],
        fallback: GIS_MAP_BOUNDARY_DEFAULT_WIDTH,
        min: GIS_MAP_BOUNDARY_WIDTH_MIN,
        max: GIS_MAP_BOUNDARY_WIDTH_MAX,
    }
}

/// 🏖️ Coastline stroke width.
pub(super) fn gis_map_coastline_width() -> MapWidth {
    MapWidth { width: GIS_MAP_COASTLINE_WIDTH, min: GIS_MAP_COASTLINE_WIDTH_MIN, max: GIS_MAP_COASTLINE_WIDTH_MAX }
}

/// 📐️ Screen stroke scale from the viewport span, damped in the region, city, district and street bands.
pub(super) fn gis_map_line_scale() -> MapLineScale {
    let caps = GIS_MAP_LOD_MAX_SPAN_DEG;
    let (country, region, city, district, street) = (caps[2], caps[3], caps[4], caps[5], caps[6]);
    let linear = |up_to_span_deg: f64, from_span_deg: f64, base: f64, slope: f64| MapLineDamping { up_to_span_deg, from_span_deg, base, slope, curve: MapDampingCurve::Linear };
    MapLineScale {
        coefficient: GIS_MAP_LINE_SCALE_COEFFICIENT,
        min_span_deg: 0.08,
        coefficient_min_span_deg: 0.25,
        min: 0.5,
        max: GIS_MAP_LINE_SCALE_CAP,
        segments: vec![
            MapLineDamping { up_to_span_deg: street, from_span_deg: 0.0, base: GIS_MAP_LINE_DAMP_MIN, slope: 0.1, curve: MapDampingCurve::SquareRoot },
            linear(district, street, 0.5, 0.1),
            linear(city, district, 0.56, 0.1),
            linear(region, city, 0.66, 0.1),
            linear(country, region, 0.8, 0.08),
        ],
        beyond: 1.0,
    }
}

/// 🏷️ Label sizing, visibility and ranking: street names are stricter than road geometry, POIs appear from district zoom, place classes each own one band range.
pub(super) fn gis_map_label_rules() -> MapLabelRules {
    let caps = GIS_MAP_LOD_MAX_SPAN_DEG;
    let rank = |classes: &[&str], rank: u16| MapClassRank { classes: names(classes), rank };
    let range = |classes: &[&str], max_span_deg: Option<f64>, min_span_deg: Option<f64>| MapClassSpanRange { classes: names(classes), max_span_deg, min_span_deg };
    MapLabelRules {
        span_floor_deg: 180.0,
        min_span_deg: 0.05,
        road_label_spans: class_spans(
            &[
                (&["motorway", "trunk"], caps[1]),
                (&["primary"], caps[3]),
                (&["secondary", "tertiary"], caps[4]),
                (&["minor", "street", "bus_guideway"], caps[5]),
                (&["residential", "service", "living_street", "unclassified", "pedestrian"], caps[6]),
                (&["path", "track", "footway", "cycleway", "steps"], caps[6] * 0.85),
            ],
            caps[4],
        ),
        road_label_ranks: MapClassRanks { rows: vec![rank(&["motorway"], 30), rank(&["trunk"], 31), rank(&["primary"], 32), rank(&["secondary"], 34), rank(&["tertiary"], 36)], fallback: 44 },
        place_label_spans: vec![
            range(&[""], Some(caps[1]), Some(caps[2])),
            range(&["continent"], None, Some(caps[1])),
            range(&["country"], Some(caps[1]), Some(caps[2])),
            range(&["state", "province"], Some(caps[2]), Some(caps[3])),
            range(&["city"], Some(caps[3]), Some(caps[4])),
            range(&["town"], Some(caps[4]), Some(caps[5])),
            range(&["village", "hamlet", "suburb"], Some(caps[5]), Some(caps[6])),
            range(&["neighbourhood", "quarter", "isolated_dwelling"], Some(caps[6]), None),
        ],
        place_label_ranks: MapClassRanks {
            rows: vec![rank(&["continent"], 0), rank(&["country", ""], 4), rank(&["state", "province"], 8), rank(&["city"], 12), rank(&["town"], 16), rank(&["village", "hamlet", "suburb"], 20), rank(&["neighbourhood", "quarter", "isolated_dwelling"], 24)],
            fallback: 28,
        },
        point_label_max_span_deg: caps[5],
    }
}
// #endregion 🔖️VectorStyle

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
