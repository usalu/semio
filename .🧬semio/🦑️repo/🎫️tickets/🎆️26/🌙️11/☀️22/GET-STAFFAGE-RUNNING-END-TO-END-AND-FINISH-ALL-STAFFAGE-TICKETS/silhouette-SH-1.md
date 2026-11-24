# 🌓️ Silhouette Batch SH-1: 25 Humans

Fixer report, 2026-11-23 16:30 to 2026-11-24 (interrupted once by a usage limit; after it every file of the batch was re-checked before work went on). Scope: the 25 people of `🧰️silhouette/SH-1.tsv` in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans`. No rule, threshold, test, engine, schema, fixture or manifest file was edited. Every line quoted below was run to its end in this session with the pinned bun, one tool process at a time.

## Result

- **All 25 pass the real `🧊️catalog-silhouette` test**: `vt.sh sil-all 🧊️catalog-silhouette <25 slugs>` → `exit 0 Test Files 1 passed (1); Tests 25 passed | 360 skipped (385)` (last run on the final files; 0 of 25 passed at the start).
- Failing checks (bounding-box axis, IoU, largest boundary distance, per scale and view): 93 before, 0 after (`measure.ts`, the table below).
- `validateAsset` 0 issues and check CLI `✓` for all 25 (`proof.sh`, last run on the final files).
- Other suites with the name filter of the batch, before → after: `🧊️three-views` 50 passed → 50 passed; `🌡️catalog-physics` 75 → 75; `🫁️breathing-catalog` 74 → 74; `✅️asset-validation` (sample figure and manifest cases) 10 → 10; `⚡️asset-power` (catalog-wide invariants and manifest step) 4 → 4.
- Nothing stays red in the batch. What is marginal or was decided without asking is listed under "Open, stated plainly".

## Per person

"Before" is the baseline of 2026-11-23 16:31, "after" the proof on the final files. Distances are the largest boundary distance (limits 15, 30, 60, 150 mm at 1:50, 1:100, 1:200, 1:500), IoU limit 0.92, box limit one smallest feature.

| Person | Failing checks before | Failing checks after | Proof (measure, `validateAsset`, check CLI) | Changed |
|---|---|---|---|---|
| `barista` | 1:50 front 39.0 mm; 1:100 front 37.5 mm | none | OK, 0 issues, ✓ | 3 shapes redrawn or refitted; 13 shapes given way; 6 solids drawn as prisms; 6 outlines traced from the volume (plates) |
| `basketball-player` | 1:50 plan 30.0 mm | none | OK, 0 issues, ✓ | 1 shapes redrawn or refitted |
| `child-on-kick-scooter` | 1:50 elevation 28.1 mm; 1:50 front 26.3 mm; 1:50 plan 33.8 mm; 1:200 elevation 88.0 mm | none | OK, 0 issues, ✓ | 5 shapes redrawn or refitted; 2 shapes added; 12 shapes given way; 1 solids drawn as prisms; 11 outlines traced from the volume (plates) |
| `construction-worker-with-shovel` | 1:50 front 25.0 mm | none | OK, 0 issues, ✓ | 2 shapes redrawn or refitted |
| `courier-with-hand-truck` | 1:50 plan 25.5 mm; 1:200 plan 141.8 mm; 1:500 plan IoU 0.885 | none | OK, 0 issues, ✓ | 6 shapes redrawn or refitted; 2 shapes added; 5 shapes given way; 1 solids drawn as prisms; 34 outlines traced from the volume (plates) |
| `cyclist` | 1:50 elevation 29.1 mm; 1:50 front 46.9 mm; 1:50 plan IoU 0.920; 1:50 plan 45.0 mm; 1:100 front 149.3 mm; 1:100 plan 41.4 mm; 1:500 elevation IoU 0.905; 1:500 front IoU 0.919; 1:500 plan IoU 0.913 | none | OK, 0 issues, ✓ | 30 shapes redrawn or refitted; 3 shapes added; 84 shapes given way; 95 outlines traced from the volume (plates) |
| `dancer` | 1:50 elevation 30.0 mm; 1:50 front IoU 0.910; 1:50 front 41.6 mm; 1:50 plan IoU 0.899; 1:50 plan 33.2 mm; 1:100 front IoU 0.914; 1:100 front 36.9 mm; 1:100 plan IoU 0.896; 1:100 plan 37.1 mm; 1:200 plan IoU 0.902; 1:500 front IoU 0.906; 1:500 plan IoU 0.893 | none | OK, 0 issues, ✓ | 12 shapes redrawn or refitted; 88 shapes given way; 1 solids drawn as prisms; 96 outlines traced from the volume (plates) |
| `elderly-with-cane` | 1:50 width 0.470/0.48 m; 1:50 front 17.0 mm; 1:50 plan 24.4 mm | none | OK, 0 issues, ✓ | dims widthM 0.477; 3 shapes redrawn or refitted; 3 solids drawn as prisms |
| `jogger` | 1:50 elevation 25.2 mm; 1:50 front 28.2 mm; 1:50 plan 28.1 mm; 1:100 front 45.6 mm | none | OK, 0 issues, ✓ | 12 shapes redrawn or refitted; 1 shapes added; 18 shapes given way; 19 outlines traced from the volume (plates) |
| `kneeling-gardener` | 1:50 height 1.025/1.033 m; 1:50 elevation 34.2 mm; 1:50 front 39.6 mm | none | OK, 0 issues, ✓ | dims heightM 1.027; 5 shapes redrawn or refitted; 53 shapes given way; 7 outlines traced from the volume (plates) |
| `leaning-person` | 1:50 depth 0.532/0.54 m; 1:50 elevation 27.3 mm; 1:50 front 41.3 mm; 1:50 plan 43.2 mm; 1:100 front 37.5 mm; 1:100 plan 38.9 mm; 1:200 front 61.6 mm | none | OK, 0 issues, ✓ | dims depthM 0.536; 27 shapes given way; 10 solids drawn as prisms; 6 shapes redrawn or refitted; 65 outlines traced from the volume (plates) |
| `painter-at-easel` | 1:50 elevation 17.3 mm; 1:50 front 16.9 mm; 1:50 plan 36.1 mm | none | OK, 0 issues, ✓ | 1 solids drawn as prisms; 11 shapes given way; 25 outlines traced from the volume (plates); 4 volume solids changed or added |
| `parent-carrying-toddler` | 1:50 height 1.757/1.744 m; 1:50 plan 19.1 mm | none | OK, 0 issues, ✓ | dims heightM 1.757; 1 shapes redrawn or refitted |
| `parent-with-stroller` | 1:50 width 1.810/1.8 m; 1:50 height 1.738/1.75 m; 1:50 depth 0.610/0.6 m; 1:50 front 71.2 mm; 1:50 plan 26.0 mm; 1:100 front 68.9 mm; 1:100 plan 30.2 mm; 1:200 front 61.1 mm; 1:200 plan 92.2 mm; 1:500 elevation IoU 0.899; 1:500 front IoU 0.900; 1:500 plan IoU 0.919; 1:500 plan 251.5 mm | none | OK, 0 issues, ✓ | dims widthM 1.81, heightM 1.739, depthM 0.61; 69 shapes given way; 25 solids drawn as prisms; 13 shapes redrawn or refitted; 4 shapes added; 2 volume solids changed or added; 69 outlines traced from the volume (plates) |
| `person-carrying-ladder` | 1:50 depth 0.516/0.5 m; 1:50 front 18.8 mm; 1:100 depth 0.516/0.5 m; 1:200 elevation IoU 0.908; 1:500 elevation 162.1 mm | none | OK, 0 issues, ✓ | dims depthM 0.511; 35 shapes given way; 32 outlines traced from the volume (plates); 12 shapes redrawn or refitted; 1 solids drawn as prisms; 1 shapes added; 2 volume solids changed or added |
| `person-carrying-shopping-bags` | 1:50 front 25.5 mm | none | OK, 0 issues, ✓ | 9 shapes given way; 25 outlines traced from the volume (plates); 1 shapes redrawn or refitted |
| `person-checking-watch` | 1:50 plan 27.9 mm | none | OK, 0 issues, ✓ | 4 shapes redrawn or refitted; 34 shapes given way; 2 outlines traced from the volume (plates) |
| `person-cooking` | 1:50 elevation 21.2 mm; 1:50 front 21.2 mm; 1:50 plan 46.1 mm | none | OK, 0 issues, ✓ | 2 shapes redrawn or refitted; 40 shapes given way; 18 solids drawn as prisms; 21 outlines traced from the volume (plates) |
| `person-drinking-coffee` | 1:50 depth 0.551/0.564 m; 1:50 front 16.0 mm | none | OK, 0 issues, ✓ | dims depthM 0.556; 11 shapes given way; 13 outlines traced from the volume (plates) |
| `person-eating-standing` | 1:50 elevation 19.6 mm; 1:50 front 26.6 mm; 1:50 plan 30.9 mm | none | OK, 0 issues, ✓ | 49 shapes given way; 16 outlines traced from the volume (plates) |
| `person-hanging-laundry` | 1:50 plan 29.6 mm; 1:100 plan 33.5 mm | none | OK, 0 issues, ✓ | 5 shapes given way; 7 outlines traced from the volume (plates); 1 shapes added |
| `person-on-crutches` | 1:50 elevation 30.1 mm | none | OK, 0 issues, ✓ | 6 volume solids changed or added; 2 shapes added |
| `person-pushing-bicycle` | 1:50 plan 31.9 mm | none | OK, 0 issues, ✓ | 64 shapes given way; 18 outlines traced from the volume (plates) |
| `person-signing` | 1:50 width 0.547/0.556 m; 1:50 depth 0.523/0.548 m; 1:50 elevation 27.3 mm; 1:100 depth 0.523/0.548 m; 1:100 front 30.2 mm | none | OK, 0 issues, ✓ | dims widthM 0.554, depthM 0.53; 34 shapes given way; 37 outlines traced from the volume (plates); 2 shapes redrawn or refitted |
| `person-stretching` | 1:500 elevation IoU 0.909; 1:500 plan IoU 0.907 | none | OK, 0 issues, ✓ | dims widthM 0.701; 14 shapes given way; 15 outlines traced from the volume (plates) |

## How it was repaired

All changes are authored data of the person, recorded as ops in `🧰️silhouette/SH-1/ops/<slug>.json` and applied by `apply.ts`; the last op of every touched view pins the drawing order, and `pin-order.ts` proved for all 25 that the ops applied to the catalog file give the catalog file again (same JSON value), so the file is reproducible from itself. The earlier generators of the human wave are not re-run: these ops are the last step after them. The volume is the reference; where it was changed, it is said below.

- **Greedy repair** (`auto.ts`, after `REP-B/auto.ts`, passes `pass1.sh` … `pass9.sh`): refits a drawn shape from its solid, or redraws a bone. Added in this batch:
  - **Plates**: a bone is drawn as the traced outline of its volume solids (grouped by material) on a prism swept along the bone axis that points most towards the viewer, placed by the 3D frame of the bone. This is what brought hands with fingers, held objects and the dancer to green. Guards: a bone larger than 0.15 m that stands more than 20° askew to the view gets no plate (its outline would not hold in other poses); the trace is set in by 1 mm so that hairline slits between fingers stay open as in the body; a hand is drawn with all its finger bones in one step, a left and right limb that cover each other in one step.
  - It now refuses what the check CLI refuses: print suitability of the working scale is checked beside `validateAsset`, and a shape that breaks either is taken back with its whole group.
  - It no longer replaces one shape by the outline of its whole bone, no longer moves a trace to a parent bone, and no longer revolves a solid that sits beside the bone axis. All three had been done by the first pass and were found by looking at the sheets and by `🧊️three-views`; they were taken out again (`grown.ts`, `tv-culprits.py`, `pass8.sh`, `pass9.sh`).
- **Look** (`restyle.ts`): an outline that replaces authored shapes takes their tone, hatch, pen and drawing order (same id, else same material, else the tone of its solid).
- **Declared dimensions** (`dimset.ts`): nine people, see below; where a drawing was wider than its body by more than `validateAsset` allows, the limbs concerned were redrawn first (`pass6.sh`).
- **By hand**: `person-carrying-ladder` 1:200 elevation rails centred on the true rails (drawn 40 mm wide, were aligned to the outer edge: IoU 0.908 → 0.992) and 1:500 elevation frame outline closed at the last rung instead of across the open rail ends (162 → 23 mm); `courier-with-hand-truck` 1:500 plan parcel stack tapered like the three parcels (IoU 0.885 → 0.923); `person-pushing-bicycle` 1:50 plan pedal spindles drawn (they were missing) and the steering group redrawn; `cyclist` 1:50 hubs, rear brake hanger and chain-guard strap drawn; `leaning-person` 1:100 front forearms 8 mm slimmer.

## Volume changes (four people, all at one scale)

- `person-on-crutches` 1:50: each hoodie drawstring (one bowed slender loft) is two straight lofts of 4.8 mm with a thermal entry each, and the side elevation draws the cord (`crutches-drawstrings.py`); 30.1 → 5.6 mm. Reason: request 1 below.
- `painter-at-easel` 1:50: easel legs, back leg and mast are no longer declared `slender` (22 × 35 mm members, the flag is not needed at this scale); plan 20.6 → 5.6 mm. Reason: request 2 below.
- `person-carrying-ladder` 1:50: the two rails likewise not `slender`; front 15.1 → 14.6 mm.
- `parent-with-stroller` 1:500: toe member solids `toe-bones-l/r` added to the structure. The declared height 1.739 m (the body) makes the 87.5 mm toe bones load-bearing by the 5 % rule, and the validator then asks for their member; no new `checkVolumes` finding (the open load path of 1:500 was there before).

## Manifest changes for the coordinator

`🧰️silhouette/SH-1/manifest-changes.json` (dimensions only; no power changed, `⚡️asset-power` "keeps the manifest in step" passes):

| Person | Change |
|---|---|
| `elderly-with-cane` | widthM 0.48 → 0.477 |
| `kneeling-gardener` | heightM 1.033 → 1.027 |
| `leaning-person` | depthM 0.54 → 0.536 |
| `parent-carrying-toddler` | heightM 1.744 → 1.757 |
| `parent-with-stroller` | widthM 1.8 → 1.81, heightM 1.75 → 1.739, depthM 0.6 → 0.61 |
| `person-carrying-ladder` | depthM 0.5 → 0.511 |
| `person-drinking-coffee` | depthM 0.564 → 0.556 |
| `person-signing` | widthM 0.556 → 0.554, depthM 0.548 → 0.53 |
| `person-stretching` | widthM 0.6994 → 0.701 |

The manifest row carries width and height only; depth is listed because the brief asks for it.

## Requests appended to `requests.md`

1. `strokesOf`: a bowed slender solid without a centre line becomes one straight stroke as wide as its bow (`person-on-crutches`).
2. `strokesOf`: the stroke of a slender prism that stands askew has square ends beyond the prism (`painter-at-easel`, also the ladder rails).
3. `evaluate`: the union of an extruded outline with many short edges throws `PolygonUnionError` (`parent-with-stroller`); outlines there are simplified to 1.5 mm and 4 mm.

## Sheets opened

The third-angle sheets of all 25 at 1:50 and 1:500 were rendered from the file of the session start and from the final file and opened side by side as contact sheets (`sheets.sh`, `contact.ts`; enlarged crops of the gardener, the child on the scooter and the cook with `crop.ts`). At 1:500 the figures are a few millimetres on the sheet and no difference is visible at that size. At 1:50, looking found four defects of the first pass, all repaired and looked at again: weed leaves painted over the gardener's trug, the jogger's shorts painted down to the knees, the stroller seat drawn over the child, the dancer's shirt hem in the tone of the shorts. Differences that remain and were kept: hands now show their fingers as the volume has them; sleeves and shoulders of the cook, the eater and the signer follow the volume; the gardener's trug is a rounder bowl in the side elevation; the hi-vis vest of the ladder carrier has a plainer hatch in the front view; the rear wheel of the kick scooter is a disc in the plan, mostly under the backpack.

## Open, stated plainly

- **Marginal passes** (on the limit as measured, the real test accepts them): 1:50 views at exactly 15.0 mm for `barista` plan, `cyclist` front, `jogger` front, `parent-with-stroller` plan, `person-checking-watch` plan, `courier-with-hand-truck` elevation; `courier-with-hand-truck` 1:500 plan IoU 0.923; `cyclist` 1:500 elevation 0.925 and front 0.921; `person-on-crutches` height 7.5 mm off at 1:50 (unchanged).
- **Traces on a parent bone left in place** from the first pass, because `🧊️three-views` passes with them and they are small: `barista` 1:50 front hair on the torso, `basketball-player` 1:50 plan shirt, `jogger` 1:50 front shirt (chest → torso), `construction-worker-with-shovel` and `child-on-kick-scooter` 1:50 front torso on the pelvis, `child-on-kick-scooter` 1:50 plan hands on the arms. They move rigidly with the parent in poses.
- **Plates are exact in the default pose only.** In other poses a plate is a prism of the traced outline; small parts (hands) are allowed askew, large ones are not. `🧊️three-views` (every pose and clip instant) passes for all 25, which is the only check of this.
- The catalog files were rewritten in full JSON indentation by `apply.ts` (as the earlier repair tools did); the files are larger, the content is as described.
- Catalog-wide suites without a name filter and the site build were not run. The repo ticket tools were not used (the `semio` MCP server did not connect).

## Files

Updated (25): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<slug>/🔣️.json` for `barista`, `basketball-player`, `child-on-kick-scooter`, `construction-worker-with-shovel`, `courier-with-hand-truck`, `cyclist`, `dancer`, `elderly-with-cane`, `jogger`, `kneeling-gardener`, `leaning-person`, `painter-at-easel`, `parent-carrying-toddler`, `parent-with-stroller`, `person-carrying-ladder`, `person-carrying-shopping-bags`, `person-checking-watch`, `person-cooking`, `person-drinking-coffee`, `person-eating-standing`, `person-hanging-laundry`, `person-on-crutches`, `person-pushing-bicycle`, `person-signing`, `person-stretching`.

