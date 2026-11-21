# Mesh Difference Across Six Cutter Walls

The retained partition cursor now tracks reciprocal cutter-face adjacency and bottom corners in bounded sets keyed by the admitted supports. One connected wall cycle must have exactly two neighboring walls at every face. A blind pocket must join every wall to one bottom and visit each matching wall-pair corner. A tunnel still requires two closed rim loops. The new capacity is 32 crossing planes, including the pocket bottom, under the existing point and work ceilings. The exact exterior-support flip and restore are shared with convex Union.

The six-wall tunnel publishes 30 vertices, 57 edges and 27 faces with volume 6.5 and area 29.32842712474619. The six-wall pocket publishes 34 vertices, 59 edges and 27 faces, volume 7.25 and area 27.414213562373096. Native verification kept every reversed cutter face, reciprocal directed uses and the expected Euler characteristic. The independent Manifold extrusions confirm both outcomes along with the earlier triangular cases.

The focused native Difference and Union suites passed three tests each. The authored facade preserved each original material and affine UV chart and reimported the four prismatic cases and the split-stock pocket. The paired Generation3d proof passed 31 geometry cases at fuels 1, 7 and 1,024 and both cancellation laws. The retained Flow tests passed all 11 tests, including all four prismatic cases. The portable package and geometry checks passed 25 tests and 4,792 assertions. Source budgets passed.

Nonprismatic four-or-more-plane recesses and nonconvex Mesh Boolean inputs still require their own partitions and topology verification.
