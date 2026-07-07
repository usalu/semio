//! 🗺️ GIS map on the infinite canvas: Web Mercator tiles, positions, routes, regions.

pub use infinite_cavas::{self as cavas, *};
pub use std::sync::Arc;

use cavas::lod::{Lod, LodScale};

// #region 🔖MapPalette
fn map_color(rgba: [f32; 4]) -> Color {
    Color::new(rgba)
}

#[derive(Clone, Copy, Debug)]
pub struct MapThemePalette {
    pub surface_clear: Color,
    pub land_fill: Color,
    pub land_stroke: Color,
    pub label_fill: Color,
    pub label_halo: Color,
    pub region_fill: Color,
    pub region_stroke: Color,
    pub route_stroke: Color,
    pub position_fill: Color,
    pub position_stroke: Color,
    pub selection_stroke: Color,
    pub hover_stroke: Color,
}

impl MapThemePalette {
    /// @emoji 🎨 Builds a map palette from centralized theme tokens.
    pub fn from_map_theme(t: &ui_styling::MapTheme) -> Self {
        Self {
            surface_clear: map_color(t.surface_clear),
            land_fill: map_color(t.land_fill),
            land_stroke: map_color(t.land_stroke),
            label_fill: map_color(t.label_fill),
            label_halo: map_color(t.label_halo),
            region_fill: map_color(t.region_fill),
            region_stroke: map_color(t.region_stroke),
            route_stroke: map_color(t.route_stroke),
            position_fill: map_color(t.position_fill),
            position_stroke: map_color(t.position_stroke),
            selection_stroke: map_color(t.position_fill),
            hover_stroke: map_color(t.route_stroke),
        }
    }
}

impl Default for MapThemePalette {
    fn default() -> Self {
        Self::from_map_theme(&ui_styling::MAP_LIGHT)
    }
}
// #endregion 🔖MapPalette

// #region 🔖MapLod
const GIS_MAP_LODS: &[Lod] = &[
    Lod { id: "world", name: "World", description: "Entire planet; coarsest OSM tiles.", max_zoom: 360.0 },
    Lod { id: "continent", name: "Continent", description: "Multi-country overview.", max_zoom: 680.0 },
    Lod { id: "country", name: "Country", description: "National extent.", max_zoom: 1_280.0 },
    Lod { id: "region", name: "Region", description: "Regional detail.", max_zoom: 2_400.0 },
    Lod { id: "city", name: "City", description: "Metropolitan area.", max_zoom: 4_400.0 },
    Lod { id: "district", name: "District", description: "Neighbourhood streets.", max_zoom: 6_400.0 },
    Lod { id: "street", name: "Street", description: "Block-level detail.", max_zoom: 7_200.0 },
    Lod { id: "building", name: "Building", description: "Maximum map fidelity.", max_zoom: f64::INFINITY },
];

const GIS_MAP_LOD_TILE_Z: &[u32] = &[0, 1, 2, 3, 4, 5, 7, 10, 18];

/// @emoji 🔭 OSM raster tiles for automatic viewport picking.
pub const MAP_RASTER_TILE_Z_MAX: u32 = 19;

/// @emoji 🌐 Upper visible longitude span (degrees) per band; coarser band when span exceeds threshold.
const GIS_MAP_LOD_MAX_SPAN_DEG: &[f64] = &[100.0, 35.0, 12.0, 4.0, 1.2, 0.35, 0.1, 0.0];

const GIS_MAP_LOD_SCALE: LodScale = LodScale { lods: GIS_MAP_LODS };

const MAX_MAP_TILE_CACHE_ENTRIES: usize = 512;

const MAX_VISIBLE_TILE_REQUESTS: usize = 256;

/// @emoji 📶 Window LOD select value: camera zoom picks the tile band.
pub const GIS_MAP_LOD_MODE_AUTOMATIC: &str = "automatic";

pub fn gis_map_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = GIS_MAP_LODS
        .iter()
        .enumerate()
        .map(|(i, lod)| {
            serde_json::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
                "maxSpanDeg": GIS_MAP_LOD_MAX_SPAN_DEG.get(i).copied().unwrap_or(0.0),
                "tileZ": GIS_MAP_LOD_TILE_Z.get(i).copied().unwrap_or(0),
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn viewport_lon_span_degrees(camera: &cavas::camera::Camera, viewport: &cavas::camera::Viewport) -> f64 {
    let cy = viewport.height as f64 * 0.5;
    let left = map_viewport::screen_to_world(camera, viewport, Point::new(0.0, cy));
    let right = map_viewport::screen_to_world(camera, viewport, Point::new(viewport.width as f64, cy));
    let (lon0, _) = projection::world_to_lonlat(left.x, left.y);
    let (lon1, _) = projection::world_to_lonlat(right.x, right.y);
    let span = (lon1 - lon0).abs();
    if !span.is_finite() {
        return 360.0;
    }
    span.min(360.0)
}

fn resolve_map_lod_index_from_span(span_deg: f64) -> usize {
    for (i, threshold) in GIS_MAP_LOD_MAX_SPAN_DEG.iter().enumerate() {
        if span_deg > *threshold {
            return i;
        }
    }
    GIS_MAP_LODS.len().saturating_sub(1)
}

fn resolve_detail_lod_index(span_deg: f64, forced_lod_id: Option<&str>) -> usize {
    if let Some(id) = forced_lod_id {
        if let Some(idx) = GIS_MAP_LOD_SCALE.index_of(id) {
            return idx;
        }
    }
    resolve_map_lod_index_from_span(span_deg)
}

/// @emoji 🔭 Representative viewport longitude span (degrees) for a pinned LOD band.
pub fn representative_viewport_span_for_lod(lod_idx: usize) -> f64 {
    match lod_idx {
        0 => 180.0,
        1 => 50.0,
        2 => 20.0,
        3 => 8.0,
        4 => 2.0,
        5 => 0.6,
        6 => 0.2,
        _ => 0.05,
    }
}

fn current_map_lod(camera: &cavas::camera::Camera, viewport: &cavas::camera::Viewport) -> &'static Lod {
    let idx = resolve_map_lod_index_from_span(viewport_lon_span_degrees(camera, viewport));
    &GIS_MAP_LODS[idx]
}

fn ideal_tile_z_for_viewport(camera: &cavas::camera::Camera, viewport: &cavas::camera::Viewport) -> u32 {
    let span = viewport_lon_span_degrees(camera, viewport).max(1e-6);
    let w = viewport.width.max(1) as f64;
    let z = ((360.0 / span) * (w / 256.0)).log2();
    z.round().clamp(0.0, f64::from(MAP_RASTER_TILE_Z_MAX)) as u32
}

fn forced_lod_tile_z(id: &str) -> Option<u32> {
    let idx = GIS_MAP_LOD_SCALE.index_of(id)?;
    GIS_MAP_LOD_TILE_Z.get(idx).copied()
}

/// @emoji 🧷 Pinned LOD is a minimum tile-detail floor; world/continent automatic bands use fixed coarse tile z.
fn pick_tile_z_target(camera: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, forced_lod_id: Option<&str>) -> u32 {
    let ideal = ideal_tile_z_for_viewport(camera, viewport);
    let span = viewport_lon_span_degrees(camera, viewport);
    let lod_idx = resolve_map_lod_index_from_span(span);
    if let Some(id) = forced_lod_id {
        let Some(pin_floor) = forced_lod_tile_z(id) else {
            return ideal;
        };
        let coarse_floor = GIS_MAP_LOD_TILE_Z.get(lod_idx).copied().unwrap_or(0);
        let floor = if lod_idx <= 1 { coarse_floor } else { pin_floor };
        return floor.max(ideal);
    }
    if lod_idx <= 1 {
        return GIS_MAP_LOD_TILE_Z.get(lod_idx).copied().unwrap_or(0);
    }
    ideal.min(vector_tiles::max_tile_z_for_span(span))
}

fn active_map_lod(forced_lod_id: Option<&str>, camera: &cavas::camera::Camera, viewport: &cavas::camera::Viewport) -> &'static Lod {
    if let Some(id) = forced_lod_id {
        if let Some(idx) = GIS_MAP_LOD_SCALE.index_of(id) {
            return &GIS_MAP_LODS[idx];
        }
    }
    current_map_lod(camera, viewport)
}

/// @emoji 🔭 Whole-world fit on an ~800px viewport is ~300; min allows shrinking the planet to a few pixels.
pub const MAP_CAMERA_ZOOM_MIN: f64 = 8.0;
/// @emoji 🔭 ~100M yields ~1.4e-3° longitude across 800px (~150 m), i.e. street scale (see `viewport_lon_span_degrees`).
pub const MAP_CAMERA_ZOOM_MAX: f64 = 100_000_000.0;

pub fn gis_map_camera_limits_json() -> String {
    gis_map_camera_limits_json_for_viewport(&cavas::camera::Viewport::default())
}

pub fn gis_map_camera_limits_json_for_viewport(viewport: &cavas::camera::Viewport) -> String {
    serde_json::json!({
        "min": projection::cover_zoom_for_viewport(viewport).max(MAP_CAMERA_ZOOM_MIN),
        "max": MAP_CAMERA_ZOOM_MAX,
    })
    .to_string()
}

fn clamp_map_zoom(zoom: f64) -> f64 {
    zoom.clamp(MAP_CAMERA_ZOOM_MIN, MAP_CAMERA_ZOOM_MAX)
}

fn clamp_map_zoom_for_viewport(zoom: f64, viewport: &cavas::camera::Viewport) -> f64 {
    zoom.max(projection::cover_zoom_for_viewport(viewport)).min(MAP_CAMERA_ZOOM_MAX)
}

/// @emoji 🧷 Keeps the viewport filled by the world map with no empty margins or outscroll.
fn clamp_camera_to_world_bounds(camera: &mut cavas::camera::Camera, viewport: &cavas::camera::Viewport) {
    camera.zoom = clamp_map_zoom_for_viewport(camera.zoom, viewport);
    let half_w = viewport.width as f64 / (2.0 * camera.zoom);
    let half_h = viewport.height as f64 / (2.0 * camera.zoom);
    let lim_x = projection::WORLD_HALF - half_w;
    let lim_y = projection::WORLD_HALF - half_h;
    camera.x = if lim_x <= 0.0 { 0.0 } else { camera.x.clamp(-lim_x, lim_x) };
    camera.y = if lim_y <= 0.0 { 0.0 } else { camera.y.clamp(-lim_y, lim_y) };
}

mod map_viewport {
    use super::Point;
    use crate::cavas::camera::{screen_to_world as cavas_screen_to_world, world_to_screen as cavas_world_to_screen, Camera, Viewport};

    pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        cavas_world_to_screen(camera, viewport, Point::new(p.x, -p.y))
    }

    pub fn screen_to_world(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        let w = cavas_screen_to_world(camera, viewport, p);
        Point::new(w.x, -w.y)
    }
}

fn map_wheel_screen(camera: &mut cavas::camera::Camera, viewport: &cavas::camera::Viewport, sx: f64, sy: f64, delta_y: f64) {
    let zoom_factor = if delta_y < 0.0 {
        1.12
    } else if delta_y > 0.0 {
        1.0 / 1.12
    } else {
        1.0
    };
    let next_zoom = clamp_map_zoom(camera.zoom * zoom_factor);
    let screen = Point::new(sx, sy);
    let world_before = map_viewport::screen_to_world(camera, viewport, screen);
    let half_w = viewport.width as f64 / 2.0;
    let half_h = viewport.height as f64 / 2.0;
    camera.x = world_before.x - (sx - half_w) / next_zoom;
    camera.y = -world_before.y - (sy - half_h) / next_zoom;
    camera.zoom = next_zoom;
}
// #endregion 🔖MapLod

// #region 🔖Projection
pub mod projection {
    use super::Point;
    use super::Rect;

    pub const MAX_LAT: f64 = 85.051_128_78;
    pub const WORLD_HALF: f64 = 1.0;

    pub fn lonlat_to_world(lon: f64, lat: f64) -> Point {
        let lon = lon.clamp(-180.0, 180.0);
        let lat = lat.clamp(-MAX_LAT, MAX_LAT);
        let x = lon / 180.0 * WORLD_HALF;
        let lat_rad = lat.to_radians();
        let y = (0.5 * (std::f64::consts::FRAC_PI_4 + lat_rad * 0.5).tan().ln() / std::f64::consts::PI) * WORLD_HALF * 2.0;
        Point::new(x, y)
    }

    pub fn world_to_lonlat(x: f64, y: f64) -> (f64, f64) {
        let lon = (x / WORLD_HALF) * 180.0;
        let lat_rad = 2.0 * (y / WORLD_HALF * std::f64::consts::PI).exp().atan() - std::f64::consts::FRAC_PI_2;
        let lat = lat_rad.to_degrees();
        (lon, lat)
    }

    /// @emoji 📐 Mercator world span in map units (`[-WORLD_HALF, WORLD_HALF]` on each axis).
    pub const WORLD_VISIBLE_SPAN: f64 = WORLD_HALF * 2.0;

    /// @emoji 📐 Minimum zoom so every viewport pixel maps inside the world (cover, no outscroll).
    pub fn cover_zoom_for_viewport(viewport: &crate::cavas::camera::Viewport) -> f64 {
        let vw = viewport.width.max(1) as f64;
        let vh = viewport.height.max(1) as f64;
        vw.max(vh) / WORLD_VISIBLE_SPAN
    }

    pub fn default_world_camera(viewport: &crate::cavas::camera::Viewport) -> crate::cavas::camera::Camera {
        let zoom = cover_zoom_for_viewport(viewport).max(super::MAP_CAMERA_ZOOM_MIN);
        crate::cavas::camera::Camera { x: 0.0, y: 0.0, zoom }
    }

    pub fn tile_world_rect(z: u32, x: u32, y: u32) -> Rect {
        let n = 2.0_f64.powi(z as i32);
        let step = (WORLD_HALF * 2.0) / n;
        let min_x = -WORLD_HALF + x as f64 * step;
        let max_y = WORLD_HALF - y as f64 * step;
        Rect::new(min_x, max_y - step, min_x + step, max_y)
    }
}
// #endregion 🔖Projection

// #region 🔖Tiles
pub mod tiles {
    use super::map_viewport;
    use super::projection::WORLD_HALF;
    use super::{pick_tile_z_target, Point, MAX_VISIBLE_TILE_REQUESTS};
    use crate::cavas::camera::{Camera, Viewport};

    pub fn pick_zoom(camera: &Camera, viewport: &Viewport, forced_lod_id: Option<&str>) -> u32 {
        let mut z = pick_tile_z_target(camera, viewport, forced_lod_id);
        while z > 0 && visible_tiles(camera, viewport, z).len() > MAX_VISIBLE_TILE_REQUESTS {
            z -= 1;
        }
        z
    }

    pub fn visible_tiles(camera: &Camera, viewport: &Viewport, z: u32) -> Vec<(u32, u32, u32)> {
        let corners = [
            map_viewport::screen_to_world(camera, viewport, Point::new(0.0, 0.0)),
            map_viewport::screen_to_world(camera, viewport, Point::new(viewport.width as f64, 0.0)),
            map_viewport::screen_to_world(camera, viewport, Point::new(viewport.width as f64, viewport.height as f64)),
            map_viewport::screen_to_world(camera, viewport, Point::new(0.0, viewport.height as f64)),
        ];
        let mut min_x = WORLD_HALF;
        let mut max_x = -WORLD_HALF;
        let mut min_y = WORLD_HALF;
        let mut max_y = -WORLD_HALF;
        for p in corners {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
        let n = 2.0_f64.powi(z as i32);
        let step = (WORLD_HALF * 2.0) / n;
        let x0 = ((min_x + WORLD_HALF) / step).floor().max(0.0) as u32;
        let x1 = ((max_x + WORLD_HALF) / step).ceil().min(n - 1.0) as u32;
        let y0 = ((WORLD_HALF - max_y) / step).floor().max(0.0) as u32;
        let y1 = ((WORLD_HALF - min_y) / step).ceil().min(n - 1.0) as u32;
        let mut out = Vec::new();
        for x in x0..=x1 {
            for y in y0..=y1 {
                out.push((z, x, y));
            }
        }
        out
    }

    pub fn tile_key(z: u32, x: u32, y: u32) -> String {
        format!("{z}/{x}/{y}")
    }

    pub fn parse_tile_key(key: &str) -> Option<(u32, u32, u32)> {
        let mut parts = key.split('/');
        let z: u32 = parts.next()?.parse().ok()?;
        let x: u32 = parts.next()?.parse().ok()?;
        let y: u32 = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Some((z, x, y))
    }

    pub fn tile_key_ancestors(z: u32, x: u32, y: u32) -> Vec<String> {
        let mut out = Vec::new();
        let mut cz = z;
        let mut cx = x;
        let mut cy = y;
        loop {
            out.push(tile_key(cz, cx, cy));
            if cz == 0 {
                break;
            }
            cz -= 1;
            cx >>= 1;
            cy >>= 1;
        }
        out
    }

    pub fn tile_retention_keys(visible: &[(u32, u32, u32)], previous: &std::collections::BTreeSet<String>) -> std::collections::BTreeSet<String> {
        let mut keys = std::collections::BTreeSet::new();
        for &(z, x, y) in visible {
            for k in tile_key_ancestors(z, x, y) {
                keys.insert(k);
            }
        }
        keys.extend(previous.iter().cloned());
        keys
    }
}
// #endregion 🔖Tiles

// #region 🔖VectorTiles
pub mod vector_tiles {
    use crate::Color;
    use prost::Message;

    /// OpenFreeMap planet tiles (OpenMapTiles schema, OSM) — max generated zoom.
    pub const MAP_VECTOR_TILE_MAX_Z: u32 = 14;
    pub const DEFAULT_MVT_EXTENT: u32 = 4096;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum GeomType {
        Unknown,
        Point,
        LineString,
        Polygon,
    }

    #[derive(Clone, Debug, Default)]
    pub struct VectorTile {
        pub layers: Vec<VectorLayer>,
    }

    #[derive(Clone, Debug)]
    pub struct VectorLayer {
        pub name: String,
        pub extent: u32,
        pub features: Vec<VectorFeature>,
    }

    #[derive(Clone, Debug)]
    pub struct VectorFeature {
        pub id: Option<u64>,
        pub geom_type: GeomType,
        pub rings: Vec<Vec<(f64, f64)>>,
        pub lines: Vec<Vec<(f64, f64)>>,
        pub points: Vec<(f64, f64)>,
        pub properties: std::collections::BTreeMap<String, String>,
    }

    #[derive(Clone, PartialEq, Message)]
    struct RawTile {
        #[prost(uint32, optional, tag = "15")]
        version: Option<u32>,
        #[prost(message, repeated, tag = "3")]
        layers: Vec<RawLayer>,
    }

    #[derive(Clone, PartialEq, Message)]
    struct RawLayer {
        #[prost(uint32, optional, tag = "15")]
        version: Option<u32>,
        #[prost(string, optional, tag = "1")]
        name: Option<String>,
        #[prost(message, repeated, tag = "2")]
        features: Vec<RawFeature>,
        #[prost(string, repeated, tag = "3")]
        keys: Vec<String>,
        #[prost(message, repeated, tag = "4")]
        values: Vec<RawValue>,
        #[prost(uint32, optional, tag = "5")]
        extent: Option<u32>,
    }

