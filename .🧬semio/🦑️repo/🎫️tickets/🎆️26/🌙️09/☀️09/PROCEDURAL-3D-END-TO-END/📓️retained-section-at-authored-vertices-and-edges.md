# Retained Section at Authored Vertices and Edges

The native concave Section cursor now reuses an original vertex when it falls on the requested plane and distinguishes tangent vertices from crossings. Along an original edge it joins the exact endpoints once when the two incident faces are on opposite sides. A tangent edge does not form a section segment. The assembled boundary still requires a single closed cycle and follows the requested plane normal.

The registered reflected-stock fixture reproduces a vertex-aligned area of 16.84528264428262 and an edge-aligned area of 13.353962887959618 using Manifold, with the authored endpoints and both normal directions checked by the exact native traversal. Before the edge change the compact source suite failed only the aligned-edge case. After the change it passed 51 tests. The independent suite passed 20 tests and 288 assertions, including independent nearest-point fixture values.

The same traversal now handles the interior plane coinciding with the rotated blind pocket floor. The pocket walls extend toward the floor face’s outward side, so the floor and surrounding material share an interior edge. The section is one rectangle of area 16. Independent Manifold slices above and below the floor and Three triangle area checks confirm an area-15 region above, plus the area-1 floor. Both normal directions passed the native traversal. The preceding source failed only this new test. The updated compact suite passed 52 tests and the independent suite passed 20 tests and 298 assertions.

The existing requirement that a Section plane cross the material interior also applies to the concave cursor. Sections with multiple rings still need additional publication support. The linked Flow runner remains active.
