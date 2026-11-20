# Round 3 — appliances/server-rack

State: finished and spliced (2026-11-20). No earlier round-3 work existed for this asset; it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts (ticket folder): `r3-server-rack-kit.py` (measures, world to upright-bone helpers, corrected rig), `-body.py` (every body solid of every scale by hand), `-lists.py` (thermal, structure, volume decisions), `-roots.py` (anchors, emitter, references, notes, assumptions, feet area, representation decisions), `-draw.py` (2D of every scale and view from the same measures), `-build.py` (candidate under `🗑️generated/r3-server-rack/` and `volumes/adhoc/r3-server-rack.json`), `-apply.py` (writes changed root members into the catalog file in its own formatting), `-suit.ts` (print-suitability violations per scale). Running build and apply again changes nothing (checked).

## 1. Reference

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Envelope | 600 W x 2000 H x 1000 D, 42U, freestanding | Rittal TS IT data sheets (DK 5529.110 600x2000x600, DK 5509.161 800x2000x1000), asset dimensions | 0.60 x 2.00 x 1.00 m kept (handle 15 mm proud of the front) |
| Height units | 42U of 44.45 mm, 482.6 mm ear width, 450 mm opening, equipment 0.8 mm under its unit | IEC 60297-3-100 | rails from 0.10 to 1.967 m (42 x 44.45 = 1.8669 m) |
| Frame | welded base and top frame, 4 corner posts, depth rails carrying adjustable 19-inch angles | Rittal TS IT, APC NetShelter (product knowledge) | frames 40 x 30 mm rings, posts 35 x 40 hollow, depth rails 20 x 30, angles 2 mm L 19.5 x 42 at z -0.40 and +0.36 |
| Panels | 1.2 to 1.5 mm sheet with folded returns, side panels often divided | product knowledge | 1.2 mm trays with 20 mm returns, two halves per side on a middle strut, 7 pressed louvres |
| Front door | vented sheet steel about 70 to 85 % open, swing handle with lock, hinges | Rittal DK 5529.110 (85 % perforated), asset layer (70 %) | folded leaf 25 mm deep, face with 20 mm holes at 28 mm (1:50), stile for handle, lock, 3 hinges |
| Rear door | divided vented door | Rittal TS IT | two perforated leaves, holes offset by half a pitch |
| Casters | 50 mm wheel, overall height 72.5 mm, 45 x 75 mm bolt pattern | StarTech RKCASTER2 data sheet | 50 x 20 mm wheel, fork, race, 62 x 95 plate, 70 mm overall, wheels at x +-0.25, z +-0.45 (anchor footprint 0.5 x 0.9) |
| Equipment | 4U storage, six 2U servers, patch, brush, 1U switch (asset) | asset notes | fronts with ears 8 mm, chassis 0.70 m deep (switch 0.40, panels 0.10) |
| Mass | 234 kg: cabinet 95, door 12, casters 7, equipment 120 | asset (authored) | unchanged; equipment centre recomputed from the stack: 0.56 m above floor, whole rack about 0.75 m |

Sources: https://www.rittal.com/uk-en/websearch?q=5529.110 , https://www.rittal.com/pdf-creator/variant/in-en/5509161 , https://media.startech.com/cms/pdfs/rkcaster2_datasheet.pdf . Both added to `provenance.references`.

## 2. Audit of the state before round 3

