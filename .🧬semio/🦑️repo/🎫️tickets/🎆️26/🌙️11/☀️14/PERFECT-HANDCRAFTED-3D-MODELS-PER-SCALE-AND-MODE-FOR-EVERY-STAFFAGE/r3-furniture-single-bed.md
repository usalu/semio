# Round 3 — furniture/single-bed

State: finished and spliced (2026-11-19). No earlier round-3 work existed for this asset; it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts (ticket folder): `r3-single-bed-kit.py` (the measures of the bed, world to bone-local helpers, rig), `-body.py` (every body solid of every scale stated by hand), `-lists.py` (thermal, structure, volume decisions), `-roots.py` (dimensions, anchors, materials, references, notes, texts, plan symbol, representation decisions), `-draw.py` (the 2D shapes of every scale and view, drawn from the same measures), `-build.py` (candidate document under `🗑️generated/r3-single-bed/` and `volumes/adhoc/r3-single-bed.json`), `-apply.py` (writes the validated candidate's root members into the catalog file in the file's own formatting; `volumes` then goes in through `volumes-insert.py`). Re-running build and apply changes nothing (checked).

## 1. Reference

Subject: a solid pine bed for a 0.90 x 2.00 m mattress with posts, frame-and-panel ends and side rails, a sprung slatted base whose head section lifts, and a 0.20 m cold-foam mattress.

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Mattress | nominal 900 x 2000 mm; inner length and width of the bed 10-20 mm larger | EN 1334:1996 | 0.90 x 2.00 x 0.20 m; clear 2.00 x 0.91 m between posts and rails |
| Overall size of a pine bed for 90 x 200 | 209 x 98 cm, headboard 92 cm, footboard 32 cm (22 mm boards) | IKEA TARVA 302.612.70, product page read 2026-11-19 (web search) | 2.12 x 1.01 m with 60 mm posts, head 0.95 m, foot 0.55 m (heights kept from the asset) |
| Posts | 60-70 mm square on post beds, eased edges, chamfered tops | carpentry practice, my knowledge | 60 x 60 mm, 5 mm ease, 12 mm chamfer |
| Side rails | boards 22-30 x 120-180 mm with a 25 mm ledger screwed inside | carpentry practice | 30 x 160 mm (0.14 to 0.30 m), ledger 25 x 25 mm, top at 0.205 m |
| End frames | top and bottom rail between the posts, panel of V-jointed boards about 18 x 90 mm | carpentry practice | rails 30 x 80 and 30 x 160 mm, panels 18 mm with nine V joints at 89 mm |
| Sprung slatted base 90 x 200, head adjustable | 28 slats, 35-38 x 8 mm laminated birch or beech in rubber caps, frame height about 45 mm, about 80 mm over the camber, 9-11 kg | retail data sheets read 2026-11-19 (web search: 28 slats; 35 x 8 mm; frame 4.5 cm; camber height 8 cm) | base rails 25 x 45 mm, lifting frame 25 x 40 mm inside them, 28 slats 38 x 8 mm (10 + 9 + 9), camber 15 mm, caps 50 x 30 x 25 mm, bottom 0.205 m, crown 0.285 m; rig mass 9.4 kg |
| Head section | hinged on bolts through the frame rails, held by a ratchet stay | product knowledge | two 8 mm bolts at x 0.78, y 0.23 m; 0 to 65 degrees (rig limit kept); stay not modelled |
| Mattress make-up | cold foam 40 kg/m3 in a quilted cover, piped seams at the top and bottom edges | asset materials (ISO 3386-1, ISO 10456), product knowledge | foam core 0.88 x 1.98 x 0.18 m inside a 10 mm cover; 22.2 kg in the rig |
| Mattress top | 0.485 m | asset notes; the sleeping-person asset lies on the datum 0.46 m in `asleep` | kept: 0.285 + 0.20 m |

Sources: https://www.ikea.com/lv/en/p/tarva-bed-frame-pine-30261270/ , https://www.inhofer.de/p/13747100/lattenrost , https://online-moebel-kaufen.de/shop/gen_typenplan.php?id=99468 , https://www.kaufland.de/product/375693295/ . Material data stay those of the asset's own references.

## 2. Audit of the state before round 3

