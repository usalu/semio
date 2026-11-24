# 🧍️ Multiview Fixer MR: the Two Reference People and the Method for the Other 71

Fixer report of 2026-11-24. Batch `🧰️matrix/batches/MR.tsv`: `standing-adult`, `walking-commuter` (cause F of `multiview-diagnosis.md` 5.6). Paths as in `📋️multiview-brief.md` (`S`, `T`, `U`); scripts in `U/🧰️matrix/MR/`.

## 1. Result

| Asset | Hard findings before (gate, every pose, clip instant and transition) | After | Matrix before | Matrix after |
|---|---|---|---|---|
| `standing-adult` | 505: `sync-bone-height` 460 (neck 1:50 ×114 56 mm, 1:100 ×113 51 mm, 1:500 ×11 35 mm; thigh-l and thigh-r 1:500 ×111 each 44 mm), `sync-bone-forward` 45 (arm-l 1:500, 149 mm in `hands-warming`) | **0** | 252 cells: 112 fail (98 `sync-bone-height`, 28 `sync-bone-forward`), 56 ok, 42 range-end, 42 group-drift | 252 cells: **0 fail**, 70 ok, 42 range-end, 140 group-drift (706 s) |
| `walking-commuter` | 402: `sync-bone-lateral` thigh-l and thigh-r 1:50 ×201 each, 58 mm | **0** | not run before (the scan had the same single code) | 252 cells: **0 fail**, 224 ok, 28 group-drift (1,278 s) |

Both assets: no anchor, band-copy, in-band, paper or cue finding before or after (official scan `T/🔎️scan.ts`: `1 clean, 0 with a hard finding`).

## 2. What was wrong and what was changed

Every finding was one shape of a bone whose implied solid was not the solid of its elevation twin. Nothing else was touched: no elevation shape, no volume, no anchor, no pose.

### `standing-adult` (`ops/standing-adult.json`, 12 ops)

| Shape | Defect | Repair |
|---|---|---|
| front `neck` 1:50, 1:100, `sil-neck` 1:500 | `extruded` sweep v prism longer than the elevation neck (u -0.063 … 0.127 against -0.041 … 0.116) and its v-span not the elevation thickness | `front-sweep`: outline cut to the u-range of the elevation twin, `z0`, `z1`, `zTip` fitted to the elevation outline |
| front `sil-thigh-l`, `sil-thigh-r` 1:500 | prism 42 mm longer at the hip than the elevation thigh (hidden inside the pelvis) | `front-sweep` |
| front `thigh-l`, `thigh-r` 1:50 and 1:100, `shank-l`, `shank-r` 1:100 | the same defect (53 mm, 35 mm), only reported today because the bone carries seam shapes too (`twin-height` of the gate) | `front-sweep`, so the four bands are built alike |
| plan `sil-arm-l` 1:500 | flat blob at the shoulder, does not follow the lifted arm | `plan-revolve`: the revolved elevation twin copied (as arm-r and the other bands already are) |

Silhouette guard before → after (IoU / largest boundary distance): 1:50 front 0.9704/14.6 → 0.9705/14.6; 1:100 front 0.9517/30.0 → 0.9526/22.5 mm; 1:500 plan 0.9464/19.4 → 0.9380/19.4 (gate 0.92); every other value identical.

### `walking-commuter` (`ops/walking-commuter.json`, 2 ops)

| Shape | Defect | Repair |
|---|---|---|
| front `thigh-l`, `thigh-r` 1:50 | flat block in asset coordinates, 272 mm wide, reaching 60 mm across the centre plane behind the coat (a hand shape of the silhouette round); it did not move with the walking leg and was 58 mm wider than the revolved plan thigh | `front-sweep` with `"lateral": "plan"`: the outline is re-expressed in the bone frame, cut to the lateral range of the plan twin and swept along v with the span of the elevation thigh |

Copying the revolved plan thigh into the front (as 1:100 has it) was tried first and not kept: front boundary distance 15.1 mm against the gate of 15.0. Silhouette guard after the kept repair: identical to before in all twelve values (1:50 front 0.9678/15.0 mm).

## 3. Proof (each run to the end in this session, after the last change)

