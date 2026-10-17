from specs.lib import *

CH = [
    ch('🕰', 'clock', 'clock', [(41, 92)],
       'Clock of the trace primitives: the installable monotonic microsecond source, its exact installer and the native and browser default readers.'),
    ch('📏', 'ceilings-and-diagnostics', 'ceilings_and_diagnostics', [(96, 201)],
       'Ceilings and diagnostics of the trace primitives: the interactive step and guest lifecycle ceilings, the soft targets, the stage enum and the runtime diagnostics switch.'),
    ch('📊', 'sample-rings-and-sites', 'sample_rings_and_sites', [(224, 385)],
       'Sample rings of the trace primitives: the bounded ring, the percentile ring, the labelled site registry and the omitted-sample counters.'),
    ch('🐕', 'timers-and-watchdog', 'timers_and_watchdog', [(389, 591)],
       'Timers and watchdog of the trace primitives: the step and callback timers, the contract violation records, the callback verdict and the watchdog with its violation ring.'),
    ch('📒', 'overrun-ledger', 'overrun_ledger', [(599, 702)],
       'Overrun ledger of the trace primitives: the sustained-overrun quarantine steps, the step quarantine and the step overrun ledger with its counters.'),
    ch('🧵', 'thread-roles-and-counters', 'thread_roles_and_counters', [(706, 877)],
       'Thread roles and counters of the trace primitives: the process-wide thread role registry and the worker, permit and queue counters.'),
    ch('🛰', 'operations-and-trace', 'operations_and_trace', [(881, 1050)],
       'Operations and trace of the trace primitives: the operation, generation and trace identifiers, the trace stage and event and the bounded trace event ring with its latency queries.'),
]

SPEC = dict(
    orig='trc.rs',
    file=TRC + '/🦀️.rs',
    children=CH,
    external=mounted(TRC + '/🦀️.rs', TRC + '/📦️packages/🦀️rust/🦀️.rs'),
)
