from specs.lib import *
F = R + '/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs'
c = Cut(F, 'dock.rs').impl('impl DockState {', [494])
c.child('🧱', 'dock-types-and-window-state', 'dock_types_and_window_state', [(20, 493)],
        'Dock types and window state: the dock path, the stack tab and node, the dock state, the drag types and the window management, split drag and frame rect methods of `DockState`.')
c.child('🎯', 'dock-state-render-and-drop', 'dock_state_render_and_drop', [(494, 816)],
        'Dock state render and drop: the render view, the window activation, the drop, the tab reorder and insertion methods of `DockState`.')
c.child('📐', 'layout-conversion-and-drop-zones', 'layout_conversion_and_drop_zones', [(821, 1201)],
        'Layout conversion and drop zones: the conversion between window layouts and dock nodes, the layout edits, the split and drop zone geometry and the stack tab bars.')
c.child('🏷', 'tab-chrome-and-layout-edits', 'tab_chrome_and_layout_edits', [(1203, 1518)],
        'Tab chrome and layout edits: the device widths, the tab chips and actions, the corner tab bars, the layout normalisation, the window removal and the dock node diff.')
c.child('🧮', 'axis-solving-and-stack-rendering', 'axis_solving_and_stack_rendering', [(1520, 1810)],
        'Axis solving and stack rendering: the axis slots, the node bounds solver, the node and axis rendering, the resize hits and the stack rendering.')
c.child('🔲', 'window-silhouette-and-stack-geometry', 'window_silhouette_and_stack_geometry', [(1812, 2140)],
        'Window silhouette and stack geometry: the window silhouette spans, edges and borders, the stack cap layout, the path parsing and the stack frame and body collection.')
SPEC = c.spec()