| Check | `standing-adult` | `walking-commuter` |
|---|---|---|
| `gate.ts <id> --twins` (poses, clip instants, transitions; 114+113+116+111 and 201+201+202+200 subjects) | hard findings: 0 | hard findings: 0 |
| `T/🔎️scan.ts --language en <id>` | clean | clean, 8 reported (group drift) |
| `T/📊️matrix.ts --language en --jobs 1 --fresh --assets <id>` | 252 cells: **0 fail**, 70 ok, 42 range-end, 140 group-drift (706 s) | 252 cells: **0 fail**, 224 ok, 28 group-drift (1,278 s) |
| `validateAsset` | 0 issues (before 0) | 0 issues (before 0) |
| check CLI `--asset <id>` | `✓ standing-adult (1/1)` | `✓ walking-commuter (1/1)` |
| `🧊️catalog-silhouette` | 1 passed | 1 passed |
| `🧊️three-views` (test and sweep) | 2 passed; sweep 0 findings | 2 passed; sweep 0 findings |
| `🌡️catalog-physics` | 3 passed | 3 passed |
| `🫁️breathing-catalog` | 3 passed | 3 passed |
| `✅️asset-validation` (sample figure, manifest cases) | 10 passed | 10 passed |
| `⚡️asset-power` | 2 passed | no case for this asset in the fixture (as before) |
| every authored scale prints | sheets 1:50, 1:100, 1:500 drawn | sheet 1:50 drawn (the only changed band) |
| second run of `fit.ts` | 0 of 12 ops change something | 0 of 2 |

Sheets opened and compared before and after: `standing-adult` 1:50 and 1:500 `relaxed`, 1:50 `hands-warming`; `walking-commuter` 1:50 `standing` and `contact-left`. No visible difference in the rest pose (what was cut is hidden inside the pelvis, the torso or the coat). Not opened: `standing-adult` 1:100 (rendered, same ops as 1:50), 1:500 `hands-warming` (rendered).

## 4. The method, as a recipe for the other people

### 4.1 The rule

The matrix compares, per bone, the extent of a shape with the extent of its twin of the same id in another view, in every pose, clip instant and transition. They agree everywhere exactly when the three shapes are projections of one solid. So for every part on a bone that a pose or clip turns (itself or an ancestor): **the elevation outline is the profile, and the front and plan shapes are solids built from it.** The bone frame is `u` along the bone, `v` its normal (the thickness the elevation shows), `w` lateral.

| Part | Elevation | Front | Plan | Op |
|---|---|---|---|---|
| Arm, forearm, sleeve: bones that poses twist and spread | `revolved` about `u`, `mirror` | the same profile and solid | the same profile and solid | `copy` from elevation (or `plan-revolve`, which copies a revolved twin) |
| Thigh, shank, neck: bones that swing in the sagittal plane | flat outline in (u, v) | `extruded`, `sweep: "v"`: the front outline in (u, w), u-range of the elevation outline, `z0`, `z1`, `zTip` = its thickness at the joint and at the tip | `revolved` about `u`: radius profile over the same u-range, axis offset fitted to the elevation thickness and the front width | `front-sweep`, `plan-revolve` |
| Foot, toe, shoe: bones that point at the front viewer | flat outline | `extruded`, `sweep: "w"`: the elevation outline over the width of the shoe | the same, or left flat | `extrude` (`"view": "front"`), review the sheet |
| A part whose front is a plain taper | flat outline | `extruded`, `sweep: "w"` with the elevation outline: exact at every pose | | `extrude` |
| Head | flat outlines | `extruded` sweep v per shape (already so) | flat (it only nods; it is not in the hard tier because the bone has several shapes) | none unless the gate reports it |
| Trunk, coat, bag: several shapes per bone | | | | reported tier only; no op |

Why the fit and not a copy of numbers: a prism swept along v has the (u, v) footprint `u ∈ [u0, u1]`, `v ∈ [z0, z1]` at the joint running linearly to `zTip` at the bone length and constant beyond both. Heights agree at every sagittal angle exactly when this footprint and the elevation outline have the same support function in every direction. `fit.ts` minimises the largest difference over 72 directions and prints it as `footprint mismatch`; it must stay below the tolerance of the asset (2 % of the largest dimension, 34 to 36 mm for an adult), with room for breathing: the two reference people end at 11 to 29 mm.

### 4.2 Rest-pose extents

