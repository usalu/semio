
use super::MapPolicy;
use super::MAX_VISIBLE_TILE_REQUESTS;
use super::canvas::camera::{Camera, Viewport};
use super::projection::{WORLD_HALF, default_world_camera, lonlat_to_world, tile_world_rect, world_to_lonlat};
use super::tiles::{self, visible_tiles};

fn test_png_1x1() -> Vec<u8> {
    use image::codecs::png::PngEncoder;
    use image::{ColorType, ImageEncoder};
    let mut buf = Vec::new();
    let enc = PngEncoder::new(&mut buf);
    enc.write_image(&[0, 0, 0, 255], 1, 1, ColorType::Rgba8.into()).expect("png");
    buf
}

/// 🧪️ A one-feature "landcover" vector tile — painted by [`land_cover_extension`], so it exercises the
/// real `tile_local_to_screen` → `world_to_screen` path the cache must translate.
fn synthetic_land_tile() -> super::vector_tiles::VectorTile {
    use super::vector_tiles::{GeomType, VectorFeature, VectorLayer, VectorTile};
    VectorTile {
        layers: vec![VectorLayer {
            name: "landcover".to_string(),
            extent: super::vector_tiles::DEFAULT_MVT_EXTENT,
            features: vec![VectorFeature { id: None, geom_type: GeomType::Polygon, rings: vec![vec![(500.0, 500.0), (3500.0, 500.0), (3500.0, 3500.0), (500.0, 3500.0)]], lines: vec![], points: vec![], properties: std::collections::BTreeMap::new() }],
        }],
    }
}

/// 🌱️ The neutral policy plus one source-layer role (`landcover` paints as land cover) and two style presets: `plain` (colored) and `silhouette` (figure-ground).
fn land_cover_extension() -> super::Arc<dyn super::MapExtension> {
    use super::{MapPaintTechnique, MapPaletteSlot, MapStylePreset};
    let layers = vec![super::MapSourceLayer { name: "landcover".into(), role: Some(super::MapLayerRole::LandCover), draw_rank: 0, label_rank: None }];
    let preset = |id: &str, technique| MapStylePreset { id: id.into(), technique, ink: MapPaletteSlot::LabelFill, paper: MapPaletteSlot::SurfaceClear, label_fill: MapPaletteSlot::LabelFill, label_halo: MapPaletteSlot::LabelHalo };
    let style_presets = vec![preset("plain", MapPaintTechnique::Colored), preset("silhouette", MapPaintTechnique::FigureGround)];
    super::Arc::new(super::DefaultMapExtension::with_policy(super::DescribedMapPolicy::new(super::MapPolicyDescriptor { layers, style_presets, ..super::MapPolicyDescriptor::neutral() })))
}

/// 📍️ A test policy with three overlay layers drawn in this order — `zones` (areas), `paths` (pickable lines, granularity `path`) and `pins` (pickable points, granularity `pin`, `accent` style filled from another slot) — each behind its own toggle, plus a `pinLabels` toggle for the pin captions.
fn overlay_extension() -> super::Arc<dyn super::MapExtension> {
    use super::{MapLayerTarget, MapLayerToggle, MapOverlayGeometry, MapOverlayLayer, MapOverlayStyleFill, MapPaletteSlot};
    let layer = |id: &str, geometry: MapOverlayGeometry, granularity: Option<&str>| MapOverlayLayer {
        id: id.into(),
        geometry,
        granularity: granularity.map(str::to_string),
        hit_radius_px: 14.0,
        line_width: 2.0,
        point_radius: 8.0,
        point_stroke: 2.0,
        label_offset_px: 6.0,
        fill: MapPaletteSlot::PositionFill,
        stroke: MapPaletteSlot::PositionStroke,
        style_fills: vec![MapOverlayStyleFill { style: "accent".into(), fill: MapPaletteSlot::RouteStroke }],
    };
    let toggle = |key: &str, target: MapLayerTarget| MapLayerToggle { key: key.into(), targets: vec![target] };
    let descriptor = super::MapPolicyDescriptor {
        overlays: vec![layer("zones", MapOverlayGeometry::Area, None), layer("paths", MapOverlayGeometry::Line, Some("path")), layer("pins", MapOverlayGeometry::Point, Some("pin"))],
        layer_toggles: vec![toggle("zones", MapLayerTarget::Overlay("zones".into())), toggle("paths", MapLayerTarget::Overlay("paths".into())), toggle("pins", MapLayerTarget::Overlay("pins".into())), toggle("pinLabels", MapLayerTarget::OverlayLabels("pins".into()))],
        ..super::MapPolicyDescriptor::neutral()
    };
    super::Arc::new(super::DefaultMapExtension::with_policy(super::DescribedMapPolicy::new(descriptor)))
}

fn overlay_host() -> super::MapHost {
    let mut host = super::MapHost::with_extension(overlay_extension());
    host.set_size(800, 600, 1.0);
    host
}

const ZURICH_PIN: &str = r#"{"overlays":{"pins":[{"id":"zurich","coordinates":[[8.54,47.37]],"label":"Zürich"}]}}"#;
const ZURICH_PIN_AND_PATH: &str = r#"{"overlays":{"pins":[{"id":"zurich","coordinates":[[8.54,47.37]],"label":"Zürich"}],"paths":[{"id":"path-a","coordinates":[[8.54,47.37],[8.55,47.38]],"width":2.0}]}}"#;

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
        assert!(p.x >= -WORLD_HALF - 1e-8 && p.x <= WORLD_HALF + 1e-8, "x out of world at ({sx},{sy}): {}", p.x);
        assert!(p.y >= -WORLD_HALF - 1e-8 && p.y <= WORLD_HALF + 1e-8, "y out of world at ({sx},{sy}): {}", p.y);
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
fn visible_tiles_json_bounded_for_default_host() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    assert!(!raw.is_empty());
    assert!(raw.len() < 512);
}

