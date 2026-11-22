# animals/pig — rework report

Earlier work on this asset: `r3-animals-pig.md` (round 3 and round 5, generator scripts `r3-pig-*`, `r5-pig-*`). Batch 07 listed it as "not started". No earlier file of this name existed.

## Pass 1 (second round) — 2026-11-22

Status: **partial**. The girdles are on the lateral rest placement, the asset is `VALID` with the measured rules and published, every pose and both clips were rendered after the last geometry change. The 1:50 body, the coarse structure lists and the 2D drawings are not at the level I would sign; the open list is at the end.

Scripts (ticket folder, all idempotent, hand-stated numbers):

- `rework-pig-rig.py` — girdle conversion of rig, poses, bound solids and scapula 2D shapes (`--apply` writes catalog and source).
- `rework-pig-lists.py` — hand edits of the volume lists at the four scales and the `decisions` texts (edits `volumes/animals/pig.json`).
- `rework-pig-2d.py` — 1:500 shapes that carry two thermal regions.

The older `r3-pig-*` / `r5-pig-*` scripts still describe the rig before the girdle conversion. Running them again would undo it; they are superseded by the source `volumes/animals/pig.json` plus the three scripts above.

### Source against catalog at the start

`volumes/animals/pig.json` and the catalog `asset.volumes` were identical at all four scales. The asset was **not** valid any more under today's rules: 62 `validateAsset` findings (8 claws at 1:200 below 30 mm, 14 body and 38 structure solids at 1:500 below 75 mm, 2 `region-undrawn`) and 9 `volumes-connected` findings (floating dewclaws, ears, ribs, scapulae, an open load path at 1:100).

### Reference table (100 kg finishing pig, Large White x Landrace type; textbook values from my own knowledge, not measurements of one animal)

| item | reference | model | judgement |
|---|---|---|---|
| height of back / withers | 0.64–0.71 m | 0.684 / 0.65 | ok |
| length snout to tail | 1.20–1.30 m | 1.27 | ok |
| width over hams | 0.30–0.35 m | 0.331 | ok |
| scapula (dorsal border to glenoid) | 0.18–0.20 m, about 55° to the horizontal, lower end further out | 0.1925 m, 56°, dorsal border 0.05 m and shoulder joint 0.095 m beside the plane | ok after this pass (was a sagittal bone with a 45 mm sideways jump to the humerus) |
| humerus / radius / metacarpus / fore digit | 0.18 / 0.14 / 0.09 / 0.09 m | 0.178 / 0.140 / 0.090 / 0.090 | ok |
| hip bone, sacroiliac joint to acetabulum | 0.12–0.14 m; sacroiliac joints 0.07–0.09 m apart, acetabula 0.16–0.18 m apart | 0.1329 m; 0.08 m and 0.17 m apart | ok after this pass (femora were hung directly on the midline pelvis bone with a z offset) |
| femur / tibia / metatarsus / hind digit | 0.20 / 0.185 / 0.13 / 0.095 m | 0.200 / 0.185 / 0.129 / 0.095 | ok |
| joint order fore | scapula – humerus – radius/ulna – carpus – metacarpus – digits | same | ok |
| joint order hind | hip bone – femur – tibia – tarsus – metatarsus – digits | same after this pass | ok |
| clavicle | none in the pig (trunk hangs in a muscular sling, synsarcosis) | none | ok |
| thoracic / lumbar spine | 14–15 thoracic, 6–7 lumbar; 0.36 / 0.245 m | 0.359 / 0.2455 m, 14 ribs at 1:50 | ok |
| head, poll to disc | 0.26–0.30 m | 0.262 | ok |
| lop ear | 0.15–0.20 x 0.10–0.12 m, 4–6 mm thick | 0.17 x 0.11 m; 8 mm at 1:50 (was 9–18 mm), 15 mm at 1:100, 5 mm sheet at 1:500 | 1:50 ok, still thick at 1:100 and 1:200 |
| claw | 50–60 x 22–25 x 30–35 mm, two per foot, two dewclaws | 52 x 24 x 35 mm | ok at 1:50 and 1:100 |
| forearm girth, cannon width | forearm about 75 mm below the elbow, cannon 40–45 mm | 80–90 mm, 44–52 mm | ok in size, no muscle or tendon form |

