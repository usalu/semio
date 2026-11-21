# Authored Convex Mesh Union

Flow exposes **Union Convex Meshes** as `brep.mesh.union` with `mesh` and `other` inputs. Generation3d uses `mesh.edit.union`. Both call the retained `MeshAuthoredUnion` job.

The operation certifies both meshes as outward, closed convex solids. It returns the containing source directly for containment and preserves the primary source for coincident bodies. Strictly separated convex bodies can be published as separate closed components when a supporting plane certifies their separation. For overlap across one matching section, the job trims the first body, matches the section ring to the opposing source face, removes both internal polygons, and welds the perimeter. Exactly matching full-face contact uses the same topology. It validates the merged exterior against the original supporting planes and checks reciprocal edge incidence.

The union keeps coplanar exterior polygons split at the welded perimeter. Each polygon keeps its source UV, color, material, shading and other representable samples. Matching seam positions share a vertex without merging corner data. Both source asset inventories pass through the retained combination rules. Incompatible schemas, ambiguous intersections and partial face contact yield a validation error. The later [partial-overlap report](./📓️mesh-partial-overlap-union.md) describes the supported multi-plane section mismatches. The weld uses exact representable position equality. Cancellation stops work before publication.

## Native checks

- Clipping passed both compact tests. The Union proof covers both overlap directions, same-face contact, separated components, containment in either direction, coincidence, and refusal for partial contact or mismatched rings. It checks the source-plane owner of every exterior face, right-source position lineage, reciprocal oriented edges, area, volume and cancellation.
- The Mesh facade passed the compact authored test. At the four welded vertices the 10 output polygons share position IDs and retain separate indexed UV, color, material and normal domains. The proof also covers the published Difference and Intersection fixtures against the current shared owners.
- The compact Flow witness passed against the production pair parser and producer, returning 12 vertices and 10 faces from two JSON Mesh inputs. Rust syntax checks passed the registered Flow workbench witness. The full linked workbench was outside the compact run.
- The typed Generation3d checkout passed two native tests covering 17 combined Boolean cases at fuel 1, 7, and 1,024, plus cancellation during capture and production.

## Portable checks

- The Mesh jobs and packaging suite passed 24 tests and 4,135 assertions. Manifold independently reproduced both overlap directions, containment, matching and partial contact, separation, and mismatched boundaries. For each admitted stitched result it also checked that the internal seam plane contributes no triangles.
- The typed Generation3d catalogue parity and independent geometry suite passed 58 tests and 381 assertions. The live Flow registry, JSON manifests and packed operator record carry the same operator ID and pair inputs.
- The facade source closure is 262,088 bytes against its existing 262,144-byte ceiling. Clipping production is about 41 KB across three files. Scoped whitespace checking passed.
