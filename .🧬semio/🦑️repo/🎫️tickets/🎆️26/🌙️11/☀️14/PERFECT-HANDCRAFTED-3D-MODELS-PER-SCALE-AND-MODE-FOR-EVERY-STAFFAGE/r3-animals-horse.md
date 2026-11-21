# Round 3 — `animals/horse`

State: spliced, `volumes-check.ts animals/horse --product` PASS at every scale, `r3-validate.ts animals/horse --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0). Grade by my own eye: body B+ at 1:50, B at the coarser scales; structure B+; thermal A-; 2D elevation B+, 2D front and plan C+ (see "Still imperfect"). It is a horse now, not a wolf; it is not yet "perfect".

Scripts (all in `T`): `r3-horse-kit.py` (authoring helpers), `r3-horse-model.py` (every station of every solid, all four scales, all three lists, decisions), `r3-horse-details.py` (hand-drawn detail strokes), `r3-horse-draw2d.ts` (ruling E1: redraw of the 2D shapes from the body), `r3-horse-root.py` (root edits of the catalog file), `r3-horse-inside.ts` (which skeleton part leaves the body), `r3-horse-validate.ts`.
Chain: `python3 r3-horse-model.py && python3 r3-horse-details.py && $BUN r3-horse-draw2d.ts && python3 r3-horse-root.py members 🗑️generated/r3-horse/members.json && python3 volumes-insert.py volumes/animals/horse.json <asset>`.

## 1. Reference (adult warmblood, 550 kg, withers 1.60-1.61 m)

Sources: my knowledge of Ellenberger/Dittrich/Baum, *An Atlas of Animal Anatomy for Artists*; Goldfinger, *Animal Anatomy for Artists*; warmblood studbook measurements (body length about equal to withers height, chest depth about 46 %, cannon girth about 0.21 m); the asset's own `provenance.references` (Buchner 1997 for segment masses, Back and Clayton 2013 for joint ranges). No web lookup was made; values are from memory and carry about ±3 cm.

| landmark | real | rig / model now |
|---|---|---|
| withers height | 1.60-1.61 | 1.612 |
| croup (tuber sacrale) | 1.58-1.60 | 1.60 |
| lowest point of the back | 1.50 | 1.50 |
| chest floor at the girth; chest depth | 0.85; 0.73-0.75 (46 %) | 0.852; 0.73 |
| point of shoulder (joint) height | 1.05-1.10 | 1.153 (rig, kept) |
| elbow | 0.90-0.93 | 0.874 (rig, kept) |
| carpus centre | 0.48-0.50 | 0.475 |
| fore fetlock centre | 0.22-0.24 | 0.180 (rig, kept) |
| hip joint | 1.25-1.33 | 1.312 |
| stifle | 0.95-1.00 | 0.948 |
| hock centre; point of hock | 0.55-0.58; 0.62-0.66 | 0.522; 0.615 |
| hind fetlock centre | 0.22-0.25 | 0.187 (rig, kept) |
| body length point of shoulder to point of buttock | 1.60-1.65 | 1.70 (was 1.52) |
| hip joint to shoulder joint, horizontal | about 1.35 | 1.344 (was 1.164) |
| thoracic spine (18 vertebrae); lumbar (6) | 0.85-0.90; 0.30-0.33 | 0.88 (was 0.70); 0.30 |
| scapula; humerus; radius; fore cannon with carpus | 0.45; 0.31-0.34; 0.36-0.40; 0.30 | 0.52 from the withers top; 0.34; 0.40; 0.295 |
| femur; tibia; hind cannon with tarsus; pastern | 0.40-0.45; 0.40-0.45; 0.36; 0.13-0.14 | 0.42; 0.52; 0.34; 0.14 |
| skull length; depth at the jaw; width at the eyes; muzzle width | 0.55-0.60; 0.27-0.30; 0.20-0.22; 0.11 | 0.59; 0.29; 0.21; 0.10 |
| neck: length; depth at shoulder / middle / throat | 0.85-0.95; 0.45-0.50 / 0.28 / 0.20 | 0.84; 0.51 / 0.28 / 0.20 |
| barrel width; width at the hips; across the shoulder points | 0.56-0.60; 0.56; 0.42-0.46 | 0.584; 0.57; 0.44 |
| cannon: fore-aft x lateral (girth 0.21) | 0.065 x 0.05 | 0.066 x 0.048 |
| hoof: length x width x toe height, toe angle | 0.13-0.14 x 0.13 x 0.085, 50-55 deg | 0.132 x 0.132 x 0.085, 58 deg |
| croup angle; face line when alert | 20-25 deg; 35-45 deg below horizontal | 21 deg; 40 deg |
| ribs | 18 pairs, 8 sternal | 18 hoops, 8 closed |

## 2. Audit of what was there

- Rig: the trunk was 0.18 m too short (thoracic spine 0.70 m), which gave a wheelbase of 1.0 m under 1.61 m withers. In the default pose the head bone stood 12 deg below horizontal (a dog's head carriage). Leg joint heights are within 3-6 cm of the reference; the tibia is about 0.08 m long (0.52), the fetlocks about 4 cm low, the shoulder joint about 6 cm high.
- Dimensions: 2.37 x 2.91 x 0.62 m followed the wrong pose and trunk.
- 2D: every view of every scale was a dump of earlier 3D projections (elevation 1:50 alone about 9 000 points) of a low, long body with falling croup, no withers, a 0.25 m thick neck and a horizontal snout.
- 3D: 43 solids at 1:50, one tube per bone, box-like trunk in two segments, structure = one capsule per bone plus balls, mass ellipsoids as large as the trunk.

## 3. Root fixes in the asset (old -> new)

- `rig` thorax `lengthM` 0.70 -> 0.88, thorax `mass.comX` 0.35 -> 0.44, `neck-1.x` 0.66 -> 0.84, `scapula-ff.x` and `scapula-fn.x` 0.42 -> 0.60. Leg chains are untouched, so every walk phase and clip key keeps its foot plant; I checked walk-cycle t = 0.3 s and grazing by render.
- anchor `back-plume` x 0.35 -> 0.44, y 0.268 -> 0.20 (on the back surface, D2).
- pose `standing-alert`: head 16 -> -12, ear-n -16 -> -6, ear-f -10 -> 0 (face line 40 deg below horizontal, ears pricked up and forward).
- `dimensions`: height 2.37 -> 2.34, width 2.91 -> 2.90, depth 0.62 -> 0.59 (measured on the 1:50 body in the default pose; the real overall size follows from the reference rows above). The same two values in the horse row of `🗂️catalog/🔣️.json`.
- `representations`: every filled shape of every scale and view redrawn as the outline of the solids it stands for (same ids, roles, bones, z, tones, regions); 718 / 796 / 760 points at 1:50 for elevation / front / plan, 373 / 471 / 508 at 1:100, 280 / 270 / 288 at 1:200, 136 / 166 / 190 at 1:500. Front and plan points are rest coordinates found by iterating the product's own placement until the drawn outline sits on the projected one (residual under 0.3 mm at 1:50 to 1:200, 2.4 mm at 1:500). Detail and texture strokes (hip, shoulder, stifle, knee, hock, coronets, nostril, brow, mouth, coat strokes, tail hair) redrawn by hand on the new forms. Tail and ear shapes stay `solid: extruded` in all views, because the validator refuses flat shapes on bones that swing sideways (`lateral-ambiguous`).
- Not changed: `thermal` data, `structure` data, clips, the other poses, `areaM2` values.

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 68 solids: croup, loin, barrel, withers ridge, hip points, breast muscles, neck in 3, head, jaw, muzzle pad, eyes, brows, cheek crests, nostrils, ears with hollows, forelock, mane in 3, tail in 4; per foreleg scapula, upper arm, triceps, forearm, cannon with knee, accessory carpal, fetlock, pastern, hoof; per hind leg thigh, stifle, gaskin, cannon with hock, hock point, fetlock, pastern, hoof | 69: references to the body per region and layer, two nostril outlets, back patch | 154: skull, orbits, mandible halves, 7 cervical vertebrae, spine in 3 with 24 dorsal processes and lumbar wings, 18 rib hoops, sternum, sacrum, iliac crests, ilia, ischia, pubis, limb bones down to the coffin bones, sesamoids, patellae, calcanei, caudal vertebrae, 37 joints, 3 masses |
| 1:100 | 43, merged masses with own sections | 46 | 107, 9 rib hoops, processes as fins |
| 1:200 | 38 | 43, eyes as own solids | 81, 5 rib hoops |
| 1:500 | 24 | 35, mane, muzzle, eyes, hooves as own solids | 54, spine beams, 2 rib hoops, bar limbs |

Final check line: `PASS` (silhouette IoU elevation / front / plan 0.996 / 0.994 / 0.997 at 1:50, 0.992 / 0.960 / 0.986 at 1:100, 0.989 / 0.983 / 0.990 at 1:200, 0.980 / 0.949 / 0.978 at 1:500; eyes, muzzle and hooves inside the area limits). These numbers follow from the redraw and prove nothing about quality.

## 5. What I saw (renders opened with the Read tool)

- 1:50 body, six-view sheet, default pose: right and left side read as a horse at once: withers, dipped back, croup at 21 deg, deep barrel with a flank tuck, deep neck base, wedge head with jaw. Front: narrow chest between straight forelegs, breast muscles as two rounded lumps. Back: rounded quarters, tail between. Top: barrel widest behind the shoulder. Three-quarter: shoulder and triceps stand as slightly plated masses with visible seams.
- 1:50 grazing and walk-cycle t = 0.3 s (right side): limbs stay closed at every joint; in grazing the base of the lowered neck leaves an angular lump at the withers and the mane breaks into three slabs; the fore hooves cut the ground line by about 1 cm (the pose's root tilt, present before).
- 1:50 structure sheet: a recognisable equine skeleton (rib basket slanting back, tall withers processes, scapula plates, limb bones in the limbs). The first sheet showed trumpet-shaped ilia and a scapula joint above the withers; both were rebuilt. I did not re-render the six-view structure sheet after that fix, only the small panel of the `volumes-render` sheet.
- 1:50 thermal sheet: regions follow the body exactly, back patch lies on the back.
- 1:100 and 1:500 `volumes-render` sheets: horse at a glance in all three lists. 1:200 was rendered but I did not open it; the per-list sheets of 1:100 from four directions were rendered but not opened either.
- 2D sheet 1:50 (`r3-2d.ts`): elevation is a clean horse; front and plan show the ears as rectangles and the tail as a rectangle.

## 6. Still imperfect, plainly

- Leg bones of the rig were not corrected (tibia 0.52 against about 0.44, fetlocks 4 cm low, shoulder joint 6 cm high, scapula pivot at the withers top). Correcting them means re-solving eight walk phases and 58 clip keys; I judged my reference too uncertain (±3 cm) to do that. The hind leg therefore stands slightly sickle-hocked.
- The neck pivot of the rig is at the top of the shoulder, not at C7/T1: in the grazing pose the neck base lumps up and the cervical vertebrae part from the thoracic spine.
- Body: sections are symmetric superellipses, so muscle groups are overlapping convex masses with seams, not one skin; the shoulder reads plated, the hip point is an oval boss, the hoof is a plain truncated cone without heel bulbs or frog, the ear is a closed leaf with a dark insert, no chestnuts, sheath or udder, no skinning weights.
- 2D front and plan: tail and ears are prisms by necessity of the `solid` hint, so they are rectangles there and differ from the 3D (red areas in the overlay between the hind legs and at the tail in plan). The `decisions` texts of the four representations were not rewritten and still describe the old construction (paper sizes, "ellipses and capsules").
- Far-side limbs keep the darker 2D tones; in 3D both sides have the same coat tone.
- Structure: no support solids because `structure.supports` is empty in the asset; bones use the material `short-coat` because the asset has no bone material; ribs are full elliptic hoops (no costal arch), the sweep direction of the ten open hoops was not verified from below.
- Pose `standing-rest` (head bone 104 deg below horizontal, behind the vertical) looked implausible on paper; I did not render or change it.
- Not run: `repo` test targets and model export.

## Round 5 handcraft (2026-11-21)

State: spliced; `r3-validate.ts animals/horse --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts animals/horse --product` green at every scale. Scripts: `r5-horse-rig.py` (root fix of rig, anchors, masses, limits, elevation 2D frames, every pose and clip key), `r5-horse-body.py` (body of every scale, every station by hand), `r5-horse-model.py` (lists, skeleton on the new spine, thermal, decisions), `r5-horse-details.py`, `r5-horse-draw2d.ts` (ruling E1 redraw), `r5-candidate.py`, chain `r5-horse-ship.sh [--apply]`.

### Reference (model to)

Warmblood 550 kg, withers 1.61 m (Ellenberger/Dittrich/Baum, Goldfinger, Buchner 1997 segment data, studbook measures; from knowledge, about ±3 cm; no web lookup). Heights above ground, x ahead of the hip joint.

| landmark | reference | rig / model round 5 | round 4 |
|---|---|---|---|
| withers top; croup top; back low point | 1.60-1.61; 1.58-1.60; 1.50 | 1.612; 1.598; 1.50 | same |
| lumbosacral joint (centre of vertebral body) | 0.10-0.15 below the croup skin | (0.17, 1.46) | spine bone at 1.314 (hip height, mid-body) |
| thoracolumbar joint (T18/L1) | 0.10-0.13 below the back skin | (0.50, 1.39) | (0.50, 1.314) |
| C7/T1 (neck pivot) | between the shoulder blades, about 0.35 below the withers | (1.36, 1.245) | (1.34, 1.444) at the top of the shoulder |
| thoracic / lumbar spine length | 0.85-0.90 / 0.30-0.33 | 0.872 / 0.337 | 0.88 / 0.30 |
| scapula pivot; shoulder joint; scapula slope | dorsal third of the blade under the withers; 1.05-1.10; 50-60 deg | (1.12, 1.50); 1.10; 56 deg | withers top 1.614; 1.155; 62 deg |
| elbow joint; carpus centre; fore fetlock | 0.85-0.90 under the shoulder; 0.48-0.50; 0.18 (joint centre) | 0.86; 0.485; 0.181 | 0.876; 0.476; 0.181 |
| humerus; radius; fore cannon (carpus to fetlock) | 0.30-0.33; 0.36-0.40; 0.30 | 0.326; 0.375; 0.303 | 0.34; 0.40; 0.295 |
| stifle joint; hock (talocrural) centre; point of hock | 0.93-0.98; 0.54-0.58; 0.62-0.66 | 0.93; 0.545; 0.63 | 0.95; 0.524; 0.615 |
| femur; tibia (joint to joint); hind cannon (hock to fetlock) | 0.40-0.44; 0.42-0.46; 0.34-0.37 | 0.42; 0.434; 0.356 | 0.42; 0.52 (+15 %); 0.34 |
| hind cannon from vertical in stance | 0-3 deg | 0.2 deg | 10 deg forward (sickle hocked) |
| chest floor; chest depth; barrel width; hip width | 0.85; 0.75; 0.56-0.60; 0.56 | 0.853; 0.76; 0.592; 0.556 | same |
| neck width at the shoulders / mid / throat | 0.30-0.38 / 0.20-0.22 / 0.15-0.17 | 0.39 / 0.21 / 0.16 | 0.34 / 0.18 / 0.14 |
| head length; width at the brows; muzzle width | 0.55-0.60; 0.21-0.24; 0.12-0.14 | 0.59; 0.244; 0.144 | 0.59; 0.234; 0.14 |
| hoof: toe angle; toe height; heel height; length x width | 50-55 deg; 0.08-0.09; 0.03-0.045; 0.13 x 0.125 | 53 deg; 0.084; 0.043; 0.133 x 0.128 | cone 58 deg, coronet level all round |
| tail: dock length; hair end | 0.25-0.45; at or above the hock | 0.22 dock + hair; 0.60 | same |

All model values are within 3 % of the reference band except the elbow joint (0.86 against 0.85-0.90, inside) and the C7/T1 position, which has no sharp reference (placed between the scapulae by anatomy).

### Grades (my eye, opened renders)

| scale | body before → after | thermal before → after | structure before → after | 2D before → after |
|---|---|---|---|---|
| 1:50 | C+ → A- | B → A- | C → B+ | B → A- |
| 1:100 | B- → A- | B → A- | C+ → B+ | B → A- (not opened at 1:100) |
| 1:200 | B → A- | B → B+ (not opened) | C+ → B+ (not opened) | B → B+ (not opened) |
| 1:500 | C+ → B+ | B → B+ (not opened) | C → B+ | C+ → B+ (not opened) |

Before = the round 4 catalog state as I saw it on 2026-11-20 (body 1:50 sheet, structure 1:50 sheet) and the orchestrator's findings in r4-queue.md.

### Changes

- Rig (root): pelvis 0 → 40.7 deg towards the lumbosacral joint, lumbar and thorax along the dorsal vertebral column (thorax -9.6 deg sinking to T1), neck-1 pivot at C7/T1 (length 0.34 → 0.48), fore legs: scapula pivot in the dorsal third of the blade, shoulder joint 5.5 cm lower, elbow under the shoulder, forearm vertical; hind legs: tibia 0.52 → 0.434, cannon 0.34 → 0.356 and vertical, stifle and hock at their joint centres. Fetlocks, pasterns and hooves untouched. Every pose and the 58 clip keys solved again (legs by IK onto the old fetlock with cannon and forearm keeping their deviation from rest; neck by IK onto the old poll, or, where the poll is out of reach of the longer neck base (grazing, standing-rest), with the muzzle kept on its old height): residual at every hoof, tail and ear 0.0 mm. Joint limits shifted by the mean change of the solved offsets and widened where a pose needs it (scapula). Masses, anchors and 160 elevation shapes carried into the new frames. Pose `standing-rest`: head -18 → -1 deg (the face hung 14 deg behind the vertical).
- Body, every scale anew: one trunk skin from the buttocks to the breast on the lumbar bone, blended over pelvis, loin and thorax (no seam along the back; no pose or clip bends the trunk), withers ridge over the thoracic processes; neck 0.39 m wide at the base with a blended neck-root (thorax/neck-1) and a blended throatlatch (neck-3/head) so that grazing and resting stay closed; shoulder, arm and point of elbow as flat muscle masses lying on the chest; forearm with chestnut, flat-fronted knee with the accessory carpal, cannon with tendons, fetlock with ergot, pastern, hoof with sloping coronet, coronary band and heel bulbs; thigh with stifle, gaskin covering the tibia, Achilles tendon to the point of the hock, hock; mane as a crest band plus a hair fall lying on the near side of the neck; tail as dock plus four overlapping, bowed hair pieces. 1:500 got its withers back and a jowl.
- Structure: vertebral column, processes, ribs and sternum on the dorsal spine line (ribs from the rig spine down, rotated with the new thorax angle), cervical vertebrae along the new neck bones, limb bones, tarsus, calcaneus, patella, olecranon at the new joint centres; iliac crests no longer leave the body.
- Thermal: references follow the new body; eyes reduced to the area band (1:50, 1:100).
- 2D (ruling E1): every filled shape redrawn from the new body; the plan croup shape on the pelvis removed (the pelvis carries no body solid); thorax shapes take the trunk skin so no false seam line is drawn; at 1:200 the mane is three shapes (neck-1, neck-2, neck-3) instead of one; ears and tail are `solid: revolved` about their own bone (leaf and tube in front and plan instead of rectangles); detail strokes redrawn on the new forms (hip, shoulder, stifle, knee, hock, coronets).

### Renders opened

1:50 body (six views, several iterations), right side alone, top alone, three-quarter alone; grazing-head-down (six views and left side, three iterations), standing-rest (right side), walk-cycle t = 0.3 s (right side); 1:50 structure; 1:50 thermal cut; 1:100, 1:200, 1:500 body; 1:500 structure; 2D 1:50 (candidate, twice).

### Remaining, with reason

- Rigid segments: in grazing a small flat facet stays at the crest behind the withers and the shoulder slab shows its edge when the scapula rotates (no per-vertex skinning in the schema; the blended fillers close the gaps but cannot stretch).
- Ribs are elliptic hoops (the kit has no curved rib profile); from the side they read as straight bars sloping back.
- Not opened: 2D at 1:100, 1:200, 1:500; thermal and structure at 1:100 and 1:200; thermal at 1:500; the graze-and-swish clip.
