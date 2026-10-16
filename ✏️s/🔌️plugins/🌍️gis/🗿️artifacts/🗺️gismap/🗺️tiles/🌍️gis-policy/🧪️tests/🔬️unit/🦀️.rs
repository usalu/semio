//! 🔬️ Unit tests of the GIS map policy: band and tile-z selection, OpenMapTiles paint gates, stroke widths, label rules, weight sliders, and a `MapHost` rendering the demotiles fixtures by it — the GIS assertions moved out of the framework tiled-map tests.

use crate::standards::v1::subsets::any::io::text::snapshot::default_document;
use crate::tile_coordinates::gis_policy::{gis_map_extension, gis_map_layer_weight_slider_ids_json, gis_map_lod_scale_json, gis_map_overlays_value, gis_map_policy, gis_map_scene_descriptor_json, GIS_MAP_LAYER_KEYS, GIS_MAP_POLICY_ID};
use crate::GisMapSnapshot;
use semio_framework_surface::tiled_map::projection::tile_world_rect;
use semio_framework_surface::tiled_map::{MapFeatureFacts, MapHost, MapLayerGates, MapLayerRole, MapPolicy, MapTileMode, MAP_CAMERA_ZOOM_MAX, MAX_VISIBLE_TILE_REQUESTS};

const FIXTURE_TILES: &[(&str, &[u8])] = &[
    ("0-0-0", include_bytes!("../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🧫️fixtures/🗺️vector-tiles/0-0-0/🛰️.pbf")),
    ("3-4-2", include_bytes!("../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🧫️fixtures/🗺️vector-tiles/3-4-2/🛰️.pbf")),
    ("5-17-11", include_bytes!("../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🧫️fixtures/🗺️vector-tiles/5-17-11/🛰️.pbf")),
];

fn fixture_tile(key: &str) -> &'static [u8] {
    FIXTURE_TILES.iter().find(|(name, _)| *name == key).expect("demotiles fixture").1
}

fn gis_host() -> MapHost {
    let mut host = MapHost::with_extension(gis_map_extension());
    host.set_size(800, 600, 1.0);
    host
}

fn band(span: f64) -> usize {
    gis_map_policy().band_for_span(span, None)
}

fn gates(span: f64, tile_z: u32, pinned: Option<&str>) -> MapLayerGates {
    let policy = gis_map_policy();
    policy.layer_gates(policy.band_for_span(span, pinned), span, tile_z)
}

fn road_visible(class: &str, span: f64, pinned: Option<&str>) -> bool {
    let policy = gis_map_policy();
    policy.road_visible(class, span, policy.band_for_span(span, pinned))
}

fn boundary_visible(admin_level: u64, span: f64, tile_z: u32, pinned: Option<&str>) -> bool {
    gis_map_policy().boundary_visible(admin_level, &gates(span, tile_z, pinned))
}

fn road_width(class: &str, line_scale: f64) -> f64 {
    gis_map_policy().stroke_width(MapLayerRole::Road, &MapFeatureFacts { class, admin_level: None, maritime: false }, line_scale)
}

fn label_visible(layer: &str, class: &str, span: f64) -> bool {
    gis_map_policy().label_gate_and_rank(layer, class, span).is_some()
}

fn current_lod(host: &MapHost) -> serde_json::Value {
    serde_json::from_str(&host.current_lod_json()).expect("current lod json")
}

#[test]
fn the_policy_carries_its_id_and_the_extension_renders_by_it() {
    let host = gis_host();
    assert_eq!(host.policy().id(), GIS_MAP_POLICY_ID);
    assert_eq!(host.extension().map_crs(), "EPSG:3857");
    assert_eq!(host.policy().descriptor(), gis_map_policy().descriptor());
}

#[test]
fn max_camera_zoom_enters_the_finest_band() {
    let mut host = gis_host();
    host.set_camera(0.0, 0.0, MAP_CAMERA_ZOOM_MAX);
    let span = host.viewport_span_degrees();
    assert!(span < gis_map_policy().band(6).max_span_deg, "max zoom should pass the street band (span={span:.6}°)");
    assert_eq!(current_lod(&host)["id"], "building");
}

#[test]
fn world_fit_uses_world_band_raster_tiles() {
    let mut host = gis_host();
    host.fit_world_camera();
    assert_eq!(host.pick_raster_tile_zoom(), 0, "world-fit automatic LOD must use world-band raster tiles");
}

