IN WORK (2026-11-26 02:40: every asset written and proven except the runs marked RUNNING in sections 1 and 3)

# 🌲️ Multiview Fixer Report: Batch MV-3 (Ten Plants and `saguaro-cactus`)

Batch `🧰️matrix/batches/MV-3.tsv` plus `saguaro-cactus`, briefs `📋️multiview-brief.md` and `📋️takeover-brief.md`. Four fixers: 2026-11-25 00:46 to 07:57, 09:00 to 13:30, 16:10 to 18:25, and the fourth from 19:00 to the end (this report is its one coherent state). `rowan` and `saucer-magnolia` of the batch list went to lane ML-3 at 22:50 (coordinator); this batch does not write them any more and says nothing final about them (section 5).

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/MV-3`, `M` = `U/🧰️matrix/MR`, `C` = `S/🗂️catalog/🌳️vegetation`, `GEN` = `U/🗑️generated/matrix/MV-3` (deleted with this report; every number below was read there in a log of a run that ended by itself).

## 1. Result

RESULT_LINE

| Asset | Hard findings before | Scan now | Matrix of the asset on the file now in the catalog (252 cells) | State |
| --- | --- | --- | --- | --- |
| `rhododendron` | 8: `sync-bone-forward` 4, `sync-bone-lateral` 4 | 0 | M_rhododendron | clean |
| `rosemary-bush` | 4: `sync-bone-forward` 1, `sync-bone-height` 1, `sync-forward` 2 | 0 | M_rosemary-bush | clean |
| `scots-pine` | 13: `sync-bone-height` 13 | 0 | 252 ok, 0 fail (00:40, 371 s) | clean |
| `small-leaved-lime` | 185: `anchor-landmark` 89, `sync-bone-lateral` 96 | 0 | M_small-leaved-lime | clean |
| `stone-pine` | 22: `sync-bone-height` 22 | 0 | 252 ok, 0 fail (00:50, 566 s) | clean |
| `sunflower` | 4: `sync-forward` 2, `sync-height` 2 | 0 | M_sunflower | clean |
| `sycamore-maple` | 27: `anchor-landmark` 21, `sync-bone-forward` 3, `sync-bone-height` 3 | 0 | 252 ok, 0 fail (01:14, 1455 s) | clean |
| `tulip-tree` | 0 | 0 | M_tulip-tree | clean, file not changed by the batch |
| `weeping-willow` | 18 (scan of 2026-11-25 20:37; the batch list names 8 extent and 10 anchor findings) | 0 | M_weeping-willow | clean |
| `wisteria-pergola` | 6: `sync-bone-forward` 6 | 0 | M_wisteria-pergola | clean |
| `saguaro-cactus` | 0 (scan 17:25 and 00:30) | 0 | 252 ok, 0 fail (2026-11-25 23:46, 4283 s) | clean, file not changed by the batch |

"Reported" is the soft tier `group-drift` of the matrix (a bone that draws several shapes in two views); it is no failing cell.

## 2. What was changed per asset

Every repair is the last step of the plant's chain `W/make.sh <id>` (steps chosen by `W/plants.tsv` and the config folders of `W`), idempotent: a second run changed no byte.

| Asset | Change in `C/<id>/🔣️.json` | By |
| --- | --- | --- |
| `rhododendron` | the plan circle of the stem cone at 1:500 (`W/fix-rhododendron.ts`) | first fixer |
| `rosemary-bush` | 23 depth plates below the flat front and plan outlines of the parts the wind turns (`W/carry.ts`); nothing of them shows beside the outlines in any pose (`W/visible.ts`) | first fixer |
| `sunflower` | 48 depth plates; nothing of them shows in any pose | first fixer |
| `wisteria-pergola` | 39 depth plates; nothing of them shows in any pose | first fixer |
| `small-leaved-lime` | 15 part outlines drawn again from their body where plan and front disagreed at rest, the crown footprint `ld3` of the 1:50 plan named `ld3-crown` (`W/bodyview.ts`); the four emitters `tp-sun-1` to `tp-sun-4` set on the drawing of their bone (`W/ops/small-leaved-lime.json`) | second fixer |
| `scots-pine` | the 66 depth plates of the first fixer replaced by 18 real bodies (`W/body.ts`, `W/body/scots-pine.json`: 539 sections; 1:50 3, 1:100 2, 1:200 6, 1:500 7) | fourth fixer, 23:21 |
| `stone-pine` | the 89 depth plates replaced by 22 real bodies (`W/body/stone-pine.json`: 970 sections) | fourth fixer, 23:35 |
| `sycamore-maple` | the 261 depth plates replaced by 52 real bodies (`W/body/sycamore-maple.json`: 5016 sections); the 3 anchors of the first fixer stay bound to the bone that draws their place (`W/ops/sycamore-maple.json`) | fourth fixer, 00:00 |
| `weeping-willow` | 217 hanging strands as lofts under the plan sheets of the eight curtains (`W/strands.ts`, flags in `W/strands/weeping-willow.flags`); one emitter bound to the drawing of its bone (`W/ops/weeping-willow.json`) | third fixer's tool, written by the fourth, 01:51 |
| `tulip-tree`, `saguaro-cactus` | nothing | |

At 19:09 another lane rewrote `asset.tags` of every catalog file (no geometry); the batch's files were byte-identical to their measured trial copies until then.

### 2.1 Depth plates are gone where they showed: `W/body.ts`

`W/visible.ts` (third fixer) measures what the plates show beside the drawn outlines in every pose but the default one. Nothing shows for `sunflower`, `rosemary-bush`, `wisteria-pergola`. For `stone-pine` (5.54 % of the silhouette at 1:50 front `storm`), `scots-pine` (3.22 %, `snow`) and `sycamore-maple` (0.46 %, `storm`) the plates showed as straight cuts and slivers; that drawing was not kept.

Tried and not used: the two carriers of MV-L1 change the rest drawing beyond the silhouette guard (measured by the tools on `rowan`: `loft.ts` 964 mm at 1:50, 1056 mm at 1:100; `slice.ts` 339 mm and 730 mm; the guard allows 15 and 30 mm), and `loft.ts` needs ellipsoids, which the pines do not have.

`W/body.ts` keeps every drawn outline and lays, per part (scale, view, bone) that carried plates, ONE `loft` `mvb-<bone>` below the outlines:

- stations along the height (front) or the forward axis (plan); each section is the ellipse over the lateral span INSIDE the drawn outlines of the part (the run of spans that hangs together, starting at the widest, so the body never bridges a notch) and over the depth span of the elevation outlines of the bone;
- every span is drawn in by 0.25 mm of paper plus 2 mm, across and along the stations; the result is evaluated in the default pose and a body that shows beside the outlines there is drawn in further (up to five times) or left out and named (left out: 4 limb parts of `scots-pine` and 3 of `stone-pine` at 1:200 and 1:500, which no finding needs);
- the body takes tone, region and material of the outlines that cover most of the part.

In a wind pose the bone turns the body itself; what shows beside the foreshortened outline is the smooth edge of one body. Measured with `visible.ts --prefix mvb-` on the written files: no island, no plate cut; the bodies show more than the plates did because they are whole (worst sheets: `scots-pine` 1:50 front `storm` 9.56 % of the silhouette, `stone-pine` 1:50 front `storm` 16.15 %, `sycamore-maple` 1:50 front `storm` 0.68 %). The chain run on the file of before the first fixer (`body.ts --ops` on the backup of 00:46) gives the same `representations` as the catalog file (`W/same.py`: equal for the three).

Cost, measured with `W/timing.ts`: `evaluate` per subject at 1:50 is 173 ms for `scots-pine` (plates: 6 ms), 599 ms for `stone-pine`, 833 ms for `sycamore-maple` (plates: 24 ms). The matrix of `sycamore-maple` took 1455 s (plates: 653 s).

Whether the lineage of MV-L1's ellipsoid contour bug (A⁻¹n against Aᵀn) reaches this batch: no. `body.ts` and `strands.ts` do not import `loft.ts` and compute no ellipsoid contour; their sections come from the drawn outlines and from cuts of the polyhedra.

### 2.2 `weeping-willow`: hanging strands, `W/strands.ts`

The eight curtains hang plumb at rest; their plan sheets are one sheet at one height, so in `storm` the plan was 1643 mm short of the elevation (`sync-forward`, 1:50) and an emitter stood 690 mm off its curtain. Prisms cost minutes per subject (request of the second fixer), depth plates failed the silhouette guard (526.6 mm at 1:200). The third fixer's `strands.ts` lays one tapering loft per tress under the plan sheets, cut from the sprays of the volume; nothing of a strand shows at rest. The fourth fixer added the last step of the tool: every scale is evaluated in the default pose and a strand of which more than a tenth of a square millimetre shows beside the union of the filled plan shapes is drawn in by 15 % (up to nine times). Without that step 5 to 15 strands per scale crossed a notch by 2 to 36 mm and the guard read 29.6 mm at 1:50 (limit 15); with it the guard reads what it read before the repair (plan 11.4, 22.8, 48.0, 93.8 mm at the four scales). The first write of 00:25 (without the step) was taken back at 01:32 and the file written again at 01:51.

In `storm` the strands stream out of the plan as tapering bodies (`visible.ts --prefix mvt-`: worst sheet 1:50 plan `storm`, 11.75 % of the silhouette, no island). The thinning of staircase outlines the third fixer searched (`W/clean.ts`, result `W/clean/weeping-willow.search.json`) is not applied: no finding needs it, and the matrix of the plant ends in 41 minutes without it.

### 2.3 Sheets looked at

By the fourth fixer: `scots-pine` 1:50 `storm`, `stone-pine` 1:50 `storm` (before and after the body took the look of its outlines: the first trial drew a hatched pad inside an unhatched body), `sycamore-maple` 1:50 `storm`, `weeping-willow` 1:200 `storm` with strands, and the trial of `rowan` 1:50 `gale`. All read as the tree in wind: one smooth edge per mass beside the scalloped outline, no straight cut, no notch. By the earlier fixers: the 27 sheets of the night, `small-leaved-lime` 1:50 and 1:100.

## 3. Proofs

Every run went through `repo capacity run` (grants of 700 to 2900 MB); the lines are the ones the chains wrote to `GEN/proof/<id>.txt`.

| Proof | `scots-pine`, `stone-pine`, `sycamore-maple` (written 23:21 to 00:00) | `weeping-willow` (written 01:51) | `saguaro-cactus` | `rhododendron`, `rosemary-bush`, `small-leaved-lime`, `sunflower`, `tulip-tree`, `wisteria-pergola` |
| --- | --- | --- | --- | --- |
| Fixer's gate `M/gate.ts --soft` (every pose, clip instant, transition) | 0 hard each | 0 hard | 0 hard (00:30) | 0 hard each (03:37 to 03:47, lime 10:21) |
| Authors' gate `T/🔎️scan.ts --language en <id>` | clean each | clean | clean | clean each (09:50 to 10:25) |
| Silhouette guard, `validateAsset`, three-views sweep (`M/measure.ts`, per scale) | OK, 0 issues, 0 findings at the four scales, each | OK, 0, 0 at the four scales | OK, 0, 0 | OK, 0, 0 each (03:38 to 03:49, lime 10:30) |
| Every authored scale prints (`W/suit.ts`) | prints | prints | prints | prints (02:36 to 03:02, lime 10:14) |
| Second run of the chain | no byte changed | no byte changed | no chain | no byte changed (night) |
| Check CLI `--asset <id>` | `✓` each | `✓` | `✓` | `✓` each (11:55 to 13:00) |
| `🧊️catalog-silhouette` (name filter, 01:26 to 01:43) | 3 of the 10 passed | T_WILLOW_1 | 1 of the 10 passed | 6 of the 10 passed |
| `🧊️three-views` | 20 passed for the ten | T_WILLOW_2 | in the 20 | in the 20 |
| `🌡️catalog-physics` | 20 passed for the ten | T_WILLOW_3 | in the 20 | in the 20 |
| `🫁️breathing-catalog` | 10 passed for the ten | T_WILLOW_4 | in the 10 | in the 10 |
| `✅️asset-validation` | 10 passed | T_WILLOW_5 | same run | same run |
| `⚡️asset-power` | no test of these names (24 skipped), as before | same | same | same |
| Matrix `T/📊️matrix.ts --jobs 1 --fresh --assets <id>` | section 1 | section 1 | section 1 (a first run of 19:05 to 20:37 gave 252 ok cells too, but its record was of the file before the tags rewrite and the merge counted it as missing) | section 1 |

The real test files ran once at the end with the names of ten assets as filter (`W/chain-v.sh final …`): 10 passed of 385, 20 of 775, 20 of 840, 10 of 290, 10 of 85, 0 of 24. For the six plants of the last column the gate, scan, guard and check CLI were not run again by the fourth fixer: their geometry did not change after those runs (only `asset.tags` at 19:09); their test files and their matrix ran on the current files.

## 4. Open

Nothing with a failing cell or a hard finding. Stated plainly:

- `rosemary-bush`, `sunflower`, `wisteria-pergola` still carry depth plates (110 in all). Nothing of them shows beside the outlines in any pose (`visible.ts`, 17:22), so they were left; `body.ts <id> --save body/<id>.json` replaces them the same way if plates are to go from the catalog altogether.
- The bodies and strands are below the outlines, not the outlines themselves: in a wind pose the edge of a mass is the smooth edge of its body, and the scalloped outline lies inside it. A loft that IS the outline changes the rest drawing beyond the guard (2.1).
- `evaluate` costs 0.2 to 0.8 s per subject at 1:50 for the three plants with bodies (2.1).

## 5. For others

- `rowan`, `saucer-magnolia` (lane ML-3): the state the first fixer wrote and proved (matrix of 13:12 and 12:13, 0 fail) held until ML-3 took the files; `rowan` carries 142 depth plates of this batch that show in `gale` (5.74 % of the silhouette at 1:50 front). The body recipe with the numbers of a trial on `rowan` is filed in `U/requests.md` as "multiview MV-3 — note for ML-3: one real body per part in the place of depth plates"; trial ops `W/body/rowan.json` (to be made again on ML-3's final file).
- `U/requests.md`, earlier: "multiview MV-3 — `evaluate`: an `extruded` solid whose profile has hundreds of points costs minutes per turned subject".
- The small-hole rule of `🧊️catalog-silhouette` ("multiview MH-4 — …") is also what turned the willow's first strands red: a strand that crosses a notch of a few millimetres moves the measured boundary from 11.4 to 29.6 mm although no drawn point lies outside the former drawing (`W/outside.ts`: 0 of 52 shapes; `W/refdiff.ts`: 8 cells of 20 mm). Tools that lay bodies under a drawing must test polygons, not points.
- Queue: measured peaks of this batch on the engine of 17:48: scan 1003 MB, gate 519 MB, `measure.ts` per scale 1550 MB at most (all scales of the willow at once: 4267 MB), `strands.ts` 1559 MB, matrix generator with its asset process 806 MB (`scots-pine`), 898 MB (`stone-pine`), 963 MB (`sycamore-maple`), 1041 MB (`weeping-willow`), 512 to 578 MB (`saguaro-cactus`), vitest with a name filter 2860 MB. A runtime restart at 21:03 killed every waiting and running request of the batch once.

## 6. Files

Updated in the catalog (repo-relative, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation`): `C/rhododendron/🔣️.json`, `C/rosemary-bush/🔣️.json`, `C/sunflower/🔣️.json`, `C/wisteria-pergola/🔣️.json` (first fixer, 02:33 to 03:02), `C/small-leaved-lime/🔣️.json` (second fixer, 10:11), `C/scots-pine/🔣️.json`, `C/stone-pine/🔣️.json`, `C/sycamore-maple/🔣️.json` (first fixer, then fourth fixer 23:21 to 00:00), `C/weeping-willow/🔣️.json` (fourth fixer, 01:51). Also written by the first fixer and now with ML-3: `C/rowan/🔣️.json`, `C/saucer-magnolia/🔣️.json`. Not changed: `C/tulip-tree/🔣️.json`, `C/saguaro-cactus/🔣️.json`. No shared engine, schema, validator, test, fixture, README or manifest was edited.

