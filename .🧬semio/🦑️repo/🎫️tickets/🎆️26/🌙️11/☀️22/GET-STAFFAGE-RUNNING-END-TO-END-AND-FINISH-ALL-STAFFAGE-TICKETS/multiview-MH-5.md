IN WORK

# 🧍️ Multiview Fixer MH-5: Twelve People

Batch `🧰️matrix/batches/MH-5.tsv`. Five fixers worked on it one after the other (2026-11-24 13:39 to 2026-11-25 22:00); this is the one report of the batch. Paths as in `📋️multiview-brief.md` (`S`, `T`, `U`); scripts in `U/🧰️matrix/MH-5/`. This file is rewritten after every finished asset; the first line becomes `FINAL` when every asset is clean with all proofs or listed under "Open".

## 1. State at the takeover (measured 2026-11-25 09:20 to 09:40)

- Catalog files against the backup of 2026-11-24 13:39 (`md5`): five are the backup, never written (`runner-with-blade-prosthesis`, `seated-office-worker`, `seated-reader`, `skateboarder`, `sleeping-person`); seven are byte-identical to the copy this batch landed last (`presenter-at-flipchart`, `schoolchild-with-backpack`, `seated-guitarist`, `standing-desk-worker`, `standing-phone-user`, `standing-violinist`, `surveyor-with-total-station`). No file half-written.
- Official scan of all twelve (`T/🔎️scan.ts --language en`, one run to the end): 5 clean, 7 with hard findings, 529 hard findings: `runner-with-blade-prosthesis` 165, `schoolchild-with-backpack` 2, `seated-office-worker` 25, `seated-reader` 50, `skateboarder` 227, `sleeping-person` 58, `surveyor-with-total-station` 2.
- Proofs of the predecessors that ran to the end on the file that is in the catalog now: `presenter-at-flipchart`, `standing-violinist`, `standing-desk-worker` (all proofs with matrix, 2026-11-24 18:38 to 20:41); `seated-guitarist` (all but the matrix); `standing-phone-user` (gate, measure, six tests, scan at 05:08; check CLI and matrix missing).
- Ops files of the predecessors that were never applied to the catalog: `runner-with-blade-prosthesis` (17 ops), `seated-office-worker` (23), `seated-reader` (22), `sleeping-person` (13), `skateboarder` (16); `schoolchild-with-backpack` has a newer ops file than the landed copy.

## 2. Result per asset

Fifth fixer (Fable 5.2, takeover 2026-11-25 18:59): no process of the batch was alive; lane B of the fourth fixer had ended at 18:25 with `memoryWaitedOut` after 3694 s in the queue, its two landings (`sleeping-person`, `runner-with-blade-prosthesis`) never ran. All twelve catalog files were byte-identical to the copy this batch landed last (`cmp` against `🗑️generated/matrix/MH-5/landed`). "Tests" are the six real tests with the name filter of the asset (`🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power`).

| Asset | Scan before (hard) | Scan now | Matrix (252 cells) | Check CLI | Tests | State |
|---|---|---|---|---|---|---|
| `presenter-at-flipchart` | 14 | clean | 0 fail (168 group-drift, 84 ok; 2026-11-24 19:21) | `✓` | 6/6 (11-24 18:41) | clean, all proofs |
| `standing-violinist` | 16 | clean | 0 fail (112 group-drift, 140 ok; 11-24 20:04) | `✓` | 6/6 (11-24 19:36) | clean, all proofs |
| `standing-desk-worker` | 9 | clean | 0 fail (84 group-drift, 126 ok, 42 range-end; 11-24 20:41) | `✓` | 6/6 (11-24 20:28) | clean, all proofs |
| `seated-guitarist` | 51 | clean | 0 fail (252 group-drift; 11-25 09:41) | `✓` | 6/6 (11-25 00:09) | clean, all proofs |
| `standing-phone-user` | 9 | clean | 0 fail (147 group-drift, 105 ok; 11-25 10:08) | `✓` 10:14 | 6/6 (11-25 05:08) | clean, all proofs |
| `seated-reader` | 50 | clean (12:08) | 0 fail (210 group-drift, 42 ok; 12:46) | `✓` | 6/6 (21:05 to 21:12) | clean, all proofs; written 12:07 (30 ops) |
| `schoolchild-with-backpack` | 2 | clean (14:03) | 0 fail (168 group-drift, 84 ok; 14:15) | `✓` | 6/6 (21:05 to 21:12) | clean, all proofs; written 14:03 (11 ops) |
| `surveyor-with-total-station` | 2 (18 before the batch) | clean (17:49) | 0 fail (189 group-drift, 63 range-end; 17:53; before the loft of the helmet 28 fail) | `✓` | 6/6 (21:05 to 21:12) | clean, all proofs; written 17:48 (26 ops and 1 loft entry) |
| `seated-office-worker` | 25 | clean (17:55) | 0 fail (224 group-drift, 28 ok; 17:57; before the lofts 112 fail) | `✓` | 6/6 (21:05 to 21:12) | clean, all proofs; written 17:54 (39 ops and lofts of the left hand, head and backrest pad) |
| `sleeping-person` | 58 | clean (19:02) | asset process 252 cells `ok`, 0 fail (19:07 to 19:51, 2648 s, peak 362 MB), but the generator counts the asset `missing`: the catalog file was changed under the run at 19:09 (see below); rerun queued 19:53 | `✓` 19:03 | 6/6 (21:05 to 21:12) | written 19:02 (45 ops and 20 loft entries: hands, feet, toes, heads, necks, hair); matrix rerun pending |
| `runner-with-blade-prosthesis` | 165 | clean (19:53) | 0 fail (154 group-drift, 98 ok; 19:54 to 20:20, 1572 s, peak 244 MB; before the loft of the right hand 28 fail) | `✓` 19:54 | 6/6 (21:05 to 21:12) | clean, all proofs; written 19:53 (19 ops and the loft of the right hand with thumb and fingers at 1:50) |
| `skateboarder` | 227 | clean (20:21) | 0 fail (196 group-drift, 56 ok; 20:22 to 20:33; before the lofts 70 fail) | `✓` 20:22 | 6/6 (21:05 to 21:12) | clean, all proofs; written 20:21 (13 ops with the split of the two wheel contacts, D45, and lofts of hood, hood collar, hoodie, trouser seat and right shoe) |