#[test]
fn vector_tile_zoom_is_clamped_to_the_openfreemap_maximum() {
    let mut host = gis_host();
    host.set_camera(0.0, 0.0, MAP_CAMERA_ZOOM_MAX);
    assert_eq!(gis_map_policy().descriptor().vector_tile_z_max, 14);
    assert!(host.pick_vector_tile_zoom() <= 14);
}

#[test]
fn bands_resolve_from_the_visible_longitude_span() {
    let mut host = gis_host();
    host.fit_world_camera();
    assert_eq!(band(host.viewport_span_degrees()), 0, "default world fit should be world LOD");
    host.set_camera(0.0, 0.0, 6000.0);
    let span = host.viewport_span_degrees();
    assert!(band(span) >= 2, "zoomed europe view should reach at least country LOD (span={span:.1}°)");
}

#[test]
fn region_and_city_road_strokes_stay_thin() {
    let policy = gis_map_policy();
    let region_scale = policy.line_scale(8.0);
    let city_scale = policy.line_scale(2.5);
    let district_scale = policy.line_scale(0.2);
    assert!(region_scale <= 1.38, "region band line_scale should not exceed cap (got {region_scale})");
    assert!(city_scale <= 1.38, "city band line_scale should not exceed cap (got {city_scale})");
    let region_lod = policy.band(band(8.0)).road_stroke_scale;
    let city_lod = policy.band(band(2.5)).road_stroke_scale;
    assert!((region_lod - 0.4).abs() < f64::EPSILON);
    assert!((city_lod - 0.3).abs() < f64::EPSILON);
    let primary_region = road_width("primary", region_scale) * region_lod;
    let tertiary_city = road_width("tertiary", city_scale) * city_lod;
    let residential_city = road_width("residential", city_scale) * city_lod;
    let minor_district = road_width("minor", district_scale);
    assert!(primary_region < 1.05, "primary roads at region zoom should stay under 1.05px (got {primary_region})");
    assert!(tertiary_city < 0.55, "tertiary roads at city zoom should stay under 0.55px (got {tertiary_city})");
    assert!(residential_city < 0.4, "residential roads at city zoom should stay under 0.4px (got {residential_city})");
    assert!(minor_district < 1.35, "minor roads at district zoom should stay under 1.35px (got {minor_district})");
}

#[test]
fn continental_span_hides_roads_and_caps_tile_z() {
    assert!(!road_visible("motorway", 42.0, None));
    assert!(gates(42.0, 4, None).land_cover);
    assert_eq!(gis_map_policy().vector_zoom_cap(42.0), 5);
    let profile = gates(42.0, 2, None);
    assert!(!profile.boundaries);
    assert!(profile.coastline);
    assert!(!profile.land_backdrop);
    assert!(profile.land_cover);
    assert!(!boundary_visible(2, 42.0, 2, None));
    assert!(boundary_visible(2, 20.0, 5, None));
    assert!(!boundary_visible(4, 20.0, 5, None));
    assert!(profile.water);
    assert!(!profile.roads);
}

#[test]
fn country_lod_hides_regional_boundaries() {
    let span = 20.0;
    assert_eq!(band(span), 2);
    let profile = gates(span, 7, None);
    assert!(profile.boundaries);
    assert!(profile.land_backdrop);
    assert!(!profile.land_cover);
    assert_eq!(profile.max_admin_level, 2);
    assert!(boundary_visible(2, span, 7, None));
    assert!(!boundary_visible(4, span, 7, None));
    assert!(!boundary_visible(6, span, 7, None));
    let forced = gates(8.0, 10, Some("country"));
    assert_eq!(forced.max_admin_level, 2);
    assert!(boundary_visible(2, 8.0, 10, Some("country")));
    assert!(!boundary_visible(4, 8.0, 10, Some("country")));
    assert!(!boundary_visible(6, 8.0, 10, Some("country")));
}

#[test]
fn region_lod_includes_country_and_state_boundaries() {
    let span = 8.0;
    assert_eq!(band(span), 3);
    let profile = gates(span, 10, None);
    assert!(profile.land_cover);
    assert!(profile.roads);
    assert_eq!(profile.max_admin_level, 6);
    assert!(boundary_visible(2, span, 10, None));
    assert!(boundary_visible(4, span, 10, None));
    assert!(boundary_visible(6, span, 10, None));
    assert_eq!(gis_map_policy().vector_zoom_cap(span), 6);
    assert!(road_visible("primary", span, None));
    assert!(!road_visible("secondary", span, None));
    assert!(!road_visible("tertiary", span, None));
}

