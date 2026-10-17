from specs.lib import *

CH = [
    ch('🆔', 'connection-identity-and-limits', 'connection_identity_and_limits', [(23, 95)],
       'Connection identity and limits of the bridge session: the connection id, the per-connection entry, the agent inbox message and the outbox, broadcast and retirement ceilings.'),
    ch('📢', 'broadcast-cursor', 'broadcast_cursor', [(97, 196)],
       'Broadcast cursor of the bridge session: the recipient state, the broadcast cursor, its steps and its completion.'),
    ch('⏳', 'async-authority', 'async_authority', [(198, 511)],
       'Async authority of the bridge session: the async state, the async authority and its scheduling, retry, drive and broadcast completion paths.'),
    ch('🎞', 'encoded-frames', 'encoded_frames', [(513, 694)],
       'Encoded frames of the bridge session: the encoded frame with its page leases and release, and the encoded lease.'),
    ch('📮', 'outbox', 'outbox', [(696, 882)],
       'Outbox of the bridge session: the outbox item, grant and rejected publish records, the outbox state, the outbox and its receiver.'),
    ch('🤝', 'handle', 'handle', [(884, 1158)],
       'Handle of the bridge session: the shared inner state, the bridge handle with its connection, outbox and agent inbox operations and the frame send helpers.'),
    ch('💬', 'agent-conversation', 'agent_conversation', [(1164, 1230)],
       'Agent conversation of the bridge session: the conversation record and its turn operations.'),
]

SPEC = dict(
    orig='ses.rs',
    file=SES + '/🦀️.rs',
    children=CH,
    external=[],
)
