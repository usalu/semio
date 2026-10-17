"""📋 rii: lists the oracle rows of a pos result for the renderer family files (WP2 crates, laws and bins), classified.
usage: rows.py <result.json> [--names n1,n2,...] [--cfg lib|test|both]"""
import json, re, sys, collections
FAM = ("engine_board", "engine_graph", "engine_attach", "engine_attach_tests", "ui_engine_tests", "engine_registry", "engine_text", "scene_state", "canvas2d_host", "scene_canvas2d", "scene_ink", "scene_feeds", "scene_tables", "scene_text_vfs", "icon_host", "scene_render", "scene_ink_tests")
PFX = "__m_semio_framework_os_renderer_wgpu_"
NAMES = "set_camera_silent begin_pointer_commit pointer_lane_is_direct pointer_lane_in_flight commit_wheel fit_camera_to_content connection_cursor_active_at_screen screen_hit screen_hit_json hovered_node_id_ref screen_geometry_census_json hovered_node_id selection_union_bounds_screen_json set_hover_channel set_dimmed adopt_camera_or_fit refit_camera_if_content_left_view set_node_statuses set_computing_progress set_selection_ids_silent set_transform_flags defers_descriptor_sync_from_js dag_set_wheel_zoom_active wheel_screen".split()
def unit(r):
    m = re.match(r"__m_([a-z0-9_]+)/", r["file"])
    if not m:
        return None
    n = m.group(1)
    return n[len("semio_framework_os_renderer_wgpu_"):] if n.startswith("semio_framework_os_renderer_wgpu_") else n
def load(path, cfg):
    return json.load(open(path))["summary"][cfg]["new_list"]
if __name__ == "__main__":
    path = sys.argv[1]
    cfgs = ["lib", "test"]
    names = set(NAMES)
    out = collections.defaultdict(list)
    for cfg in cfgs:
        for r in load(path, cfg):
            u = unit(r)
            if r.get("stub"):
                continue
            hit = re.search(r"`([A-Za-z_0-9]+)`", r["msg"])
            if r["code"] == "E0425" and hit and hit.group(1) in names:
                out[cfg].append((u, r["file"].split("/")[-2], r["line"], hit.group(1)))
    for cfg in cfgs:
        print("==", cfg, len(out[cfg]))
        c = collections.Counter((u, f) for u, f, l, n in out[cfg])
        for k, v in sorted(c.items()):
            print("  ", v, k)
    json.dump({k: v for k, v in out.items()}, open(path.replace("-result.json", "-ownrows.json"), "w"), ensure_ascii=False)
