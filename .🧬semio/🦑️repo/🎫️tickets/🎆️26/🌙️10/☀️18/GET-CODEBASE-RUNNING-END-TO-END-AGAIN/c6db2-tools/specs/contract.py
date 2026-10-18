from specs.lib import *
F = DB + '/🗿️artifact/🧬️contract/🦀️.rs'
CH = [
    ch('🏷', 'ids-command-and-messages', 'ids_command_and_messages', [(16, 80), (1005, 1061)],
       'Ids, command and messages: the core document and actor id bridges, the wire and dsl error mappers, the command batch and submit options, the recent touch limit and the artifact message.'),
    ch('🗺', 'pathmap-diff-and-bridge', 'pathmap_diff_and_bridge', [(83, 232)],
       'Pathmap, diff and bridge: the pathmap schema and codecs, the diff, inverse and touched entries and the envelope bridge from an operation.'),
    ch('⚔', 'conflict-records', 'conflict_records', [(235, 383)],
       'Conflict records: the conflict record, the command touch, the history transition codes, the recent touch memory and the concurrent write grading.'),
    ch('🧾', 'receipts-and-checkpoints', 'receipts_and_checkpoints', [(386, 596)],
       'Receipts and checkpoints: the command receipt, the checkpoint publication snapshot, the durable group journal append, decision and recovered checkpoint and the committed recovery state, advance and terminals.'),
    ch('🛡', 'durable-group-recovery', 'durable_group_recovery', [(598, 1002)],
       'Durable group recovery: the committed recovery operations, the decision from a transaction and the recovery target, driver and owner.'),
]
SPEC = dict(orig='contract.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='contract.rs'))
