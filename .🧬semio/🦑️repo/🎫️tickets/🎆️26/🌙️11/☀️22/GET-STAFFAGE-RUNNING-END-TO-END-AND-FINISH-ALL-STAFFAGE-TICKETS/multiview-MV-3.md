IN WORK (fourth fixer since 2026-11-25 19:00; state of 20:30 below, the sections after it are the interim report of the earlier fixers with their placeholders)

## 0. State of the fourth fixer (2026-11-25 20:30)

- No process of the batch was left at 19:00. Catalog files of the eleven plants other than `weeping-willow` were byte-identical to the measured trial copies until 19:09, when another lane rewrote `asset.tags` of every catalog file (no geometry); `weeping-willow` and `saguaro-cactus` carry no change of this batch.
- Read in logs that ran to their end: the matrix of the eleven plants other than `weeping-willow` (11:55 to 13:26, 0 `fail` each), scan of `saguaro-cactus` clean (17:25).
- Depth plates in wind poses (`visible.ts`, log of 17:22): nothing shows for `sunflower`, `rosemary-bush`, `wisteria-pergola`; plates show beside the outlines for `rowan` (5.74 % of the silhouette at 1:50 front `gale`), `stone-pine` (5.54 %), `scots-pine` (3.22 %), `sycamore-maple` (0.46 %). These four are NOT accepted as they are; trials with real bodies (`🧰️matrix/MV-L1/loft.ts`, `slice.ts`) are queued.
- `weeping-willow`: the catalog file is the untouched one; the trial with hanging strands as lofts (`strands.ts`) waits for its anchors, gate and guard runs.
- `saguaro-cactus`: matrix running since 19:05 (scale entry 10 of 12 at 20:25).
- Not run yet: everything else named as a placeholder below.

# 🌲️ Multiview Fixer Report: Batch MV-3 (12 Plants)

