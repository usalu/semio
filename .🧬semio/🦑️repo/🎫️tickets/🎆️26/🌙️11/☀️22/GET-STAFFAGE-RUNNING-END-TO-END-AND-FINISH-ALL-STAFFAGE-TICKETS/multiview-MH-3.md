IN WORK

# 🧍️ Multiview Fixer MH-3: Twelve People

Batch `🧰️matrix/batches/MH-3.tsv`, five fixers in a row (2026-11-24 13:39 to 2026-11-25; the fifth from 19:00, Fable 5.2). Paths as in `📋️multiview-brief.md` (`R`, `S`, `T`, `U`); scripts in `U/🧰️matrix/MH-3/`, method of `multiview-MR.md` section 4, lofts by the recipe of `multiview-MX-2.md` (D54). Every figure below was read in a log that ran to its end; what was not run says so.

## 1. Result

BICYCLE_RESULT

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
| `person-pushing-bicycle` | 4 / 55; anchors 284 | BICYCLE_CELLS | BICYCLE_S |
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
| `person-on-crutches` | 0 hard (138) | CRUTCHES_LIGHT | CRUTCHES_TESTS | `✓` | 0 fail, 19:07 |
| `person-pushing-bicycle` | 0 hard (557) | BICYCLE_LIGHT | BICYCLE_TESTS | `✓` | BICYCLE_G |
| `person-queuing` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 10:48 |
| `person-signing` | 0 hard | clean, OK, 0, 0, suitable | 9 of 9 | `✓` | 0 fail, 13:40 |

Shared cases of (e) (`accepts the sample figure`, `validates the manifest`): 10 of 10 in every run. The vitest lines of the first ten assets are those of the runs of 10:17 (6 files passed, 66 tests) and 17:38 (6 files passed, 48 tests).

**Tags.** At 19:09:27 another lane rewrote `asset.tags` of every catalog document (strings to `{ id, title }`). `same-but-tags.py` compared the twelve catalog files with the documents their proofs measured at 20:02: eleven were the same but for `tags`; the twelfth, the bicycle, was written after that with the new tags in it. No proof was repeated for the tags.

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

BICYCLE_NOTES

## 4. Open

OPEN_SECTION

## 5. Requests filed in `U/requests.md`

REQUESTS_SECTION

## 6. Files

Updated (repo-relative): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<id>/🔣️.json` for the twelve ids of the batch. No manifest, engine, test or fixture was edited by this lane.

Kept in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️matrix/MH-3/`: `fit.ts`, `volume.ts`, `stand-feet.ts`, `neck-skin.ts`, `suit.ts`, `zoom.ts`, `ext.ts`, `probe.ts`, `probe-feature.ts`, `shape-time.ts`, `same-but-tags.py` (created by the fifth fixer), `cells-by-asset.py`, `silcmp.py`, `plan.py`, `compose.py`, `dump.py`, the shell scripts (`qsh`, `qbun`, `in-*.sh`, `apply.sh`, `try.sh`, `proof.sh`, `vt.sh`, `w.sh`, `st.sh`, `wait.sh`, `queue.sh`, `gate-all.sh`, `measure-all.sh`, `look-try.sh`, `prove-many.sh`), `chain.tsv`, `ops/<id>.json` (12), `lofts/person-on-crutches.json`, `lofts/person-pushing-bicycle.json`, `var/` (the variants that were tried), `chains/` (the step lists of every chain, `t5-1.txt` to `t5-7.txt` by the fifth fixer).

Removed: `🗑️generated/matrix/MH-3/` (backups of 11-24, trial copies, logs, sheets, matrix output) after this report was written.
