from specs.lib import *
F = W + '/⚡️events/🦀️.rs'
H = 'impl EventRouter {'
c = Cut(F, 'ev.rs').impl(H, [1773, 2090, 2602, 3113])
c.child('🧭', 'event-model-and-focus', 'event_model_and_focus', [(12, 447)],
        'Event model and focus: the pointer, accessibility and IME events, the hit test with the tree item lookup, the capture state and the focus ring.')
c.child('🧮', 'control-commit-rules', 'control_commit_rules', [(449, 901)],
        'Control commit rules: the fired actions, the edit commit and refusal rules for text, number, stepper and slider controls, the press bindings and the bubbling.')
c.child('🧲', 'overlays-drag-and-commands', 'overlays_drag_and_commands', [(903, 1366)],
        'Overlays, drag and commands: the overlay stack and placement, the tooltip and caret clock steps, the drag session, the text helpers, `UiCommand` and the `EventRouter` state.')
c.child('🏗', 'router-construction-and-intents', 'router_construction_and_intents', [(1368, 1772)],
        'Router construction and intents: the `EventRouter` constructor, the interaction transfer, the close step, the flow and tree drag settings, the intent API and the hover updates.')
c.child('⏱', 'tooltip-clock-and-overlay-api', 'tooltip_clock_and_overlay_api', [(1773, 2088)],
        'Tooltip clock and overlay API: the retained caret, the clock deadlines and advance, the row reason hints and the overlay open, close and dismiss methods.')
c.child('✍', 'drag-scroll-and-edit-api', 'drag_scroll_and_edit_api', [(2090, 2600)],
        'Drag, scroll and edit API: the drag payload and promotion, the scroll thumbs, the stepper and slider edit helpers and the text, IME and edit key routing.')
c.child('🖱', 'accessibility-and-pointer-dispatch', 'accessibility_and_pointer_dispatch', [(2602, 3112)],
        'Accessibility and pointer dispatch: the cursor queries, the accessibility dispatch and the pointer dispatch.')
c.child('⌨', 'widget-keyboard-and-commit', 'widget_keyboard_and_commit', [(3113, 3354)],
        'Widget keyboard and commit: the inline key mirror, the scene command, the focused activation, the select and value keys and the pointer commit.')
SPEC = c.spec()