- Rig: all four bones lay horizontal (`angleDeg` 0) from the floor origin, the 2 m `frame` bone pointing 2 m sideways out of the cabinet along the width, `equipment` and `door` (1.87 m) the same; joints at the floor for door and equipment although both start at the lowest unit (0.10 m). Equipment centre of mass at 0.80 m although storage and servers fill the lowest 16 units (real 0.56 m); structural assumption said 0.95 m.
- Emitter anchor and direction tied to the horizontal frame; the old emitter solid was a 0.5 m disc floating over the roof.
- 2D: elevation door drawn behind the equipment without frame; front view without rear door edge and depth 0.975 (asymmetric -0.475..0.5); plan cable slot over the top frame; casters as 30 mm rounded posts; panel border line that no panel has; 1:500 without casters (red band in QA).
- Solids (qa-visual: B/B/B at 1:50 to 1:200, C at 1:500): flat door plate, 30 mm slab walls, casters as posts (area 0.025 against 0.06), closed rear panel (no exit for the front-to-back air), ellipsoid mass floating in the void, rails as hairlines, 1:200 a near copy of 1:100, 1:500 with an open pocket and free-standing posts.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `rig.frame` | angle 0, mass com (0, 1.0) | angle 90 (upright), com (1.0, 0) |
| `rig.feet` | angle 0, com (0, 0.03) | angle 0 relative (upright), com (0.035, 0) |
| `rig.equipment` | joint (0, 0), length 1.87, com (0, 0.8) | joint at 0.10 m, length 1.8669 (42U), com 0.458 along the bone (0.558 m above floor) |
| `rig.door` | joint (0, 0), length 1.87, com (0, 1.03) | joint at 0.10 m, length 1.867, com 0.9335 (1.0335 m) |
| `anchors.exhaust-top` | x 0, y 2 | x 2.0, y 0 (same world point on the upright frame) |
| `thermal.emitters.exhaust-air.directionDeg` | 90 | 0 (still world up on the rotated frame) |
| `thermal.regions.feet.areaM2` | 0.06 | 0.072 (honest 1:50 casters, plate tops bolted to the base marked as mounted faces; factor 1.2) |
| `thermal.assumptions[1]`, `structure.assumptions[0]` | edge emitter, centre 0.95 m | emitter over the two roof fans, centre about 0.75 m |
| `provenance` | 7 references, notes | + `rittal-ts-it-2024`, `startech-rkcaster2-2024`; notes rewritten (construction, rails, centre of mass) |
| `representations[*].geometry` | 72/9/6, 18/7/3, 10/5/2, 3/1/1 shapes | redrawn under E1: 1:50 elevation with rails, units, door field, frame ring, perforation, handle, lock, hinges; front with door and rear door edges, joint, louvres, handle, caster wheels and forks; plan with fan guards, cable slot, door and rear edges, handle, hinge; 1:100 top group merged; 1:500 caster band |
| `representations[*].decisions` | old texts | rewritten per scale (en, de) |

Dimensions, poses, clips, materials, summary unchanged. 1:100 2D needed two passes: the first redraw was print-unsuitable (rail strip 0.197 mm, switch sliver behind the door top rail 0.184 mm against 0.2 mm), so the rails are omitted at 1:100 and patch, brush and switch are one top group; all four scales are suitable under `office-laser` now.

## 4. What was modelled

### 1:50 — body 80 solids (195 with repeats), thermal 27 (33), structure 37 (50)
Base and top frame rings, gland plate with side cable cut-out, four hollow posts, four depth rails, two middle struts, side panels as 1.2 mm trays with returns (two halves per side), 7 louvres, roof skirt (1.5 mm) and roof sheet (7.5 mm, the feature) with two fan openings and a cable slot, two fan housings with hubs, guard rings and spokes, brush; two 0U PDUs; split rear door (returns, perforated faces offset half a pitch, latches); front door (return ring, perforated face with 16 x 64 holes, handle housing, lever, lock, three hinges); four L-section 19-inch angles; storage array, six servers, patch panel, brush panel, switch, each as front plate with ears and chassis, with drive bays, handles, grilles, ports and LEDs; four swivel casters (plate, race, crown, two cheeks, wheel). Thermal: 1.2 mm outer skins of side halves, rear leaves, roof, base; door leaf; 2 and 1.5 mm skins on the fronts; caster references; fan outlets flush in the roof openings. Structure: posts, rings, depth rails, struts, angles, door leaf, caster forks as members; 8 welded corners, rail bolts, hinges as joints; wheel-contact ring; one mass slab per unit spanning the angles, top-group slab, door slab, cabinet mass at its centre.

