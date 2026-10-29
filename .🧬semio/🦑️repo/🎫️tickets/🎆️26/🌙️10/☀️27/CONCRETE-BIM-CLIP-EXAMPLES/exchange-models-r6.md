# Exchange and Models Consumer Repairs R6

## Scope

Updated the exchange and models Python chapters plus co-located exchange regression fixture and tests. No AGENTS files, git state, source provider, or other chapters changed by this agent.

## Repairs

- Exchange bay crop iterates curtain-wall disconnected paths and clips each original segment to the crop rectangle. It does not join surviving points across a door gap. Opening crop labels use actual source tags.
- Exchange slab beat highlights the named salon as deliberate architectural context. It uses the resolved room outline and avoids indexing eleven rooms into a three-item palette.
- Both slab beats draw every source opening, show every opening tag with its independently derived ring area, and subtract the complete opening area. Exchange opening animation grows both hole shapes with one synchronized area tracker.
- Exchange analysis mapping outlines and labels every omitted opening at the source mid-surface elevation instead of assigning their total area to only the ramp.
- Models first-bay dimension uses the first two actual grid x coordinates. Its box dimensions and discipline/schema heading counts derive from model data.
- English and German narration and legends describe ramp and stair openings.

## Regression Inputs and Checks

Language-neutral fixture: clip/🔌️exchange/🧪️cases/🔣️slab.json. It precedes consumer source edits and records SL-1 gross408.50m², ramp25.00m², stair5.29m², total30.29m², net378.21m², eleven first-floor spaces, and named salon context. Rectangle segment cases cover crossing, parallel rejection, vertical crossing, and diagonal crossing.

New independent test file: clip/🔌️exchange/🧪️tests/🐍️slab.py. It avoids importing Manim. It parses chapter methods, executes AST-extracted drafting.signed_area and _clip_segment, and compares outputs to Shapely Polygon areas and LineString/box intersections. It checks disconnected paths, all-opening loops/labels, deliberate salon selection, and source grid dimensions.

## Verification Status

Source inspections and fixture creation completed. Python/WASM compilation and test execution requested from the coordinating agent; results should be appended after completion. Native Manim rendering remains unavailable, so this report makes no rendered-frame or visual-layout verification claim.

## Files

- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🧪️cases/🔣️slab.json
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🧪️tests/🐍️slab.py