Where the views differ already at rest (cause G), the cause in both reference people was hidden length: the front or plan part runs further into the neighbouring part than the elevation part does (thigh into the pelvis, neck into the torso). `front-sweep` and `plan-revolve` cut the outline at the two ends of the elevation twin (`clipStretch`), which removes only hidden length and leaves the visible outline where it was; an end that falls short is stretched to it. The elevation is the reference because the volume agrees with it (the thigh loft of `standing-adult` starts at u = -0.030 like the elevation, the front prism started at -0.082). If the silhouette guard says the elevation is the wrong one, repair the elevation outline by hand first, then run the ops.

### 4.3 Anchors

Neither reference person had an anchor finding, with or without the descendant rule of the suite, so no anchor was changed and the binding they show is the pattern: contacts `heel-*`, `ball-*` on `foot-*`; emitters `mouth`, `nostrils`, `crown` and the attachment `ear` on `head`; a grip on the hand that draws it with its real `z` (`strap-grip` on `hand-r`, z 0.024). For a person with anchor findings: `fit.ts` has the op `{ "op": "anchor", "id", "set": { "bone", "x", "y", "z" } }`; bind the anchor to the bone whose shape draws its place, give it the lateral `z` of that place, and repair the drawing first when the anchor only left its shape because the shape did not follow the bone (the gate then drops the finding by itself). Not exercised in this batch.

### 4.4 Steps for one person (`<B>` is your batch, `<id>` the asset)

```sh
source U/🧰️matrix/MR/env.sh            # B = pinned bun, S, T, U, M = this folder; BATCH=<B> sets W and GEN to your folders
cp "$S/🗂️catalog/👤️humans/<id>/🔣️.json" "$GEN/backup/<id>.json"
cd "$T" && "$B" "$M/gate.ts" <id> --twins                              # 2 to 5 min: hard findings per scale, bone and worst subject
"$B" "$M/suggest.ts" <id> > "$W/ops/<id>.json"                          # proposals from the document; keep the ops of the bones the gate names
"$B" "$M/fit.ts" <id> --ops "$W/ops/<id>.json" --out "$GEN/try/<id>.json"   # dry run on a copy; read the mismatch and REFUSED lines
"$B" "$M/gate.ts" "$GEN/try/<id>.json" --twins                         # the copy must be clean
(cd "$S/📦️packages/🟦️typescript" && "$B" "$M/measure.ts" "$GEN/backup/<id>.json" "$GEN/try/<id>.json")   # silhouette, validate, sweep: before and after
"$B" "$M/fit.ts" <id> --ops "$W/ops/<id>.json" --write                  # refuses when the file changed on disk since it was read
"$B" "$M/fit.ts" <id> --ops "$W/ops/<id>.json"                          # must report "0 changed something"
TAG=before "$M/look.sh" <id> 50 500; "$M/look.sh" <id> 50 500; POSE=<pose> "$M/look.sh" <id> 50   # open the sheets
MATRIX=1 "$M/prove.sh" <id>                                             # every proof line of the brief, logs in $GEN/proof
python3 "$M/cells.py" "$GEN/matrix-<id>"                                # cells by status and failing check
```

`gate.ts`, `fit.ts`, `suggest.ts`, `measure.ts` take an asset id of any category or a path to an asset document; `look.sh`, `vt.sh`, `prove.sh` take an id. `fit.ts` is the last step of the chain of an asset: run it after the generator and after the silhouette ops of the asset (`U/🧰️silhouette/<batch>/ops/<id>.json`); it reads the elevation twin anew each time and rebuilds only the shapes its ops name.

### 4.5 The scripts

| Script | Purpose |
|---|---|
| `env.sh` | paths; `BATCH=<B>` points `W` and `GEN` at another fixer's folders while `M` stays this folder |
| `lib.ts` | finds an asset document by id, reads it, writes it back in its own indentation and refuses when the file changed on disk meanwhile |
| `gate.ts` | the matrix's own `geometryFindings` over every subject of the matrix (poses, clip instants, transitions) at every authored scale, per code the count, worst delta, subject and bones; `--twins` adds every shape against its twin of the same id (finds the defect on bones that carry further shapes); exit 1 on a hard finding |
| `suggest.ts` | proposes ops from the document: which twins are flat, swept or revolved on bones that turn |
| `fit.ts` | the repair: ops `front-sweep`, `plan-revolve`, `extrude`, `copy`, `set`, `remove`, `anchor` (header of the file) |
| `measure.ts` | silhouette guard per scale and view, `validateAsset`, three-views sweep, outside the test runner |
| `look.sh` | product sheet (side, front, plan; line and solid) of a scale in a pose, from the catalog or the backup |
| `vt.sh`, `prove.sh` | one real test with a name filter; the whole proof of one asset |
| `cells.py` | summary of a matrix output folder |
| `ops/standing-adult.json`, `ops/walking-commuter.json` | the repairs of this batch |

