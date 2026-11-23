# 🩹 Repair Report: People of HW-01 … HW-09 against the Catalog Suites

Repair author A of `☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE` (`K`), 2026-11-22 22:14 to 2026-11-23 04:20. Scope: the 23 people of HW-01 … HW-09. No manifest edit, no engine, test, schema or fixture edit, no git change. `R` = repo root, `S` = `🎓️teaching/🏛️architecture/🧍️staffage`.

## 1. The claim about the Hausdorff gate and the established humans

**The finisher measured with the real gate and the claim is true: established humans fail the real test today.**

- Real test, run as `📜️script.ts test` runs it (`bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🧊️catalog-silhouette/🟦️.ts -t "(worker-carrying-box|seated-reader|jogger) fills"` in `S/📦️packages/🟦️typescript`, pinned bun), 2026-11-22 22:14: 3 failed of 3.
  - `worker-carrying-box 1:50 elevation Hausdorff: expected 0.10744911586420801 to be less than or equal to 0.015`
  - `seated-reader 1:100 elevation Hausdorff: expected 0.042426406871192854 to be less than or equal to 0.03`
  - `jogger 1:100 front Hausdorff: expected 0.04562071897723664 to be less than or equal to 0.03`
- The finisher's script `K/🧰️scripts/HW-03/measure.ts` calls the same `measureAsset(asset, scale, undefined, undefined, featureOf(scale))` and applies the same three limits (bbox within one feature, IoU at least 0.92, `hausdorffM` at most two features). It is neither stricter nor different; I worked to the real test and used the script as its per-view printout.
- The gate itself is not new: `expect(measured.hausdorffM …).toBeLessThanOrEqual(2 * feature)` is already in the test of commit `3d62b2ef22c` (2026-11-18 13:37), the commit after the guard ticket of 11/17.
- What changed is the established humans, not the test. The catalog file of each commit, measured with today's code (`measure.ts <file> --no-volumes`; side / front / top):