#[test]
fn pick_vector_tile_zoom_clamped_to_the_policy_vector_pyramid() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
    assert!(host.pick_vector_tile_zoom() <= host.policy().descriptor().vector_tile_z_max);
}

#[test]
fn visible_vector_tiles_overzoom_at_max_camera_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
    assert!(super::vector_tiles_available_at_camera_zoom(super::MAP_CAMERA_ZOOM_MAX));
    let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
    let z = host.pick_vector_tile_zoom();
    assert!(z <= host.policy().descriptor().vector_tile_z_max);
    assert!(z <= host.policy().vector_zoom_cap(span));
    assert!(z <= host.pick_raster_tile_zoom());
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
    assert!(!raw.is_empty());
}

#[test]
fn layer_visibility_json_round_trip() {
    let mut host = overlay_host();
    let pins = super::MapLayerTarget::Overlay("pins".into());
    assert!(host.layer_visible(&pins));
    host.set_layer_visibility_from_json(r#"{"pins":false,"paths":true}"#).expect("parse");
    assert!(!host.layer_visible(&pins));
    assert!(host.layer_visible(&super::MapLayerTarget::Overlay("paths".into())));
    let parsed: std::collections::BTreeMap<String, bool> = serde_json::from_str(&host.layer_visibility_json()).expect("serialize");
    assert_eq!(parsed.get("pins"), Some(&false));
}

#[test]
fn layer_stroke_scale_json_clamps_weights() {
    let mut host = overlay_host();
    host.set_layer_stroke_scale_from_json(r#"{"pins":9,"paths":0.1}"#).expect("parse");
    assert!((host.layer_weight(&super::MapLayerTarget::Overlay("pins".into())) - super::MAP_LAYER_WEIGHT_MAX).abs() < f64::EPSILON);
    assert!((host.layer_weight(&super::MapLayerTarget::Overlay("paths".into())) - super::MAP_LAYER_WEIGHT_MIN).abs() < f64::EPSILON);
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
fn visible_vector_tiles_at_world_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    assert!(super::vector_tiles_available_at_camera_zoom(host.camera.zoom));
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
    assert!(!raw.is_empty());
}

#[test]
fn tile_raster_affine_maps_image_north_up() {
    let mut host = super::MapHost::new();
    host.set_size(256, 256, 1.0);
    host.fit_world_camera();
    let rect = tile_world_rect(1, 0, 0);
    let nw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y1()));
    let sw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y0()));
    assert!(nw.y < sw.y, "north edge should be above south edge on screen");
    let aff = host.tile_raster_affine(rect, 256, 256);
    let coeffs = aff.as_coeffs();
    assert!(coeffs[0] > 0.0, "east should map to increasing screen x");
    assert!(coeffs[3] > 0.0, "image rows should map downward on screen");
}

#[test]
fn sync_map_json_keeps_overlay_captions() {
    let mut host = overlay_host();
    host.sync_map_json(ZURICH_PIN).expect("descriptor");
    assert_eq!(host.overlay_feature("pins", "zurich").and_then(|feature| feature.label.as_deref()), Some("Zürich"));
}

#[test]
fn sync_map_json_parses_rich_overlay_metadata() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"pins":[{"id":"site-1","coordinates":[[8.54,47.37]],"label":"Site","style":"accent","icon":"package","link":"https://example.test/site"}]}}"#).expect("descriptor");
    let feature = host.overlay_feature("pins", "site-1").expect("feature");
    assert_eq!(feature.style.as_deref(), Some("accent"));
    assert_eq!(feature.icon.as_deref(), Some("package"));
    assert_eq!(feature.link.as_deref(), Some("https://example.test/site"));
    host.sync_map_json(r#"{"overlays":{}}"#).expect("empty descriptor");
    assert!(host.overlay_feature("pins", "site-1").is_none(), "a sync replaces the overlay tables wholesale");
}

#[test]
fn feature_screen_json_projects_a_known_point() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN).expect("descriptor");
    let raw: serde_json::Value = serde_json::from_str(&host.feature_screen_json("pin", "zurich")).expect("json");
    assert!(raw.get("x").and_then(|v| v.as_f64()).is_some());
    assert!(raw.get("y").and_then(|v| v.as_f64()).is_some());
    assert_eq!(host.feature_screen_json("pin", "missing"), "null");
}

#[test]
fn hit_test_feature_prefers_the_topmost_closest_target() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    let screen: serde_json::Value = serde_json::from_str(&host.feature_screen_json("pin", "zurich")).expect("json");
    let sx = screen["x"].as_f64().expect("x");
    let sy = screen["y"].as_f64().expect("y");
    let hit: serde_json::Value = serde_json::from_str(&host.hit_test_feature_json(sx, sy)).expect("hit");
    assert_eq!(hit["kind"], "pin", "the path starts under the pin too; the later-declared layer wins the tie");
    assert_eq!(hit["id"], "zurich");
    assert_eq!(host.hit_test_feature_json(-1_000.0, -1_000.0), "null");
}

