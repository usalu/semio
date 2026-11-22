# Round 3: machinery/tower-crane

Author: round-3 author of `machinery/tower-crane`, 2026-11-20. No earlier round-3 state existed for this asset (no `r3-tower-crane-*` scripts, no report), so I started from the round-2 model (`🗑️generated/r3-tower-crane/round2-volumes.json`, a copy of the catalog `volumes` at the start).

Scripts (all in T): `r3-tower-crane-kit.py` (numbers stated by hand, lattice bars), `r3-tower-crane-parts.py` (every assembly per scale: solids and 2D shapes from the same numbers), `r3-tower-crane-build.py` (writes `volumes/machinery/tower-crane.json` and the candidate document `🗑️generated/r3-tower-crane/🔣️.json`), `r3-tower-crane-decisions.json` (3D and 2D decision texts, en/de), `r3-tower-crane-apply.py` + `r3-tower-crane-jsontext.py` (root fixes into the catalog by text surgery, member order kept), `r3-tower-crane-zoom.ts` (zoomed shaded views of a 72 m asset), `r3-tower-crane-suit.ts` (print suitability per scale), `r3-tower-crane-probe.ts` (kernel frame probe: extrude planes yz = (y,z) along x, zx = (z,x) along y, xy = (x,y) along z).

## 1. Reference

Subject: a top-slewing hammerhead tower crane with tower head (cat head) and pendants, 55 m jib, 40 m freestanding lattice mast, class of the Liebherr EC-H / Potain MD cranes of 8 to 10 t. Sources: EN 14439 and EN 13001-2 (asset references `en-14439`); manufacturer type sheets of top-slewing cranes of this class (Liebherr EC-H and EC-B technical data sheets, Potain MD/MDT data; a web search on 2026-11-20 found the Liebherr data sheets but no extractable numbers, so the dimensions below are my knowledge of the class, stated as such); EN 1993-1-1 for the steel; EN 206 for concrete density.

| item | real subject (class value) | source |
|---|---|---|
| mast section | square lattice 1.6 to 2.0 m, four chords 120 to 180 mm (tube or angle), K/Warren bracing, sections 5 to 6 m | class knowledge; asset `mast.section` 4 x 150 mm at 2.0 m |
| freestanding base | cross-base (two crossing box beams 6 to 8 m, on four pads) with 4 to 8 concrete ballast blocks on its arms, or a cast foundation; always square in plan, because the jib slews 360 degrees | class knowledge; asset anchor `base-fixed` footprint 7 x 7 m |
| ballast | 40 to 60 t for 40 m hook height and 55 m jib | asset `base` mass 45 t |
| slewing ring | circular, 1.8 to 2.5 m, with 1 or 2 vertical slewing gear motors on the platform | class knowledge |
| jib | triangular lattice: one top chord (tube) on the axis, two bottom chords (rectangular or angle sections) that are the trolley track; width 1.2 to 1.6 m, depth 1.4 to 1.8 m, Warren diagonals in the two inclined faces, horizontals in the bottom plane | class knowledge; asset `jib.section` "Triangular lattice, depth 1.6 m" |
| jib suspension | two pendants (bars or ropes) from the tower-head apex to the top chord at about 0.4 and 0.75 of the jib | class knowledge |
| counter-jib | flat box lattice or two parallel girders with walkway, 14 to 18 m, winch on top, counterweight slabs hung at the tail, pendant from the apex to the tail | class knowledge |
| counterweight | vertical concrete slabs of about 2.0 to 2.5 m height side by side, 15 to 25 t | asset `counterweight` mass 20 t |
| cab | about 1.8 m long, 1.4 m wide, 2.1 m high, raked windscreen, side windows, on a console beside the tower head so the operator sees down past the jib | class knowledge (LiCAB, Potain Ultra View) |
| trolley | open frame, four wheels on the bottom chords, two rope sheaves; hoist reeved in two falls (8 t class) | class knowledge |
| hook block | two side plates, sheave, crosshead, forged hook | class knowledge |
| concrete skip | 1.0 m3: cylinder about 1.3 m across, cone to a 0.3 to 0.4 m spout with gate, legs, bail | class knowledge |
| hoist rope / pendants | 14 to 18 mm hoist rope; pendant bars 50 to 80 mm or rope pendants | class knowledge |

## 2. Audit of the round-2 asset (deviations with numbers)

Rig: joints are right (mast top 40.0 m, slewing ring 0.5 m, jib and counter-jib pins 4.4 m above the ring, apex 8.6 m, trolley 0.8 m below the jib axis). No rig joint was moved. Root defects were in `dimensions`, the drawings and the solids:

