# animals/goat — author report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published; `r3-validate.ts animals/goat --catalog-only --volumes` ends `VALID animals/goat (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (the catalog asset was `INVALID` with 76 + 82 findings when I started). The Read tool of this session stopped answering after the first publish (hook timeout on every image), so **nothing rendered after the trunk and hind-leg re-cut of 1:50 was looked at**: structure, thermal, 1:100, 1:200, 1:500, both poses, both clips, the 2D sheets and everything after the girdle change were rendered (22 sheets, no `NOT DRAWN`) but not read. The next pass must look first.

### Reference (dairy doe, Saanen type, 0.76 m at the withers, 55 kg; from memory, ±10 %)

| measure | reference | model (rig / solids) | verdict |
|---|---|---|---|
| withers height | 0.76 m | scapula top 0.745 m, withers skin 0.761 m | ok |
| trunk length, point of shoulder to pin bone | 0.80–0.85 m | 0.84 m (x −0.10 … +0.74) | ok |
| muzzle to pin bones | 1.25–1.30 m | 1.284 m; `dimensions.widthM` was 1.31, now 1.28 | corrected |
| chest depth at the heart girth | 0.34–0.36 m | 0.358 m | ok |
| chest breadth at the heart girth / between the shoulders | 0.25 / 0.20–0.22 m | 0.248 / 0.20 m | ok |
| greatest breadth of the barrel (full rumen), low behind the last rib | 0.30–0.34 m | was a tube of 0.34 m at mid height; now 0.30 m at mid height and 0.316 m low (belly lobe) | re-cut |
| breadth over the hooks / between hip joints | 0.17–0.19 / 0.13–0.14 m | trunk 0.25 m at the hooks, hip joints ±0.070 m | hooks not modelled as points (open) |
| scapula | 0.165–0.18 m | 0.1786 m (dorsal border ±0.040 m to shoulder joint ±0.075 m) | ok, now lateral |
| humerus | 0.16–0.17 m | 0.174 m | ok |
| radius (elbow to carpus) | 0.19–0.21 m | 0.212 m | upper end of range |
| metacarpus (carpus to fetlock) | 0.13–0.14 m | 0.147 m | 5 % long, kept |
| digit, fetlock to ground along the axis | 0.085–0.095 m | 0.100 m | kept |
| hip bone, sacroiliac joint to hip joint | 0.11–0.12 m | 0.119 m (new bones `ilium-n/-f`) | new |
| femur | 0.19–0.21 m | 0.214 m | upper end |
| tibia | 0.23–0.24 m | 0.236 m | ok |
| metatarsus (hock to fetlock) | 0.16–0.17 m | 0.172 m | ok |
| elbow / carpus / stifle / hock height | 0.43 / 0.22 / 0.46 / 0.26 m | 0.438 / 0.226 / 0.462 / 0.250 m | ok |
| cannon girth | 85–90 mm (27–29 mm thick) | 28–30 mm | ok (goat legs are this slim; the "thin rods" verdict of pass 07 was wrong for the cannons) |
| gaskin, fore-aft at mid length | 0.09–0.10 m | 0.106 m | ok after re-cut |

Joint order was already right (scapula–humerus–radius/ulna–metacarpus–digit; femur–tibia–metatarsus–digit), both sides alike.

### Found

- Catalog and source were identical, but the catalog asset was `INVALID`: `volumes-feature` (per-scale rule) 64 findings at 1:500, 11 at 1:200, 1 at 1:100; `volumes-connected` at every scale (scapula joints, sternum, hip bones, tail and udder not on the load path).
- 1:50 body: trunk a straight tube in three pieces, no withers, rump a box with the hams as lobes hidden inside a barrel wider than the thighs, the hind leg broke between ham and gaskin (a 0.1 m step behind the stifle), `decisions` of 1:200 and 1:500 described solids that did not exist ("eight distinct claws", "udder left out").
- Rig: shoulder breadth faked with a `z` step at the humerus under a sagittal scapula; thighs hung directly from the pelvis root.

### Rebuilt (scripts in the ticket folder, run in this order)