#[test]
fn city_lod_fills_tile_backdrop_to_hide_seams() {
    let span = 2.0;
    assert_eq!(band(span), 4);
    let profile = gates(span, 10, None);
    assert!(profile.land_backdrop);
    assert!(profile.land_cover);
    assert!(profile.roads);
}

#[test]
fn region_lod_exceeds_country_detail() {
    let span = 8.0;
    let country = gates(span, 10, Some("country"));
    let region = gates(span, 10, Some("region"));
    assert!(!country.land_cover);
    assert!(region.land_cover);
    assert!(country.land_backdrop);
    assert!(region.land_backdrop);
    assert!(!country.roads);
    assert!(region.roads);
    assert_eq!(country.max_admin_level, 2);
    assert_eq!(region.max_admin_level, 6);
    assert!(boundary_visible(2, span, 10, Some("region")));
    assert!(boundary_visible(4, span, 10, Some("region")));
    assert!(!boundary_visible(4, span, 10, Some("country")));
}

#[test]
fn country_span_vector_profile_matches_lod_band() {
    let span = 20.0;
    assert_eq!(band(span), 2);
    let profile = gates(span, 2, None);
    assert!(profile.boundaries);
    assert_eq!(gis_map_policy().vector_zoom_cap(span), 7);
    assert_eq!(profile.boundaries, gates(span, 1, None).boundaries);
}

#[test]
fn buildings_need_the_finest_band_below_city_span_and_tile_z_13() {
    assert!(gates(0.05, 14, None).buildings);
    assert!(!gates(0.05, 12, None).buildings);
    assert!(!gates(0.2, 14, None).buildings);
    assert!(!gates(2.0, 14, Some("building")).buildings);
}

#[test]
fn weight_slider_keys_follow_lod_and_render_mode() {
    let keys = |lod: &str, mode: &str| -> Vec<String> { serde_json::from_str(&gis_map_layer_weight_slider_ids_json(lod, mode)).expect("slider ids") };
    let world_combined = keys("world", "combined");
    assert!(world_combined.contains(&"raster".to_string()));
    assert!(world_combined.contains(&"water".to_string()));
    assert!(!world_combined.contains(&"roads".to_string()));
    assert!(!world_combined.contains(&"buildings".to_string()));
    let street_vector = keys("street", "vector");
    assert!(!street_vector.contains(&"raster".to_string()));
    assert!(street_vector.contains(&"roads".to_string()));
    assert!(!street_vector.contains(&"buildings".to_string()));
    let building_combined = keys("building", "combined");
    assert!(building_combined.contains(&"buildings".to_string()));
    assert!(building_combined.contains(&"raster".to_string()));
    let image_only = keys("city", "image");
    assert!(image_only.contains(&"raster".to_string()));
    assert!(!image_only.contains(&"roads".to_string()));
    assert!(!image_only.contains(&"water".to_string()));
    let vector_city = gis_map_policy().slider_keys("city", MapTileMode::Vector);
    assert!(!vector_city.contains(&"raster"));
    let bogus_mode = gis_map_policy().slider_keys("city", MapTileMode::parse("bogus-mode"));
    assert!(bogus_mode.contains(&"raster"));
    assert!(bogus_mode.contains(&"positions"));
}

#[test]
fn world_vector_tiles_follow_the_span_cap_while_raster_stays_coarse() {
    let mut host = gis_host();
    host.fit_world_camera();
    assert_eq!(host.pick_raster_tile_zoom(), 0, "world raster stays at z0");
    assert_eq!(host.pick_vector_tile_zoom(), gis_map_policy().vector_zoom_cap(host.viewport_span_degrees()), "world vector tiles must be finer than raster so countries/coastlines paint");
}

#[test]
fn forced_building_lod_bounds_tile_requests_at_world_zoom() {
    let mut host = gis_host();
    host.fit_world_camera();
    host.set_lod_mode("building");
    let z = host.pick_raster_tile_zoom();
    assert!(z < gis_map_policy().band(7).raster_tile_z, "forced building at world view must clamp tile z (got {z})");
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    assert!(raw.len() <= MAX_VISIBLE_TILE_REQUESTS);
}

#[test]
fn forced_country_lod_allows_finer_tiles_when_zoomed_in() {
    let mut host = gis_host();
    host.set_lod_mode("country");
    host.set_camera(0.0, 0.0, 6000.0);
    let z = host.pick_raster_tile_zoom();
    assert!(z > gis_map_policy().band(2).raster_tile_z, "pinned country LOD must not cap tile z when zoomed in (got {z})");
    let lod = current_lod(&host);
    assert_eq!(lod["id"], "country");
    assert_eq!(lod["mode"], "country");
}

