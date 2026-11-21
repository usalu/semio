# Retained selected Box Fillet in an authored frame

`brep.solid.filletEdges` now accepts certified rectangular Boxes that have been rotated, translated, or reflected. A selected edge, or up to four parallel selected edges, keeps the authored local direction. Each fillet is an exact quarter-cylinder between the tangent planar stock faces, with true circular trims on both caps.

The native admission verifies that each picked edge label belongs to that solid's own rectangular publication. Reflected stock reverses its native cap ring, and the selected edge is mapped to the correct material corner before forming the profile. The finite axial profile certificate keeps reciprocal coedges on each trim and routes through the existing private topology staging. Foreign selections, repeated edges, crossing directions and radii that eliminate a required tangent span are refused. The original world-axis route uses the same analytic profile construction.

## Evidence

- The neutral `🔣️original-oriented-fillet.json` covers each authored axis, opposite parallel edges, reflections and a compound rotation followed by a reflection. The independent Manifold construction removes the corner material outside the tangent cylinders and compares volume, area and topology. The combined selected and all-edge Manifold run passed nine tests and 54 assertions.
- `🐍️box-edge-fillet-source-check.py` compiled the actual profile and reflected ordinal logic and matched its results against authored endpoints on the exact source prism. All 17 tests passed. The compact proof checks each cylinder axis and center, radius, exact trim coincidence, one reciprocal use in each direction, and integrated preview mass and area. It also exercises repeat, cross-direction and radius refusers and existing unrotated and clipped-prism regressions.
- The registered `original-brep-booleans` test uses real Rotate, Mirror, Translate and BRep selections, rejects a foreign source selection, and cancels after private face insertion. It asserts a single atomic live result, reciprocal native incidence, exact cylindrical patches and trim arcs, preview labels and Measure. Its managed linked run remains pending shared capacity.
- Canonical source closure passed Native Registry at 3,965 lines and 216,701 bytes, Solid Transaction at 3,939 and 203,611, Flow BRep at 2,796 and 151,488, and original-booleans at 1,618 and 190,609.

This feature covers parallel authored selections on rectangular stock. Multi-direction fillet networks and compound rounded-corner blends remain outside the selective operator's current certificate.
