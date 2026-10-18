from specs.lib import *
TR = PL + '/⏯️tool-run'
CH = [
    ch('⏱', 'limits-and-job-slot', 'limits_and_job_slot', [(42, 58), (104, 278)],
       'Limits and job slot of the tool run ledger: the turn, refold, discard, panel and trace limits, the job site, the job handle and slot with their transitions and the trace lane helpers.'),
    ch('🔁', 'refold-finalize-and-entry', 'refold_finalize_and_entry', [(283, 607)],
       'Refold, finalize and entry of the tool run ledger: the refold, settings values and finalize phases and the per-run entry with its progress, trace and publication operations.'),
    ch('📒', 'driver-and-ledger', 'driver_and_ledger', [(609, 1118)],
       'Driver and ledger of the tool run ledger: the tick receipt, the driver, the ledger with its trace window and the ledger operations over the selected run.'),
    ch('👥', 'member-owners-and-visitors', 'member_owners_and_visitors', [(1122, 1439)],
       'Member owners and visitors of the tool run ledger: the composed member owners, the member run, fold, emission, generation and retire visitors and the erased retirement helpers.'),
    ch('🏷', 'action-outcome-and-labels', 'action_outcome_and_labels', [(1443, 1534)],
       'Action outcome and labels of the tool run ledger: the action outcome, the exact integer argument reader and the step, verdict and state labels.'),
    ch('🪧', 'panel-groups', 'panel_groups', [(1536, 1710)],
       'Panel groups of the tool run ledger: the ready group and the run panel group of the history panel.'),
]
SPEC = dict(orig='tr.rs', file=TR + '/🦀️.rs', children=CH, wiring='each', external=mounted(TR + '/🦀️.rs', orig='tr.rs'))