- Dimensions: `depthM` 0.90 m with a mattress drawn 0.82 m wide, although the summary names a 0.90 x 2.00 m bed. A bed for a 0.90 m mattress is 0.96 to 1.04 m wide.
- Rig: the slat bones hinged at y 0.285 m, the top of the slats under the mattress; the real hinge is the bolt in the frame rail, 55 mm lower. The mattress bones sat on the same point. `ground` (floor reference) carried a length of 2.12 m, which made it a structural member that needed a solid; the old structure list hung the centre beam on it.
- Anchors: foot posts at z +-0.42 m, head contact 0.90 m wide.
- Thermal data: slat 12 mm (real 8 mm); slat region 3.0 m2 from a reviewer's 21 slats of 60 x 22 mm.
- 2D: headboard a solid slab down to the floor in the front view; footboard two posts and one rail with nothing between; mattress 0.21 m, sunk 10 mm into the slats, as three separate cushions with all corners rounded; slats a 22 mm strip; side rails 40 mm overlapping the posts by 20 mm; 1:500 posts drawn 0.10 m.
- Solids: 13 / 12 / 9 / 7 body solids, all boxes: headboard one rounded slab 0.06 x 0.95 x 0.90 m, mattress three rounded boxes with gaps, slats 60 x 22 mm, a centre beam that carried nothing. Structure: the mattress as three foam blocks, the headboard slab as a member (qa-visual grade C at every scale).

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `dimensions.depthM` | 0.90 | 1.01 (height 0.95, length 2.12 and mass 59.2 kg unchanged) |
| `rig` `ground.lengthM` | 2.12 | 0 (a reference, not a member) |
| `rig` `slat-head`, `slat-mid`, `slat-foot` `y` | 0.285 | 0.23 (the hinge bolt); `mass.comY` -0.01 -> 0.015 (same world height band, in the frame rails) |
| `rig` `mat-head`, `mat-mid`, `mat-foot` `y` | 0 | 0.055 (mattress underside 0.285 m as before; poses, loads and the sleeping-person datum are unchanged) |
| `anchors` | `foot-head` width 0.9; `foot-foot-l/r` z +-0.42 | width 1.01; z +-0.475 |
| `materials` | 4 | + `cap-rubber` (ISO 10456 butadiene rubber), `steel-fitting` (ISO 10456 steel); `birch-slat` title 12 mm -> 8 mm |
| `provenance` | 9 references | + `en-1334-1996`, `ikea-tarva-90x200`, `slatted-base-90x200`; notes rewritten (en, de) |
| `thermal.regions[slats]` | area 3.0 m2; layer 12 mm, 0.08 m2K/W, 12480 J/m2K | 3.5 m2 (hand sum of 28 slats, rails, bars and caps; the 1:50 model measures 3.516; inside the factor 1.5); layer 8 mm, 0.053333 m2K/W, 8320 J/m2K, region capacity 8320 |
| `thermal.assumptions[1]` | 0.6 m2 of the 1.7 m2 top | 0.63 m2 of the 1.8 m2 top |
| `structure.assumptions[0]` | head board panel, foot posts at +-0.42, hinge at 0.78 | two head posts as one contact, centre leg under the cross beam, foot posts at +-0.475, bolts at x 0.78, y 0.23 |
| `symbols[0]` | 0.90 wide, mattress 0.82 x 2.01 x 0.21 | 1.01 wide, mattress 0.90 x 2.00 x 0.20, slat band 0.205 to 0.285, leg 0.14 |
| `representations[*].geometry` | 13/12/7, 13/12/7, 7/8/7, 7/5/5 shapes | 16/18/16, 10/16/16, 10/13/14, 7/5/6 (elevation/front/plan at 1:50, 1:100, 1:200, 1:500), ruling E1 |
| `representations[*].decisions` | old texts | rewritten per scale (en, de) |

Mattress 4.5 m2 and frame 4.7 m2 were left: the model measures 4.62 and 4.79. Poses, clips, supports, masses, summary and the catalog index (height, length) are unchanged. `humans/sleeping-person` still validates.

2D (E1): every shape is drawn from the same measures as the solids of its scale: chamfered posts, rail, leg, the slat sections as one outline each (rail with cap teeth at 1:50, rail with deck at 1:100, slab at 1:200), the mattress as three parts square at the joints and rounded only at the free ends, piped seams as strokes; front view from the foot end with posts, rails, boarded panels and V-joint strokes; plan with posts, end rails, side rails and ledgers. Slat sections and mattress parts stay extruded `solid` shapes in front and plan so they tilt and sink. The head part's seam strokes lie below the middle part so their tail is covered when the head is raised.

## 4. What was modelled

### 1:50 — body 46 solids (130 with repeats), thermal 49 (127), structure 45
- Body: posts as extruded eased squares with tapered chamfer tops; top rails with rounded upper edges; panels as extrusions of the real V-jointed section; plain bottom rails; side rails with eased edges; ledgers; cross beam 40 x 65 mm on a 60 mm leg; base rails and end bars; lifting frame with rounded hinge ends and cross bar; two revolved bolts with heads; 28 slats as lofts with a 15 mm camber; 56 rubber caps; the mattress as three superelliptic lofts (0.90 x 0.20 m section, rounded head and foot ends) that meet flush at the two joints; twelve piping beads (8 mm) along the long edges, round the corners and across the ends, top and bottom.
- Thermal: cover = the body surface, foam core nested 10 mm inside (layers `quilt`, `foam-core`); the slatted base as own solids of the hidden region; all frame timber as `bed-frame`.
- Structure: posts, rails, ledgers, cross beam, leg, base rails, lifting frame and all slats as members; hinge bolts, eight bed bolts, eight dowelled rail ends and the caps as joints; four glides as supports (the head pair is one solid for the one anchor); per mattress part a 30 mm bearing strip of foam on the slat crowns plus its mass. Panels, cover and piping are absent.