1. `rework-goat-trunk.py` — chest, loin, croup and a new solid `belly` (+ `th-belly`) from world station tables (top line, under line, half breadth), every scale at its own stations: withers 15 mm above the back, level loin, croup falling to the pin bones, trunk narrower than the thighs between them, shared stations at the seams so the three pieces meet without a step.
2. `rework-goat-hindleg.py` — ham and gaskin at horizontal stations, both sides, every scale: rear line unbroken from pin bone to the point of the hock, stifle in front of the flank; floors of 32 mm (1:200) and 76 mm (1:500).
3. `rework-goat-coarse.py` — 1:200: one cloven hoof per foot, no teats; 1:500: one foot per leg, one udder, horns, ears, arm, forearm, shoulder thickened to their drawn stroke, structure as a bar skeleton (pelvis one block, skull stands for the jaw, tail piece for both caudal bones); 1:100 lamina to the feature.
4. `rework-goat-loadpath.py` — structure at every scale: sacral wings, pubic rami, symphyseal tendon that carries the udder's suspensory lamina, scapular cartilage up to the joint, the sling to the withers, sternum taking the costal cartilages, patella on the femur, first caudal on the sacrum.
5. `rework-goat-texts.py` — `decisions` of all four scales rewritten (en, de) to what is modelled.
6. `rework-goat-girdle.py` — after `rig-lateral-rest.md` appeared: scapulae with `abductionDeg ∓11.30°` (tip at the shoulder joint, humerus at `x = lengthM`, no `z` step), new bones `ilium-n/-f` from the sacroiliac joint (±0.030 m) to the hip joint (±0.070 m) with the femurs re-parented without moving, scapula solids re-expressed in the new frame at every scale and list, `hip-bone-n/-f` bound to the ilium bones at every scale (new bars at 1:100, 1:200, 1:500). Limb rest frames unchanged to 3e-6 m (`rig-rest.ts`). Fore-limb elevation shapes of 1:50 and 1:100 (shoulder, arm, forearm, cannon) got an extruded `solid` span because the limb now leaves the paper plane by up to 6° when the scapula swings.
7. 2D (`rework-07-2d.py` as ruler-carrier): elevation, front and plan of croup, hindbody, chest, thighs and gaskins at every scale; feet at 1:200 and 1:500; 1:500 front and plan of the legs and head. Ear shapes of 1:500 front and plan were put back from git `HEAD` after the ruler had cut the tips.
8. `dimensions.widthM` 1.31 → 1.28.

`rework-goat-compare.ts` (anchors by id; `rig-lateral-compare.ts` pairs placed shapes by index and reports order changes as moves of 30–190 mm): pelvis-only change moves no anchor in any pose; with the scapulae the hoof contacts move at most 8.9 mm in the walk phases, 21 mm in `climbing-ledge`, 49 mm in `browsing-upright` (raised fore hooves).

### Remaining (not done or not verified)

- Not looked at after my changes: every sheet except the 1:50 body in the default pose before the girdle change.
- 1:50 body: fore limb (shoulder plate with a facet at the withers, arm, forearm, carpus, cannon, pastern, claws), neck, head, horns, ears, tail and udder are the earlier author's solids, only judged on the first sheet (they read right, no close-ups made); the belly lobe shows a faint crease where it leaves the barrel; hooks (tuber coxae) and the spine ridge are not modelled; the rump corner at the tail is still angular.
- 1:100, 1:200: fore limb, head and neck lists are the earlier solids at fewer sections, not re-modelled by me.
- 1:50 structure: cervical column is a nearly straight tube high in the neck (real: S-curve, C7 low at the thoracic inlet), no atlas/axis, thoracic spinous processes as one plate, ribs are capsule chains; not rebuilt.
- 2D: the replaced shapes are ruler outlines, not economical hand drawings; at 1:200 and 1:500 the croup beyond the thigh (12 mm) is not drawn; the swept fore-limb shapes were not viewed in a pose.
- Rig: no separate pastern bone (fetlock and coffin joint are one `hoof` bone), neck is one bone; the humerus offsets in poses were not re-solved, so the fore hooves drift laterally by the amounts above.
- `mass` of the ilium bones: none (the pelvis keeps it).

### Fundamental issues (outside this asset)

- `rig-lateral-compare.ts` compares placed shapes by index; with a lateral bone the depth order of shapes changes and it reports 30–190 mm "moves" for a rig whose anchors do not move at all. It should pair shapes by a stable key (placed shapes carry no `id`).
- `volumes-feature` takes its limit from "the thinnest drawing of the bone": after carrying the 3D outline into the drawing with `r3-project2d.ts` the cleaned outline is 1–5 mm thicker than the solid it came from, so the rule then fails the very solid the drawing was taken from (1:500 feet 45.0 < 46.9 mm, shoulder 64.0 < 64.8 mm). I thickened the solids; the ruler and the rule should agree.
- A pose offset with `abductionDeg`/`twistDeg` on a bone without `twistMinDeg`/`twistMaxDeg` produced bones 40–60° out of plane when I tried to re-solve the humerus offsets (candidate only, not applied); either my decomposition or the clamping of undeclared limits — not investigated.
- `validator.md` has no section "Rule changes authors must know".
- The Read tool (PreToolUse hook) timed out on every image from 02:25 on.

