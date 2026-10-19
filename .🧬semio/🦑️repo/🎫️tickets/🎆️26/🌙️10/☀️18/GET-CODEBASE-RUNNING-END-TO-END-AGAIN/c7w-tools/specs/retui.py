from specs.lib import *
F = R + '/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs'
c = Cut(F, 'retui.rs')
c.child('📕', 'close-scene-pointer-and-dispatch', 'close_scene_pointer_and_dispatch', [(40, 496)],
        'Close, scene pointer and dispatch: the ui document close queue, the scene targets, the pointer owner claims and cancellation, the clipped hit registration, the catalogue drag, the window clock and the ui event dispatch.')
c.child('♿', 'accessibility-dispatch-and-focus', 'accessibility_dispatch_and_focus', [(498, 819)],
        'Accessibility dispatch and focus: the accessibility event dispatch, the select check and the retained node focus kind and identity.')
c.child('📋', 'receipts-drop-and-clipboard', 'receipts_drop_and_clipboard', [(821, 1194)],
        'Receipts, drop and clipboard: the retained input receipts, the action publication, the drop target actions and payload, the clipboard paste handling, the pointer button codes and the scene ui command.')
SPEC = c.spec()
