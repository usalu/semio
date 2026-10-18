from specs.lib import *
F = NRM + '/🖼️scene/🦀️.rs'
CH = [
    ch('🧱', 'scene-and-style-state-impls', 'scene_and_style_state_impls', [(24, 172)],
       'Scene and style state impls of the board scene: the scene state and style state methods.'),
    ch('🖌', 'brush-icon-and-highlight-paint', 'brush_icon_and_highlight_paint', [(174, 269)],
       'Brush icon and highlight paint of the board scene: the brush node icon paint, the highlighted node preview and the drag delta.'),
    ch('🔘', 'handle-geometry', 'handle_geometry', [(271, 375)],
       'Handle geometry of the board scene: the handle positions JSON, effective scale and radius, world positions, cap attachment and the indirect handle marker radius.'),
    ch('🔗', 'link-compatibility-and-curves', 'link_compatibility_and_curves', [(377, 596)],
       'Link compatibility and curves of the board scene: the compatible link targets and handles, the free handle probes, edge and wire curves, the level of detail gates and node hits.'),
    ch('🔍', 'draw-lod-and-style-impl', 'draw_lod_and_style_impl', [(598, 1044)],
       'Draw level of detail and style impl of the board scene: the scene state helpers, the draw level of detail for a frame, the overlay paint state and the style state methods.'),
    ch('🧲', 'grid-snap-and-brush-candidates', 'grid_snap_and_brush_candidates', [(1046, 1230)],
       'Grid snap and brush candidates of the board scene: the current draw level of detail, grid snap steps, link rule gates, brush anchors and candidates and the region snap.'),
    ch('🧬', 'scene-types-and-delete-records', 'scene_types_and_delete_records', [(1232, 1396)],
       'Scene types and delete records of the board scene: the fill text bytes, the fill capture fault, the scene and style state, the wheel plan, the pointer plan fault and the delete and property audit records.'),
    ch('🗑', 'delete-plan-and-property-audit', 'delete_plan_and_property_audit', [(1398, 1641)],
       'Delete plan and property audit of the board scene: the delete plan methods and the property audit methods.'),
    ch('👆', 'pointer-plan', 'pointer_plan', [(1643, 1874)],
       'Pointer plan of the board scene: the pointer plan kinds and record, its methods, the JSON string byte count, the admitted pointer id and the wheel plan impl.'),
    ch('🧭', 'scene-state-impl', 'scene_state_impl', [(1877, 2104)],
       'Scene state impl of the board scene: the large scene state method block.'),
    ch('✂', 'culling-markers-and-snap', 'culling_markers_and_snap', [(2106, 2406)],
       'Culling, markers and snap of the board scene: the world bounds culls, handle markers, node icon paint, the wheel zoom flag, link snap tolerance and commit proximity, node drag proximity and the planned link edge id.'),
]
SPEC = dict(orig='scene.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='scene.rs'))
