# animals/red-squirrel — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Rig girdles, the 1:50 skeleton and the coarse lists were rebuilt and published; the body and thermal solids of 1:50 and the 2D drawings were not re-modelled, and no image could be read with the image viewer in this pass (see "How it was looked at").

Validation after the last change: `r3-validate.ts animals/red-squirrel --catalog-only --volumes` → `VALID animals/red-squirrel (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. Before this pass the same command gave one finding (`volumes-connected` 1:50 structure: both pelvis halves 4.8 mm off the column).

Source and catalog were identical at the start (no cut-off edit found). The earlier batch report (`rework/08.md`, section 3) had reviewed the asset and changed nothing.

### How it was looked at

The Read tool failed on every call of this session ("PreToolUse hook did not respond before its timeout"), also for copies in the scratchpad, so no PNG was read as an image. Substitute: every sheet was rendered with `r3-sheet.ts` and turned into a character raster (84 to 230 columns, pure-python PNG decoder, members of the structure list marked by colour). That shows silhouette, proportions, pose, missing or stray parts and single ribs, but not surface quality, seams, small gaps, faces, hands or feet. Everything below that needs a real look is listed under "Remaining".

Rendered after the last change, all without a `NOT DRAWN` solid: 1:50 body and structure in all 14 poses (right view), two instants of each clip (nibble 0.7 / 1.6 s, bound 0.07 / 0.2 s, climb 0.6 / 1.9 s), body, thermal and structure sheets at 1:100, 1:200, 1:500, the 2D sheet at 1:50 and at 1:100 (`running`). Read as character raster: 1:50 body default pose right and front, `climbing`, `bound-support`, `bound-push`, `bound-gather`, `bound-flight`, bound at 0.2 s, nibble at 1.6 s; 1:50 structure default pose and `bound-extension`; 1:500 body; 1:100 structure top.

### Reference (Sciurus vulgaris, adult, from my own knowledge) against the model

| Measure | Reference | Model | Verdict |
|---|---|---|---|
| Head and body | 195–245 mm | 210 mm (72 + 62 + 24 + 52 along the bones) | ok |
| Tail vertebrae | 150–200 mm, 23–25 caudals | 175 mm, 23 centra on 6 bones | ok |
| Mass | 250–350 g | 300 g | ok |
| Skull length / zygomatic breadth | 50–54 / 29–32 mm | 52 / 30 mm | ok |
| Scapula | 26–30 mm | 31.3 mm (bone `scapula-n/-f`) | ok, upper end |
| Clavicle (present, climber) | 14–17 mm | 16.3 mm (bone `clavicle-n/-f`), was missing | new |
| Humerus / radius / hand with fingers | 38–42 / 38–41 / 24–28 mm | 40 / 40 / 26 mm | ok |
| Distance of the shoulder joints | 28–32 mm | 30 mm | ok |
| Hip bone, crest to ischium | 40–46 mm | 46 mm | ok |
| Sacroiliac joint to hip joint | 12–15 mm | 16.5 mm (bone `hip-n/-f`) | slightly long |
| Distance of the hip joints | 22–26 mm | 26 mm | ok |
| Femur / tibia / hind foot | 47–52 / 55–60 / 51–63 mm | 50 / 58 / 50 mm + toes | ok |
| Vertebrae | C7 T12 L7 S3 | C7 T12 L7 S3 (+2 caudals in the pelvis), was two smooth rods | new |
| Thoracic / lumbar vertebra length | about 4 / 7–8 mm | 3.9 / 6.6 mm | lumbar 10–15 % short |
| Sacrum length / breadth at the alae | about 19 / 16 mm | 25.5 / 20 mm | too large, see Remaining |
| Ribs | 12 pairs, 7 sternal, 3 on the costal arch, 2 floating; thorax 30–34 mm wide at ribs 9–10, about 11 mm at the inlet | 12 pairs so, 32.4 mm at rib 10, 11 mm at rib 1; before: 9 closed hoops of 22 mm diameter hanging under the column, sternum 10 mm below them | rebuilt |

### Defects found

1. Rig without girdles: arms hung from `spine-f`, thighs from `spine-r`; scapulae and pelvis halves were only solids on the spine bones; no clavicle.
2. Structure 1:50: ribs were nine closed revolve hoops (both sides in one solid) of 11 mm radius, not attached to the vertebrae they belong to and not reaching the sternum; pelvis halves 4.8 mm clear of the column, no sacrum, no pubis (open ring); column as smooth rods.
3. Coarse scales: 1:100, 1:200 and 1:500 carried the 1:50 fingers, claws, pupils and lids in the body list (36 to 42 solids each, with their thermal references), and the 18 phalanges and 23 caudal centra of 1:50 in the structure list; their decisions texts said these parts were merged.
4. Decisions texts did not describe the lists (see 3).

### What was rebuilt

Edit scripts (hand-chosen numbers, reproducible; running the four in this order gives the published source and catalog byte for byte): `rework-red-squirrel-girdles.py`, `rework-red-squirrel-coarse.py`, `rework-red-squirrel-decisions.py`, `rework-red-squirrel-poses.py`; they build from `rework-red-squirrel-orig.json` (rig, poses, structure lists and the hand/sole solids as they were before this pass).

- **Rig** (28 bones, was 22), lateral rest placement as in `rig-lateral-rest.md`:
  - `scapula-n/-f` on `spine-f`: dorsal border [0.1025, 0.0103, ∓0.0108] to the shoulder joint [0.124, −0.012, ∓0.015], 46° below the horizontal, lower end 7.7° outwards, joint ±20° (swings in its own plane). `arm-n/-f` re-parented to the scapula tip without moving.
  - `clavicle-n/-f` on `spine-f`: from the manubrium [0.1305, −0.0069, ∓0.0018] sideways to the acromion [0.126, −0.0095, ∓0.0172], joint ±15°. It is a leaf: the chain is thorax – scapula – humerus, as in a quadruped; the clavicle does not follow the scapula.
  - `hip-n/-f` on `spine-r`: sacroiliac joint [0.030, 0.0095, ∓0.0086] to the hip joint [0.016, 0.002, ∓0.013], joint ±2°. `thigh-n/-f` re-parented without moving.
  - Proof: `rig-rest.ts` gives the same world rest frame for arms and thighs as before (origin, u, v, w to four decimals), mirrored frames on both sides; so solids, 2D shapes, anchors, masses and all existing pose and clip offsets keep their meaning. Segment lengths and joint order of the limbs were right and are unchanged.
- **Poses**: in the four poses whose hands are off the ground (`running`, `bound-gather`, `bound-push`, `bound-flight`) the scapula takes 15 % of the humerus swing (12.5° to 19.8°) and the arm offset is reduced by the same angle: the humerus keeps its direction, the shoulder joint travels up to 10 mm forward on the arc of the scapula. Poses with hand contacts (`bound-extension`, `-support`, `-swing`, `climb*`, `climbing`) and the sitting poses are unchanged, so every contact is where it was.
- **Structure 1:50** (111 → 113 solids): scapulae re-bound to their bones and given spine and acromion; hip bones re-bound, plus one pubic bow per side from the acetabulum through the symphysis to the ischium (closed ring, obturator opening); new sacrum with alae that enter the ilia; collarbones; column beaded vertebra by vertebra with a crest of spinous processes on lumbar and thoracic part; 24 ribs as lofts along hand-set anatomical paths (head at its vertebra, angle, widest point, costochondral bend, sternal end or costal arch or free end); sternum with a broad manubrium and spatulate xiphoid.
- **Structure 1:100 / 1:200 / 1:500** (64 → 42, 63 → 38, 62 → 36): phalanges removed; 23 caudal centra replaced by one bar per tail bone; the single pelvis lump replaced by sacrum mass and two hip-bone bars; scapula and collarbone bars for the new bones; ribs as 5 / 3 / 2 pairs of the same paths; sternum bar (1:100 had none).
- **Body and thermal 1:100 / 1:200 / 1:500** (body 104 → 64, 94 → 51, 88 → 45): fingers, claws, pupils and lids removed (and muzzle pads, nostril recess at 1:200 and 1:500) together with their thermal references; the finger group is now a mitten to the finger tips and the sole runs to the toe tips, so the bare-skin regions keep their surface. Corneas and nose pad stay because the thermal regions `eyes` and `nose` need a carrier.
- **Decisions**: structure texts of all four scales and the last entry of the three coarse scales rewritten (en, de).
- Not touched: body and thermal lists of 1:50, all 2D drawings, anchors, thermal regions, masses.

### Remaining (precise, for the next pass)

- All scales, all lists: nothing was judged on a real image. Every sheet of the brief's step 1 has to be read with the image viewer, first of all 1:50 structure (new rib cage, pubic bow, sacrum, beaded column, collarbone meeting acromion and manubrium) from all six directions and in `climbing`, `bound-extension`, `sitting-eating`.
- 1:50 body: not re-modelled and not judged from six directions (trunk 52 × 62 mm with coat, head 42 × 38 mm, thigh 22 × 38 mm were only measured from the sections and look plausible). Shoulder region in the four poses with scapula rotation: check that the arm dome stays closed against the chest.
- 1:50 thermal: untouched, not judged.
- 1:50 structure: sacrum too long and too wide (25.5 × 20 mm against about 19 × 16 mm) because the ilia stand 19 mm apart; lumbar vertebrae 6.6 mm (7–8 mm real); vertebrae without neural arches and transverse processes, the spinous processes are one scalloped crest per bone; radius and ulna one bar, tibia and fibula one bar; metacarpals and metatarsals are one plate per hand and foot; skull one loft with mandible, no orbit, zygomatic arch or incisors; no joint solids for the new girdle bones.
- Rig: the spine is still two rigid bones (pelvis with five lumbar vertebrae, and L1–L2 with the thorax); the lumbar flexion of the bound happens at one joint. Splitting it (pelvis, lumbar, thorax) needs the rump loft split, 2D shapes at two scales and three views, and re-solved hand contacts. Hands and feet have no finger or toe bones (both sides alike). The clavicle does not follow the scapula in the four poses that rotate it (acromion and clavicle tip separate by up to about 10 mm there).
- Poses and clips: contact poses do not use the scapula yet; `climb` and `nibble` unchanged.
- 1:100 / 1:200 / 1:500 body: besides the removed details the lists were not re-modelled; brush locks, nose polyhedron and corneas are still the 1:50 solids.
- 2D drawings: unchanged; at 1:100 the hand and foot outlines were not redrawn for the mitten and the longer sole (the silhouette rule passes).
- Decisions: the body texts of 1:100 to 1:500 still say "eye and nose pad are gone" and count the solids differently from the lists; only the structure texts and the last entry were rewritten.

### Fundamental issues (outside this asset)

- Read tool: every call ended with "PreToolUse hook did not respond before its timeout (host client may be unreachable)", each after several minutes. Images cannot be read by an author while this lasts.
- `rig-lateral-compare.ts` throws `RepresentationError: ... no available representation at 1:200` for an asset whose coarse representations are unavailable (the squirrel); it should skip unavailable scales.
- `volumes-slender` / `volumes-feature` leave no valid form for a short load-bearing bone at coarse scales: a 16 mm collarbone must have a member solid at 1:100 to 1:500 (`volumes-structure-coverage`, bone above 5 % of the height), may not be `slender` unless 20 times as long as thick (so it is drawn 0.8 mm thick here) and may not be a plain solid under the feature size. The same holds for scapula and hip bone, which are bars instead of plates at the coarse scales for this reason.
- The repo MCP server (`semio`) did not connect in this session, so no ticket tool was called; all work lies in the given ticket folder.

## Pass 2 (second round), 2026-11-22

Status: **partial**. Published and valid: `r3-validate.ts animals/red-squirrel --catalog-only --volumes` → `VALID animals/red-squirrel (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after the last change). Three stages, each validated in full as a candidate (`--file … --volumes`, VALID) and then published: spine and collarbones, 1:50 skeleton, coarse lists and decisions.

