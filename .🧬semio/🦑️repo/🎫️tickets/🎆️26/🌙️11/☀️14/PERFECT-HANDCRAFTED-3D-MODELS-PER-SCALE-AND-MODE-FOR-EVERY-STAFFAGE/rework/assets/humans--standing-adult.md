# humans/standing-adult — author report

## Pass 1 (second round), 2026-11-22 15:35–16:45

State found: no report file. Author 01 (`rework/01.md`) rebuilt rig, body, skeleton and lists in the night; the rig agent turned the clavicles into lateral bones (`rig-lateral-rest.md` 1.3); a first round of this pass (cut off by a reboot of the host at 15:33, no report) added the pelvic girdle (`rework-standing-adult-pelvis.py`), a skull-fitting hair cap (`-head.py`) and re-ruled filled outlines at 1:50 / 1:100 (`-2d.py`). Source and catalog differed in the four body lists only (the head change was in the source, not published). Judged by rendering: the source was the better state; both agree now (published three times in this round).

### Reference (neutral adult, 1.69 m barefoot, 58 kg, de Leva female table; 26 mm sole)

| Quantity | Real | Model (measured) | Verdict |
|---|---|---|---|
| Stature with shoes and hair | 1.72 | 1.72 (`dimensions.heightM`) | ok |
| Head vertex–menton | H/7.5 = 0.225 | 0.225 (head bone 0.149 + menton 0.076 below the joint) | ok |
| Hip-joint height | 0.53 H + sole = 0.92 | 0.903 | ok (joint centre, not trochanter) |
| Thigh (hip–knee) | 0.245 H = 0.414 | 0.41325, both sides | ok |
| Shank (knee–ankle) | 0.246 H = 0.416 | 0.40375, both sides | 12 mm short, inside the spread of the tables |
| Ankle height with sole | 0.039 H + 0.026 = 0.092 | 0.086 | ok |
| Upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.314 / 0.247 / 0.183 | 0.3135 / 0.247 / 0.098 + 0.084 fingers, both sides | ok |
| Shoulder joint height | 0.818 H + sole = 1.408 | 1.3975 | ok |
| Biacromial breadth | 0.355–0.365 (female P50) | 0.360 (acromion ±0.180, joint centre ±0.153) | ok — the rig agent asked for a decision: 0.36 is confirmed for this figure; 0.39–0.40 would be a 1.75 m male |
| Clavicle | 0.14–0.15 | 0.157 joint to joint, 19° back, 11° up | ok |
| Hip-joint spacing | 0.17 | ±0.085 | ok |
| Bi-iliac breadth | 0.28 | crest to ±0.136–0.14 | ok |
| Hip breadth clothed | 0.34 | 0.336 | ok |
| Buttock depth behind the hip joint | 0.11–0.12 | 0.111 (seat loft), was 0.120 as two balls | corrected form |
| Lumbar / thorax / neck | 0.17 / 0.30 / 0.10 | 0.170 / 0.297 / 0.102 | ok |

Rig topology (58 bones, symmetric): pelvis → lumbar → torso → neck → head; torso → clavicle-l/-r (lateral) → arm → forearm → hand → thumb (3) and four fingers (3 each), both hands alike; pelvis → ilium-l/-r (lateral, sacroiliac joint to hip joint, 0.097 m) → thigh → shank → foot → toe. Joint order and segment lengths agree with the table on both sides; nothing in the rig had to be changed in this round.

### Defects found (sheets read at full size) and what was rebuilt

Script: `rework-standing-adult-forms.py` (hand-chosen numbers, idempotent), 2D: `rework-standing-adult-sleeves.py`.

