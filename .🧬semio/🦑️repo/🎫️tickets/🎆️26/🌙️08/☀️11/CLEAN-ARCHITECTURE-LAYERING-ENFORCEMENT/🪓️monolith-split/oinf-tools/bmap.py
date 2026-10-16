"""Field -> component map for the BoardHost decomposition."""
COMPONENTS = {
 'scene': 'entities, camera, viewport and the cached world content of one board',
 'focus': 'selection, preselect, hover and highlight state with the active utility',
 'style': 'style catalogs, themes, LOD and grid policy',
 'brush': 'brush candidates, previews and suggestion state',
 'outbox': 'the outgoing board event queue and its emit dedupe signatures',
 'gestures': 'transform, region and link gesture state',
 'commit': 'pending pointer commit and delete operations',
 'closing': 'close and opaque scene retirement state',
}
F = {}
def put(comp, names):
    for n in names.split():
        assert n not in F, n
        F[n] = comp
put('scene', 'camera nodes handles edges wires regions width height dpr content_scene_generation world_content_cache interaction_revision')
put('focus', 'selection preselect preselect_removed selection_exit_highlight selection_options hovered_id hovered_kind highlighted_ids interaction selection_screen_preview selection_preview_crossing active_utility')
put('style', 'handle_kinds wire_kinds node_kinds edge_kinds edge_tips link_compat_rules canvas_theme grid_visible grid_factor grid_snap_enabled preserve_original_element_style automatic_lod forced_draw_lod icon_paint_cache world_raster_tiling wheel_zoom_active wheel_zoom_render_lod port_mode naming placement_naming')
put('brush', 'suggestion_offset brush_node_size brush_slot_source_id brush_candidates brush_candidate_index brush_preview drop_preview brush_candidates_emit_key brush_preview_emit_key brush_placement_serial brush_node_kind_weights brush_handle_kind_weights brush_alt_pressed brush_slot_suggestions_active')
put('outbox', 'events event_overflow event_batch_overflow event_schema_fault link_compat_nodes_emit_key link_target_ring_emit_key last_select_emit_sig last_preselect_emit_sig gesture_serial')
put('gestures', 'transform_flags transform_drag region_drag region_paint area_brush_extent link_screen_preview')
put('commit', 'pending_delete_planning pending_delete_operation pending_pointer_commit queued_pointer_commit cancelled_pointer_plan_retirement pointer_publication')
put('closing', 'close_phase close_entity_retirement close_strings close_string_len close_node_handles opaque_scene_retirement opaque_scene_fault')
ORDER = list(COMPONENTS)