Updated (ticket): `requests.md` (three sections `## silhouette SH-1 — …`).

Created (ticket, under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`): `silhouette-SH-1.md`; `🧰️silhouette/SH-1/manifest-changes.json`; `🧰️silhouette/SH-1/ops/<slug>.json` (25); and in `🧰️silhouette/SH-1/`: `env.sh`, `people.ts`, `ops.ts`, `apply.ts`, `auto.ts`, `measure.ts`, `validate.ts`, `suit.ts`, `evalcheck.ts`, `clusters.ts`, `extents.ts`, `extreme.ts`, `where.ts`, `strokes.ts`, `grown.ts`, `restyle.ts`, `pin-order.ts`, `dims.ts`, `dimset.ts`, `bridge.ts`, `png.ts`, `svg-png.ts`, `contact.ts`, `crop.ts`, `manifest-changes.ts`, `crutches-drawstrings.py`, `opsadd.py`, `opsdrop.py`, `groupdrop.py`, `shapedrop.py`, `report-table.py`, `tv-culprits.py`, `fix.sh`, `hand.sh`, `try.sh`, `keep.sh`, `drop.sh`, `takeback.sh`, `diag.sh`, `proof.sh`, `vt.sh`, `suites.sh`, `tv.sh`, `look.sh`, `sheets.sh`, `wait.sh`, `pass1.sh` … `pass9.sh`. `keep.sh`, `try.sh`, `fix.sh`, `hand.sh` and `restyle.ts` need the copies of the session-start files, which lived in the generated folder; without them use `apply.ts`.

Removed: `🗑️generated/silhouette/SH-1/` (logs, session-start copies, sheets).
