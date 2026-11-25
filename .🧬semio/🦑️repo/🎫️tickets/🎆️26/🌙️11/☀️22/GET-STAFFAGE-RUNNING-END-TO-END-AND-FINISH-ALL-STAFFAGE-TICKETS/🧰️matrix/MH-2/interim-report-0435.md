# 🧍️ Multiview Fixer MH-2: Twelve People — INTERIM, not final

> **State at the takeover, 2026-11-25 02:25 (second fixer; measured, supersedes the tables below where they differ).** The first fixer stopped at about 00:20. Catalog files: eleven are written with the ops of section 2 (byte-identical to the trial copies); `patient-with-iv-pole` is NOT written (identical to the backup of 2026-11-24 13:39). Proven after 23:00 over every subject: only `kneeling-gardener` (gate 0, scan clean, 19 tests passed, check `✓`; no matrix yet). `nurse-walking` as written: gate over every subject **1,528** hard findings (not 316), scan 61 hard; tests and check pass. `office-worker-carrying-laptop` as written: gate **202** hard (not 16; among them `evaluate-error` ×2 at 1:50), scan 11 hard; tests passed, the check CLI was in flight and counts as not run. The "0 (trial)" entries of section 1 are trials on a subset of the subjects, not proofs; the proof round of the other eight assets is running now. `jogger`'s proof of 22:00 to 23:28 fell into the hour in which the system killed or suspended runs and is being repeated. The placeholders `PROOF-…` below are unfilled.
>
> **04:35.** `jogger` repeated through the queue: gate over every subject 0 hard, scan clean, 19 tests passed (5 files, `asset-power` has no case); its check CLI has been waiting for a 5,000 MB grant since 03:03. The queue grants a 1,000 to 2,500 MB run after 5 to 40 minutes and has 7,000 MB requests waiting for two hours. Walkers: `ops/<id>.json` of the three were edited (the `front-sweep` of the hands at 1:50 and 1:100 taken out, originals in `ops-v1/`); a new tool `vol.ts` builds front and plan solids from the volume and is being tried on copies (`🗑️generated/matrix/MH-2/lab/`). The catalog files of the three walkers are unchanged since the takeover: `nurse-walking` and `office-worker-carrying-laptop` are in the half-repaired state above (not worse than the backup in any test that ran, but with `evaluate-error` ×2 in `office-worker-carrying-laptop` 1:50 front `shirt-cuff-l`), `patient-with-iv-pole` is the untouched backup.

Fixer report of 2026-11-24/25. Batch `🧰️matrix/batches/MH-2.tsv` (12 people). Paths as in `📋️multiview-brief.md` (`S`, `T`, `U`); scripts in `U/🧰️matrix/MH-2/`; the recipe and the base tools are those of `multiview-MR.md` (`U/🧰️matrix/MR/`, not edited).

## 1. Result

"Gate" is `MR/gate.ts <id> --twins`: the matrix's own `geometryFindings` over every pose, clip instant and transition at every authored scale. "Before" was measured on 2026-11-24 13:40 to 17:16 on the untouched files (after the two shared-code changes: anchors against descendant bones, hard tier for twins of one id). "Trial" is the same gate on the repaired copy, restricted to the subjects the first gate named (`gate2.ts --only`); "proof" is the gate over every subject on the catalog file.