#[test]
fn an_unknown_pinned_band_falls_back_to_the_span() {
    let policy = gis_map_policy();
    assert_eq!(policy.band_for_span(50.0, Some("bogus-lod")), policy.band_for_span(50.0, None));
    assert_eq!(policy.band_for_span(50.0, Some("city")), policy.band_index("city").expect("city band"));
}

#[test]
fn street_names_are_stricter_than_road_geometry_at_city_span() {
    assert!(label_visible("transportation_name", "primary", 2.0));
    assert!(!label_visible("transportation_name", "secondary", 2.0));
    assert!(!label_visible("transportation_name", "street", 2.0));
    assert!(label_visible("transportation_name", "secondary", 0.8));
}

#[test]
fn label_px_scales_with_span_inside_the_city_band() {
    let policy = gis_map_policy();
    let wide = policy.label_px(band(2.0), 2.0, 1.0);
    let narrow = policy.label_px(band(1.3), 1.3, 1.0);
    assert!(narrow > wide, "labels should grow when zooming in (span shrinks): {wide} vs {narrow}");
    assert!((wide - 12.5 * 4.0 / 2.0).abs() < 1e-6);
}

#[test]
fn poi_labels_hide_until_district_span() {
    assert!(!label_visible("poi", "", 2.0));
    assert!(label_visible("poi", "", 0.3));
}

#[test]
fn place_labels_cover_every_admin_class() {
    let v = |class: &str, span: f64| label_visible("place", class, span);
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
fn label_ranks_put_layer_overrides_before_place_classes() {
    let policy = gis_map_policy();
    assert_eq!(policy.label_gate_and_rank("poi", "", 0.3), Some(48));
    assert_eq!(policy.label_gate_and_rank("centroids", "country", 20.0), Some(8));
    assert_eq!(policy.label_gate_and_rank("place", "continent", 50.0), Some(0));
    assert_eq!(policy.label_gate_and_rank("transportation_name", "motorway", 2.0), Some(30));
    assert_eq!(policy.label_gate_and_rank("water", "lake", 2.0), None, "a paint role carries no label");
}

#[test]
fn lod_scale_json_lists_all_eight_bands() {
    let rows: Vec<serde_json::Value> = serde_json::from_str(&gis_map_lod_scale_json()).expect("json");
    assert_eq!(rows.len(), 8);
    assert_eq!(rows[0]["id"], "world");
    assert!(rows[0]["maxZoom"].as_f64().expect("world max zoom") > 0.0);
    assert!(rows[7]["maxZoom"].is_null(), "the finest band is unbounded");
}

#[test]
fn band_selection_matches_frozen_specification_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("fixture json");
    let policy = gis_map_policy();
    for entry in fixture["lodBands"].as_array().expect("lodBands array") {
        let span = entry["spanDeg"].as_f64().expect("spanDeg");
        let index = policy.band_for_span(span, None);
        assert_eq!(index as u64, entry["lodIndex"].as_u64().expect("lodIndex"), "band for span={span}");
        assert_eq!(u64::from(policy.band(index).raster_tile_z), entry["tileZ"].as_u64().expect("tileZ"), "tile z for span={span}");
    }
}

#[test]
fn the_current_band_tracks_the_span_resolver() {
    let mut host = gis_host();
    host.fit_world_camera();
    let fit = host.camera.zoom;
    for span_probe in [180.0, 50.0, 20.0, 8.0, 2.0, 0.6, 0.2, 0.05] {
        host.set_camera(0.0, 0.0, fit * (180.0 / f64::max(span_probe, 1e-6)));
        let expected = gis_map_policy().band(band(host.viewport_span_degrees())).id.clone();
        assert_eq!(current_lod(&host)["id"], expected.as_str(), "current band for span probe {span_probe}");
    }
}

