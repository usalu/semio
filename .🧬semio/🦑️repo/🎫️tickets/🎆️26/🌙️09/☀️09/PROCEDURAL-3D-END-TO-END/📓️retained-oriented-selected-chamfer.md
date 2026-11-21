# Retained selected Box Chamfer in an authored frame

`brep.solid.chamferEdges` now accepts one native edge from a rotated, translated or reflected rectangular Box. It clips the selected local profile corner at equal distances from the two original faces. The resulting five-sided prism has the exact planar bevel and straight trim edges. The transformed exact recipe uses the same authored axes and solid-owned edge selection as selective Fillet, including reflected native ring ordering.

The operation uses the existing funded staging and rollback path. It refuses a foreign edge, multiple selected edges, a nonpositive distance or a distance that removes an adjacent face. The world-axis Chamfer and Draft regression keeps its original exact profile.

## Evidence

- The neutral `🔣️original-oriented-chamfer.json` covers all three local axes, their reflected equivalents and a compound rotation with reflection. Manifold independently splits the original stock with the authored bevel plane. Its combined Chamfer and Fillet run passed 15 tests and 52 assertions, checking area, volume, Euler genus and material conservation across the split.
- The compact production-source profile witness passed 26 tests. It checks the single bevel's expected outward direction and perpendicular plane offset, exact line-to-trim agreement, reciprocal edge incidence, oriented face closure, area, volume and refusal of collapsed geometry. The separate world-axis Chamfer and Draft proof passed two tests.
- The registered `original-brep-booleans` test picks the edge from the retained BRep, confirms no live result before private topology is complete, cancels after native face insertion and validates the completed preview and topology. It also exercises the foreign and multiple-edge refusals. Managed execution remains pending shared capacity.
- Canonical source closure passed Native Registry at 3,968 lines and 216,861 bytes, Solid Transaction at 3,954 and 204,352, Flow BRep at 2,796 and 151,488, and original-booleans at 1,645 and 195,766.

The supported scope is a single original edge of certified rectangular stock.
