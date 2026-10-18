from specs.lib import *
F = DAG + '/🧭️host/🦀️.rs'
CH = [
    ch('🪟', 'wasm-session-and-input-application', 'wasm_session_and_input_application', [(34, 270)],
       'WASM session and input application of the DAG host: the wasm-only DAG session and its methods and the input application with its retirement.'),
    ch('🏠', 'host-core-impl', 'host_core_impl', [(272, 517)],
       'Host core impl of the DAG host: the large host method block with construction, loading and graph access.'),
    ch('🖍', 'paint-state-and-palette', 'paint_state_and_palette', [(519, 627)],
       'Paint state and palette of the DAG host: the paint state methods, the canvas palette setter and the label overlay rows of a node spec.'),
    ch('🎨', 'node-visual-paint', 'node_visual_paint', [(629, 845)],
       'Node visual paint of the DAG host: painting one node with its body, label, handles, status border and chrome.'),
    ch('🧱', 'host-and-state-impls', 'host_and_state_impls', [(847, 1015)],
       'Host and state impls of the DAG host: further host methods and the paint, pointer and notes state methods.'),
    ch('📝', 'statuses-hits-and-note-editing', 'statuses_hits_and_note_editing', [(1017, 1283)],
       'Statuses, hits and note editing of the DAG host: dimming, computing progress, node statuses, connection cursor, screen hits, note editing and the widget pointer press.'),
    ch('🧬', 'host-state-records-and-sources', 'host_state_records_and_sources', [(1285, 1369)],
       'Host state records and sources of the DAG host: the selection source, the host and state structs, the pending open instance id and the output candidates.'),
    ch('👆', 'host-pointer-impl', 'host_pointer_impl', [(1371, 1482)],
       'Host pointer impl of the DAG host: the pointer method block and the pointer plan derivation.'),
    ch('🧮', 'projection-and-geometry-census', 'projection_and_geometry_census', [(1484, 1622)],
       'Projection and geometry census of the DAG host: the bounded node hit, the down and move projections, the screen geometry census JSON and the first screen point probe.'),
    ch('🗺', 'minimap-widget-and-retirement', 'minimap_widget_and_retirement', [(1624, 1765)],
       'Minimap widget and retirement of the DAG host: the minimap state methods, widget layout, hit, cursor, JSON and paint, the state retirements and the host retirement.'),
]
SPEC = dict(orig='dhost.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='dhost.rs'), inert_regions=True)
