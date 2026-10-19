from specs.lib import *
F = R + '/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🦀️.rs'
c = Cut(F, 'ngraph.rs')
c.child('🎥', 'scene-camera-and-drag-drop', 'scene_camera_and_drag_drop', [(28, 325)],
        'Scene camera and drag drop: the component scene note edit helpers, the node graph camera fit, catalogue and wheel zoom, the drag payloads and ghosts and the flow widget and catalogue drop actions.')
c.child('🖱', 'wheel-and-pointer-into', 'wheel_and_pointer_into', [(327, 567)],
        'Wheel and pointer into: the wheel, pointer down, move, up and cancel writers, the bounded publish, the graph edit planning, the interaction snapshot and the connection cursor.')
c.child('🔗', 'screen-pointer-and-commit', 'screen_pointer_and_commit', [(569, 872)],
        'Screen pointer and commit: the screen path gesture, the screen pointer outcome and application, the bounded edit rows, the pointer planning and commit and the interaction snapshot.')
c.child('📌', 'interaction-dispatch', 'interaction_dispatch', [(874, 1054)],
        'Interaction dispatch: the payload digest, the interaction dispatch, the target prefixes, the geometry census and the interaction action writers.')
c.child('🏷', 'labels-and-overlays', 'labels_and_overlays', [(1056, 1309)],
        'Labels and overlays: the label chrome, the legible font clamp, the overlay rows, the node graph labels, the overlay snapshot and the selection marquee and bounds.')
SPEC = c.spec(nest=True)
