from specs.lib import *
F = STORE + '/🔗️read/🦀️.rs'
CH = [
    ch('📖', 'derived-snapshots-replays-and-history-read-plan', 'derived_snapshots_replays_and_history_read_plan', [(54, 332)],
       'Derived snapshots, replays and history read plan: the derived snapshot, replay and history preview, the history read plan and the history read retirement.'),
    ch('🧮', 'cursor-ledger-and-vcs-retirements', 'cursor_ledger_and_vcs_retirements', [(356, 755)],
       'Cursor, ledger and VCS retirements: the revision accumulator, cursor and mutation DAG retirements, the history ledger and VCS retirements with their factories and the document envelope birth.'),
    ch('🗑', 'envelope-genesis-and-displaced-snapshot-retirements', 'envelope_genesis_and_displaced_snapshot_retirements', [(757, 1098)],
       'Envelope, genesis and displaced snapshot retirements: the envelope metadata and retirement, the genesis retirement, the displaced snapshot retirements with their capacity and the cursor disposer phase.'),
]
SPEC = dict(orig='rd.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='rd.rs'))
