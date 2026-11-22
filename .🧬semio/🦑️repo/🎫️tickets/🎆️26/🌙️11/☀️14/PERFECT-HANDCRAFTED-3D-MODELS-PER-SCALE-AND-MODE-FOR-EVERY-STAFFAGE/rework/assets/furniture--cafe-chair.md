# `furniture/cafe-chair` — running report

## Pass 1 (second round), 2026-11-22

State found: source `volumes/furniture/cafe-chair.json` and catalog `volumes` were identical; author 14 had judged the asset "no defect found, not changed" (`rework/14.md`, section 2). Judged again from the sheets of all four scales and three lists, all five poses and two instants of both clips.

### Reference table (bistro side chair, steel tube and beech; mm)

| Quantity | Reference | Model before | Model now |
|---|---|---|---|
| Seat height (front edge, top) | 450 – 470 (EN 1729 size 6: 460) | 467 | 467 |
| Seat depth / width front / width back | 400 – 430 / 400 – 450 / 30 – 50 narrower | 430 / 430 / 398 | unchanged |
| Seat slope | 2 – 4 degrees to the back | 2 | 2 |
| Seat board | 18 – 20 solid beech or 10 – 12 ply | 20, notched round the posts, front corners R30 | unchanged |
| Back top above floor, back rake | 820 – 900, 5 – 15 degrees | 882, 8 | unchanged |
| Tube | diameter 20 – 25, wall 1.5 – 2; rear leg bends into the post | 22, bend radius 80 | unchanged |
| Leg splay | front 2 – 4 degrees, rear raked 8 – 12 degrees | 2.5 / 7.7 | unchanged |
| Side stretchers | diameter 12 – 16 at 120 – 200 | 16 at 150 | unchanged |
| Back board and rail | formed beech ply 8 – 12 thick, radius 400 – 700, board 100 – 180 high | **18 thick**, sagitta 31 over 410 (radius about 680), 170 and 50 high | **12** (15 at 1:100) |
| Fixings of back and seat | rivets or bolts through tube and board, heads 10 – 14 | **none modelled** | 6 dome rivets, 4 coach bolts (1:50) |
| Glides, caps | 22 – 28 | 25 / 22 | unchanged |
| Mass | 5 – 8 kg | 7.8 | 7.4 |

### Defects found

- 1:50 and 1:100 body: back board and rail were 18 mm planks, outside the reference (8 – 12 mm formed ply); the side view read as two timber battens.
- 1:50 body: no joinery at all between timber and tube (boards only tangent to round posts, apron only coped round the tubes).
- Root: thermal layer of `back-beech` 18 mm, mass of the `back` bone and total mass computed with the 18 mm boards.
- Everything else (rig, structure, thermal lists, coarse scales, poses, clips) judged right: the chair is rigid, so all members are fixed to `ground` and the poses move the root (pulled out, tipped about the rear glides); no lateral pivot exists, a lateral bone would be wrong here. Structure is the true load path (two bent side tubes, front legs, stretchers welded, seat frame bolted at four corners, boards on the posts, four glide supports, masses at member centres) and one connected group at every scale.

### Rebuilt (`rework-cafe-chair-joinery.py`, `rework-cafe-chair-root.py`)

- Body 1:50: `back-board` and `back-rail` lofts 12 mm, shifted so they still lie on the post fronts; six dome-head rivets through board and tube (`rivet-*`, two per board end, one per rail end, axis normal to the curved face); four coach bolts through the tubes into the coped apron corners (`bolt-seat-*`). Body 1:100: boards 15 mm (the feature), same seat on the posts. 1:200 and 1:500 unchanged (30 and 75 mm slabs are the feature there).
- Thermal and structure follow through their `of` twins; region `back-beech`: layer 12 mm, R 0.0667, heat capacity 13 824 J/m²K, area 0.205 m².
- Rig: `back` bone mass 1.64 kg, comX 0.25; `dimensions.massKg` 7.4 (sum of bone masses).
- 2D, redrawn by hand: elevation board and rail rectangles at 1:50 (y −0.0283, h 0.0433) and 1:100 (y −0.0313, h 0.0463); plan board and rail front arcs moved back 5.9 mm (1:50) and 3 mm (1:100). `r3-project2d --check`: 1:50 IoU 0.983 / 0.986 / 0.994, 1:100 0.974 / 0.981 / 0.997 (elevation / front / plan).
- Texts: volume decisions 1:50 and 1:100, provenance notes, `representations.50.decisions.omitted` and `representations.100.decisions.thinElements`, en and de.

### Validation

`VALID furniture/cafe-chair (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`r3-validate.ts furniture/cafe-chair --catalog-only --volumes`, after the last write). Published with `volumes-insert.py`; source and catalog agree.

Rendered and read after the last change: body at 1:50 (also three-quarter close-up), 1:50 poses `tilted-on-two-legs`, `seated-occupied`, clips `pull-out-and-sit` t 1.0 and `tip-back` t 0.75 and 1.8, structure 1:100 and 1:200, thermal 1:50 cut, 1:100, 1:200, 1:500, 2D 1:50. Read before the change and not touched by it: body 1:200 and 1:500, structure 1:50 and 1:500, thermal 1:50. Rendered but not read: `pulled-out`, `seated-pulled-out`, `pull-out-and-sit` t 2.0 (pure root translations of the identical body; the sheet recentres, so they equal the default sheet), body 1:100 after the 15 mm change, structure 1:50 after the change (members are `of` twins of the body).

### Remaining imperfect

- 1:50 seat board has sharp arrises (no 3 mm round-over, no saddle); below the 7.5 mm feature.
- Rivets and bolts are not drawn in 2D (12 – 13 mm heads are under the 0.2 mm print minimum at the 1:75 end of the band); the text says so.
- `provenance.notes` still says the back is "screwed" in one sentence that I did not rewrite beyond the thickness.
- 1:500: the raked rear tubes emerge behind the 75 mm seat slab as two pointed tips in the back view; true to the geometry, but a coarser sign could carry the slab back over them.
- Mass 7.4 kg and area 0.205 m² are hand-computed from the solids, not measured by a tool.

### Fundamental issues

- None in kernel or validator met. Tool note: `r3-sheet.ts` recentres every sheet on the model, so a pose that only translates the root (`pulled-out`) cannot be told from the default pose on a sheet.