| Asset | Hard findings before | After | State |
|---|---|---|---|
| `jogger` | 91: `sync-bone-forward` ×87 (74 mm), `sync-bone-height` ×4 (36 mm) | **0** (proof) | clean, matrix 252 cells, 0 fail |
| `kneeling-gardener` | 144: `sync-bone-forward` ×72 (91 mm), `sync-bone-height` ×72 (59 mm) | **0** (second repair, see 2) | PROOF-kneeling-gardener |
| `leaning-person` | 1,038: `anchor-landmark` ×476 (157 mm), `sync-bone-forward` ×304 (113 mm), `sync-bone-height` ×154 (186 mm), `sync-bone-lateral` ×37, `sync-forward` ×59, `sync-lateral` ×8 | **0** (trial) | PROOF-leaning-person |
| `mobility-scooter-user` | 1,053: `anchor-landmark` ×978 (60 mm), `sync-bone-forward` ×54, `sync-bone-height` ×11, `sync-lateral` ×10 (97 mm) | **0** (trial) | PROOF-mobility-scooter-user |
| `painter-at-easel` | 139: `sync-bone-forward` ×92 (64 mm), `sync-bone-height` ×47 (69 mm) | **0** (trial) | PROOF-painter-at-easel |
| `parent-carrying-toddler` | 826: `anchor-landmark` ×246 (51 mm), `sync-bone-forward` ×527 (71 mm), `sync-bone-height` ×53 | **0** (trial) | PROOF-parent-carrying-toddler |
| `parent-with-child-on-shoulders` | 780: `anchor-landmark` ×42, `sync-bone-forward` ×539 (100 mm), `sync-bone-height` ×157 (129 mm), `sync-forward` ×27, `sync-height` ×15 | **0** (trial) | PROOF-parent-with-child-on-shoulders |
| `parent-with-stroller` | 1,207: `anchor-landmark` ×62 (99 mm), `sync-bone-forward` ×285 (116 mm), `sync-bone-height` ×852 (133 mm), `sync-height` ×8 | **0** (trial) | PROOF-parent-with-stroller |
| `person-carrying-ladder` | 847: `sync-bone-forward` ×722 (285 mm), `sync-bone-height` ×71, `sync-bone-lateral` ×40, `sync-forward` ×14 | **0** (trial) | PROOF-person-carrying-ladder |
| `office-worker-carrying-laptop` | 4,400: `anchor-landmark` ×142, `sync-bone-forward` ×2,717 (183 mm), `sync-bone-height` ×1,251 (124 mm), `sync-forward` ×196, `sync-height` ×94 | **16** (trial): left hand in the plan, see 4 | PROOF-office-worker-carrying-laptop |
| `patient-with-iv-pole` | 4,194: `anchor-landmark` ×136, `sync-bone-forward` ×2,706 (179 mm), `sync-bone-height` ×1,212 (121 mm), `sync-forward` ×62, `sync-height` ×78 | **13** (trial): left hand in the plan, see 4 | PROOF-patient-with-iv-pole |
| `nurse-walking` | 6,963: `anchor-landmark` ×56, `sync-bone-forward` ×3,619 (495 mm), `sync-bone-height` ×2,303 (160 mm), `sync-forward` ×540 (344 mm), `sync-height` ×445 (115 mm) | **316** (trial): arms and hands in the plan, shoes at 1:50, see 4 | PROOF-nurse-walking |

Sum of the hard findings of the gate: 21,682 before, 345 after (three walkers).

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

## 4. Open

**Three walkers keep hard findings** (gate on the repaired copies, subjects of the first gate; numbers per code):

| Asset | Left | Cause |
|---|---|---|
| `office-worker-carrying-laptop` | 16: `sync-bone-forward` `hand-l` ×8 (74 mm) at 1:50 and 1:100, `sync-forward` ×8 (58 mm) | plan of the left hand |
| `patient-with-iv-pole` | 13: `sync-bone-forward` `hand-l` ×8 (68 mm) at 1:50 and 1:100, `sync-forward` ×5 (57 mm, coat at 1:50) | plan of the left hand; the coat core at 1:50 fails the guard (plan 22.7 mm against 15) |
| `nurse-walking` | 316: `sync-bone-forward` ×230 (hands 74 to 117 mm at 1:50 and 1:100; arms 71 to 102 mm at 1:50; forearms, arms and left hand at 1:500, forearm-r 495 mm), `sync-forward` ×65 (82 to 344 mm), `sync-height` ×21 (95 mm, shoes at 1:50) | below |