### 1:100 — body 24 (29), thermal 24 (27), structure 35 (40)
Plain rounded posts, rails and panels as boards at true thickness, rail and ledger as one L section per side, base rails, end bars and lifting frame, one cambered deck per slat section, mattress with a simpler rounding and no piping. Structure with 15 mm pins.

### 1:200 — body 13 (16), thermal 13, structure 24 (25)
Square posts, one 30 mm board per end, side rails, cross beam, leg, one slab per slat section, mattress with a three-station rounding. Structure carries the top and bottom rails of both ends as its own members instead of the boards.

### 1:500 — body 7, thermal 8, structure 19 (23)
Two end panels at their true heights, rails and base as one tray on the leg, three mattress blocks with soft long edges. Thermal adds one hidden slat plate inside the tray. Structure: four posts, two side rails, two end rails, leg, three slat plates, strips, masses, glides.

Decisions per scale are two bilingual sentences each (body; thermal and structure).

## 5. Checks (final, on the catalog asset)

- `r3-validate.ts furniture/single-bed --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Catalog `volumes` equal `volumes/furniture/single-bed.json`.
- `volumes-check.ts furniture/single-bed --product`: `PASS (16.4 s)`. Silhouette IoU elevation / front / plan: 1:50 1.000 / 1.000 / 0.998; 1:100 0.999 / 1.000 / 1.000; 1:200 1.000 / 1.000 / 0.999; 1:500 1.000 / 1.000 / 1.000. Bounds deviation 0 at every scale. Thermal area deviation at 1:50: mattress 0.028, slats 0.005, frame 0.019; 1:100: 0.003, 0.243, 0.026; totals 1:200 0.057, 1:500 0.263.
- Finding sent to `author-requests.md`: flagged `slender`, the piping beads cost the front and plan silhouette (IoU 0.949); they are 8 mm, so the flag was dropped.

## 6. What the renders show (opened with the Read tool)

Seen: `r3-sheet` body at 1:50, 1:100, 1:200, 1:500 (six views each); 1:50 from below; 1:50 `head-raised`; clip `sit-up-to-read` at 4.5 s; thermal cut at 1:50 and 1:100 (six views), 1:500 three-quarter; structure at 1:50 (six views), 1:100 `head-raised` (six views), 1:200 and 1:500 three-quarter; `volumes-render` overlay at 1:50; `r3-2d` at 1:50, 1:100, 1:200, 1:500 and 1:50 `head-raised`. Rendered but not opened: 1:200 thermal, 1:100 structure three-quarter, 1:50 `asleep` right side.

- Front (from the foot end): foot posts with chamfers, top rail, boarded panel, bottom rail; behind them the head posts, head rail and boarded head panel; leg in the middle below.
- Right and left side: tall head post, low foot post, side rail, leg at mid length, one mattress with rounded ends and two fine joint seams, piping at the top and bottom edge.
- Back: head posts, top rail, panel with nine V joints, bottom rail.
- Top: posts at the corners, thin side rails, end rails, mattress with rounded corners and the piping loop.
- Three-quarter: reads at once as a pine bed with a mattress. From below: slats in their caps on the base rails, the lifting frame inside them with the bolt head, ledgers, cross beam and leg; nothing floats.
- `head-raised` and the clip frame: head frame, slats and mattress head part turn about the bolts, the fold closes on top, the base rails stay.
- Thermal: foam core inside the cover at 1:50, the green slatted base under it, frame around. Structure: frame, slat grid with orange caps, strips with masses on the slats; panels ghosted. Coarser scales stay a bed down to the 4 mm sign.
- 2D: clean elevations and plan at every scale; the raised pose draws the lifted comb of caps and the tilted mattress part in all three views.

## 7. Still imperfect

- Raised head section, 3D: the three mattress parts are rigid, so the head part's last 0.14 m turns into the middle part. The fold is closed, but the side faces coincide there and the tail of the head part's piping shows as a faint diagonal bead on the side. A true bend needs per-vertex skinning, which a solid does not have.
- The ratchet stay that holds the raised head frame, the centre strap of the slats, glides, zip and handles are not modelled (no bone for the stay; the rest is under 7.5 mm or hidden).
- The joint between the middle and foot mattress part is a rig seam with no real counterpart; it shows as a fine line in 3D and as a divider in the drawings.
- Plan drawings: the mattress is an extruded shape, so its corners are square where the 3D mattress has 45 mm rounded corners.
- Frame mass: the modelled timber comes to about 29.6 kg against 27.6 kg in the rig (7 %); the masses were not touched.
- The centre leg under a cross beam is unusual on a 0.90 m bed; it was kept because the asset declares that support, and given a beam to stand under.
- 1:500 thermal: the tray box makes the frame region 63 % larger than declared; only the total is limited there (0.263 against 0.3).
- 1:200 plan reports a 45 mm p95 from the 5 mm gap between mattress and rail falling under the raster cell, not from a form difference.
- `r3-sheet --list thermal` paints the mattress grey at 1:100 and coarser (its legend only knows the 1:50 layers); the data are right.
- The repo test suites were not run; only the two checks of the brief.
