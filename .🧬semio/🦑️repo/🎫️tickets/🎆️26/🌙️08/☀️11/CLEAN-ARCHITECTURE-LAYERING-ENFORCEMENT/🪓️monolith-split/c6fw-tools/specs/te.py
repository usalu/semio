from specs.lib import *
TE = B3 + '/💡️queries/🧩tessellation'
CR = B3 + '/💡️queries/🧩tessellation/🦀️.rs'
CH = [
    ch('⏱', 'tessellation-job', 'tessellation_job', [(130, 435)],
       'Tessellation job: the resumable tessellation phases, progress, steps and input, the topology cursor and the tessellation job with its stepping.'),
    ch('🧵', 'edge-sampling', 'edge_sampling', [(441, 567)],
       'Edge sampling: the single pass edge discretization with its uniform and adaptive curve sampling, polyline helpers and curve and surface kinds.'),
    ch('🧊', 'face-meshing', 'face_meshing', [(573, 923)],
       'Face meshing: the face mesh assembly with its loop UV collection, seam and pole handling, the tessellation report measurement, vertex normals, winding fix and vertex welding.'),
    ch('🔺', 'triangulation-and-refinement', 'triangulation_and_refinement', [(929, 1333)],
       'Triangulation and refinement: the constrained Bowyer-Watson triangulation with its edge recovery and geometric predicates and the adaptive interior refinement.'),
]
SPEC = dict(orig='te.rs', file=TE + '/🦀️.rs', children=CH, wiring='block', external=mounted(TE + '/🦀️.rs', orig='te.rs'),
            crate_files=crate_files(CR, TE + '/🦀️.rs'))
