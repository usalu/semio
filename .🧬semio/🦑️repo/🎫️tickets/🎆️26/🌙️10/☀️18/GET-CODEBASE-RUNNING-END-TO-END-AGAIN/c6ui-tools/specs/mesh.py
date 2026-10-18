from specs.lib import *
F = U + '/🎬️scene/📐️math/🕸️mesh/🦀️.rs'
CH = [
    ch('📐', 'schema-faults-and-lease-types', 'schema_faults_and_lease_types', [(18, 175)],
       'Mesh schema, faults and lease types: the mesh capacities, the vertex fields and schema, the faults and the write token, lease and admission step types.'),
    ch('🧱', 'mesh-owner', 'mesh_owner', [(177, 426)],
       'Mesh owner: the mesh layout and page, the component length and demand helpers, the `Mesh3dOwner` with its reservations and drop.'),
    ch('🏛', 'mesh-authority-and-api', 'mesh_authority_and_api', [(428, 795)],
       'Mesh authority and api: the slots, the exclusive `Mesh3dAuthority` and the `mesh3d_*` begin, allocate, write, read, update, seal, abort and close entry points.'),
    ch('🔖', 'mesh-lease-and-cursors', 'mesh_lease_and_cursors', [(801, 993)],
       'Mesh lease and cursors: the `Mesh3dLease` reads and retirement, the item and page cursors and the mesh items.'),
    ch('🌈', 'scene-instance-materials', 'scene_instance_materials', [(995, 1088)],
       'Scene instance materials: the colour source, section role, instance material, `Instance3d` and component source.'),
]
SPEC = dict(orig='mesh.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='mesh.rs'))