### 1:100 — body 31 (50), thermal 16 (17), structure 24 (35)
One tray per side, plain posts and rings, gland plate, 22 mm rails, 15 mm roof sheet with openings over plain fan frames, door face with 30 mm holes at 45 mm, one handle block, plain rear leaves, units as front + chassis (top group merged), casters as plate, fork, wheel.

### 1:200 — body 11 (13), thermal 12, structure 15 (17)
Side walls, roof, base ring and rear door as 30 mm plates; door as 30 mm frame around a face with 30 mm holes at 60 mm; server block and top group; four caster blocks. Structure: 30 mm posts, base and top ring, four angles, door frame, casters.

### 1:500 — body 5, thermal 11, structure 12 (14)
Cabinet block, door face with two windows in which server block and top group sit 10 mm behind the door plane, caster band. Structure: 75 mm posts, two rings, two angles, door frame, caster band, contact ring, masses.

## 5. Checks (final, on the catalog asset)

- `r3-validate.ts appliances/server-rack --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Catalog `volumes` equal `volumes/appliances/server-rack.json`.
- `volumes-check.ts appliances/server-rack --product`: `PASS (54.2 s)`. Silhouette IoU elevation/front/plan 1:50 0.957/0.998/0.999, 1:100 0.999/0.998/0.999, 1:200 and 1:500 1.000. Area deviation 1:50: door 0.074, servers 0.034, switch 0.078, blanks 0.019, cabinet 0.044, feet 0.005; 1:100 all within 0.08; totals 1:200 0.003, 1:500 0.215.
- `r3-server-rack-suit.ts`: all four scales print-suitable.

## 6. What the renders show (opened with the Read tool)

Seen: `r3-sheet` body 1:50 (six views, before and after the caster fix), 1:50 right side alone, 1:50 three-quarter from the rear (turn 220), 1:50 thermal cut, 1:50 structure, 1:100 body, structure and thermal cut, 1:200 and 1:500 body; `volumes-render` overlays and lists at 1:50, 1:200, 1:500; `r3-2d` at 1:50 (candidate), 1:100, 1:200, 1:500.

- Door face (product elevation): perforated door in its leaf, hinges on the left, handle with lock on the right stile, casters under the base; the lower 16U show the servers through the holes, the top units show through the top rows.
- Side views (front/back of the sheet): divided side panel with joint, louvres top rear on the right panel, door and rear handles at the edges, casters with wheel and fork in profile.
- Rear (three-quarter turn 220): two perforated leaves with latches, roof with fan guards and brush slot. Top: two guarded fans, cable slot, door edges.
- Thermal: skins, door, fronts and casters in their colours; cut shows the green server fronts and the switch and blanks band inside. Structure: a closed frame of posts and rings with orange corners, angles, mass slabs on the angles, contact ring under the wheels.
- Coarser scales stay a rack at a glance: 1:100 coarser grid and plain rear, 1:200 waffle door on four caster blocks, 1:500 block with two equipment windows on a dark caster band.

## 7. Still imperfect

- In the open units (16 to 39) the half-pitch offset of the rear holes still lets 6 x 6 mm sight lines through both doors; they show as fine light specks in the door view (as with a real empty rack) and cost the 1:50 elevation IoU (0.957). Blanking panels would close them but the asset declares those units open.
- Door faces and the 1:50 roof sheet are 7.5 mm (the smallest feature) instead of 1.5 mm; the real 5 mm perforation is generalised to 20 mm holes (1:50), 30 mm (1:100, 1:200).
- The cabinet mass sphere sits at the cabinet's centre of mass in the empty middle of the frame: physically right, it touches nothing.
- The product elevation is the door face of the rack (the asset's established convention), so the product "front" view is the side panel.
- The 1:500 door face is an extrude with two windows; `r3-sheet` shades its cap with a smooth gradient (renderer interpolation, not geometry).
- Feet area: 0.072 is the honest 1:50 caster; at 1:200 (blocks) and 1:500 (band) the feet region is far larger, only the totals are compared there (0.003, 0.215).
- Thermal regions have no internal heat sources (server internals); the regions are the asset's six surfaces and were not changed.
- The repo test suites were not run; only the checks of the brief.
