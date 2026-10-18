from specs.lib import *
F = DB + '/🗿️artifact/⚙️engine/🦀️.rs'
H = 'impl ArtifactEngine {'
B = [998, 1268, 1456]
CH = [
    ch('🧾', 'wal-batch-state-retirement-and-document-state', 'wal_batch_state_retirement_and_document_state', [(24, 398)],
       'WAL batch, state retirement and document state: the WAL batch helpers, the state retirement reservations and `DocumentState`.'),
    ch('⚙', 'config-index-backlog-and-open-rejection', 'config_index_backlog_and_open_rejection', [(401, 697)],
       'Config, index backlog and open rejection: the engine config and struct, the index backlog and consistency resolver and the open rejection.'),
    ch('🌱', 'engine-lifecycle', 'engine_lifecycle', [(699, 998)],
       'Engine lifecycle: close, park, create, open and assemble of the engine.'),
    ch('🚦', 'engine-submit-pipeline', 'engine_submit_pipeline', [(1000, 1271)],
       'Engine submit pipeline: planning, committing, history transition refusal and `submit`.'),
    ch('↩', 'engine-undo-durable-group-query-and-preview', 'engine_undo_durable_group_query_and_preview', [(1273, 1635)],
       'Engine undo, durable group, query and preview: receipts, index maintenance, undo, durable group decisions, compaction, snapshots, queries, conflict advice and previews.'),
    ch('📖', 'state-query-source-and-pages', 'state_query_source_and_pages', [(1638, 1909)],
       'State query source and pages: the state query source and the state page codecs.'),
    ch('📈', 'ledger-tail', 'ledger_tail', [(1912, 2077)],
       'Ledger tail: the ledger points and commits and `artifact_ledger_tail`.'),
]
SPEC = dict(orig='aeng.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='aeng.rs'))
