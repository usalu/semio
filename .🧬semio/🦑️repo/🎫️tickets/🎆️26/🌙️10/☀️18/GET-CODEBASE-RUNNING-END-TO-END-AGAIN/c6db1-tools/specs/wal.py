from specs.lib import *
F = DB + '/📝️wal/🧾️log/🦀️.rs'
CH = [
    ch('🏷', 'record-kinds-and-field-codec', 'record_kinds_and_field_codec', [(28, 264)],
       'Record kinds and field codec: the record kind constants, the varint and field codecs and the cursor control.'),
    ch('🧱', 'wal-bytes', 'wal_bytes', [(266, 499)],
       'WAL bytes: the owned bytes, their cursor and rejection and the payload reference.'),
    ch('🧾', 'wal-record', 'wal_record', [(501, 859)],
       'WAL record: `WalRecord`, the record batch and the record encoding.'),
    ch('📤', 'payload-transform-group-commit-and-sink', 'payload_transform_group_commit_and_sink', [(862, 1133)],
       'Payload transform, group commit and sink: the payload transform, the group commit policy, the tail window and the shared buffer sink.'),
    ch('📖', 'page-source-reader-and-frames', 'page_source_reader_and_frames', [(1136, 1540)],
       'Page source, reader and frames: the page sources, the page reader, the retained record decoder and the verified frame steps.'),
    ch('⛓', 'transaction-gate-and-segment-chain', 'transaction_gate_and_segment_chain', [(1542, 1823)],
       'Transaction gate and segment chain: the transaction gate, the recovery report and the segment chain.'),
    ch('🔐', 'authenticated-source-and-replay-cursor', 'authenticated_source_and_replay_cursor', [(1825, 2117)],
       'Authenticated source and replay cursor: the authenticated source and the replay cursor.'),
    ch('✅', 'committed-replay-and-validation', 'committed_replay_and_validation', [(2119, 2463)],
       'Committed replay and validation: the committed cursor, the turn, committed replay, retained page scans and prefix validation.'),
]
SPEC = dict(orig='wal.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='wal.rs'))
