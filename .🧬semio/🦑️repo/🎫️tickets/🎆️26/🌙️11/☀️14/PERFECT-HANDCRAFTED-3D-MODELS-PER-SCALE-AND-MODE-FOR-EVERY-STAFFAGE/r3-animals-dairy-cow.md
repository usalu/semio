# Round 3 — `animals/dairy-cow`

State: spliced, catalog asset `VALID` (`r3-validate.ts animals/dairy-cow`: schema 0, validateAsset 0). `volumes-check.ts animals/dairy-cow --product` is **not green**: 6 measurements of `volumes-thermal-coverage` fail (areas of `lower-legs` and `hooves` at 1:50 and 1:100, the totals at 1:200 and 1:500); every other rule passes at every scale. The failing values are physiological shares that an author must not change (C18, C27); the model is at true size and was not bent to them (request in `author-requests.md`, "r3 animals/dairy-cow"). Grade by my own eye: body B+ at 1:50, B at 1:100 and 1:200, B- at 1:500; structure B+; thermal B+; 2D elevation B+, front and plan B. It is a Holstein cow at first sight at every scale; it is not "perfect" (section 6).

I resumed the work of an earlier author who was cut off: the trunk rig fix (`r3-dairy-cow-rig.py`) was already applied to the catalog file, which left the catalog INVALID (the 2D trunk shapes had moved 0.27 m up with their bones); a 1:50 body was half done. I kept the rig fix, repaired a bug in the kit (`pieces()` dropped every inner station of the last piece), rebuilt the body and did everything else.

Scripts (all in `T`): `r3_dairy_cow_kit.py` (lofts, straps, hoops, plates stated in the world or a bone frame), `r3_dairy_cow_body.py` (every station of the body, four scales), `r3_dairy_cow_frame.py` (skeleton, four scales), `r3-dairy-cow-model.py` (lists, thermal, emitters, decisions), `r3-dairy-cow-draw2d.ts` (ruling E1), `r3-dairy-cow-root.py`, `r3_dairy_cow_io.py` (member-level writes of the catalog file), `r3-dairy-cow-rig.py`, `r3-dairy-cow-lying.py`, `r3-dairy-cow-material.py`, `r3-dairy-cow-decisions.py` (root fixes), `r3-dairy-cow-probe.ts`, `r3-dairy-cow-bbox.ts`, `r3-dairy-cow-inside.ts` (measurements).
Chain: `python3 r3-dairy-cow-model.py && $BUN r3-dairy-cow-draw2d.ts && python3 r3-dairy-cow-root.py "🗑️generated/r3-dairy-cow/members.json" [--candidate] && python3 volumes-insert.py volumes/animals/dairy-cow.json <asset>`.

## 1. Reference (adult Holstein-Friesian cow, 650 kg, withers 1.45 m)

