from specs.lib import *
F = DB + '/⚙️engine/🥾️catalog-bootstrap/🦀️.rs'
CH = [
    ch('🎟', 'bootstrap-admission', 'bootstrap_admission', [(12, 116)],
       'Catalog bootstrap admission: the slot, item, page and byte credits, the fixed admission state and the admission handle that releases its slot on drop.'),
    ch('🔑', 'bootstrap-results-and-rejections', 'bootstrap_results_and_rejections', [(118, 425)],
       'Catalog bootstrap results and rejections: the bootstrap key, the progress and close steps, the terminal witness, the result, the rejected close owner and the rejected bootstrap with its close.'),
    ch('🧱', 'bootstrap-state-and-wake', 'bootstrap_state_and_wake', [(427, 613)],
       'Catalog bootstrap state and wake: the backend future, the work, the phase, the driver authority, the shared bootstrap state and the wake.'),
    ch('⚙', 'bootstrap-state-operations', 'bootstrap_state_operations', [(615, 1110)],
       'Catalog bootstrap state operations: the drive, submit, retry and completion operations of the shared bootstrap state.'),
    ch('📬', 'bootstrap-future-and-terminal-handles', 'bootstrap_future_and_terminal_handles', [(1112, 1424)],
       'Catalog bootstrap future and terminal handles: the bootstrap future, the terminal handle and result, the terminal taker and the live bootstrap counter.'),
]
SPEC = dict(orig='cboot.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='cboot.rs'))
