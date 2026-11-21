# Exact Point Classification of Retained Orthogonal Solids

The registered `brep.measure.classify` operator now accepts the material certificate retained by orthogonal Boolean results. Inside membership is taken over the closed occupied cells. Distance is measured against exposed face rectangles, including their edges and corners. Adjacent occupied cells share no exposed face, so their internal seams and intersections stay inside. Exterior walls, notch surfaces, and the walls of enclosed cavities are true boundaries.

The classifier uses the same adjacency rule as boundary publication. It visits only set bits of the existing 27-cell arrangement, checks at most six candidate faces per cell, and allocates no scratch storage. The existing contact tolerance of 0.000001 model units applies to the signed Euclidean distance on either side of the material boundary.

The shared neutral fixture adds 33 point probes over Fuse, a notched Cut, an enclosed cavity, and Intersect after retained Fuse. It exercises true and internal edges and planes, hole membership, and diagonal points on both sides of the contact threshold. Manifold-3D Boolean meshes, Three.js triangle distances and independent ray parity matched all probes. The full query oracle passed three tests with 67 assertions.

The disk-light Rust witness first reproduced the unsupported route and then passed all 11 tests. It compiles the production material-cell and boundary routines and applies the exact retained dispatch arm and tolerance to the neutral fixtures. The registered Flow fixture invokes Box, Translate, Fuse or Cut, optional Intersect, and Classify. The linked Flow test was added but not run independently during the shared Cargo build.

Nightly Rust syntax and scoped whitespace checks passed. The Boolean and native registry source closures have 3,996 and 3,999 active lines, respectively.
