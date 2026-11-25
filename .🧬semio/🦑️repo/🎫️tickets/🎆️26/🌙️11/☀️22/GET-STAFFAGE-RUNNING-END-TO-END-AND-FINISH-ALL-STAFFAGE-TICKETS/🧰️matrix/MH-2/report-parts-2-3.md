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

### The three walkers: `nurse-walking` (97 ops), `office-worker-carrying-laptop` (86), `patient-with-iv-pole` (104)

Every limb was a flat shape in front and plan, the plan drawn for the stride of the default pose (thigh 438 mm long in the plan of a vertical bone). `walker-ops.py` proposes per closed limb shape: `front-sweep`; `plan-revolve` with `"radius": "front"` (a body of revolution of the width the front shows: the limb volumes are lofts of elliptic section, and the mean of width and thickness of the MR recipe misses the plan guard, thigh 17.0 mm against 15); `extrude` for shoes. `greedy.py` adds these per scale in nine groups and keeps a group only while the silhouette guard of that scale holds. Kept: legs at every scale; forearms; sleeves except nurse 1:50; shoes at 1:100 to 1:500; hands in the front; the plan of the hands at 1:200 (and 1:500 for two). `patient-with-iv-pole` also got `plan-depth` and `plan-core` for the `coat` at 1:100 to 1:500 (63 mm in `passing-left`). Refused by the guard, with the numbers, in section 4.

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

