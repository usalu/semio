from specs.lib import *
F = U + '/🖌️render/🖱️dispatch/🦀️.rs'
H = 'impl Dispatcher {'
B = [903, 1059, 1276]
CH = [
    ch('🎯', 'hit-testing', 'hit_testing', [(49, 176)],
       'Hit testing: the frame node, hitbox and focus ids, the dispatch flags and the hit test of a point against the hitbox tree.'),
    ch('🌿', 'listeners-and-dispatch-tree', 'listeners_and_dispatch_tree', [(182, 329)],
       'Listeners and dispatch tree: the typed `ListenerSet`, the `DispatchNode` and the frame-local `DispatchTree`.'),
    ch('🫧', 'events-bubble-and-capture', 'events_bubble_and_capture', [(337, 508)],
       'Events, bubble and capture: the pointer, ime and accessibility events, the `DispatchEvent`, the bubble walk and the pointer capture entries.'),
    ch('🪟', 'overlays-scroll-and-edit-state', 'overlays_scroll_and_edit_state', [(522, 673)],
       'Overlays, scroll and edit state: the overlay anchors and stack with their placement, the nearest scrollable ancestor and the edit state with its caret helpers.'),
    ch('🫳', 'drag-outcome-and-dispatcher-core', 'drag_outcome_and_dispatcher_core', [(681, 822), (824, 903)],
       'Drag, outcome and dispatcher core: the focusable collection, the drag payload, ghost and session, the cursor and ime requests, the `DispatchOutcome`, the `Dispatcher` state and its accessors and registration.'),
    ch('🔦', 'focus-and-overlay-api', 'focus_and_overlay_api', [(905, 1062)],
       'Focus and overlay api: the focus transitions and focus ring navigation and the overlay open, close and dismiss operations.'),
    ch('✍', 'drag-scroll-and-edit-routing', 'drag_scroll_and_edit_routing', [(1064, 1282)],
       'Drag, scroll and edit routing: the drag promotion, update and drop target resolution, the scroll routing and the text, ime and key edit routing with the clipboard intents.'),
    ch('🎬', 'fire-cursor-and-dispatch', 'fire_cursor_and_dispatch', [(1284, 1584)],
       'Fire, cursor and dispatch: the listener firing, the cursor resolution, the `Dispatcher::dispatch` entry, the target resolution and the hover update.'),
]
SPEC = dict(orig='dispatch.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='dispatch.rs'))
