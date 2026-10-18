from specs.lib import *
F = FILES['pr']
CH = [
    ch('🧱', 'board-event-limits-window-export-and-dispatch-work', 'board_event_limits_window_export_and_dispatch_work', [(28, 357)],
       'Board event limits, window, export and dispatch work: the board event batch limit and extent, the window command work, the revision hex, the export work, the dispatch emit and the generic and active example extents.'),
    ch('🕸', 'force-layout-work', 'force_layout_work', [(359, 795)],
       'Force layout work: the force layout limits and stages, the work owners and the force layout work with its stepping.'),
    ch('🎨', 'force-layout-command-and-redraw-handles-work', 'force_layout_command_and_redraw_handles_work', [(797, 1282)],
       'Force layout command and redraw handles work: the retained command implementation of the force layout, the redraw limits, stages and shape and the redraw handles work with its retained command implementation.'),
]
SPEC = dict(orig='pr.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='pr.rs'), keep_empty=empties('pr'))