Both sides of the rig are alike (mirror rule: `z`, `abductionDeg`, `twistDeg` negated).

### Defects found by looking (1:50 six views, structure, poses)

- Rig: scapulae and femora attached without girdle geometry (table above).
- 1:50 body: ears were flat hexagonal planks; dewclaws floated 3.8 mm off the pastern.
- 1:50 body, still there: shoulder pad reads as an oval disc with a seam on the barrel; legs are smooth tubes with a step at carpus and fetlock; neck has a collar ring behind the ears; head is a plain wedge without jowl, lips or nostrils; trunk is nearly parallel-sided in plan with one seam ring at the thoracolumbar joint.
- Structure: root joint marker floated between the hips; at 1:100 ear cartilage and five ribs did not touch skull and spine; at 1:200 and 1:500 the scapulae hung beside the rib cage without any connection; the 1:500 list was the 1:100 parts list with sections of 8–52 mm.
- 1:200 body: no upper arm, so the far forearm did not meet the trunk in the standing pose.

### Rebuilt

**Rig (stage 1).**
- `scapula-fn/-ff`: rest `abductionDeg ∓13.5166`, length 0.1872 → 0.1925 m; origin and tip (the shoulder joint) at the same world points as before.
- New `ilium-hn/-hf` (hip bones: ilium, ischium, pubis half) under `pelvis`: origin at the sacroiliac joint (0.125 m along the pelvis bone, ∓0.04 m), `angleDeg 180`, `abductionDeg ∓19.7989`, 0.1329 m, joint ±3°, tip at the hip joint. No mass of their own (the pelvis mass includes them).
- `humerus-f*` and `thigh-h*` re-parented without moving (`x`, `angleDeg`, `abductionDeg`, `twistDeg` solved; rest frames equal to the old ones within 0.2 mm, checked in the script and with `rig-rest.ts`).
- Poses: the 13 poses with a scapula swing (standing and the 12 walk phases) were re-solved. The scapula now swings about its own tilted joint axis; flexion of humerus, forearm and metacarpus was solved so that fetlock position and sole pitch in the side elevation are unchanged (0.00 mm). No abduction or twist offsets on the limb (they would make every flat front and plan shape of the fore limb `lateral-ambiguous`). Consequence, stated plainly: a fore toe now moves up to 9.4 mm sideways with the scapula swing (in the standing pose the far fore toe, in the walk ±6 mm over the cycle).
- Solids bound to a scapula (body shoulder, blade, spine of the scapula, joint markers) and the ilium / ischium / hip-bone solids (moved from `pelvis` to the new bones) were re-expressed in the new frames: 30 solids per file, same world geometry.
- 2D: the 1:50 elevation shape `shoulder-n` moved from the scapula to the thorax (same place on paper; a flat shape on a bone that leaves the paper plane is ambiguous). It no longer follows the scapula swing of the walk (up to 0.4 mm on paper). The two small 1:500 scapula elevation shapes were stretched by 1 / cos 13.5°.
- Thermal regions, anchors, structure settings: nothing is bound to the changed bones; unchanged.

**1:50.** Ears: smooth leaf through the six hand stations (16 sections and a rounded tip), 8 mm thick. Dewclaws seated on the pastern. Structure: root joint marker moved into the sacrum; joint markers at the two sacroiliac joints.

**1:100.** Ear roots sunk 12 mm into the head, ear leaf smoothed; ear cartilage rooted in the skull; rib heads placed on the vertebral bodies; tail root seated on the sacrum.

**1:200.** Upper arm added (body). The two claws of each foot merged into one cloven hoof 48 mm wide (body and thermal). Structure: scapulothoracic joint solid on each side.

**1:500.** Body: ears as declared 5 mm sheets; pastern and claws replaced by one foot solid per leg (52–60 mm). Thermal: per foot a pastern solid (leg skin) and a hoof solid (hooves) cut from that foot. Structure remodelled: spine, ribs, femur, tibia, humerus, radius-ulna with olecranon and metatarsus with calcaneus as true shafts declared `slender`; neck, sacrum, hip bones, metacarpus and digits merged to 75 / 51 mm; scapula and ear cartilage as declared sheets; a tail-head lump for the load-bearing `tail-1`; scapulothoracic joint solids; all members touch at their joints.

**2D.** At 1:500 `head` names the regions skin-head and snout, `hindbody` skin-trunk and tail (fixes `region-undrawn`). No other shape was redrawn.

