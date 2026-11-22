# humans/leaning-person

## Pass 1 (second round), 2026-11-22

Author of this pass: one agent, this asset only. State found, what was changed, what is still wrong.

### State found

- No report existed. `rework/01.md` (author 01) never reached this asset.
- An earlier agent of this round had left `rework-leaning-person-build.py`, `-stage1.py`, `-stage2.py` and had published at 13:52. Catalog and `T/volumes/humans/leaning-person.json` were identical. Stage 2 (coarse hoodies) was written at 13:57, after that publish, and was not in the catalog.
- That work already held: `lumbar` bone, clavicles and hip bones on the lateral rest placement of `rig-lateral-rest.md` (arms re-parented to `clavicle-l/-r`, legs to `ilium-l/-r`, largest move of a pre-existing bone origin 0.0035 mm), 15 finger bones per hand, a skeleton per scale. I judged it by rendering and kept it.
- The build reads the pre-rework catalog snapshot `🗑️generated/rework-leaning-person/base.json` and applies every stage. That file is the input of the build and must stay (it sits in the generated folder; see "remaining").

### Reference (slim man, 1.72 m barefoot, 66 kg; sneakers 20 to 26 mm sole, beanie)

| Quantity | Real (Drillis-Contini / Winter 2009, DIN 33402, ANSUR II) | Model (rig rest pose, 1:50 solids) |
|---|---|---|
| Stature barefoot | 1.72 | 1.727 (vertex 0.822 above the hip joint, hip joint 0.905 above the sole of the foot) |
| Hip joint height | 0.530 H = 0.912 | 0.905 |
| Thigh (hip to knee) | 0.245 H = 0.421 | 0.4202 |
| Shank (knee to ankle) | 0.246 H = 0.423 | 0.4105 (12 mm short) |
| Ankle height | 0.039 H = 0.067 | 0.088 to the shoe sole underside less 0.02 sole = 0.068 |
| Shoulder joint height | 0.818 H = 1.407 | 1.423 |
| Upper arm / forearm / hand | 0.186 H = 0.320 / 0.146 H = 0.251 / 0.108 H = 0.186 | 0.318 / 0.252 / 0.098 + middle finger 0.095 = 0.193 |
| Shoulder joint centres apart | 0.34 to 0.36 | 0.350 |
| Biacromial breadth | 0.39 to 0.40 | acromioclavicular joints 0.330 apart (narrow); hoodie 0.386 broad at the shoulders; breadth over both sleeves not measured |
| Hip joints apart | 0.17 to 0.18 | 0.180 |
| Clavicle length | 0.15 to 0.16 | 0.166 |
| Neck C7 to atlanto-occipital / head to vertex | 0.11 / 0.14 to 0.15 | 0.110 / 0.139 |
| Head height, breadth, length | 0.23 / 0.152 / 0.196 | head loft 0.152 broad, 0.196 long |
| Neck breadth | 0.115 to 0.12 | 0.110 to 0.132 |
| Chest breadth / depth clothed | 0.32 + ease / 0.23 + ease | hoodie 0.386 / 0.22 to 0.26 |
| Thigh girth top / above knee | 0.55 / 0.38 | sections 172 x 196 mm (0.58) / 113 x 121 mm (0.37) |
| Calf max | 0.36 (115 mm) | jeans leg 116 x 124 mm at 0.14 m below the knee, centre 11 mm behind the axis |
| Upper-arm sleeve / forearm sleeve / wrist | 0.30 + fleece / 0.26 / 55 x 40 mm | 111 x 116 mm / 87 x 91 mm / 57 x 38 mm |
| Hand breadth / thickness | 0.084 / 0.028 | 0.084 / 0.025 to 0.035 |
| Finger lengths index / middle / ring / little (three phalanges) | 0.087 / 0.095 / 0.089 / 0.072 | 0.087 / 0.095 / 0.089 / 0.072, both hands since this pass |
| Shoe length / breadth | 0.285 / 0.105 | 0.293 over sole and toe / 0.108 |

### Defects found (read at full size: 1:50 body six views in all five poses, close-ups of the right hand and the face; 1:100, 1:200, 1:500 body; structure 1:50, 1:200, 1:500; thermal 1:50 cut; 2D 1:50; clip instants)

1. Rig, right hand: the finger bones had been measured from the curled grip. Index proximal 34.8 mm against 41.8 mm on the left, little proximal 41.8 mm against 34.3 mm (the little finger longer than the index), end joints resting at 110 degrees, thumb bones 2 to 6 mm shorter than left. Both hands were not alike.
2. Rig, finger joint limits: middle and end joints could only extend 5 degrees from a rest curl of 22 degrees, so a finger could never be straightened.
3. 1:50 body, elbow: the forearm sleeve began 84 x 88 mm under an upper sleeve ending 92 x 97 mm; a step like a rolled edge on the hanging arm.
4. 1:200 and 1:500 body (stage 2 as left): the rounded hoodie no longer reached the wall, 17.5 mm and 15.3 mm from it (`volumes-connected`).
5. Pose `leaning-arms-folded`: both open hands reached about 60 mm past the opposite elbows.
6. Decisions of 1:200 and 1:500 described a skeleton that no longer existed (rib hoops, 38 mm bars).

