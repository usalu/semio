from specs.lib import *
TT = PL + '/⏪️time-travel'
CH = [
    ch('⏱', 'limits-and-outcomes', 'limits_and_outcomes', [(64, 148)],
       'Limits and outcomes of the time-travel session: the replay and refresh budgets, the planning and frame retirement grants, the action refusals and the action outcome.'),
    ch('✏', 'editor-and-member-subjects', 'editor_and_member_subjects', [(151, 217)],
       'Editor and member subjects of the time-travel session: the editor, the composed member subject and metadata and the review children.'),
    ch('📒', 'time-travel-ledger', 'time_travel_ledger', [(219, 629)],
       'Time-travel ledger: the per-instance ledger hosting the pure session reducer, the supersede authoring records and the ledger operations.'),
    ch('🗄', 'store-state', 'store_state', [(636, 1058)],
       'Store state of the time-travel session: the store-free owners trait, the per-store history-edit state, the replay, preview and commit steps and the typed store commands, outputs and queries.'),
    ch('🧱', 'store-owners-and-member-visitors', 'store_owners_and_member_visitors', [(1060, 1212)],
       'Store owners and member visitors of the time-travel session: the owners implementation of the store state, the composed member run, commit, settle and query visitors and the selection and generation argument readers.'),
    ch('🔀', 'supersession-ledger', 'supersession_ledger', [(1216, 1457)],
       'Supersession ledger of the time-travel session: the supersede role, record and ledger with their labels and per-mutation overlay.'),
]
SPEC = dict(orig='tt.rs', file=TT + '/🦀️.rs', children=CH, wiring='each', external=mounted(TT + '/🦀️.rs', orig='tt.rs'))