## Pass 2 (second round), 2026-11-22

Status: **partial**. Published; `r3-validate.ts animals/goat --catalog-only --volumes` ends `VALID animals/goat (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. Girdles are on the lateral rest placement (pass 1, checked again with `rig-rest.ts`: scapula origin ±0.040 m, shoulder joint ±0.075 m, sacroiliac ±0.030 m, hip joint ±0.070 m).

**How this pass could look.** The Read tool worked for the first seven images (1:50 body, structure, thermal in the default pose; `browsing-upright` and `climbing-ledge`; two close-ups) and then timed out on its PreToolUse hook for the rest of the session, as did the browser pane. Everything rendered after my changes was read only as a character raster (`rework-goat-ascii.py`, `sips` to BMP, 100–200 columns: silhouette, black outline, seams, shading in three steps). That shows outline, proportions and where a solid stands proud; it does not show surface quality. No picture after the changes was seen at full resolution.

### Seen in the seven images (before changes)

- 1:50 body: the shoulder was a narrow sausage with a knob at the point of the shoulder and a hook at the withers, standing proud of the trunk; the arm a second sausage; the breast stood 40 mm in front of the point of the shoulder with a faceted under line (pigeon breast); the top of the ham was a three-facet polygon that made the rump read as a box from behind; the belly lobe shows as a seam panel (seams of two solids are always drawn grey, the same as chest/loin); the poll has a sharp corner above the neck.
- 1:50 structure: cervical column a nearly straight tube along the crest; thoracic spines one plate; ribs sloped bars with a long costal arch; pelvis, sacral wings, pubic rami and hip bones sit where they belong and meet; the udder masses hang under the pelvis.
- 1:50 thermal: reads as the same object, regions follow the body solids.
- Poses: both read naturally; fore limbs hang from the shoulder joints at the true breadth, no crack at any joint in the two sheets.

### Rebuilt in this pass

1. `rework-goat-shoulder.py` (new) — shoulder and arm at every scale from horizontal world stations, written into the abducted scapula frame and the humerus frame with the full 3D rest basis: the shoulder is the flat muscle sheet over the blade (0.065 m wide at the withers, 0.12 m at mid blade, outer face 0.106 m beside the plane at the point of the shoulder, i.e. 0.21 m between the points, reference 0.20–0.22 m), the arm the triceps wedge from the rear border of the blade to the olecranon (0.14 m fore-aft at mid arm, 0.064 m thick). 1:500 structural scapula bars thickened from 65.6 to 76 mm because the new, wider drawing of the blade lifted the feature limit of the bone to the true 75 mm.
2. `rework-goat-trunk.py` (table changed) — chest front at seven stations instead of four, ending 0.782 m ahead of the hip joint (was 0.798), 25 mm in front of the point of the shoulder; under line of the brisket rounded.
3. `rework-goat-hindleg.py` (table changed) — top of the ham at five stations instead of three (dome instead of facets), picks of the coarser scales re-mapped.
4. `rework-goat-cervical.py` (new) — cervical column at every scale on its S-curve (up to 50 mm below the neck axis in the lower half, rising to the occiput); at 1:50 seven vertebrae with articular ends and waisted bodies, atlas wings 62 mm wide, axis crest 54 mm high; coarser scales keep the curve with one tube.
5. `rework-goat-spines.py` (new) — 1:50: the thoracic plate became a column of bodies (28 × 26 mm) with thirteen spinous processes (T2–T5 64–68 mm above the bodies, swept 28–31 mm towards the loin, shortening to the upright T11–T13) tied by the supraspinous ligament; all stay at least 10 mm under the back line. 1:100 and coarser keep the plate (a process of 8 mm is below their feature).
6. 2D: `rework-07-2d.py goat --replace thorax,thigh-hn,thigh-hf,scapula-fn,scapula-ff,humerus-fn,humerus-ff` at every scale and view (ruler outlines), and `rework-goat-limb2d.py` (new) for the four elevation profiles with an extruded span at 1:50 and 1:100 (shoulder span 0.035–0.107 m, arm 0.044–0.108 m beside the plane; it reads the ruler output `r3-project2d.ts animals/goat --scale N --view elevation > 🗑️generated/rework-goat/ruler-N.txt`, which is deleted with the generated folder).
7. `rework-goat-texts.py` — `decisions` of 1:50 (legs, structure) say the new shoulder, breast, cervical column and spines, en and de.

### Rig: humerus pose offsets (tried, measured, not applied)

`rework-goat-resolve.ts` solves per pose the humerus `angleDeg`/`abductionDeg`/`twistDeg` that put elbow, carpus, fetlock and hoof tip back on the limb as authored (blade swinging in the paper plane). Measured drift before → after: walk phases 0.03–8.9 mm → ≤ 0.34 mm, `climbing-ledge` 13.8 / 24.8 mm → 1.7 / 1.3 mm, `browsing-upright` 40.5 / 49.4 mm → 2.4 / 3.4 mm (the rest is the shoulder joint itself moving with the blade on the rib wall). With these offsets `validateAsset` reports 8 × `lateral-ambiguous` (flat front and plan shapes of humerus, forearm, metacarpus and hoof at all scales, up to 5.4 mm on paper), whose remedy is to author 30-odd front/plan shapes as solids. I could not look at 2D sheets, so I restored the poses and kept the drift: the fore hooves on the ground move at most 8.9 mm sideways over the walk cycle (0.18 mm at 1:50), the 49 mm concern hooves in the air. The script is kept for the pass that redraws those shapes.

### Reference table additions

| measure | reference | model | verdict |
|---|---|---|---|
| breadth between the points of the shoulder | 0.20–0.22 m | 0.212 m | ok (was 0.21 m as a knob) |
| prosternum ahead of the point of the shoulder | 0.01–0.03 m | 0.025 m | corrected (was 0.042 m) |
| cervical vertebrae C3–C7 / axis / atlas | 45–55 / 65–70 / 35–40 mm | 46–56 / 70 / 40 mm (scaled to the 0.39 m path) | new |
| atlas wings, breadth | 60–65 mm | 62 mm | new |
| thoracic spinous processes at the withers | 60–75 mm | 64–68 mm | new |

### Remaining (precise)

- All scales, all lists: nothing rendered after the changes of this pass was seen as a picture (character rasters only: 1:50 body elevation, plan, front, three-quarter shoulder crop, structure neck and thorax crops, both poses in elevation; 1:100 and 1:500 body elevation; 1:500 structure elevation). Not read at all after the changes: thermal sheets, 1:200, the 2D sheets, the clip instants, cut sheets (26 sheets were rendered without `NOT DRAWN` or error and then deleted).
- 1:50 body: hooks (tuber coxae) and the spine ridge not modelled; pin bones are one rounded end, not two points with the hollow under the tail; the poll corner above the neck; neck, head, jaw, horns, ears, tail, udder, forearm, carpus, cannons, pasterns and claws are still the earlier author's solids (judged in the first sheet as right in proportion, no close-up); the belly lobe remains a second solid with a visible seam.
- 1:100, 1:200: forearm, head and neck lists still the earlier solids at fewer sections. 1:500: body and structure thickened to the feature by floors (shoulder and arm now from the station tables with a 38 mm floor).
- 1:50 structure: ribs are still capsule chains; lumbar column a plain tube with one loft for the transverse processes; no individual lumbar vertebrae. 1:100 keeps the thoracic plate.
- 2D: every replaced shape is a ruler outline, not an economical hand drawing; the scapula elevation profiles at 1:50 and 1:100 are the flattened projection reused as the profile of the extruded span (the ruler calls this "reference only" because the blade rests 11.3° out of the paper; up to about 4 mm real compression); no 2D sheet was viewed.
- Rig: humerus pose offsets not re-solved (see above); no separate pastern bone; the neck is one bone; metacarpus 5 % long and digit 0.100 m kept.
- Thermal: `areaM2` not checked by hand; no cut sheet looked at.

### Fundamental issues (outside this asset)

- Host: the Read tool and the browser pane time out on their PreToolUse hook ("host client may be unreachable") after a few images; `python3` has no PIL. A character raster is the only way left to look; `r3-sheet.ts` could offer a `--text` raster itself.
- `lateral-ambiguous` and solved pose offsets: giving a child bone the `abductionDeg`/`twistDeg` that brings a limb back into the paper plane after its parent swings on an inclined rest axis is reported for every flat front/plan shape down the chain, although the limb is then closer to the paper plane than without the offset (the unsolved pose with 49 mm drift validates). The rule seems to measure the offset of the bone, not the resulting frame against its rest frame.
- `volumes-feature` takes its limit from the thinnest drawing of the bone: redrawing a 2D shape changes which 3D solids of another list (here the 1:500 structural scapula) are valid.
- `rework-07-2d.py` skips shapes that carry a `solid`; there is no tool path that redraws the profile of an extruded span on a bone resting out of the paper.
