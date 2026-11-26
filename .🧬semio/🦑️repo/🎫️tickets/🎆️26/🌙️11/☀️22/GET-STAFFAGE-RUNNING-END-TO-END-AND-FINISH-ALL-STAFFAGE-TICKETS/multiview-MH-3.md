FINAL

# 🧍️ Multiview Fixer MH-3: Twelve People

Batch `🧰️matrix/batches/MH-3.tsv`, five fixers in a row (2026-11-24 13:39 to 2026-11-25; the fifth from 19:00, Fable 5.2). Paths as in `📋️multiview-brief.md` (`R`, `S`, `T`, `U`); scripts in `U/🧰️matrix/MH-3/`, method of `multiview-MR.md` section 4, lofts by the recipe of `multiview-MX-2.md` (D54). Every figure below was read in a log that ran to its end; what was not run says so.

## 1. Result

**12 of 12 assets are clean with every proof of the brief: scan and gate without a hard finding, matrix without a `fail` cell, check CLI `✓`, guard, validation, sweep, print suitability and the name-filtered tests as before.** Nothing is open and no request was filed. The last two (`person-on-crutches`, `person-pushing-bicycle`) were written and proven by the fifth fixer on 2026-11-25 between 19:02 and 22:13; `person-eating-standing` was found written and fully proven by its predecessor (logs of 17:38 and 17:54 read to their end).

| Asset | Scan of 11-24 (sync hard / reported; anchors) | Matrix now (252 cells) | Seconds |
|---|---|---|---|
| `person-carrying-shopping-bags` | 1 / 134; 4 extent; anchors 18 | 0 fail: 63 ok, 168 group-drift, 21 range-end | 1,855 |
| `person-checking-watch` | 0 / 6 | 0 fail: 133 ok, 119 group-drift | 2,768 |
| `person-cooking` | 0 / 2; 1 extent | 0 fail: 154 ok, 98 group-drift | 3,606 |
| `person-drinking-coffee` | 8 / 45; 3 extent | 0 fail: 98 ok, 133 group-drift, 21 range-end | 1,245 |
| `person-eating-standing` | 1 / 10; 3 extent | 0 fail: 70 ok, 182 group-drift | 218 |
| `person-hanging-laundry` | 2 / 37 | 0 fail: 189 group-drift, 63 range-end | 995 |
| `person-in-rain-poncho` | 0 / 68; 18 extent; anchors 24 | 0 fail: 252 group-drift | 656 |
| `person-in-winter-coat` | 2 / 32; 4 extent; anchors 18 | 0 fail: 210 group-drift, 42 range-end | 1,100 |
| `person-on-crutches` | 0 / 12; 3 extent; anchors 8 | 0 fail: 231 group-drift, 21 range-end | 354 |
| `person-pushing-bicycle` | 4 / 55; anchors 284 | 0 fail: 196 ok, 14 group-drift, 42 range-end (before: 189 fail, 21 ok, 42 range-end) | 2,932 |
| `person-queuing` | 2 / 8; anchors 1 | 0 fail: 63 ok, 126 group-drift, 63 range-end | 1,567 |
| `person-signing` | 5 / 72 | 0 fail: 133 ok, 119 group-drift | 2,464 |

`group-drift` and `range-end` are the reported tier of the matrix (bones with several shapes, band ends), not failures. The matrices before 17:48 ran on the engine of their hour, the last three (`person-eating-standing`, `person-on-crutches`, `person-pushing-bicycle`) on the engine of 17:48; none was repeated for an engine change (second takeover, point 2).

## 2. Proof per asset

The proof of an asset is: (a) `gate.ts --twins` of MR over every pose, clip instant and transition at the four scales, 0 hard findings; (b) the authors' scan `T/🔎️scan.ts --language en`, clean; (c) silhouette guard, `validateAsset` 0 issues, three-views sweep 0 findings (`MR/measure.ts`); (d) every authored scale suitable for print (`suit.ts`); (e) one vitest process over `🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power` with the name filter of the asset; (f) check CLI `--asset <id>` `✓`; (g) `T/📊️matrix.ts --assets <id>` to its end, 0 fail. Logs in `🗑️generated/matrix/MH-3/proof/` (deleted with this report, see section 6).

| Asset | (a) gate subjects at 1:50 | (b) to (d) | (e) tests of the asset | (f) | (g) |
|---|---|---|---|---|---|
| `person-carrying-shopping-bags` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 11-25 14:42 |
| `person-checking-watch` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 11:36 |
| `person-cooking` | 0 hard (76) | clean, OK, 0, 0, suitable | 11 of 11 | `✓` | 0 fail, 12:38 |
| `person-drinking-coffee` | 0 hard (94) | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 17:37 |
| `person-eating-standing` | 0 hard (104) | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 17:54 |
| `person-hanging-laundry` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 17:02 |
| `person-in-rain-poncho` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 14:10 |
| `person-in-winter-coat` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 13:59 |
| `person-on-crutches` | 0 hard (138; again at 21:24 on the file with the new tags) | clean, OK, 0, 0, suitable (21:26) | 9 of 9 | `✓` | 0 fail, 19:07 |
| `person-pushing-bicycle` | 0 hard (557) | clean, OK, 0, 0, suitable (21:26) | 9 of 9 | `✓` | 0 fail, 22:13 (md5 `c6e49967` before and after) |
| `person-queuing` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 10:48 |
| `person-signing` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 13:40 |

