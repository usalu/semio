IN WORK

# 🧍️ Multiview Fixer MH-2: Twelve People

Third fixer (takeover of 2026-11-25 09:00). This file is rewritten as one report when the batch ends; until then the table of section 1 is the measured state and is updated after every finished asset. The interim report of the predecessors (04:35) is kept as `🧰️matrix/MH-2/interim-report-0435.md`.

## 1. State (fourth fixer, takeover of 2026-11-25 19:00; updated after every asset)

Catalog files of the nine are unchanged since 2026-11-24 (md5 before and after every matrix equal). Matrix = `T/📊️matrix.ts --assets <id>`, run to its end, log `[matrix] 252 cells`.

| Asset | Gate (twins) and scan, hard | Check CLI | Matrix: fail cells (ended) | Tests |
|---|---|---|---|---|
| `jogger` | 0, 0 (16:00) | ✓ 14:36 | 0 of 252 (11:29) | silhouette passed 18:01; five suites queued |
| `kneeling-gardener` | 0, 0 (16:04) | ✓ 14:36 | 0 of 252, 42 unavailable (12:17) | same |
| `leaning-person` | 0, 0 (16:09) | ✓ 14:37 | 0 of 252 (18:49) | same |
| `mobility-scooter-user` | 0, 0 (16:12) | ✓ 14:38 | 0 of 252 (15:03) | same |
| `painter-at-easel` | 0, 0 (11:21) | ✓ 14:39 | 0 of 252 (15:47) | same |
| `parent-carrying-toddler` | 0, 0 (11:36) | ✓ 14:40 | 0 of 252 (16:25) | same |
| `parent-with-child-on-shoulders` | 0, 0 (11:44) | ✓ 14:40 | 0 of 252 (18:43) | same |
| `parent-with-stroller` | 0, 0 (12:27) | ✓ 14:43 | 0 of 252 (19:00) | same |
| `person-carrying-ladder` | 0, 0 (12:36) | ✓ 14:44 | running | same |
| `nurse-walking` | catalog: gate 1528, scan 61; loft trial 0 hard, guard OK | | not run | loft plan being written (19:10) |
| `office-worker-carrying-laptop` | catalog: gate 200, scan 11; loft trial 0 hard, guard fails 1:50 elevation 48.8 mm | | not run | in work |
| `patient-with-iv-pole` | catalog: gate 4194, scan 176; loft trial 0 hard, guard OK | | not run | loft plan being written (19:10) |

## 2. What was wrong and what was changed

