import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tf
TM = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map"
src = TM + "/🧪️tests/🔬️unit/🦀️.rs"
t = open(src, encoding="utf-8").read()
names = ["lonlat_world_round_trip","default_world_camera_fits_extent","visible_tiles_at_world_view","typed_visible_tile_cursor_matches_legacy_order_and_resumes_one_item_at_a_time","map_camera_max_zoom_reaches_street_longitude_span","map_lod_picks_bounded_tile_zoom_at_world_fit","ideal_tile_z_rises_when_viewport_span_shrinks","map_wheel_screen_keeps_world_under_cursor_with_flipped_y","map_render_mode_parse","map_vector_style_parse","parse_tile_key_round_trips_tile_key","parse_tile_key_rejects_malformed_keys","clamp_map_layer_weight_handles_bounds_and_non_finite_values","map_layer_stroke_scale_sanitized_clamps_every_field","map_camera_limits_json_min_never_exceeds_max","map_layer_weight_slider_keys_respect_render_mode","map_vector_style_as_str_round_trips_all_variants","clamp_map_zoom_and_for_viewport_stay_within_bounds"]
blocks = tf.take(t, names)
assert len(blocks) == len(names), (len(blocks), len(names))
body = "\n\n".join(blocks).replace("    use super::canvas::camera::{Camera, Viewport};\n", "")
header = '''//! 🧪️ Unit proofs of the tiled-map model: projection round trips, tile addressing, camera clamps, layer-state parsing and the map-policy slider surface.

use super::projection::{default_world_camera, lonlat_to_world, world_to_lonlat, WORLD_HALF};
use super::tiles::{self, visible_tiles};
use super::MAX_VISIBLE_TILE_REQUESTS;
use canvas::camera::{Camera, Viewport};

'''
out = TM + "/🧮️model/🧪️tests/🔬️unit/🦀️.rs"
os.makedirs(os.path.dirname(out), exist_ok=True)
open(out, "w", encoding="utf-8").write(header + body + "\n")
open(src, "w", encoding="utf-8").write(tf.drop(t, names))
print("moved", len(blocks))