    #[derive(Clone, PartialEq, Message)]
    struct RawFeature {
        #[prost(uint64, optional, tag = "1")]
        id: Option<u64>,
        #[prost(uint64, repeated, packed = "true", tag = "2")]
        tags: Vec<u64>,
        #[prost(enumeration = "RawGeomType", optional, tag = "3")]
        geom_type: Option<i32>,
        #[prost(uint32, repeated, packed = "true", tag = "4")]
        geometry: Vec<u32>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, prost::Enumeration)]
    #[repr(i32)]
    enum RawGeomType {
        Unknown = 0,
        Point = 1,
        LineString = 2,
        Polygon = 3,
    }

    #[derive(Clone, PartialEq, Message)]
    struct RawValue {
        #[prost(string, optional, tag = "1")]
        string_value: Option<String>,
        #[prost(float, optional, tag = "2")]
        float_value: Option<f32>,
        #[prost(double, optional, tag = "3")]
        double_value: Option<f64>,
        #[prost(int64, optional, tag = "4")]
        int_value: Option<i64>,
        #[prost(uint64, optional, tag = "5")]
        uint_value: Option<u64>,
        #[prost(bool, optional, tag = "7")]
        bool_value: Option<bool>,
    }

    fn zigzag_decode(n: u32) -> i32 {
        ((n >> 1) as i32) ^ (-((n & 1) as i32))
    }

    fn decode_properties(tags: &[u64], keys: &[String], values: &[RawValue]) -> std::collections::BTreeMap<String, String> {
        let mut out = std::collections::BTreeMap::new();
        let mut i = 0usize;
        while i + 1 < tags.len() {
            let key_idx = tags[i] as usize;
            let val_idx = tags[i + 1] as usize;
            i += 2;
            let Some(key) = keys.get(key_idx) else {
                continue;
            };
            let Some(raw) = values.get(val_idx) else {
                continue;
            };
            let val = raw_value_string(raw);
            if !val.is_empty() {
                out.insert(key.clone(), val);
            }
        }
        out
    }

    fn raw_value_string(v: &RawValue) -> String {
        if let Some(s) = &v.string_value {
            return s.clone();
        }
        if let Some(n) = v.int_value {
            return n.to_string();
        }
        if let Some(n) = v.uint_value {
            return n.to_string();
        }
        if let Some(n) = v.double_value {
            return n.to_string();
        }
        if let Some(n) = v.float_value {
            return n.to_string();
        }
        if let Some(b) = v.bool_value {
            return b.to_string();
        }
        String::new()
    }

    fn decode_geometry(geometry: &[u32], geom_type: GeomType) -> (Vec<Vec<(f64, f64)>>, Vec<Vec<(f64, f64)>>, Vec<(f64, f64)>) {
        let mut rings = Vec::new();
        let mut lines = Vec::new();
        let mut points = Vec::new();
        let mut i = 0usize;
        let mut cursor_x = 0i32;
        let mut cursor_y = 0i32;
        let mut current: Vec<(f64, f64)> = Vec::new();
        while i < geometry.len() {
            let cmd_int = geometry[i];
            i += 1;
            let cmd = cmd_int & 0x7;
            let count = (cmd_int >> 3) as usize;
            match cmd {
                1 => {
                    for _ in 0..count {
                        if i + 1 >= geometry.len() {
                            break;
                        }
                        cursor_x += zigzag_decode(geometry[i]);
                        cursor_y += zigzag_decode(geometry[i + 1]);
                        i += 2;
                        if matches!(geom_type, GeomType::Polygon) && !current.is_empty() {
                            rings.push(std::mem::take(&mut current));
                        }
                        if matches!(geom_type, GeomType::LineString) && !current.is_empty() {
                            lines.push(std::mem::take(&mut current));
                        }
                        current.push((cursor_x as f64, cursor_y as f64));
                    }
                }
                2 => {
                    for _ in 0..count {
                        if i + 1 >= geometry.len() {
                            break;
                        }
                        cursor_x += zigzag_decode(geometry[i]);
                        cursor_y += zigzag_decode(geometry[i + 1]);
                        i += 2;
                        current.push((cursor_x as f64, cursor_y as f64));
                    }
                }
                7 => {
                    if current.len() >= 2 {
                        if matches!(geom_type, GeomType::Polygon) {
                            if let Some(first) = current.first() {
                                if current.last() != Some(first) {
                                    current.push(*first);
                                }
                            }
                            rings.push(std::mem::take(&mut current));
                        } else if matches!(geom_type, GeomType::LineString) {
                            lines.push(std::mem::take(&mut current));
                        }
                    }
                    current.clear();
                }
                _ => break,
            }
        }
        if !current.is_empty() {
            match geom_type {
                GeomType::Polygon => rings.push(current),
                GeomType::LineString => lines.push(current),
                GeomType::Point => {
                    if let Some(p) = current.first() {
                        points.push(*p);
                    }
                }
                GeomType::Unknown => {}
            }
        }
        (rings, lines, points)
    }

    #[cfg(test)]
    mod decode_geometry_tests {
        use super::{decode_geometry, GeomType};

        fn zigzag(n: i32) -> u32 {
            ((n << 1) ^ (n >> 31)) as u32
        }

        #[test]
        fn mvt_segment_is_tile_seam_on_extent_bbox() {
            let extent = 4096;
            assert!(super::mvt_segment_is_tile_seam(extent, (0.0, 0.0), (4096.0, 0.0)));
            assert!(super::mvt_segment_is_tile_seam(extent, (4096.0, 100.0), (4096.0, 900.0)));
            assert!(!super::mvt_segment_is_tile_seam(extent, (0.0, 0.0), (200.0, 200.0)));
            assert!(!super::mvt_segment_is_tile_seam(extent, (200.0, 200.0), (300.0, 400.0)));
            assert!(super::mvt_segment_touches_tile_bbox(extent, (0.0, 0.0), (200.0, 200.0)));
            assert!(!super::mvt_segment_touches_tile_bbox(extent, (200.0, 200.0), (300.0, 400.0)));
            assert!(super::mvt_ring_is_tile_bbox_cover(extent, &[(0.0, 0.0), (4096.0, 0.0), (4096.0, 4096.0), (0.0, 4096.0), (0.0, 0.0)],));
        }

        #[test]
        fn continent_water_filter_drops_inland_polygons() {
            let mut props = std::collections::BTreeMap::new();
            props.insert("class".to_string(), "lake".to_string());
            assert!(super::water_polygon_visible_for_lod(0, &props));
            assert!(!super::water_polygon_visible_for_lod(1, &props));
            assert!(super::water_polygon_visible_for_lod(2, &props));
            props.insert("class".to_string(), "ocean".to_string());
            assert!(super::water_polygon_visible_for_lod(1, &props));
            assert!(!super::waterway_visible_for_lod(1));
            assert!(super::waterway_visible_for_lod(2));
            assert!(super::country_polygon_holes_visible_for_lod(0));
            assert!(!super::country_polygon_holes_visible_for_lod(1));
            assert!(super::country_polygon_holes_visible_for_lod(2));
        }

        #[test]
        fn weighted_opaque_fill_keeps_alpha_solid() {
            let c = super::weighted_opaque_fill(crate::Color::from_rgba8(40, 50, 60, 128), 1.0);
            assert_eq!(c.to_rgba8().a, 255);
        }

        #[test]
        fn linestring_moveto_starts_new_part() {
            let geometry = vec![(1 << 3) | 1, zigzag(0), zigzag(0), (1 << 3) | 2, zigzag(10), zigzag(0), (1 << 3) | 1, zigzag(90), zigzag(100), (1 << 3) | 2, zigzag(10), zigzag(0)];
            let (_, lines, _) = decode_geometry(&geometry, GeomType::LineString);
            assert_eq!(lines.len(), 2, "each MoveTo must start a new line part");
            assert_eq!(lines[0], vec![(0.0, 0.0), (10.0, 0.0)]);
            assert_eq!(lines[1], vec![(100.0, 100.0), (110.0, 100.0)]);
        }
    }

    pub fn decode_mvt(bytes: &[u8]) -> Result<VectorTile, String> {
        let raw = RawTile::decode(bytes).map_err(|e| e.to_string())?;
        let mut layers = Vec::new();
        for layer in raw.layers {
            let extent = layer.extent.filter(|e| *e > 0).unwrap_or(DEFAULT_MVT_EXTENT);
            let name = layer.name.unwrap_or_default();
            let mut features = Vec::new();
            for feat in layer.features {
                let geom_type = match feat.geom_type.and_then(|g| RawGeomType::try_from(g).ok()) {
                    Some(RawGeomType::Point) => GeomType::Point,
                    Some(RawGeomType::LineString) => GeomType::LineString,
                    Some(RawGeomType::Polygon) => GeomType::Polygon,
                    _ => GeomType::Unknown,
                };
                let props = decode_properties(&feat.tags, &layer.keys, &layer.values);
                let (rings, lines, points) = decode_geometry(&feat.geometry, geom_type.clone());
                features.push(VectorFeature { id: feat.id.filter(|id| *id != 0), geom_type, rings, lines, points, properties: props });
            }
            layers.push(VectorLayer { name, extent, features });
        }
        Ok(VectorTile { layers })
    }

