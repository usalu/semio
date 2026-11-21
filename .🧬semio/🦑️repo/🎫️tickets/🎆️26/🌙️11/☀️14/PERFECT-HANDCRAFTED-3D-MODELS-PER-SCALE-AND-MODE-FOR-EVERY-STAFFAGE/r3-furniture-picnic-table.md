# Round 3 — furniture/picnic-table

No earlier round-3 work existed (no `r3-picnic-table-*`, no report, no generated folder); started at step 1. The catalog `volumes` were identical to `volumes/furniture/picnic-table.json` at the start (the 03:40 bulk passes had not touched this asset).

Scripts (all in the ticket folder): `r3-picnic-table-kit.py` (measures, every part of every scale stated by hand in world coordinates, part to bone-local solid), `r3-picnic-table-lists.py` (body, thermal, structure, decisions), `r3-picnic-table-draw.py` (2D of every scale and view from the same parts, per part and view: shown or hidden, paint order), `r3-picnic-table-roots.py` (rig, anchors, dimensions, thermal, texts, symbol, references), `r3-picnic-table-build.py` (candidate document), `r3-picnic-table-apply.py` (writes roots into the catalog file), `r3-picnic-table-ship.sh` (build, validate candidate, apply, splice, validate catalog).

## 1. Reference

| measure | real subject | source | model |
|---|---|---|---|
| length of top | 1.8 m (6 ft) | classic A-frame picnic table drawings (US Forest Service / park service type), market products | 1.80 m |
| top width, height | 0.70-0.76 m wide, 0.72-0.76 m high | same | 0.70 m, 0.75 m |
| top planks | five boards about 140 x 40-45 mm with 6-10 mm drainage gaps | same | 5 x 132 x 40 mm, 10 mm gaps, 5 mm eased edges |
| seat height, seat width | 0.43-0.46 m, two boards (about 0.29-0.30 m) | same; EN 581 seating range | 0.44 m, 2 x 145 x 40 mm with 10 mm gap = 0.30 m |
| gap between top edge and seat | 0.08-0.12 m | same | 0.11 m |
| overall width | 1.5-1.6 m | same; asset `dimensions` 1.52 m | 1.52 m |
| legs | 2 x 4 to 2 x 6 timber (about 95-145 x 45 mm) raked about 60 degrees to the ground, level cuts | same | 95 x 45 mm at 60.0 degrees |
| A-frame joints | cleat under the top and seat support lapped on the leg faces, clamped by 3/8 in (M10) carriage bolts | same | 95 x 45 mm cleat and stretcher in a second layer on the outer leg faces, M10 carriage bolts, M12 pivot bolt |
| bracing | two diagonal braces from the seat supports to a centre cleat under the top | same | 2 x 70 x 45 mm braces, centre cleat 70 x 45 mm |
| fold-down bench hardware | heavy folding seat brackets: 6-8 mm galvanised plate, M12 pivot, spring plunger locking in both positions | manufacturers' data of locking fold-down brackets (generic, no single datasheet) | 8 mm folded gusset (web and 40 mm flange), 8 mm bearing plate 88 x 120 mm, M12 pivot, spring plunger with stow hole |
| density | pine 500 kg/m3, steel 7800 kg/m3 | asset materials (ISO 10456, EN 1993-1-1) | masses from modelled volumes |

## 2. Audit (before)

- Mechanism impossible: the benches were 1.70 m boards crossing both A-frame planes; in `stowed` and through both fold clips they swung straight through the legs (seen in the render of the stowed pose). Nothing held a bench level against its own fold direction.
- Legs 50 x 45 mm (drawn 60 mm horizontally at 56.5 degrees): matchsticks; real 95 x 45 mm. Cleat and stretcher 45 x 45 mm in the same plane as the legs (interpenetrating, no lap). No braces, no centre cleat. Top planks without gaps. Bench one slab 0.36 m wide, 50 mm from the top edge. Hinges four steel blocks 80 x 100 x 110 mm.
- Rig: `ground` 1.52 m long (a load-bearing "member" that had to be faked with foot boxes in the structure list); `frame` a 1.4 m horizontal bone on the ground from the centre to beyond the right foot; bench joints at the board underside in mid-air (0.46, 0.40), not on a pivot.
- Anchors: feet at x = +-0.69 (legs end 0.66-0.72); sitting points 0.12 m from the joint.
- Structure lists: 1:100 a copy of 1:50; masses lying on the surfaces (qa-visual).
- Texts contradicted each other (benches "swing up" and "hang down").

