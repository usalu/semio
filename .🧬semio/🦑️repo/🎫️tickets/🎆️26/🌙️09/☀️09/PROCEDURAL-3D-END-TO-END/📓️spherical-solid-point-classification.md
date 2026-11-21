# Exact Point Classification of Retained Spherical Boolean Results

The registered `brep.measure.classify` operator now reads retained spherical cap, band, and cavity certificates. For a trimmed spherical result, it projects a point into the certificate's meridian frame and measures the nearest retained spherical patch or planar disk. A projection outside the latitude span is measured to the corresponding circular rim. Material membership follows the original retained intersection, union, or subtraction. Discarded sphere walls and planes extended beyond their disks do not count as boundary.

The spherical-shell classifier compares the existing signed distance to its certified primitive exterior with the inward distance to the inner sphere. The certificate guarantees strict nesting, so the nearest boundary and sign are exact for both sphere exteriors and box exteriors. Both implementations run in constant work, use no mesh approximation, and retain the existing contact tolerance.

The neutral fixture adds 52 probes in eight groups. It covers one-plane intersect and cut, a two-plane band, sphere lens, sphere fuse and partial sphere cut, an eccentric sphere cavity, and a box with a spherical void. Independent Manifold-3D Boolean meshes and Three.js ray and triangle queries matched all 52 probes. The full query oracle passed four tests and 119 assertions.

The disk-light Rust witness reproduced the unsupported route, then passed all 27 tests. It uses the production Boolean dispatcher, the exact spherical distance routines, and the retained classification tolerance. An additional direct test checks the Euclidean diagonal beyond an oblique rim. The linked Flow test invokes the registered Box, Cylinder, Sphere, Translate, Boolean, and Classify operators against the same fixture. The shared Cargo lane owns linked execution.

Nightly Rust syntax and scoped whitespace checks passed. Source closures have 2,688 active Representation lines, 3,997 Native Registry lines, and 3,448 Solid Transaction lines.
