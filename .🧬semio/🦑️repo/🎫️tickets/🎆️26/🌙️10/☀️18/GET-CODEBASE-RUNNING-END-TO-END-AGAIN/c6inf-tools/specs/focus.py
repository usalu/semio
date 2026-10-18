from specs.lib import *
F = NRM + '/🎯️focus/🦀️.rs'
CH = [
    ch('🧲', 'focus-and-outbox-state-impls', 'focus_and_outbox_state_impls', [(20, 221)],
       'Focus and outbox state impls of the board focus: the focus state and the outbox state methods.'),
    ch('🖍', 'selection-staging-and-sync', 'selection_staging_and_sync', [(223, 426)],
       'Selection staging and sync of the board focus: staging and restoring selection ids, cancelling a node drag, background clicks, selection flags, select events and area preselect.'),
    ch('🔘', 'indirect-ring-and-link-targets', 'indirect_ring_and_link_targets', [(428, 578)],
       'Indirect ring and link targets of the board focus: the indirect ring node, sole handle hits, link target ring hits and the ring snapshot.'),
    ch('🔗', 'link-gesture-and-hit-resolution', 'link_gesture_and_hit_resolution', [(580, 856)],
       'Link gesture and hit resolution of the board focus: link gesture events, the active link wire curve and the world hover and hit resolution.'),
    ch('🧭', 'focus-state-impls', 'focus_state_impls', [(858, 1042)],
       'Focus state impls of the board focus: further methods of the focus state.'),
    ch('🎥', 'camera-style-kinds-and-overlay', 'camera_style_kinds_and_overlay', [(1044, 1220)],
       'Camera, style kinds and overlay of the board focus: external link preview, the camera setters, kind hover ids, style kind resolution, overlay entity ids and edge stroke paint.'),
    ch('🗳', 'selection-drag-and-area', 'selection_drag_and_area', [(1222, 1383)],
       'Selection drag and area of the board focus: draggable members, union bounds, bounded selection drag and area selection with an initial set.'),
    ch('🧬', 'focus-types-and-descriptor-sync', 'focus_types_and_descriptor_sync', [(1385, 1742)],
       'Focus types and descriptor sync of the board focus: the focus and outbox state structs, pointer intent, descriptor sync, scene clearing and snapshot loading.'),
    ch('🎨', 'paint-hover-and-link-commit', 'paint_hover_and_link_commit', [(1744, 2132)],
       'Paint, hover and link commit of the board focus: the indirect handle ring, node, handle and edge paint, icon spans, hover ids, wheel handling, link edge commit and snapshot JSON loading.'),
]
SPEC = dict(orig='focus.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='focus.rs'), inert_regions=True)
