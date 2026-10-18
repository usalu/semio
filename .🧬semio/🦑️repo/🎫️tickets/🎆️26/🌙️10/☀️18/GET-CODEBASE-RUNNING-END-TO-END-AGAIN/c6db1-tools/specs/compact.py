from specs.lib import *
F = DB + '/🗜️compact/🦀️.rs'
CH = [
    ch('💰', 'budget-lease-retention-and-index', 'budget_lease_retention_and_index', [(41, 330)],
       'Budget, lease, retention and index: the compaction budget and lease, WAL retention, payload GC and index compaction.'),
    ch('📸', 'snapshot-consolidation', 'snapshot_consolidation', [(334, 682)],
       'Snapshot consolidation: the retained pages, their retirement and the snapshot consolidator.'),
    ch('🧊', 'cold-archive-and-compactor', 'cold_archive_and_compactor', [(686, 831)],
       'Cold archive and compactor: the cold archive build and the compactor.'),
    ch('🧮', 'job-limits-ledger-and-admission', 'job_limits_ledger_and_admission', [(835, 1105)],
       'Job limits, ledger and admission: the job limits, the backing ledger, admission and the segment and hash owners.'),
    ch('🏗', 'job-execution', 'job_execution', [(1107, 1622)],
       'Job execution: the horizons, the WAL, lease and snapshot steps and the retained execution.'),
    ch('🧵', 'job-state', 'job_state', [(1624, 2063)],
       'Job state: the driver authority, the terminal owners, the core, the state and its registry.'),
    ch('📬', 'job-result-future-and-rejection', 'job_result_future_and_rejection', [(2065, 2365)],
       'Job result, future and rejection: the job result, the future, the close of a rejected job and the rejection.'),
]
SPEC = dict(orig='compact.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='compact.rs'))
