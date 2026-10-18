from specs.lib import *
F = DB + '/⚙️engine/🏗️catalog-create/🦀️.rs'
H = 'impl DatabaseCreateCatalogState {'
B = [1098, 1413]
CH = [
    ch('🧮', 'limits-ledger-and-admission', 'limits_ledger_and_admission', [(14, 156)],
       'Limits, ledger and admission: the create-catalog limits, the backing ledger and the admission slots.'),
    ch('📋', 'progress-result-and-rejection', 'progress_result_and_rejection', [(158, 496)],
       'Progress, result and rejection: the progress and close steps, the result, the rejected owner and close and the rejection.'),
    ch('🧵', 'work-cursor-phase-and-state', 'work_cursor_phase_and_state', [(498, 821)],
       'Work, cursor, phase and state: the work, the encode cursor, the phase, the driver authority, the state and its wake.'),
    ch('🚦', 'state-driver', 'state_driver', [(823, 1098)],
       'State driver: the phase accessors, scheduling, retries, callback close and driving steps of the state.'),
    ch('🔎', 'state-scan-clone-and-encode', 'state_scan_clone_and_encode', [(1100, 1416)],
       'State scan, clone and encode: the scan, candidate reservation, clone, snapshot, encode and seal steps of the state.'),
    ch('📌', 'state-claim-revalidate-and-publish', 'state_claim_revalidate_and_publish', [(1418, 1848)],
       'State claim, revalidate and publish: the claim, handoff, backend poll, revalidation, intermediate retirement, publish and terminal retirement of the state.'),
    ch('📤', 'future-terminal-and-state-drop', 'future_terminal_and_state_drop', [(1850, 2109)],
       'Future, terminal and state drop: the create-catalog future, its terminal handle, the live counter and the state drop.'),
]
SPEC = dict(orig='ccreate.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='ccreate.rs'))
