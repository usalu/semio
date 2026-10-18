from specs.lib import *
F = STORE + '/🚪️io/📝️text/🦀️.rs'
CH = [
    ch('🗒', 'text-files-parsed-document-and-ops-log', 'text_files_parsed_document_and_ops_log', [(38, 402)],
       'Text files, parsed document and ops log: the artifact text files, the parsed document text, the edit line print, the clocks, the ops lines, the loaded history replay, the supersession addresses and the ops log and document text print.'),
    ch('🛂', 'history-messages-and-persisted-validation', 'history_messages_and_persisted_validation', [(404, 865)],
       'History messages and persisted validation: the undo policy ordinals, the message and operation meta conversions, the persisted message, conflict, history and lane validation and the history edit and conflict conversions.'),
    ch('📦', 'document-spr-pack-and-history-composition', 'document_spr_pack_and_history_composition', [(867, 1143)],
       'Document spr, pack and history composition: the empty document spr, the identity stamp, the genesis member pack, the history events, the history composition, the spr print and parse and the pack print.'),
    ch('🔁', 'replay-ops-and-document-parsing', 'replay_ops_and_document_parsing', [(1145, 1433)],
       'Replay ops and document parsing: the replay of the ops over an envelope, the parsed envelope settlement and discard and the document text and pack parse.'),
]
SPEC = dict(orig='iot.rs', file=F, children=CH, wiring='block', rebase_self=True, external=crate_ext(STORE + '/🚪️io/🦀️.rs', F, orig='iot.rs', children=CH))