#[test]
fn features_in_rect_crossing_includes_intersecting_lines() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    let raw: serde_json::Value = serde_json::from_str(&host.features_in_rect_json(0.0, 0.0, 800.0, 600.0, true)).expect("json");
    assert!(raw["pins"].as_array().is_some_and(|rows| !rows.is_empty()));
    assert!(raw["paths"].as_array().is_some_and(|rows| !rows.is_empty()));
    assert!(raw.get("zones").is_none(), "a layer without a granularity is not pickable");
}

#[test]
fn sync_interaction_updates_host_state_scoped_to_active_granularity() {
    let mut host = overlay_host();
    host.sync_interaction("pin", &["a".to_string()], Some("a"));
    assert_eq!(host.selected_ids("pin").collect::<Vec<_>>(), ["a"]);
    assert_eq!(host.selected_ids("path").count(), 0);
    assert_eq!(host.hovered_kind(), Some("pin"));
    assert_eq!(host.hovered_id(), Some("a"));

    host.sync_interaction("path", &["b".to_string()], None);
    assert_eq!(host.selected_ids("pin").count(), 0, "switching granularity clears the other kind's ids");
    assert_eq!(host.selected_ids("path").collect::<Vec<_>>(), ["b"]);
    assert!(host.hovered_kind().is_none());
    assert!(host.hovered_id().is_none());
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
fn overlay_layers_paint_behind_their_toggles() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    let all = host.build_vector_scene().path_count();
    assert!(all > 0);
    host.set_layer_visibility_from_json(r#"{"pinLabels":false}"#).expect("captions off");
    let without_captions = host.build_vector_scene().path_count();
    assert!(without_captions < all, "the pin caption is its own toggle target");
    host.set_layer_visibility_from_json(r#"{"pins":false,"paths":false}"#).expect("overlays off");
    assert_eq!(host.build_vector_scene().path_count(), 0);
    let mut neutral = super::MapHost::new();
    neutral.set_size(800, 600, 1.0);
    neutral.fit_world_camera();
    neutral.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    assert_eq!(neutral.build_vector_scene().path_count(), 0, "the neutral policy declares no overlay layer");
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
    assert!(scaled.path_count() >= logical.path_count());
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
fn vector_style_resolves_to_a_policy_preset() {
    let mut host = super::MapHost::with_extension(land_cover_extension());
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("silhouette");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_style_str(), "silhouette");
    host.set_vector_style("unknown");
    assert_eq!(host.vector_style_str(), "plain", "an undeclared preset paints the first one");
    assert_eq!(super::MapHost::new().vector_style_str(), super::MAP_STYLE_PRESET_NEUTRAL_ID);
}

/// 🎣️ Seeds a vector host whose camera sits *off* a tile boundary. Straight from
/// `fit_world_camera` the viewport edge lands exactly on one, where `visible_tile_cursor`'s
/// `ceil - 1` correctly excludes the zero-overlap tile — so any nudge in either direction pulls a
/// whole extra row in (measured live: 48 → 56 tiles for a 0.2 px pan) and legitimately invalidates
/// the cache. A sub-tile pan only stays within one visible set away from that boundary.
fn vector_cache_host_off_tile_boundary() -> super::MapHost {
    let mut host = super::MapHost::with_extension(land_cover_extension());
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    host.camera.y -= 0.017;
    host
}

#[test]
fn vector_scene_cache_hit_reuses_translated_scene_on_pure_pan() {
    let mut host = vector_cache_host_off_tile_boundary();
    let first = host.build_vector_scene();
    assert!(!first.is_empty());
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    let anchor_x = host.camera.x;
    let anchor_y = host.camera.y;
    host.camera.x = anchor_x;
    host.camera.y = anchor_y - 0.002;
    let panned = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "a sub-tile pan must reuse the cache, not rebuild");
    assert_eq!(panned.path_count(), first.path_count());

    let mut rebuilt = vector_cache_host_off_tile_boundary();
    rebuilt.camera.x = anchor_x;
    rebuilt.camera.y = anchor_y - 0.002;
    let forced = rebuilt.build_vector_scene();
    assert_eq!(rebuilt.vector_scene_rebuild_count.get(), 1);
    assert_eq!(panned.path_count(), forced.path_count(), "cached-translate pan must match a forced rebuild at the same camera");
}

#[test]
fn vector_scene_cache_rebuilds_when_a_pan_changes_the_visible_tile_set() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    let before = host.visible_vector_tiles_revision();
    host.camera.y += 0.017;
    assert_ne!(host.visible_vector_tiles_revision(), before, "this pan must cross a tile boundary for the test to mean anything");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a pan that changes the visible tile set must rebuild, never translate a stale set");
}

#[test]
fn vector_scene_cache_invalidates_on_zoom_change() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.camera.zoom *= 1.001;
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a zoom change must not take the translate path");
}

