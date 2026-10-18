from specs.lib import *
F = DAG + '/🕸️graph/🦀️.rs'
H = 'impl DagGraphState {'
CUTS = [('fn journal_connect', H)]


def pre(t):
    t, _ = cut_impls(t, CUTS)
    return t


CH = [
    ch('🪪', 'identity-cursor-and-channel-row-paint', 'identity_cursor_and_channel_row_paint', [(26, 154)],
       'Identity cursor and channel row paint of the DAG graph: the identity cursor and the edge path, channel row highlight and divider and sided widget column paint of the graph state.'),
    ch('🧾', 'viewport-paint-and-overlay-json', 'viewport_paint_and_overlay_json', [(156, 278)],
       'Viewport paint and overlay JSON of the DAG graph: the viewport state methods, the node overlay JSON and the slider overlay state JSON.'),
    ch('🧲', 'graph-state-selection-and-engine-rebuild', 'graph_state_selection_and_engine_rebuild', [(280, 499)],
       'Graph state selection and engine rebuild of the DAG graph: widget positions, selection, hover, port insert hits, layout rebuilds and the engine edge sync.'),
    ch('📓', 'graph-state-journal-and-hits', 'graph_state_journal_and_hits', [(501, 790)],
       'Graph state journal and hits of the DAG graph: the connect and disconnect journal, graph edits, moves, port handle lookups and the widget, node and port hit tests.'),
    ch('🔭', 'viewport-lod-grid-and-camera', 'viewport_lod_grid_and_camera', [(792, 986)],
       'Viewport LOD, grid and camera of the DAG graph: the viewport settings, automatic and forced level of detail, grid steps and paint, channel hover and selection and the selection camera focus.'),
    ch('🧮', 'world-snap-engine-sync-and-hit-tests', 'world_snap_engine_sync_and_hit_tests', [(988, 1160)],
       'World snap, engine sync and hit tests of the DAG graph: grid snap helpers, screen to world conversion, engine position and event sync and the channel, anchor, port and connection hit tests.'),
    ch('🧬', 'state-records-and-screen-conversion', 'state_records_and_screen_conversion', [(1162, 1208)],
       'State records and screen conversion of the DAG graph: the viewport state, the graph state and the DAG screen and world conversions.'),
    ch('🎚', 'graph-state-interaction-impl', 'graph_state_interaction_impl', [(1210, 1514)],
       'Graph state interaction impl of the DAG graph: widget and node ids, hover and selection chrome, pointer plans, graph edit planning, selection domains and channel views.'),
    ch('🖱', 'hover-rect-and-pick-targets', 'hover_rect_and_pick_targets', [(1516, 1597)],
       'Hover, rect and pick targets of the DAG graph: the minimap pointer hover, world to screen points, the hovered node, entity screen rects and the pick targets at a screen point.'),
    ch('📐', 'content-bounds-and-align-selection', 'content_bounds_and_align_selection', [(1599, 1772)],
       'Content bounds and align selection of the DAG graph: content world bounds, the camera state, minimap bounds, snapshot node sync and the selection alignment.'),
    ch('🎞', 'camera-fit-and-entity-screen-json', 'camera_fit_and_entity_screen_json', [(1774, 1989)],
       'Camera fit and entity screen JSON of the DAG graph: content coverage, fitting and refitting the camera, the selection union bounds, the entity screen JSON and the retirement of both states.'),
]
SPEC = dict(orig='graph.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='graph.rs'), pre=pre, pre_extra=['}', H] * len(CUTS), inert_regions=True)
