from specs.lib import *
F = DB + '/📸️snapshot/🦀️.rs'
CH = [
    ch('📇', 'snapshot-descriptor', 'snapshot_descriptor', [(54, 293)],
       'Snapshot descriptor: the descriptor format limits, the snapshot descriptor with its encoding and the varint, segment and field writers and readers.'),
    ch('🧵', 'snapshot-segment-io', 'snapshot_segment_io', [(297, 493)],
       'Snapshot segment io: the pack sub sources, the descriptor reader and decoder, the page sink and the page sources.'),
    ch('📚', 'snapshot-generation-pages', 'snapshot_generation_pages', [(497, 935)],
       'Snapshot generation pages: the publication claims, the generation page source and retained expectation builders, the generation handle and the generation open and read operations.'),
    ch('🗂', 'snapshot-manager-and-lease', 'snapshot_manager_and_lease', [(939, 1368), (1372, 1410)],
       'Snapshot manager and lease: the origin, body, retained publication and policy, the manager, its chain cursor and cursor control, and the snapshot lease.'),
]
SPEC = dict(orig='snap.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='snap.rs'))
