# Isolated Convex Bevel on Concave Closed Meshes

The retained Bevel job now accepts a single selected convex edge on a concave closed manifold when its local cut can be certified. Both endpoints must have three incident faces, with exactly two faces in common. The four faces must be planar. Every profile plane must place the two selected endpoints strictly outside and all other source vertices strictly inside with mesh-scaled clearance. The certificate evaluates one face corner or plane-vertex sample per retained work step before starting the cut.

The certified cut uses the existing profile and authored surface transfer. One segment produces one strip face. Two segments produce two. The original convex path continues to serve one or multiple selected edges.

The neutral witness is a height-two L-prism. At width 0.25, the selected outer vertical edge produces a closed result with 14 vertices, 9 faces, outward signed volume 9.9375, and surface area 33.64460678118655. The reflex edge, a convex edge whose profile would remove a remote vertex, and a width exceeding the local clearance each produce their expected refusal. All three failures and cancellation leave the original serialized mesh unchanged.

## Verification

- A Three.js extrusion of the trimmed L outline independently reproduced the vertex count, area and signed volume. The Mesh job and packaging suites passed 18 tests and 3,836 assertions.
- Two compact Rust regressions passed against the actual Mesh retained modeling sources. The local regression covers the plain mesh with the reverse halfedge handle, the authored one- and two-segment profiles, UV interpolation, smooth and flat shading, vertex normals, distinct retained and generated face material assignments, paired preservation of an unrelated seam, and the three refusals and cancellation. The prior convex authored Bevel regression also passed.
- The Flow registration and all four JSON manifest carriers now advertise the local scope. The packed descriptor still decodes to 19 fields, with only the manifest field modified and all following bytes unchanged.
- The Flow workbench adds a registered Bevel invocation using the shared concave fixture and expected output counts. The test parsed as Rust. Executable Flow integration remains unverified.
- The framework 3D source closure measured 249,000 bytes, 2,904 active lines and 15 files. Both source packaging gates passed at their existing limits.
