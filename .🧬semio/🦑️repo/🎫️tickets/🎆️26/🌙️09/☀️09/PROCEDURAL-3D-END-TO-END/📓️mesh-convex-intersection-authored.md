# Authored Convex Mesh Intersection

The Mesh modeling facade publishes the intersection of two closed convex polygon meshes. The retained job certifies clipping in the geometry owner and projects the authored surfaces in Reconstruction. Flow registers it as `brep.mesh.intersect` with two Mesh inputs and a PolygonMesh output. The Generation3d schema entry is `mesh.edit.intersect`.

A contributing output face carries the supporting polygon's material, shading and attributes. The job preserves raw corner UVs, directed seams and representable vertex, corner, edge and face attributes. It interpolates affine UV, normal and color data on cut faces and preserves uncut authored corner values. When coincident output corners carry different authored raw vertex normals, the job retains those values per corner under `intersectionRawNormal`. The existing source combination job merges contributed asset inventories and detects conflicting asset keys. Contact and disjoint input publish a valid empty PolygonMesh. The unchanged contained left input can return directly. Cancellation retires the operation before publication.

The operation refuses ambiguous source-face provenance when an output polygon crosses an authored supporting-face boundary on a coplanar right plane. This includes a boundary between coplanar faces that happen to carry equivalent payloads. It also refuses clipped nonaffine linear data, incompatible categorical samples and authored edge data that has no corresponding source edge. These refusals preserve the input sources and return no partial model.

## Checks

- Current compact native proof passed the authored fixture on the Mesh facade, Reconstruction, Clipping and actual Mesh Engine. It observed both indexed UV/color/material domains, seam and shading samples, both raw normal domains, halfedge import of the authored result and the canonical empty result, uncut nonaffine values on the contained right operand, clipped nonaffine and coplanar-boundary refusal, and cancellation.
- The portable Mesh suite passed 22 tests and 3,977 assertions. Manifold independently intersected the authored box pair and confirmed volume 1.5 and area 8, including areas 1.5 and 6.5 on left and right supporting surfaces respectively.
- Rust syntax parsing passed the Flow workbench proof. A full linked Flow suite was outside the compact native run. The Flow manifest and packed contribution advertise the same operator ID and input names.
- Reconstruction production Rust totals 186,044 bytes across nine files, within the 262,144-byte and 32-file owner ceilings.