    pub fn property_str<'a>(properties: &'a std::collections::BTreeMap<String, String>, key: &str) -> Option<&'a str> {
        properties.get(key).map(|s| s.as_str()).filter(|s| !s.is_empty())
    }

    pub fn property_class(properties: &std::collections::BTreeMap<String, String>) -> &str {
        property_str(properties, "class").unwrap_or("")
    }

    pub fn property_u64(properties: &std::collections::BTreeMap<String, String>, key: &str) -> Option<u64> {
        properties.get(key)?.parse().ok()
    }

    pub fn feature_label(properties: &std::collections::BTreeMap<String, String>) -> Option<String> {
        for key in ["name:en", "name_en", "name_int", "name", "name:latin", "NAME", "Name", "title", "TITLE", "ref"] {
            if let Some(v) = properties.get(key) {
                let t = v.trim();
                if !t.is_empty() {
                    return Some(t.to_string());
                }
            }
        }
        None
    }

    pub fn layer_draw_rank(layer: &str) -> u8 {
        match layer {
            "water" => 0,
            "landcover" => 1,
            "landuse" => 2,
            "park" => 3,
            "transportation" => 4,
            "building" => 5,
            "boundary" => 6,
            "waterway" => 7,
            "aeroway" => 8,
            "countries" => 9,
            "geolines" => 10,
            "place" => 20,
            "transportation_name" => 21,
            "water_name" => 22,
            "poi" => 24,
            "centroids" => 25,
            _ => 15,
        }
    }

    /// @emoji 🔭 Per-viewport vector draw gates (span + tile z) to match OSM raster LOD.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct VectorDetailProfile {
        pub draw_water: bool,
        pub draw_land_backdrop: bool,
        pub draw_landcover: bool,
        pub draw_transportation: bool,
        pub draw_buildings: bool,
        pub draw_boundary: bool,
        pub draw_coastline: bool,
        pub max_admin_level: u64,
    }

    /// @emoji 🗺️ Vector tile z cap from viewport span (finer than raster LOD floor so oceans stay separated).
    pub fn max_tile_z_for_span(span_deg: f64) -> u32 {
        let z = if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[0] {
            3
        } else if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[1] {
            5
        } else if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[2] {
            7
        } else if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[3] {
            6
        } else {
            MAP_VECTOR_TILE_MAX_Z
        };
        z.min(MAP_VECTOR_TILE_MAX_Z)
    }

    pub fn vector_detail_profile(span_deg: f64, tile_z: u32, forced_lod_id: Option<&str>) -> VectorDetailProfile {
        let lod_idx = super::resolve_detail_lod_index(span_deg, forced_lod_id);
        vector_detail_profile_for_lod(lod_idx, span_deg, tile_z)
    }

    fn vector_detail_profile_for_lod(lod_idx: usize, span_deg: f64, tile_z: u32) -> VectorDetailProfile {
        match lod_idx {
            0 | 1 => VectorDetailProfile { draw_water: true, draw_land_backdrop: false, draw_landcover: true, draw_transportation: false, draw_buildings: false, draw_boundary: false, draw_coastline: true, max_admin_level: 0 },
            2 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: false, draw_transportation: false, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 2 },
            3 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: true, draw_transportation: true, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 6 },
            4 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: true, draw_transportation: true, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 6 },
            5 | 6 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: true, draw_transportation: true, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 8 },
            _ => VectorDetailProfile {
                draw_water: true,
                draw_land_backdrop: true,
                draw_landcover: true,
                draw_transportation: true,
                draw_buildings: span_deg < super::GIS_MAP_LOD_MAX_SPAN_DEG[4] && tile_z >= 13,
                draw_boundary: true,
                draw_coastline: false,
                max_admin_level: 8,
            },
        }
    }

    fn boundary_visible_for_lod(admin_level: u64, lod_idx: usize) -> bool {
        match lod_idx {
            0 | 1 => false,
            2 => admin_level == 2,
            3 | 4 => admin_level >= 2 && admin_level <= 6,
            _ => admin_level >= 2 && admin_level <= 8,
        }
    }

    pub fn property_flag(properties: &std::collections::BTreeMap<String, String>, key: &str) -> bool {
        property_str(properties, key).is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    }

    pub fn transportation_visible(class: &str, span_deg: f64, tile_z: u32, forced_lod_id: Option<&str>) -> bool {
        if !vector_detail_profile(span_deg, tile_z, forced_lod_id).draw_transportation {
            return false;
        }
        let lod_idx = super::resolve_detail_lod_index(span_deg, forced_lod_id);
        if lod_idx == 3 {
            return match class {
                "motorway" | "trunk" | "primary" => span_deg < 12.0,
                _ => false,
            };
        }
        match class {
            "motorway" | "trunk" => span_deg < 22.0,
            "primary" => span_deg < 12.0,
            "secondary" | "tertiary" => span_deg < 6.0,
            "minor" | "street" | "bus_guideway" => span_deg < 2.8,
            "residential" | "service" | "living_street" | "unclassified" | "pedestrian" => span_deg < 1.2,
            "path" | "track" | "footway" | "cycleway" | "steps" => span_deg < 0.45,
            _ => span_deg < 4.0,
        }
    }

    /// @emoji 📏 Per-LOD road stroke multiplier (city 30% of prior default).
    pub fn transportation_stroke_lod_scale(span_deg: f64, forced_lod_id: Option<&str>) -> f64 {
        match super::resolve_detail_lod_index(span_deg, forced_lod_id) {
            3 => ui_styling::strokes::MAP_ROAD_LOD_REGION,
            4 => ui_styling::strokes::MAP_ROAD_LOD_CITY,
            _ => 1.0,
        }
    }

    /// @emoji 📏 Screen stroke scale from viewport longitude span; damped in region/city bands.
    pub fn vector_line_scale(span_deg: f64) -> f64 {
        let cap = ui_styling::strokes::MAP_LINE_SCALE_CAP;
        let span = span_deg.max(0.08);
        let raw = ui_styling::strokes::MAP_LINE_SCALE_RAW / span.max(0.25);
        let country = super::GIS_MAP_LOD_MAX_SPAN_DEG[2];
        let region = super::GIS_MAP_LOD_MAX_SPAN_DEG[3];
        let city = super::GIS_MAP_LOD_MAX_SPAN_DEG[4];
        let district = super::GIS_MAP_LOD_MAX_SPAN_DEG[5];
        let street = super::GIS_MAP_LOD_MAX_SPAN_DEG[6];
        let damp = if span_deg <= street {
            let t = (span / street).sqrt();
            ui_styling::metrics::map::LINE_SCALE_DAMP_MIN + 0.1 * t
        } else if span_deg <= district {
            let u = ((span - street) / (district - street)).clamp(0.0, 1.0);
            0.5 + 0.1 * u
        } else if span_deg <= city {
            let u = ((span - district) / (city - district)).clamp(0.0, 1.0);
            0.56 + 0.1 * u
        } else if span_deg <= region {
            let u = ((span - city) / (region - city)).clamp(0.0, 1.0);
            0.66 + 0.1 * u
        } else if span_deg <= country {
            let u = ((span - region) / (country - region)).clamp(0.0, 1.0);
            0.8 + 0.08 * u
        } else {
            1.0
        };
        (raw * damp).clamp(0.5, cap)
    }

    pub fn transportation_stroke_width(class: &str, line_scale: f64) -> f64 {
        let base = match class {
            "motorway" => ui_styling::strokes::MAP_ROAD_MOTORWAY,
            "trunk" => 2.1,
            "primary" => 1.8,
            "secondary" => 1.45,
            "tertiary" => 1.2,
            "minor" | "street" => 0.95,
            "residential" | "service" | "living_street" => 0.78,
            "path" | "track" | "footway" | "cycleway" | "steps" => ui_styling::strokes::MAP_ROAD_PATH,
            _ => 0.9,
        };
        (base * line_scale).clamp(ui_styling::strokes::MAP_ROAD_CLAMP_MIN, ui_styling::strokes::MAP_ROAD_CLAMP_MAX)
    }

    pub fn boundary_visible(admin_level: u64, span_deg: f64, tile_z: u32, forced_lod_id: Option<&str>) -> bool {
        let profile = vector_detail_profile(span_deg, tile_z, forced_lod_id);
        if !profile.draw_boundary || admin_level > profile.max_admin_level {
            return false;
        }
        let lod_idx = super::resolve_detail_lod_index(span_deg, forced_lod_id);
        boundary_visible_for_lod(admin_level, lod_idx)
    }

    pub fn boundary_stroke_width(admin_level: u64, line_scale: f64) -> f64 {
        let base = match admin_level {
            2 => ui_styling::strokes::MAP_BOUNDARY_ADMIN2,
            3 | 4 => 1.3,
            5 | 6 => 1.0,
            _ => ui_styling::strokes::MAP_BOUNDARY_DEFAULT,
        };
        (base * line_scale).clamp(ui_styling::strokes::MAP_BOUNDARY_CLAMP_MIN, ui_styling::strokes::MAP_BOUNDARY_CLAMP_MAX)
    }

    pub fn coastline_stroke_width(line_scale: f64) -> f64 {
        (ui_styling::strokes::MAP_COASTLINE_MULT * line_scale).clamp(ui_styling::strokes::MAP_COASTLINE_CLAMP_MIN, ui_styling::strokes::MAP_COASTLINE_CLAMP_MAX)
    }

    pub fn place_label_visible(class: &str, span_deg: f64) -> bool {
        let caps = super::GIS_MAP_LOD_MAX_SPAN_DEG;
        match class {
            "" => span_deg <= caps[1] && span_deg > caps[2],
            "continent" => span_deg > caps[1],
            "country" => span_deg <= caps[1] && span_deg > caps[2],
            "state" | "province" => span_deg <= caps[2] && span_deg > caps[3],
            "city" => span_deg <= caps[3] && span_deg > caps[4],
            "town" => span_deg <= caps[4] && span_deg > caps[5],
            "village" | "hamlet" | "suburb" => span_deg <= caps[5] && span_deg > caps[6],
            "neighbourhood" | "quarter" | "isolated_dwelling" => span_deg <= caps[6],
            _ => false,
        }
    }

    /// @emoji 🛣️ Street-name labels use a stricter span gate than road geometry to avoid city-band overload.
    pub fn transportation_name_visible(class: &str, span_deg: f64) -> bool {
        let caps = super::GIS_MAP_LOD_MAX_SPAN_DEG;
        match class {
            "motorway" | "trunk" => span_deg < caps[1],
            "primary" => span_deg < caps[3],
            "secondary" | "tertiary" => span_deg < caps[4],
            "minor" | "street" | "bus_guideway" => span_deg < caps[5],
            "residential" | "service" | "living_street" | "unclassified" | "pedestrian" => span_deg < caps[6],
            "path" | "track" | "footway" | "cycleway" | "steps" => span_deg < caps[6] * 0.85,
            _ => span_deg < caps[4],
        }
    }

    /// @emoji 📍 POI captions only appear from district zoom inward.
    pub fn poi_label_visible(span_deg: f64) -> bool {
        span_deg <= super::GIS_MAP_LOD_MAX_SPAN_DEG[5]
    }

    const GIS_MAP_LABEL_BAND_SCREEN_PX: &[f64] = ui_styling::metrics::map::LABEL_PX_BANDS;

    fn map_lod_band_floor_span_deg(lod_idx: usize) -> f64 {
        if lod_idx == 0 {
            return 180.0;
        }
        super::GIS_MAP_LOD_MAX_SPAN_DEG.get(lod_idx.saturating_sub(1)).copied().unwrap_or(180.0).max(0.05)
    }

    /// @emoji 🔤 Label screen px scaled with viewport span inside one map LOD band.
    pub fn vector_label_px_for_lod(lod_idx: usize, span_deg: f64, weight: f64) -> f64 {
        let base = GIS_MAP_LABEL_BAND_SCREEN_PX.get(lod_idx).copied().unwrap_or(10.5);
        let span = span_deg.max(0.05);
        let floor = map_lod_band_floor_span_deg(lod_idx);
        base * floor / span * weight
    }

    /// @emoji 🔤 Resolves the active map LOD band from viewport span, then returns its zoom-scaled label px.
    pub fn vector_label_px(span_deg: f64, weight: f64) -> f64 {
        let lod_idx = super::resolve_map_lod_index_from_span(span_deg);
        vector_label_px_for_lod(lod_idx, span_deg, weight)
    }

    pub fn place_label_rank(class: &str, layer: &str) -> u16 {
        if layer == "poi" {
            return 48;
        }
        if layer == "water_name" {
            return 40;
        }
        if layer == "centroids" {
            return 8;
        }
        match class {
            "continent" => 0,
            "country" | "" => 4,
            "state" | "province" => 8,
            "city" => 12,
            "town" => 16,
            "village" | "hamlet" | "suburb" => 20,
            "neighbourhood" | "quarter" | "isolated_dwelling" => 24,
            _ => 28,
        }
    }

    pub fn transportation_name_rank(class: &str) -> u16 {
        match class {
            "motorway" => 30,
            "trunk" => 31,
            "primary" => 32,
            "secondary" => 34,
            "tertiary" => 36,
            _ => 44,
        }
    }

    pub fn color_with_alpha(color: Color, alpha: u8) -> Color {
        let rgba = color.to_rgba8();
        Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
    }

    /// @emoji 🎨 Opaque land/water base fill; weight scales RGB only so tile composites do not seam.
    pub fn weighted_opaque_fill(color: Color, weight: f64) -> Color {
        let w = super::clamp_map_layer_weight(weight).clamp(0.25, 1.0);
        let rgba = color.to_rgba8();
        let scale = |c: u8| ((f64::from(c) * w).round() as u8).min(255);
        Color::from_rgba8(scale(rgba.r), scale(rgba.g), scale(rgba.b), 255)
    }

    const TILE_BBOX_EPS: f64 = 1.0;

    pub fn mvt_point_on_tile_bbox_edge(extent: u32, x: f64, y: f64) -> bool {
        let eps = TILE_BBOX_EPS;
        let e = f64::from(extent);
        x <= eps || y <= eps || x >= e - eps || y >= e - eps
    }

    pub fn mvt_segment_is_tile_seam(extent: u32, a: (f64, f64), b: (f64, f64)) -> bool {
        if !mvt_point_on_tile_bbox_edge(extent, a.0, a.1) || !mvt_point_on_tile_bbox_edge(extent, b.0, b.1) {
            return false;
        }
        let eps = TILE_BBOX_EPS;
        let e = f64::from(extent);
        (a.1 <= eps && b.1 <= eps) || (a.1 >= e - eps && b.1 >= e - eps) || (a.0 <= eps && b.0 <= eps) || (a.0 >= e - eps && b.0 >= e - eps)
    }

    pub fn mvt_segment_touches_tile_bbox(extent: u32, a: (f64, f64), b: (f64, f64)) -> bool {
        mvt_point_on_tile_bbox_edge(extent, a.0, a.1) || mvt_point_on_tile_bbox_edge(extent, b.0, b.1)
    }

    pub fn mvt_polyline_is_tile_bbox_artifact(extent: u32, line: &[(f64, f64)]) -> bool {
        line.len() >= 2 && line.iter().all(|&(x, y)| mvt_point_on_tile_bbox_edge(extent, x, y))
    }

    fn water_class_is_open_sea(class: &str) -> bool {
        matches!(class, "ocean" | "sea" | "bay" | "strait" | "fjord" | "lagoon" | "sound" | "gulf")
    }

    /// @emoji 🌊 Keep continent zoom free of inland water while preserving adjacent LODs.
    pub fn water_polygon_visible_for_lod(lod_idx: usize, properties: &std::collections::BTreeMap<String, String>) -> bool {
        lod_idx != 1 || water_class_is_open_sea(property_class(properties))
    }

    pub fn waterway_visible_for_lod(lod_idx: usize) -> bool {
        lod_idx >= 2
    }

    /// @emoji 🗺️ Suppress country-polygon lake holes only at continent zoom.
    pub fn country_polygon_holes_visible_for_lod(lod_idx: usize) -> bool {
        lod_idx != 1
    }

    pub fn mvt_ring_is_tile_bbox_cover(extent: u32, ring: &[(f64, f64)]) -> bool {
        if ring.len() < 4 {
            return false;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for &(x, y) in ring {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        let eps = TILE_BBOX_EPS;
        let e = f64::from(extent);
        min_x <= eps && min_y <= eps && max_x >= e - eps && max_y >= e - eps
    }
}
// #endregion 🔖VectorTiles

// #region 🔖MapExtension
pub trait MapExtension: cavas::CanvasExtension {
    fn map_crs(&self) -> &str;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultMapExtension;

impl cavas::CanvasExtension for DefaultMapExtension {
    fn extension_id(&self) -> &str {
        "gis.map/default"
    }
}

impl MapExtension for DefaultMapExtension {
    fn map_crs(&self) -> &str {
        "EPSG:3857"
    }
}
// #endregion 🔖MapExtension

// #region 🔖MapContent
#[derive(Clone, Debug, serde::Deserialize)]
pub struct PositionData {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default, alias = "sourceUrl")]
    pub source_url: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct RouteData {
    pub id: String,
    pub points: Vec<[f64; 2]>,
    #[serde(default = "default_route_stroke")]
    pub stroke_width: f64,
}

fn default_route_stroke() -> f64 {
    ui_styling::strokes::MAP_ROUTE_DEFAULT
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct RegionData {
    pub id: String,
    pub ring: Vec<[f64; 2]>,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct MapDescriptorJson {
    #[serde(default)]
    pub positions: Vec<PositionData>,
    #[serde(default)]
    pub routes: Vec<RouteData>,
    #[serde(default)]
    pub regions: Vec<RegionData>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MapTileMode {
    #[default]
    Combined,
    Image,
    Vector,
}

impl MapTileMode {
    pub fn from_str(mode: &str) -> Self {
        match mode {
            "image" => Self::Image,
            "vector" => Self::Vector,
            _ => Self::Combined,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MapVectorStyle {
    #[default]
    Colored,
    FigureGround,
    InvertedFigure,
}

impl MapVectorStyle {
    pub fn from_str(style: &str) -> Self {
        match style {
            "figureGround" => Self::FigureGround,
            "invertedFigure" => Self::InvertedFigure,
            _ => Self::Colored,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Colored => "colored",
            Self::FigureGround => "figureGround",
            Self::InvertedFigure => "invertedFigure",
        }
    }
}

/// @emoji 🎚️ Layer ids that accept a weight slider at the given LOD and tile render mode.
pub fn map_layer_weight_slider_keys_at_lod(lod_id: &str, render_mode: &str) -> Vec<&'static str> {
    let lod_idx = GIS_MAP_LOD_SCALE.index_of(lod_id).unwrap_or(0);
    let span = representative_viewport_span_for_lod(lod_idx);
    let mut tile_z = GIS_MAP_LOD_TILE_Z.get(lod_idx).copied().unwrap_or(0);
    if lod_idx + 1 >= GIS_MAP_LODS.len() {
        tile_z = tile_z.max(vector_tiles::MAP_VECTOR_TILE_MAX_Z);
    }
    let profile = vector_tiles::vector_detail_profile(span, tile_z, Some(lod_id));
    let mode = MapTileMode::from_str(render_mode);
    let mut keys: Vec<&'static str> = Vec::new();
    if matches!(mode, MapTileMode::Image | MapTileMode::Combined) {
        keys.push("raster");
    }
    if matches!(mode, MapTileMode::Vector | MapTileMode::Combined) {
        if profile.draw_water || profile.draw_coastline {
            keys.push("water");
        }
        if profile.draw_landcover || profile.draw_land_backdrop {
            keys.push("land");
        }
        if profile.draw_transportation {
            keys.push("roads");
        }
        if profile.draw_buildings {
            keys.push("buildings");
        }
        if profile.draw_boundary {
            keys.push("borders");
        }
        let vector_paint = profile.draw_water || profile.draw_coastline || profile.draw_landcover || profile.draw_land_backdrop || profile.draw_transportation || profile.draw_buildings || profile.draw_boundary;
        if vector_paint {
            keys.push("labels");
        }
    }
    keys.push("positions");
    keys.push("positionLabels");
    keys.push("routes");
    keys.push("regions");
    keys
}

/// @emoji 🎚️ JSON array of layer ids with weight sliders for window options at a LOD.
pub fn gis_map_layer_weight_slider_ids_json(lod_id: &str, render_mode: &str) -> String {
    let keys = map_layer_weight_slider_keys_at_lod(lod_id, render_mode);
    serde_json::to_string(&keys).unwrap_or_else(|_| "[]".into())
}

/// OpenFreeMap OSM vector tiles (OpenMapTiles schema) track viewport zoom up to z14.
pub fn vector_tiles_available_at_camera_zoom(camera_zoom: f64) -> bool {
    camera_zoom > 0.0
}

fn map_layer_default_true() -> bool {
    true
}

/// @emoji 🎚️ Minimum layer line/label weight multiplier from window sliders.
pub const MAP_LAYER_WEIGHT_MIN: f64 = ui_styling::metrics::map::LAYER_WEIGHT_MIN;

/// @emoji 🎚️ Maximum layer line/label weight multiplier from window sliders.
pub const MAP_LAYER_WEIGHT_MAX: f64 = ui_styling::metrics::map::LAYER_WEIGHT_MAX;

fn map_layer_default_weight() -> f64 {
    1.0
}

/// @emoji 🎚️ Clamps a layer weight slider value to the supported multiplier range.
pub fn clamp_map_layer_weight(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(MAP_LAYER_WEIGHT_MIN, MAP_LAYER_WEIGHT_MAX)
    } else {
        1.0
    }
}

/// @emoji 🎚️ Per-layer line/label weight multipliers (1.0 = default cartography).
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MapLayerStrokeScale {
    #[serde(default = "map_layer_default_weight")]
    pub raster: f64,
    #[serde(default = "map_layer_default_weight")]
    pub water: f64,
    #[serde(default = "map_layer_default_weight")]
    pub land: f64,
    #[serde(default = "map_layer_default_weight")]
    pub roads: f64,
    #[serde(default = "map_layer_default_weight")]
    pub buildings: f64,
    #[serde(default = "map_layer_default_weight")]
    pub borders: f64,
    #[serde(default = "map_layer_default_weight")]
    pub labels: f64,
    #[serde(default = "map_layer_default_weight")]
    pub positions: f64,
    #[serde(default = "map_layer_default_weight")]
    pub position_labels: f64,
    #[serde(default = "map_layer_default_weight")]
    pub routes: f64,
    #[serde(default = "map_layer_default_weight")]
    pub regions: f64,
}

impl Default for MapLayerStrokeScale {
    fn default() -> Self {
        Self { raster: 1.0, water: 1.0, land: 1.0, roads: 1.0, buildings: 1.0, borders: 1.0, labels: 1.0, positions: 1.0, position_labels: 1.0, routes: 1.0, regions: 1.0 }
    }
}

impl MapLayerStrokeScale {
    pub fn sanitized(self) -> Self {
        Self {
            raster: clamp_map_layer_weight(self.raster),
            water: clamp_map_layer_weight(self.water),
            land: clamp_map_layer_weight(self.land),
            roads: clamp_map_layer_weight(self.roads),
            buildings: clamp_map_layer_weight(self.buildings),
            borders: clamp_map_layer_weight(self.borders),
            labels: clamp_map_layer_weight(self.labels),
            positions: clamp_map_layer_weight(self.positions),
            position_labels: clamp_map_layer_weight(self.position_labels),
            routes: clamp_map_layer_weight(self.routes),
            regions: clamp_map_layer_weight(self.regions),
        }
    }
}

/// @emoji 👁️ Per-layer show/hide gates for base map vector paint and user overlays.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MapLayerVisibility {
    #[serde(default = "map_layer_default_true")]
    pub raster: bool,
    #[serde(default = "map_layer_default_true")]
    pub water: bool,
    #[serde(default = "map_layer_default_true")]
    pub land: bool,
    #[serde(default = "map_layer_default_true")]
    pub roads: bool,
    #[serde(default = "map_layer_default_true")]
    pub buildings: bool,
    #[serde(default = "map_layer_default_true")]
    pub borders: bool,
    #[serde(default = "map_layer_default_true")]
    pub labels: bool,
    #[serde(default = "map_layer_default_true")]
    pub positions: bool,
    #[serde(default = "map_layer_default_true")]
    pub position_labels: bool,
    #[serde(default = "map_layer_default_true")]
    pub routes: bool,
    #[serde(default = "map_layer_default_true")]
    pub regions: bool,
}

impl Default for MapLayerVisibility {
    fn default() -> Self {
        Self { raster: true, water: true, land: true, roads: true, buildings: true, borders: true, labels: true, positions: true, position_labels: true, routes: true, regions: true }
    }
}

pub struct MapHost {
    pub camera: cavas::camera::Camera,
    pub viewport: cavas::camera::Viewport,
    pub positions: std::collections::BTreeMap<String, PositionData>,
    pub routes: std::collections::BTreeMap<String, RouteData>,
    pub regions: std::collections::BTreeMap<String, RegionData>,
    tile_images: std::collections::BTreeMap<String, std::sync::Arc<RasterImage>>,
    last_raster_visible: std::collections::BTreeSet<String>,
    vector_tiles: std::collections::BTreeMap<String, vector_tiles::VectorTile>,
    last_vector_visible: std::collections::BTreeSet<String>,
    render_mode: MapTileMode,
    vector_style: MapVectorStyle,
    forced_lod_id: Option<String>,
    layer_visibility: MapLayerVisibility,
    layer_stroke_scale: MapLayerStrokeScale,
    pub events: Vec<serde_json::Value>,
    interaction: MapInteraction,
    theme: MapThemePalette,
    selected_positions: std::collections::BTreeSet<String>,
    selected_routes: std::collections::BTreeSet<String>,
    hovered_kind: Option<String>,
    hovered_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
enum MapInteraction {
    #[default]
    None,
    Pan {
        origin: cavas::camera::Camera,
        start_screen: Point,
    },
}

fn map_point_segment_distance(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len2 = dx * dx + dy * dy;
    if len2 < 1e-12 {
        return ((px - x0).powi(2) + (py - y0).powi(2)).sqrt();
    }
    let t = ((px - x0) * dx + (py - y0) * dy) / len2;
    let t = t.clamp(0.0, 1.0);
    let qx = x0 + t * dx;
    let qy = y0 + t * dy;
    ((px - qx).powi(2) + (py - qy).powi(2)).sqrt()
}

fn map_segments_intersect_rect(x0: f64, y0: f64, x1: f64, y1: f64, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> bool {
    if (x0 >= min_x && x0 <= max_x && y0 >= min_y && y0 <= max_y) || (x1 >= min_x && x1 <= max_x && y1 >= min_y && y1 <= max_y) {
        return true;
    }
    let edges = [
        (min_x, min_y, max_x, min_y),
        (max_x, min_y, max_x, max_y),
        (max_x, max_y, min_x, max_y),
        (min_x, max_y, min_x, min_y),
    ];
    for (ax, ay, bx, by) in edges {
        if map_segments_intersect(x0, y0, x1, y1, ax, ay, bx, by) {
            return true;
        }
    }
    false
}

fn map_segments_intersect(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, dx: f64, dy: f64) -> bool {
    fn orient(px: f64, py: f64, qx: f64, qy: f64, rx: f64, ry: f64) -> f64 {
        (qy - py) * (rx - qx) - (qx - px) * (ry - qy)
    }
    fn on_segment(px: f64, py: f64, qx: f64, qy: f64, rx: f64, ry: f64) -> bool {
        rx <= px.max(qx) + 1e-9 && rx + 1e-9 >= px.min(qx) && ry <= py.max(qy) + 1e-9 && ry + 1e-9 >= py.min(qy)
    }
    let o1 = orient(ax, ay, bx, by, cx, cy);
    let o2 = orient(ax, ay, bx, by, dx, dy);
    let o3 = orient(cx, cy, dx, dy, ax, ay);
    let o4 = orient(cx, cy, dx, dy, bx, by);
    if o1 * o2 < 0.0 && o3 * o4 < 0.0 {
        return true;
    }
    if o1.abs() < 1e-9 && on_segment(ax, ay, bx, by, cx, cy) {
        return true;
    }
    if o2.abs() < 1e-9 && on_segment(ax, ay, bx, by, dx, dy) {
        return true;
    }
    if o3.abs() < 1e-9 && on_segment(cx, cy, dx, dy, ax, ay) {
        return true;
    }
    if o4.abs() < 1e-9 && on_segment(cx, cy, dx, dy, bx, by) {
        return true;
    }
    false
}

fn map_polyline_intersects_rect(points: &[Point], min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> bool {
    for p in points {
        if p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y {
            return true;
        }
    }
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        if map_segments_intersect_rect(a.x, a.y, b.x, b.y, min_x, min_y, max_x, max_y) {
            return true;
        }
    }
    false
}

fn map_point_in_polygon(px: f64, py: f64, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    if n < 3 {
        return false;
    }
    let mut j = n - 1;
    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[j];
        let intersect = (pi.y > py) != (pj.y > py) && px < (pj.x - pi.x) * (py - pi.y) / (pj.y - pi.y + 1e-12) + pi.x;
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn map_polyline_intersects_polygon(points: &[Point], polygon: &[Point]) -> bool {
    for p in points {
        if map_point_in_polygon(p.x, p.y, polygon) {
            return true;
        }
    }
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let n = polygon.len();
        for i in 0..n {
            let c = polygon[i];
            let d = polygon[(i + 1) % n];
            if map_segments_intersect(a.x, a.y, b.x, b.y, c.x, c.y, d.x, d.y) {
                return true;
            }
        }
    }
    false
}

impl Default for MapHost {
    fn default() -> Self {
        let viewport = cavas::camera::Viewport::default();
        Self {
            camera: projection::default_world_camera(&viewport),
            viewport,
            positions: std::collections::BTreeMap::new(),
            routes: std::collections::BTreeMap::new(),
            regions: std::collections::BTreeMap::new(),
            tile_images: std::collections::BTreeMap::new(),
            last_raster_visible: std::collections::BTreeSet::new(),
            vector_tiles: std::collections::BTreeMap::new(),
            last_vector_visible: std::collections::BTreeSet::new(),
            render_mode: MapTileMode::Combined,
            vector_style: MapVectorStyle::Colored,
            forced_lod_id: None,
            layer_visibility: MapLayerVisibility::default(),
            layer_stroke_scale: MapLayerStrokeScale::default(),
            events: Vec::new(),
            interaction: MapInteraction::None,
            theme: MapThemePalette::default(),
            selected_positions: std::collections::BTreeSet::new(),
            selected_routes: std::collections::BTreeSet::new(),
            hovered_kind: None,
            hovered_id: None,
        }
    }
}

// #region 🔖LabelDeclutter
struct LabelDeclutter {
    cell: f64,
    cols: usize,
    rows: usize,
    mask: Vec<bool>,
    count: usize,
    max_count: usize,
    width: f64,
    height: f64,
}

impl LabelDeclutter {
    fn for_viewport(viewport: &cavas::camera::Viewport, cell_px: f64, max_count: usize) -> Self {
        let width = viewport.width.max(1) as f64;
        let height = viewport.height.max(1) as f64;
        let cell = cell_px.max(12.0);
        let cols = ((width / cell).ceil() as usize).max(1) + 1;
        let rows = ((height / cell).ceil() as usize).max(1) + 1;
        Self { cell, cols, rows, mask: vec![false; cols * rows], count: 0, max_count: max_count.max(1), width, height }
    }

    fn estimate_box(label: &str, px: f64, origin: Point) -> (f64, f64, f64, f64) {
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let w = (label.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::MAP_WIDTH_MIN, ui_styling::metrics::label::MAP_WIDTH_MAX);
        let h = (px * 1.6 + pad * 2.0).clamp(14.0, 96.0);
        let x = origin.x;
        let y = origin.y - px * 0.85;
        (x, y, w, h)
    }

    fn try_place(&mut self, label: &str, origin: Point, px: f64) -> bool {
        if self.count >= self.max_count {
            return false;
        }
        let (x, y, w, h) = Self::estimate_box(label, px, origin);
        if x + w < 0.0 || y + h < 0.0 || x > self.width || y > self.height {
            return false;
        }
        let cx0 = (x / self.cell).floor().max(0.0) as usize;
        let cy0 = (y / self.cell).floor().max(0.0) as usize;
        let cx1 = ((x + w) / self.cell).ceil().min(self.width) as usize;
        let cy1 = ((y + h) / self.cell).ceil().min(self.height) as usize;
        let cx1 = cx1.min(self.cols.saturating_sub(1));
        let cy1 = cy1.min(self.rows.saturating_sub(1));
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                if self.mask[cy * self.cols + cx] {
                    return false;
                }
            }
        }
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                self.mask[cy * self.cols + cx] = true;
            }
        }
        self.count += 1;
        true
    }
}
// #endregion 🔖LabelDeclutter

impl MapHost {
    pub fn new() -> Self {
        Self::default()
    }

    fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
        let r = u8::try_from(arr.first()?.as_u64().unwrap_or(0).min(255)).ok()?;
        let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
        Some(Color::from_rgba8(r, g, b, a))
    }

    fn apply_theme_field(next: &mut MapThemePalette, v: &serde_json::Value, key: &str, assign: impl FnOnce(&mut MapThemePalette, Color)) {
        if let Some(arr) = v.get(key).and_then(|x| x.as_array()) {
            if let Some(c) = Self::color_from_json_rgba8(arr) {
                assign(next, c);
            }
        }
    }

    pub fn set_map_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let mut next = self.theme;
        Self::apply_theme_field(&mut next, &v, "surfaceClear", |t, c| t.surface_clear = c);
        Self::apply_theme_field(&mut next, &v, "landFill", |t, c| t.land_fill = c);
        Self::apply_theme_field(&mut next, &v, "landStroke", |t, c| {
            let rgba = c.to_rgba8();
            t.land_stroke = Color::from_rgba8(rgba.r, rgba.g, rgba.b, 0);
        });
        Self::apply_theme_field(&mut next, &v, "labelFill", |t, c| t.label_fill = c);
        Self::apply_theme_field(&mut next, &v, "labelHalo", |t, c| t.label_halo = c);
        Self::apply_theme_field(&mut next, &v, "regionFill", |t, c| t.region_fill = c);
        Self::apply_theme_field(&mut next, &v, "regionStroke", |t, c| t.region_stroke = c);
        Self::apply_theme_field(&mut next, &v, "routeStroke", |t, c| t.route_stroke = c);
        Self::apply_theme_field(&mut next, &v, "positionFill", |t, c| t.position_fill = c);
        Self::apply_theme_field(&mut next, &v, "positionStroke", |t, c| t.position_stroke = c);
        Self::apply_theme_field(&mut next, &v, "selectionStroke", |t, c| t.selection_stroke = c);
        Self::apply_theme_field(&mut next, &v, "hoverStroke", |t, c| t.hover_stroke = c);
        self.theme = next;
        Ok(())
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport.set_size(width, height, dpr);
        self.clamp_camera_to_world();
    }

    pub fn fit_world_camera(&mut self) {
        self.camera = projection::default_world_camera(&self.viewport);
    }

    pub fn clamp_camera_to_world(&mut self) {
        clamp_camera_to_world_bounds(&mut self.camera, &self.viewport);
    }

    pub fn camera_json(&self) -> String {
        serde_json::json!({
            "x": self.camera.x,
            "y": self.camera.y,
            "zoom": self.camera.zoom,
        })
        .to_string()
    }

    pub fn current_lod_json(&self) -> String {
        let lod = active_map_lod(self.forced_lod_id.as_deref(), &self.camera, &self.viewport);
        let tile_z = self.pick_raster_tile_zoom();
        let span_deg = viewport_lon_span_degrees(&self.camera, &self.viewport);
        serde_json::json!({
            "id": lod.id,
            "name": lod.name,
            "description": lod.description,
            "tileZ": tile_z,
            "spanDeg": span_deg,
            "mode": self.forced_lod_id.as_deref().unwrap_or(GIS_MAP_LOD_MODE_AUTOMATIC),
        })
        .to_string()
    }

    pub fn visible_tiles_json(&self) -> String {
        let z = self.pick_raster_tile_zoom();
        let list = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let rows: Vec<serde_json::Value> = list.iter().map(|(tz, tx, ty)| serde_json::json!({ "z": tz, "x": tx, "y": ty, "key": tiles::tile_key(*tz, *tx, *ty) })).collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    pub fn visible_vector_tiles_json(&self) -> String {
        if !vector_tiles_available_at_camera_zoom(self.camera.zoom) {
            return "[]".into();
        }
        let z = self.pick_vector_tile_zoom();
        let list = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let rows: Vec<serde_json::Value> = list.iter().map(|(tz, tx, ty)| serde_json::json!({ "z": tz, "x": tx, "y": ty, "key": tiles::tile_key(*tz, *tx, *ty) })).collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    pub fn set_render_mode(&mut self, mode: &str) {
        self.render_mode = MapTileMode::from_str(mode);
    }

    pub fn set_vector_style(&mut self, style: &str) {
        self.vector_style = MapVectorStyle::from_str(style);
    }

    pub fn set_lod_mode(&mut self, mode: &str) {
        if mode == GIS_MAP_LOD_MODE_AUTOMATIC {
            self.forced_lod_id = None;
            return;
        }
        if GIS_MAP_LOD_SCALE.index_of(mode).is_some() {
            self.forced_lod_id = Some(mode.to_string());
        }
    }

    pub fn set_layer_visibility_from_json(&mut self, json: &str) -> Result<(), String> {
        self.layer_visibility = serde_json::from_str(json).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn layer_visibility_json(&self) -> String {
        serde_json::to_string(&self.layer_visibility).unwrap_or_else(|_| "{}".into())
    }

    pub fn set_layer_stroke_scale_from_json(&mut self, json: &str) -> Result<(), String> {
        let parsed: MapLayerStrokeScale = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.layer_stroke_scale = parsed.sanitized();
        Ok(())
    }

    pub fn layer_stroke_scale_json(&self) -> String {
        serde_json::to_string(&self.layer_stroke_scale).unwrap_or_else(|_| "{}".into())
    }

    pub fn pick_raster_tile_zoom(&self) -> u32 {
        tiles::pick_zoom(&self.camera, &self.viewport, self.forced_lod_id.as_deref())
    }

    pub fn pick_vector_tile_zoom(&self) -> u32 {
        let span = viewport_lon_span_degrees(&self.camera, &self.viewport);
        let span_cap = vector_tiles::max_tile_z_for_span(span);
        let lod_idx = resolve_map_lod_index_from_span(span);
        let mut z = if lod_idx <= 1 { span_cap } else { self.pick_raster_tile_zoom().min(span_cap) };
        z = z.min(vector_tiles::MAP_VECTOR_TILE_MAX_Z);
        while z > 0 && tiles::visible_tiles(&self.camera, &self.viewport, z).len() > MAX_VISIBLE_TILE_REQUESTS {
            z -= 1;
        }
        z
    }

    pub fn render_mode_str(&self) -> &'static str {
        match self.render_mode {
            MapTileMode::Image => "image",
            MapTileMode::Vector => "vector",
            MapTileMode::Combined => "combined",
        }
    }

    pub fn vector_style_str(&self) -> &'static str {
        self.vector_style.as_str()
    }

    fn retain_tiles_for_keys(&mut self, keys: &std::collections::BTreeSet<String>) {
        self.tile_images.retain(|k, _| keys.contains(k));
        while self.tile_images.len() > MAX_MAP_TILE_CACHE_ENTRIES {
            if self.tile_images.pop_first().is_none() {
                break;
            }
        }
    }

    fn retain_vector_tiles_for_keys(&mut self, keys: &std::collections::BTreeSet<String>) {
        self.vector_tiles.retain(|k, _| keys.contains(k));
        while self.vector_tiles.len() > MAX_MAP_TILE_CACHE_ENTRIES {
            if self.vector_tiles.pop_first().is_none() {
                break;
            }
        }
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera.x = x;
        self.camera.y = y;
        self.camera.zoom = zoom;
        self.clamp_camera_to_world();
        self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        map_wheel_screen(&mut self.camera, &self.viewport, sx, sy, delta_y);
        self.clamp_camera_to_world();
        self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
    }

    pub fn upload_tile(&mut self, z: u32, x: u32, y: u32, png_bytes: &[u8]) -> Result<(), String> {
        let img = image::load_from_memory(png_bytes).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        let image = RasterImage::rgba8(w, h, Arc::new(rgba.into_raw()));
        self.tile_images.insert(tiles::tile_key(z, x, y), std::sync::Arc::new(image));
        Ok(())
    }

    pub fn upload_vector_tile(&mut self, z: u32, x: u32, y: u32, pbf_bytes: &[u8]) -> Result<(), String> {
        let tile = vector_tiles::decode_mvt(pbf_bytes)?;
        self.vector_tiles.insert(tiles::tile_key(z, x, y), tile);
        Ok(())
    }

    pub fn sync_map_json(&mut self, json: &str) -> Result<(), String> {
        let desc: MapDescriptorJson = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.positions.clear();
        self.routes.clear();
        self.regions.clear();
        for p in desc.positions {
            self.positions.insert(p.id.clone(), p);
        }
        for r in desc.routes {
            self.routes.insert(r.id.clone(), r);
        }
        for reg in desc.regions {
            self.regions.insert(reg.id.clone(), reg);
        }
        Ok(())
    }

    pub fn drain_events_json(&mut self) -> String {
        let out = std::mem::take(&mut self.events);
        serde_json::to_string(&out).unwrap_or_else(|_| "[]".into())
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
        if button != 1 {
            return;
        }
        self.interaction = MapInteraction::Pan { origin: self.camera.clone(), start_screen: Point::new(sx, sy) };
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        let MapInteraction::Pan { origin, start_screen } = self.interaction.clone() else {
            return;
        };
        let dx = (sx - start_screen.x) / self.camera.zoom;
        let dy = (sy - start_screen.y) / self.camera.zoom;
        self.camera.x = origin.x - dx;
        self.camera.y = origin.y - dy;
        self.clamp_camera_to_world();
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
        if matches!(self.interaction, MapInteraction::Pan { .. }) {
            let click = if let MapInteraction::Pan { start_screen, .. } = &self.interaction {
                let dx = sx - start_screen.x;
                let dy = sy - start_screen.y;
                (dx * dx + dy * dy).sqrt() < 6.0
            } else {
                false
            };
            if !click {
                self.push_event("camera", serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
            }
        }
        self.interaction = MapInteraction::None;
    }

    fn hit_test_position(&self, sx: f64, sy: f64) -> Option<String> {
        let hit_r = 14.0;
        let hit_r2 = hit_r * hit_r;
        let mut best: Option<(String, f64)> = None;
        for pos in self.positions.values() {
            let w = projection::lonlat_to_world(pos.lon, pos.lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            let dx = s.x - sx;
            let dy = s.y - sy;
            let d2 = dx * dx + dy * dy;
            if d2 <= hit_r2 && best.as_ref().map_or(true, |(_, best_d2)| d2 < *best_d2) {
                best = Some((pos.id.clone(), d2));
            }
        }
        best.map(|(id, _)| id)
    }

    fn hit_test_route(&self, sx: f64, sy: f64) -> Option<String> {
        let hit_r = ui_styling::strokes::MAP_ROUTE_DEFAULT * 4.0 + 6.0;
        let mut best: Option<(String, f64)> = None;
        for route in self.routes.values() {
            if route.points.len() < 2 {
                continue;
            }
            let mut min_d = f64::INFINITY;
            let mut prev: Option<Point> = None;
            for [lon, lat] in &route.points {
                let w = projection::lonlat_to_world(*lon, *lat);
                let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
                if let Some(p0) = prev {
                    min_d = min_d.min(map_point_segment_distance(sx, sy, p0.x, p0.y, s.x, s.y));
                }
                prev = Some(s);
            }
            if min_d <= hit_r && best.as_ref().map_or(true, |(_, best_d)| min_d < *best_d) {
                best = Some((route.id.clone(), min_d));
            }
        }
        best.map(|(id, _)| id)
    }

    pub fn hit_test_feature_json(&self, sx: f64, sy: f64) -> String {
        let pos_hit = self.hit_test_position(sx, sy);
        let route_hit = self.hit_test_route(sx, sy);
        match (pos_hit, route_hit) {
            (Some(pos_id), Some(route_id)) => {
                let pos_d = self.position_screen_distance(sx, sy, &pos_id).unwrap_or(f64::INFINITY);
                let route_d = self.route_screen_distance(sx, sy, &route_id).unwrap_or(f64::INFINITY);
                if pos_d <= route_d {
                    serde_json::json!({ "kind": "position", "id": pos_id }).to_string()
                } else {
                    serde_json::json!({ "kind": "route", "id": route_id }).to_string()
                }
            }
            (Some(pos_id), None) => serde_json::json!({ "kind": "position", "id": pos_id }).to_string(),
            (None, Some(route_id)) => serde_json::json!({ "kind": "route", "id": route_id }).to_string(),
            (None, None) => "null".into(),
        }
    }

    fn position_screen_distance(&self, sx: f64, sy: f64, id: &str) -> Option<f64> {
        let pos = self.positions.get(id)?;
        let w = projection::lonlat_to_world(pos.lon, pos.lat);
        let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
        Some(((s.x - sx).powi(2) + (s.y - sy).powi(2)).sqrt())
    }

    fn route_screen_distance(&self, sx: f64, sy: f64, id: &str) -> Option<f64> {
        let route = self.routes.get(id)?;
        if route.points.len() < 2 {
            return None;
        }
        let mut min_d = f64::INFINITY;
        let mut prev: Option<Point> = None;
        for [lon, lat] in &route.points {
            let w = projection::lonlat_to_world(*lon, *lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            if let Some(p0) = prev {
                min_d = min_d.min(map_point_segment_distance(sx, sy, p0.x, p0.y, s.x, s.y));
            }
            prev = Some(s);
        }
        Some(min_d)
    }

    pub fn features_in_rect_json(&self, x0: f64, y0: f64, x1: f64, y1: f64, crossing: bool) -> String {
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let mut positions: Vec<String> = Vec::new();
        let mut routes: Vec<String> = Vec::new();
        for pos in self.positions.values() {
            let w = projection::lonlat_to_world(pos.lon, pos.lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            let hit = if crossing {
                s.x >= min_x && s.x <= max_x && s.y >= min_y && s.y <= max_y
            } else {
                s.x >= min_x && s.x <= max_x && s.y >= min_y && s.y <= max_y
            };
            if hit {
                positions.push(pos.id.clone());
            }
        }
        for route in self.routes.values() {
            if route.points.len() < 2 {
                continue;
            }
            let screen_pts: Vec<Point> = route
                .points
                .iter()
                .map(|[lon, lat]| {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    map_viewport::world_to_screen(&self.camera, &self.viewport, w)
                })
                .collect();
            let hit = if crossing {
                map_polyline_intersects_rect(&screen_pts, min_x, min_y, max_x, max_y)
            } else {
                screen_pts.iter().all(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y)
            };
            if hit {
                routes.push(route.id.clone());
            }
        }
        serde_json::json!({ "positions": positions, "routes": routes }).to_string()
    }

    pub fn features_in_polygon_json(&self, points_json: &str, crossing: bool) -> String {
        let points: Vec<Point> = match serde_json::from_str::<Vec<[f64; 2]>>(points_json) {
            Ok(rows) => rows.into_iter().map(|[x, y]| Point::new(x, y)).collect(),
            Err(_) => return serde_json::json!({ "positions": [], "routes": [] }).to_string(),
        };
        if points.len() < 3 {
            return serde_json::json!({ "positions": [], "routes": [] }).to_string();
        }
        let mut positions: Vec<String> = Vec::new();
        let mut routes: Vec<String> = Vec::new();
        for pos in self.positions.values() {
            let w = projection::lonlat_to_world(pos.lon, pos.lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            let inside = map_point_in_polygon(s.x, s.y, &points);
            if crossing {
                if inside {
                    positions.push(pos.id.clone());
                }
            } else if inside {
                positions.push(pos.id.clone());
            }
        }
        for route in self.routes.values() {
            if route.points.len() < 2 {
                continue;
            }
            let screen_pts: Vec<Point> = route
                .points
                .iter()
                .map(|[lon, lat]| {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    map_viewport::world_to_screen(&self.camera, &self.viewport, w)
                })
                .collect();
            let hit = if crossing {
                map_polyline_intersects_polygon(&screen_pts, &points)
            } else {
                screen_pts.iter().all(|p| map_point_in_polygon(p.x, p.y, &points))
            };
            if hit {
                routes.push(route.id.clone());
            }
        }
        serde_json::json!({ "positions": positions, "routes": routes }).to_string()
    }

    pub fn feature_screen_json(&self, kind: &str, id: &str) -> String {
        match kind {
            "position" => self.position_screen_json(id),
            "route" => {
                let Some(route) = self.routes.get(id) else {
                    return "null".into();
                };
                if route.points.is_empty() {
                    return "null".into();
                }
                let mut cx = 0.0;
                let mut cy = 0.0;
                for [lon, lat] in &route.points {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
                    cx += s.x;
                    cy += s.y;
                }
                let n = route.points.len() as f64;
                serde_json::to_string(&serde_json::json!({ "x": cx / n, "y": cy / n })).unwrap_or_else(|_| "null".into())
            }
            _ => "null".into(),
        }
    }

    pub fn set_selection_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.selected_positions.clear();
        self.selected_routes.clear();
        if let Some(rows) = v.get("positions").and_then(|x| x.as_array()) {
            for row in rows {
                if let Some(id) = row.as_str() {
                    self.selected_positions.insert(id.to_string());
                }
            }
        }
        if let Some(rows) = v.get("routes").and_then(|x| x.as_array()) {
            for row in rows {
                if let Some(id) = row.as_str() {
                    self.selected_routes.insert(id.to_string());
                }
            }
        }
        Ok(())
    }

    pub fn set_hover_json(&mut self, json: &str) -> Result<(), String> {
        if json == "null" {
            self.hovered_kind = None;
            self.hovered_id = None;
            return Ok(());
        }
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.hovered_kind = v.get("kind").and_then(|x| x.as_str()).map(str::to_string);
        self.hovered_id = v.get("id").and_then(|x| x.as_str()).map(str::to_string);
        Ok(())
    }

    pub fn position_screen_json(&self, id: &str) -> String {
        let Some(pos) = self.positions.get(id) else {
            return "null".into();
        };
        let w = projection::lonlat_to_world(pos.lon, pos.lat);
        let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
        serde_json::to_string(&serde_json::json!({ "x": s.x, "y": s.y })).unwrap_or_else(|_| "null".into())
    }

    pub fn focus_feature(&mut self, kind: &str, id: &str) -> bool {
        let limits: serde_json::Value =
            serde_json::from_str(&gis_map_camera_limits_json_for_viewport(&self.viewport)).unwrap_or_default();
        let min_zoom = limits.get("min").and_then(|value| value.as_f64()).unwrap_or(0.05);
        let max_zoom = limits.get("max").and_then(|value| value.as_f64()).unwrap_or(64.0);
        match kind {
            "position" => {
                let Some(pos) = self.positions.get(id) else {
                    return false;
                };
                let world = projection::lonlat_to_world(pos.lon, pos.lat);
                self.camera.x = world.x;
                self.camera.y = world.y;
                self.camera.zoom = (self.camera.zoom * 1.75).clamp(min_zoom, max_zoom);
                self.clamp_camera_to_world();
                self.push_event(
                    "camera",
                    serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }),
                );
                true
            }
            "route" => {
                let Some(route) = self.routes.get(id) else {
                    return false;
                };
                if route.points.len() < 2 {
                    return false;
                }
                let mut min_x = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for [lon, lat] in &route.points {
                    let world = projection::lonlat_to_world(*lon, *lat);
                    min_x = min_x.min(world.x);
                    max_x = max_x.max(world.x);
                    min_y = min_y.min(world.y);
                    max_y = max_y.max(world.y);
                }
                let span = (max_x - min_x).max(max_y - min_y).max(64.0);
                let fit_zoom = ((self.viewport.width as f64) * 0.55 / span).clamp(min_zoom, max_zoom);
                self.camera.x = (min_x + max_x) * 0.5;
                self.camera.y = (min_y + max_y) * 0.5;
                self.camera.zoom = fit_zoom;
                self.clamp_camera_to_world();
                self.push_event(
                    "camera",
                    serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }),
                );
                true
            }
            _ => false,
        }
    }

    pub fn has_tile(&self, key: &str) -> bool {
        self.tile_images.contains_key(key)
    }

    pub fn has_vector_tile(&self, key: &str) -> bool {
        self.vector_tiles.contains_key(key)
    }

    pub fn selected_positions_json(&self) -> Vec<String> {
        self.selected_positions.iter().cloned().collect()
    }

    pub fn selected_routes_json(&self) -> Vec<String> {
        self.selected_routes.iter().cloned().collect()
    }

    pub fn hovered_kind(&self) -> Option<&str> {
        self.hovered_kind.as_deref()
    }

    pub fn hovered_id(&self) -> Option<&str> {
        self.hovered_id.as_deref()
    }

    fn push_event(&mut self, kind: &str, payload: serde_json::Value) {
        self.events.push(serde_json::json!({ "type": kind, "payload": payload }));
    }

    pub fn prepare_visible_tiles(&mut self) {
        let z = self.pick_raster_tile_zoom();
        let visible = tiles::visible_tiles(&self.camera, &self.viewport, z);
        let keys = tiles::tile_retention_keys(&visible, &self.last_raster_visible);
        self.retain_tiles_for_keys(&keys);
        self.last_raster_visible = visible.iter().map(|(tz, tx, ty)| tiles::tile_key(*tz, *tx, *ty)).collect();
        if matches!(self.render_mode, MapTileMode::Vector | MapTileMode::Combined) {
            if vector_tiles_available_at_camera_zoom(self.camera.zoom) {
                let vz = self.pick_vector_tile_zoom();
                let vvisible = tiles::visible_tiles(&self.camera, &self.viewport, vz);
                let vkeys = tiles::tile_retention_keys(&vvisible, &self.last_vector_visible);
                self.retain_vector_tiles_for_keys(&vkeys);
                self.last_vector_visible = vvisible.iter().map(|(tz, tx, ty)| tiles::tile_key(*tz, *tx, *ty)).collect();
            } else {
                self.vector_tiles.clear();
                self.last_vector_visible.clear();
            }
        }
    }

    fn tile_local_to_screen(&self, z: u32, x: u32, y: u32, extent: u32, tx: f64, ty: f64) -> Point {
        let rect = projection::tile_world_rect(z, x, y);
        let step = rect.width() / extent as f64;
        let wx = rect.x0() + tx * step;
        let wy = rect.y1() - ty * step;
        map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy))
    }

    fn tile_rect_intersects_viewport(&self, rect: Rect) -> bool {
        let w = self.viewport.width as f64;
        let h = self.viewport.height as f64;
        let corners = [
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1())),
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y1())),
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y0())),
            map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y0())),
        ];
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for p in corners {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
        min_x < w && max_x > 0.0 && min_y < h && max_y > 0.0
    }

    fn tile_raster_affine(&self, rect: Rect, img_w: u32, img_h: u32) -> Affine {
        let nw = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1()));
        let ne = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y1()));
        let sw = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y0()));
        let w = img_w.max(1) as f64;
        let h = img_h.max(1) as f64;
        Affine::new([(ne.x - nw.x) / w, (ne.y - nw.y) / w, (sw.x - nw.x) / h, (sw.y - nw.y) / h, nw.x, nw.y])
    }

    fn screen_segment_jump_limit(&self) -> f64 {
        let w = self.viewport.width as f64;
        let h = self.viewport.height as f64;
        (w * w + h * h).sqrt() * 0.45
    }

    fn tile_screen_segment_jump_limit(&self, z: u32, x: u32, y: u32) -> f64 {
        let rect = projection::tile_world_rect(z, x, y);
        let nw = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1()));
        let se = map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y0()));
        nw.distance(se) * 1.2
    }

    fn bleed_screen_quad_corners(corners: [Point; 4], bleed: f64) -> [Point; 4] {
        let cx = corners.iter().map(|p| p.x).sum::<f64>() / 4.0;
        let cy = corners.iter().map(|p| p.y).sum::<f64>() / 4.0;
        std::array::from_fn(|i| {
            let p = corners[i];
            let dx = p.x - cx;
            let dy = p.y - cy;
            let len = (dx * dx + dy * dy).sqrt();
            if len < 1e-9 {
                return p;
            }
            let scale = (len + bleed) / len;
            Point::new(cx + dx * scale, cy + dy * scale)
        })
    }

    fn append_viewport_fill(&self, scene: &mut Scene, fill: Color) {
        let w = self.viewport.width as f64;
        let h = self.viewport.height as f64;
        if w <= 0.0 || h <= 0.0 {
            return;
        }
        let bleed = 1.0;
        let corners = Self::bleed_screen_quad_corners([Point::new(0.0, 0.0), Point::new(w, 0.0), Point::new(w, h), Point::new(0.0, h)], bleed);
        let mut path = BezPath::new();
        path.move_to(corners[0]);
        for p in &corners[1..] {
            path.line_to(*p);
        }
        path.close_path();
        scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &path);
    }

    fn append_vector_tile_quad_backdrop(&self, scene: &mut Scene, z: u32, x: u32, y: u32, fill: Color) {
        let rect = projection::tile_world_rect(z, x, y);
        let corners = Self::bleed_screen_quad_corners(
            [
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y1())),
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y1())),
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x1(), rect.y0())),
                map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(rect.x0(), rect.y0())),
            ],
            2.0,
        );
        let mut path = BezPath::new();
        path.move_to(corners[0]);
        for p in &corners[1..] {
            path.line_to(*p);
        }
        path.close_path();
        scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &path);
    }

    fn append_vector_tile_land_backdrop(&self, scene: &mut Scene, z: u32, x: u32, y: u32, fill: Color) {
        self.append_vector_tile_quad_backdrop(scene, z, x, y, fill);
    }

    fn append_screen_polyline(path: &mut BezPath, pts: &[(f64, f64)], to_screen: impl Fn(f64, f64) -> Point, jump_limit: f64) {
        if pts.len() < 2 {
            return;
        }
        let mut prev: Option<Point> = None;
        for (i, &(lx, ly)) in pts.iter().enumerate() {
            let s = to_screen(lx, ly);
            let jump = prev.map(|p| p.distance(s)).unwrap_or(0.0);
            if i == 0 || jump > jump_limit {
                path.move_to(s);
            } else {
                path.line_to(s);
            }
            prev = Some(s);
        }
    }

    fn append_screen_ring(path: &mut BezPath, ring: &[(f64, f64)], to_screen: impl Fn(f64, f64) -> Point, jump_limit: f64) {
        if ring.len() < 3 {
            return;
        }
        Self::append_screen_polyline(path, ring, to_screen, jump_limit);
        path.close_path();
    }

    fn append_vector_tile_polygon(&self, scene: &mut Scene, tz: u32, tx: u32, ty: u32, extent: u32, rings: &[Vec<(f64, f64)>], fill: Color, stroke: Color, stroke_width: f64) {
        if rings.is_empty() {
            return;
        }
        let jump = self.tile_screen_segment_jump_limit(tz, tx, ty);
        let mut path = BezPath::new();
        let mut has_path = false;
        for ring in rings {
            if ring.len() < 3 {
                continue;
            }
            has_path = true;
            Self::append_screen_ring(&mut path, ring, |lx, ly| self.tile_local_to_screen(tz, tx, ty, extent, lx, ly), jump);
        }
        if !has_path {
            return;
        }
        scene.fill(FillRule::EvenOdd, Affine::IDENTITY, fill, None, &path);
        if stroke.to_rgba8().a > 5 {
            scene.stroke(&Stroke::new(stroke_width), Affine::IDENTITY, stroke, None, &path);
        }
    }

    fn append_vector_tile_polygon_rings_nonzero(&self, scene: &mut Scene, tz: u32, tx: u32, ty: u32, extent: u32, rings: &[Vec<(f64, f64)>], fill: Color) {
        self.append_vector_tile_polygon(scene, tz, tx, ty, extent, rings, fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
    }

    fn append_vector_tile_polygon_filled_rings(&self, scene: &mut Scene, tz: u32, tx: u32, ty: u32, extent: u32, rings: &[Vec<(f64, f64)>], fill: Color) {
        for ring in rings {
            self.append_vector_tile_polygon(scene, tz, tx, ty, extent, std::slice::from_ref(ring), fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
        }
    }

    fn append_vector_tile_lines(&self, scene: &mut Scene, tz: u32, tx: u32, ty: u32, extent: u32, lines: &[Vec<(f64, f64)>], stroke: Color, width: f64) {
        if lines.is_empty() || stroke.to_rgba8().a <= 5 || width <= 0.0 {
            return;
        }
        let to_screen = |lx: f64, ly: f64| self.tile_local_to_screen(tz, tx, ty, extent, lx, ly);
        for line in lines {
            if line.len() < 2 {
                continue;
            }
            if vector_tiles::mvt_polyline_is_tile_bbox_artifact(extent, line) {
                continue;
            }
            let jump = self.tile_screen_segment_jump_limit(tz, tx, ty);
            let mut path = BezPath::new();
            let mut prev: Option<Point> = None;
            for window in line.windows(2) {
                let a = (window[0].0, window[0].1);
                let b = (window[1].0, window[1].1);
                if vector_tiles::mvt_segment_is_tile_seam(extent, a, b) {
                    prev = None;
                    continue;
                }
                let sa = to_screen(a.0, a.1);
                let sb = to_screen(b.0, b.1);
                match prev {
                    None => path.move_to(sa),
                    Some(p) if p.distance(sa) > jump => path.move_to(sa),
                    _ => {}
                }
                path.line_to(sb);
                prev = Some(sb);
            }
            if prev.is_some() {
                scene.stroke(&Stroke::new(width), Affine::IDENTITY, stroke, None, &path);
            }
        }
    }

    fn append_vector_tiles(&self, scene: &mut Scene) {
        if !vector_tiles_available_at_camera_zoom(self.camera.zoom) {
            return;
        }
        let span = viewport_lon_span_degrees(&self.camera, &self.viewport);
        let forced_lod = self.forced_lod_id.as_deref();

        let mut draw: Vec<(u32, u32, u32, &vector_tiles::VectorTile)> = Vec::new();
        for (key, tile) in &self.vector_tiles {
            let Some((tz, tx, ty)) = tiles::parse_tile_key(key) else {
                continue;
            };
            let rect = projection::tile_world_rect(tz, tx, ty);
            if !self.tile_rect_intersects_viewport(rect) {
                continue;
            }
            draw.push((tz, tx, ty, tile));
        }
        if draw.is_empty() {
            return;
        }
        let render_z = draw.iter().map(|(tz, _, _, _)| *tz).max().unwrap_or(0);
        draw.retain(|(tz, _, _, _)| *tz == render_z);
        draw.sort_by_key(|(tz, tx, ty, _)| (*tz, *tx, *ty));

        match self.vector_style {
            MapVectorStyle::Colored => self.append_vector_tiles_colored(scene, &draw, render_z, span, forced_lod),
            MapVectorStyle::FigureGround => {
                let ink = self.theme.label_fill;
                let paper = self.theme.surface_clear;
                self.append_vector_tiles_figure(scene, &draw, render_z, span, forced_lod, ink, paper);
            }
            MapVectorStyle::InvertedFigure => {
                let ink = self.theme.surface_clear;
                let paper = self.theme.label_fill;
                self.append_vector_tiles_figure(scene, &draw, render_z, span, forced_lod, ink, paper);
            }
        }
        if self.layer_visibility.labels {
            let (label_fill, label_halo) = match self.vector_style {
                MapVectorStyle::Colored => (self.theme.label_fill, self.theme.label_halo),
                MapVectorStyle::FigureGround => (self.theme.label_fill, self.theme.surface_clear),
                MapVectorStyle::InvertedFigure => (self.theme.surface_clear, self.theme.label_fill),
            };
            self.append_vector_tile_labels(scene, &draw, span, forced_lod, label_fill, label_halo);
        }
    }

    fn append_vector_tile_labels(&self, scene: &mut Scene, draw: &[(u32, u32, u32, &vector_tiles::VectorTile)], span: f64, _forced_lod: Option<&str>, label_fill: Color, label_halo: Color) {
        struct LabelCandidate {
            label: String,
            screen: Point,
            rank: u16,
        }

        let px = vector_tiles::vector_label_px(span, self.layer_stroke_scale.labels);
        let mut candidates: Vec<LabelCandidate> = Vec::new();
        for (tz, tx, ty, tile) in draw {
            let mut layers: Vec<_> = tile.layers.iter().collect();
            layers.sort_by_key(|l| vector_tiles::layer_draw_rank(l.name.as_str()));
            for layer in layers {
                let extent = layer.extent.max(1);
                let lname = layer.name.as_str();
                for feat in &layer.features {
                    let (rank, visible) = match lname {
                        "transportation_name" => {
                            let class = vector_tiles::property_class(&feat.properties);
                            (vector_tiles::transportation_name_rank(class), vector_tiles::transportation_name_visible(class, span))
                        }
                        "poi" => (vector_tiles::place_label_rank("", lname), vector_tiles::poi_label_visible(span)),
                        "place" | "centroids" | "water_name" => {
                            let class = vector_tiles::property_class(&feat.properties);
                            (vector_tiles::place_label_rank(class, lname), vector_tiles::place_label_visible(class, span))
                        }
                        _ => continue,
                    };
                    if !visible {
                        continue;
                    }
                    let Some(label) = vector_tiles::feature_label(&feat.properties) else {
                        continue;
                    };
                    let anchor = feat.points.first().or_else(|| feat.rings.first().and_then(|r| r.first())).copied().unwrap_or((extent as f64 / 2.0, extent as f64 / 2.0));
                    let s = self.tile_local_to_screen(*tz, *tx, *ty, extent, anchor.0, anchor.1);
                    candidates.push(LabelCandidate { label, screen: s, rank });
                }
            }
        }
        if candidates.is_empty() {
            return;
        }
        candidates.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.label.cmp(&b.label)));
        let cell = (px * ui_styling::metrics::label::DECLUTTER_CELL_RATIO).clamp(ui_styling::metrics::label::DECLUTTER_CELL_MIN, ui_styling::metrics::label::DECLUTTER_CELL_MAX);
        let viewport_area = self.viewport.width.max(1) as f64 * self.viewport.height.max(1) as f64;
        let max_labels = (viewport_area / (cell * cell * 2.6)).round() as usize;
        let max_labels = max_labels.clamp(48, 140);
        let mut declutter = LabelDeclutter::for_viewport(&self.viewport, cell, max_labels);
        for candidate in candidates {
            if declutter.try_place(&candidate.label, candidate.screen, px) {
                cavas::text::append_label(scene, &candidate.label, candidate.screen, px, label_fill, label_halo);
            }
        }
    }

    fn append_vector_tiles_colored(&self, scene: &mut Scene, draw: &[(u32, u32, u32, &vector_tiles::VectorTile)], render_z: u32, span: f64, forced_lod: Option<&str>) {
        let weights = self.layer_stroke_scale;
        let land_fill = vector_tiles::weighted_opaque_fill(self.theme.land_fill, weights.land);
        let border_stroke = self.theme.land_stroke;
        let road_stroke = self.theme.route_stroke;
        let region_stroke = self.theme.region_stroke;
        let water_fill = vector_tiles::weighted_opaque_fill(self.theme.region_fill, weights.water);
        let building_fill = vector_tiles::color_with_alpha(land_fill, (220.0 * weights.buildings).clamp(32.0, 255.0) as u8);
        let line_scale = vector_tiles::vector_line_scale(span);
        let road_lod_scale = vector_tiles::transportation_stroke_lod_scale(span, forced_lod);

        let vis = self.layer_visibility;
        let profile = vector_tiles::vector_detail_profile(span, render_z, forced_lod);
        let lod_idx = resolve_detail_lod_index(span, forced_lod);
        let fine_land_canvas = profile.draw_land_backdrop && vis.land;
        let coarse_lod = lod_idx <= 2;
        let land_canvas = vis.land && (fine_land_canvas || coarse_lod);
        let draw_coastline = profile.draw_coastline && vis.water;
        let park_fill = if fine_land_canvas { vector_tiles::weighted_opaque_fill(self.theme.land_fill, (weights.land * 0.94).clamp(0.25, 1.0)) } else { vector_tiles::weighted_opaque_fill(self.theme.region_fill, weights.land) };
        if land_canvas {
            self.append_viewport_fill(scene, land_fill);
        }
        for (tz, tx, ty, tile) in draw {
            let tile_has_countries = tile.layers.iter().any(|l| l.name == "countries" && l.features.iter().any(|f| !f.rings.is_empty()));
            let draw_water = profile.draw_water && vis.water;
            let draw_land = profile.draw_landcover && vis.land;
            let draw_tile_countries = draw_land && !fine_land_canvas && tile_has_countries;
            let draw_landcover = draw_land && !fine_land_canvas && (!draw_tile_countries || lod_idx == 1);
            let draw_buildings = profile.draw_buildings && vis.buildings;
            let draw_roads = profile.draw_transportation && vis.roads;
            let draw_borders = profile.draw_boundary && vis.borders && !fine_land_canvas;
            let mut layers: Vec<_> = tile.layers.iter().collect();
            layers.sort_by_key(|l| vector_tiles::layer_draw_rank(l.name.as_str()));

            for layer in layers {
                let extent = layer.extent.max(1);
                let lname = layer.name.as_str();

                for feat in &layer.features {
                    match lname {
                        "water" if draw_water => {
                            if !feat.rings.is_empty() && vector_tiles::water_polygon_visible_for_lod(lod_idx, &feat.properties) {
                                self.append_vector_tile_polygon(scene, *tz, *tx, *ty, extent, &feat.rings, water_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                            }
                        }
                        "landcover" | "landuse" if draw_landcover => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon(scene, *tz, *tx, *ty, extent, &feat.rings, land_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                            }
                        }
                        "park" if draw_land => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon(scene, *tz, *tx, *ty, extent, &feat.rings, park_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                            }
                        }
                        "building" if draw_buildings => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon(scene, *tz, *tx, *ty, extent, &feat.rings, building_fill, border_stroke, 0.5 * weights.buildings);
                            }
                        }
                        "transportation" if draw_roads => {
                            let class = vector_tiles::property_class(&feat.properties);
                            if vector_tiles::transportation_visible(class, span, *tz, forced_lod) && !feat.lines.is_empty() {
                                let w = vector_tiles::transportation_stroke_width(class, line_scale) * road_lod_scale * weights.roads;
                                self.append_vector_tile_lines(scene, *tz, *tx, *ty, extent, &feat.lines, road_stroke, w);
                            }
                        }
                        "boundary" | "geolines" if draw_borders || draw_coastline => {
                            let maritime = vector_tiles::property_flag(&feat.properties, "maritime");
                            if maritime {
                                if draw_coastline && !feat.lines.is_empty() {
                                    let w = vector_tiles::coastline_stroke_width(line_scale) * weights.water;
                                    self.append_vector_tile_lines(scene, *tz, *tx, *ty, extent, &feat.lines, border_stroke, w);
                                }
                                continue;
                            }
                            if lname == "geolines" {
                                if draw_coastline && !feat.lines.is_empty() {
                                    let w = vector_tiles::coastline_stroke_width(line_scale) * weights.water;
                                    self.append_vector_tile_lines(scene, *tz, *tx, *ty, extent, &feat.lines, border_stroke, w);
                                }
                                continue;
                            }
                            let Some(admin) = vector_tiles::property_u64(&feat.properties, "admin_level") else {
                                continue;
                            };
                            if vector_tiles::boundary_visible(admin, span, *tz, forced_lod) && !feat.lines.is_empty() {
                                let w = vector_tiles::boundary_stroke_width(admin, line_scale) * weights.borders;
                                self.append_vector_tile_lines(scene, *tz, *tx, *ty, extent, &feat.lines, region_stroke, w);
                            }
                        }
                        "waterway" if draw_water && vector_tiles::waterway_visible_for_lod(lod_idx) => {
                            if !feat.lines.is_empty() {
                                self.append_vector_tile_lines(scene, *tz, *tx, *ty, extent, &feat.lines, water_fill, (1.0 * line_scale * weights.water).clamp(0.5, 6.0));
                            }
                        }
                        "countries" if draw_tile_countries => {
                            if !feat.rings.is_empty() {
                                if vector_tiles::country_polygon_holes_visible_for_lod(lod_idx) {
                                    self.append_vector_tile_polygon(scene, *tz, *tx, *ty, extent, &feat.rings, land_fill, Color::from_rgba8(0, 0, 0, 0), 0.0);
                                } else {
                                    self.append_vector_tile_polygon_filled_rings(scene, *tz, *tx, *ty, extent, &feat.rings, land_fill);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn append_vector_tiles_figure(&self, scene: &mut Scene, draw: &[(u32, u32, u32, &vector_tiles::VectorTile)], render_z: u32, span: f64, forced_lod: Option<&str>, ink: Color, paper: Color) {
        let vis = self.layer_visibility;
        let transparent_stroke = Color::from_rgba8(0, 0, 0, 0);
        let profile = vector_tiles::vector_detail_profile(span, render_z, forced_lod);
        let lod_idx = resolve_detail_lod_index(span, forced_lod);
        let draw_land_backdrop = profile.draw_land_backdrop && vis.land;
        let draw_coastline = profile.draw_coastline && vis.water;
        let draw_buildings = profile.draw_buildings && vis.buildings;
        if draw_buildings {
            self.append_viewport_fill(scene, paper);
        } else if draw_land_backdrop {
            self.append_viewport_fill(scene, ink);
        } else if draw_coastline {
            self.append_viewport_fill(scene, paper);
        } else {
            self.append_viewport_fill(scene, paper);
        }

        for (tz, tx, ty, tile) in draw {
            let draw_water = profile.draw_water && vis.water;
            let draw_land = profile.draw_landcover && vis.land;
            let draw_countries = draw_land && !draw_land_backdrop;
            let has_countries = tile.layers.iter().any(|l| l.name == "countries" && l.features.iter().any(|f| !f.rings.is_empty()));
            let use_land_mass_silhouette = draw_land_backdrop || (draw_coastline && !has_countries && draw_water && !draw_buildings);

            let mut layers: Vec<_> = tile.layers.iter().collect();
            layers.sort_by_key(|l| vector_tiles::layer_draw_rank(l.name.as_str()));

            if draw_buildings {
                for layer in &layers {
                    if layer.name.as_str() != "building" {
                        continue;
                    }
                    let extent = layer.extent.max(1);
                    for feat in &layer.features {
                        if !feat.rings.is_empty() {
                            self.append_vector_tile_polygon(scene, *tz, *tx, *ty, extent, &feat.rings, ink, transparent_stroke, 0.0);
                        }
                    }
                }
                continue;
            }

            if use_land_mass_silhouette {
                self.append_vector_tile_land_backdrop(scene, *tz, *tx, *ty, ink);
            }

            for layer in &layers {
                let extent = layer.extent.max(1);
                let lname = layer.name.as_str();
                for feat in &layer.features {
                    match lname {
                        "landcover" | "landuse" | "park" if draw_land && !use_land_mass_silhouette => {
                            if !feat.rings.is_empty() {
                                self.append_vector_tile_polygon_rings_nonzero(scene, *tz, *tx, *ty, extent, &feat.rings, ink);
                            }
                        }
                        "countries" if draw_countries => {
                            if !feat.rings.is_empty() {
                                if vector_tiles::country_polygon_holes_visible_for_lod(lod_idx) {
                                    self.append_vector_tile_polygon_rings_nonzero(scene, *tz, *tx, *ty, extent, &feat.rings, ink);
                                } else {
                                    self.append_vector_tile_polygon_filled_rings(scene, *tz, *tx, *ty, extent, &feat.rings, ink);
                                }
                            }
                        }
                        "water" if draw_water => {
                            if !feat.rings.is_empty() && vector_tiles::water_polygon_visible_for_lod(lod_idx, &feat.properties) {
                                self.append_vector_tile_polygon(scene, *tz, *tx, *ty, extent, &feat.rings, paper, transparent_stroke, 0.0);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn append_tiles(&self, scene: &mut Scene) {
        if !self.layer_visibility.raster {
            return;
        }
        let mut draw: Vec<(u32, u32, u32, std::sync::Arc<RasterImage>)> = Vec::new();
        for (key, img) in &self.tile_images {
            let Some((tz, tx, ty)) = tiles::parse_tile_key(key) else {
                continue;
            };
            let rect = projection::tile_world_rect(tz, tx, ty);
            if !self.tile_rect_intersects_viewport(rect) {
                continue;
            }
            draw.push((tz, tx, ty, img.clone()));
        }
        draw.sort_by_key(|(z, x, y, _)| (*z, *x, *y));
        for (tz, tx, ty, img) in draw {
            let rect = projection::tile_world_rect(tz, tx, ty);
            let aff = self.tile_raster_affine(rect, img.width(), img.height());
            cavas::raster::draw_image_arc(scene, &img, aff);
        }
    }

    fn append_regions(&self, scene: &mut Scene) {
        if !self.layer_visibility.regions {
            return;
        }
        let fill = self.theme.region_fill;
        let stroke = self.theme.region_stroke;
        let jump = self.screen_segment_jump_limit();
        for reg in self.regions.values() {
            if reg.ring.len() < 3 {
                continue;
            }
            let mut path = BezPath::new();
            let ring: Vec<(f64, f64)> = reg
                .ring
                .iter()
                .map(|[lon, lat]| {
                    let w = projection::lonlat_to_world(*lon, *lat);
                    (w.x, w.y)
                })
                .collect();
            Self::append_screen_ring(&mut path, &ring, |wx, wy| map_viewport::world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy)), jump);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &path);
            scene.stroke(&Stroke::new(2.0 * self.layer_stroke_scale.regions), Affine::IDENTITY, stroke, None, &path);
        }
    }

    fn append_routes(&self, scene: &mut Scene) {
        if !self.layer_visibility.routes {
            return;
        }
        let stroke_color = self.theme.route_stroke;
        let selection_color = self.theme.selection_stroke;
        let hover_color = self.theme.hover_stroke;
        for route in self.routes.values() {
            if route.points.len() < 2 {
                continue;
            }
            let selected = self.selected_routes.contains(&route.id);
            let hovered = self.hovered_kind.as_deref() == Some("route") && self.hovered_id.as_deref() == Some(route.id.as_str());
            let mut path = BezPath::new();
            for (i, [lon, lat]) in route.points.iter().enumerate() {
                let w = projection::lonlat_to_world(*lon, *lat);
                let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
                if i == 0 {
                    path.move_to(s);
                } else {
                    path.line_to(s);
                }
            }
            let width = route.stroke_width * self.layer_stroke_scale.routes;
            if selected || hovered {
                let halo = if selected { selection_color } else { hover_color };
                scene.stroke(
                    &Stroke::new(width * 2.4),
                    Affine::IDENTITY,
                    halo,
                    None,
                    &path,
                );
            }
            let color = if selected {
                selection_color
            } else if hovered {
                hover_color
            } else {
                stroke_color
            };
            scene.stroke(&Stroke::new(width), Affine::IDENTITY, color, None, &path);
        }
    }

    fn append_positions(&self, scene: &mut Scene) {
        if !self.layer_visibility.positions {
            return;
        }
        let label_fill = self.theme.label_fill;
        let label_halo = self.theme.label_halo;
        let span = viewport_lon_span_degrees(&self.camera, &self.viewport);
        let lod_idx = resolve_map_lod_index_from_span(span);
        let pos_scale = self.layer_stroke_scale.positions;
        let pos_label_px = vector_tiles::vector_label_px_for_lod(lod_idx, span, self.layer_stroke_scale.position_labels);
        for pos in self.positions.values() {
            let selected = self.selected_positions.contains(&pos.id);
            let hovered = self.hovered_kind.as_deref() == Some("position") && self.hovered_id.as_deref() == Some(pos.id.as_str());
            let fill = match pos.kind.as_deref() {
                Some("donor") => self.theme.route_stroke,
                _ => self.theme.position_fill,
            };
            let stroke = if selected {
                self.theme.selection_stroke
            } else if hovered {
                self.theme.hover_stroke
            } else {
                self.theme.position_stroke
            };
            let w = projection::lonlat_to_world(pos.lon, pos.lat);
            let s = map_viewport::world_to_screen(&self.camera, &self.viewport, w);
            let r = ui_styling::radii::MAP_POSITION_MARKER * pos_scale;
            let circle = Circle::new(s, r);
            if selected || hovered {
                let halo_r = r * 1.75;
                let halo = Circle::new(s, halo_r);
                let halo_color = if selected { self.theme.selection_stroke } else { self.theme.hover_stroke };
                scene.fill(FillRule::NonZero, Affine::IDENTITY, halo_color, None, &halo);
            }
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
            let stroke_width = if selected || hovered {
                ui_styling::strokes::MAP_POSITION_MULT * pos_scale * 1.5
            } else {
                ui_styling::strokes::MAP_POSITION_MULT * pos_scale
            };
            scene.stroke(&Stroke::new(stroke_width), Affine::IDENTITY, stroke, None, &circle);
            if self.layer_visibility.position_labels {
                let label = pos.name.as_deref().or(pos.label.as_deref()).map(str::trim).filter(|t| !t.is_empty());
                if let Some(label) = label {
                    let anchor = Point::new(s.x, s.y - r - ui_styling::radii::MAP_LABEL_ANCHOR_OFFSET);
                    cavas::text::append_label(scene, label, anchor, pos_label_px, label_fill, label_halo);
                }
            }
        }
    }

    pub fn build_vector_scene(&self) -> Scene {
        let mut scene = Scene::new();
        match self.render_mode {
            MapTileMode::Image | MapTileMode::Combined => self.append_tiles(&mut scene),
            MapTileMode::Vector => {}
        }
        match self.render_mode {
            MapTileMode::Vector | MapTileMode::Combined => self.append_vector_tiles(&mut scene),
            MapTileMode::Image => {}
        }
        self.append_regions(&mut scene);
        self.append_routes(&mut scene);
        self.append_positions(&mut scene);
        scene
    }

    /// @emoji 📐 Scales the logical viewport scene to the physical GPU surface (matches puzzle2d dpr handling).
    pub fn build_render_scene(&self) -> Scene {
        let inner = self.build_vector_scene();
        let scale = self.viewport.dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            return inner;
        }
        let mut scene = Scene::new();
        scene.append(&inner, Some(Affine::IDENTITY.scale(scale)));
        scene
    }
}

impl cavas::canvas_content::CanvasContent for MapHost {
    fn build_scene(&self) -> Scene {
        self.build_render_scene()
    }

    fn clear_color(&self) -> Color {
        self.theme.surface_clear
    }
}
// #endregion 🔖MapContent

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
struct MapSessionInner {
    host: MapHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
}

#[cfg(target_arch = "wasm32")]
impl MapSessionInner {
    fn set_logical_size(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        self.host.prepare_visible_tiles();
        let scene = self.host.build_render_scene();
        self.gpu.render_frame(&scene, cavas::canvas_content::CanvasContent::clear_color(&self.host))
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct MapSession {
    state: Rc<RefCell<MapSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl MapSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(MapSessionInner { host: MapHost::new(), gpu: cavas::gpu_session::CanvasGpuSession::default() })) }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        if inner.borrow().gpu.gpu_ready() {
            return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
        }
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let canvas = canvas.clone();
        future_to_promise(async move {
            let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|e| JsValue::from_str(&e))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.set_logical_size(lw, lh, dpr, pw, ph);
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        self.state.borrow_mut().set_logical_size(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = fitWorldCamera)]
    pub fn fit_world_camera(&mut self) {
        self.state.borrow_mut().host.fit_world_camera();
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = syncMapJson)]
    pub fn sync_map_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.sync_map_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = uploadTile)]
    pub fn upload_tile(&mut self, z: u32, x: u32, y: u32, bytes: &[u8]) -> Result<(), JsValue> {
        self.state.borrow_mut().host.upload_tile(z, x, y, bytes).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = uploadVectorTile)]
    pub fn upload_vector_tile(&mut self, z: u32, x: u32, y: u32, bytes: &[u8]) -> Result<(), JsValue> {
        self.state.borrow_mut().host.upload_vector_tile(z, x, y, bytes).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setRenderMode)]
    pub fn set_render_mode(&mut self, mode: &str) {
        self.state.borrow_mut().host.set_render_mode(mode);
    }

    #[wasm_bindgen(js_name = setVectorStyle)]
    pub fn set_vector_style(&mut self, style: &str) {
        self.state.borrow_mut().host.set_vector_style(style);
    }

    #[wasm_bindgen(js_name = vectorStyleStr)]
    pub fn vector_style_str(&self) -> String {
        self.state.borrow().host.vector_style_str().to_string()
    }

    #[wasm_bindgen(js_name = setLodMode)]
    pub fn set_lod_mode(&mut self, mode: &str) {
        self.state.borrow_mut().host.set_lod_mode(mode);
    }

    #[wasm_bindgen(js_name = setLayerVisibilityJson)]
    pub fn set_layer_visibility_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_layer_visibility_from_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = layerVisibilityJson)]
    pub fn layer_visibility_json_wasm(&self) -> String {
        self.state.borrow().host.layer_visibility_json()
    }

    #[wasm_bindgen(js_name = setLayerStrokeScaleJson)]
    pub fn set_layer_stroke_scale_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_layer_stroke_scale_from_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = layerStrokeScaleJson)]
    pub fn layer_stroke_scale_json_wasm(&self) -> String {
        self.state.borrow().host.layer_stroke_scale_json()
    }

    #[wasm_bindgen(js_name = layerWeightSliderIdsJson)]
    pub fn layer_weight_slider_ids_json(&self, lod_id: &str, render_mode: &str) -> String {
        gis_map_layer_weight_slider_ids_json(lod_id, render_mode)
    }

    #[wasm_bindgen(js_name = setMapThemeJson)]
    pub fn set_map_theme_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_map_theme_from_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = visibleVectorTilesJson)]
    pub fn visible_vector_tiles_json_wasm(&self) -> String {
        self.state.borrow().host.visible_vector_tiles_json()
    }

    #[wasm_bindgen(js_name = drainEventsJson)]
    pub fn drain_events_json(&mut self) -> String {
        self.state.borrow_mut().host.drain_events_json()
    }

    #[wasm_bindgen(js_name = positionScreenJson)]
    pub fn position_screen_json_wasm(&self, id: &str) -> String {
        self.state.borrow().host.position_screen_json(id)
    }

    #[wasm_bindgen(js_name = hitTestFeatureJson)]
    pub fn hit_test_feature_json_wasm(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.hit_test_feature_json(sx, sy)
    }

    #[wasm_bindgen(js_name = featuresInRectJson)]
    pub fn features_in_rect_json_wasm(&self, x0: f64, y0: f64, x1: f64, y1: f64, crossing: bool) -> String {
        self.state.borrow().host.features_in_rect_json(x0, y0, x1, y1, crossing)
    }

    #[wasm_bindgen(js_name = featuresInPolygonJson)]
    pub fn features_in_polygon_json_wasm(&self, points_json: &str, crossing: bool) -> String {
        self.state.borrow().host.features_in_polygon_json(points_json, crossing)
    }

    #[wasm_bindgen(js_name = featureScreenJson)]
    pub fn feature_screen_json_wasm(&self, kind: &str, id: &str) -> String {
        self.state.borrow().host.feature_screen_json(kind, id)
    }

    #[wasm_bindgen(js_name = setSelectionJson)]
    pub fn set_selection_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_selection_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setHoverJson)]
    pub fn set_hover_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_hover_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json_wasm(&self) -> String {
        self.state.borrow().host.camera_json()
    }

    #[wasm_bindgen(js_name = visibleTilesJson)]
    pub fn visible_tiles_json_wasm(&self) -> String {
        self.state.borrow().host.visible_tiles_json()
    }

    #[wasm_bindgen(js_name = currentLodJson)]
    pub fn current_lod_json_wasm(&self) -> String {
        self.state.borrow().host.current_lod_json()
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json_wasm(&self) -> String {
        gis_map_lod_scale_json()
    }

    #[wasm_bindgen(js_name = cameraLimitsJson)]
    pub fn camera_limits_json_wasm(&self) -> String {
        gis_map_camera_limits_json_for_viewport(&self.state.borrow().host.viewport)
    }

    #[wasm_bindgen(js_name = reclampCamera)]
    pub fn reclamp_camera(&mut self) {
        self.state.borrow_mut().host.clamp_camera_to_world();
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }
}
// #endregion 🔖WasmSession

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::projection::{default_world_camera, lonlat_to_world, tile_world_rect, world_to_lonlat, WORLD_HALF};
    use super::tiles::{self, visible_tiles};
    use super::MAX_VISIBLE_TILE_REQUESTS;
    use crate::cavas::camera::{Camera, Viewport};

    fn test_png_1x1() -> Vec<u8> {
        use image::codecs::png::PngEncoder;
        use image::{ColorType, ImageEncoder};
        let mut buf = Vec::new();
        let enc = PngEncoder::new(&mut buf);
        enc.write_image(&[0, 0, 0, 255], 1, 1, ColorType::Rgba8.into()).expect("png");
        buf
    }

    #[test]
    fn lonlat_world_round_trip() {
        let w = lonlat_to_world(8.5, 47.4);
        let (lon, lat) = world_to_lonlat(w.x, w.y);
        assert!((lon - 8.5).abs() < 0.01);
        assert!((lat - 47.4).abs() < 0.5);
    }

    #[test]
    fn default_world_camera_fits_extent() {
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let cam = default_world_camera(&viewport);
        assert!(cam.zoom > 0.0);
        assert_eq!(cam.x, 0.0);
        assert_eq!(cam.y, 0.0);
    }

    #[test]
    fn visible_tiles_at_world_view() {
        let camera = Camera { x: 0.0, y: 0.0, zoom: 200.0 };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let tiles = visible_tiles(&camera, &viewport, 2);
        assert!(!tiles.is_empty());
        assert!(tiles.len() < 256, "world view must not enumerate excessive tiles");
        let _ = WORLD_HALF;
    }

    #[test]
    fn map_set_size_reclamps_cover_zoom_when_viewport_grows() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        let zoom = host.camera.zoom;
        host.set_size(1024, 768, 1.0);
        assert!(host.camera.zoom >= zoom - 1e-6);
        assert_viewport_corners_inside_world(&host);
    }

    #[test]
    fn map_pan_and_zoom_stay_inside_world() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.set_camera(4.0, 4.0, super::MAP_CAMERA_ZOOM_MIN);
        assert_viewport_corners_inside_world(&host);
        host.wheel_screen(400.0, 300.0, -5000.0);
        assert_viewport_corners_inside_world(&host);
        host.pointer_down_screen(400.0, 300.0, 0);
        host.pointer_move_screen(40.0, 30.0);
        host.pointer_up_screen(40.0, 30.0);
        assert_viewport_corners_inside_world(&host);
    }

    fn assert_viewport_corners_inside_world(host: &super::MapHost) {
        let w = host.viewport.width as f64;
        let h = host.viewport.height as f64;
        for (sx, sy) in [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)] {
            let p = super::map_viewport::screen_to_world(&host.camera, &host.viewport, super::Point::new(sx, sy));
            assert!(p.x >= -super::projection::WORLD_HALF - 1e-8 && p.x <= super::projection::WORLD_HALF + 1e-8, "x out of world at ({sx},{sy}): {}", p.x);
            assert!(p.y >= -super::projection::WORLD_HALF - 1e-8 && p.y <= super::projection::WORLD_HALF + 1e-8, "y out of world at ({sx},{sy}): {}", p.y);
        }
    }

    #[test]
    fn map_camera_zoom_not_clamped_to_puzzle_scale() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        let fit_zoom = host.camera.zoom;
        assert!(fit_zoom > super::MAP_CAMERA_ZOOM_MIN);
        host.set_camera(0.0, 0.0, fit_zoom);
        assert!((host.camera.zoom - fit_zoom).abs() < 1e-6);
    }

    #[test]
    fn map_camera_max_zoom_reaches_street_longitude_span() {
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let camera = Camera { x: 0.0, y: 0.0, zoom: super::MAP_CAMERA_ZOOM_MAX };
        let span = super::viewport_lon_span_degrees(&camera, &viewport);
        assert!(span < 0.01, "max camera zoom must reach sub-city scale (span={span:.6}°)");
        assert!(span < super::GIS_MAP_LOD_MAX_SPAN_DEG[6], "max zoom should enter street LOD band (span={span:.6}°)");
    }

    #[test]
    fn map_lod_picks_bounded_tile_zoom_at_world_fit() {
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let camera = default_world_camera(&viewport);
        let z = tiles::pick_zoom(&camera, &viewport, None);
        assert_eq!(z, 0, "world-fit automatic LOD must use world-band raster tiles (got {z})");
        let tiles = visible_tiles(&camera, &viewport, z);
        assert!(tiles.len() < MAX_VISIBLE_TILE_REQUESTS);
    }

    #[test]
    fn visible_tiles_json_bounded_for_default_host() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
        assert!(!raw.is_empty());
        assert!(raw.len() < 512);
    }

    #[test]
    fn pick_vector_tile_zoom_clamped_to_openfreemap_max() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
        assert!(host.pick_vector_tile_zoom() <= super::vector_tiles::MAP_VECTOR_TILE_MAX_Z);
    }

    #[test]
    fn visible_vector_tiles_overzoom_at_max_camera_zoom() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
        assert!(super::vector_tiles_available_at_camera_zoom(super::MAP_CAMERA_ZOOM_MAX));
        let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
        let z = host.pick_vector_tile_zoom();
        assert!(z <= super::vector_tiles::MAP_VECTOR_TILE_MAX_Z);
        assert!(z <= super::vector_tiles::max_tile_z_for_span(span));
        assert!(z <= host.pick_raster_tile_zoom());
        let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
        assert!(!raw.is_empty());
    }

    #[test]
    fn map_lod_resolves_from_visible_longitude_span() {
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let camera = default_world_camera(&viewport);
        assert_eq!(super::resolve_map_lod_index_from_span(super::viewport_lon_span_degrees(&camera, &viewport)), 0, "default world fit should be world LOD");
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_camera(0.0, 0.0, 6000.0);
        let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
        let idx = super::resolve_map_lod_index_from_span(span);
        assert!(idx >= 2, "zoomed europe view should reach at least country LOD (span={span:.1}°, idx={idx})");
    }

    #[test]
    fn ideal_tile_z_rises_when_viewport_span_shrinks() {
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let world = default_world_camera(&viewport);
        let zoomed = Camera { x: 0.0, y: 0.0, zoom: 1800.0 };
        assert!(super::ideal_tile_z_for_viewport(&zoomed, &viewport) > super::ideal_tile_z_for_viewport(&world, &viewport));
    }

    #[test]
    fn region_and_city_road_strokes_stay_thin() {
        let region_scale = super::vector_tiles::vector_line_scale(8.0);
        let city_scale = super::vector_tiles::vector_line_scale(2.5);
        let district_scale = super::vector_tiles::vector_line_scale(0.2);
        assert!(region_scale <= 1.38, "region band line_scale should not exceed cap (got {region_scale})");
        assert!(city_scale <= 1.38, "city band line_scale should not exceed cap (got {city_scale})");
        let region_lod = super::vector_tiles::transportation_stroke_lod_scale(8.0, None);
        let city_lod = super::vector_tiles::transportation_stroke_lod_scale(2.5, None);
        assert!((region_lod - 0.4).abs() < f64::EPSILON);
        assert!((city_lod - 0.3).abs() < f64::EPSILON);
        let primary_region = super::vector_tiles::transportation_stroke_width("primary", region_scale) * region_lod;
        let tertiary_city = super::vector_tiles::transportation_stroke_width("tertiary", city_scale) * city_lod;
        let residential_city = super::vector_tiles::transportation_stroke_width("residential", city_scale) * city_lod;
        let minor_district = super::vector_tiles::transportation_stroke_width("minor", district_scale);
        assert!(primary_region < 1.05, "primary roads at region zoom should stay under 1.05px (got {primary_region})");
        assert!(tertiary_city < 0.55, "tertiary roads at city zoom should stay under 0.55px (got {tertiary_city})");
        assert!(residential_city < 0.4, "residential roads at city zoom should stay under 0.4px (got {residential_city})");
        assert!(minor_district < 1.35, "minor roads at district zoom should stay under 1.35px (got {minor_district})");
    }

    #[test]
    fn continental_span_hides_roads_and_caps_tile_z() {
        assert!(!super::vector_tiles::transportation_visible("motorway", 42.0, 4, None));
        assert!(super::vector_tiles::vector_detail_profile(42.0, 4, None).draw_landcover);
        assert_eq!(super::vector_tiles::max_tile_z_for_span(42.0), 5);
        let profile = super::vector_tiles::vector_detail_profile(42.0, 2, None);
        assert!(!profile.draw_boundary);
        assert!(profile.draw_coastline);
        assert!(!profile.draw_land_backdrop);
        assert!(profile.draw_landcover);
        assert!(!super::vector_tiles::boundary_visible(2, 42.0, 2, None));
        assert!(super::vector_tiles::boundary_visible(2, 20.0, 5, None));
        assert!(!super::vector_tiles::boundary_visible(4, 20.0, 5, None));
        assert!(profile.draw_water);
        assert!(!profile.draw_transportation);
    }

    #[test]
    fn country_lod_hides_regional_boundaries() {
        let span = 20.0;
        assert_eq!(super::resolve_map_lod_index_from_span(span), 2);
        let profile = super::vector_tiles::vector_detail_profile(span, 7, None);
        assert!(profile.draw_boundary);
        assert!(profile.draw_land_backdrop);
        assert!(!profile.draw_landcover);
        assert_eq!(profile.max_admin_level, 2);
        assert!(super::vector_tiles::boundary_visible(2, span, 7, None));
        assert!(!super::vector_tiles::boundary_visible(4, span, 7, None));
        assert!(!super::vector_tiles::boundary_visible(6, span, 7, None));
        let forced = super::vector_tiles::vector_detail_profile(8.0, 10, Some("country"));
        assert_eq!(forced.max_admin_level, 2);
        assert!(super::vector_tiles::boundary_visible(2, 8.0, 10, Some("country")));
        assert!(!super::vector_tiles::boundary_visible(4, 8.0, 10, Some("country")));
        assert!(!super::vector_tiles::boundary_visible(6, 8.0, 10, Some("country")));
    }

    #[test]
    fn region_lod_includes_country_and_state_boundaries() {
        let span = 8.0;
        assert_eq!(super::resolve_map_lod_index_from_span(span), 3);
        let profile = super::vector_tiles::vector_detail_profile(span, 10, None);
        assert!(profile.draw_landcover);
        assert!(profile.draw_transportation);
        assert_eq!(profile.max_admin_level, 6);
        assert!(super::vector_tiles::boundary_visible(2, span, 10, None));
        assert!(super::vector_tiles::boundary_visible(4, span, 10, None));
        assert!(super::vector_tiles::boundary_visible(6, span, 10, None));
        assert_eq!(super::vector_tiles::max_tile_z_for_span(span), 6);
        assert!(super::vector_tiles::transportation_visible("primary", span, 6, None));
        assert!(!super::vector_tiles::transportation_visible("secondary", span, 6, None));
        assert!(!super::vector_tiles::transportation_visible("tertiary", span, 6, None));
    }

    #[test]
    fn city_lod_fills_tile_backdrop_to_hide_seams() {
        let span = 2.0;
        assert_eq!(super::resolve_map_lod_index_from_span(span), 4);
        let profile = super::vector_tiles::vector_detail_profile(span, 10, None);
        assert!(profile.draw_land_backdrop);
        assert!(profile.draw_landcover);
        assert!(profile.draw_transportation);
    }

    #[test]
    fn region_lod_exceeds_country_detail() {
        let span = 8.0;
        let country = super::vector_tiles::vector_detail_profile(span, 10, Some("country"));
        let region = super::vector_tiles::vector_detail_profile(span, 10, Some("region"));
        assert!(!country.draw_landcover);
        assert!(region.draw_landcover);
        assert!(country.draw_land_backdrop);
        assert!(region.draw_land_backdrop);
        assert!(!country.draw_transportation);
        assert!(region.draw_transportation);
        assert_eq!(country.max_admin_level, 2);
        assert_eq!(region.max_admin_level, 6);
        assert!(super::vector_tiles::boundary_visible(2, span, 10, Some("region")));
        assert!(super::vector_tiles::boundary_visible(4, span, 10, Some("region")));
        assert!(!super::vector_tiles::boundary_visible(4, span, 10, Some("country")));
    }

    #[test]
    fn country_span_vector_profile_matches_lod_band() {
        let span = 20.0;
        assert_eq!(super::resolve_map_lod_index_from_span(span), 2);
        let profile = super::vector_tiles::vector_detail_profile(span, 2, None);
        assert!(profile.draw_boundary);
        assert_eq!(super::vector_tiles::max_tile_z_for_span(span), 7);
        let coarse_tile = super::vector_tiles::vector_detail_profile(span, 1, None);
        assert_eq!(profile.draw_boundary, coarse_tile.draw_boundary);
    }

    #[test]
    fn layer_visibility_json_round_trip() {
        let mut host = super::MapHost::new();
        assert!(host.layer_visibility.positions);
        host.set_layer_visibility_from_json(r#"{"positions":false,"routes":true}"#).expect("parse");
        assert!(!host.layer_visibility.positions);
        assert!(host.layer_visibility.routes);
        let parsed: super::MapLayerVisibility = serde_json::from_str(&host.layer_visibility_json()).expect("serialize");
        assert!(!parsed.positions);
    }

    #[test]
    fn weight_slider_keys_follow_lod_and_render_mode() {
        let world_combined = super::map_layer_weight_slider_keys_at_lod("world", "combined");
        assert!(world_combined.contains(&"raster"));
        assert!(world_combined.contains(&"water"));
        assert!(!world_combined.contains(&"roads"));
        assert!(!world_combined.contains(&"buildings"));
        let street_vector = super::map_layer_weight_slider_keys_at_lod("street", "vector");
        assert!(!street_vector.contains(&"raster"));
        assert!(street_vector.contains(&"roads"));
        assert!(!street_vector.contains(&"buildings"));
        let building_combined = super::map_layer_weight_slider_keys_at_lod("building", "combined");
        assert!(building_combined.contains(&"buildings"));
        assert!(building_combined.contains(&"raster"));
        let image_only = super::map_layer_weight_slider_keys_at_lod("city", "image");
        assert!(image_only.contains(&"raster"));
        assert!(!image_only.contains(&"roads"));
    }

    #[test]
    fn layer_stroke_scale_json_clamps_weights() {
        let mut host = super::MapHost::new();
        host.set_layer_stroke_scale_from_json(r#"{"roads":9,"water":0.1}"#).expect("parse");
        assert!((host.layer_stroke_scale.roads - super::MAP_LAYER_WEIGHT_MAX).abs() < f64::EPSILON);
        assert!((host.layer_stroke_scale.water - super::MAP_LAYER_WEIGHT_MIN).abs() < f64::EPSILON);
    }

    #[test]
    fn pick_vector_tile_zoom_tracks_viewport_not_camera_lod_steps() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        let z_world = host.pick_vector_tile_zoom();
        host.set_camera(0.0, 0.0, 6000.0);
        let z_zoomed = host.pick_vector_tile_zoom();
        assert!(z_zoomed > z_world, "vector tile z must rise when zooming in (world={z_world}, zoomed={z_zoomed})");
    }

    #[test]
    fn pick_vector_tile_zoom_uses_span_cap_at_world_lod_while_raster_stays_coarse() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        assert_eq!(host.pick_raster_tile_zoom(), 0, "world raster stays at z0");
        assert_eq!(host.pick_vector_tile_zoom(), super::vector_tiles::max_tile_z_for_span(super::viewport_lon_span_degrees(&host.camera, &host.viewport,)), "world vector tiles must be finer than raster so countries/coastlines paint");
    }

    #[test]
    fn forced_building_lod_bounds_tile_requests_at_world_zoom() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.set_lod_mode("building");
        let z = host.pick_raster_tile_zoom();
        assert!(z < super::GIS_MAP_LOD_TILE_Z[7], "forced building at world view must clamp tile z (got {z})");
        let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
        assert!(raw.len() <= super::MAX_VISIBLE_TILE_REQUESTS);
    }

    #[test]
    fn forced_country_lod_allows_finer_tiles_when_zoomed_in() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_lod_mode("country");
        host.set_camera(0.0, 0.0, 6000.0);
        let z = host.pick_raster_tile_zoom();
        assert!(z > super::GIS_MAP_LOD_TILE_Z[2], "pinned country LOD must not cap tile z when zoomed in (got {z})");
        let lod: serde_json::Value = serde_json::from_str(&host.current_lod_json()).expect("json");
        assert_eq!(lod["id"], "country");
        assert_eq!(lod["mode"], "country");
    }

    #[test]
    fn visible_vector_tiles_at_world_zoom() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        assert!(super::vector_tiles_available_at_camera_zoom(host.camera.zoom));
        let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
        assert!(!raw.is_empty());
    }

    #[test]
    fn map_wheel_screen_keeps_world_under_cursor_with_flipped_y() {
        use crate::cavas::camera::{Camera, Viewport};
        let mut camera = Camera { x: 0.15, y: -0.25, zoom: 320.0 };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let sx = 220.0;
        let sy = 140.0;
        let before = super::map_viewport::screen_to_world(&camera, &viewport, super::Point::new(sx, sy));
        super::map_wheel_screen(&mut camera, &viewport, sx, sy, -80.0);
        let after = super::map_viewport::screen_to_world(&camera, &viewport, super::Point::new(sx, sy));
        assert!(camera.zoom > 320.0);
        assert!((after.x - before.x).abs() < 1e-9, "world x should stay under cursor");
        assert!((after.y - before.y).abs() < 1e-9, "world y should stay under cursor");
    }

    #[test]
    fn tile_raster_affine_maps_image_north_up() {
        let mut host = super::MapHost::new();
        host.set_size(256, 256, 1.0);
        host.fit_world_camera();
        let rect = super::projection::tile_world_rect(1, 0, 0);
        let nw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y1()));
        let sw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y0()));
        assert!(nw.y < sw.y, "north edge should be above south edge on screen");
        let aff = host.tile_raster_affine(rect, 256, 256);
        let coeffs = aff.as_coeffs();
        assert!(coeffs[0] > 0.0, "east should map to increasing screen x");
        assert!(coeffs[3] > 0.0, "image rows should map downward on screen");
    }

    #[test]
    fn map_render_mode_from_str() {
        assert_eq!(super::MapTileMode::from_str("image"), super::MapTileMode::Image);
        assert_eq!(super::MapTileMode::from_str("vector"), super::MapTileMode::Vector);
        assert_eq!(super::MapTileMode::from_str("combined"), super::MapTileMode::Combined);
    }

    #[test]
    fn map_vector_style_from_str() {
        assert_eq!(super::MapVectorStyle::from_str("colored"), super::MapVectorStyle::Colored);
        assert_eq!(super::MapVectorStyle::from_str("figureGround"), super::MapVectorStyle::FigureGround);
        assert_eq!(super::MapVectorStyle::from_str("invertedFigure"), super::MapVectorStyle::InvertedFigure);
        assert_eq!(super::MapVectorStyle::from_str("unknown"), super::MapVectorStyle::Colored);
    }

    #[test]
    fn sync_map_json_keeps_position_labels() {
        let mut host = super::MapHost::new();
        let json = r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[],"regions":[]}"#;
        host.sync_map_json(json).expect("descriptor");
        let pos = host.positions.get("zurich").expect("position");
        assert_eq!(pos.label.as_deref(), Some("Zürich"));
    }

    #[test]
    fn sync_map_json_parses_rich_position_metadata() {
        let mut host = super::MapHost::new();
        let json = r#"{"positions":[{"id":"donor-1","lon":8.54,"lat":47.37,"name":"Donor site","kind":"donor","icon":"package","sourceUrl":"https://example.test/donor"}],"routes":[],"regions":[]}"#;
        host.sync_map_json(json).expect("descriptor");
        let pos = host.positions.get("donor-1").expect("position");
        assert_eq!(pos.name.as_deref(), Some("Donor site"));
        assert_eq!(pos.kind.as_deref(), Some("donor"));
        assert_eq!(pos.icon.as_deref(), Some("package"));
        assert_eq!(pos.source_url.as_deref(), Some("https://example.test/donor"));
    }

    #[test]
    fn position_screen_json_projects_known_position() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.sync_map_json(r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[],"regions":[]}"#).expect("descriptor");
        let raw: serde_json::Value = serde_json::from_str(&host.position_screen_json("zurich")).expect("json");
        assert!(raw.get("x").and_then(|v| v.as_f64()).is_some());
        assert!(raw.get("y").and_then(|v| v.as_f64()).is_some());
        assert_eq!(host.position_screen_json("missing"), "null");
    }

    #[test]
    fn hit_test_feature_prefers_closest_target() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.sync_map_json(
            r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[{"id":"route-a","points":[[8.54,47.37],[8.55,47.38]],"stroke_width":2}],"regions":[]}"#,
        )
        .expect("descriptor");
        let screen: serde_json::Value = serde_json::from_str(&host.position_screen_json("zurich")).expect("json");
        let sx = screen["x"].as_f64().expect("x");
        let sy = screen["y"].as_f64().expect("y");
        let hit: serde_json::Value = serde_json::from_str(&host.hit_test_feature_json(sx, sy)).expect("hit");
        assert!(hit.get("kind").is_some());
        assert!(hit.get("id").is_some());
    }

    #[test]
    fn features_in_rect_crossing_includes_intersecting_route() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.sync_map_json(
            r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[{"id":"route-a","points":[[8.54,47.37],[8.55,47.38]],"stroke_width":2}],"regions":[]}"#,
        )
        .expect("descriptor");
        let raw: serde_json::Value = serde_json::from_str(&host.features_in_rect_json(0.0, 0.0, 800.0, 600.0, true)).expect("json");
        assert!(raw["positions"].as_array().map(|rows| !rows.is_empty()).unwrap_or(false));
        assert!(raw["routes"].as_array().map(|rows| !rows.is_empty()).unwrap_or(false));
    }

    #[test]
    fn set_selection_and_hover_json_updates_host_state() {
        let mut host = super::MapHost::new();
        host.set_selection_json(r#"{"positions":["a"],"routes":["b"]}"#).expect("selection");
        host.set_hover_json(r#"{"kind":"position","id":"a"}"#).expect("hover");
        assert!(host.selected_positions.contains("a"));
        assert!(host.selected_routes.contains("b"));
        assert_eq!(host.hovered_kind.as_deref(), Some("position"));
        assert_eq!(host.hovered_id.as_deref(), Some("a"));
    }

    #[test]
    fn pointer_up_emits_camera_after_middle_button_pan() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.pointer_down_screen(100.0, 100.0, 1);
        host.pointer_move_screen(180.0, 140.0);
        host.pointer_up_screen(180.0, 140.0);
        let events: Vec<serde_json::Value> = serde_json::from_str(&host.drain_events_json()).expect("events");
        assert!(events.iter().any(|row| row["type"] == "camera"));
    }

    #[test]
    fn position_labels_build_non_empty_scene() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.sync_map_json(r#"{"positions":[{"id":"zurich","lon":8.54,"lat":47.37,"label":"Zürich"}],"routes":[],"regions":[]}"#).expect("descriptor");
        let scene = host.build_vector_scene();
        assert!(!scene.encoding().is_empty());
    }

    #[test]
    fn set_map_theme_from_json_updates_surface_clear() {
        let mut host = super::MapHost::new();
        let json = r#"{"surfaceClear":[1,2,3,255]}"#;
        host.set_map_theme_from_json(json).expect("theme json");
        assert_eq!(host.theme.surface_clear.to_rgba8(), super::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
    }

    #[test]
    fn set_map_theme_from_json_zeros_land_stroke_alpha() {
        let mut host = super::MapHost::new();
        host.set_map_theme_from_json(r#"{"landStroke":[51,64,65,107]}"#).expect("theme json");
        assert_eq!(host.theme.land_stroke.to_rgba8().a, 0);
    }

    #[test]
    fn build_render_scene_scales_for_device_pixel_ratio() {
        let mut host = super::MapHost::new();
        host.set_size(400, 300, 2.0);
        host.fit_world_camera();
        let logical = host.build_vector_scene();
        let scaled = host.build_render_scene();
        assert!(scaled.encoding().path_tags.len() >= logical.encoding().path_tags.len());
    }

    #[test]
    fn build_vector_scene_respects_render_mode() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_render_mode("vector");
        let _ = host.build_vector_scene();
        host.set_render_mode("image");
        let _ = host.build_vector_scene();
    }

    #[test]
    fn build_vector_scene_respects_vector_style() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_render_mode("vector");
        host.set_vector_style("figureGround");
        let _ = host.build_vector_scene();
        host.set_vector_style("invertedFigure");
        let _ = host.build_vector_scene();
        assert_eq!(host.vector_style_str(), "invertedFigure");
    }

    #[test]
    fn parse_tile_key_round_trips_tile_key() {
        let key = tiles::tile_key(3, 5, 7);
        assert_eq!(tiles::parse_tile_key(&key), Some((3, 5, 7)));
        assert_eq!(tiles::parse_tile_key("bad"), None);
        assert_eq!(tiles::parse_tile_key("1/2"), None);
    }

    #[test]
    fn tile_retention_keeps_raster_ancestor_after_zoom_in() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        let png = test_png_1x1();
        host.upload_tile(0, 0, 0, &png).expect("upload");
        assert!(host.tile_images.contains_key("0/0/0"));
        host.set_camera(0.0, 0.0, 6000.0);
        let z_fine = host.pick_raster_tile_zoom();
        assert!(z_fine >= 4, "zoomed camera should request finer raster tiles (got {z_fine})");
        host.prepare_visible_tiles();
        assert!(host.tile_images.contains_key("0/0/0"), "ancestor tile must survive zoom-level change for pyramid fallback");
    }

    #[test]
    fn tile_rect_intersects_viewport_for_on_and_off_screen_tiles() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_camera(0.0, 0.0, 3200.0);
        let z = host.pick_raster_tile_zoom();
        let visible = visible_tiles(&host.camera, &host.viewport, z);
        let (tz, tx, ty) = visible[0];
        let on_screen = tile_world_rect(tz, tx, ty);
        assert!(host.tile_rect_intersects_viewport(on_screen));
        let off_screen = super::Rect::new(4.0, 4.0, 5.0, 5.0);
        assert!(!host.tile_rect_intersects_viewport(off_screen));
    }

    #[test]
    fn tile_retention_keeps_vector_ancestor_after_zoom_in() {
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.fit_world_camera();
        host.upload_vector_tile(0, 0, 0, &[]).expect("upload");
        assert!(host.vector_tiles.contains_key("0/0/0"));
        host.set_camera(0.0, 0.0, 2500.0);
        let vz = host.pick_vector_tile_zoom();
        assert!(vz >= 1, "zoomed camera should request finer vector tiles (got {vz})");
        host.prepare_visible_tiles();
        assert!(host.vector_tiles.contains_key("0/0/0"), "ancestor vector tile must survive zoom-level change for pyramid fallback");
    }

    #[test]
    fn decode_mvt_empty_bytes_yields_empty_tile() {
        let tile = super::vector_tiles::decode_mvt(&[]).expect("empty pbf");
        assert!(tile.layers.is_empty());
    }

    #[test]
    #[ignore = "requires .repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-2-2-1.pbf from demotiles"]
    fn decode_demotile_fixture_has_named_layers() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-2-2-1.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
        assert!(!tile.layers.is_empty());
        assert!(tile.layers.iter().any(|l| !l.features.is_empty()));
        let names: Vec<&str> = tile.layers.iter().map(|l| l.name.as_str()).collect();
        assert!(names.contains(&"countries"));
        assert!(names.contains(&"centroids"));
    }

    #[test]
    fn demotile_fixture_countries_use_multi_ring_polygons() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-2-2-1.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
        let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries layer");
        let multi = countries.features.iter().filter(|f| f.rings.len() > 1).count();
        assert!(multi > 0, "fixture should include multi-ring countries");
    }

    #[test]
    fn fixture_linestrings_split_at_moveto() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES");
        let mut found = false;
        for entry in std::fs::read_dir(&dir).expect("fixture dir") {
            let path = entry.expect("entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("pbf") {
                continue;
            }
            let bytes = std::fs::read(&path).expect("read pbf");
            let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
            for layer in &tile.layers {
                if layer.features.iter().any(|f| f.lines.len() > 1) {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        assert!(found, "at least one fixture tile must decode multi-part line features");
    }

    #[test]
    fn demotile_z5_has_countries_and_centroids() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
        let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries");
        let centroids = tile.layers.iter().find(|l| l.name == "centroids").expect("centroids");
        assert!(countries.features.len() >= 10);
        assert!(centroids.features.len() >= 5);
        assert!(centroids.features.iter().any(|f| super::vector_tiles::feature_label(&f.properties).is_some()));
    }

    #[test]
    fn demotile_z0_has_many_country_features() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-0-0-0.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let tile = super::vector_tiles::decode_mvt(&bytes).expect("decode");
        let countries = tile.layers.iter().find(|l| l.name == "countries").expect("countries");
        assert!(countries.features.len() >= 50, "world tile should include many countries");
    }

    #[test]
    fn figure_world_tile_paints_many_countries() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-0-0-0.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_render_mode("vector");
        host.set_vector_style("figureGround");
        host.set_lod_mode("world");
        host.fit_world_camera();
        host.upload_vector_tile(0, 0, 0, &bytes).expect("vector tile");
        let scene = host.build_vector_scene();
        assert!(!scene.encoding().is_empty());
    }

    #[test]
    fn colored_world_tile_paints_land_over_water_backdrop() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-0-0-0.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_render_mode("vector");
        host.set_vector_style("colored");
        host.fit_world_camera();
        host.upload_vector_tile(0, 0, 0, &bytes).expect("vector tile");
        let scene = host.build_vector_scene();
        assert!(scene.encoding().path_tags.len() > 4, "colored world LOD must paint country landmasses, not only the water backdrop");
    }

    #[test]
    fn figure_country_lod_uses_land_mass_backdrop() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-3-4-2.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_render_mode("vector");
        host.set_vector_style("figureGround");
        host.set_lod_mode("country");
        host.fit_world_camera();
        host.upload_vector_tile(3, 4, 2, &bytes).expect("vector tile");
        let scene = host.build_vector_scene();
        assert!(!scene.encoding().is_empty());
    }

    fn zoom_host_to_representative_lod(host: &mut super::MapHost, lod_id: &str) {
        let idx = super::GIS_MAP_LOD_SCALE.index_of(lod_id).expect("lod id");
        let target = super::representative_viewport_span_for_lod(idx);
        host.fit_world_camera();
        let mut lo = host.camera.zoom;
        let mut hi = super::MAP_CAMERA_ZOOM_MAX;
        for _ in 0..48 {
            let mid = (lo + hi) * 0.5;
            host.set_camera(0.0, 0.0, mid);
            let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
            if span > target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        host.set_camera(0.0, 0.0, hi);
    }

    fn zoom_host_over_tile(host: &mut super::MapHost, lod_id: &str, tz: u32, tx: u32, ty: u32) {
        zoom_host_to_representative_lod(host, lod_id);
        let rect = super::projection::tile_world_rect(tz, tx, ty);
        let cx = (rect.x0() + rect.x1()) * 0.5;
        let cy = (rect.y0() + rect.y1()) * 0.5;
        host.set_camera(cx, -cy, host.camera.zoom);
    }

    #[test]
    fn transportation_name_visible_is_stricter_than_road_geometry_at_city_span() {
        let v = super::vector_tiles::transportation_name_visible;
        assert!(v("primary", 2.0));
        assert!(!v("secondary", 2.0));
        assert!(!v("street", 2.0));
        assert!(v("secondary", 0.8));
    }

    #[test]
    fn vector_label_px_scales_with_span_inside_city_band() {
        let wide = super::vector_tiles::vector_label_px(2.0, 1.0);
        let narrow = super::vector_tiles::vector_label_px(1.3, 1.0);
        assert!(narrow > wide, "labels should grow when zooming in (span shrinks): {wide} vs {narrow}");
        assert!((wide - 12.5 * 4.0 / 2.0).abs() < 1e-6);
    }

    #[test]
    fn poi_labels_hidden_until_district_span() {
        let v = super::vector_tiles::poi_label_visible;
        assert!(!v(2.0));
        assert!(v(0.3));
    }

    #[test]
    fn place_label_visible_covers_admin_document() {
        let v = super::vector_tiles::place_label_visible;
        assert!(v("", 20.0));
        assert!(!v("", 8.0));
        assert!(v("continent", 50.0));
        assert!(!v("continent", 20.0));
        assert!(v("country", 20.0));
        assert!(!v("country", 8.0));
        assert!(v("state", 8.0));
        assert!(v("province", 8.0));
        assert!(v("city", 2.0));
        assert!(v("town", 0.8));
        assert!(v("village", 0.2));
        assert!(v("suburb", 0.2));
        assert!(v("quarter", 0.05));
        assert!(v("neighbourhood", 0.05));
    }

    #[test]
    fn label_camera_setup_intersects_fixture_tile() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_lod_mode("country");
        zoom_host_over_tile(&mut host, "country", 5, 17, 11);
        host.upload_vector_tile(5, 17, 11, &bytes).expect("vector tile");
        let rect = super::projection::tile_world_rect(5, 17, 11);
        let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
        assert!(host.tile_rect_intersects_viewport(rect), "tile must intersect (span={span})");
        let scene = host.build_vector_scene();
        assert!(!scene.encoding().is_empty(), "fixture tile should paint geometry (span={span})");
    }

    #[test]
    fn figure_ground_labels_increase_scene_when_enabled() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_render_mode("vector");
        host.set_vector_style("figureGround");
        host.set_lod_mode("country");
        zoom_host_over_tile(&mut host, "country", 5, 17, 11);
        host.upload_vector_tile(5, 17, 11, &bytes).expect("vector tile");
        host.set_layer_visibility_from_json(r#"{"labels":false}"#).expect("labels off");
        let without = host.build_vector_scene().encoding().path_tags.len();
        host.set_layer_visibility_from_json(r#"{"labels":true}"#).expect("labels on");
        let with_labels = host.build_vector_scene().encoding().path_tags.len();
        assert!(with_labels > without, "figure-ground labels should add glyph paths (with={with_labels}, without={without})");
    }

    #[test]
    fn colored_vector_labels_increase_scene_when_enabled() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.repo/🎫/26/06/03/MAP-VECTOR-TILES/sample-5-17-11.pbf");
        let bytes = std::fs::read(path).expect("fixture pbf");
        let mut host = super::MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_render_mode("vector");
        host.set_vector_style("colored");
        host.set_lod_mode("country");
        zoom_host_over_tile(&mut host, "country", 5, 17, 11);
        host.upload_vector_tile(5, 17, 11, &bytes).expect("vector tile");
        host.set_layer_visibility_from_json(r#"{"labels":false}"#).expect("labels off");
        let without = host.build_vector_scene().encoding().path_tags.len();
        host.set_layer_visibility_from_json(r#"{"labels":true}"#).expect("labels on");
        let with_labels = host.build_vector_scene().encoding().path_tags.len();
        assert!(with_labels > without, "colored vector labels should add glyph paths (with={with_labels}, without={without})");
    }
}
// #endregion 🔖Tests

// #region 🔖DocumentVcs
use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const GIS_MAP_SCHEMA: &str = "gis.map";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapDocument {
    pub layers: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapDiff {
    pub layers: Option<Vec<serde_json::Value>>,
}

impl OperationDiff<GisMapDocument> for GisMapDiff {
    fn apply(&self, projection: &GisMapDocument) -> GisMapDocument {
        GisMapDocument {
            layers: self.layers.clone().unwrap_or_else(|| projection.layers.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.layers.is_some() {
            self.layers = other.layers;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum GisMapOp {
    SetLayers { layers: Vec<serde_json::Value> },
}

impl Operation<GisMapDocument> for GisMapOp {
    type Diff = GisMapDiff;

    fn diff(&self, _projection: &GisMapDocument) -> GisMapDiff {
        match self {
            GisMapOp::SetLayers { layers } => GisMapDiff {
                layers: Some(layers.clone()),
            },
        }
    }

    fn backwards(&self, projection: &GisMapDocument) -> Vec<Self> {
        vec![GisMapOp::SetLayers {
            layers: projection.layers.clone(),
        }]
    }
}

pub type GisMapEnvelope = DocumentVcsEnvelope<GisMapDocument, GisMapOp>;
pub type GisMapStore = DocumentVcsStore<GisMapDocument, GisMapOp>;

pub fn empty_gis_map_projection() -> GisMapDocument {
    GisMapDocument { layers: Vec::new() }
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct GisMapDocumentVcs {
        store: RefCell<GisMapStore>,
    }

    #[wasm_bindgen]
    impl GisMapDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<GisMapDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: GisMapEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    GisMapStore::new(envelope)
                }
                None => GisMapStore::new(create_document_vcs_envelope(
                    GIS_MAP_SCHEMA,
                    "gis",
                    empty_gis_map_projection(),
                    None,
                )),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

#[cfg(test)]
mod gis_map_vcs_tests {
    use super::*;

    #[test]
    fn gis_map_document_vcs_replays_ops() {
        let mut store = GisMapStore::new(create_document_vcs_envelope(
            GIS_MAP_SCHEMA,
            "gis",
            empty_gis_map_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![GisMapOp::SetLayers {
                    layers: vec![serde_json::json!({ "id": "base" })],
                }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").layers.len(), 1);
    }
}
// #endregion 🔖DocumentVcs

// #region 🔖OpenUrl
/** @emoji 🌐 Opens a URL in the system browser when available. */
pub fn open_url(url: &str) -> bool {
    if url.trim().is_empty() {
        return false;
    }
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsValue;
        return web_sys::window()
            .and_then(|window| window.open_with_url(url).ok())
            .flatten()
            .is_some();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = url;
        eprintln!("[DEBUG] open_url native fallback: {url}");
        false
    }
}
// #endregion 🔖OpenUrl