#[test]
fn vector_scene_cache_invalidates_on_viewport_resize() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.set_size(801, 600, 1.0);
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a viewport resize must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_new_tile_upload() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.upload_vector_tile(1, 0, 0, &[]).expect("upload");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a newly uploaded vector tile must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_vector_style_change() {
    let mut host = super::MapHost::with_extension(land_cover_extension());
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.set_vector_style("silhouette");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a vector-style change must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_policy_change() {
    let mut host = vector_cache_host_off_tile_boundary();
    let styled = host.build_vector_scene();
    assert!(!styled.is_empty(), "the land-cover role paints the synthetic tile");
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    host.set_extension(super::Arc::new(super::DefaultMapExtension::default()));
    let neutral = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a replaced policy must never reuse a scene built by the previous one");
    assert!(neutral.path_count() < styled.path_count(), "the neutral policy gives the landcover layer no role");
}

/// 🚫️ The vector-tile layer must NOT key on interaction. Selection and hover are painted by the
/// overlay layers, which are rebuilt every frame outside this cache, so keying on
/// `interaction_revision` bought nothing — and because `pointer_move_screen`/`set_camera` bump that
/// revision on every frame of a pan, it silently invalidated the cache once per frame and pinned a
/// drag at the uncached cost (measured live: 35 ms/frame with it, 2.7 ms without).
#[test]
fn vector_scene_cache_survives_interaction_change_because_selection_draws_outside_it() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.sync_interaction("pin", &["a".to_string()], Some("a"));
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "selection/hover must not rebuild the vector-tile cache");
}

/// 🖐️ The whole point of the cache: a drag must reuse it on every frame that does not cross a tile
/// boundary, even though each `pointer_move_screen` bumps `interaction_revision`.
#[test]
fn vector_scene_cache_survives_a_drag_that_stays_within_one_tile_set() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    let revision = host.visible_vector_tiles_revision();

    host.pointer_down_screen(400.0, 300.0, 1);
    for step in 1..=8 {
        host.pointer_move_screen(400.0, 300.0 + f64::from(step));
        let _ = host.build_vector_scene();
    }
    host.pointer_up_screen(400.0, 308.0);

    assert_eq!(host.visible_vector_tiles_revision(), revision, "this drag must stay inside one tile set for the test to mean anything");
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "a drag within one tile set must never rebuild the vector scene");
}

#[test]
fn tile_retention_keeps_raster_ancestor_after_zoom_in() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let png = test_png_1x1();
    host.upload_tile(0, 0, 0, &png).expect("upload");
    assert!(host.tiles.tile_images.contains_key("0/0/0"));
    host.set_camera(0.0, 0.0, 6000.0);
    let z_fine = host.pick_raster_tile_zoom();
    assert!(z_fine >= 4, "zoomed camera should request finer raster tiles (got {z_fine})");
    host.prepare_visible_tiles();
    assert!(host.tiles.tile_images.contains_key("0/0/0"), "ancestor tile must survive zoom-level change for pyramid fallback");
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
    assert!(host.tiles.vector_tiles.contains_key("0/0/0"));
    host.set_camera(0.0, 0.0, 2500.0);
    let vz = host.pick_vector_tile_zoom();
    assert!(vz >= 1, "zoomed camera should request finer vector tiles (got {vz})");
    host.prepare_visible_tiles();
    assert!(host.tiles.vector_tiles.contains_key("0/0/0"), "ancestor vector tile must survive zoom-level change for pyramid fallback");
}

#[test]
fn map_point_segment_distance_zero_length_segment_uses_point_distance() {
    let d = super::map_point_segment_distance(3.0, 4.0, 0.0, 0.0, 0.0, 0.0);
    assert!((d - 5.0).abs() < 1e-9);
}

#[test]
fn interaction_plan_matches_direct_wheel_and_pan_semantics() {
    let mut direct = super::MapHost::new();
    let mut planned = super::MapHost::new();
    direct.set_size(800, 600, 1.0);
    planned.set_size(800, 600, 1.0);
    direct.wheel_screen(320.0, 240.0, -12.0);
    let wheel = planned.plan_interaction(super::MapInteractionIntent::Wheel { sx: 320.0, sy: 240.0, delta_y: -12.0 });
    assert!(planned.commit_interaction(wheel));
    assert!(super::map_camera_matches(&direct.camera, &planned.camera));

    for intent in [super::MapInteractionIntent::PointerDown { sx: 100.0, sy: 120.0, button: 1 }, super::MapInteractionIntent::PointerMove { sx: 130.0, sy: 150.0 }, super::MapInteractionIntent::PointerUp { sx: 130.0, sy: 150.0 }] {
        let plan = planned.plan_interaction(intent);
        assert!(planned.commit_interaction(plan));
    }
    direct.pointer_down_screen(100.0, 120.0, 1);
    direct.pointer_move_screen(130.0, 150.0);
    direct.pointer_up_screen(130.0, 150.0);
    assert!(super::map_camera_matches(&direct.camera, &planned.camera));
    assert_eq!(direct.drain_events_json(), planned.drain_events_json());
}

#[test]
fn stale_interaction_plan_never_mutates_replaced_camera() {
    let mut host = super::MapHost::new();
    let plan = host.plan_interaction(super::MapInteractionIntent::Wheel { sx: 10.0, sy: 20.0, delta_y: -1.0 });
    host.set_camera(42.0, -7.0, 2.0);
    let replacement = [host.camera.x, host.camera.y, host.camera.zoom];
    assert!(!host.commit_interaction(plan));
    assert_eq!([host.camera.x, host.camera.y, host.camera.zoom], replacement);
}

#[test]
fn map_point_in_polygon_rejects_degenerate_polygon() {
    assert!(!super::map_point_in_polygon(0.0, 0.0, &[]));
    assert!(!super::map_point_in_polygon(0.0, 0.0, &[super::Point::new(0.0, 0.0), super::Point::new(1.0, 1.0)]));
}