At 19:09:29 another worker rewrote `asset.tags` of all 375 catalog files (slugs became `{ id, title }`). Measured on the twelve files of this batch: nothing else differs from the copies landed here (`retag.py`, JSON comparison). The backups and the landed copies of this batch follow that change (`retag.py`; the untouched backups are in `backup-before-tags` until the folder is deleted), so every later landing keeps the new tags. Matrices that ended before 19:09 ran on the same geometry with the old tags and are not repeated.

## 2a. Loft shapes (recipe of `multiview-MX-2.md`, D54)

Where no prism, plate or body of revolution agreed in all poses, the part is drawn as `loft` shapes from its body solids: `ops/<id>.lofts.json`, applied by `../MX-2/loft.ts` after `ops/<id>.json` (`apply.sh`; a second run changes nothing). Each was proven on a copy first (model gate, silhouette guard, `validateAsset`, three-views sweep, real gate of MR with 0 hard findings, check CLI `✓`), then landed.

| Asset | Lofts | Matrix fail before → after |
|---|---|---|
| `surveyor-with-total-station` | helmet with rim, peak and ridge at 1:50 and 1:100 | 28 → 0 |
| `seated-office-worker` | left hand with thumb and middle finger at 1:50 and 1:100, head with hair and nose at 1:100, head at 1:500 | 112 → 0 |
| `runner-with-blade-prosthesis` | right hand (palm, thumb, fingers) at 1:50, replacing sixteen traced plates | 28 → 0 |
| `skateboarder` | hood and hood collar at 1:50 and 1:100, hoodie and trouser seat at all four scales, right shoe with sole at 1:50 | 70 → 0 |
| `sleeping-person` | hands, feet and toes at 1:50, feet and toes at 1:100, feet at 1:200, head, hair and neck in elevation and plan at 1:50 and 1:100, head at 1:200 and 1:500; the quilt baffles are `add` ops of `ops/sleeping-person.json` | 161 → @@SLEEP@@ |

Sheets opened by the fifth fixer (1:50): `skateboarder` cruising (landed copy and trial), `sleeping-person` curled, `runner-with-blade-prosthesis` blade-push-off: hands, shoes and heads read in all three views, nothing is uglier than before. A whole-limb loft variant of `skateboarder` at 1:50 (`lofts/skate.f.json` of the fourth fixer: model gate 0, guard OK, real gate not run) was not landed; the landed variant is the one whose real gate ran to 0.

## 3. Open

@@OPEN@@

Not repeated, by the rule of the second takeover: the matrices and tests of the five assets finished before 2026-11-25 10:14 ran on earlier engines; the full matrix on the frozen catalog decides. `⚡️asset-power` has no test that matches a person of this batch (24 skipped, as before the batch).

Requests filed in `requests.md`: none.

## 3a. Files

Updated (catalog, `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<id>/🔣️.json`): `presenter-at-flipchart`, `standing-violinist`, `standing-desk-worker`, `seated-guitarist`, `standing-phone-user`, `seated-reader`, `schoolchild-with-backpack`, `surveyor-with-total-station`, `seated-office-worker`, `sleeping-person`, `runner-with-blade-prosthesis`, `skateboarder`. No manifest, engine, test or fixture was edited.

Created or updated in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `multiview-MH-5.md`; `🧰️matrix/MH-5/` (scripts of section 5, `ops/<id>.json` for the twelve assets, `ops/<id>.lofts.json` for the five of section 2a). Removed: `🗑️generated/matrix/MH-5` (with the FINAL report).

## 4. What was wrong and how it is repaired

