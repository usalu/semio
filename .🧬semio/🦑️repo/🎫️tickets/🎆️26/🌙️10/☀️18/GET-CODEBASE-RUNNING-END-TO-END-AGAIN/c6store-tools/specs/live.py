from specs.lib import *
F = STORE + '/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs'
X = IS([(237, 5, [675, 903, 1385]), (1520, 5, [1872, 2132])])
CH = [
    ch('🧱', 'live-fold-and-parts-types', 'live_fold_and_parts_types', [(69, 217)],
       'Live fold and parts types: the incrementally maintained store fold, the replay state, the parts bundle and its borrowed view, the document root commit authority and the edit actor derivation.'),
    ch('📸', 'snapshot-reads-and-retained-replacements', 'snapshot_reads_and_retained_replacements', [X.rng(237, 674)],
       'Snapshot reads and retained replacements: the projection event, the derived snapshots and history previews, the snapshot reads and the retained replacement of roots, edit ids, checkpoints and the tail undo cache.'),
    ch('🧮', 'time-travel-reads-publication-and-supersede-inputs', 'time_travel_reads_publication_and_supersede_inputs', [X.rng(675, 902), X.rng(1385, 1518)],
       'Time travel reads, publication and supersede inputs: the supersessions and mutation outcomes, the report replay, the one item publication batches, the tool transactions and the supersede and draft inputs.'),
    ch('🎞', 'event-sourced-history-reprojection', 'event_sourced_history_reprojection', [X.rng(903, 1384)],
       'Event sourced history reprojection: the remote transition adoption, the reprojection, the live prefix state, the replayed projection adoption, the convergence checkpoints and the prefix snapshot ring.'),
    ch('📣', 'replay-invalidation-announcements-and-transactions', 'replay_invalidation_announcements_and_transactions', [X.rng(1520, 1871)],
       'Replay invalidation, announcements and transactions: the projection invalidation, the pending reports, the suffix replay, the announcement and admission of transitions, the append and the open transaction.'),
    ch('⏳', 'deferred-reprojection-and-supersede-reports', 'deferred_reprojection_and_supersede_reports', [X.rng(1872, 2131)],
       'Deferred reprojection and supersede reports: the deferred remote and local replays, the reprojection drive, the pending adoption, the replay reports and the merge report absorption.'),
    ch('🛰', 'history-lane-commands-and-outbound', 'history_lane_commands_and_outbound', [X.rng(2132, 2503)],
       'History lane commands and outbound: the local alternatives and checkpoints, the command application, the edit messages, the conflict resolution, the quarantined events and the outbound flushes.'),
    ch('🟰', 'convergence-early-exit-and-applied-mutation', 'convergence_early_exit_and_applied_mutation', [X.rng(2523, 2657)],
       'Convergence early exit and applied mutation: the convergence early exit of the live fold, the applied mutation as history editing addresses it, the transition retraction closure and the document backbone batches.'),
]
SPEC = dict(orig='live.rs', file=F, children=CH, wiring='block', pre=X.pre, expect_extra=X.extra_full(open(T + '/orig/live.rs', encoding='utf-8').read()), rebase_self=True,
            external=crate_ext(STORE + '/⚙️engine/🎞️live/🦀️.rs', F, orig='live.rs', children=CH))