Shared cases of (e) (`accepts the sample figure`, `validates the manifest`): 10 of 10 in every run. The vitest lines of the first ten assets are those of the runs of 10:17 (6 files passed, 66 tests) and 17:38 (6 files passed, 48 tests); the last two assets ran at 21:26 (5 files passed, 1 skipped because `⚡️asset-power` has no case of these two names; 28 tests).

**Tags.** At 19:09:27 another lane rewrote `asset.tags` of every catalog document (strings to `{ id, title }`). `same-but-tags.py` compared the twelve catalog files with the documents their proofs measured at 20:02: eleven were the same but for `tags`; the twelfth, the bicycle, was written after that with the new tags in it. The same comparison at 22:13, after the last proof: eleven the same but for `tags`, the bicycle identical to its measured document. No proof was repeated for the tags.

**Sheets opened** (product 2D sheet: side, front, plan; line and solid) by the fifth fixer: `person-eating-standing` 1:50 before and after (no visible difference: only point counts changed); `person-on-crutches` 1:500 before and after in `tripod-stand` and after in `crutches-forward`, 1:200 before and after, 1:50 after; `person-pushing-bicycle` 1:50 before and after in `pushing-bicycle`, after in `lifting-onto-stand`, 1:100 after. Nothing uglier or wrong was seen; the bicycle hands now show fingers on the grips in all three views. The sheets of the other nine assets were opened by the earlier fixers (claimed; their PNGs are in `sheets/`, not opened again).

## 3. What was changed per asset

The repair of an asset is its reproducible chain, last step of the asset: `chain.tsv` (an earlier script, two assets), then `ops/<id>.json` with `fit.ts` until a run reports `0 changed something`, then `lofts/<id>.json` with `MX-2/loft.ts` until `0 views changed`. `in-try-apply.sh <id>` runs it on a copy, measures and writes only when clean.

| Asset | Ops |
|---|---|
| `person-carrying-shopping-bags` | thighs and shanks at four scales: front prism swept along v and plan body of revolution from the elevation outline (`front-sweep` 18, `plan-revolve` 16); shoes as prisms over the shoe width (`extrude` 14, `run` 6); forearms 1:500 plan prism (`plan-sweep-u` 2) |
| `person-checking-watch` | neck at four scales and head 1:500 as front prisms (`front-sweep` 5), head 1:500 plan prism |
| `person-cooking` | right sleeve 1:50 one solid in all views (`copy`, `front-sweep`); head and hair front prisms at four scales (8) |
| `person-drinking-coffee` | neck, head, hair and fringe front prisms (15); sleeves at 1:50 to 1:200 and arm silhouettes at 1:500 revolved in all views (`plan-revolve` 11, `copy` 15); 1:500 front left hand is the prism of its side view |
| `person-eating-standing` | neck, head, hair front prisms (14); right sleeve 1:50 revolved in all views; traced two-ring paths at 1:50 reduced (`simplify`: 938 to 51 points, state 4,336 ms to 346 ms) |
| `person-hanging-laundry` | `neck-skin.ts` first; thighs, shanks, sleeves, jacket, hands as front prisms and plan bodies of revolution (`front-sweep` 38, `plan-revolve` 16, `plan-sweep-u` 2, `copy` 3) |
| `person-in-rain-poncho` | thighs (16 ops), head and hood front and plan prisms, poncho plan prism (`plan-sweep-u` 12), shoes as prisms (`extrude` 16, `run` 6) |
| `person-in-winter-coat` | shoes as prisms (`extrude` 14, `run` 6); left hand 1:50 from its elevation (`elevation-extrude`, `copy`) |
| `person-on-crutches` | cast shank 1:200 and 1:500 and upper sleeves 1:50 to 1:200 as front prisms and plan bodies of revolution (16 ops); cast shoe 1:200 as prisms; at 1:500 each crutch is two `member` bars (tube and tip, 18 ops) and the upper sleeves and the shoe are `loft` shapes of their body solids (9 views) |
| `person-pushing-bicycle` | `stand-feet.ts` first: the lumped contact `stand-foot` is split into `stand-foot-l` and `stand-foot-r` on the feet of the stand (D45; 140 failing `anchor-landmark` cells before); 1:100 right hand and forearm front from the elevation, 1:500 right hand plan prism, 1:100 right shoe plan prism, 1:500 left shank elevation from the right one; 1:50 paths reduced; at 1:50 both arms and hands are `loft` shapes of their body solids in all three views (18 views: upper sleeve, lower sleeve, cuff, palm, four fingers, thumb), hand pen 0.18 mm |
| `person-queuing` | neck front prism at four scales (4) |
| `person-signing` | neck front prisms; thighs and right shank at 1:200 and 1:500, sleeves at four scales, forearms at 1:200 and 1:500 as solids in all views (`front-sweep` 10, `plan-revolve` 19, `copy` 17) |

