# Authored Four-Wall Through-Tunnel Difference

The existing Flow `brep.mesh.difference` and Generation3d `mesh.edit.difference` operators support a convex cutter with four active walls and a common through-axis. The cutter's remaining support halfspaces must contain the stock. The retained job constructs the exterior partitions incrementally and exposes the four reversed, cutter-owned polygons.

Publication checks reciprocal oriented edges and positive volume. The four walls must form one adjacency cycle without a corner where three walls meet. Rim tracing then proves exactly two simple openings, each passing through all four walls. The volume contains no interior partition polygons. Per-wall materials, corner UVs and flipped normals use the original supporting faces. The tunnel certificate refuses ambiguous topology and nontransverse support sets. The later [blind-pocket report](./📓️mesh-blind-pocket-difference.md) describes the certified five-plane extension.

The centered and alternate-axis neutral cases produce a genus-one shell with 24 vertices, 44 edges, 20 polygons, volume 6 and area 30. The offset case retains the same topology with volume 6.5 and area 29.5. The registered Flow operator advertises the existing plane, corner and slab cases alongside through-tunnels.

## Checks

- The compact native Clipping run passed both tests. It verified reciprocal incidence, all four reversed wall owners and normals, both portal heights for each wall, Euler characteristic zero, volume and cancellation. The blind and nontransverse cases return validation errors.
- The native authored facade witness passed. It checked each wall's material and flipped named normals, distinct UV samples at shared wall vertices, the complete six-entry material inventory, native tessellation and PolygonMesh reimport.
- The portable Mesh jobs and packaging suite passed 24 tests and 4,249 assertions. Manifold independently confirmed genus one, one connected shell, both wall openings, reversed wall normals, volume and area for the centered, offset and alternate-axis tunnels.
- All five compact retained Flow producer/parser tests passed. The registered workbench fixture passed Rust syntax parsing and scoped whitespace validation passed. The full linked workbench was outside the compact run.