Batch `🧰️matrix/batches/MV-3.tsv`, briefs `📋️multiview-brief.md` and `📋️takeover-brief.md`. First fixer 2026-11-25 00:46 to 07:57 (stopped without a final report), takeover 09:00 to WILLOW_END. Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/MV-3`, `M` = `U/🧰️matrix/MR`, `C` = `S/🗂️catalog/🌳️vegetation`.

## 1. Result

RESULT_LINE

| Asset | Hard findings before (scan 00:55) | After (scan of the takeover) | Matrix of the asset (252 cells) | State |
| --- | --- | --- | --- | --- |
| `rhododendron` | 8: `sync-bone-forward` 4, `sync-bone-lateral` 4 | 0 | 252 ok, 0 fail | clean |
| `rosemary-bush` | 4: `sync-bone-forward` 1, `sync-bone-height` 1, `sync-forward` 2 | 0 | 196 ok, 56 reported group drift, 0 fail | clean |
| `rowan` | 8: `anchor-landmark` 3, `derived-view-in-band` 1, `sync-forward` 2, `sync-height` 2 | 0 | 189 ok, 63 reported, 0 fail | clean |
| `saucer-magnolia` | 1: `derived-view-in-band` 1 | 0 | 182 ok, 70 reported, 0 fail | clean |
| `scots-pine` | 13: `sync-bone-height` 13 | 0 | 252 ok, 0 fail | clean |
| `small-leaved-lime` | 185: `anchor-landmark` 89, `sync-bone-lateral` 96 | 0 | 0 ok, 252 reported, 0 fail | clean |
| `stone-pine` | 22: `sync-bone-height` 22 | 0 | 252 ok, 0 fail | clean |
| `sunflower` | 4: `sync-forward` 2, `sync-height` 2 | 0 | 252 ok, 0 fail | clean |
| `sycamore-maple` | 27: `anchor-landmark` 21, `sync-bone-forward` 3, `sync-bone-height` 3 | 0 | 252 ok, 0 fail | clean |
| `tulip-tree` | 0 | 0 | 0 ok, 252 reported, 0 fail | clean, file not changed |
WILLOW_ROW
| `wisteria-pergola` | 6: `sync-bone-forward` 6 | 0 | 252 ok, 0 fail | clean |

"Reported" is the soft tier `group-drift` of the matrix (a bone that draws several shapes in two views); it is no failing cell.

`saguaro-cactus` was offered as a thirteenth asset if no batch list holds it: `🧰️matrix/batches/MB-2.tsv` holds it, so it was not taken. Nothing of it was touched.

## 2. What was changed per asset

Every repair is the last step of the plant's chain `W/make.sh <id>` (its steps are chosen by `W/plants.tsv` and the config folders of `W`), idempotent: a second run changed no byte in any written file.

| Asset | Change in `C/<id>/🔣️.json` | Bytes before, after |
| --- | --- | --- |
| `rhododendron` | the plan circle of the stem cone at 1:500 (`W/fix-rhododendron.ts`) | 2,275,399 after |
| `rosemary-bush` | 23 depth plates below the flat front and plan outlines of the parts the wind turns (`W/carry.ts`) | 842,507, 885,201 |
| `rowan` | the 1:500 crown as 10 foliage masses from the 1:200 body, in the body and in the three views (`W/crown500.ts`, clears the copied front); 142 depth plates | 1,839,417, 2,477,212 |
| `saucer-magnolia` | the 1:500 crown as 9 foliage masses (`W/crown500.ts`); no plates (they failed the silhouette guard at 1:100) | 1,394,434, 1,474,156 |
| `scots-pine` | 66 depth plates | 6,765,915, 6,830,238 |
| `small-leaved-lime` | 15 part outlines drawn again from their body where plan and front disagreed at rest (1:50 plan 12 bones, 1:50 front `ln2a`, 1:100 plan `la2a`, `ls4a`), the crown footprint `ld3` of the 1:50 plan named `ld3-crown` (`W/bodyview.ts`, `W/bodyview/small-leaved-lime.json`); the four emitters `tp-sun-1` to `tp-sun-4` set on the drawing of their bone, `tp-sun-2` bound to `le1b` (`W/ops/small-leaved-lime.json`) | 23,064,514, 23,039,890 |
| `stone-pine` | 89 depth plates | 4,682,798, 5,047,256 |
| `sunflower` | 48 depth plates | 1,455,755, 1,482,467 |
| `sycamore-maple` | 261 depth plates; 3 anchors bound to the bone that draws their place (`W/ops/sycamore-maple.json`) | 1,821,866, 2,062,170 |
| `tulip-tree` | nothing (the 104 findings of the diagnosis were cause A of the matrix, cleared by the shared repair) | 1,754,760 |
WILLOW_CHANGE
| `wisteria-pergola` | 39 depth plates | 847,959, 875,708 |

Sheets (`W/look.sh`: side, front, plan, line and solid) of the changed views were looked at before writing: by the first fixer for the nine plants of the night (27 sheets), in the takeover again for `rowan` 1:500 and `saucer-magnolia` 1:500 (new crowns: read as trees, no defect) and for `small-leaved-lime` 1:50 calm and gale and 1:100 calm before and after (no visible difference at rest, the gale plan follows the elevation)WILLOW_SHEETS.

## 3. Proofs

All runs went through `repo capacity run`; every run named here ended by itself (exit code and result line kept in `GEN/proof/<id>.txt` until the folder was deleted with this report).

| Proof | Ten plants of the night (`rhododendron`, `rosemary-bush`, `rowan`, `saucer-magnolia`, `scots-pine`, `stone-pine`, `sunflower`, `sycamore-maple`, `tulip-tree`, `wisteria-pergola`) | `small-leaved-lime` | `weeping-willow` |
| --- | --- | --- | --- |
| Authors' gate `T/🔎️scan.ts --language en <id>` | takeover 09:50 to 10:14: each `1 clean, 0 with a hard finding` (also 03:37 to 03:48) | 10:25: clean (10:01 before the write: 185) | WILLOW_SCAN |
| Fixer's gate `M/gate.ts --soft` (every pose, clip instant, transition) | 03:37 to 03:47: 0 hard each | 10:21: 0 hard, 574 subjects | WILLOW_GATE |
| Every authored scale prints (`W/suit.ts`) | 02:36 to 03:02: prints | 10:14: prints | WILLOW_SUIT |
| Silhouette guard, `validateAsset`, three-views sweep (`M/measure.ts`, file before and now) | 03:38 to 03:49: OK, 0 issues, 0 findings, before and now | 10:30: OK, 0, 0, before and now | WILLOW_MEASURE |
| `🧊️catalog-silhouette` (name filter) | 04:00: 10 passed of 385 | 10:31: 1 passed | WILLOW_T1 |
| `🧊️three-views` | 20 passed | 2 passed | WILLOW_T2 |
| `🌡️catalog-physics` | 20 passed | 2 passed | WILLOW_T3 |
| `🫁️breathing-catalog` | 10 passed | 1 passed | WILLOW_T4 |
| `✅️asset-validation` | 10 passed | 10 passed | WILLOW_T5 |
| `⚡️asset-power` | no test of these names (24 skipped), as before | same | same |
| Check CLI `--asset <id>` | `✓` for nine 06:36 to 07:08; `rhododendron` `✓` 11:58 (its request of the night for 5000 MB was given up by the queue after 9215 s); AGAIN_CHECK | `✓` 10:56 | WILLOW_CHECK |
| Matrix `T/📊️matrix.ts --jobs 1 --assets <id>` | 07:12 to 07:57, all ten to the end, 0 `fail`; AGAIN_MATRIX | 11:55: 252 cells, 0 fail, 3644 s, peak 1197 MB | WILLOW_MATRIX |

Catalog files of the nine written plants are byte-identical to the trial copies that were measured (compared by hash at 09:05), and none changed after 03:02.

## 4. `weeping-willow`

WILLOW_SECTION

## 5. Findings for others

- `evaluate` with an `extruded` solid of a many-point profile: filed in `U/requests.md` as "multiview MV-3 — `evaluate`: an `extruded` solid whose profile has hundreds of points costs minutes per turned subject".
- The small-hole rule of `🧊️catalog-silhouette` (already filed as "multiview MH-4 — `🧊️catalog-silhouette`: the small-hole rule turns a corrected drawing red") is also what rejected depth plates here: plates that close a slit of the rest drawing by a few centimetres move the measured boundary by half a metre (`weeping-willow` 1:200 plan 48.0 to 526.6 mm with unchanged reference and 60 changed cells of 20 mm; `W/refdiff.ts`, `W/culprit.ts`). The plates of `saucer-magnolia` that the first fixer left out at 1:100 (front 210 mm) are probably the same case; that plant is clean without them.
- Queue: with about twelve fixers on one agent session the queue admitted a request of 2200 to 3000 MB only after 10 to 30 minutes ("this agent session already holds its share"); chains under one grant (`W/grant.sh`) were the only way to work. Measured peaks of this batch: scan 250 to 870 MB, check CLI below 1000 MB, matrix of one tree up to 1197 MB (`small-leaved-lime`), `measure.ts` 1575 to 1654 MB (willow, lime), vitest with a name filter up to 2860 MB, `carry.ts` on the lime 2652 MB.

## 6. Files

Updated in the catalog (repo-relative, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation`): `C/rhododendron/🔣️.json`, `C/rosemary-bush/🔣️.json`, `C/rowan/🔣️.json`, `C/saucer-magnolia/🔣️.json`, `C/scots-pine/🔣️.json`, `C/stone-pine/🔣️.json`, `C/sunflower/🔣️.json`, `C/sycamore-maple/🔣️.json`, `C/wisteria-pergola/🔣️.json` (first fixer, 02:33 to 03:02), `C/small-leaved-lime/🔣️.json` (takeover, 10:11)WILLOW_FILE. Not changed: `C/tulip-tree/🔣️.json`WILLOW_NOT. No shared engine, schema, validator, test, fixture, README or manifest was edited.

