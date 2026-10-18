from specs.lib import *
F = STORE + '/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs'
X = IS([(1042, 5, [1381])])
CH = [
    ch('📇', 'receipts-envelope-view-and-history-columns', 'receipts_envelope_view_and_history_columns', [X.rng(72, 313)],
       'Receipts, envelope view and history columns: the command receipt, the artifact envelope view, the superseded input conversion, the history column layout over the checkpoint lanes and the mirror oracle constants.'),
    ch('🧩', 'store-component-structs', 'store_component_structs', [X.rng(336, 474)],
       'Store component structs: the store log, owners, wire and cursor components with the mint token and the log accessors.'),
    ch('📒', 'store-log-operations', 'store_log_operations', [X.rng(494, 1040)],
       'Store log operations: the clock, envelope reads, authoring verb, open transaction, operation wire, conflicts, time travel reads, history shape admission and the event log payloads of the store log.'),
    ch('🏗', 'store-owners-retained-reads-and-retirements', 'store_owners_retained_reads_and_retirements', [X.rng(1042, 1380), X.rng(2519, 2525)],
       'Store owners retained reads and retirements: the retained derived reads, the snapshot alias and read lease retirement, the pruned conflicts and the factory alias check of the store owners.'),
    ch('📬', 'store-owners-batch-fold-and-history', 'store_owners_batch_fold_and_history', [X.rng(1381, 1850)],
       'Store owners batch fold and history: the one item publication batch fold, the event sourced history and the remaining owner operations over the owned stage.'),
    ch('🖱', 'store-cursor-operations', 'store_cursor_operations', [X.rng(1969, 2499)],
       'Store cursor operations: the cursor demands, time travel reads, one item publication and event sourced history operations of the store cursor.'),
    ch('🗑', 'store-wire-scratch-owner-and-fold-history', 'store_wire_scratch_owner_and_fold_history', [X.rng(1852, 1967), X.rng(2545, 2624)],
       'Store wire, scratch owner and fold history: the durable group root reads of the store wire, the scratch owner with its deref and drop, and the history fold.'),
]
SPEC = dict(orig='core.rs', file=F, children=CH, wiring='block', pre=X.pre, expect_extra=X.extra_full(open(T + '/orig/core.rs', encoding='utf-8').read()), rebase_self=True,
            external=crate_ext(STORE + '/⚙️engine/🧱️core/🦀️.rs', F, orig='core.rs', children=CH))
