# Reference Window Unit

## Implemented source inventory

The window assembly contains the fixed outer glass; bonded centre base, cover, threaded sleeve and divider; two-piece inner brass guide; radial folded paper, bent carriers, clips, outer guide and reveal fixings; rubber gasket; reveal closure and 40 mm angle supports; and the observed-reference inner sash with glass, hinges, upper/lower locks and an authored inspection stop. Every unit part is tagged window_unit, including the externally visible centre hardware. Full and exterior assemblies retain the complete named inventory with different mesh resolution. The separate long wall luminaire is implemented and reviewed in the independently owned light domain.

[The published section](https://bpb-us-e1.wpmucdn.com/blogs.uoregon.edu/dist/1/16902/files/2019/02/Facade-Construction-Manual-2h2g0dz.pdf), citing Detail/jpn 33 (1972), supplies fixed glass 6 mm, base Ø140 mm, cover Ø120 × 5 mm, sleeve Ø20 mm, divider 1.2 mm, bent carrier sheet 2 mm, and the 40 × 40 × 4 mm angle. These facts are separate from inferred component sections and placement.

[WRI's A904 field reference](https://madoken.jp/en/series/20219/) shows an inward-opening inner sash and between-pane radial fan. Its original installation date and prevalence across all 140 units are not established; no retrofit is asserted. Inner-pane thickness, frame/hinge dimensions and a 10 mm roomward inset remain explicit assumptions. The right-edge hinge is visual inference. The 90-degree fan settings are sourced; a 90-degree sash view is an authored inspection pose, not a measured maximum.

## Real state geometry and contacts

WindowUnitState(sash_angle, blind_fraction, blind_rotation) drives actual geometry. Closed/stowed, quarter/half/three-quarter/covered fan states and 0/45/90-degree sash poses are represented. No roller, Venetian slat, depth overlay or hidden leaf substitutes for the folded fan. Paper facets have finite normal thickness and occupy a genuine interval between the panes. The initial outward inner-pane assumption caused a P/S collision; the observed projecting sash is now placed roomward of the lining, with that unmeasured depth labelled as inferred.

At default size, front-family fixed glass occupies local Y 4.031333333…4.037333333 m, closed inner glass 3.887…3.893 m, and the thickest stowed paper envelope 3.920175246…4.004158087 m. P/S fixed glass occupies local X 1.231876275…1.237876275 m, inner glass 1.137…1.143 m, and stowed paper 1.145446717…1.229429558 m. All partial states remain between their actual pane surfaces. inspect-window-unit.py regenerates these actual mesh measurements.

The independent contact oracle checks the glued base, sleeve/divider/cover, guides, paper/carriers, clips/fixings, gasket seats and closure angles. Hinge leaves derive their endpoints from the actual annular chord at each LOD, correcting the former approximately 0.95 mm analytic-circle gap.

## Verification and scope

Eight unit tests pass in the final physical gate in 50.186 s (earlier targeted run: 56.148 s), covering every named part in both LODs, exact primary versus inferred fields, all six families and partial/stowed/covered states, real pane separation, independent Rhino right-hinge rotation, actual material contacts, Rhino watertight meshes and positive NumPy volumes for every state. Dedicated portable cases and tests are under cases/🪟️unit and tests/🪟️unit. The AJV gate independently validates the source model and malformed device parameters.

Furnished reference views remain closed. Standalone open unit inspections preserve their actual geometry and do not claim clearance from movable pillows; the auditor found pillow contact in four end-window reference layouts. Pillows are unchanged. The final shared physical and circulation gates pass as part of the 132-test physical acceptance. Cairo inspections and native scene gates remain separate and await the final visual/source freeze.