| Asset | `3d62b2ef22c` (11-18) | `5db034ffe53` (11-20) | `b4927d2c311` (11-21) | `01324745f80` (11-22 13:42) |
|---|---|---|---|---|
| `worker-carrying-box` | passes, 1:50 15 / 13 / 14 mm; dimensions 0.68 × 1.815 × 0.5 m | fails 1:50: 107 / 17 / 27 mm; dimensions 0.65 × 1.82 × 0.52 m | fails 1:50: 107 / 17 / 27 mm | fails 1:50: 107 / 20 / 27 mm (today's file) |
| `seated-reader` | 1 failing view (1:500 side IoU 0.917) | passes | passes | fails 1:100 side 42 mm and front 42 mm (today's file) |
| `jogger` | 1 failing view (1:500 top IoU 0.919) | fails 1:50 (17 / 43 / 26 mm) | fails 1:50 and one bbox axis | fails 1:100 front 42 mm and 1:50 (26 / 17 / 24 mm); the working file of 15:54 differs again |

  The base figures were re-authored after the guard ticket (new dimensions, files of 1.0 to 1.7 MB rewritten in each commit of 11-20, 11-21 and 11-22) and have not met the largest-distance gate since the commit of 2026-11-20 (`worker-carrying-box`, `jogger`) or of 2026-11-22 (`seated-reader`). Today's change of `standing-adult` (16:58, revolved sleeves) is one more such rewrite, not the cause: `worker-carrying-box` fails with the identical 107 mm in the three commits before it. I did not edit an established human and did not survey all of them (three of three tested fail). The full `catalog-silhouette` suite is therefore red independently of the wave; the request section in `requests.md` stands, with the correction that the gate was met on 11-18 and lost by later rewrites of the assets.

## 2. Method

- **Chain.** The worker's generator stays the first stage; its output is kept and `K/🧰️scripts/REP-A/apply.py` applies the repair ops `REP-A/ops/<slug>.<stage>-<name>.json` and writes the catalog file (`REP-A/build.sh <slug>` runs the whole chain). The ops are design inputs like the `ops/` of HW-01: replaced, added, changed or removed drawn shapes, changed volume solids, declared region areas. No shipped JSON was edited by hand. Only `schoolchild-with-backpack` and `worker-with-jackhammer` were built again from their generator in this session; for the other 21 the generator output is the catalog file the finisher left at 22:15 (he reports that every generator reproduced it).
- **Silhouette repairs (`REP-A/fit.ts`).** For a failing view the worst boundary point is repaired where it lies: an area the body has and the drawing lacks gets a drawn patch on the bone of the solid there; an area the drawing has and the body lacks is cut out of the shapes that draw it; a displaced part is retraced; hairline slits between fingers are closed. On a bone that poses turn out of the paper the patch is a thin `extruded` plate in the frame of the bone, so `lateral-ambiguous` holds. A change is kept only when the view improves and `validateAsset` (0 issues), `measureSuitability` of the scale and (since 02:10) the three-views guard over every pose and clip instant do not get worse.
- **Volume repairs.** Where the model was wrong the model changed: finger capsules 1.2 times thicker where interlocked hands left one-cell slits (teenager, tennis player, basketball player, schoolchild, traveller, shovel worker), the coat skirt of the traveller joined to the suitcase grip at 1:200 (12.2 mm), the fists of the toddler's 1:500 forearms slimmed to the declared depth, the rack left out of the bicycle's 1:500 volume as its 1:500 drawing leaves it out (39 solids of the three lists), the hands of nurse and office worker joined to the forearms at 1:500 and 1:200, patellae, scapulae and foot bones of the HW-04 skeletons moved onto the bones they articulate with (`REP-A/join.ts`).
- **Declared areas.** `areaM2` of 17 regions set to the outer surface their thermal solids have at the finest scale that reported them (`REP-A/areas.py`, files `*.21-areas.json` with the old value in `was`): head, arm clothing and gloves of the three HW-04 people, instrument and tripod, face of the nurse, desk frame, paper and easel frame, top and backpack of the schoolchild, shorts, socks and shoes of the toddler. `powerW` does not depend on them.
- **Generators changed.** `HW-05/build_schoolchild.py`: breathing pattern `hitch-effort` inhale 0.45 → 0.41, exhale 0.42 → 0.46 (the breathing suite requires the exhale to be the longer phase; the generator reproduces its previous output except for `thermal`). `HW-04/jackhammer.py`: face wettedness 0.75 in the three working poses of 6 met and more (0.25 before in every pose); see the catalog-physics line.

## 3. Decisions taken without asking, and their limits

- An automatic patch or cut makes the default pose agree. The three-views suite then showed that 37 such ops of six people (barista 7, waiter 1, basketball player 12, tennis player 6, person stretching 8, shovel worker 3) changed an extent in another pose or failed to draw at a clip instant the check CLI does not sample (`unionPolygons`, the engine defect already in `requests.md`). `REP-A/prune.ts` removed exactly those ops (generator output: 0 three-views findings; catalog files after it: 0). That re-opened silhouette views of five of them. A further pass with the three-views gate inside `fit.ts` was started and stopped after 25 minutes on the barista's front view (22 candidates in a row broke another pose), so those views stay red. I did not trade one suite for the other: three-views is green for them, the silhouette guard is not.
- **Result in the real `catalog-silhouette` test: 17 of the 23 people pass, 6 fail** (barista, basketball player, tennis player, person stretching, shovel worker, person pushing bicycle); before the repair 4 passed. The catalog is not green for this scope.
- `🧊️three-views` fails for `e-scooter-rider` (1:500, pose `kicking-off`: front reaches -0.082 m, side -0.023 m, tolerance 0.058 m; the front shape `shoe-rear-r`) and `person-pushing-bicycle` (1:500, 22 findings, front reaches -0.105 m against 0.000 m) **in the generator output of the workers already** (`REP-A/views.ts` on the kept outputs: 1 and 22 findings); pruning changes nothing there. One attempt on the scooter rider's shoe prism did not move the extent; both stay red.
- Strokes: the hem lines of schoolchild and basketball player were shortened to the shirt they belong to (they ended 16 to 20 mm outside the body).
- The traced patches and plates are right in the default pose and ride rigidly on their bone in the others; they are filler under the worker's shapes, not new detail, and no shape was deleted wholesale. This is the same kind of repair as the `fit-*` and `-sil` shapes of the 11/17 guard ticket, with the same caveat.
- Not done: `checkVolumes` findings that need a decision about a prop or a skeleton rather than a correction (below), the bicycle's 1:50 plan around the fork crown.
- Twice a short diagnostic (`core.ts`, `cover.ts`) ran while a background suite was still running: the one-process rule was broken for a few seconds each time; nothing was written by those runs.

## 4. Row files

No evaluated value of a manifest row changed: `dimensions`, poses, clips, representations, symbols, exports and `powerW` are those of the finisher's rows; `⚡️asset-power` (whole file, with "keeps the manifest in step with the assets") passes, see the test lines. No row file was regenerated; the coordinator has nothing to re-merge from this scope.

## 5. Files

Created in `K/🧰️scripts/REP-A/`: `env.sh`, `apply.py`, `build.sh`, `fit.ts`, `fit.sh`, `gate.sh`, `try.sh`, `ship.sh`, `redo.sh`, `after.sh`, `suite.sh`, `diag.ts`, `mask.ts`, `cover.ts`, `core.ts`, `low.ts`, `views.ts`, `join.ts`, `prune.ts`, `areas.py`, `table.py`, `report.py`, `report-text.txt`, `people.txt`, `ops/<slug>.<stage>-<name>.json` (table at the end).

Updated: `K/🧰️scripts/HW-05/build_schoolchild.py`, `K/🧰️scripts/HW-04/jackhammer.py`; the catalog files `S/🗂️catalog/👤️humans/<slug>/🔣️.json` of every person with an ops file in the table at the end.

Removed: `K/🗑️generated/REP-A/` (logs, masks, work copies, kept generator outputs) and the proof sheets rendered into `K/🗑️generated/FIN-A/` in this session.

Umbrella ticket: created `humans-a-repair.md` (this file). `requests.md` not changed.

## 6. Sheets opened

Default-pose sheets (side, front, top at 1:50, 1:100, 1:200, 1:500; `FIN-A/proof.sh`) opened after the repair: `mobility-scooter-user` (after its first pass), `basketball-player` and `schoolchild-with-backpack` (final files). They read as before; no patch seam or cut is visible at sheet size. The sheets of the other 17 changed people were not opened, and no pose other than the default one was looked at. Mask images of drawing against body (`REP-A/mask.ts`) were opened for the teenager's hands (1:50 plan), the basketball player's hand (1:50 side), the traveller's grip (1:200 plan) and the bicycle (1:500 side).

## Per person: failing gates before and after

Measured with `K/🧰️scripts/HW-03/measure.ts` (the same `measureAsset` call and limits as the real test, plus `checkVolumes` of every rule). A silhouette entry reads scale, view, IoU/largest boundary distance.

| Person | Silhouette views failing before | after | checkVolumes findings before | after |
|---|---|---|---|---|
| `standing-desk-worker` | none | none | 4 connected, 3 structure-coverage, 3 thermal-coverage | 4 connected, 3 structure-coverage, 1 thermal-coverage |
| `presenter-at-flipchart` | 1:100 elevation 0.951/37mm; 1:100 front 0.979/34mm; 1:50 elevation 0.953/30mm; 1:50 front 0.981/33mm; 1:50 plan 0.962/20mm | none | 3 connected, 7 structure-coverage, 4 thermal-coverage | 3 connected, 7 structure-coverage |
| `office-worker-carrying-laptop` | none | none | 2 connected | none |
| `waiter-with-tray` | 1:200 elevation 0.945/80mm; 1:100 front 0.954/34mm; 1:50 elevation 0.981/17mm; 1:50 front 0.957/46mm; 1:50 plan 0.952/23mm | none | none | none |
| `barista` | 1:500 plan 0.914/17mm; 1:100 front 0.943/38mm; 1:50 elevation 0.974/17mm; 1:50 front 0.952/39mm; 1:50 plan 0.946/15mm | 1:100 front 0.943/38mm; 1:50 front 0.968/39mm | 4 connected, 1 cross-scale, 1 silhouette, 1 views-disagree | 4 connected, 1 cross-scale, 1 silhouette, 1 views-disagree |
| `nurse-walking` | 1:100 elevation 0.992/39mm; 1:50 plan 0.983/21mm | none | 2 connected, 2 thermal-coverage, 1 views-disagree | none |
| `doctor-with-clipboard` | none | none | none | none |
| `patient-with-iv-pole` | none | none | none | none |
| `construction-worker-with-shovel` | 1:100 front 0.970/34mm; 1:50 elevation 0.986/23mm; 1:50 front 0.970/33mm; 1:50 plan 0.934/27mm | 1:50 front 0.986/25mm | 3 connected, 3 thermal-coverage, 2 views-disagree | 2 views-disagree |
| `worker-with-jackhammer` | 1:100 front 0.973/37mm; 1:50 elevation 0.985/20mm; 1:50 front 0.974/39mm; 1:50 plan 0.953/26mm | none | 3 connected, 5 thermal-coverage | 2 thermal-coverage |
| `surveyor-with-total-station` | 1:500 front 0.939/175mm; 1:200 elevation 0.975/60mm; 1:100 elevation 0.982/59mm; 1:100 plan 0.965/130mm; 1:50 elevation 0.981/60mm; 1:50 front 0.980/24mm; 1:50 plan 0.960/25mm | none | 3 connected, 9 thermal-coverage | 2 connected |
| `walking-toddler` | 1:500 elevation 0.918/17mm; 1:500 plan 0.857/18mm; 1:200 plan 0.899/17mm; 1:100 plan 0.906/16mm; 1:50 plan 0.909/18mm | none | 1 silhouette, 5 thermal-coverage | none |
| `schoolchild-with-backpack` | 1:500 elevation 0.918/136mm; 1:500 plan 0.907/34mm; 1:50 elevation 0.983/26mm; 1:50 plan 0.950/21mm | none | 4 structure-coverage, 3 thermal-coverage | 4 structure-coverage |
| `teenager-with-headphones` | 1:100 elevation 0.986/35mm; 1:50 elevation 0.991/15mm; 1:50 front 0.983/23mm; 1:50 plan 0.963/35mm | none | none | none |
| `elderly-person-knitting` | 1:100 elevation 0.981/45mm; 1:100 front 0.989/42mm | none | none | none |
| `mobility-scooter-user` | 1:50 elevation 0.989/34mm; 1:50 front 0.991/16mm | none | none | none |
| `tennis-player` | 1:500 plan 0.915/20mm; 1:50 elevation 0.960/27mm; 1:50 front 0.946/26mm; 1:50 plan 0.959/27mm | 1:500 plan 0.915/20mm; 1:50 plan 0.972/15mm | none | none |
| `basketball-player` | 1:100 elevation 0.956/32mm; 1:100 front 0.944/35mm; 1:100 plan 0.944/37mm; 1:50 elevation 0.953/47mm; 1:50 front 0.946/34mm; 1:50 plan 0.936/38mm | 1:50 plan 0.965/30mm | 2 silhouette | none |
| `hiker-with-trekking-poles` | none | none | 4 connected | 4 connected |
| `person-stretching` | 1:500 elevation 0.892/40mm; 1:500 plan 0.892/24mm; 1:200 elevation 0.901/38mm; 1:200 plan 0.908/24mm; 1:100 front 0.943/31mm; 1:50 elevation 0.967/21mm; 1:50 front 0.941/34mm; 1:50 plan 0.918/21mm | 1:500 elevation 0.909/28mm; 1:500 plan 0.907/23mm | 1 silhouette | none |
| `e-scooter-rider` | 1:500 plan 0.903/52mm; 1:200 plan 0.914/68mm; 1:100 plan 0.937/32mm; 1:50 elevation 0.979/19mm; 1:50 front 0.979/21mm; 1:50 plan 0.933/33mm | none | none | none |
| `traveller-with-rolling-suitcase` | 1:200 plan 0.972/70mm; 1:100 front 0.986/112mm; 1:100 plan 0.965/70mm; 1:50 elevation 0.994/51mm; 1:50 front 0.988/21mm; 1:50 plan 0.968/70mm | none | none | none |
| `person-pushing-bicycle` | 1:500 elevation 0.937/216mm; 1:500 front 0.945/175mm; 1:500 plan 0.900/139mm; 1:200 elevation 0.958/63mm; 1:100 front 0.985/35mm; 1:100 plan 0.963/32mm; 1:50 elevation 0.977/19mm; 1:50 front 0.986/35mm; 1:50 plan 0.965/50mm | 1:50 plan 0.977/32mm | 13 connected | 13 connected |

## Final test lines

- `asset-power`: 24 passed (24)
- `breathing-catalog`: 67 passed | 223 skipped (290)
- `catalog-physics`: 69 passed | 771 skipped (840)
- `catalog-silhouette`: 6 failed | 17 passed | 362 skipped (385)
  - barista 1:100 front Hausdorff: expected 0.0375 to be less than or equal to 0.03
  - basketball-player 1:50 plan Hausdorff: expected 0.03 to be less than or equal to 0.015
  - construction-worker-with-shovel 1:50 front Hausdorff: expected 0.025015620120236875 to be less than or equal to 0.015
  - person-pushing-bicycle 1:50 plan Hausdorff: expected 0.031875 to be less than or equal to 0.015
  - person-stretching 1:500 elevation: expected 0.9090248962655602 to be greater than or equal to 0.92
  - tennis-player 1:500 plan: expected 0.9152107085878828 to be greater than or equal to 0.92
- `three-views-a`: 16 passed | 759 skipped (775)
- `three-views-b`: 16 passed | 759 skipped (775)
- `three-views-c`: 2 failed | 12 passed | 761 skipped (775)
  - expected '[spread-elevation-y-front-y] 1:500 e-…' to be '' // Object.is equality
  - expected '[spread-elevation-y-front-y] 1:500 pe…' to be '' // Object.is equality

Scope of the suite runs (each as `REP-A/suite.sh`: vitest with the config of `🧪️tests/🎚️config`, one test file, name filter over the 23 people, at most 4 minutes per call):

- `🌡️catalog-physics` failed before for `worker-with-jackhammer` (core 38.22 °C in the generator output, 38.09 °C with the corrected areas, limit below 38); it passes with the sweating face of the working poses.
- `🫁️breathing-catalog` failed before for `schoolchild-with-backpack` (`hitch-effort`: exhale 0.42 below inhale 0.45); it passes.
- `⚡️asset-power` was run whole (24 tests, includes the manifest agreement).
- NOT run: `🧊️model-catalog` (every pose, clip, scale and mode of every model file set, up to 15 minutes per asset), `🔭️multiview-coverage` and `📑️multiview-sheet` (catalog-wide cells, no per-asset filter). `✅️asset-validation`, `📏️print-suitability`, `🧊️volumes-rules`, `🧊️volumes-connected`, `🧊️model-silhouette`, `🧍️front-projection` and `🧍️plan-projection` take their cases from fixtures or fixed id lists and do not reach these people; `validateAsset` (0 issues) and the check CLI (`✓`, print suitability at four scales, 9 samples per clip) were run on the final file of each of the 20 changed people.
- No suite iterates `checkVolumes` over the catalog; its findings are measured with `HW-03/measure.ts` only.

## What stays red, with the measurement

- `standing-desk-worker`:
  - volumes-connected volumes.500.structure 1 group of members and joints of the structure of 1:500 has its own load path to a support solid, the ground or an anchor but passes within 75.0 mm (one smallest feature) of the members, joints or supports of the rest of the struc
  - volumes-connected volumes.200.body 1 group of body solids of 1:200 stands on the ground or at an anchor but does not touch the solids of the bone that a movable joint of the rig ties it to (contact tolerance 6.0 mm): "desk-top" (+9) 31.7 mm from "hand-r", joint des
  - volumes-structure-coverage volumes.100.structure Member solids of 1:100 lie 22.5 mm outside the body silhouette in the front (limit 15.0 mm); the worst are "mass-desk" 22.5 mm.
  - volumes-connected volumes.100.body 1 group of body solids of 1:100 stands on the ground or at an anchor but does not touch the solids of the bone that a movable joint of the rig ties it to (contact tolerance 3.0 mm): "desk-top" (+11) 3.8 mm from "skin-ring-distal-l
  - volumes-thermal-coverage volumes.50.thermal The outer surface of region "desk-drive" at 1:50 is 0.123 m² against areaM2 0.0973 m² (deviation 26 %, limit 15 %).
  - volumes-structure-coverage volumes.50.structure Member solids of 1:50 lie 9.4 mm outside the body silhouette in the elevation (limit 7.5 mm); the worst are "mass-desk" 9.4 mm.
  - volumes-structure-coverage volumes.50.structure Member solids of 1:50 lie 24.4 mm outside the body silhouette in the front (limit 7.5 mm); the worst are "mass-desk" 24.4 mm.
  - volumes-connected volumes.50.body 1 group of body solids of 1:50 stands on the ground or at an anchor but does not touch the solids of the bone that a movable joint of the rig ties it to (contact tolerance 1.5 mm): "desk-top" (+15) 3.2 mm from "skin-ring-distal-l"
- `presenter-at-flipchart`:
  - volumes-structure-coverage volumes.500.structure Member solids of 1:500 lie 329.5 mm outside the body silhouette in the plan (limit 75.0 mm); the worst are "easel-feet-bar" 329.5 mm, "st-lift" 128.6 mm.
  - volumes-structure-coverage volumes.500.structure[21].members Member solid "hand-bones-r" of 1:500 names bone "marker" in `members` but does not hold it: the bone runs up to 82.1 mm away from the solid (limit 75.0 mm, one smallest feature). A member solid spans only bones that lie in it or b
  - volumes-structure-coverage volumes.200.structure Member solids of 1:200 lie 323.3 mm outside the body silhouette in the plan (limit 30.0 mm); the worst are "easel-feet-bar" 323.3 mm, "st-lift" 74.3 mm.
  - volumes-structure-coverage volumes.100.structure Member solids of 1:100 lie 326.6 mm outside the body silhouette in the plan (limit 15.0 mm); the worst are "easel-feet-bar" 326.6 mm, "st-lift" 71.3 mm.
  - volumes-connected volumes.100.structure The structure of 1:100 has an open load path: 1 group of members and joints reaches neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching sol
  - volumes-connected volumes.100.structure 1 group of members and joints of the structure of 1:100 has its own load path to a support solid, the ground or an anchor but passes within 15.0 mm (one smallest feature) of the members, joints or supports of the rest of the struc
  - volumes-structure-coverage volumes.50.structure Member solids of 1:50 lie 14.3 mm outside the body silhouette in the elevation (limit 7.5 mm); the worst are "scapula-r" 14.3 mm, "easel-feet-bar" 9.4 mm.
  - volumes-structure-coverage volumes.50.structure Member solids of 1:50 lie 7.7 mm outside the body silhouette in the front (limit 7.5 mm); the worst are "easel-feet-bar" 7.7 mm.
  - volumes-structure-coverage volumes.50.structure Member solids of 1:50 lie 329.8 mm outside the body silhouette in the plan (limit 7.5 mm); the worst are "easel-feet-bar" 329.8 mm, "st-lift" 73.1 mm.
  - volumes-connected volumes.50.body The body of 1:50 is not one connected object: 1 group of solids touches neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance 1.5 mm); worst: "hand-l-index" 2.4 mm from "hand-l-middle". Joi
- `barista`:
  - volumes-cross-scale volumes.200.body The body of 1:200 disagrees with 1:100 in the plan: IoU 1.000 (at least 0.850), boundary distance p95 90.0 mm (at most 60.0 mm).
  - volumes-connected volumes.200.body The body of 1:200 is not one connected object: 1 group of solids touches neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance 6.0 mm); worst: "milk-jug" (+1) 10.4 mm from "hand-l". Join ev
  - volumes-connected volumes.200.structure The structure of 1:200 has an open load path: 1 group of members and joints reaches neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching sol
  - barista 1:100 bbox 0.545/0.551 1.729/1.726 0.466/0.462  elevation 0.973/15mm  front 0.943/38mm!  plan 0.945/17mm
  - volumes-connected volumes.100.structure The structure of 1:100 has an open load path: 2 groups of members and joints reach neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching soli
  - barista 1:50 bbox 0.545/0.551 1.729/1.726 0.466/0.462  elevation 0.975/14mm  front 0.968/39mm!  plan 0.951/15mm
  - volumes-views-disagree representations.50 The elevation and the plan of 1:50 disagree on the length of the asset: 555 mm against 541 mm (more than 11.1 mm); repair the 2D drawing, no 3D model can agree with both.
  - volumes-silhouette volumes.50.body The body of 1:50 disagrees with the front drawing: p95SoftM 31.9 mm (at most 30.0 mm); the largest boundary distance is 39.0 mm.
  - volumes-connected volumes.50.structure The structure of 1:50 has an open load path: 1 group of members and joints reaches neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching soli