#[test]
fn map_polyline_intersects_rect_detects_edge_crossing_without_endpoints_inside() {
    let line = [super::Point::new(-10.0, 5.0), super::Point::new(10.0, 5.0)];
    assert!(super::map_polyline_intersects_rect(&line, 0.0, 0.0, 4.0, 10.0));
    let miss = [super::Point::new(-10.0, 50.0), super::Point::new(10.0, 50.0)];
    assert!(!super::map_polyline_intersects_rect(&miss, 0.0, 0.0, 4.0, 10.0));
}

#[test]
fn map_polyline_intersects_polygon_detects_crossing_edge() {
    let square = [super::Point::new(0.0, 0.0), super::Point::new(10.0, 0.0), super::Point::new(10.0, 10.0), super::Point::new(0.0, 10.0)];
    let crossing = [super::Point::new(-5.0, 5.0), super::Point::new(15.0, 5.0)];
    assert!(super::map_polyline_intersects_polygon(&crossing, &square));
    let outside = [super::Point::new(-5.0, 50.0), super::Point::new(-1.0, 50.0)];
    assert!(!super::map_polyline_intersects_polygon(&outside, &square));
}

#[test]
fn features_in_polygon_json_invalid_json_returns_empty() {
    let host = overlay_host();
    let out = host.features_in_polygon_json("not json", true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert!(v["pins"].as_array().unwrap().is_empty());
    assert!(v["paths"].as_array().unwrap().is_empty());
}

#[test]
fn features_in_polygon_json_requires_at_least_a_triangle() {
    let host = overlay_host();
    let out = host.features_in_polygon_json("[[0.0,0.0],[1.0,1.0]]", true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert!(v["pins"].as_array().unwrap().is_empty());
}

#[test]
fn features_in_polygon_json_finds_an_enclosed_point() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"pins":[{"id":"p1","coordinates":[[0.0,0.0]]}]}}"#).expect("sync");
    let sv: serde_json::Value = serde_json::from_str(&host.feature_screen_json("pin", "p1")).expect("json");
    let (sx, sy) = (sv["x"].as_f64().unwrap(), sv["y"].as_f64().unwrap());
    let polygon = format!("[[{},{}],[{},{}],[{},{}]]", sx - 20.0, sy - 20.0, sx + 20.0, sy - 20.0, sx, sy + 20.0);
    let out = host.features_in_polygon_json(&polygon, true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert_eq!(v["pins"].as_array().unwrap().first().and_then(|x| x.as_str()), Some("p1"));
}

#[test]
fn feature_screen_json_covers_lines_missing_and_unknown_kind() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"paths":[{"id":"r1","coordinates":[[0.0,0.0],[1.0,1.0]]}]}}"#).expect("sync");
    let line_json = host.feature_screen_json("path", "r1");
    assert_ne!(line_json, "null");
    assert!(serde_json::from_str::<serde_json::Value>(&line_json).expect("json")["polyline"].as_array().is_some_and(|rows| rows.len() == 2));
    assert_eq!(host.feature_screen_json("path", "missing"), "null");
    assert_eq!(host.feature_screen_json("bogus", "r1"), "null");
    host.sync_map_json(r#"{"overlays":{"paths":[{"id":"empty","coordinates":[]}]}}"#).expect("sync");
    assert_eq!(host.feature_screen_json("path", "empty"), "null");
}

#[test]
fn sync_interaction_replaces_previous_selection_within_active_granularity() {
    let mut host = overlay_host();
    host.sync_interaction("pin", &["p1".to_string(), "p2".to_string()], None);
    assert_eq!(host.selected_ids("pin").count(), 2);
    host.sync_interaction("path", &["r1".to_string()], None);
    assert_eq!(host.selected_ids("pin").count(), 0, "switching to path clears pins");
    assert_eq!(host.selected_ids("path").collect::<Vec<_>>(), ["r1"]);
    host.sync_interaction("path", &[], None);
    assert_eq!(host.selected_ids("path").count(), 0);
}

#[test]
fn sync_interaction_none_clears_hover_and_some_sets_kind_and_id() {
    let mut host = overlay_host();
    host.sync_interaction("pin", &[], Some("p1"));
    assert_eq!(host.hovered_kind(), Some("pin"));
    assert_eq!(host.hovered_id(), Some("p1"));
    host.sync_interaction("pin", &[], None);
    assert_eq!(host.hovered_kind(), None);
    assert_eq!(host.hovered_id(), None);
}

