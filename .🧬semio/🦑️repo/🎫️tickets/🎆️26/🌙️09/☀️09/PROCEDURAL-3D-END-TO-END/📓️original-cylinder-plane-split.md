# Exact Plane Split of a Retained Cylinder

The original `brep.intersect.split` job now accepts a certified finite cylinder when the plane is perpendicular to its placed axis and strictly between the authored caps. Both halves retain the original radial frame and radius. Their cut caps share the same center and seam direction, with opposite outward normals. The positive and negative output ports follow the signed plane normal, including when that normal opposes the cylinder axis.

The split recipe checks incidence in the placed frame and refuses outside, tangent, oblique, longitudinal, zero-normal, and numerically unresolved cuts. Both solids use the existing analytic topology, measure, and preview paths. The retained Split job still stages both handles before one atomic live-index insertion. Abandoning the pair retires both unpublished bodies.

Two neutral fixture rows exercise unequal cuts with both signed normals on a translated cylinder whose axis was rotated from Z to negative X. Manifold-3D independently matched each half's ordered bounds and volume, genus, and volume conservation. The oracle also confirmed that the three external and tangent cases yield no second material volume. The combined box and cylinder oracle suite passed four tests and 61 assertions.

The disk-light Rust harness reproduced the original box-only refusal and then passed all three tests. It checks the fixture, an oblique cylinder placement, a tiny scaled normal, authored-frame preservation, exact cap coincidence, rejection cases, and existing box output order. The linked Flow fixture invokes the registered Cylinder, Rotate, Translate, Split, and Volume operators. It checks unchanged live handles until both staged bodies commit, cancellation after both are staged, the corresponding native cap faces and circular trim geometry, and labeled preview bounds. That linked fixture was added but not run independently while the shared Cargo lane was active.

Nightly Rust syntax and scoped whitespace checks passed. The engine source closure has 3,999 active lines, the native registry 3,998, the solid transaction 3,442, and the Flow BRep extension 2,796.
