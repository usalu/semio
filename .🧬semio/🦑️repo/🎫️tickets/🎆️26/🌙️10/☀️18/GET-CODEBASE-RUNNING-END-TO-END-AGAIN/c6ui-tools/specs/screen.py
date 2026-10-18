from specs.lib import *
F = U + '/⌨️tui/📟️vt/🧱️screen/🦀️.rs'
H = 'impl VtScreen {'
B = [445, 709, 948, 1109]
CH = [
    ch('🧊', 'screen-grid-and-state', 'screen_grid_and_state', [(11, 178)],
       'Screen grid and state: the scrollback and match limits, the cell point, match, modes, charset, pen and saved cursor, the `Grid` and the `VtScreen` state with its default tab stops.'),
    ch('🏗', 'construction-view-and-resize', 'construction_view_and_resize', [(180, 445)],
       'Construction, view and resize: the `VtScreen` constructor, grid access, replies, the scrollback view anchoring, row access, blit and the reflowing resize.'),
    ch('🖍', 'cursor-motion-and-scrolling', 'cursor_motion_and_scrolling', [(447, 712)],
       'Cursor motion and scrolling: cursor clamping, erase spans, scroll up and down, line feeds, tab stops, charsets, printing with wrap, cursor moves and cursor save and restore.'),
    ch('🎚', 'erase-edit-and-modes', 'erase_edit_and_modes', [(714, 954)],
       'Erase, edit and modes: the scroll region, display, line and cell erase, line and cell insert and delete, the alignment test and the ansi and private modes.'),
    ch('🎨', 'alternate-screen-sgr-and-reports', 'alternate_screen_sgr_and_reports', [(956, 1118)],
       'Alternate screen, sgr and reports: entering and leaving the alternate screen, the cursor style, soft and hard reset, the sgr colours and attributes and the cursor, text area and colour reports.'),
    ch('🔎', 'text-extraction-and-search', 'text_extraction_and_search', [(1120, 1245)],
       'Text extraction and search: the cell text, text between points, character classes, word, line and url spans and the find.'),
]
SPEC = dict(orig='screen.rs', file=F, children=CH, wiring='block', nest=True, rebase_super=True, pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='screen.rs'))