### What was done

- `rework-leaning-person-stage3.py` (rig, poses): right finger and thumb bones take lengths and rest placement of the left hand by the mirror rule. The phone grip is now a pose offset in all five poses: index, middle and ring with the first phalanx flat behind the phone and the middle phalanx hooked round its long edge (knuckle 2 to 3, middle joint 78 to 80, end joint 28 to 30 degrees), little finger round the lower corner (-5 / 25 / 20), thumb as it lay with its end joint pressed 12 degrees back. Limits on both hands: knuckle -35..76, middle joint -25..90, end joint -25..60, thumb end joint -25..80 (offsets on the rest curl). Right-hand finger capsules of body and structure and the metacarpal ends follow the new bones.
- `rework-leaning-person-stage4.py` (1:50 body and thermal): four forearm sections round the elbow set to the girth of the upper sleeve, the air layer 4 mm inside.
- `rework-leaning-person-stage2.py` (1:200, 1:500 body): the hood is merged into the upper back (five slices moved back by 8 to 22 mm), so the garment bears on the wall again.
- `rework-leaning-person-stage5.py` (pose): in `leaning-arms-folded` the left fingers and thumb close (46 / 48 / 26 degrees on the rest curl).
- `rework-leaning-person-stage6.py` (decisions): rig and skeleton stated for 1:50 and 1:100, the skeleton sentences of 1:200 and 1:500 corrected, en and de.
- Judged and kept as found: trunk rig and girdles, 1:50 body (hoodie, jeans, head, sneakers), the skeleton of every scale (1:50 skull with mandible, vertebrae, ribs and sternum, scapulae, clavicles, pelvis, long bones, hands, feet; 1:200 rods, rib barrel and girdle parts; 1:500 spine rod, pelvic ring, joints at the 75 mm floor), thermal lists (layers nested in the cut sheet), 2D drawings of 1:50 (agree with the body; the silhouette rule passes at every scale).

### Validation

- `r3-validate.ts --file <candidate> --volumes`: `VALID file/lp-cand (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Published with `python3 rework-leaning-person-build.py --publish` (catalog keys `rig`, `anchors`, `poses`, `clips`, `volumes`, `representations`, `structure`, `thermal`, `dimensions`, and the volumes source).
- `r3-validate.ts humans/leaning-person --catalog-only --volumes`: `VALID humans/leaning-person (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- No rule, test or shared tool was changed. Vitest suites were not run.

### Rendered and read after the last geometry change

1:50 body: `leaning-shoulder-wall` (before stage 4 as six views; hand close-up after stage 3 with the first little-finger numbers), `leaning-arms-folded`, `phone-call`, `standing-away`, `pushing-off`, `push-off` 1.1 s, `phone-call-idle` 3 s. 1:100, 1:200, 1:500 body. Structure 1:50 and 1:200 (`standing-away`), 1:500. Thermal 1:50 cut. 2D 1:50.
Rendered and not read: `push-off` 1.9 s, `phone-call-idle` 1.5 s, structure 1:100, thermal 1:200. Not rendered: thermal 1:100 and 1:500, 2D of 1:100, 1:200, 1:500, the right-hand close-up after the final little-finger and thumb numbers.

### What remains imperfect

- Pose `leaning-arms-folded`: the hands lie beside the opposite upper arms, not on them; the right hand with the phone still reaches past the left elbow. Needs the wrists and forearm twist re-solved by hand.
- Pose `phone-call` and clip `phone-call-idle`: the description says the left hand is in the pocket; it lies flat in front of the kangaroo pocket.
- 1:50 right-hand grip: seen once in a close-up; the phone passes close to the palm and the final little-finger and thumb offsets were not looked at again.
- 1:50 legs: the jeans are close to a straight tube from knee to hem (calf 116 x 124 mm, hem 112 x 120 mm); a slim leg would show more calf and a narrower shin. Shank 12 mm short of the reference.
- Elbow at 1:100, 1:200, 1:500: the overlap of upper sleeve and forearm shows as a band on the hanging arm.
- 1:200 and 1:500 hoodie: a sharp ledge behind the neck where the merged hood ends.
- Biacromial breadth 0.33 m between the acromioclavicular joints is at the narrow end for a 1.72 m man (0.36 to 0.39); widening needs the arms' front and plan shapes shifted (`rig-lateral-rest.md` 1.6, item 3).
- The build depends on `🗑️generated/rework-leaning-person/base.json` (snapshot of the catalog asset before this round). If the generated folder is deleted the scripts cannot rebuild; the next author should either keep it or turn the stages into edits of the catalog file.
- The body, thermal and structure lists of the earlier agent were judged by sheet, not re-measured solid by solid.

### Fundamental issues (outside this asset)

- `r3-sheet.ts --file` and `r3-2d.ts --file` name the output after the folder of the candidate. Several authors use the folder name `cand`; my first render `🗑️generated/r3-sheet/file/cand.s50.body.png` showed another author's wheelchair user. Use a folder name with the slug; the tool should take the asset id from the document.