**Decisions.** One sentence per scale (en, de) appended to `volumes[scale].decisions`.

### Validation

- `r3-validate.ts animals/pig --volumes`: `VALID animals/pig (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`.
- After `volumes-insert.py`: `r3-validate.ts animals/pig --catalog-only --volumes`: `VALID animals/pig (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- `rig-rest.ts`: scapula tip [0.705, −0.060, ∓0.095], ilium tip [0, 0, ∓0.085], humerus and thigh `u`, `v`, `w` sagittal as before.

### Sheets read

- Before any change: 1:50 body and structure (six views).
- After the rig change: 1:50 structure (candidate), 1:50 body `walk-phase-9`, 2D 1:50 `rooting` before and after (identical).
- After the list edits: 1:500 structure and body; 1:50 close-up three-quarter; 1:50 and 1:100 body after the ear remodel; 1:50 `lying-wallowing`; 1:50 `root-cycle` at 2 s; 1:200 structure; 1:500 thermal cut; 2D 1:50.
- Rendered but **not read**: 1:50 `rooting` pose sheet, `walk-cycle` at 0.3 s and 0.8 s, `root-cycle` at 0 s (all after the last change), 1:200 body, 1:100 structure, thermal 1:50 / 1:100 / 1:200, 2D 1:500. The 2D of 1:100 and 1:200 was not rendered.

### Remaining (for the next pass)

- 1:50 body: shoulder pad (oval disc with a seam, should blend into the barrel); legs (no forearm muscle, no tendon relief, step at carpus and fetlock); neck collar ring and flat chest cap; head (no jowl, lips, nostrils; ears are flat in cross-section, not cupped); trunk seam ring at the thoracolumbar joint and parallel-sided plan.
- 1:50 structure: skull is a wedge block without orbit or zygomatic arch; pelvis halves are bars rather than a winged ilium; not re-modelled in this pass.
- 1:100 structure: ear cartilage is a 16 mm rod, should be a sheet as at 1:500. 1:100 thermal and structure sheets not read.
- 1:200 body: ears 17 mm thick; body sheet not read. 1:200 structure: ribs are 30 mm tubes (could be true sections declared `slender`), hip bones thin bars.
- 1:500 structure: `near/far-rib-3` reads as a detached hoop near the scapula; the tail-head lump is a 75 mm ball forced by the coverage rule (see fundamental issues).
- 2D: 1:50 `shoulder-n` no longer follows the scapula; ear outlines in plan at 1:50 and 1:100 still show the old angular ear (the silhouette rule passes); pose drawings show the leg segments as separate blocks; 1:100, 1:200 and 1:500 drawings not read.
- Poses: fore toes drift up to 9.4 mm sideways with the scapula swing; the lying pose still puts a hind claw tip into the ground plane (noted in round 5, not changed).
- Thermal: the 1:50 list was not looked at after the ear and dewclaw change beyond the validator.

### Fundamental issues (outside this asset)

- `rig-lateral-compare.ts` compares placed shapes by list index. Adding a bone changes the order of the evaluated shapes, so it reported moves of 40–100 mm for a conversion whose 2D drawings are pixel-identical (checked by rendering `rooting` before and after). It should match shapes by id and bone.
- `lateral-ambiguous` (`🔨️modules/🗂️catalog`): as soon as a pose gives a limb bone an `abductionDeg` or `twistDeg` offset, every flat front and plan shape of that bone and its descendants is reported with the total flexion of the pose as the "tilt" (up to 134° here). A limb under a tilted girdle bone therefore cannot keep its posed plane exactly; I had to re-solve flexion only and accept the sideways drift. The guide `rig-lateral-rest.md` 1.6 item 4 recommends exactly the offsets that trigger this.
- `volumes-structure-coverage` together with `volumes-feature`: a short load-bearing bone (tail root 43 mm, metacarpus 90 mm, digits 95 mm) needs a member at 1:500 that is at least 51–75 mm thick, because it is too short to be `slender`. The result is a lump that is not the bone. Either the 5 % height threshold for "load-bearing" should not apply at a scale where the bone is below the feature, or such a bone should be allowed to be covered by its parent's member.
- The pig scripts of rounds 3 and 5 are generators that rewrite the rig and all lists; they are now stale (see top).
