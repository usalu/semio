from specs.lib import *
EU = B3 + '/🛠️operations/🔺️euler'
CR = B3 + '/🛠️operations/🦀️.rs'
CH = [
    ch('🏗', 'topology-editors', 'topology_editors', [(35, 354)],
       'Topology editors: the vertex, edge, face, shell and solid makers, the solid scaffold retirement, the edge splits and the planar face split by a line.'),
    ch('🪡', 'boundary-splice', 'boundary_splice', [(360, 517)],
       'Boundary splice: the boundary vertex splice by point and by UV with the loop ring index and chain endpoint helpers.'),
    ch('⛓', 'face-chain-splits', 'face_chain_splits', [(519, 787)],
       'Face chain splits: the face splits by chain, interior curve and interior chain with their chord p-curve and signed area helpers.'),
    ch('🪚', 'seam-crossing-and-edge-merge', 'seam_crossing_and_edge_merge', [(789, 931)],
       'Seam crossing and edge merge: the face split by seam crossing and the edge kill that merges the two faces.'),
    ch('🗂', 'uv-arrangement', 'uv_arrangement', [(937, 1106)],
       'UV arrangement: the boundary hit record, the UV segment and line intersection, the edge parameter lookup and the loop walks with their p-curve chains.'),
]
SPEC = dict(orig='eu.rs', file=EU + '/🦀️.rs', children=CH, wiring='block', external=mounted(EU + '/🦀️.rs', orig='eu.rs'),
            crate_files=crate_files(CR, EU + '/🦀️.rs'))
