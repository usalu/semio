from specs.lib import *
F = DB + '/⚙️engine/📖️catalog-read/🦀️.rs'
CH = [
    ch('🎟', 'read-admission', 'read_admission', [(13, 110)],
       'Catalog read admission: the slot, item and byte credits, the fixed admission state and the admission handle that releases its slot on drop.'),
    ch('🔑', 'read-results-and-rejections', 'read_results_and_rejections', [(112, 303)],
       'Catalog read results and rejections: the root key, the progress and close steps, the read result and the rejected read with its close.'),
    ch('🧱', 'read-state-and-lease', 'read_state_and_lease', [(305, 513)],
       'Catalog read state and lease: the backend future, the work, the phase, the shared read state, its lease and the wake.'),
    ch('⚙', 'read-state-operations', 'read_state_operations', [(515, 1014)],
       'Catalog read state operations: the drive, submit, retry and completion operations of the shared read state.'),
    ch('📬', 'read-future-and-terminal-handles', 'read_future_and_terminal_handles', [(1016, 1331)],
       'Catalog read future and terminal handles: the read future, the terminal handle and result and the live read counter.'),
]
SPEC = dict(orig='cread.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='cread.rs'))