1. Foundation: 7.0 x 3.5 m strip of four 1.75 x 1.0 x 3.5 m blocks in a row (`dimensions.depthM` 3.5, 2D front and plan 3.5 m deep) although the support anchor declares a 7 x 7 m footprint. A crane whose jib slews round cannot stand on a 3.5 m strip; the round-2 area review itself described the ballast as "2 x 2 blocks 3.5 x 3.5". Frustum mast foot 3.0 -> 2.0 m through which the bottom mast diagonals ran.
2. Jib: drawn and modelled as an inverted trapezoid with two 140 mm top chords at +-0.74 m and two 120 mm rails at +-0.45 m, contradicting the bone section ("triangular lattice") and every real hammerhead jib (one top chord). The chords ran through the tower head to the axis (x = 0); no heel pins, no tip frame.
3. Counter-jib: chords from the axis through the tower head; no heel posts, no tail frame for the pendant (the pendant ended in the air at 15.6 m).
4. Pendants: 12 mm, attached at 22 m and 50 m at +-0.12 to 0.135 m off the axis (beside a top chord that no longer exists in a triangular jib).
5. Cab: a 1.9 x 2.2 x 1.0 m box (1.0 m wide), on the platform under the jib in the trolley's path (trolley at its innermost position overlapped the cab roof), two flat glass boxes; thermal shells inverted (30 mm lining outside the 3 mm steel; the layer order says steel is the exterior).
6. Slewing ring: an ellipse 2.8 x 2.0 m; no slewing gear visible.
7. Counterweight: three horizontal slabs 2.4 x 0.73 x 2.4 m stacked (31.7 t of concrete for a 20 t mass), touching the chords without any suspension.
8. Winch house: floor 50 mm above the counter-jib chords (floating).
9. Trolley: a filled 1.4 x 0.55 x 2.0 m block (model-findings), 2.0 m wide although the rails were 0.9 m apart; single sheave.
10. Hoist: one rope in one fall while the block carries a sheave (two falls); hook block a box; skip a square tapered box; four slings to a point inside the block body.
11. Glazing layers ordered `pane-out, gap, pane-in` while layer lists are interior first.
12. 2D: front and plan of the foundation 3.5 m deep; jib plan as four parallel strips; cab 1.0 m wide; trolley 2.0 m wide; counterweight 2.4 m long; skip front as 25 stacked rectangles.

## 3. Root fixes (catalog file, old -> new)

- `dimensions.depthM` 3.5 -> 7.0 (anchor footprint and stability; the cross-base is 7 x 7 m).
- `rig.bones[base]`: title "Ballasted foundation" -> "Cross-base with ballast"; `mass.comY` 0.5 -> 1.05 (ballast blocks at 0.7 to 1.5 m). Mass 45 t unchanged (4 blocks 2.2 x 0.8 x 2.4 m = 42.2 t + steel cross 3 t).
- `anchors[base-fixed].title` -> "Cross-base on its four pads, fully fixed".
- `thermal.regions[cab-glazing].layers` order `pane-out, gap, pane-in` -> `pane-in, gap, pane-out` (interior first).
- `thermal.regions[ballast].title` -> "Concrete ballast blocks"; `pendants.characteristicLengthM` -> 0.016, `rope.characteristicLengthM` -> 0.014.
- Region areas: see section 5 (set from the 1:50 model, each within a factor 1.5 of the old value).
- 2D, all four scales, all three viewpoints, for the bones base, jib, counter-jib, counterweight, trolley, hoist-block, skip, rope-1..8 and the slewing parts slew-ring, slew-platform, slew-motor-l/r, cab-console, cab, cab-glass, cab-glass-front, pendant-fore-1/2, pendant-back, plus new `winch-skids`: redrawn from the same numbers as the solids (members for every lattice bar a view sees distinctly, extruded or revolved solids for the masses, true-width pens up to 2.0 mm). Mast, tower head, winch house and the hook (1:50 to 1:200) keep their round-2 drawing. 2D decision texts amended (silhouette sentence, retained, merged, omitted, thin elements).
- `symbols[tower-crane-symbol]`: ballast 7 x 7 m (front 7 m wide, 1.5 m high), counterweight 1.6 m long, cab 1.85 x 1.4 m at its new place, jib 1.32 m wide from 0.56 m, counter-jib from 0.785 m, trolley 1.46 m wide and 1.07 m deep, fore pendant to 40.55 m.

## Status

Work in progress: checks running, areas to be set, renders to be judged.

## Full fine-scale thermal verification on November 21

The four-scale strict run completed locally in 937.78 seconds with zero findings. The heat solver now bounds probes against individual, demonstrably separated copies before testing triangles. The 125 ladder rungs on the 38.6 m rail exercise this path. [Independent repeated-solid fixtures](digital-thermal-bounds.ts) check containment and masking. [Catalog rule status](digital-volume-status.md).

## Lower heel connection and pin seats reviewed November 21

The detailed structural mesh previously extended a box beyond the lower heel stock. The lower joint now marks both ends of the 60 mm cross tube. The upper joint collar is on the 80 mm transverse pin where that pin is individually represented. The long main members retain their own bindings. The exact published crane passed all thirteen rules at all four scales, the document checks and twelve GLB scenes at 7,903,284 bytes. [Catalogue structural coverage](digital-member-coverage.md).

## November 21 carriage and open truss

Published `ee9104e032eb463250971a14faa7aec4d2fbbb5bf4d4a5a0bb71dd8f5a97eb6a`. The 1:500 mast and two jibs now have visible space between their chords. The trolley runs on four wheels with 125 mm tread radii on two bottom rails. Separate 16 mm cheek plates support the two grooved sheaves and the reeving passes along two rope falls. All four scales use the actual steel for the temperature surfaces and structural connections. The detail views show the cheeks, roller positions and shared carriage frame. The Gherkin regression measures tread contact, wheelbase and sheave groove depth.

All 52 volume checks, both ends of each paper band, schema validation and independent readback of twelve scenes passed. The exported GLB is 8,246,056 bytes. [Buffer measurements](digital-crane-export.md)