#[test]
fn the_scene_descriptor_ships_the_policy_to_a_neutral_host() {
    let mut renderer = MapHost::new();
    renderer.set_size(800, 600, 1.0);
    renderer.set_lod_mode("city");
    assert_eq!(current_lod(&renderer)["mode"], "automatic", "the neutral policy declares no city band yet");
    renderer.sync_map_json(&gis_map_scene_descriptor_json(&GisMapSnapshot::default())).expect("scene descriptor");
    assert_eq!(renderer.policy().descriptor(), gis_map_policy().descriptor());
    assert_eq!(current_lod(&renderer)["mode"], "city", "the pinned band applies once the policy declares it");
    let mut gis = gis_host();
    gis.set_lod_mode("city");
    renderer.set_camera(0.0, 0.0, 6000.0);
    gis.set_camera(0.0, 0.0, 6000.0);
    assert_eq!(renderer.pick_raster_tile_zoom(), gis.pick_raster_tile_zoom());
    assert_eq!(renderer.pick_vector_tile_zoom(), gis.pick_vector_tile_zoom());
}

fn zoom_to_band(host: &mut MapHost, band_id: &str) {
    let policy = gis_map_policy();
    let target = policy.band(policy.band_index(band_id).expect("band id")).representative_span_deg;
    host.fit_world_camera();
    let mut lo = host.camera.zoom;
    let mut hi = MAP_CAMERA_ZOOM_MAX;
    for _ in 0..48 {
        let mid = (lo + hi) * 0.5;
        host.set_camera(0.0, 0.0, mid);
        if host.viewport_span_degrees() > target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    host.set_camera(0.0, 0.0, hi);
}

fn zoom_over_tile(host: &mut MapHost, band_id: &str, tz: u32, tx: u32, ty: u32) {
    zoom_to_band(host, band_id);
    let rect = tile_world_rect(tz, tx, ty);
    host.set_camera((rect.x0() + rect.x1()) * 0.5, -(rect.y0() + rect.y1()) * 0.5, host.camera.zoom);
}

#[test]
fn figure_world_tile_paints_many_countries() {
    let mut host = gis_host();
    host.set_render_mode("vector");
    host.set_vector_style("figureGround");
    host.set_lod_mode("world");
    host.fit_world_camera();
    host.upload_vector_tile(0, 0, 0, fixture_tile("0-0-0")).expect("vector tile");
    assert!(!host.build_vector_scene().is_empty());
}

#[test]
fn colored_world_tile_paints_land_over_water_backdrop() {
    let mut host = gis_host();
    host.set_render_mode("vector");
    host.set_vector_style("colored");
    host.fit_world_camera();
    host.upload_vector_tile(0, 0, 0, fixture_tile("0-0-0")).expect("vector tile");
    assert!(host.build_vector_scene().path_count() > 4, "colored world LOD must paint country landmasses, not only the water backdrop");
}

#[test]
fn the_openmaptiles_paint_roles_come_from_the_gis_policy() {
    let mut neutral = MapHost::new();
    neutral.set_size(800, 600, 1.0);
    neutral.set_render_mode("vector");
    neutral.fit_world_camera();
    neutral.upload_vector_tile(0, 0, 0, fixture_tile("0-0-0")).expect("vector tile");
    let mut host = gis_host();
    host.set_render_mode("vector");
    host.fit_world_camera();
    host.upload_vector_tile(0, 0, 0, fixture_tile("0-0-0")).expect("vector tile");
    assert!(host.build_vector_scene().path_count() > neutral.build_vector_scene().path_count(), "the OpenMapTiles roles live in the GIS policy, not the framework");
}

#[test]
fn figure_country_lod_uses_land_mass_backdrop() {
    let mut host = gis_host();
    host.set_render_mode("vector");
    host.set_vector_style("figureGround");
    host.set_lod_mode("country");
    host.fit_world_camera();
    host.upload_vector_tile(3, 4, 2, fixture_tile("3-4-2")).expect("vector tile");
    assert!(!host.build_vector_scene().is_empty());
}

#[test]
fn label_camera_setup_centres_the_fixture_tile() {
    let mut host = gis_host();
    host.set_lod_mode("country");
    zoom_over_tile(&mut host, "country", 5, 17, 11);
    host.upload_vector_tile(5, 17, 11, fixture_tile("5-17-11")).expect("vector tile");
    let rect = tile_world_rect(5, 17, 11);
    let (cx, cy) = (host.camera.x, -host.camera.y);
    assert!(rect.x0() <= cx && cx <= rect.x1() && rect.y0() <= cy && cy <= rect.y1(), "the camera centres the fixture tile");
    assert!(!host.build_vector_scene().is_empty(), "fixture tile should paint geometry (span={})", host.viewport_span_degrees());
}

fn labelled_path_counts(style: &str) -> (usize, usize) {
    let mut host = gis_host();
    host.set_render_mode("vector");
    host.set_vector_style(style);
    host.set_lod_mode("country");
    zoom_over_tile(&mut host, "country", 5, 17, 11);
    host.upload_vector_tile(5, 17, 11, fixture_tile("5-17-11")).expect("vector tile");
    host.set_layer_visibility_from_json(r#"{"labels":false}"#).expect("labels off");
    let without = host.build_vector_scene().path_count();
    host.set_layer_visibility_from_json(r#"{"labels":true}"#).expect("labels on");
    (without, host.build_vector_scene().path_count())
}

#[test]
fn figure_ground_labels_increase_scene_when_enabled() {
    let (without, with_labels) = labelled_path_counts("figureGround");
    assert!(with_labels > without, "figure-ground labels should add glyph paths (with={with_labels}, without={without})");
}

#[test]
fn colored_vector_labels_increase_scene_when_enabled() {
    let (without, with_labels) = labelled_path_counts("colored");
    assert!(with_labels > without, "colored vector labels should add glyph paths (with={with_labels}, without={without})");
}

#[test]
fn the_gis_policy_declares_its_layer_toggles_overlays_and_style_presets() {
    let descriptor = gis_map_policy().descriptor();
    assert_eq!(descriptor.layer_toggles.iter().map(|toggle| toggle.key.as_str()).collect::<Vec<_>>(), GIS_MAP_LAYER_KEYS);
    let overlays: Vec<(&str, Option<&str>)> = descriptor.overlays.iter().map(|layer| (layer.id.as_str(), layer.granularity.as_deref())).collect();
    assert_eq!(overlays, [("regions", None), ("routes", Some("route")), ("positions", Some("position"))], "regions paint below routes below positions; only routes and positions pick");
    assert_eq!(descriptor.style_presets.iter().map(|preset| preset.id.as_str()).collect::<Vec<_>>(), ["colored", "figureGround", "invertedFigure"]);
    assert_eq!(gis_map_policy().style_preset("unknown").id, "colored", "an unknown style falls back to the first preset");
}

#[test]
fn the_overlays_value_carries_every_document_feature() {
    let document = default_document();
    let overlays = gis_map_overlays_value(&document);
    let ids = |layer: &str| -> Vec<String> { overlays[layer].as_array().expect("overlay rows").iter().map(|row| row["id"].as_str().expect("id").to_owned()).collect() };
    assert_eq!(ids("positions"), document.positions.iter().map(|feature| feature.id.clone()).collect::<Vec<_>>());
    assert_eq!(ids("routes"), document.routes.iter().map(|feature| feature.id.clone()).collect::<Vec<_>>());
    assert_eq!(ids("regions"), document.regions.iter().map(|feature| feature.id.clone()).collect::<Vec<_>>());
    assert!(overlays["positions"].as_array().expect("positions").iter().all(|row| row["coordinates"].as_array().is_some_and(|points| points.len() == 1)), "a position is one coordinate");
}

#[test]
fn gis_hits_marquees_and_selection_keep_their_position_and_route_shapes() {
    let mut host = gis_host();
    host.fit_world_camera();
    host.sync_map_json(r#"{"overlays":{"positions":[{"id":"p","coordinates":[[0.0,0.0]],"label":"P","style":"donor","link":"https://example.org"}],"routes":[{"id":"r","coordinates":[[-20.0,-20.0],[20.0,-20.0]]}],"regions":[{"id":"g","coordinates":[[-5.0,-5.0],[5.0,-5.0],[5.0,5.0]]}]}}"#).expect("sync");
    let point: serde_json::Value = serde_json::from_str(&host.feature_screen_json("position", "p")).expect("screen point");
    let (x, y) = (point["x"].as_f64().expect("x"), point["y"].as_f64().expect("y"));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&host.hit_test_feature_json(x, y)).expect("hit"), serde_json::json!({ "kind": "position", "id": "p" }));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&host.features_in_rect_json(0.0, 0.0, 800.0, 600.0, false)).expect("rect"), serde_json::json!({ "positions": ["p"], "routes": ["r"] }), "regions never pick");
    assert_eq!(host.overlay_feature("positions", "p").and_then(|feature| feature.link.as_deref()), Some("https://example.org"));
    host.sync_interaction_json(r#"{"positions":["p"],"routes":["r"]}"#, r#"{"kind":"route","id":"r"}"#);
    assert_eq!((host.selected_ids("route").collect::<Vec<_>>(), host.hovered_id()), (vec!["r"], Some("r")), "routes win over positions, as before");
}
