from specs.lib import *
F = U + '/⌨️tui/⚙️engine/🦀️.rs'
H = 'impl Tui {'
B = [366, 586, 728, 961]
CH = [
    ch('🎯', 'construction-and-focus', 'construction_and_focus', [(132, 366)],
       'Construction and focus: the `Tui` constructor, frame size and width mode, the focus model and ring, the focus navigation and the cursor, hover and capture queries.'),
    ch('⏱', 'frame-layout-and-overlays', 'frame_layout_and_overlays', [(368, 589)],
       'Frame, layout and overlays: the tick, the dirty and deadline queries, the layout and chrome sync, and the menu, dialog, palette and tooltip overlays with their placement.'),
    ch('⌨', 'input-dispatch', 'input_dispatch', [(591, 734)],
       'Input dispatch: event dispatch and routing, key sanitising and dispatch, the modal gate and the click counting.'),
    ch('🖱', 'mouse-and-hover', 'mouse_and_hover', [(736, 970)],
       'Mouse and hover: mouse dispatch, move, down, drag and up handling, chrome presses and the hover target tracking.'),
    ch('🖌', 'paint-and-gutters', 'paint_and_gutters', [(972, 1054)],
       'Paint and gutters: the paint pass, the diffed and full renders and the gutter hit testing and painting.'),
]
SPEC = dict(orig='eng.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='eng.rs'))
