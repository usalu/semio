"""Field -> component map for the World3dState decomposition."""
COMPONENTS = {
 'session': 'surface identity, bounds, domain binding, utilities and presentation of one 3D surface',
 'camera': 'orbit rig, projection, camera fit and camera sync',
 'selection': 'hover, selection, marquee and pick-target state',
 'gumball': 'gumball, brush preview, catalogue drop preview and relocate drag',
 'interaction': 'interaction mesh/object registries and instance interaction ids',
 'authority': 'the interaction authority state machine with its active cursor or job',
 'retirement': 'the in-flight dynamic retirement cursor',
 'scene': 'resident meshes, draws, scene records (vortices, references, lod, environment), placeholder meshes and dynamic retirement slots',
 'terrain': 'terrain style, tiles, session and build',
 'overlay': 'face overlay build, sub-element paint, scalar field and prepared status',
 'bridge': 'scene bridge, snapshot apply and scene digests',
 'assets': 'asset io authority, url bookkeeping and brush mesh announce',
 'trace': 'tool run trace layer',
}
F = {}
def put(comp, names):
    for n in names.split():
        assert n not in F, n
        F[n] = comp
put('session', 'interaction_revision snapshot_fault surface_id controller_id bounds pick_bounds bound_domain_id bound_domain_granularity_id modelling_locale declared_action_ids active_utility presentation')
put('camera', 'orbit projection_spec projection_frame_key projection_frame_zoom projection_selected projection_frame_owed projection_frame_policy camera_user_moved camera_sync fit_request fit_owner fit_applied_key fit_bounds_cursor scene_camera_digest resolved_lod_pick gizmo_hovered_tip')
put('selection', 'selection_method local_hover_id local_hover_granularity_id selected_ids hovered_vortex_id hovered_component_id hovered_component_object_id hovered_component_mode active_object_id granularity component_ids interaction_mode marquee_points marquee_active marquee_preview_ids paint_stroke_active pick_hover_key pick_targets selection_targets show_edges highlighted_instance_ids provisional_instance_ids disabled_instance_ids celebrating_instance_ids scene_selection_digest drag_object_id drag_object_z drag_last_position press_object_id right_press_point')
put('gumball', 'gumball_selection_ids gumball_config gumball_live_dispatch gumball_target gumball_handle gumball_pivot gumball_drag_anchor gumball_drag_start_vec gumball_preview_translate gumball_preview_angle gumball_preview_scale transform_mode brush_preview catalogue_drop_preview relocate')
put('authority', 'interaction_authority')
put('retirement', 'dynamic_retirement')
put('interaction', 'interaction_meshes interaction_objects instance_interaction_ids instance_interaction_granularity_ids')
put('scene', 'meshes mesh_versions mesh_appearances mesh_pool mesh_source_urls mesh_paint_textures reference_pixels dynamic_mesh_close dynamic_blocked_mesh dynamic_blocked_owner native_mesh_admission dynamic_fault scene_mesh_digests mesh_lod_catalog mesh_url_fallback')
put('scene', 'draws draw_rebuild retired_draws draw_generation geometry_generation instance_positions lod chunking visible_chunks environment vortices attractions target_volumes references engagement_preview')
put('terrain', 'terrain_style terrain_applied_signature terrain_session terrain_visible_tiles terrain_built_tiles pending_terrain_tile_urls terrain_tile_misses terrain_build terrain_pending terrain_revision')
put('scene', 'placeholder_generation placeholder_build')
put('overlay', 'face_overlay_build face_overlay_generation face_overlay_retired_generation face_overlay_colors face_overlay_applied_revision face_overlay_applied_draw_generation sub_element_paint scalar_field_status prepared_status')
put('bridge', 'scene_bridge scene_bridge_close scene_bridge_lease scene_bridge_retired scene_bridge_generation scene_bridge_digest snapshot_lease snapshot_apply  scene_document_lanes_digest')
put('assets', 'asset_io asset_generation asset_url_misses brush_mesh_announced brush_mesh_run brush_mesh_feedback_digest pending_glb_urls pending_image_urls')
put('trace', 'tool_run_trace tool_run_trace_visibility tool_run_trace_meshes_digest tool_run_trace_mesh_keys tool_run_trace_window_id')
ORDER = list(COMPONENTS)
