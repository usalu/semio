# Bounded Convex Mesh Difference

Flow provides **Subtract Convex Mesh** as `brep.mesh.difference` on its `mesh` and `other` inputs. Generation3d provides `mesh.edit.difference`. Both use the retained `MeshAuthoredDifference` job.

Two outward, closed convex polygon meshes are certified one event at a time. If one support plane of the second operand crosses the first body, and the first body satisfies every other interior halfspace of the second, Difference retains the exterior halfspace and publishes a closed polygon solid. The new cap uses the contributing face of the second operand with opposite winding. Containment and coincidence yield an empty PolygonMesh. A separating plane or zero-volume contact returns the original first mesh. Multiple cutting halfspaces yield a typed invalid-value refusal because that case requires more general topology. Cancellation discards in-flight state before publication.

The new cap uses its source material, UVs, colors and shading. Named and raw normal values flip with the cap. Seam samples and other edge attributes require a corresponding source edge. The shared projection refuses cap patches that cross a coplanar authored-face boundary or require interpolating nonaffine authored data.

## Checks

- The focused native clipping proof passed both tests, including the established clipping provenance regression. It checked both cut directions, reversed cap provenance and winding, reciprocal edges, signed volume, surface area, contact, containment, deterministic multi-plane refusal, and cancellation.
- The focused native Mesh facade proof passed. It exercised the authored material and indexed UV domains, flipped cap normal attributes and raw normals, and the existing authored Intersection cases against the current Clipping and Reconstruction owners.
- The portable Mesh jobs and package suite passed 23 tests and 4,022 assertions. Manifold independently reproduced volumes and areas for eight bounds fixtures and the reversed cap normals for both cutting directions.
- Rust syntax parsing passed the retained Flow preparation and workbench witness. The packed and source Flow descriptor records contain the same input names, operator ID and localized entry. The linked Flow workbench was outside the compact check.
- The expanded Clipping and Reconstruction production subtrees measured 29,444 bytes across two files and 186,890 bytes across nine files, respectively. The wider clipping audit passed 14 of 15 tests. Its historical aggregate assertion counter still expects 1,153 against 1,316 in the current original test files, which now also include newer regressions. The retirement field inventory was updated to match actual clipping ownership.