In `U/🧰️matrix/MV-3/` (all kept):
- first fixer: `q.sh`, `grant.sh`, `run.sh`, `tool.sh`, `scan.sh`, `gate.sh`, `suit.sh`, `suit.ts`, `look.sh`, `look-many.sh`, `trial.sh`, `trial-many.sh`, `measure-many.sh`, `make.sh`, `make-many.sh`, `prove.sh`, `prove-group.sh`, `check.sh`, `matrix.sh`, `vt.sh`, `status.sh`, `state.py`, `carry.ts`, `anchors.ts`, `rebind.ts`, `bone.ts`, `probe.ts`, `extreme.ts`, `parts.ts`, `crown500.ts`, `crown500/rowan.json`, `crown500/saucer-magnolia.json`, `bodyview.ts`, `bodyview/small-leaved-lime.json`, `prism.ts`, `fix-rhododendron.ts`, `plants.tsv`, `before.json`, `ops/<id>.json` for the twelve plants.
- takeover, created: `chain-a.sh`, `chain-b.sh`, `chain-c.sh` (chains under one grant: measure, trial, write and prove), `timing.ts` (milliseconds of `evaluate` per subject), `outside.ts`, `refdiff.ts`, `culprit.ts` (why a silhouette guard fails although the rest drawing looks the same).
- takeover, updated: `carry.ts` (`--points`, `--inside`, `--keep`), `plants.tsv` (flags of `weeping-willow`), `ops/weeping-willow.json` (anchor search on the final trial).
- takeover, removed: `prism/weeping-willow.json` (the prism trial is withdrawn; `prism.ts` stays as the tool it was).

`U/requests.md`: one section appended. `U/multiview-MV-3.md`: this report, rewritten. `U/🗑️generated/matrix/MV-3`: deleted after this report was written.
