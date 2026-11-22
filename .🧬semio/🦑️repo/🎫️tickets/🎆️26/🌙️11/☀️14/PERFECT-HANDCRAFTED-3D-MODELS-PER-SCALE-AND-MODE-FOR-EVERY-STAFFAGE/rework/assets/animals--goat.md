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