Every hard finding of these people is one shape whose three pictures are not three views of one solid (causes D, F and G of `multiview-diagnosis.md` 5.6): plates of the silhouette rounds (arms, hands, necks, heads drawn as `extruded` shapes 0.8 to 20 mm thick, each view in another plane of the bone: right in the default pose, three different pictures in every other pose), hidden length (a front or plan outline that runs further into the neighbouring part than the elevation twin) and flat shapes on bones that turn. The silhouette guard pins the default pose of every view to twice the feature size (15 mm at 1:50) and most of these people stand at that limit, so every repair is measured against the guard, shape by shape.

The repair of an asset is `ops/<id>.json`, applied by `fit.ts` to the asset as it was before this batch (last step of its chain; a second run reports `0 changed something`). Ops: `sync` (the three shapes fitted as one solid where a view does not show the difference), `extrude`, `front-sweep`, `plan-revolve`, `copy` (one real solid in two or three views), `copy-scale` (the solid of a neighbouring band), `clip`, `set` and `add` (fitted geometry stated whole), `anchor`, `anchor-split` (D45).

Order of a landing (`step.sh`, inside the worker grant): `fit` (ops on the backup into a trial copy, second run, model gate) → `measure` (silhouette guard, `validateAsset`, three-views sweep of backup and copy) → `gate` (the real gate of MR over every pose, clip instant and transition) → check CLI on the copy → `write` (refused unless the catalog file is the backup or what this batch landed last) → official scan → check CLI → matrix → the six tests.

## 5. Tools (`U/🧰️matrix/MH-5/`)

Of the predecessors (kept, some corrected): `solid.ts`, `mgate.ts` (model gate in seconds), `fit.ts`, `sil.ts`, `auto.py` (search), `freeze.py`, `trial.sh`, `variants.sh`, `anchors.sh`, `land.sh`, `step.sh`, `worker.sh`, `q.sh`, `b.sh`, `chain.sh`, `search.sh`, `try.sh`, `sil.sh`, `rows.py`, `report.py`, `scan-table.py`, `status.sh`, `wait.sh`, `tick.sh`, `hold.sh`, `drain.sh`, `sheets.sh`, `dump.ts`, `shapes.py`, `show.py`, `gate-all.sh`, `auto-all.sh`, `report-template.md`, `report-files.md`, `report-open.md`.

Changed by the third fixer: `q.sh`, `b.sh`, `step.sh` (reservations of the queue rule of 2026-11-25 09:20: 800 MB scans, gates, fits, measures; 1000 MB check CLI; 1200 MB matrix; 2500 MB tests; `write` no longer asks for a grant of its own inside a grant: it waited for the lock its own lane held), `worker.sh` (`guarded`: the worker under `../MM/guarded.sh`, which stops the process tree above its reservation and logs the peak; `JOBS=<dir>` for a second lane; twenty minutes of idle time).

New of the fifth fixer: `retag.py` (follows the tags change of 19:09 in backups and landed copies); `q.sh` and `step.sh` reserve 800 MB for the matrix. New of the fourth: `apply.sh` (fit, then lofts), `lt.sh`, `ltc.sh` (loft trials), `sheetf.sh` (sheets of a trial copy), `until.sh`, `who.ts`, `parts.py`, `quilt.py`.

New of the third: `mx.sh` (matrix of one person under a guarded grant with the peak in the log), `await.sh` (wait for a change in the lanes), `addops.py` (append ops with a reason), `hp.py`, `fs.py`, `span.py`, `reshape.py` (candidate ops for trials: one prism in three views, front outline swept through the elevation thickness, a prism of another band with another span or a scaled profile), `dims.sh` (which op breaks a default-pose dimension), `checks.sh` (check CLI and model gate of trial variants), `where.ts` (where an anchor is in the poses and which bones are nearest: D45).

## 6. Memory and queue

- Fifth fixer: two lanes, never more than two requests. Matrix of a person reserved at 800 MB from 20:05 (figures of MM); peaks printed by the generator: `runner-with-blade-prosthesis` 244 MB (1572 s), `sleeping-person` 362 MB (2648 s). The six tests of one asset in one grant: peak of the process tree 1888 MB (seven assets in 445 s), so 2500 MB holds; the same chain was stopped by the guard at 1354 MB inside a grant of 1200 MB (a measurement, 20:33). Lane B waited 123 minutes for 2500 MB (19:00 to 21:03) and was then cancelled from outside together with lane A (SIGTERM at 21:03); both were restarted and admitted within two minutes.
- Measured peaks of the process tree (`../MM/guarded.sh`, sampled every 3 s): matrix of `seated-guitarist` 392 MB (2194 s), of `standing-phone-user` 357 MB (1171 s); `sil.ts` on three variants of `sleeping-person` more than 882 MB (stopped by the guard at a limit of 800 MB), so the worker lanes reserve 1500 MB.
- From 09:50 on this fixer never had more than two requests in the queue (two worker lanes, each one grant, the jobs of a landing chained inside).
