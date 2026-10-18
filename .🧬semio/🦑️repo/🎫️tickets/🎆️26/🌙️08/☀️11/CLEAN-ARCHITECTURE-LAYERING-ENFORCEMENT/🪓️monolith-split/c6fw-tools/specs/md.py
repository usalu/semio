from specs.lib import *
MD = M + '/🧊️3d/🥽️mesh/📄️document'
CH = [
    ch('🧱', 'mesh-records', 'mesh_records', [(67, 196)],
       'Mesh records: the vertex, half-edge, face and transfer records, the halfedge mesh struct and their retirement owners.'),
    ch('🕸', 'halfedge-mesh-operations', 'halfedge_mesh_operations', [(198, 574)],
       'Halfedge mesh operations: the primitives, transforms, edits and UV operations of the halfedge mesh.'),
    ch('🔺', 'polygon-triangulation', 'polygon_triangulation', [(580, 805)],
       'Polygon triangulation: the vector helpers, the plane basis, the indexed polygon triangulation with holes and bridges and the face loop merge.'),
    ch('⏱', 'tessellation-job-and-obj-export', 'tessellation_job_and_obj_export', [(835, 1169), (1217, 1221)],
       'Tessellation job and OBJ export: the mesh tessellation job with its steps and retirement, the halfedge mesh tessellation entry points and the OBJ export.'),
]
SPEC = dict(orig='md.rs', file=MD + '/🦀️.rs', children=CH, wiring='block', external=mounted(MD + '/🦀️.rs', orig='md.rs'),
            crate_files=[], keep_empty=('Primitives', 'Tests'))