- `construction-worker-with-shovel`:
  - volumes-views-disagree representations.100 The elevation and the plan of 1:100 place the asset differently along its length: -433 to 436 mm against -420 to 457 mm (an edge 21.1 mm apart, more than 17.5 mm); repair the 2D drawing, no 3D model can agree with both.
  - construction-worker-with-shovel 1:50 bbox 0.868/0.869 1.740/1.74 0.583/0.578  elevation 0.987/15mm  front 0.986/25mm!  plan 0.956/15mm
  - volumes-views-disagree representations.50 The front and the plan of 1:50 disagree on the width of the asset: 585 mm against 571 mm (more than 11.7 mm); repair the 2D drawing, no 3D model can agree with both.
- `worker-with-jackhammer`:
  - volumes-thermal-coverage volumes.100.thermal The outer surface of region "breaker-grips" at 1:100 is 0.098 m² against areaM2 0.07 m² (deviation 39 %, limit 30 %).
  - volumes-thermal-coverage volumes.50.thermal The outer surface of region "breaker-grips" at 1:50 is 0.052 m² against areaM2 0.07 m² (deviation 25 %, limit 15 %).
- `surveyor-with-total-station`:
  - volumes-connected volumes.50.body The body of 1:50 is not one connected object: 1 group of solids touches neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance 1.5 mm); worst: "handle" 40.0 mm from "standard-r". Join every
  - volumes-connected volumes.50.structure The structure of 1:50 has an open load path: 2 groups of members and joints reach neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching solid