## 3. Root fixes (old -> new)

- Rig: `ground.lengthM` 1.52 -> 0. `frame` (0, 0, 0 deg, 1.4 m) -> vertical (0, 0, 90 deg, 0.71 m: ground to underside of the top), mass 15.0 -> 20.7 kg, com (0, 0.4) -> 0.427 m up the bone. `top` now in the frame's vertical frame (0.71, 0.35, -90 deg; world joint unchanged at (-0.35, 0.71)), mass 25.0 kg kept (model gives 25.0). `bench-r` / `bench-l` joints from (+-0.46, 0.40) to the pivot bolts (+-0.45, 0.375) through leg centre line and stretcher, length 0.30 -> 0.31, mass 10.8 -> 13.2 kg each, com from the model. Pose angles are offsets from rest, so poses and clips are unchanged and now read right.
- `dimensions.massKg` 61.6 -> 72.1 (sum of the bones). Height, width, depth unchanged (0.75, 1.52, 1.80).
- Anchors: feet to x = +-0.665 under the real legs (frame-local y = -+0.665), footprint 0.09 x 0.05 -> 0.11 x 0.045; `seat-r` / `seat-l` (0.12, +-0.04) -> (0.16, +-0.065) = middle of the plank tops.
- Thermal: `areaM2` table-top 2.96 -> 3.07, bench-seats 2.7 -> 2.26, a-frame 1.2 -> 2.32, hinges 0.22 -> 0.39 (measured outer areas of the 1:50 model, hand sums agree). a-frame (factor 1.93) and hinges (factor 1.78) exceed the factor 1.5 of C16: they are applied because the construction itself was corrected (real sections, braces, real brackets), and listed for the area reviewers in `area-review.md` (section 2026-11-20). Hinge layer 5 mm -> 8 mm (resistance 0.00016, capacity 28080); a-frame layer title 45 x 95; characteristic lengths updated; assumption 3 rewritten.
- Texts: summary, `stowed` pose, both fold clips, the deploy event, provenance notes, structure assumption 1, frame bone title; two references added (`usda-fs-picnic-table`, `folding-seat-brackets`; the second is generic manufacturers' data, not one datasheet).
- Symbol `picnic-table-symbol` redrawn on the new construction.
- 2D: all four representations redrawn in all three views from the same parts as the solids (E1); 1:50 hatches: grain along each leg (60 / 120 degrees), horizontal on beams and plank ends, along the planks in plan.

## 4. Models

Mechanism (all scales): each bench hangs between the A-frames (boards 1.50 m, frame inner faces 1.555 m apart) on two folded gusset brackets that turn on the M12 pivot bolts; in `stowed` it hangs plumb at x = 0.475-0.515 m, 65 mm above the ground, clear of legs, stretchers, braces and top.

| scale | body / thermal / structure entries (expanded body solids) | what is modelled |
|---|---|---|
| 1:50 | 52 / 52 / 52 (66) | 5 eased planks, centre cleat; per A-frame 2 raked legs with eased toe, bevelled cleat and stretcher lapped outside, brace; M10 dome heads and nuts, M12 pivot nut and washer; per bench 2 eased planks, batten, 2 gusset webs, 2 flanges, pivot heads, plungers; 4 bearing plates with open stow hole. Thermal: reference per body solid with its layer. Structure: every timber and gusset as member, bearing plates, 4 pivot bolts, 12 lap bolts, 15 plank screws as joints, 4 foot pads, 5 masses inside the timber |
| 1:100 | 24 / 24 / 44 (26) | square planks with gaps, centre cleat, legs, bevelled cleats, plain stretchers, braces, bench planks, batten, bracket as 15 mm web with its flange at the 15 mm floor; no bolts. Structure with 15 mm pins, one lap bolt per cleat joint |
| 1:200 | 17 / 17 / 32 (17) | top slab, legs, square cleats, stretchers, braces, one board per bench, bracket as 30 mm block between board end and leg. Structure: pivot pins 30 mm, seat blocks of the slab on the cleats |
| 1:500 | 9 / 9 / 24 (9) | 75 mm top slab, each A-frame one 90 mm plate of legs and stretcher, 75 mm bench boards, 75 mm bracket blocks under the board ends; braces omitted (45 mm). Structure: 75 mm pivot pins, seat blocks, foot pads, masses inside |

Final check: `volumes-check.ts furniture/picnic-table --product` -> `PASS (12.5 s)`; silhouette IoU 1:50 0.991 / 1.000 / 0.999, 1:100 0.991 / 1.000 / 0.999, 1:200 1 / 1 / 1, 1:500 1 / 1 / 0.995; thermal area deviation at 1:50 under 0.4 % per region. `r3-validate.ts furniture/picnic-table --volumes` -> `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Spliced.

## 5. What I saw (renders opened)

- 1:50 body sheet, six views: reads at first sight as a bolted A-frame picnic table; front shows both frames in two layers, braces rising to the centre cleat, bench between the frames; right and left side show the lapped cleat and stretcher over the raked legs with bolt heads, brace end-on, bench plank ends and gusset wedges behind the legs; top shows 5 planks with gaps, 2 planks per bench, legs and stretchers reaching out; three-quarter from above and from below (turn 200, tilt -20): gusset webs and flanges under the planks, bearing plates with stow hole on the legs, nuts on the inner leg faces, brace butting on the stretcher.
- 1:50 `stowed` sheet and `fold-benches` at 1.5 s (right side): benches turn about the pivot bolts and hang plumb between the frames, nothing interpenetrates.
- 1:50 structure (six views) and thermal with `--cut`: regions separate cleanly (top, benches, frame timber, steel); structure is all timber and gussets as members with small orange joints and green pads; masses are inside the timber and not visible from outside.
- 1:100, 1:200, 1:500 body sheets; 1:500 structure sheet; 1:200 thermal rendered. 2D sheets of 1:50 (deployed and stowed) and 1:500 opened; 1:100 and 1:200 2D rendered and covered by the silhouette numbers but not opened.

## 6. Still imperfect

- Structure at 1:50 and 1:100 is mostly `of` references to the body: in a bolted timber table every timber is a member (D3 allows this), but the list looks like the body in blue; only bolts, plates, pads and masses are its own.
- The plunger and pivot bolt head stay with their bone; holes in gusset and timber for bolts are not cut (below the feature), only the open stow hole in the bearing plate.
- Plank screws are not in the body (5 mm heads); frame timbers have sharp edges (3 mm easing is below 7.5 mm).
- 1:100 flange is drawn 15 mm thick (real 8 mm) so the steel region keeps an honest area (measured deviation 29.7 %, limit 30 %: it passes with almost no margin).
- a-frame and hinges `areaM2` changed by more than the factor 1.5 (see section 3); the area reviewers have the rows.
- The older scripts `author-16-picnic.py` and `author-50-picnic2d.py` describe the superseded model; left in place (not mine to delete).

State: DONE

## Round 5 handcraft (2026-11-21)

Sheets rendered before and after the fix with `sh r5-sheets.sh furniture/picnic-table before|after "stowed occupied bench-proof-load half-folded-key" "fold-benches 1.5"`. Opened:
- before: body 1:50 (six views with rig); a montage of body 1:100 / 1:200 / 1:500 and structure 1:50; a montage of the 1:50 thermal cut, structure and thermal cut at 1:100 / 1:200 / 1:500, and `fold-benches` at 1.5 s; a montage of the poses `stowed`, `occupied`, `bench-proof-load` and `half-folded-key`; the 1:50 body from below;
- after: body 1:50 and 1:200, structure 1:200 and the 1:200 2D; the 2D at 1:50 and 1:100, the 1:200 thermal cut and `stowed` 1:50.

Measurements: `r5-furniture-extent.ts furniture/picnic-table 50`.

### Reference against model (1:50, pose deployed)

| Quantity | Reference (USDA FS / NPS A-frame table, EN 581) | Model | Deviation |
| --- | --- | --- | --- |
| Top length x width x height | 1800 x 700-760 x 720-760 mm | 1800 x 700 x 750 | 0 % |
| Overall width | 1500-1600 mm (asset 1520) | 1520 | 0 % |
| Top planks | 5 x 2 x 6 (about 140 x 40), 6-10 mm gaps | 5 x 132 x 40, 10 mm gaps | in range |
| Seat height / width | 430-460 / about 300 mm | 440 / 300 | 0 % |
| Horizontal gap, top edge to seat | 80-120 mm | 110 | in range |
| Legs | 95 x 45 mm raked about 60° | 95 x 45 at 60.0° (x extent 0.514 m) | 0 % |
| Centre cleat | 2 x 4 on edge (89 x 38-45 mm) | before 70 x 45; now 89 x 45 | before -21 %, now 0 % |
| Braces | 2 x 3 / 2 x 4 from the seat supports to the centre cleat | 70 x 45, ending on the cleat | 0 % |

### Grades

| Scale | Body before / after | Thermal | Structure before / after |
| --- | --- | --- | --- |
| 1:50 | A- / A | A | A- / A- |
| 1:100 | A- / A | A | A- / A- |
| 1:200 | B+ / A | A | B+ / A- |
| 1:500 | A- / A- | A | A- / A- |

Defects found:
- 1:200: the centre cleat was missing, although at 45 mm it is above the 30 mm feature. The decision called it "merged into the slab", but the slab is only 40 mm. So the two braces ended under the slab with a 46 mm sliver between their tips.
- 1:50 and 1:100: the 70 mm centre cleat was shallower than the plumb cut of the braces (75 mm). The brace tips stood 7 mm below the cleat, leaving a notch.
- The catalog was INVALID under the print check: `range-end-gap` for brace+brace-b at 1:150 (bands 1:100 and 1:200), and for top planks 3+4 at 1:20.

Root fixes:
- **Kit.** In `r3-picnic-table-kit.py`, `MID_Y0` changed from 0.64 to 0.621, making the centre cleat 89 x 45 mm (a 2 x 4 on edge) that takes up both brace ends. `parts200` gains the `top-cleat`.
- **Draw script.** `r3-picnic-table-draw.py` now writes the valid ranges. The 1:50 band starts at 1:30 instead of 1:20: finer than that, the 10 mm plank gaps open to a sliver under the 0.5 mm pen.
- **Decisions and notes.** The 1:200 body and drawing decisions and the provenance notes are rewritten (en, de).
- **Rebuilt.** All four scales were rebuilt with build, then apply, then `volumes-insert.py`. The masses follow from the model: top 25.0 to 25.3 kg, `massKg` 72.1 to 72.4.
- **Solid counts.** 1:200 is now body 18, thermal 18, structure 33 (was 17 / 17 / 32). The other scales are unchanged in count.

Checks: `r3-validate.ts furniture/picnic-table --volumes` gives `VALID ... schema 0, validateAsset 0, checkVolumes 0`. `volumes-check.ts --product` gives `PASS (23.5 s)` with 41 lines ok.

What I saw after the fix:
- At 1:200 the braces now meet the centre cleat in the long-side view, in 3D and in 2D.
- 1:50 is unchanged except for the deeper cleat.
- `stowed` still hangs plumb and clear.

Open points:
- The structure at 1:50 and 1:100 is mostly the timber itself (allowed by D3).
- The plank screws are not modelled.
- The a-frame and hinge areas are still with the area reviewers (round 3).