Sources: my knowledge of Dyce, Sack, Wensing, *Textbook of Veterinary Anatomy* (the asset's `dyce-2010`), Ellenberger, Dittrich, Baum, *An Atlas of Animal Anatomy for Artists*, and Holstein linear-type measurements (stature, rump width, heart girth). No web lookup; values are from memory and carry about ±3 cm. Heights above ground, x ahead of the hip joint.

| landmark | real | rig / model now |
|---|---|---|
| withers; sacrum (stature); lowest point of the back | 1.45; 1.46-1.48; 1.40 | 1.455; 1.47; 1.40 |
| hook bones (tuber coxae): height, width | 1.38-1.42, 0.55-0.58 | 1.39, 0.52 |
| pin bones (tuber ischii): height, width | 1.25-1.30, 0.30-0.35 | 1.285, 0.34 |
| hip joint; thurl width | 1.10-1.15; 0.50 | 1.104 (rig); 0.53 |
| stifle | 0.75-0.82 | 0.80 (rig) |
| hock joint centre; point of hock | 0.46-0.50; 0.55-0.60 | 0.424 (rig, kept); 0.50 |
| hind fetlock; fore fetlock | 0.14-0.15 | 0.135 |
| carpus; elbow; point of shoulder | 0.40; 0.76-0.80; 0.95-1.03 | 0.394; 0.783; 0.972 (rig) |
| scapula; humerus; radius; metacarpus | 0.42-0.50; 0.33-0.35; 0.31-0.33; 0.21-0.22 | 0.46; 0.36; 0.39 (to mid carpus); 0.26 (carpus to fetlock) |
| femur; tibia; metatarsus with distal tarsus | 0.44-0.48; 0.39-0.41; 0.32-0.33 | 0.40; 0.48; 0.29 (rig, kept) |
| body length, point of shoulder to pin bone | 1.65-1.75 | 1.74 |
| chest floor at the girth; chest depth; heart girth | 0.66-0.68; 0.78; 2.05 | 0.665; 0.79; about 2.0 |
| barrel at the rumen: width, belly line | 0.70-0.75; 0.60-0.62 | 0.73; 0.61 |
| lumbar shelf width (transverse processes) | 0.40-0.45 | 0.43 |
| skull length; width at the orbits; depth with mandible; muzzle width | 0.50-0.55; 0.22-0.23; 0.24-0.27; 0.14-0.15 | 0.53; 0.23; 0.27; 0.15 |
| neck: depth at the shoulder / throat, width | 0.45-0.55 / 0.20-0.22, 0.15-0.25 | 0.54 / 0.20, 0.15-0.24 |
| cannon girth fore / hind | 0.20-0.21 / 0.22-0.23 | 0.21 / 0.22 |
| claw: sole length, width, height at the coronet | 0.11-0.13, 0.05, 0.065-0.075 | 0.112-0.118, 0.05-0.054, 0.068 |
| udder: length, width, floor; teats | 0.45-0.55, 0.30-0.35, 0.52-0.56; 0.06 x 0.025-0.03 | 0.58, 0.34, 0.54; 0.065 x 0.028 |
| tail: tip height, switch | at the hock or below | 0.33, switch 0.26 x 0.08 |
| vertebrae and ribs | C7, T13, L6; 13 pairs of ribs, 8 sternal | 7 / 13 / 6; 13 pairs, 8 closed |

## 2. Audit of what was there

- Rig: `pelvis`, `lumbar`, `thorax` ran at the height of the hip joint (1.10 m), 0.27 m below the vertebral column (fixed by the earlier author, kept). The hock joint is about 5 cm low (0.424 against 0.46-0.50), because the tibia bone is 0.48 m and the femur 0.40 m (real 0.40 and 0.45): not corrected, see section 6. Fore limb joints are within my reference uncertainty.
- Pose `lying-resting`: the near hind hoof hung 0.25 m below the plane the cow lies on (contact heights -0.783 against -0.53), the point of the near hock 4 cm above it, the udder was turned by 85 degrees so that its rear half went 0.15 m into the ground.
- Dimensions: 1.49 x 2.55 x 0.72 m (height above the real stature of 1.47).
- 2D: every view of every scale was a dump of earlier projections (elevation 1:50 about 13 000 points, front and plan about 5 000 to 6 500 each); tones contradicted the materials (white-coat trunk drawn dark, black-coat head light); at 1:200 the trunk lay over the near thigh; in the front views the chest lay over the forelegs; after the rig fix the trunk shapes floated 0.27 m too high (asset INVALID, 7 extent findings).
- 3D: 40 solids at 1:50 as one loft per bone, patches as blocks, structure one capsule per bone with a ball per joint, 1:100 a re-cut of 1:50; hooves 1.7 times the real size and the udder too small (area reviewers' verdict).

## 3. Root fixes in the asset (old -> new)

- `rig` (earlier author, `r3-dairy-cow-rig.py`): pelvis angle 0 -> 57.80, length 0.25 -> 0.319; lumbar joint at (0.17, 1.374 m), thorax joint at (0.62, 1.374 m) falling 15.6 degrees to T1 at (1.30, 1.184 m), length 0.70 -> 0.706; every child bone, mass and anchor keeps its world position; anchors `contact-sternum` on the chest floor (h 0.62), `back-plume` on the back; pose `grazing-head-down` thorax -2.817.
- `poses.lying-resting` (`r3-dairy-cow-lying.py`): thigh-hn 38 -> 31, shank-hn -40 -> -47.0, metatarsus-hn 51.2 -> 105.7, shank-hf -41.4 -> -46.8, metatarsus-hf 103.6 -> 109.5, forearm-fn 61.5 -> 70.1, forearm-ff 59 -> 62.3, udder 85 -> 0. All seven contacts now lie within 6 mm of one plane (the plane of sternum and belly).
- `materials`: added `bone` (density 1900 kg/m3, emissivity 0.94, absorptivity 0.45, authored estimate) for the skeleton.
- `dimensions`: 1.49 x 2.55 x 0.72 -> 1.47 x 2.54 x 0.73 m, measured on the 1:50 body (stature 1.47 m is the real value of the table); the same two values in the dairy-cow row of `🗂️catalog/🔣️.json`.
- `representations`: every filled shape of every scale and view redrawn as the outline of the solids it stands for (same ids, roles, bones, regions): 789 / 755 / 710 points at 1:50 for elevation / front / plan, 560 / 516 / 511 at 1:100, 340 / 309 / 294 at 1:200, 161 / 136 / 120 at 1:500. Patch shapes are the visible part of the patch (depth test), tones follow the materials (white 0.9, black 0.14, far limbs 14 % darker). Strokes redrawn by hand: nostril, mouth line, coronet lines, knee line, two coat strokes, neck fold (1:50), hip line (1:100), ear stroke (1:500). Changes of the shape lists: `rumen-flank` removed from front and plan at 1:50 and 1:100 (a sliver on both flanks; drawn in the elevation); `patch-rump` and `patch-barrel` added at 1:200; at 1:500 the four `*-hind-leg` / `*-fore-leg` strokes became filled outlines; layer order corrected: forelegs over the chest in every front view (z 3.25), `mass-200-lumbar` z 5.5 -> 3. The eight `decisions` texts that described the old construction were rewritten (en, de).
- Not changed: `thermal` data and every `areaM2`, `structure` data, clips, the other poses, leg bones.

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 81 solids: barrel, back with lumbar shelf and withers, rump, hook and pin bones, sacral crest, brisket; saddle, croup and chest patch as straps on the skin; per hind leg thigh, stifle, gaskin, point of hock, cannon, pastern, 2 dewclaws, 2 claws; per foreleg shoulder, upper arm, elbow point, forearm, cannon, pastern, dewclaws, claws; neck, crest, dewlap, head, poll, brows, eyes, blaze, muzzle, nostrils, jaw, ears, udder halves, 4 teats, tail in 4 | 86: the body per region and coat layer, a rumen-flank strap per side, 2 nostril outlets, back outlet | 154: skull, orbits, 2 mandible halves, atlas and 6 cervical vertebrae, spine in 3, 13 thoracic and 6 lumbar processes, 6 transverse processes, sacrum and crest, 13 pairs of ribs (8 hoops, 5 open arcs), sternum, pelvis girdle, limb bones to the claw bones, patellae, trochanters, calcanei, olecrana, caudal vertebrae, ear cartilages, udder ligament, 30 joints, 3 masses |
| 1:100 | 50, own tables with fewer sections, joint knobs merged, hoof blocks, one udder bag | 55 | 87: 7 rib pairs, processes as fins, one mandible |
| 1:200 | 35, one mass per bone, saddle and croup patch kept | 43, muzzle and eyes as own solids | 66: 4 rib pairs, hip bones, digits as bars |
| 1:500 | 30, two trunk eggs in one mixed tone, two-piece lower legs with hoof, ears as texture solids | 40, patches, flank, eyes, muzzle as own solids | 46: beams and bars, 2 rib pairs |

Final check: `FAIL: 6 measurement(s) over 4 rule line(s)`: `areaDeviation:lower-legs` 0.39 (1:50) and 0.45 (1:100), `areaDeviation:hooves` 3.02 and 1.87, `areaDeviationTotal` 0.32 (1:200) and 0.37 (1:500). Silhouette IoU elevation / front / plan 0.995 / 0.988 / 0.996 at 1:50, 0.994 / 0.985 / 0.993 at 1:100, 0.989 / 0.984 / 0.989 at 1:200, 0.979 / 0.960 / 0.949 at 1:500 (they follow from the redraw and prove nothing). No support solids: `structure.supports` is empty in the asset.

## 5. What I saw (renders opened with the Read tool)

- 1:50 body, six-view sheet, default pose: right and left side read as a dairy cow at once: level back, withers, boxy rump with hook and pin bones, deep barrel with the belly lowest at the rumen, udder under the flank with teats, deep narrow neck with dewlap, wedge head carried low, cloven hooves. Front: black face with blaze, ears out sideways, round barrel behind a narrow chest. Back: square rump, udder between the hind legs, tail down the middle. Top: barrel widest behind the ribs, saddle and croup patch across the back. Three-quarter: the patches lie on the skin with feathered, slightly stepped edges.
- 1:50 grazing (six views): closed at every joint, the neck base holds. 1:50 lying (right side, before and after the pose fix): after the fix belly, sternum and all legs lie on one plane; the rigid udder dips about 6 cm into it.
- 1:50 structure sheet (twice): a bovine skeleton at first sight; the first sheet had the five rear rib arcs opened sideways, rebuilt as arcs and seen again. 1:50 thermal cut sheet: regions follow the body, flank straps and back outlet lie on the surface.
- 1:100 body six-view sheet and the 1:100, 1:200 and 1:500 `volumes-render` sheets (overlays, body, thermal, structure): cow at a glance in every list; at 1:500 the ear stroke shows red in the plan overlay (ears are texture solids, section 6).
- 2D sheets (`r3-2d.ts`) 1:50, 1:200, 1:500 and a walk-cycle frame at 1:100 (t = 0.3 s): Holstein in line and tone, limbs stay closed in the walk frame.
- Not opened: the per-list single views of 1:100 from four directions, the 2D sheet of 1:100 in the default pose, the clips other than one walk frame, the poses `kneeling-front` and `lowering-hindquarters`.

## 6. Still imperfect, plainly

- Hind leg bones of the rig are not corrected: hock joint about 5 cm low, tibia 0.48 m (real 0.40), femur 0.40 m (real 0.45). A correction needs an inverse-kinematic re-solve of 13 poses and four clips; with a reference from memory (±3 cm) I did not do it. The point of the hock is modelled at 0.50 m, still 5 to 8 cm low.
- The check is red on the six area measurements above; the decision lies with the area reviewers.
- Body: sections are symmetric superellipses, so the masses meet in visible seams (thigh plate on the rump, shoulder on the chest); hook and pin bones read as oval bosses; patch edges are stepped where a strap dives into the facets of the trunk; the patches are the same on both sides; no paralumbar hollow, no milk veins, no horns (dehorned cow), no skinning weights. The head is slightly long in the muzzle.
- A vertical seam line at the thoracolumbar joint shows in the 2D elevations (two trunk shapes of the same tone).
- 2D front and plan: teats and tail switch stay `solid: extruded` prisms, so they are rectangles there; the rumen flank is not drawn in front and plan; at 1:500 the ears are a stroke while the 3D ears are `texture` solids (a 75 mm thick ear was the alternative).
- Lying pose: the udder is rigid and dips into the ground; the lie-down clip was not re-checked frame by frame after the pose fix.
- Structure: bones use the new material `bone` with authored thermal numbers; the scapula is a tapered plate with a spine bar; costal cartilages are not modelled (sternal ribs are full hoops).
- Not run: `repo` test targets and the model export.