In `U/🧰️matrix/MV-3/` (all kept):

- first fixer: `q.sh`, `grant.sh`, `run.sh`, `tool.sh`, `scan.sh`, `gate.sh`, `suit.sh`, `suit.ts`, `look.sh`, `look-many.sh`, `trial.sh`, `trial-many.sh`, `measure-many.sh`, `make.sh`, `make-many.sh`, `prove.sh`, `prove-group.sh`, `check.sh`, `matrix.sh`, `vt.sh`, `status.sh`, `state.py`, `carry.ts`, `anchors.ts`, `rebind.ts`, `bone.ts`, `probe.ts`, `extreme.ts`, `parts.ts`, `crown500.ts`, `crown500/rowan.json`, `crown500/saucer-magnolia.json`, `bodyview.ts`, `bodyview/small-leaved-lime.json`, `prism.ts`, `fix-rhododendron.ts`, `plants.tsv`, `before.json`, `ops/<id>.json` for the twelve plants.
- second and third fixer: `chain-a.sh`, `chain-b.sh`, `chain-c.sh`, `chain-d.sh`, `timing.ts`, `outside.ts`, `refdiff.ts`, `culprit.ts`, `hot.sh`, `wind.sh`, `worker.sh`, `visible.ts`, `strands.ts`, `strands/weeping-willow.flags`, `clean.ts`; removed by the second: `prism/weeping-willow.json`.
- fourth fixer, created: `body.ts`, `body/scots-pine.json`, `body/stone-pine.json`, `body/sycamore-maple.json`, `body/rowan.json` (trial, for ML-3), `same.py`, `chain-e.sh` (write, compare, prove), `chain-s.sh`, `chain-m.sh` (matrix of one or several assets), `chain-w.sh` (willow), `chain-v.sh` (the real test files).
- fourth fixer, updated: `make.sh` (step `body.ts --ops`), `plants.tsv` (no `carry.ts` for the three plants with bodies: their ops remove the plates and set the sheet suffixes), `strands.ts` (the default-pose step), `culprit.ts` (prefix argument, lofts), `ops/weeping-willow.json` (anchor search on the strands), `clean/weeping-willow.json` renamed to `clean/weeping-willow.search.json` (a search result, not applied by the chain).

`U/requests.md`: one note appended by the fourth fixer (section 5). `U/multiview-MV-3.md`: this report. `U/🗑️generated/matrix/MV-3`: deleted after this report was written.
