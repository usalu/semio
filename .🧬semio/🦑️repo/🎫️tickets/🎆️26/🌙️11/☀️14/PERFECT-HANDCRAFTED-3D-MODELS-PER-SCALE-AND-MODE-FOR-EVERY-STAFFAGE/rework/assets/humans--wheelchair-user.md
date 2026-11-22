# `humans/wheelchair-user` — running report

Earlier rounds: `T/r3-humans-wheelchair-user.md` (round 3 rebuild and round 5 touch-up, 20/21 Nov). Batch report `T/rework/04.md` lists this asset as "not started".

## Pass 1 (second round), 2026-11-22

### State found

- Catalog file and `T/volumes/humans/wheelchair-user.json` were both of 21 Nov 21:50 and agreed (no half-written edit).
- An agent of this round had been cut off at 13:59 after writing `rework-wheelchair-user-rig.py` (girdles on the lateral rest placement, trunk split into `lumbar` and `torso`) and `rework-wheelchair-user-coarse.py` (hands, wheels, axle carriers and long bones of 1:100 / 1:200 / 1:500 restated per feature size). Neither was applied; its candidate had 0 `validateAsset` findings and 9 `volumes-connected` findings. I read both scripts, rendered the candidate, judged it better than the catalog (the catalog itself had 138 `validateAsset` findings under today's rules, all coarse-scale `volumes-feature`) and continued from it.

### Reference table (man, 1.75 m, 75 kg, seated in an active rigid wheelchair)

Sources from my own knowledge, no lookup: Winter 2009 / Drillis-Contini segment fractions, de Leva 1996, ANSUR II male means, ISO 7176-7 chair measures, ETRTO 37-540.

| quantity | reference | model (rig / solids) | deviation |
|---|---|---|---|
| upper arm / forearm / hand | 0.326 / 0.256 / 0.189 m | 0.3234 / 0.2548 / 0.1862 | -0.8 / -0.5 / -1.5 % |
| thigh / shank / ankle height | 0.429 / 0.430 / 0.068 m | 0.4263 / 0.430 / 0.085 in a sneaker with 19 mm sole | -0.6 / 0 % |
| foot length (shoe) | 0.266 (0.29) m | ankle to ball 0.1485 + toe 0.066, shoe 0.292 | ok |
| hip joint to shoulder joint | 0.504 m (0.818 H - 0.53 H) | 0.534 | **+6 %** (kept, see below) |
| hip joint to C7/T1 | 0.578 m | 0.599 | +3.6 % |
| C7 to vertex | 0.245 m | 0.245 | 0 |
| sitting height above the seat | 0.52 H = 0.91 m | 0.90 to 0.91 | ok |
| head: chin-vertex / breadth | 0.233 / 0.155 m | 0.233 / 0.154 | ok |
| shoulder joint centres apart | biacromial 0.40 minus 2 x 2.5 cm = 0.35 m | 0.352 (z +-0.176) | ok |
| clavicle (SC to AC joint) | 0.15 to 0.16 m, running back 15 to 25 degrees, up 5 to 10 | 0.1605 m, 22 degrees back, 3 degrees up; SC joints 44 mm apart | ok |
| hip joint centres apart | 0.17 to 0.18 m | 0.175 | ok |
| hip bone (SI joint to hip centre) | about 0.10 m | 0.1026; SI joints 80 mm apart | ok |
| sacrum base width | 0.10 to 0.11 m | 0.104 (was 0.052: SI joints lay outside the bone) | fixed |
| lumbar spine (L5/S1 to T12/L1) | 0.16 to 0.18 m | 0.17 | ok |
| thoracic spine (T12/L1 to C7/T1) | 0.28 to 0.30 m | 0.325 | +8 % (carries the long trunk above) |
| scapula | triangular blade about 0.15 high, 0.10 wide, glenoid at the lateral angle | blade 0.14 x 0.104 with spine, acromion and glenoid neck (was an oval disc 0.15 x 0.10 whose rim missed the shoulder joint by 8 cm) | fixed |
| rear wheel / hand rim | 24 in, outer r 0.305, hand rim dia 0.53 outboard | same | 0 |
| castor / wheelbase | 4 to 6 in / 0.40 to 0.45 | 0.135 / 0.44 | ok |
| seat width / depth / rear height | 0.40-0.44 / 0.40-0.46 / 0.44-0.48 | 0.42 / 0.43 / 0.45 | ok |

Segment lengths are equal left and right; left-arm pose and clip offsets are now exact mirrors of the right arm.

### Defects found

1. Rig: no girdle bones (arms hung from the trunk bone 0.176 m beside it, thighs from the pelvis bone), one trunk bone from L5 to C7, left arm offsets not the mirror of the right in poses and clips.
2. Structure: the sitter's skeleton had no load path to the chair at any scale (cushion and slings were not structural members; ischia hung in the air as far as the structure was concerned); castor forks, axle plates and axle receivers were missing as members at the coarse scales although they carry the wheels; sacrum half as wide as a real one, so the sacroiliac joints lay outside it; ilium blade not joined to the sacroiliac joint; scapula an oval disc joined to nothing at 1:100; acromion ended 18 mm short of the clavicle; metacarpals 1 and 4 and the thumb ray started outside the carpal block; finger bones started 8 mm beyond the metacarpal heads; 1:200 sternum a 6 mm rod that did not reach the sternoclavicular joints; 1:200 and 1:500 tibia ended above the tarsal block with nothing between.
3. Chair body: rear cross tube 1.7 mm short of the axle plates at 1:50; footplate floated 6.7 mm (1:50) and 4.1 mm (1:100) above the footrest loop; at 1:500 the rear wheels with their axle receivers were not attached to the frame at all (33 mm gap); at 1:100 and 1:200 the cushion lay on nothing (no sling).
4. Coarse scales (found by the earlier agent, in its script): hands, hand-rim standoffs, hubs, finger bones and long bones below the feature size of their scale.

### What was rebuilt

Scripts (hand-stated numbers): `rework-wheelchair-user-rig.py`, `rework-wheelchair-user-coarse.py` (earlier agent, read and adopted), `rework-wheelchair-user-pass1.py`, `rework-wheelchair-user-pass1b.py` (mine); rulers `rework-wheelchair-user-dump.py`, `rework-wheelchair-user-connected.ts`. Build: `python3 rework-wheelchair-user-rig.py` then `python3 rework-wheelchair-user-pass1.py` on a catalog that is not converted yet; the published catalog is converted, so a later pass edits the catalog/source directly or through a new script.

- **Rig** (27 bones, was 22): `lumbar` (0.17 m) carries `torso` (thorax, 0.325 m); `clavicle-l/-r` from the sternoclavicular to the acromioclavicular joint on the full 3D rest placement carry the arms; `ilium-l/-r` from the sacroiliac joint to the hip joint carry the thighs. Arms and thighs keep their world rest frames (checked to 2e-5 m in the script), so their solids, 2D shapes, anchors and masses are untouched. Trunk offsets of all 4 poses and 195 clip keys moved to `lumbar`; left arm = mirror of right. Trunk solids of every list and scale, the torso elevation shapes of all four scales and the torso anchors re-expressed in the thorax frame; shoulder-girdle and hip-bone solids re-expressed in the clavicle and ilium frames.
- **Structure**: cushion, seat sling, back sling (1:500 seat and back block) are members; forks, axle plates, receivers and struts are members wherever the body has them; sacrum with alae and coccyx; body of the ilium (arcuate line) from the sacroiliac joint to the hip joint at 1:50, 1:100, 1:200; scapula as a triangular blade with glenoid neck (1:50, 1:100) and scapular spine (1:50); acromion at 1:100 and closed onto the clavicle; hand rays closed onto the carpus; 1:200 sternum as typified manubrium and body; talus dome on the tarsal block at 1:200 and 1:500; `joint-clavicle-*`, `joint-ilium-*` at every scale.
- **Body (chair)**: rear cross tube to the axle plates; footplate as a 15.5 mm moulded plate on the loop (1:50), 16 mm (1:100); seat sling as a declared sheet at 1:100 and 1:200 (with its thermal reference); 1:500 back post runs down to the axle receiver (post and axle strut as one tube).
- **Thermal**: unchanged apart from the trunk re-binding and `th-seat-sling` at 1:100 / 1:200.
- **2D**: only the re-binding of the torso elevation shapes; the drawings were checked, not redrawn.

### Rendered and read after the last change

Candidate before the last chisel (scapula): 1:50 body `seated-level` and `hands-on-rims` (six views), 1:50 structure (six views), 1:50 thermal cut (six views), 1:50 `push-cycle` t = 0.5 s. After the last change: 1:50 structure back view and a three-quarter view, 1:100 structure, 1:500 body, 1:500 structure, 2D 1:50 `hands-on-rims` (line and solid, three views).

What I saw: the man sits naturally in every image read; hands lie on the thighs in the default pose and close over the hand rims outboard of the tyres in `hands-on-rims`; in the push recovery the hands are above and ahead of the rims, trunk leaning forward. The skeleton reads as a skeleton from all sides: skull with mandible, curved spine with processes, twelve rib pairs to the sternum, clavicles from sternum to acromion, triangular scapulae on the back of the rib cage, pelvis with sacrum, long bones with their ends, finger and foot bones. Thermal shows nested hoodie / jeans / shoe layers, face and beanie, tyres and frame apart. 1:500 still reads as a wheelchair user (rings for the wheels, blocks for seat and back, angular beanie).

Rendered and **not read**: `ramp-ascent`, `push-cycle` t = 2.2 s, `rest-in-chair` t = 2.5 s (1:50 body); 1:200 structure; 2D 1:500. Not rendered at all in this pass: 1:100 and 1:200 body, thermal of 1:100 / 1:200 / 1:500, close-ups of hands, face and feet.

### Validation

- Candidate: `r3-validate.ts --file …/wu-p1/🔣️.json --volumes` → `VALID file/wu-p1 (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Published with `rework-wheelchair-user-pass1.py --publish` (members `rig`, `representations`, `poses`, `clips` replaced in the catalog text, volumes source written, `volumes-insert.py`).
- `r3-validate.ts humans/wheelchair-user --catalog-only --volumes` → `VALID humans/wheelchair-user (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Also read after the first publish: 1:50 body `push-end` and `ramp-roll` t = 1.1 s (six views each): hands at the front of the rims at the end of the stroke; on the ramp the chair is pitched, the trunk leans into the slope and the hands hold the rims. Both read naturally.

### Still imperfect (open list for the next pass)

1. Rig, hands: one bone per hand, no finger bones. Both hands are alike, but the same half-curled hand lies on the thigh and grips the rim; it does not open or close.
2. Rig, trunk: hip joint to shoulder joint 0.534 m against 0.504 m (+6 %), thoracic segment 0.325 m against about 0.30 m. Kept because the four poses and 192 push keys were solved for these shoulder positions (grip misses under 4 mm); shortening needs the pose solver of `r3-wheelchair-user-roots.ts` re-run on the new rig.
3. Rig, chair: both rear wheels on one `rear-wheel` bone and both castors on one `castor` bone (no camber, no castor swivel); no clavicle or ilium offsets are used by any pose (the shrug of a push stroke is not modelled).
4. 1:50 structure: ilium blade is still a flat oval (no iliac crest, no acetabulum cup); ribs have no costal cartilage distinction; scapula has no coracoid.
5. 1:200 / 1:500 structure: shoulder and pelvis are bars; 1:200 sternum is 30 mm thick (feature floor); 1:500 spine and sacrum are 75 mm lofts.
6. 1:500 body: the back post now reaches the axle receiver, but the 1:500 elevation drawing was not redrawn for the extra 75 mm of tube (the silhouette rule passes).
7. `decisions` texts of volumes and representations were not updated for the girdle bones, the structural seat and the 1:500 post.
8. Body muscle form (thigh, calf, forearm sections), face, hands and feet were not re-judged in close-ups in this pass; the round-5 grades (1:50 A-, 1:100 A-, 1:200 B+, 1:500 B) are the earlier author's, not confirmed by me.
9. Thermal lists of 1:100, 1:200, 1:500 not looked at in this pass.
10. Tests and fixtures that embed this asset (`model-skeleton`, `three-views`, projections) were not run.

### Fundamental issues

- `r3-sheet.ts --file` / `r3-2d.ts --file` name their output after the parent folder of the candidate (`file/cand...`), so two authors with a folder `cand` overwrite each other's images. I renamed my folder to `wu-p1`.
- A capsule whose end lies about 4 mm inside a four-section loft (`spinous-4`, `spinous-14` on the spine lofts) was reported 7.9 mm and 1.7 mm away from it by `volumes-connected`; pushing the end deeper closed it. Either the loft surface between sections is not where linear interpolation of the sections puts it, or the triangle distance misses a shallow intersection. Not investigated.
- `rework-10-connected.ts` cannot read a candidate file (`load` has no `--file`); `rework-wheelchair-user-connected.ts` does it for this asset. A shared `r3-connected.ts --file` that prints each loose group with its nearest solid would save every author several rounds (the finding lists only the first four).
