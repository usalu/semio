# Verification: grasshopper_tower, Beat3_Tower (en and de)

Scope: fresh keyframe capture in both languages (22 PNG each, 1280x720), every PNG read, checked against `audit-grasshopper-tower.md`. Frames: `T/🗑️generated/frames/🦗️grasshopper/Beat3_Tower/en/` and `.../de/`, layout reports `layout.md` in each folder. Only this report was written; the capture writes only to `🗑️generated`.

Capture note: the en run wrote all 22 frames and `layout.md` but exited with a cp1252 UnicodeEncodeError on the final `print` (emoji in the path). The de run used `PYTHONUTF8=1` and exited 0. The en frames and report are complete.

## Verdict

NOT READY. All three audit blockers are fixed in both languages. Nothing new is a blocker. Still open as majors: the count formula literals, the missing ten-term legend in the recap, the EN tower slider labels, the swap counts and recolour-only swap, the "Planes 40" remainder bar, and the cull-index values that are never shown.

## Audit blockers

| audit item | status | frame evidence |
| --- | --- | --- |
| nowire: storeys count contradicts (slider, panel, drawing) | FIXED | nowire: slider "Capsule storeys 10", panel "0 10", formula "10 × 4 × 2 = 80", drawing about 10 storeys. Same in de. |
| ontop: Z 34.375 m mislabelled | FIXED | ontop: "Capital base Z = 33 m" (de "Kopf Unterkante Z = 33 m"). 10 capsule storeys + 2 podium = 12 × 2.75 = 33.00 m. |
| types: Value List feeds L, index unwired, static panel | FIXED | types: pattern list wired to List Item L, Series S wired to i, "0 8" to Series C, legend A to D added. Same in de. |

## Audit majors

| audit item | status | frame evidence / remaining gap |
| --- | --- | --- |
| parts/recap: core outline not visible | PARTLY | parts: blue core outline now visible around the shaft; capital outlines mauve, close to the pink group; planes have no outline. |
| all canvas: numbers without unit | FIXED | "Core side 5.5 m", "Slot pitch 2.75 m", "Plane O -12.475, 2.75, 5.5 m"; de "O -12,475; 2,75; 5,5 m". X/Y/Z are directions (unitless, acceptable). |
| shaft: "5.5 m" overlaps level mark "2" | PARTLY | The "2" level mark is gone from shaft and core. Strict checker still reports a dimension stroke through the "5.5 m" text (layout.md, shaft and core). |
| axes (de): decimal comma vs list separator | FIXED | de axes: "1; 0; 0", "-12,475; 2,75; 5,5 m". |
| capital: typed 4.00 m and 0.60 m | FIXED | capital: "Capital height 2.75 m", "Capital overhang 0.25 m" (model values). |
| podium: Unit Z F unwired | FIXED | podium: Multiplication R wired to Unit Z F, Unit Z V to Extrude D. |
| count: typed literals 4 and 2 in "10 × 4 × 2 = 80" | NOT FIXED | count and nowire, en and de: formula still text. |
| towers: facade counts not on screen | FIXED | towers: "A 9 × 3 × 2 = 54", "B 11 × 4 × 2 = 88", "Cull Index -2", "B = 86". |
| towers: "Storeys A/B" label | PARTLY | de "Kapselgeschosse A/B" FIXED. en still "Capsule storeys A/B". |
| swap: panel values computed in scene, no wire | PARTLY | Panels now wired (Subtraction 0, Addition 70), but the inputs are typed Value Lists "0 35" and "0 35", not counted from the definition. |
| swap: caption promises change, only recolour | PARTLY | Type B recolours to D. No type parameter (storyboard's window diameter slider) changes geometry. |
| types/swap: no legend for four type colours | FIXED | Legend A, B, C, D in types and swap. Panel letters are not colour-linked (minor). |
| why: "Capsules 54" counts components | PARTLY | Renamed to "5 + 49 components" (caption "components in the definition"). "Planes 40" is still the remainder of unnamed objects. |
| recap: ten-term legend promised by storyboard | NOT FIXED | recap: no legend, en and de. |

Minors from the audit, checked in passing: hook now says "two towers" (FIXED, en and de); facades caption says "facade angles" (FIXED); de podium caption still wraps "Höhe." alone (NOT FIXED); capital "1.00" tick still covered by a wire grip (NOT FIXED); capital frame still runs past the left canvas edge (NOT FIXED); Move T in orient still unwired (NOT FIXED); "[1]" tag still touches the red X arrowhead in items (NOT FIXED); core caption still says "storey count" (NOT FIXED); dimension precision still mixed (NOT FIXED).

## New findings (fresh frames)

Blockers: none.

Majors:
1. fix (en and de): the "Cull indices" input shows only the label, no values. The caption "removes exactly those two planes" cannot be checked on screen. Show the two indices (e.g. "-2" and the plane positions) in the panel.
2. Carried into the same concern: the tower swap counts (see audit major table) are typed literals, so the storyboard's "count per type from the definition" is still not met.

Minors:
3. mistake (en and de): caption says the two capsules "collide with the service side", but no service side is drawn; only red X markers appear.
4. count (de): "Anzahl gleich Geschosse mal Fassaden mal Items" is ungrammatical; use "Die Anzahl ist Geschosse mal Fassaden mal Items".
5. capital (en): caption "sits on top of the core" still with no core drawn beneath it.
6. Strict checker (layout.md): 22 findings in 11 of 22 frames in both languages, mostly wires and dimension strokes through labels. Not all are visible at 720p.

## Must still change before READY

1. count: build "10 × 4 × 2" from the engine (facade count and items per facade), not typed literals.
2. recap: add the ten-term legend or remove the clause from the storyboard.
3. towers (en): "Capsule storeys A/B" to "Capsule floors A/B".
4. swap: count each type from the definition (Count plus Addition) instead of the typed "35" Value Lists; add a type parameter to the drawing or change the caption to "takes the colour of type D".
5. why: compute the Planes bar from its member list, or relabel it, so "40" does not remain a remainder.
6. fix: show the cull indices values on the canvas.
7. Minor fixes listed above (de podium caption, capital tick grip, Move T wire, items tag, core and capital labels).

Frames: `T/🗑️generated/frames/🦗️grasshopper/Beat3_Tower/en/*.png` and `.../de/*.png`; layout reports `.../en/layout.md`, `.../de/layout.md`.