### How it was looked at

The image viewer was unreachable again (the first Read call ended with "PreToolUse hook did not respond before its timeout"; the plan tool and the repo MCP server failed the same way), so again **no PNG was read as an image**. Substitute: `rework-red-squirrel-look.py` (sips → BMP → character raster with a ten-step grey ramp and colour marks, crop by fractions, up to 170 columns). It shows silhouette, pose, trunk continuity and the placement of the skeleton, not surfaces, seams, faces, hands or feet. Rendered from the catalog after the last change, all without a `NOT DRAWN` solid (48 renders): body, thermal and structure sheets at 1:50, 1:100, 1:200, 1:500; body and structure (right view) in all 14 poses at 1:50; clip instants nibble 0.7 / 1.6 s, bound 0.07 / 0.2 s, climb 0.6 / 1.9 s; 2D sheets at 1:50 and 1:100 in `running`. Read as raster: 1:50 body default pose right and front, `running` body and structure, `bound-extension` structure right and top, `bound-reach`, `bound-push`, `bound-flight` body, 1:100 body right. The rest was rendered and checked for drawing errors only.

### Verified from pass 1

Girdles on lateral rest placement are in the catalog as reported (`rig-rest.ts`: scapula, clavicle and hip frames as in the pass 1 table, arm and thigh world rest frames unchanged). The rib cage, pubic ring and beaded column render without error in every pose; they were judged on rasters only.

