from specs.lib import *
F = DB + '/⚙️engine/🎫️handle/🦀️.rs'
CH = [
    ch('🎟', 'history-view-lease-and-journal-sink', 'history_view_lease_and_journal_sink', [(18, 155)],
       'History view, lease and journal sink: the history view, the handle lease, the compaction future and the journal sink.'),
    ch('📨', 'submit-admission-and-state', 'submit_admission_and_state', [(157, 594)],
       'Submit admission and state: the submit limits, the admission slots and the submit state machine.'),
    ch('📤', 'submit-future-and-terminal', 'submit_future_and_terminal', [(596, 785)],
       'Submit future and terminal: the submit future and its terminal job and work.'),
    ch('🚪', 'history-admission-and-wake', 'history_admission_and_wake', [(787, 1019)],
       'History admission and wake: the history limits, admission slots, outcome types, registry and wake.'),
    ch('📚', 'history-state', 'history_state', [(1021, 1388)],
       'History state: the history state machine.'),
    ch('🔭', 'history-future-and-terminals', 'history_future_and_terminals', [(1390, 1797)],
       'History future and terminals: the history future with its terminal handle, job, work, reservation and construction.'),
    ch('🤝', 'artifact-handle', 'artifact_handle', [(1799, 1989)],
       'Artifact handle: `ArtifactHandle` with its submit, history and compaction entry points.'),
]
SPEC = dict(orig='handle.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='handle.rs'))
