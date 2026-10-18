from specs.lib import *
TO = B3 + '/📸️representation/🕸️topology'
CR = B3 + '/📸️representation/🦀️.rs'
CH = [
    ch('📒', 'entity-records', 'entity_records', [(24, 129)],
       'Entity records: the vertex, edge, coedge, loop, face, shell and solid records with their retirement, the coedge use probe and the loop ring builders.'),
    ch('🗄', 'wire-and-body-arenas', 'wire_and_body_arenas', [(135, 272)],
       'Wire and body arenas: the wire record, the body with its arenas and retirement owners, its traversal accessors and the remap type.'),
    ch('🌱', 'arena-seed', 'arena_seed', [(278, 468)],
       'Arena seed: the seed records of every entity kind, the body arena seed and the body construction from a seed.'),
    ch('🧭', 'reachability-walk', 'reachability_walk', [(651, 794)],
       'Reachability walk: the arena entity reference, the reach set with its marks and the resumable reachability job.'),
    ch('🔗', 'merge-and-remap', 'merge_and_remap', [(796, 1035)],
       'Merge and remap: the remap, entity count and merge map records with the body merge, compaction remap and counting operations.'),
]
SPEC = dict(orig='to.rs', file=TO + '/🦀️.rs', children=CH, wiring='block', external=mounted(TO + '/🦀️.rs', orig='to.rs'),
            crate_files=crate_files(CR, TO + '/🦀️.rs'), keep_empty=('🔖️Tests',))