### 4.6 What the tool was tried on beyond the batch, and its limits

- `suggest.ts` and `fit.ts` were tried on a copy of `office-worker-carrying-laptop` with ALL 83 proposed ops, unfiltered (the copy is deleted, the catalog file was not touched): 79 applied, 4 refused by name (`plan-revolve` on the level toe bones: the axis of a revolved solid cannot be offset along the normal of a level bone; `extrude` with `"view": "plan"` is the op there). Gate over the poses only: **204 hard findings before, 58 after.** Gone: every limb finding (`sync-bone-forward` shank 165 to 181 mm, `sync-bone-height` shank and foot 53 to 124 mm at the four scales). New, and the reason the proposals marked `"review": true` are not to be applied blindly: `plan-revolve` on the `coat` at 1:200 and 1:500 made the plan trunk a body of revolution (torso plan 58 to 60 mm deeper than the elevation, `dimension-plan-length` 52 and 56 mm). Left: `sync-forward` 40 mm and foot 40 mm at 1:50 and 1:100, heel anchors 37 to 46 mm off the foot. The trial was not measured against the silhouette guard and no sheet of it was opened; it shows what the ops do to a person with flat limbs (cause D), not a finished repair.
- A refit of a fit can still move numbers below 0.3 mm; `fit.ts` keeps the old geometry then. On the 83-op trial the second run still changed 4 revolved profiles and the third none. Run it until it reports `0 changed something`.
- A flat ELEVATION shape on a bone that a pose twists about its own axis (a turned head, a rotated forearm drawn flat) foreshortens in the side view while the plan solid does not. `standing-adult` shows it in the reported tier only (`twin-forward` head and hair 37 to 44 mm in `pointing`, the head has several shapes). The remedy is a solid in the elevation too (arms: `revolved`); no op of `fit.ts` creates it.
- `front-sweep` keeps the front outline; with a skew elevation outline the footprint cannot match better than the clamp of `zTip` allows (neck: 22 to 29 mm of the 34 mm tolerance). Where that is not enough, `extrude` is exact and costs the front its own outline.
- Shapes that share an id across views but draw different parts (`hem-rib`, `hood`, `hood-roll` of `standing-adult`) show large `twin-*` deltas in `gate.ts --twins`; they are different parts, not defects of a solid, and belong to the reported tier.

## 5. Files

Updated:
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/standing-adult/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/walking-commuter/🔣️.json`

Created (all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`):
- `multiview-MR.md`
- `🧰️matrix/MR/env.sh`, `lib.ts`, `gate.ts`, `suggest.ts`, `fit.ts`, `measure.ts`, `look.sh`, `vt.sh`, `prove.sh`, `cells.py`
- `🧰️matrix/MR/ops/standing-adult.json`, `🧰️matrix/MR/ops/walking-commuter.json`

Removed: `🗑️generated/matrix/MR` (backups, trial copies, logs, sheets, matrix folders).

Manifest: no declared dimension changed. `U/requests.md`: nothing added.

## 6. Open

Nothing for the two assets of the batch: scan and gate without a hard finding, matrix without a `fail` cell.

For the people that follow (not part of this batch, stated so nobody assumes it):
- The reported tier of both assets still shows group drift (`standing-adult` torso 88 mm at 1:50 in `arms-crossed`, shank-l 46 mm at 1:500; `walking-commuter` lumbar 444 mm at 1:50, where elevation and front draw different parts of the coat on the bone). It is not a failure of the matrix (D41, D48) and was not repaired.
- `fit.ts` has no op that turns a flat elevation shape into a solid (4.6) and its `anchor` op was not exercised.
- The gate of a person costs 2 to 5 minutes, the matrix 12 to 21 minutes (measured: 706 s and 1,278 s on the shared host).
