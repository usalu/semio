# Authored Two-Plane Mesh Difference

The existing Difference operators, `brep.mesh.difference` and `mesh.edit.difference`, can remove a corner from a certified outward closed convex mesh. The cutter supplies two nonparallel supporting planes. Both cross sections must intersect and the two cutter patches must share a crease. The published mesh can be nonconvex.

The retained job clips three branches. It keeps only the left-surface polygons of the two exterior partitions and reverses the cutter-owned polygons of the interior partition. It welds exact positional matches and inserts collinear split vertices where the partitions meet. The result verifies reciprocal oriented edge incidence, a shared cutter crease and positive signed volume before publication. It retains separate corner surface data and samples both cut faces from their original supporting polygons. Both cap materials, UVs, named normals and the cutter-edge seam have dedicated native assertions.

The original separation, contact, containment and one-plane cases retain their behavior. Unmatched partition interfaces and ambiguous authored charts yield a validation error. The later [three-plane Difference report](./📓️mesh-octant-difference.md) describes the certified three-plane extension. The subsequent [slab Difference report](./📓️mesh-slab-difference.md) describes exactly opposing slab cuts. The job shares the existing four-million-event ceiling. Its geometry work can be canceled before publication.

## Geometry and authored checks

- The scoped native Clipping and Mesh facade witnesses cover the original, offset and opposite corners. The symmetric L-section publishes 14 vertices, 23 reciprocal edges and 11 polygons. It has volume 6, area 22 and two independently owned reversed cap polygons. The native runs passed two Clipping tests and the authored Mesh facade test. The authored witness also checks distinct corner cap materials, UV sampling, reversed named normals and the authored crease seam.
- The Mesh portable and packaging suite passed 24 tests and 4,147 assertions. Manifold independently reproduced area, volume and the two outward cap orientations of all three corner cases.
- The Flow registry and both checked-in descriptor forms advertise one cutting plane or two intersecting cutting planes. The compact retained Flow producer and parser passed both Difference and Union tests. Rust syntax parsing passed the registered Flow workbench fixture. The full linked workbench was outside the compact run.