| Scale / list | Part | Defect | Done |
|---|---|---|---|
| 1:50, 1:100 body (thermal follows through `of`) | `buttock-l/-r` | two balls (0.058 × 0.150 × 0.144) standing out of the seat on every side, read as a nappy in the back view of every pose | ellipsoid 0.060 × 0.150 × 0.132 at [-0.020, 0.046, ±0.068] of the pelvis: top and flanks buried in the seat loft, which carries the 0.111 m depth; only the lower round shows towards the thigh |
| 1:500 body | `shank-l/-r` | ended 14 mm above the shoe (a visible gap, inside the 15 mm tolerance of `volumes-connected`) | last section at 0.428 m: the shank runs into the shoe |
| 1:500 structure | `rib-l/-r-2/-6/-9` | six rib hoops of 10–13 mm read as a pretzel at a 75 mm feature | one `rib-cage` basket on `torso` (five sections, apex 0.128 wide at rib 1, 0.28 at rib 8, 0.21 deep), like the pelvic girdle of that scale |
| 1:200 structure | `iliac-wing-l/-r` | a leaf ellipsoid beside the hip-bone bar, read as detached | blade loft from the acetabulum to the crest, its plane from the anterior superior spine back and inwards |
| 2D, all four scales, side and front elevation | sleeves, forearms, cuffs (36 shapes) | `extruded` prisms: with the upper arm turned about its axis (`arms-crossed`, `hands-warming`, `shiver`) they drew as hatched slabs 0.13 m wide (also reported twice by the rig agent) | same body of revolution about the bone as the plan twin (`revolved` about `u`); `arms-crossed` at 1:50 read afterwards: round, tapering sleeves in side and front |
| decisions | 1:50 body and structure, 1:100 body, 1:200 structure, 1:500 body and structure | described the ribs, hip bones and seat of the previous state | rewritten, en and de |

Tried and withdrawn: finger rows for the 1:100 hand skeleton — the list already has the finger rays (seen in the side view only after the first attempt); nothing of it is in the source.

Read and judged good, not changed: 1:50 body in `relaxed`, `arms-crossed`, `pointing`, `hands-warming` (six views each; limbs leave from the girdles, no gaps at shoulder, elbow, hip or knee); 1:50 structure and thermal in `relaxed`; 1:100 structure; 1:200 body and structure; 1:500 body and structure.

### Validation

- `r3-validate.ts humans/standing-adult --volumes` (source merged, with print checks): `VALID … schema 0, validateAsset 0, checkVolumes 0`, no PRINT line.
- After the last publish and the 2D change: `r3-validate.ts humans/standing-adult --catalog-only --volumes`: `VALID humans/standing-adult (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`, no PRINT line.

### What is not done or not checked (plainly)

- From 16:35 the Read tool stopped answering (hook timeout), so no image could be read after that. Rendered but NOT read: the 2D drawings after the sleeve change in `relaxed` (all four scales), `pointing`, `hands-warming`, `shiver`; the validator's silhouette and view rules pass for them, the look is unverified.
- Never read in this round: any clip instant (`idle-breathing`, `shiver`, `fanning`; rendered twice), the structure list in the three other poses, thermal at 1:100 / 1:200 / 1:500, body at 1:100 after the buttock change.
- 1:50 / 1:100 body: the back of the jeans is now a plain seat; there is no gluteal fold line, no cleft, no back pockets or yoke. The front of the seat reads as a brief-shaped panel between the thigh tops.
- 1:500: skull is a faceted block, shoes are plain bars (75 mm feature); 1:200 hand skeleton ends at mid hand.
- `hands-warming`: the kernel clamps thumb and finger twist at the joint limit (rig agent, `rig-lateral-rest.md` 4.2/4.5): 3D model and 2D differ by up to 5.5 mm in that pose. Not touched.
- 2D side elevation: the hem rib projects beyond the lumbar hollow as a band; it is what the 3D projects, not redrawn.
- `rework-01-standing-adult-{rig,body,skeleton,lists,build,2d}.py` (author 01's build) describe the state before the girdles and before this round; running that build again would undo all of it. They were not updated.

### Fundamental issues / notes outside the asset

- `sharp` under bun hangs since the reboot (`rework-standing-adult-montage.ts` slept for ten minutes without output); montage and crop helpers are unusable right now.
- By mistake I killed one hung process of another author (`bun rework-small-leaved-lime-montage.ts …`, pid 3998, sleeping since its start) while clearing my own hung montage. No file was touched; that author has to start the montage again.
- 2D `extruded` shapes on a limb that twists draw as slabs in every human asset built on this one (walking-commuter, standing-phone-user, …): the remedy used here (the plan's `revolved` twin in side and front elevation) is a ten-line change per asset.