#[test]
fn sync_interaction_json_resolves_the_declared_granularity_like_the_react_twin() {
    let mut host = overlay_host();
    host.sync_interaction_json(r#"{"pins":["pin-1"],"paths":[]}"#, "null");
    assert_eq!(host.selected_ids("pin").collect::<Vec<_>>(), ["pin-1"]);
    host.sync_interaction_json(r#"{"pins":["pin-1"],"paths":["path-2"]}"#, r#"{"kind":"path","id":"path-2"}"#);
    assert_eq!((host.selected_ids("path").collect::<Vec<_>>(), host.selected_ids("pin").count()), (vec!["path-2"], 0), "the first declared pickable layer wins");
    assert_eq!((host.hovered_kind(), host.hovered_id()), (Some("path"), Some("path-2")));
    host.sync_interaction_json(r#"{"pins":[],"paths":[]}"#, r#"{"kind":"pin","id":"pin-1"}"#);
    assert_eq!((host.hovered_kind(), host.hovered_id(), host.selected_ids("pin").count()), (Some("pin"), Some("pin-1"), 0));
    host.sync_interaction_json(r#"{"zones":["zone-1"],"pins":["pin-1"]}"#, r#"{"kind":"path","id":"path-2"}"#);
    assert_eq!((host.selected_ids("pin").collect::<Vec<_>>(), host.hovered_id()), (vec!["pin-1"], None), "a non-pickable layer never selects and a foreign hover never lands");
    host.sync_interaction_json("not json", "null");
    assert_eq!((host.selected_ids("pin").count(), host.hovered_id()), (0, None));
}

#[test]
fn focus_feature_point_moves_camera_and_missing_id_returns_false() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"pins":[{"id":"p1","coordinates":[[10.0,20.0]]}]}}"#).expect("sync");
    assert!(!host.focus_feature("pin", "missing"));
    assert!(host.focus_feature("pin", "p1"));
    let expected = lonlat_to_world(10.0, 20.0);
    assert!((host.camera.x - expected.x).abs() < 1e-9);
    assert!((host.camera.y - expected.y).abs() < 1e-9);
}

#[test]
fn focus_feature_line_requires_at_least_two_points_and_fits_bounds() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"paths":[{"id":"short","coordinates":[[0.0,0.0]]},{"id":"r1","coordinates":[[0.0,0.0],[5.0,5.0]]}]}}"#).expect("sync");
    assert!(!host.focus_feature("path", "short"));
    assert!(host.focus_feature("path", "r1"));
    assert!(!host.focus_feature("path", "missing"));
}

#[test]
fn focus_feature_unknown_kind_returns_false() {
    let mut host = super::MapHost::new();
    assert!(!host.focus_feature("bogus", "anything"));
}

#[test]
fn has_tile_and_has_vector_tile_reflect_uploads() {
    let mut host = super::MapHost::new();
    assert!(!host.has_tile("0/0/0"));
    host.upload_tile(0, 0, 0, &test_png_1x1()).expect("upload png");
    assert!(host.has_tile("0/0/0"));
    assert!(!host.has_tile("1/0/0"));
    assert!(!host.has_vector_tile("0/0/0"));
}

#[test]
fn upload_tile_invalid_png_bytes_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.upload_tile(0, 0, 0, b"not a png").is_err());
}

#[test]
fn upload_tile_png_decode_matches_image_crate_oracle_for_every_8_bit_color_type() {
    use image::codecs::png::PngEncoder;
    use image::{ColorType, ImageEncoder};
    let (width, height) = (19u32, 13u32);
    for (color, channels) in [(ColorType::Rgba8, 4usize), (ColorType::Rgb8, 3), (ColorType::L8, 1), (ColorType::La8, 2)] {
        let samples: Vec<u8> = (0..width as usize * height as usize * channels).map(|index| (index * 37 + index / 7 * 11) as u8).collect();
        let mut png = Vec::new();
        PngEncoder::new(&mut png).write_image(&samples, width, height, color.into()).expect("encode oracle png");
        let oracle = image::load_from_memory(&png).expect("oracle decode").to_rgba8();
        let owned = semio_framework_pixels::decode_png(&png).expect("owned decode");
        assert_eq!((owned.width, owned.height), oracle.dimensions(), "{color:?} extent");
        assert_eq!(owned.pixels, oracle.into_raw(), "{color:?} pixels");
        let mut host = super::MapHost::new();
        host.upload_tile(0, 0, 0, &png).expect("upload decodes through the owned codec");
        let stored = host.tiles.tile_images.get("0/0/0").expect("stored tile");
        assert_eq!((stored.width(), stored.height()), (width, height), "{color:?} stored extent");
    }
}

#[test]
fn upload_vector_tile_invalid_bytes_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.upload_vector_tile(0, 0, 0, &[0x80]).is_err());
}

#[test]
fn upload_tile_second_call_on_existing_key_is_idempotent_and_skips_decode() {
    let mut host = super::MapHost::new();
    host.upload_tile(0, 0, 0, &test_png_1x1()).expect("first upload decodes");
    let before = host.tiles.tile_images.get("0/0/0").cloned().expect("cached after first upload");
    let result = host.upload_tile(0, 0, 0, b"not a png");
    assert!(result.is_ok(), "re-uploading an already-cached key must short-circuit before decoding");
    let after = host.tiles.tile_images.get("0/0/0").cloned().expect("still cached after second upload");
    assert!(std::sync::Arc::ptr_eq(&before, &after), "cached image must be untouched by the skipped decode");
}

#[test]
fn upload_vector_tile_second_call_on_existing_key_is_idempotent_and_skips_decode() {
    let mut host = super::MapHost::new();
    host.upload_vector_tile(0, 0, 0, &[]).expect("first upload decodes");
    let result = host.upload_vector_tile(0, 0, 0, &[0x80]);
    assert!(result.is_ok(), "re-uploading an already-cached key must short-circuit before decoding");
    assert!(host.has_vector_tile("0/0/0"), "still cached after second upload");
}

