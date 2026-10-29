# Model Source Freeze R14

The coordinated source snapshot contains 340 elements, 67 physical column products, 61 beams, five slabs, four storeys and 24 spaces. Source identity uses canonical UUIDv5-derived IFC identifiers. The final analytical link refresh reloaded the corrected composite-material provider, recomputed all three links and returned an empty consistency issue list. SL-1 now identifies MAT-HOLLOW-SLAB as its assembly material; RC-owned beam strips retain their solid material. SL-KIOSK uses the roof weathering assembly.

## Physical verification

The focused remodel and architecture suite passed all 25 tests before the final material-declaration and link-only refresh. The tests use Shapely polygon intersections against actual column contours, physical walls after doorway subtraction, actual slab voids and local-datum furniture geometry. A subsequent repeat exceeded the 55-second observation limit; this is not recorded as a second pass. The earlier result is retained in generated/wasm-output/model-r14-tests.json while generated artifacts remain available.

A separate all-fixture versus physical-column intersection audit found a cabinet conflict with numbered column 27 and incorrect hall table centres. The cabinet now has a real 2cm-clear notch; the integrated table rings wrap actual columns 9 and 8 with inner radius 0.145m. Repeating that audit found zero furnishing/detail collisions with physical column volumes, excluding nominal floor-finish overlays.

The continuous 0.8m first-floor public route passes through the office side of the measured stair rather than claiming the narrow strip beside its ramp is usable. Portable fixtures now record that route and finished first-floor datum 3.17m. The route checks actual doors, walls, column contours and openings.

## Honest analytical scope

Source declarations now retain slab openings and describe hollow-clay/RC field material rather than claiming uniform solid RC. The declared losses explicitly omit exclusive beam selfweight and the complete beam-to-column network from the slab teaching analysis. Slab-owned RC strips remain included. Column-birth fixity and roof point-load transfer are declared assumptions.

## Remaining source fidelity work

This snapshot does not complete the whole Villa Savoye reconstruction. Parent-bed alcove post identification is pending; its portable case explicitly remains unimplemented rather than naming fabricated posts. The primary-source garage hoist and coal hopper, external individual foundations and refined aerodynamic column vertical sections require the next evidence-backed wave. Static parking for three assumed 4.6m cars must be rechecked against the actual numbered profiles; an earlier packing fixture predates that register. No claim of drivable parking access is made. The physical roof cornice elevations, original versus restored chimneys and some architectural partitions remain approximate. Native renderer output has not been verified by this agent. See foundations-and-hoist-r15.md, structural-register-r9.md and source-materials-r10.md for evidence and limits.
