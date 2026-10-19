from specs.lib import *
F = R + '/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs'
c = Cut(F, 'sstate.rs')
c.child('📐', 'viewport-drag-and-accessibility-controls', 'viewport_drag_and_accessibility_controls', [(8, 272)],
        'Viewport, drag and accessibility controls: the viewport, the scene drag and ink edit state, the host temporal presentation and the accessibility control records.')
c.child('🧯', 'surface-state-and-retirement', 'surface_state_and_retirement', [(274, 775)],
        'Surface state and retirement: the scene surface state, the value retirement, the surface retirement with its close step and the retired surface close.')
c.child('🖱', 'canvas-gestures-input-kit-and-chrome', 'canvas_gestures_input_kit_and_chrome', [(777, 1115)],
        'Canvas gestures, input kit and chrome: the canvas click and gesture records, the scene input kit and row transfer, the chrome labels, the engine hosts, the table accessibility cells and the tiled map retirement.')
c.child('✍', 'editor-ink-gumball-and-canvas-layer-wire', 'editor_ink_gumball_and_canvas_layer_wire', [(1121, 1509)],
        'Editor, ink, gumball and canvas layer wire: the text editor ui state, the ink bounds and outline, the vfs focus target, the gumball state and algebra and the canvas layer wire records.')
SPEC = c.spec()
