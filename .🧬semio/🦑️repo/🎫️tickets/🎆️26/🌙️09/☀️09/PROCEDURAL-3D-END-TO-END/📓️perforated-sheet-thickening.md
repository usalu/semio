# Retained Thicken for a perforated planar sheet

The original Flow Thicken operation accepts a horizontal rectangular planar face with one complete circular opening. The face may be a selected cap from a bored solid or an independently published sheet such as the output of Offset Face. Either normal and either thickness sign produce the expected cap elevations. The exact solid uses the certified bored boundary shared with rectangular stock through-cuts. It has two cap openings and an inward-facing analytic cylindrical wall. The source face stays live.

A fixed-size certification walks four native outer coedges and the one-member inner ring. It checks closed outer incidence, rectangular bounds, the planar circle and its seam, opposite trim orientation, and strict clearance from the exterior walls. The same arena paths are included in the retained demand. Native construction and retirement reuse the funded solid publisher. The retained path refuses profiles outside this certificate. The synchronous ThickenJob still has outer-ring-only side stitching.

## Verification

- The neutral witness defines three signed thickness scenarios, exact cap elevations, and expected net volume. The independent Three.js surface suite passed 13 tests, including extrusion of the holed profile without bevels, both cap elevations and volume.
- The isolated Rust source harness passed all five tests, including actual borrowed rectangular cap certification and BoreBoundary recipe generation. It checked both face normals, negative thickness, exact bore parameters and volume, and refusal for overlap or an oblique sweep.
- The linked `original-brep-faces` fixture extends the funded pick-to-offset flow with an actual original Thicken call. It checks paid cancellation after a draft circle appears, full rollback and source survival, exact cap planes and inward cylinder, viewport labels, open cap centers and mesh volume. Linked execution is pending the shared native build.
- Rust AST checks and scoped diff whitespace verification passed. The repository closure counter reported 3,980 active BRep Engine lines, 3,776 native registry lines, 1,714 solid transaction lines and 1,960 lines in the faces integration target.
