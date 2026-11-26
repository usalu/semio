FINAL

# 🧍️ Multiview Fixer MH-1: Twelve People

Fourth fixer of the batch (takeover of 2026-11-25 18:58). Batch `🧰️matrix/batches/MH-1.tsv`; paths as in `📋️multiview-brief.md` (`S`, `T`, `U`); scripts in `U/🧰️matrix/MH-1/`, tool output in `U/🗑️generated/matrix/MH-1/` (`GEN`). "Gate" is `../MR/gate.ts --twins` (the matrix's own `geometryFindings` over every pose, clip instant and transition, with every shape against its twin), "scan" the authors' `T/🔎️scan.ts --language en`, "measures" `../MR/measure.ts` (silhouette guard, `validateAsset`, three-views sweep). Every line below was read in a log that ran to its end (`GEN/proof/summary.log`, one line per step with the md5 of the file); what a predecessor wrote without a log is marked "claimed".

## 1. Result

| Asset | Before: gate hard / scan hard | Now: gate / scan | Matrix of the asset | State |
|---|---|---|---|---|
| `barista` | 442 / – | 0 / clean | 252 cells, 0 fail | clean |
| `basketball-player` | 159 / 10 | 0 / clean | 252 cells, 0 fail | clean |
| `child-on-kick-scooter` | 81 / 12 | 0 / clean | 252 cells, 0 fail | clean |
| `construction-worker-with-shovel` | 970 / 51 | 0 / clean | 252 cells, 0 fail | clean |
| `courier-with-hand-truck` | 1229 / – | 0 / clean | 252 cells, 0 fail | clean |
| `cyclist` | – / 74 | 0 / clean | 252 cells, 0 fail | clean |
| `dancer` | 631 / 21 | 0 / clean | 252 cells, 0 fail | clean |
| `doctor-with-clipboard` | 483 / 22 | 0 / clean | 252 cells, 0 fail | clean |
| `e-scooter-rider` | 336 / 18 | 0 / clean | 252 cells, 0 fail | clean |
| `elderly-person-knitting` | 606 / 35 | 0 / clean | 252 cells, 0 fail | clean |
| `elderly-with-cane` | 665 / 24 | 0 / clean | 252 cells, 0 fail | clean |
| `hiker-with-trekking-poles` | 778 / – | 0 / clean | 252 cells, 0 fail | clean |

"Before" is the gate of 2026-11-24 (`GEN/gate/before-*.log`; the cyclist's gate did not end then) and the scan of 2026-11-25 09:03 on the files of that time (the three without a number were already repaired then).

Twelve of twelve are clean with all proofs; nothing stays open. Last measurement over the whole batch (2026-11-26 00:10, catalog files as they are): scan `12 assets: 12 clean, 0 with a hard finding, 0 hard findings`; replay of the ops of every asset changes nothing.

## 2. Proofs per asset

The six real tests are `🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power`, each with the name filter of the asset; "tests green" means 1, 2, 3, 3, 10 passed and no case in `⚡️asset-power` unless stated. "Replay" is `fit.ts <id> --ops ops/<id>.json` without writing on the catalog file: it must change nothing.

- `barista` (written 2026-11-24 22:08, `ops/barista.json`): gate 0 (before: `sync-bone-height` 1:50 ×103 57 mm, 1:100 ×105 51 mm, 1:500 forward, height, lateral up to 47 mm), scan clean, silhouette OK, validate 0, sweep 0 (25 05:15), check CLI `✓` 11:01, tests green 05:15; matrix 12:25: 0 fail (98 ok, 133 group-drift, 21 range-end), peak 397 MB. Another worker wrote the file again at 20:35 (after the tag write; not this batch), so everything was run once more on the file as it is (md5 6718256e): replay 0, gate 0, scan clean, measures OK, check CLI `✓` 26 00:22, tests green 00:17, matrix 00:29: 0 fail (98 ok, 133 group-drift, 21 range-end), peak 314 MB.
- `construction-worker-with-shovel` (written 25 09:27, equal to its measured trial): gate 0, scan clean, silhouette OK, validate 0, sweep 0, check CLI `✓` 09:41, tests green 09:31; matrix 10:19: 0 fail (189 group-drift, 63 range-end), peak 365 MB. Gate and measures again at 17:38 on the engine that reads `lateralScale`: 0 hard, silhouette OK, validate 0. Sheets 1:50 to 1:500 opened by the third fixer (claimed). With `lateralScale` read, the replay of the ops refitted one shape (`plan-revolve` 1:200 `shank-r`, radius × 0.973); it was written on 26 00:14 so that the chain is stable (replay 0 since), and the file proven again (md5 b7e7e34e): gate 0, scan clean, measures OK, check CLI `✓` 00:16, tests green 00:16, matrix 00:20: 0 fail (189 group-drift, 63 range-end), peak 328 MB.
- `child-on-kick-scooter` (written 25 09:27): gate 0, scan clean, silhouette OK, validate 0, sweep 0, check CLI `✓` 09:41, tests green 09:36 and 26 00:17 (breathing 2 passed); matrix 10:57: 0 fail (252 group-drift), peak 317 MB. The matrix ran on the file before the tag write of 19:09 (md5 2527f333, now 76f453bb: `tags` only); scan, replay and tests were repeated on the file as it is, the matrix was not.
- `courier-with-hand-truck` (written 24 18:07): gate 0 and measures OK at 17:38, scan clean, check CLI `✓` 11:03; matrix 19:28: 0 fail (105 ok, 147 group-drift), peak 318 MB; tests green 21:27 and 23:14.
- `hiker-with-trekking-poles`: the engine reads `lateralScale` of revolved drawing solids since 12:20 (request "multiview MX-2 — `lateralScale` …"); the file proven on 2026-11-24 then failed the guard at 1:50 plan (15.1 mm), and a partial removal written by the third fixer at 17:24 left 4 hard findings and check `✗`. At 19:00 the op `unscale` (last entry of the ops) removed the remaining 50 members, so the file draws what was proven before the engine read the member: replay changes nothing, gate 0, scan clean, silhouette OK, validate 0, sweep 0, check CLI `✓` 19:03; matrix 19:56: 0 fail (14 ok, 154 group-drift, 84 range-end), peak 254 MB; tests green 21:26 and 23:13. (The matrix run of 19:15 crossed the catalog-wide tag write of 19:09 and produced no cell; it was repeated.)
- `e-scooter-rider` (landed 19:44; 18 ops, jacket as loft in the plan at 1:100 to 1:500): replay 0, gate 0, scan clean, measures OK, check CLI `✓` 19:46; matrix 19:52: 0 fail (119 ok, 63 range-end, 70 group-drift), peak 354 MB; tests green 21:27 and 23:13. Sheets 1:50, 1:100, 1:500 opened.
- `dancer` (landed 21:32; 16 ops: lofts for the waist, right shoe and soles at 1:50, the shirt at 1:100 front and at 1:200 in three views, prisms and bodies of revolution for shoes and shanks at 1:200 and 1:500): the last hard finding of the trial, `sync-forward` 47 mm at 1:200 in the pirouette (the elevation prism of the shirt was shoulder-wide, the plan drew the core only), went with the shirt as one loft; guard at 1:200 plan 37.5 mm before, 22.5 mm after. Replay 0, gate 0, scan clean, measures OK, check CLI `✓` 21:37; matrix 22:06: 0 fail (14 ok, 238 group-drift), peak 283 MB; tests green 23:14. Sheets 1:50 and 1:200 (arabesque) opened before and after.
- `elderly-with-cane` (landed 22:08; 44 ops, plan `loft4`: right sleeve and right hand as lofts; the trial with a prism drew a box beside the body in the 1:50 plan and was not landed): replay 0, gate 0, scan clean, measures OK, check CLI `✓` 22:10; matrix 22:17: 0 fail (168 ok, 84 group-drift), peak 253 MB; tests green 23:15. Sheets 1:50 before and after, 1:200 opened.
- `doctor-with-clipboard` (landed 22:17; 17 ops, plan `loft4`: sleeve, forearms and hands as lofts where the guard held): replay 0, gate 0, scan clean, measures OK, check CLI `✓` 22:18; matrix 22:21: 0 fail (133 ok, 119 group-drift), peak 239 MB; tests green 23:16. Sheets 1:50 before and after, 1:200 opened.
- `basketball-player` (landed 22:39; 7 ops, plan `loft5`: right hand and shirt as lofts, the shorts as the prism of the elevation outline in the front, because a loft of the shorts left 40 hard findings on the pelvis): replay 0, gate 0, scan clean, measures OK, check CLI `✓` 22:42; matrix 22:50: 0 fail (28 ok, 224 group-drift), peak 283 MB; tests green 23:17. Sheets 1:50 before and after opened.
- `elderly-person-knitting` (landed 22:50; 22 ops): the version without lofts (gate 0) drew the plan torso as a rectangle and had a hand 0.198 mm wide at 1:500; now the cardigan is a loft in the plan at 1:50, 1:100 and 1:500 and in three views at 1:200 (a plan loft beside a front prism left `sync-bone-lateral` 27 mm at 1:200), the hands at 1:500 are 100 mm wide prisms (0.21 mm on paper), and the ops that the lofts superseded are removed (replayed on a loft they wrote NaN). Replay 0 (one op refused: its target is already what it would write), gate 0, scan clean, measures OK, every scale prints, check CLI `✓` 22:51; matrix 22:59: 0 fail (56 ok, 175 group-drift, 21 range-end), peak 268 MB; tests green 23:18. Sheets 1:50 before and after, 1:200 opened.
- `cyclist` (landed 23:22; 17 ops: pedal spindle anchors at their lateral place z ±0.13, thighs as loft or body of revolution at 1:50, 1:100, 1:500, jacket as loft at 1:100 and 1:200 and in front and plan at 1:500, sleeves as lofts at 1:50, shoes at 1:500; the 1:500 saddle is 10 mm longer at the rear in elevation and plan, because the plan jacket now follows the torso and the 81 mm of saddle that show behind the rider standing on the pedals printed 0.192 mm): replay 0, gate 0 (trial of 22:20: 187), scan clean (74), measures OK, every scale prints, check CLI `✓` 23:25, tests green 23:25 to 23:26; matrix 26 00:10: 0 fail (112 ok, 140 group-drift), peak 473 MB. Sheets 1:50 before and after, 1:100, 1:500 standing on the pedals before and after opened.

## 3. What changed in the tools at this takeover

- At 19:09 another worker rewrote `tags` in every catalog document (objects with titles). The backups of the batch carry the old tags, so `take.sh` (backup, replay, compare with the trial) no longer applies. `carry.py` writes the measured trial into the catalog with every member the ops do not own taken from the catalog file as it is; `land4.sh` runs it, the replay, the light proof, the check CLI and the matrix inside one grant.
- `retry.sh` (ops corrected by hand to a measured trial, without the solver), `prints.ts` (print profile per scale with the shape that fails), `tests.sh` (the real tests suite by suite with the worker peak), `strip.py` (rows of the sheets for a before/after reading).
- The lane (`lane.sh a`, one grant of 1300 MB) and a queued test request were terminated from outside at 21:02 and restarted; nothing was written half.

## 4. Requests filed

None. No defect of the shared engine, the suite or a fixture was measured in this batch.

## 5. Files

Updated (repo-relative, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans`): `C/<id>/🔣️.json` of `basketball-player`, `construction-worker-with-shovel`, `cyclist`, `dancer`, `doctor-with-clipboard`, `e-scooter-rider`, `elderly-person-knitting`, `elderly-with-cane`, `hiker-with-trekking-poles` at this takeover (`barista`, `child-on-kick-scooter`, `courier-with-hand-truck` by the earlier fixers of the batch). No manifest, engine, suite, test or fixture was edited.

In `U/🧰️matrix/MH-1/` (`U` = this ticket folder): created `carry.py`, `land4.sh`, `retry.sh`, `prints.ts`, `tests.sh`, `strip.py`, `plan/basketball-player.loft5.json`, `plan/cyclist.loft2.json`, `plan/cyclist.loft3.json`, `ops/cyclist.json`; updated `ops/dancer.json`, `ops/elderly-person-knitting.json`, `ops/basketball-player.json`, `ops/doctor-with-clipboard.json`, `ops/elderly-with-cane.json` (settled again by the solver), `proof.sh`, `prove.sh`, `big.sh` (reservations of MM: 800 MB for the matrix of a person). `ops/<id>.json` is the reproducible chain of each asset (`fit.ts <id> --ops ops/<id>.json`, the last step after the generators).

Updated `U/multiview-MH-1.md`. Removed `U/🗑️generated/matrix/MH-1/` (backups, trials, logs, sheets) after this report was written.

## 6. Open

Nothing. Reported tier only (not failing): group drift on bones with several shapes, 8 to 1678 entries per asset in the scan.
