# Retained Perpendicular Cone Split

A perpendicular plane strictly inside a certified cone now produces a tip cone and a base frustum through the registered Split operator. The smaller radius comes from the authored cone's linear profile, and both published pieces use the same cut circle with opposing cap normals. The certificates preserve the original conical support, including its apex and half-angle. The two output ports follow the signed plane normal.

The planner admits both boundaries together. The existing funded cursor inserts the pieces incrementally and publishes their two live handles atomically. The linked fixture exercises both normal directions on a placed horizontal cone, verifies original cone supports, cut circles, cap orientation, pcurves and component labels, and checks cancellation of the staged pair. Tangent, external, oblique and zero-normal inputs follow the existing refusal path.

The neutral fixtures specify both volumes, bounds and section radii. The independent Manifold-3D suite verified ordered bounds, volumes, conservation and endpoint/external absence of a second volume. Across box, cylinder and cone the suite passed five tests and 101 assertions. The disk-light Rust witness passed all four tests, including reciprocal frustum trims and original cone apex. The linked registered test was left to the managed Cargo lane.

Nightly Rust syntax, scoped whitespace, and path-length checks passed. The Native Registry and Solid Transaction source closures contain 3,999 and 3,989 active lines, respectively.
