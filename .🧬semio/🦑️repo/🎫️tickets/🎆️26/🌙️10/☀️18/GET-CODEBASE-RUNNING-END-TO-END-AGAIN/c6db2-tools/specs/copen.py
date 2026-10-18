from specs.lib import *
F = DB + '/⚙️engine/🔭️capability-open/🦀️.rs'
CH = [
    ch('🎟', 'open-admission', 'open_admission', [(14, 126)],
       'Capability open admission: the slot, item and byte credits, the fixed admission state and the admission handle that releases its slot on drop.'),
    ch('🔑', 'open-results-and-rejections', 'open_results_and_rejections', [(128, 221)],
       'Capability open results and rejections: the progress and close steps, the open result and the rejected open.'),
    ch('🧱', 'open-state-and-lease', 'open_state_and_lease', [(223, 418)],
       'Capability open state and lease: the backend future, the work, the phase, the shared open state, its lease and the wake.'),
    ch('⚙', 'open-state-operations', 'open_state_operations', [(420, 958)],
       'Capability open state operations: the drive, submit, retry and completion operations of the shared open state.'),
    ch('📬', 'open-future-and-terminal-handles', 'open_future_and_terminal_handles', [(960, 1310)],
       'Capability open future and terminal handles: the open future, the terminal handle and result, the terminal takers and the live open counter.'),
]
SPEC = dict(orig='copen.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='copen.rs'))