What the silhouette guard refused, each tried on the backup with the groups before it (`greedy.py`, `subset.sh`): the plan of the hands at 1:50 and 1:100 as the elevation outline over the hand width (43 to 47 mm against 15 and 30), as the prism of the front (43 to 44 mm), as a body of revolution (52 mm); the shoes at 1:50 (front 18.8 to 20.2 mm, plan 18.8 to 23.5 mm against 15); the nurse's sleeves at 1:50 (front 23.5, plan 22.9 mm) and her arms and hands in the plan at 1:500 (IoU 0.903 to 0.920 against 0.92). These parts need their plan remade from the volume solids of the band in one pass with the guard (`U/requests.md`, "multiview MH-2 — three walkers").

**Matrix defect, not an asset defect:** `blank-shapes` on the symbol alternative `gardener-block` of `kneeling-gardener` (30 cells): the symbol is an unfilled outline, the matrix looks only for fills (`U/requests.md`, "multiview MH-2 — `blank-shapes`"). These 30 cells stay `fail` until `T` is repaired.

PROOF-OPEN

## 5. Proof

PROOF-TABLE

## 6. Limits of this work

- Four silhouette values sit at their limit after the repair: `leaning-person` 1:50 plan 15.0 mm (limit 15.0); `painter-at-easel` 1:200 plan 57.7 mm (60) with the hand finding at 31 mm of 37.3; `parent-with-child-on-shoulders` 1:500 plan IoU 0.9214 and `parent-with-stroller` 1:500 front IoU 0.9221 (0.92).
- `plan-core` moves five trunks into the reported tier (section 3).
- Sheets: opened before and after as a difference image: `jogger` 1:50, `leaning-person` 1:50 and 1:200, `painter-at-easel` 1:50, and the plan of `nurse-walking` 1:50 with revolved arms (the hands became round blobs: that variant was not kept). The other sheets were compared by pixel count only (section 5); at 1:200 and 1:500 the product sheet draws the figure 4 to 9 mm high and shows nothing to judge.
- The memory incident of 23:05: from then on every tool process ran through `repo capacity run` (`q.sh`). The first proof round was stopped by me at that moment (its shells, not the running matrix): `jogger`'s matrix was in flight and finished; `kneeling-gardener`'s first proof (old repair) is superseded. Repeated through the queue: see section 5.

## 7. Files

Updated (all under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/`): `jogger/🔣️.json`, `kneeling-gardener/🔣️.json`, `leaning-person/🔣️.json`, `mobility-scooter-user/🔣️.json`, `nurse-walking/🔣️.json`, `office-worker-carrying-laptop/🔣️.json`, `painter-at-easel/🔣️.json`, `parent-carrying-toddler/🔣️.json`, `parent-with-child-on-shoulders/🔣️.json`, `parent-with-stroller/🔣️.json`, `patient-with-iv-pole/🔣️.json`, `person-carrying-ladder/🔣️.json`.

Updated: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (two entries `multiview MH-2`).

Created (all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`): `multiview-MH-2.md`; `🧰️matrix/MH-2/` with `fit2.ts`, `gate2.ts`, `ext.ts`, `who.ts`, `pick.py`, `facts.py`, `codes.py`, `opsum.py`, `dump.py`, `walker-ops.py`, `greedy.py`, `split-anchors.py`, `sheetdiff.py`, `try.sh`, `quick.sh`, `subset.sh`, `apply.sh`, `look-file.sh`, `gate-all.sh`, `round3.sh`, `q.sh`, `ops/<id>.json` (12), `anchors/mobility-scooter-user.json`, `anchors/parent-with-stroller.json`.

Removed: nothing in the repository. `🗑️generated/matrix/MH-2`: REMOVED-STATE.

Manifest: not edited. Declared dimensions: no asset's extent left its tolerance; the anchors of two assets changed ids (`patch-*`, `wheel-*`), which the coordinator's sync should see.