### Rig (29 bones, was 28)

| Segment | Reference | Model now |
|---|---|---|
| Pelvis with sacrum (tail root to lumbosacral joint) | 38–42 mm | `spine-r` 38.5 mm (was 72 mm with five lumbar vertebrae) |
| Caudal lumbar spine L3–L7 | 5 × 7–8 = 35–40 mm | new bone `lumbar` 33.5 mm, joint at the column (x 0.0385, y 0.0146 on the pelvis bone), ±40° |
| Thorax with L1–L2 | 12 × 4 + 2 × 7 = 62 mm | `spine-f` 62 mm, rest frame unchanged, re-parented to `lumbar` |
| Sacrum length / breadth at the alae | about 19 / 16 mm | 19.5 / 16.7 mm (was 25.5 / 20) at every scale |

- `lumbar` takes 0.010 kg of the 0.024 kg of the old pelvis bone (sum 0.30 kg unchanged).
- Poses with free hands move about 40 % of the trunk flexion into the lumbosacral joint: `running`, `bound-gather`, `bound-reach` −15° (thorax −23° / −22°), `bound-push` −5° (−8°), `bound-flight` −2° (−4°). Contact poses, sitting and climbing poses keep the lumbar joint at 0, so every hand and foot contact is where it was: `rig-lateral-compare.ts` against the catalog before this pass, with the cut trunk drawings put back for the comparison, gives 0 mm for all other poses and clip instants and at most 9.5 mm in the five poses named and the bound instants between them.
- Collarbone: joint limit ±50° (was ±15°), and in the four poses that turn the shoulder blade (`running`, `bound-gather`, `bound-push`, `bound-flight`) it turns with it (21° to 35° forward, 12° to 17° outward, solved per side): the tip stays at the acromion within 0.05 to 0.73 mm (was up to about 10 mm apart).
- Re-bound for the new bone at all four scales: body `rump` and `belly-patch-rump` cut at the joint into `rump` / `loin` and `belly-patch-rump` / `belly-patch-loin`, each end tucked 8 mm into the other at 90 % of its section; thermal underfur lofts and guard references the same; structure `m-spine-r` cut into `m-spine-r` and `m-lumbar` at 1:50 and 1:100 (one bar with `members: ["lumbar"]` at 1:200 and 1:500, where a 34 mm bar is below two features), spinous crest of the lumbar vertebrae moved to the bone, joint solid `j-lumbar`; elevation shapes `rump`, `rump-core`, `belly-patch-rump` at 1:50 and 1:100 cut into a pelvis and a loin shape (8 mm overlap).
- Script: `rework-red-squirrel-spine.py` (applies to the published state of pass 1; refuses a second run).