- `schoolchild-with-backpack`:
  - volumes-structure-coverage volumes.200.structure Member solids of 1:200 lie 45.9 mm outside the body silhouette in the elevation (limit 30.0 mm); the worst are "sk-strap-l" 45.9 mm, "sk-strap-r" 45.9 mm.
  - volumes-structure-coverage volumes.100.structure Member solids of 1:100 lie 47.3 mm outside the body silhouette in the elevation (limit 15.0 mm); the worst are "sk-strap-l" 47.3 mm, "sk-strap-r" 47.3 mm.
  - volumes-structure-coverage volumes.100.structure Member solids of 1:100 lie 15.5 mm outside the body silhouette in the plan (limit 15.0 mm); the worst are "sk-strap-l" 15.5 mm.
  - volumes-structure-coverage volumes.50.structure Member solids of 1:50 lie 43.8 mm outside the body silhouette in the elevation (limit 7.5 mm); the worst are "sk-strap-l" 43.8 mm, "sk-strap-r" 43.8 mm.
- `tennis-player`:
  - tennis-player 1:500 bbox 0.792/0.789 1.603/1.603 0.659/0.667  elevation 0.943/20mm  front 0.948/28mm  plan 0.915/20mm!
  - tennis-player 1:50 bbox 0.792/0.789 1.603/1.603 0.668/0.667  elevation 0.971/15mm  front 0.966/15mm  plan 0.972/15mm!
