from specs.lib import *
F = NRM + '/🕹️gestures/🦀️.rs'
CH = [
    ch('🖌', 'brush-and-drop-preview', 'brush_and_drop_preview', [(21, 142)],
       'Brush and drop preview of the board gestures: entering a brush slot, rebuilding and moving the brush preview and the drop and brush preview paint.'),
    ch('🛤', 'pointer-lane-and-interaction', 'pointer_lane_and_interaction', [(144, 294)],
       'Pointer lane and interaction of the board gestures: the event authority emptiness, the pointer lane gates, the interaction JSON and the world pick target resolution.'),
    ch('🏁', 'closing-state-and-quarantine', 'closing_state_and_quarantine', [(296, 386)],
       'Closing state and quarantine of the board gestures: the closing state methods, the world content quarantine step and the brush state methods.'),
    ch('🎯', 'brush-slot-preview', 'brush_slot_preview', [(388, 586)],
       'Brush slot preview of the board gestures: slot centres and hit distances, the nearest slot source, preview building and sync events, slot clear, commit and finish and the alt update.'),
    ch('🌀', 'transform-drag', 'transform_drag', [(588, 667)],
       'Transform drag of the board gestures: the transform flags and the transform drag update, commit and cancel.'),
    ch('🗺', 'target-regions', 'target_regions', [(669, 917)],
       'Target regions of the board gestures: the target regions JSON, region grip hits, region drag and region paint gestures and the target region paint.'),
    ch('🧬', 'gesture-state-records', 'gesture_state_records', [(918, 1069)],
       'Gesture state records of the board gestures: the brush, gestures, commit and closing state, the close phase, delete operation, removed entity, property container and retirement records and the pointer commit operation.'),
    ch('🧰', 'commit-operation-and-retirements', 'commit_operation_and_retirements', [(1071, 1351)],
       'Commit operation and retirements of the board gestures: the pointer commit operation impl, the authority step and the property, entity, publication and plan retirement impls and the commit state.'),
    ch('🗑', 'delete-planning-step', 'delete_planning_step', [(1353, 1599)],
       'Delete planning step of the board gestures: the resumable delete planning step.'),
]
SPEC = dict(orig='gest.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='gest.rs'), inert_regions=True)
