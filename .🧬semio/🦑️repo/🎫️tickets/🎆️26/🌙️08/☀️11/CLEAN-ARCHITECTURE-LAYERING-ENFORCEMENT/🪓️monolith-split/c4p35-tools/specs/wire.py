from specs.lib import *

SPEC = dict(
    orig='wire.rs',
    file=WIRE + '/🦀️.rs',
    nest=False,
    keep_empty=('🧪️Tests',),
    external=[WIRE + '/🧪️tests/🔬️unit/🦀️.rs'],
    children=[
        ch('🥾', 'artifact-bootstrap', 'artifact_bootstrap', [(75, 412)],
           'Artifact bootstrap: the bootstrap limits and constants, the pair, the bootstrap, the rebootstrap request, the progress and control and the assembler.', glob=False),
        ch('🛰', 'server-frame', 'server_frame', [(414, 592)],
           'Server frames: the apply outcome, the ack stage, the snapshot chunk bytes and the server frame enum.'),
        ch('🧵', 'codec-combinators', 'codec_combinators', [(599, 808)],
           'Frame codec parts: the frontier and envelope vector combinators, the artifact bootstrap codec, the bootstrap, apply outcome and ack stage codecs.'),
        ch('🔌', 'frame-codec', 'frame_codec', [(811, 1043)],
           'Frame codec: the client and server frame encoders and decoders and the snapshot chunk readers.'),
    ],
)