**`person-pushing-bicycle` in numbers.** Matrix of the untouched file (18:34, 2,178 s): 189 failing cells: `anchor-landmark` 140 cells (contact `stand-foot` 78 mm off the drawn stand in front and plan at every scale), `sync-bone-height` 98 cells (forearm and hand right 40 to 41 mm at 1:50 and 1:100, left shank 60 mm at 1:200 and 1:500), `sync-bone-forward` 84 cells (right foot 38 mm at 1:100, left shank 159 mm at 1:200 and 1:500). After: none. Silhouette guard before to after (overlap / largest deviation): 1:50 elevation 0.9788/11.9 mm to 0.9819/11.9, front 0.9877/13.3 to 0.9884/13.3, plan 0.9806/14.6 to 0.9865/13.1; 1:100 plan 0.9635/26.3 to 0.9663/24.0; 1:500 elevation 0.9637/26.4 to 0.9608/26.4, plan 0.9376/46.3 to 0.9365/46.3 (guard OK at every scale).

The hand at 1:50 took seven variants on copies (`var/bicycle-l3` to `l6`, `p13` to `p25`), because the print check and the guard pull against each other: single finger lofts with a 0.10 mm pen stand 0.012 mm apart on paper (gap, 0.15 mm required) and finger 2 is 0.173 mm wide (feature, 0.2 mm required); swelling the fingers by 1 mm closes the gap but leaves 0.191 mm, by 2 mm leaves the little finger at 0.182 mm and takes the front guard to 15.1 mm (limit 15); the four fingers fitted into one loft (6 or 10 stations, exponent free or 2) are suitable but take the front guard to 16.9 mm; a pen of 0.13 mm leaves a gap of 0.009 mm; **a pen of 0.18 mm** (a pen the document already uses for 18 body shapes at 1:50) closes the gap, gives a smallest feature of 0.214 mm and leaves the guard at 13.3 mm. That variant is in the catalog.

**`person-on-crutches` in numbers.** Guard before to after: 1:50 front 0.9950/13.7 mm to 0.9953/13.3, plan 0.9974/1.9 to 0.9917/10.6; 1:100 front 13.5 to 11.9 mm, plan 10.8 to 11.1; 1:200 front 0.9872/15.9 to 0.9841/25.7, plan 0.9859 to 0.9787 (15.7 both); 1:500 elevation 0.9790 to 0.9855, front 0.9768/38.4 to 0.9841/14.3, plan 0.9462/43.0 to 0.9488/42.4 (guard OK at every scale; the 1:50 plan and 1:200 front deviations grew inside their limits, which the report states plainly). The 1:500 sheet prints 1.83 m and 0.68 m where it printed 1.82 m and 0.67 m (the crutch bars and the lofts reach 1 cm further); the matrix has no dimension failure for it and the manifest is the coordinator's to sync.

**Runs that did not end and why.** The write chain of the crutches of 17:36 and the bicycle chain of 19:49 were ended by the queue after 2,594 s and 2,112 s of waiting (`memoryWaitedOut`); the first bicycle matrix (20:41) and the waiting light proof were ended by a SIGTERM from outside this lane at 21:02 together with every other matrix process on the host. Each was started again and ran to its end; no result above comes from a run that was cut.

## 4. Open

Nothing. No asset of the batch has a failing cell, a hard finding or a refused proof.

Not run by this lane, by the rules: the whole `🔭️multiview-coverage` suite and the whole-catalog matrix (they run once at the end on the frozen catalog); the matrices of the nine assets proven before 17:48 were not repeated on the later engine.

## 5. Requests filed in `U/requests.md`

None. Every finding of the batch could be cured in the asset.

## 6. Files

Updated (repo-relative): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<id>/🔣️.json` for the twelve ids of the batch. No manifest, engine, test or fixture was edited by this lane.

Kept in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️matrix/MH-3/`: `fit.ts`, `volume.ts`, `stand-feet.ts`, `neck-skin.ts`, `suit.ts`, `zoom.ts`, `ext.ts`, `probe.ts`, `probe-feature.ts`, `shape-time.ts`, `same-but-tags.py` (created by the fifth fixer), `cells-by-asset.py`, `silcmp.py`, `plan.py`, `compose.py`, `dump.py`, the shell scripts (`qsh`, `qbun`, `in-*.sh`, `apply.sh`, `try.sh`, `proof.sh`, `vt.sh`, `w.sh`, `st.sh`, `wait.sh`, `queue.sh`, `gate-all.sh`, `measure-all.sh`, `look-try.sh`, `prove-many.sh`), `chain.tsv`, `ops/<id>.json` (12), `lofts/person-on-crutches.json`, `lofts/person-pushing-bicycle.json`, `var/` (the variants that were tried), `chains/` (the step lists of every chain, `t5-1.txt` to `t5-8.txt` by the fifth fixer).

Removed: `🗑️generated/matrix/MH-3/` (backups of 11-24, trial copies, logs, sheets, matrix output) after this report was written.
