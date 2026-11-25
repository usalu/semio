IN WORK

# 🧍️ Multiview Fixer MH-1: Twelve People

Fourth fixer of the batch (takeover of 2026-11-25 18:58). Batch `🧰️matrix/batches/MH-1.tsv`; paths as in `📋️multiview-brief.md` (`S`, `T`, `U`); scripts in `U/🧰️matrix/MH-1/`, tool output in `U/🗑️generated/matrix/MH-1/` (logs named below are relative to it). This file is rewritten after every finished asset. "Gate" is `../MR/gate.ts --twins` (the matrix's own `geometryFindings` over every pose, clip instant and transition), "scan" the authors' `T/🔎️scan.ts --language en`. A line counts only when its log ran to the end; "claimed" marks what a predecessor wrote without a log.

## 1. State per asset

| Asset | Before (gate hard / scan hard) | Now (gate / scan) | Matrix | State |
|---|---|---|---|---|
| `barista` | 442 / – | 0 / clean | 252 cells, 0 fail | clean |
| `construction-worker-with-shovel` | 970 / 51 | 0 / clean | 252 cells, 0 fail | clean |
| `child-on-kick-scooter` | 81 / 12 | 0 / clean | 252 cells, 0 fail | clean |
| `hiker-with-trekking-poles` | 778 / – | 0 / clean | 252 cells, 0 fail (19:56) | tests queued |
| `courier-with-hand-truck` | 1229 / – | 0 / clean | 252 cells, 0 fail (19:28) | tests queued |
| `e-scooter-rider` | 336 / 18 | 0 / clean | 252 cells, 0 fail (19:52) | tests queued |
| `elderly-with-cane` | 665 / 24 | trial 0 | not run | in work |
| `basketball-player` | 159 / 10 | trial 0 | not run | in work |
| `doctor-with-clipboard` | 483 / 22 | trial 0 | not run | in work |
| `elderly-person-knitting` | 606 / 35 | trial 0 / 3 | not run | in work |
| `dancer` | 631 / 21 | trial 170 | not run | in work |
| `cyclist` | – / 74 | no trial | not run | in work |

## 2. Finished assets and their proofs

- `barista` (catalog md5 0d48deb2, written 2026-11-24 22:08, `ops/barista.json`): gate 0 (442 before: `sync-bone-height` 1:50 ×103 57 mm, 1:100 ×105 51 mm, 1:500 forward, height, lateral), scan clean, silhouette OK, validate 0, sweep 0 (`proof/summary.log` 25 05:15), check CLI `✓` 11:01, `🧊️catalog-silhouette` 1 passed, `🧊️three-views` 2, `🌡️catalog-physics` 3, `🫁️breathing-catalog` 3, `✅️asset-validation` 10, `⚡️asset-power` no case; matrix 12:25: 252 cells, 0 fail (98 ok, 133 group-drift, 21 range-end), peak 397 MB.
- `construction-worker-with-shovel` (ff34e5ad, written 25 09:27 by `take.sh`, equal to its measured trial): gate 0 (970), scan clean (51), silhouette OK, validate 0, sweep 0, check CLI `✓`, the six tests as above (breathing 3 passed); matrix 10:19: 252 cells, 0 fail (189 group-drift, 63 range-end), peak 365 MB. Sheets 1:50 to 1:500 opened by the third fixer (claimed). Gate and measures repeated at 17:38 on the engine that reads `lateralScale`: 0 hard, silhouette OK, validate 0.
- `child-on-kick-scooter` (2527f333, written 25 09:27): gate 0 (81), scan clean (12), silhouette OK, validate 0, sweep 0, check CLI `✓`, the six tests green (breathing 2 passed); matrix 10:57: 252 cells, 0 fail (252 group-drift), peak 317 MB.

## 3. In work

- `hiker-with-trekking-poles`: the engine reads `lateralScale` of revolved drawing solids since 12:20 (request "multiview MX-2 — `lateralScale` …"); the file proven on 2026-11-24 (c28158f9) then failed the guard at 1:50 plan (15.1 mm). A partial removal written at 17:24 (1fd8567b) left 4 hard findings and check `✗`. At 19:00 the op `unscale` (last entry of `ops/hiker-with-trekking-poles.json`) removed the remaining 50 members, so the file draws what was proven before the engine read the member: e38accb0, replay of the ops changes nothing, gate 0, scan clean, silhouette OK, validate 0, sweep 0, check CLI `✓` 19:03. Tests and matrix running.
- `courier-with-hand-truck` (d3b58dd4, written 24 18:07): gate 0 at 17:38, scan clean, silhouette OK, validate 0, check CLI `✓` 11:03, tests of 24 18:14 green; matrix running (the run of 12:25 was cancelled).
- `e-scooter-rider`: landed 19:44 (`land4.sh`, 341438d3), replay changes nothing, gate 0, scan clean, silhouette OK, validate 0, sweep 0, check CLI `✓`, matrix 252 cells 0 fail (119 ok, 63 range-end, 70 group-drift), peak 354 MB.
- At 19:09 another worker rewrote `tags` in every catalog document (objects with titles). The backups carry the old tags, so `take.sh` no longer applies; `carry.py` and `land4.sh` land a trial with the members other workers changed kept. The matrix of the hiker that ran across that write (19:15) had no cells and was repeated.
- The others: see the table.

## 4. Requests filed

None so far.
