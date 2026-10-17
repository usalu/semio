from specs.lib import *

CH = [
    ch('🪪', 'limits-and-identifiers', 'limits_and_identifiers', [(12, 198)],
       'Limits and identifiers of the ABI: the version and size ceilings, the schema document, the operation and event codes, the request id, the handle and the byte newtypes.'),
    ch('📨', 'status-and-messages', 'status_and_messages', [(200, 380)],
       'Status and messages of the ABI: the status and error codes, the request, reply, event and page records and the control and message enums.'),
    ch('🧱', 'message-codec', 'message_codec', [(386, 611)],
       'Message codec of the ABI: the canonical versioned little-endian encoder and decoder of one message.'),
    ch('⏳', 'page-transfer', 'page_transfer', [(617, 923)],
       'Page transfer of the ABI: the work budget, the cursor step and cancel outcome and the resumable page writer and reader with the handle classifier.'),
    ch('🗃', 'ledgers-and-port', 'ledgers_and_port', [(929, 1111)],
       'Ledgers and port of the ABI: the generation-checked handle table, the reply ledger and the port poll, rejection and trait.'),
]

SPEC = dict(
    orig='abi.rs',
    file=ABI + '/🦀️.rs',
    children=CH,
    external=mounted(ABI + '/🦀️.rs'),
)