### 1:50 structure (113 → 142 solids), script `rework-red-squirrel-skeleton.py`

- Forearm: radius (head, shaft 2.6 × 2.2 mm, broad distal end) and ulna (olecranon 5.8 mm behind the elbow, thin shaft, styloid) instead of one bar.
- Shank: fibula (1.1 mm shaft, head under the lateral condyle, lateral malleolus) beside the tibia.
- Hand: carpus block and four metacarpals that end where the phalanges start; foot: tarsus with calcaneus (heel process 7.4 mm behind the ankle) and five metatarsals (the fifth shorter).
- Skull: braincase 24.6 mm wide, postorbital constriction 17 mm, rostrum 13.6 → 11 mm; zygomatic arches as separate bars from the squamosal root over the widest point (30 mm) to the maxilla, so the orbit is open; mandible as two rami with a high ascending part that meet at the symphysis (was one flat plate across the whole head); upper and lower incisor pairs.
- Sacrum at its true size (all scales).

### 1:100 / 1:200 / 1:500, script `rework-red-squirrel-coarse-tail.py`

- Body: the single hair locks of the brush (7 × 3 mm, 30 / 20 / 16 solids) left the three lists, at 1:100 also the muzzle pads and the nostril recess, with their thermal references (body 66 / 53 / 47 → 35 each; thermal → 37 each). Cross-scale, silhouette (1:100 drawing), thermal coverage and area rules pass without them.
- Decisions: body texts of the three coarse scales rewritten to what the lists hold (count, loin on the lumbar bone, mitten, sole, corneas and nose pad as carriers of their thermal regions); structure texts extended (lumbar bar or spanning bar, sacrum size); 1:50 body, thermal and structure texts extended by this pass's changes (en, de).