The ops of every asset are `🧰️matrix/MH-2/ops/<id>.json` (applied by `apply.sh`, the last step of the asset's chain; idempotent); two assets also have `anchors/<id>.json`.

### `jogger` (10 ops)

| Shape | Defect | Repair |
|---|---|---|
| plan `neck`, 4 scales | flat sheet on a bone the stretch clip pitches with the trunk (74 mm, plan a line 11 mm long) | `plan-revolve`: body of revolution fitted to the elevation thickness and the front width |
| plan `shorts` (pelvis), 4 scales | flat sheet, 43 mm in the hamstring stretch | `plan-prism`: the plan outline kept as the transverse profile and swept along the bone over the length of the elevation outline (footprint mismatch 25 to 30 mm of 35 mm) |
| front `shirt` 1:500 | sweep along v longer than the elevation chest (36 mm) | `front-sweep` |
| plan `fingers-l-skin` 1:50 | a tracer plate 4 mm thick; the right hand's is 29 mm (37 mm in three clip instants) | plate given the span of the right hand, mirrored |

Silhouette guard: every value as before except plan 1:50 IoU 0.9795 → 0.9778 and front 1:500 48.3 → 55.6 mm (limit 75).

### `kneeling-gardener` (2 ops, second repair)

The 1:100 `shirt` of the torso was a prism swept along v in front and plan whose span was not the thickness of the elevation outline (91 mm in the plan, 59 mm in the front). First repair, `front-sweep` and the same prism in the plan: clean on the subjects of the first gate, but the proof over every subject found `sync-bone-height` ×4, 41 mm in `reach-to-trug` (the elevation shirt is cut askew at the chest, a prism of the front outline with a linear taper has a footprint mismatch of 36.0 mm of 36.7 mm; the matrix of that state: 72 fail cells, 42 of them this finding). Second repair, kept: `extrude` with `"fit": true`, the elevation outline swept laterally with the taper of the front outline (span ±0.155 m at the waist, ±0.118 m at the shoulders), the same body in the plan. Gate over every subject on the copy: 0 hard findings.

### `leaning-person` (42 ops)

| Shape | Defect | Repair |
|---|---|---|
| elevation `hand-l`, 4 scales; plan `hand-r` and `phone` 1:200, 1:500 | lateral spans of `sweep: "w"` written relative to the bone (`z0: -0.028`) instead of in asset coordinates (the bone lies at z ±0.175): the part swung 0.175 m beside its hand when the arm turned (186 mm, and the grips `grip-l`, `grip-r`, `phone-ear` up to 157 mm off their hand: 476 anchor findings) | spans moved by the rest z of the bone; no anchor was touched |
| elevation `hand-l` 1:50 | one mitten outline with straight fingers while front and plan draw the fifteen finger bones (48 mm when the fingers curl in `leaning-arms-folded`) | the outline cut at the knuckles; the fifteen finger solids of the front copied into the elevation |
| plan `thigh-l`, `thigh-r`, `neck`, 4 scales | flat sheets (48, 43, 37 mm) | `plan-revolve` |
| plan `torso` 1:200, 1:500 | flat outline drawn for the lean of the default pose, 42 mm deeper than the trunk is thick | `plan-depth` (outline scaled to the rest depth of the elevation twin) and `plan-core` (see 3) |
| front `upper-arm-r` 1:50, plan `hand-l-skin` 1:50 | tracer plates 5 mm thick | plates given depth (30 mm; the length of the hand) |

Silhouette guard: 1:50 plan 14.3 → 15.0 mm (limit 15.0), IoU 0.9852 → 0.9719; 1:200 plan 40.5 → 28.2 mm; 1:500 plan 42.0 → 30.5 mm; the rest unchanged.

### `mobility-scooter-user` (8 ops, anchors split)

- **Anchors (D45).** `patch-rear` and `patch-front` were one lumped contact per axle at z 0, 60 mm off the tyres in the front view in every subject (978 findings). Split into `patch-rear-l`, `patch-rear-r`, `patch-front-l`, `patch-front-r` at z ±0.235 (the tyres are drawn at 0.195 to 0.275); the contacts of the four clips and four poses and the first structural assumption follow (`anchors/mobility-scooter-user.json`, 19 changes). No fixture names these anchors.
- front `glove-r` 1:50, 1:100, 1:200 and `fingers-r` 1:100, 1:200: tracer plates 0.8 mm thick whose outline was the front view of the default pose laid into the side plane of the hand (364 mm long for a hand of 157 mm; 72 to 97 mm lateral in `greeting>reaching-into-basket`): replaced by the elevation outline over the width the plan draws.
- `jacket` 1:500: `front-sweep`, `plan-depth`, `plan-core`.

Silhouette guard: unchanged in all twelve values within 0.003 IoU.

### `painter-at-easel` (2 ops)

- plan `sleeve-r` 1:50: tracer plate 4.4 mm thick (59 mm in `stepping-back-to-look`). The revolved elevation sleeve in its place fails the guard (plan 36.1 mm against 15); the plate got the depth −0.027 … 0.022 m: finding 33 mm (tolerance 37.3), plan guard 5.6 → 13.3 mm.
- `hand-r` 1:200: front and plan drew another profile than the elevation (64 and 69 mm). The elevation solid in all three views fails the plan guard (83.4 mm against 60), the front solid in the elevation fails there (66.0 mm). Kept: the elevation profile in front and plan with their own lateral spans (front −0.232 … −0.148, plan −0.222 … −0.148): findings 20 to 31 mm, plan guard 39.0 → 57.7 mm (limit 60). Narrow on both sides; stated in 6.

### `parent-carrying-toddler` (25 ops)

Thighs: the plan was a transverse profile swept along the bone without taper (57 mm at the knee): `plan-revolve` rebuilt, `front-sweep` refit. Boots 1:200, 1:500: front a transverse profile swept along the foot, plan a sweep along v (71 and 53 mm): `extrude` (the elevation outline over the width of the boot). Plan `sleeve-upper-r`, 4 scales: flat, `plan-revolve`. Anchor `toddler-crown` (emitter): at x 0.218, the top of the pompom that only 1:50 to 1:200 draw, 51 mm above the hat at 1:500 (246 findings): moved to x 0.19, inside the pompom at the fine scales and 22 mm above the hat at 1:500.

### `parent-with-child-on-shoulders` (18 ops)

Plan thighs, 4 scales: flat, `plan-revolve`. Boots 1:200, 1:500 and front `toddler-suit` 1:500: flat on bones that point at the viewer, `extrude`. Plan `toddler-suit` 1:500: `plan-core` (the prism alone drops the plan IoU to 0.9159). The heel and sole anchors left their boots only because the boots did not follow: no anchor was touched. Silhouette guard 1:500 plan: IoU 0.9349 → 0.9214 (limit 0.92), 30.2 → 44.9 mm (limit 75): at the limit, stated in 6.

### `parent-with-stroller` (19 ops, anchors split)

- **Anchors (D45).** `wheel-rear` and `wheel-front` had no `z` and sat on the stroller bone, which hangs from the right hand at z −0.193: 0.193 m beside the centre of the stroller and up to 99 mm off every wheel in the kerb tilt. Split into `wheel-rear-l`, `-r` (z 0.47 and −0.084 from the bone, the tyres at ±0.277) and `wheel-front-l`, `-r` (0.408, −0.022; tyres at ±0.215); the contacts of clips and poses follow (34 changes). The bones `wheel-rear` and `wheel-front` of the same name are untouched.
- `hood` 1:50 to 1:200: flat in front and plan on a bone that folds 100 degrees (133 mm): `extrude` (the hood is the side outline over its width).
- `child-body` 1:200, 1:500: `extrude` front and plan; `torso` 1:200, 1:500: `front-sweep`, `plan-depth`, `plan-core`; plan `hand-l` 1:500: `plan-revolve` refit.
- front `hub-rear-r` 1:50: the only wheel part left flat (40 mm below the ground in `push-toe-off-l`): the solid of the left hub, mirrored.
- front 1:500: both rear wheels were one tracer plate `wheel-rear-rubber` that did not tilt with the stroller (the split anchors 92 mm off): replaced by the two tyre discs of the elevation.

Silhouette guard: 1:500 front IoU 0.9324 → 0.9221 (limit 0.92), 53.2 → 62.8 mm (limit 75); 1:50 front 14.3 → 11.3 mm; the rest unchanged. At the limit, stated in 6.

### `person-carrying-ladder` (22 ops)

Plan thighs and shanks: flat (up to 285 mm at 1:500), `plan-revolve`. Shoes 1:200: `extrude` front and plan. The plan drew only the upper rail of the ladder (the lower one lies beneath it while the ladder is level); tipped up, the lower rail reaches 69 mm further ahead: `rail-lower`, `foot-lower`, `cap-lower` added to the plan of 1:50 and 1:100 as the solids of the elevation. Plan `sleeve-l` 1:100, 1:200: tracer plate 39 mm thick (68 mm): depth 0.08 … 0.20 m along the arm (the full length of the arm widens the plan of the default pose to 0.526 m against the declared 0.511). Silhouette guard: plan 1:200 60.0 → 45.0 mm, 1:500 85.8 → 60.1 mm, front 1:200 45.0 → 22.5 mm.

### The three walkers: `nurse-walking`, `office-worker-carrying-laptop`, `patient-with-iv-pole`

Every limb was a flat shape in front and plan, the plan drawn for the stride of the default pose (thigh 438 mm long in the plan of a vertical bone). Two steps, in this order:

1. **Prisms and bodies of revolution (first fixer, 2026-11-25 00:15; `ops/<id>.json`, `walker-ops.py`, `greedy.py`).** Written for `nurse-walking` (97 ops) and `office-worker-carrying-laptop` (86 ops); the 104 ops of `patient-with-iv-pole` were never written and are not part of its chain. This step left hard findings (gate 1528, 200 and, untouched, 4194): the silhouette guard refuses every prism and every circular body of revolution for hands, shoes at 1:50 and the nurse's sleeves (request of 11:26 in `requests.md`).
2. **Loft shapes (D54; third fixer built `limbs.ts` and the plans 16:10 to 17:56, fourth fixer finished and wrote them 19:18 to 19:5x).** `plans/<id>.lofts.json` names per scale and bone the body solids of the band; `limbs.ts` replaces the drawn shape of the same part in the named views by a `loft` with the sections of the solid (one loft solid) or with sections fitted to several solids (palm with thenar and hypothenar, a finger as a path along its three capsules, sole with heel). Elevation, front and plan are then projections of one body, which is what the matrix asks for.

| Asset | Lofts | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|---|
| `nurse-walking` | 83 | 32: sleeves, forearms, wrists, palms, 10 fingers, thighs, shanks, socks, shoes (soles in elevation and front only) | 26 | 14, front and plan only (lofts in the elevation fail the guard there: 71.5 mm against 60) | 11 |
| `office-worker-carrying-laptop` | 65 | 25: left arm and hand with 5 fingers, right sleeve (elevation and front; the plan keeps the arm that carries the laptop), legs, socks, shoes | 18 | 12 | 10 |
| `patient-with-iv-pole` | 93 | 37: both arms and hands with 10 fingers, legs, ankles, shoes, neck, collar, `coat-core` beneath the plan outline of the coat | 29 | 15 | 12 |

Decisions of the fourth fixer: the plan of `office-worker-carrying-laptop` of 17:50 failed the guard at 1:50 (elevation 48.8 mm against 15: `MH-6/where.ts` found a gap of 44 mm at the ankle, the lofted shank ends where the solid `shank` ends and the band has a solid `sock` that no shape drew; the old trouser outline had covered it). Added: `sock-l`, `sock-r` at 1:50 as lofts of their own in all three views, as `nurse-walking` and `patient-with-iv-pole` already have them. Elevation 9.4 mm after.

Silhouette guard of the written files (limit: IoU 0.92 and twice the smallest feature), elevation, front, plan:

| Asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| `nurse-walking` | 0.9968/9.4, 0.9982/3.8, 0.9905/8.4 mm | 0.9957/5.3, 0.9982/3.8, 0.9950/4.7 | 0.9879/43.3, 0.9960/6.8, 0.9956/4.6 | 0.9726/128.4, 0.9908/50.9, 0.9594/106.3 |
| `patient-with-iv-pole` | 0.9909/14.3, 0.9926/14.3, 0.9833/9.4 | 0.9917/16.8, 0.9868/16.8, 0.9630/17.6 | 0.9415/30.0, 0.9887/7.5, 0.9597/17.7 | 0.9823/24.1, 0.9862/16.0, 0.9690/21.1 |
| `office-worker-carrying-laptop` | 0.9985/9.4, 0.9985/13.5, 0.9800/13.7 | 0.9973/3.8, 0.9963/15.9, 0.9630/23.4 | 0.9922/29.8, 0.9961/16.1, 0.9737/39.2 | 0.9876/29.8, 0.9940/20.4, 0.9668/41.0 |

Sheets opened (side, front, plan; line and solid): 1:50 `contact-left` before and after for all three, 1:50 `standing` for the office worker. Arms, hands with fingers, legs and shoes read as before or better (the fingers of the nurse's swinging hand are drawn in all three views, the clipped plan hands are gone); the sock shows as a short band between trouser hem and shoe.

## 3. Tools added (`🧰️matrix/MH-2/`)

| Script | Purpose |
|---|---|
| `fit2.ts` | `MR/fit.ts` with further ops: `plan-prism` (plan outline swept along an upright bone), `plan-core` and `plan-depth` (below), `elevation-extrude` (a flat elevation shape as a solid over the lateral span of its twin: the op the MR report missed for twisted heads; written, not needed by this batch), `plan-revolve` with `"radius": "front"`, `copy` with its own span per view, `extrude` with `"fit"`; two corrections of `extrude` (a shape swept along u kept its solid; a flat front on a leaning bone got twice its span) |
| `gate2.ts` | `MR/gate.ts` with `--only <regex>` (the subjects a finding named) and `--json`: the trial loop in one to four minutes |
| `pick.py`, `facts.py`, `codes.py`, `opsum.py`, `dump.py`, `ext.ts`, `who.ts` | which ops the gate asks for; the findings beside their shapes; which shapes make the extent of a view in one subject |
| `walker-ops.py`, `greedy.py`, `subset.sh` | the ops of the walkers and their choice against the silhouette guard |
| `split-anchors.py`, `anchors/*.json` | D45: one anchor per contact, references in clips and poses repeated |
| `try.sh`, `quick.sh`, `apply.sh`, `look-file.sh`, `sheetdiff.py`, `gate-all.sh`, `round3.sh`, `q.sh` | trial on a copy, writing into the catalog, sheets with pixel difference, the proof round; `q.sh` sends every tool process through `repo capacity run` |

**`plan-core`, stated plainly.** For a trunk in a long coat no single solid reproduces the rounded plan outline (a prism of the plan outline over the skew coat: footprint mismatch 64 to 132 mm; the elevation outline swept laterally: plan IoU 0.90 to 0.916 against 0.92). `plan-core` keeps the flat outline and adds beneath it a second plan shape, the elevation outline swept laterally over ±0.09 to ±0.15 m, so the plan reaches as far as the side view when the trunk pitches. The bone then has two plan shapes, and the matrix compares it in the reported tier (`group-drift`), no longer in the hard per-bone tier; the whole-figure `sync-forward` still guards the extent. Used for: `leaning-person` torso 1:200, 1:500; `mobility-scooter-user` jacket 1:500; `parent-with-stroller` torso 1:200, 1:500; `parent-with-child-on-shoulders` toddler suit 1:500; `patient-with-iv-pole` coat 1:100 to 1:500. The pattern exists in the catalog since the silhouette rounds (`jogger` `shirt-core`). If the owner of the ticket wants these bones in the hard tier, they need a plan redrawn from the volume.