- `basketball-player`:
  - basketball-player 1:50 bbox 0.670/0.67 1.851/1.851 0.868/0.868  elevation 0.976/13mm  front 0.967/15mm  plan 0.965/30mm!
- `hiker-with-trekking-poles`:
  - volumes-connected volumes.200.structure The structure of 1:200 has an open load path: 1 group of members and joints reaches neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching sol
  - volumes-connected volumes.200.structure 2 groups of members and joints of the structure of 1:200 have their own load path to a support solid, the ground or an anchor but pass within 30.0 mm (one smallest feature) of the members, joints or supports of the rest of the str
  - volumes-connected volumes.100.structure The structure of 1:100 has an open load path: 1 group of members and joints reaches neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching sol
  - volumes-connected volumes.50.structure The structure of 1:50 has an open load path: 3 groups of members and joints reach neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching solid
- `person-stretching`:
  - person-stretching 1:500 bbox 0.715/0.6994 1.711/1.7176 0.966/0.9714  elevation 0.909/28mm!  front 0.927/39mm  plan 0.907/23mm!
- `person-pushing-bicycle`:
  - volumes-connected volumes.500.body The body of 1:500 is not one connected object: 1 group of solids touches neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance 15.0 mm); worst: "crank-arm-f" (+1) 25.9 mm from "chainstay-l"
  - volumes-connected volumes.500.body 1 group of body solids of 1:500 stands on its own (on the ground or at an anchor) but within 75.0 mm (one smallest feature) of the rest without touching it (contact tolerance 15.0 mm), a gap no drawing of this scale shows: "stand-
  - volumes-connected volumes.500.structure 1 group of members and joints of the structure of 1:500 has its own load path to a support solid, the ground or an anchor but passes within 75.0 mm (one smallest feature) of the members, joints or supports of the rest of the struc
  - volumes-connected volumes.200.body The body of 1:200 is not one connected object: 1 group of solids touches neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance 6.0 mm); worst: "fender-f" 8.0 mm from "fork-crown". Join ever
  - volumes-connected volumes.200.body 2 groups of body solids of 1:200 stand on the ground or at an anchor but do not touch the solids of the bone that a movable joint of the rig ties them to (contact tolerance 6.0 mm): "stand-leg-l" 19.0 mm from "tyre-r", joint stand
  - volumes-connected volumes.200.structure 1 group of members and joints of the structure of 1:200 has its own load path to a support solid, the ground or an anchor but passes within 30.0 mm (one smallest feature) of the members, joints or supports of the rest of the struc
  - volumes-connected volumes.100.body The body of 1:100 is not one connected object: 3 groups of solids touch neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance 3.0 mm); worst: "fender-f" 8.0 mm from "fork-crown", "pedal-f"
  - volumes-connected volumes.100.body 1 group of body solids of 1:100 stands on its own (on the ground or at an anchor) but within 15.0 mm (one smallest feature) of the rest without touching it (contact tolerance 3.0 mm), a gap no drawing of this scale shows: "fender-
  - volumes-connected volumes.100.structure The structure of 1:100 has an open load path: 4 groups of members and joints reach neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching soli
  - volumes-connected volumes.100.structure 2 groups of members and joints of the structure of 1:100 have their own load path to a support solid, the ground or an anchor but pass within 15.0 mm (one smallest feature) of the members, joints or supports of the rest of the str
  - person-pushing-bicycle 1:50 bbox 1.867/1.868 1.791/1.791 0.898/0.898  elevation 0.979/12mm  front 0.988/13mm  plan 0.977/32mm!
  - volumes-connected volumes.50.body The body of 1:50 is not one connected object: 2 groups of solids touch neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance 1.5 mm); worst: "stand-bracket" (+5) 2.5 mm from "chainstay-l",
  - volumes-connected volumes.50.body 1 group of body solids of 1:50 stands on its own (on the ground or at an anchor) but within 7.5 mm (one smallest feature) of the rest without touching it (contact tolerance 1.5 mm), a gap no drawing of this scale shows: "tyre-f" (
  - volumes-connected volumes.50.structure The structure of 1:50 has an open load path: 7 groups of members and joints reach neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching solid

## Ops per person (design inputs applied after the worker's generator)

| Person | ops files (stage-name: number of ops) |
|---|---|
| `standing-desk-worker` | 21-areas: 1 |
| `presenter-at-flipchart` | 21-areas: 2, 50-auto: 25, 53-auto: 3 |
| `office-worker-carrying-laptop` | 20-volumes: 2 |
| `waiter-with-tray` | 50-auto: 5, 51-auto: 25 |
| `barista` | 50-auto: 20 |
| `nurse-walking` | 20-volumes: 2, 21-areas: 1, 50-auto: 2 |
| `doctor-with-clipboard` | none (generator output unchanged) |
| `patient-with-iv-pole` | none (generator output unchanged) |
| `construction-worker-with-shovel` | 20-volumes: 1, 21-areas: 3, 22-joins: 5, 50-auto: 18, 54-auto: 1, 55-auto: 1 |
| `worker-with-jackhammer` | 21-areas: 3, 22-joins: 5, 50-auto: 18, 52-auto: 2 |
| `surveyor-with-total-station` | 21-areas: 5, 22-joins: 2, 50-auto: 25 |
| `walking-toddler` | 20-volumes: 2, 21-areas: 3, 50-auto: 19 |
| `schoolchild-with-backpack` | 20-volumes: 1, 21-areas: 2, 50-auto: 12, 51-auto: 1, 52-auto: 1, 53-auto: 1, 54-auto: 1, 60-manual: 1 |
| `teenager-with-headphones` | 20-volumes: 1, 50-auto: 4, 51-auto: 1, 54-auto: 1, 55-auto: 6 |
| `elderly-person-knitting` | 50-auto: 3 |
| `mobility-scooter-user` | 50-auto: 7 |
| `tennis-player` | 20-volumes: 1, 50-auto: 0, 51-auto: 28, 54-auto: 1, 55-auto: 1 |
| `basketball-player` | 20-volumes: 1, 50-auto: 33, 51-auto: 22, 53-auto: 4, 54-auto: 2, 60-manual: 1 |
| `hiker-with-trekking-poles` | none (generator output unchanged) |
| `person-stretching` | 50-auto: 50 |
| `e-scooter-rider` | 50-auto: 37, 54-auto: 2 |
| `traveller-with-rolling-suitcase` | 20-volumes: 2, 50-auto: 23, 52-auto: 2, 54-auto: 1, 55-auto: 3 |
| `person-pushing-bicycle` | 20-volumes: 39, 50-auto: 38, 52-auto: 2, 55-auto: 2 |