### Remaining (precise)

- All scales, all lists: still not judged on a real image (viewer unreachable in both passes). First to read: 1:50 structure head close-up (zygomatic arch, orbit, rami, incisors), forearm and shank (radius / ulna, fibula), hand and foot (metacarpals, metatarsals), rib cage and pelvis from six directions; 1:50 body trunk at the lumbosacral cut in `running`, `bound-gather`, `bound-reach` (tucked ends) and the shoulder dome in the four scapula poses.
- 1:50 body: not re-modelled in either pass (only the trunk cut); six directions not judged.
- 1:50 thermal: only the trunk cut; not judged.
- 1:50 structure: vertebrae without neural arches and transverse processes (spinous processes are one scalloped crest per bone); lumbar vertebrae 6.7 mm (7–8 mm real); no joint solids at the sternoclavicular and sacroiliac joints; carpals and tarsals are one block each.
- Rig: the thorax bone still carries L1–L2 and pivots on the body axis (y 0), 14.6 mm below the column, as before (moving the pivot to the column changes every contact pose); no finger or toe bones (both sides alike); `hip` 16.5 mm (12–15 real).
- Poses and clips: contact poses (`bound-extension`, `-support`, `-swing`, `climbing`, `climb-a/b/c`) and the sitting poses use neither the shoulder blade nor the lumbar joint (both need the hand contacts re-solved).
- 1:100 / 1:200 / 1:500 body: nose polyhedron and corneas are still the 1:50 solids (kept as carriers of the thermal regions); the three lists now hold the same 35 parts and differ only in their sections; the limbs were not re-modelled.
- 2D drawings: front and plan shapes of the trunk are still one shape on the pelvis bone (they are world-rest drawings); 1:100 hand and foot outlines not redrawn for mitten and longer sole.

### Fundamental issues (outside this asset)

- Read tool, plan tool and the repo MCP server: "PreToolUse hook did not respond before its timeout" / connection timeout in this session too; an author cannot read images or use the ticket tools while this lasts.
- `rig-lateral-compare.ts` pairs the points of a shape by index: a shape whose outline was cut (same id, fewer points) reports 30 to 54 mm of "move" in poses where no bone moved. It should compare the placed outline (for example the Hausdorff distance) or say that the point count changed.
- `r3-2d.ts` sheets are thin line drawings on white; without the image viewer they cannot be checked at all (a `--fill` or thick-pen debug option would help a raster check).
