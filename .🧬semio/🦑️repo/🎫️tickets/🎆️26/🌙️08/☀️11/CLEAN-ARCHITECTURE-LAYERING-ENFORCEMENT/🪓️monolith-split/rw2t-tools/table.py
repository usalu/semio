"""📊 rw2t: per-crate resolve/privacy rows of oracle results (artifact / foreign / carve-attributable)."""
import sys, re, collections, json
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rw2t-tools")
from rows import *
G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/"
WP = ('engine_registry', 'engine_text', 'engine_board', 'engine_graph', 'engine_attach', 'scene_state', 'canvas2d_host', 'scene_canvas2d', 'scene_ink', 'scene_feeds', 'scene_tables', 'scene_text_vfs', 'icon_host', 'scene_render', 'engine_attach_tests', 'scene_ink_tests', 'ui_engine_tests')
ART = re.compile(r"element_contracts|host_base|asset_authority|canvas2d_paint|media_slots|map_tiles|infinite_world|camera_storage")
FOREIGN = re.compile(r"`(set_camera_silent|begin_pointer_commit|pointer_lane_is_direct|pointer_lane_in_flight|commit_wheel|set_selection_ids_silent|set_hover_channel|set_dimmed|adopt_camera_or_fit|refit_camera_if_content_left_view|set_node_statuses|set_computing_progress|set_transform_flags|fit_camera_to_content|connection_cursor_active_at_screen|screen_hit|screen_hit_json|hovered_node_id_ref|hovered_node_id|screen_geometry_census_json|selection_union_bounds_screen_json|defers_descriptor_sync_from_js)`")
def cls(r):
    if r["cat"] == "privacy":
        return "carve"
    if ART.search(r["msg"]):
        return "oracle-artifact"
    if FOREIGN.search(r["msg"]):
        return "foreign"
    return "carve"
def table(path):
    out = {}
    for cfg in ("lib", "test"):
        c = collections.Counter()
        for r in load(path, cfg):
            if pk_of(r) in WP and r["cat"] in ("privacy", "resolve") and not r["stub"]:
                c[(r["cat"], cls(r))] += 1
        out[cfg] = c
    return out
if __name__ == "__main__":
    for p in sys.argv[1:]:
        t = table(G + p)
        print("##", p)
        for cfg, c in t.items():
            print("  ", cfg, dict(c))
