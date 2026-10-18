from specs.lib import *
F = STORE + '/🔁️replay/🦀️.rs'
CH = [
    ch('🧮', 'effective-operations-and-folds', 'effective_operations_and_folds', [(38, 278)],
       'Effective operations and folds: the pair merge, the effective operation with its drop, the supersession aware effective operations, the replacement admission and the effective edit, operation and recorded folds.'),
    ch('🔢', 'prefix-digests-replay-progress-and-edit-replay', 'prefix_digests_replay_progress_and_edit_replay', [(286, 548), (1147, 1170)],
       'Prefix digests, replay progress and edit replay: the ledger order, the shared prefix digests, the superseded positions, the draft supersessions, the replay mode, steps and convergence, the edit replay with its result and the prefix snapshot ring limits.'),
    ch('🧱', 'replay-owned-state-and-result-types', 'replay_owned_state_and_result_types', [(550, 616), (1044, 1145)],
       'Replay owned state and result types: the replay owned state with its constructor and the replay result owned state.'),
    ch('⏩', 'replay-owned-state-operations', 'replay_owned_state_operations', [(618, 1042)],
       'Replay owned state operations: the owned replay steps, the retirement and the drive of the replay owned state.'),
]
SPEC = dict(orig='rpl.rs', file=F, children=CH, wiring='block', rebase_self=True, external=mounted(F, orig='rpl.rs'))
