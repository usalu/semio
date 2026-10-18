from specs.lib import *
MM = M + '/🧊️3d/🥽️mesh/🛠️modeling'
CR = M + '/🧊️3d/🦀️.rs'
CH = [
    ch('🧵', 'modeling-job', 'modeling_job', [(25, 258)],
       'Modeling job: the retained mesh modeling job with its work enum, its stepping and the tessellation handoff close.'),
    ch('🧰', 'modeling-operations', 'modeling_operations', [(260, 738)],
       'Modeling operations: the public mesh modeling operations trait with its halfedge mesh implementation and the internal operations trait.'),
    ch('🔄', 'transform-and-knife', 'transform_and_knife', [(740, 943)],
       'Transform and knife: the transform selection, the position policy, the transform work and the knife work.'),
    ch('🎈', 'inflate-orient-and-fill-holes', 'inflate_orient_and_fill_holes', [(945, 1216)],
       'Inflate, orient and fill holes: the inflate, orient and hole filling work records with their stepping.'),
    ch('🪢', 'loop-cut-mirror-and-subdivide', 'loop_cut_mirror_and_subdivide', [(1218, 1473)],
       'Loop cut, mirror and subdivide: the loop cut, mirror and subdivision work records with their stepping.'),
    ch('📉', 'bevel-and-decimate', 'bevel_and_decimate', [(1475, 1798)],
       'Bevel and decimate: the bevel and its profile, the decimation work and its candidate collapse records.'),
]
SPEC = dict(orig='mm.rs', file=MM + '/🦀️.rs', children=CH, wiring='block', nest=True, external=mounted(MM + '/🦀️.rs', orig='mm.rs'),
            crate_files=crate_files(CR, MM + '/🦀️.rs'))
