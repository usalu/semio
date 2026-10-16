"""Field -> component map for the DagHost decomposition."""
COMPONENTS = {
 'viewport': 'size, device ratio, LOD and grid policy, theme and pan anchor of the canvas',
 'graph': 'the host snapshot, the board engine and the id maps between them, with pending wire edits',
 'pointer': 'widget drag, ghost node, pending click intents and the last pointer down',
 'paint': 'icon paint cache, computing progress, evaluation status and dimming',
 'minimap': 'the minimap widget state',
 'notes': 'the inline note editing state',
}
F = {}
def put(comp, names):
    for n in names.split():
        assert n not in F, n
        F[n] = comp
put('viewport', 'width height dpr last_screen_x last_screen_y wheel_zoom_active wheel_zoom_render_lod automatic_lod forced_draw_lod grid_visible grid_snap_enabled grid_factor canvas_theme pan_anchor')
put('graph', 'host_snapshot engine node_id_map handle_key_map handle_port_shape handle_port_visible edge_id_map edge_engine_ids edge_route_style pending_graph_edits journal_refusal')
put('pointer', 'widget_drag pending_port_insert ghost_node pending_cluster_explode pending_export_click pending_open_instance_id last_pointer_down_at_ms last_pointer_down_world last_pointer_down_node_id')
put('paint', 'icon_paint_cache computing_active computing_stale computing_active_anim_phase computing_stale_anim_phase node_eval_status unresolved_input_ports dimmed')
put('minimap', 'minimap_widget_visible minimap_widget_hovered minimap_widget_drag')
put('notes', 'editing_note caret_visible')
ORDER = list(COMPONENTS)