#[test]
fn visible_tiles_revision_is_stable_then_changes_on_pan_and_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let r1 = host.visible_tiles_revision();
    let r2 = host.visible_tiles_revision();
    assert_eq!(r1, r2, "revision must be stable across calls with an unchanged camera");
    let z = host.pick_raster_tile_zoom();
    let tile_span = tile_world_rect(z, 0, 0).width();
    host.set_camera(tile_span * 4.0, 0.0, host.camera.zoom);
    let r3 = host.visible_tiles_revision();
    assert_ne!(r1, r3, "revision must change after a pan that moves the visible tile range");
    host.set_camera(0.0, 0.0, host.camera.zoom * 6.0);
    let r4 = host.visible_tiles_revision();
    assert_ne!(r1, r4, "revision must change after a zoom that changes the tile z");
}

#[test]
fn visible_vector_tiles_revision_matches_json_unavailable_sentinel() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.camera.zoom = 0.0;
    assert_eq!(host.visible_vector_tiles_json(), "[]");
    assert_eq!(host.visible_vector_tiles_revision(), 0, "unavailable sentinel must match the json [] case");
    host.camera.zoom = 6000.0;
    assert_ne!(host.visible_vector_tiles_json(), "[]");
}

#[test]
fn prefetch_tiles_are_disjoint_from_visible_share_z_and_respect_cap() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let visible: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    let prefetch: Vec<serde_json::Value> = serde_json::from_str(&host.prefetch_tiles_json()).expect("json");
    assert!(!prefetch.is_empty(), "expect a prefetch ring around a finite visible window");
    let visible_z = visible[0]["z"].as_u64().unwrap();
    let visible_keys: std::collections::BTreeSet<String> = visible.iter().map(|v| v["key"].as_str().unwrap().to_string()).collect();
    for row in &prefetch {
        let key = row["key"].as_str().unwrap();
        assert!(!visible_keys.contains(key), "prefetch row {key} must not duplicate a visible tile");
        assert_eq!(row["z"].as_u64().unwrap(), visible_z, "prefetch must share the visible z");
    }
    assert!(visible.len() + prefetch.len() <= MAX_VISIBLE_TILE_REQUESTS);
}

#[test]
fn prefetch_vector_tiles_json_is_empty_when_vector_tiles_unavailable() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.camera.zoom = 0.0;
    assert_eq!(host.prefetch_vector_tiles_json(), "[]");
}

#[test]
fn lru_eviction_never_evicts_a_pinned_visible_tile_and_takes_the_stalest_one_first() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let vz = host.pick_raster_tile_zoom();
    assert!(vz >= 4, "expected a finer raster zoom for this camera (got {vz})");
    let visible_now = visible_tiles(&host.camera, &host.viewport, vz);
    let (_, vx, vy) = visible_now[0];
    let visible_key = tiles::tile_key(vz, vx, vy);

    let n = 1u32 << vz;
    let stale_x = (vx + n / 2) % n;
    let stale_key = tiles::tile_key(vz, stale_x, vy);

    host.upload_tile(vz, stale_x, vy, &test_png_1x1()).expect("seed stale tile");
    host.upload_tile(vz, vx, vy, &test_png_1x1()).expect("seed visible tile");
    host.prepare_visible_tiles();
    assert!(host.has_tile(&stale_key), "an off-screen tile must survive below the cache cap");
    assert!(host.has_tile(&visible_key));

    let filler_z = 999;
    for i in 0..(super::MAX_MAP_TILE_CACHE_ENTRIES as u32 + 8) {
        host.upload_tile(filler_z, i, 0, &test_png_1x1()).expect("filler upload");
    }
    host.prepare_visible_tiles();

    assert!(!host.has_tile(&stale_key), "least-recently-touched tile must be evicted first under cap pressure");
    assert!(host.has_tile(&visible_key), "a currently-visible (pinned) tile must never be evicted");
}

#[test]
fn set_map_theme_from_json_invalid_json_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.set_map_theme_from_json("not json").is_err());
}

