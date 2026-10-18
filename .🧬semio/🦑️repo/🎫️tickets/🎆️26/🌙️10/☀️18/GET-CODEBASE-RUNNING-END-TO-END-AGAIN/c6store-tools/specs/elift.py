from specs.lib import *
F = STORE + '/⚙️engine/📥️lifted/🏪️store/🦀️.rs'
X = IS([(372, 5, [782, 982, 1216]), (1592, 5, [1888])])
CH = [
    ch('🗒', 'draft-config-store-and-checkpoint-identity', 'draft_config_store_and_checkpoint_identity', [X.rng(83, 352)],
       'Draft and config store: the draft store alias, the checkpoint ancestors, the merge base, the alternative reconciliation, the config store alias, the artifact store with its constructors and the checkpoint identity.'),
    ch('🌱', 'runtime-seed-construction-and-owned-close', 'runtime_seed_construction_and_owned_close', [X.rng(372, 781)],
       'Runtime seed, construction and owned close: the preflight and adoption of the runtime seed, the store construction, the state and checkpoint pins and the owned demands and close steps.'),
    ch('🚦', 'cursor-admission-dispatch-and-apply-batch', 'cursor_admission_dispatch_and_apply_batch', [X.rng(782, 1215)],
       'Cursor admission, dispatch and apply batch: the cursor close admission, the structural owner checks, the dispatch of commands, the apply batch, the transition installs and the inner dispatch.'),
    ch('↩', 'supersede-undo-redo-and-backbone-pump', 'supersede_undo_redo_and_backbone_pump', [X.rng(1216, 1590)],
       'Supersede, undo, redo and backbone pump: the supersede commands, the replays, the authored supersessions, the undo and redo lanes, the text and binary dispatch, the backbone attachment and the pump.'),
    ch('🪞', 'delegating-accessors-and-reads', 'delegating_accessors_and_reads', [X.rng(1592, 1887)],
       'Delegating accessors and reads: the store accessors that delegate to the wire, cursor, log, owners and live components.'),
    ch('📑', 'delegating-operations-and-publication', 'delegating_operations_and_publication', [X.rng(1888, 2199)],
       'Delegating operations and publication: the store operations that delegate to the live, replay, cursor and log components.'),
    ch('🧯', 'close-host-drop-and-pair-merge', 'close_host_drop_and_pair_merge', [X.rng(2201, 2495)],
       'Close host, drop and pair merge: the close host, the history finalization and supersede scope, the store drop, the merge by id and the pair merge.'),
]
SPEC = dict(orig='elift.rs', file=F, children=CH, wiring='block', pre=X.pre, expect_extra=X.extra_full(open(T + '/orig/elift.rs', encoding='utf-8').read()), rebase_self=True,
            external=crate_ext(STORE + '/⚙️engine/🦀️.rs', F, orig='elift.rs', children=CH))
