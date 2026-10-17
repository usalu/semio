"""✏️ rii step 1: explicit imports of the unimported infinite free functions and the manifest edges. usage: step1.py [--dry]"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ed import *
import addep
DRY = "--dry" in sys.argv
E = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T = E + "/🎯️targets/🧊️wgpu"
B = E + "/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu"
INF = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed"
CR = {
    "focus": ("semio-framework-os-infinite-board-normal-focus", INF + "/➕️normal/🎯️focus/📦️packages/🦀️rust"),
    "gestures": ("semio-framework-os-infinite-board-normal-gestures", INF + "/➕️normal/🕹️gestures/📦️packages/🦀️rust"),
    "host": ("semio-framework-os-infinite-board-normal-host", INF + "/➕️normal/🧭️host/📦️packages/🦀️rust"),
    "dag-graph": ("semio-framework-os-infinite-board-dag-graph", INF + "/🕸️dag/🕸️graph/📦️packages/🦀️rust"),
    "dag-host": ("semio-framework-os-infinite-board-dag-host", INF + "/🕸️dag/🧭️host/📦️packages/🦀️rust"),
}
def deps(*ids):
    return [CR[i] for i in ids]

replace(B + "/🎲️board2d/🦀️.rs", [(
    "use super::paint2d::write_interaction_hover;\n",
    "use semio_framework_os_infinite_board_normal_focus::{commit_wheel, set_camera_silent};\n"
    "use semio_framework_os_infinite_board_normal_gestures::{pointer_lane_in_flight, pointer_lane_is_direct};\n"
    "use semio_framework_os_infinite_board_normal_host::begin_pointer_commit;\n"
    "use super::paint2d::write_interaction_hover;\n", 1)], dry=DRY)
replace(B + "/🕸️node-graph/🦀️.rs", [(
    "use dag::dag_set_wheel_zoom_active;\n",
    "use semio_framework_os_infinite_board_dag_graph::{dag_set_wheel_zoom_active, fit_camera_to_content, hovered_node_id, hovered_node_id_ref, selection_union_bounds_screen_json};\n"
    "use semio_framework_os_infinite_board_dag_host::{connection_cursor_active_at_screen, screen_geometry_census_json, screen_hit, screen_hit_json};\n", 1)], dry=DRY)
replace(B + "/🧩️engine-surface-attach/🦀️.rs", [(
    "use infinite_canvas::{board_set_camera, board_set_grid_visible};\n",
    "use infinite_canvas::{board_set_camera, board_set_grid_visible};\n"
    "use semio_framework_os_infinite_board_dag_graph::{adopt_camera_or_fit, refit_camera_if_content_left_view, set_hover_channel};\n"
    "use semio_framework_os_infinite_board_dag_host::{set_computing_progress, set_dimmed, set_node_statuses};\n"
    "use semio_framework_os_infinite_board_normal_focus::{set_camera_silent, set_selection_ids_silent};\n"
    "use semio_framework_os_infinite_board_normal_gestures::set_transform_flags;\n", 1)], dry=DRY)
replace(T + "/🧿️ui-engine-tests/🦀️.rs", [(
    "    use semio_framework_os_renderer_wgpu_canvas2d_host::ring;\n",
    "    use semio_framework_os_infinite_board_normal_focus::{set_camera_silent, set_selection_ids_silent};\n"
    "    use semio_framework_os_infinite_board_normal_gestures::{pointer_lane_in_flight, pointer_lane_is_direct};\n"
    "    use semio_framework_os_infinite_board_normal_host::begin_pointer_commit;\n"
    "    use semio_framework_os_renderer_wgpu_canvas2d_host::ring;\n", 1)], dry=DRY)
replace(T + "/🧷️engine-attach-tests/🦀️.rs", [(
    "    use semio_framework_os_node_graph::GraphHost;\n    use semio_framework_os_renderer_wgpu_canvas2d_host::local;\n",
    "    use semio_framework_os_infinite_board_normal_gestures::defers_descriptor_sync_from_js;\n    use semio_framework_os_node_graph::GraphHost;\n    use semio_framework_os_renderer_wgpu_canvas2d_host::local;\n", 1)], dry=DRY)
M = "/📦️packages/🦀️rust/Cargo.toml"
addep.add(T + "/🎲️engine-board" + M, "dependencies", deps("focus", "gestures", "host"), dry=DRY)
addep.add(T + "/🕸️engine-graph" + M, "dependencies", deps("dag-graph", "dag-host"), dry=DRY)
addep.add(T + "/🔗️engine-attach" + M, "dependencies", deps("dag-graph", "dag-host", "focus", "gestures"), dry=DRY)
addep.add(T + "/🧿️ui-engine-tests" + M, "dependencies", deps("focus", "gestures", "host"), dry=DRY)
addep.add(T + "/🧷️engine-attach-tests" + M, "dependencies", deps("gestures"), dry=DRY)
