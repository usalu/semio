# Exact Point Classification of Retained Cylindrical Boolean Results

The registered `brep.measure.classify` operator now supports the finite extruded profiles retained by parallel-cylinder Intersect, Fuse and through-notch Cut, and by clipped box–cylinder Intersect. A two-circle certificate retains the authored planar set operation alongside the circle supports. A disk–rectangle certificate determines planar membership from the disk and the oriented line sides.

The classifier measures the distance to each retained profile edge. It uses the circle projection only within the authored angular trim and otherwise measures to the nearer endpoint. Line projections are bounded by their endpoints. It combines the signed profile distance and axial interval distance as an exact Euclidean product. Thus diagonal points outside both a side and a cap use the proper edge distance, while discarded circle walls and extensions beyond the finite caps do not create contact. The profile is bounded to eight members and the query allocates no scratch storage.

The neutral fixture adds 41 probes across five Boolean cases, including equal and unequal Fuse, offset finite lens, inward notch, and a mixed straight/circular clip. Manifold-3D Boolean meshes and Three.js ray and triangle queries matched all probes. The full independent suite passed five tests with 160 assertions.

The disk-light Rust witness reproduced the prior refusal and then passed all 24 tests. It runs the original Boolean dispatch and exact classification arm. The extra native case checks horizontal antiparallel cylinders and the actual diagonal distance at the cap edge. The spherical source regression suite passed all 27 tests after the certificate update. The linked fixture invokes the original registered Box, Cylinder, Translate, Boolean, and Classify operators against the same neutral rows and awaits the shared Cargo lane.

Nightly Rust syntax and scoped whitespace checks passed. Source closures contain 3,995 Construction lines, 3,998 Native Registry lines, and 3,518 Solid Transaction lines.