#[test]
fn set_layer_visibility_from_json_invalid_json_returns_err_and_partial_json_overrides_only_given_fields() {
    let mut host = super::MapHost::new();
    assert!(host.set_layer_visibility_from_json("not json").is_err());
    host.set_layer_visibility_from_json(r#"{"roads":false}"#).expect("partial");
    let v: serde_json::Value = serde_json::from_str(&host.layer_visibility_json()).expect("json");
    assert_eq!(v["roads"], false);
    assert_eq!(v["water"], true);
}

#[test]
fn set_layer_stroke_scale_from_json_sanitizes_out_of_range_weights() {
    let mut host = super::MapHost::new();
    assert!(host.set_layer_stroke_scale_from_json("not json").is_err());
    host.set_layer_stroke_scale_from_json(r#"{"roads":999.0,"water":-5.0}"#).expect("scale");
    let v: serde_json::Value = serde_json::from_str(&host.layer_stroke_scale_json()).expect("json");
    assert_eq!(v["roads"].as_f64().unwrap(), super::MAP_LAYER_WEIGHT_MAX);
    assert_eq!(v["water"].as_f64().unwrap(), super::MAP_LAYER_WEIGHT_MIN);
}

#[test]
fn map_lod_scale_json_lists_the_neutral_band() {
    let host = super::MapHost::new();
    let v: Vec<serde_json::Value> = serde_json::from_str(&super::map_lod_scale_json(host.policy())).expect("json");
    assert_eq!(v.len(), 1);
    assert_eq!(v[0]["id"], "all");
    assert!(v[0]["maxZoom"].is_null(), "the neutral band covers every zoom");
}

#[test]
fn band_for_span_falls_back_to_the_span_when_the_pinned_band_is_unknown() {
    let policy = fixture_band_policy();
    let span = 50.0;
    assert_eq!(policy.band_for_span(span, Some("bogus-lod")), policy.band_for_span(span, None));
    assert_eq!(policy.band_for_span(span, Some("band-4")), 4);
}

#[test]
fn weighted_opaque_fill_keeps_alpha_solid() {
    let c = super::weighted_opaque_fill(super::Color::from_rgba8(40, 50, 60, 128), 1.0);
    assert_eq!(c.to_rgba8().a, 255);
}

#[test]
fn label_declutter_rejects_when_over_capacity_or_offscreen() {
    let viewport = Viewport { width: 200, height: 200, dpr: 1.0 };
    let mut declutter = super::LabelDeclutter::for_viewport(&viewport, 20.0, 1);
    assert!(declutter.try_place("first", super::Point::new(50.0, 50.0), 12.0));
    assert!(!declutter.try_place("second", super::Point::new(150.0, 150.0), 12.0), "max_count of 1 must reject a second label");
    let mut roomy = super::LabelDeclutter::for_viewport(&viewport, 20.0, 10);
    assert!(!roomy.try_place("offscreen", super::Point::new(10_000.0, 10_000.0), 12.0));
}

// #region 🔖️MercatorOracleFixture
/// 🌐️ `lodBands` are specification vectors of the sample band table in `lodConstants` (`maxSpanDeg`, `tileZ`), fed to a
/// [`super::DescribedMapPolicy`] — no third party defines a band scheme. The `projection`/`tileNumbering`/`tileBounds`
/// groups of the same fixture ARE cross-checked against the `mercantile` library by
/// `../../🧪️tests/🗺️tiled-map-mercator-oracle/🦀️.rs` and by `🧪️tests/🕸️web-mercator-tile-oracle/🐍️.py`; all three read this
/// one fixture so oracle and subject compare identical inputs.
///
/// @see 🧪️tests/🕸️web-mercator-tile-oracle/🥒️.feature
const MERCATOR_ORACLE_FIXTURE: &str = include_str!("../🧫️fixtures/🕸️web-mercator-tile-oracle/🔣️.json");

/// 📶️ A policy whose band table is the fixture's `lodConstants`, band ids `band-<index>`.
fn fixture_band_policy() -> super::DescribedMapPolicy {
    let fixture: serde_json::Value = serde_json::from_str(MERCATOR_ORACLE_FIXTURE).expect("fixture json");
    let constants = &fixture["lodConstants"];
    let template = super::MapPolicyDescriptor::neutral().bands.remove(0);
    let bands = constants["maxSpanDeg"]
        .as_array()
        .expect("maxSpanDeg")
        .iter()
        .zip(constants["tileZ"].as_array().expect("tileZ"))
        .enumerate()
        .map(|(index, (span, tile_z))| super::MapLodBand { id: format!("band-{index}"), max_span_deg: span.as_f64().expect("span"), raster_tile_z: tile_z.as_u64().expect("tile z") as u32, ..template.clone() })
        .collect();
    super::DescribedMapPolicy::new(super::MapPolicyDescriptor { bands, ..super::MapPolicyDescriptor::neutral() })
}

#[test]
fn band_selection_of_a_described_policy_matches_frozen_specification_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(MERCATOR_ORACLE_FIXTURE).expect("fixture json");
    let policy = fixture_band_policy();
    for entry in fixture["lodBands"].as_array().expect("lodBands array") {
        let span = entry["spanDeg"].as_f64().expect("spanDeg");
        let expected_idx = entry["lodIndex"].as_u64().expect("lodIndex") as usize;
        let expected_tile_z = entry["tileZ"].as_u64().expect("tileZ") as u32;
        let idx = policy.band_for_span(span, None);
        assert_eq!(idx, expected_idx, "band index for span={span}");
        assert_eq!(policy.band(idx).raster_tile_z, expected_tile_z, "tile z for span={span}");
    }
}

#[test]
fn a_host_reports_the_band_its_installed_policy_resolves() {
    let mut host = super::MapHost::with_extension(super::Arc::new(super::DefaultMapExtension::with_policy(fixture_band_policy())));
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let fit = host.camera.zoom;
    for &span_probe in &[180.0, 50.0, 20.0, 8.0, 2.0, 0.6, 0.2, 0.05] {
        host.set_camera(0.0, 0.0, fit * (180.0 / f64::max(span_probe, 1e-6)));
        let expected = format!("band-{}", host.policy().band_for_span(host.viewport_span_degrees(), None));
        let lod: serde_json::Value = serde_json::from_str(&host.current_lod_json()).expect("current lod json");
        assert_eq!(lod["id"], expected.as_str(), "current band for span probe {span_probe}");
    }
}

#[test]
fn visible_tile_count_never_exceeds_max_visible_tile_requests() {
    for zoom in [super::MAP_CAMERA_ZOOM_MIN, 5_000.0, 500_000.0, super::MAP_CAMERA_ZOOM_MAX] {
        let mut host = super::MapHost::new();
        host.set_size(1920, 1080, 2.0);
        host.set_camera(0.0, 0.0, zoom);
        assert!(visible_tiles(&host.camera, &host.viewport, host.pick_raster_tile_zoom()).len() <= MAX_VISIBLE_TILE_REQUESTS);
        assert!(visible_tiles(&host.camera, &host.viewport, host.pick_vector_tile_zoom()).len() <= MAX_VISIBLE_TILE_REQUESTS);
    }
}
// #endregion 🔖️MercatorOracleFixture
