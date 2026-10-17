from specs.lib import *

CH = [
    ch('🧱', 'riff-and-headers', 'riff_and_headers', [(70, 341)],
       'RIFF container and AVI headers: the chunk iterator and writers, the real-bytes sniffer and the main, stream and format header parsers and writers.'),
    ch('📖', 'decoder', 'decoder', [(345, 442)],
       'Decoder of the AVI codec: the RIFF walk that builds the snapshot of the main header, the streams and the chunks.'),
    ch('🎬', 'encode-avi-and-cursor-state', 'encode_avi_and_cursor_state', [(446, 579)],
       'Encoder entry and cursor state of the AVI codec: the one-shot encoder, the advance result, the phase machine and the resumable encode cursor record.'),
    ch('⏯', 'encode-cursor-steps', 'encode_cursor_steps', [(581, 983)],
       'Resumable encoding of the AVI codec: the steps of the encode cursor that emit the RIFF headers, the stream headers and the chunk payloads.'),
    ch('📏', 'encoded-sizes', 'encoded_sizes', [(985, 1117)],
       'Encoded sizes of the AVI codec: the chunk, list and payload totals and the header builders the cursor steps use.'),
]

SPEC = dict(
    orig='avi.rs.txt',
    file=AVI + '/🦀️.rs',
    nest=False,
    external=sub(AVI, skip=folders(CH)),
    outside=outside('avi'),
    children=CH,
)
