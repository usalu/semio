from specs.lib import *

SPEC = dict(
    orig='prs.rs',
    file=PRS + '/🦀️.rs',
    nest=False,
    external=[
        WIRE + '/🧪️tests/🔬️presence-codec/🦀️.rs',
        WIRE + '/🧪️tests/🔬️assemble-presence-interaction/🦀️.rs',
        M + '/📡️replication/🚪️io/👥️presence/🔍️borrowed/🦀️.rs',
        M + '/📡️replication/👕️peer-overlay/🦀️.rs',
    ],
    children=[
        ch('🪟', 'view-records-and-codec', 'view_records_and_codec', [(26, 269)],
           'Presence view: the window view, the view kind and the interface records with their value conversions and encoders.'),
        ch('👤', 'peer-records', 'peer_records', [(274, 719)],
           'Presence peer records: the peer, the typing excerpt, the principal kind, the tool run, the history edit and the value conversions of the peer.'),
        ch('📤', 'peer-wire-encode', 'peer_wire_encode', [(721, 888)],
           'Presence peer encoding: the peer, tool run and history edit encoders and the peer wire limits.'),
        ch('📥', 'peer-wire-decode', 'peer_wire_decode', [(890, 1126)],
           'Presence peer decoding: the peer reader and the peer, tool run and history edit decoders.'),
        ch('🕹', 'interaction-and-domain', 'interaction_and_domain', [(1136, 1300)],
           'Presence interaction: the interaction and domain records with their value conversions, the vector and domain codecs and the interaction assembler.'),
    ],
)
