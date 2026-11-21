# Mesh Booleans With a Contained Concave Prism

The retained Mesh Boolean certificate can now prove a simple concave prism inside a convex operand. It matches its translated and oppositely oriented original caps, verifies reciprocal wall quads, and checks that both projected cap rings are simple. The proof accepts rotated placements. The containment pass checks one vertex against one enclosing plane per turn and requires strict interior clearance.

Intersect transfers the complete authored prism. Union transfers the complete authored enclosing operand. Difference publishes the original outer shell and the original prism with reversed orientation. With reversed Difference operands, the enclosing operand removes the prism entirely. A concave cutter touching or crossing the convex enclosure still requires other Boolean work.

The two-support authored projection previously imposed the convex half-space test even when transferring a complete concave ring. It now first verifies the full source ring in its original or exactly reversed cyclic order. Matching rings transfer direct corner and edge identity, including nonaffine UV values and original materials. Partial faces still use the existing clipped-patch verification.

The exact native Clipping suite passed 10 tests. The authored façade passed both focused tests, including nonaffine corner UV, material transfer, reimport, tessellation, and signed volume for the concave cavity. Focused native Flow passed 15 tests and Generate3d passed six, including both operand orders and fuel sizes 1, 7, and 1024. Independently, Manifold returned the expected intersection (volume 3, area 14), union (volume 64, area 96) and difference (volume 61, area 110), along with the area and volume of the rotated cavity. The independent Clipping and packaging suite passed 18 tests with 4,735 assertions.

General intersecting nonconvex Mesh operands remain open. The official linked and mounted browser runs are tracked separately.
