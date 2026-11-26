FINAL

# 🧍️ Multiview Fixer MH-6: Eleven People (final, 2026-11-25 22:25)

Batch `🧰️matrix/batches/MH-6.tsv`. Paths as in `📋️multiview-brief.md` (`R`, `S`, `T`, `U`); scripts in `U/🧰️matrix/MH-6/`, method of `multiview-MR.md` section 4, lofts by the recipe of `multiview-MX-2.md` (D54). Four chats worked on the batch (24th; 25th 08:52, 16:15, 19:00). This is one report over all of them. "Gate" is `MR/gate.ts --twins`: the matrix's own `geometryFindings` over every pose, clip instant and transition at the four authored scales.

## 1. Result

**11 of 11 assets are clean with every proof of the brief, read in logs that ran to their end on the file that is in the catalog now. Nothing is open.**

| Asset | File (sha1, 12) | Hard findings before (gate, 24th, claimed: the logs of that day are gone) | Gate now | Matrix now (252 cells, `--fresh`) | Matrix ended |
|---|---|---|---|---|---|
| `teenager-with-headphones` | `21a1df8cbf5e` | 162 | 0 | 0 fail, 0 unavailable; 133 ok, 119 group-drift | 20:31 |
| `tennis-player` | `2f293f0fdcc0` | 184 | 0 | 0 fail, 0 unavailable; 231 group-drift, 21 range-end | 21:42 |
| `tour-guide` | `623c5db0a6f9` | 170 | 0 | 0 fail, 0 unavailable; 70 ok, 182 group-drift | 20:15 |
| `traveller-with-rolling-suitcase` | `32cc2fef3427` | 4687 | 0 | 0 fail, 0 unavailable; 105 ok, 105 group-drift, 42 range-end | 20:41 |
| `waiter-with-tray` | `f3c911286a78` | 1025 | 0 | 0 fail, 0 unavailable; 63 ok, 147 group-drift, 42 range-end | 22:17 |
| `walking-toddler` | `68abfa9bdc63` | 218 | 0 | 0 fail, 0 unavailable; 98 ok, 133 group-drift, 21 range-end | 19:26 |
| `waving-person` | `5ec04069bcd1` | 529 | 0 | 0 fail, 0 unavailable; 70 ok, 182 group-drift | 20:50 |
| `wheelchair-user` | `87db8564b984` | 4928 | 0 | 0 fail, 0 unavailable; 98 ok, 154 group-drift | 20:03 |
| `worker-carrying-box` | `e0c96d865d65` | 2396 | 0 | 0 fail, 0 unavailable; 252 group-drift | 21:55 |
| `worker-with-jackhammer` | `7259c7b150fd` | 829 | 0 | 0 fail, 0 unavailable; 189 group-drift, 63 range-end | 20:57 |
| `worker-with-pallet-jack` | `fe2a46d3963a` | 558 | 0 | 0 fail, 0 unavailable; 140 ok, 112 group-drift | 22:00 |

`group-drift` and `range-end` are the reported tier of the matrix, not `fail`; they were not worked on.

## 2. Proof per asset (all on 2026-11-25 between 19:17 and 22:17, on the files of the table)

The same lines hold for each of the eleven assets; every one was read in `🗑️generated/matrix/MH-6/proof/<id>.out` (small proof and matrix, each with "file unchanged during the proof / the matrix: yes") before that folder was deleted:

- `gate exit 0 hard findings: 0`
- `scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings` (`T/🔎️scan.ts --language en <id>`)
- `<id> silhouette: OK`, `<id> validate: 0 issues`, `<id> three-views sweep: 0 findings` (`MR/measure.ts`)
- `check: ✓ <id> (1/1)` (`S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>`)
- `matrix exit 0 [matrix] 252 cells: … 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing` (`T/📊️matrix.ts --language en --out <dir> --jobs 1 --fresh --assets <id>`; cells as in the table)

The real tests, one name filter over the eleven ids, each suite in its own grant (`vt-chain.sh`, 21:27 to 22:06, after the last change of every file):

- `🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 33 passed | 807 skipped (840); peak 476 MB`
- `🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 11 passed | 374 skipped (385); peak 1136 MB`
- `⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24); peak 1163 MB` (the fixture has no case for an asset of this batch, as before)
- `✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 75 skipped (85); peak 187 MB`
- `🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 32 passed | 258 skipped (290); peak 1313 MB` (3 per asset, 2 for `walking-toddler`, as before)
- `🧊️three-views exit 0  Test Files 1 passed (1); Tests 22 passed | 753 skipped (775); peak 1346 MB`

The peaks are the largest process of `/usr/bin/time -l` for a filter over eleven people; they lie inside MM's figures of 20:20.

Every authored scale prints: the matrix has no `unavailable` cell for any asset. `MC/prints.ts` on the eleven files (20:15): the four keys 1:50, 1:100, 1:200, 1:500 print for all eleven; five assets are refused at an END of a valid range only (`feature` 0.16 to 0.199 mm of 0.2 mm: `tennis-player` and `walking-toddler` at 1:1000, `traveller-with-rolling-suitcase` and `waiter-with-tray` at 1:350, `worker-with-jackhammer` at 1:350 and 1:1000). These are the `range-end` cells of the matrix; they were so before the repairs of this batch touched those bands and are not a `fail`.

The chain is reproducible: `OUT=<copy> land.sh <id>` on a copy of each of the eleven catalog files changes nothing (`fit.ts` 0 changed, `bodies.ts` 0 changed, `loft.ts` 0 views changed; 20:15, `tour-guide` 22:17).

Sheets opened in the fourth chat (product sheet side, front, plan, line and solid, of the trial copies that are byte for byte the landed geometry): `tennis-player` 1:50 `ready-stance` against the catalog sheet before, and 1:500 `ready-stance`; `worker-carrying-box` 1:50 `carrying-box-low`; `worker-with-pallet-jack` 1:100 `pulling` before and after. Arms, hands and shirt read the same in the three views; nothing is uglier than before. The sheets of the earlier repairs were opened by the chats that made them (their list is in `🧰️matrix/MH-6/report-predecessor-0002.md.txt`, claimed); `tour-guide` 1:50 was opened by MX-2 (`multiview-MX-2.md`).

## 3. What was changed, per asset

All repairs are ops files in `🧰️matrix/MH-6/ops/`, landed by `land.sh <id>`: `<id>.json` (`fit.ts`, until a run changes nothing), then `<id>.bodies.json` (`bodies.ts`: a part redrawn in the three views from the volume solids of its band as prisms or one body of revolution), then `<id>.lofts.json` (`MX-2/loft.ts`: a part drawn as `loft` shapes from its volume solids). No elevation outline of a part that was not redrawn, no pose and no declared dimension was changed.

| Asset | Defect (measured before) | Repair |
|---|---|---|
| `walking-toddler` | plan `shirt-core` at 1:50 and 1:100 a slab that did not follow the leaning trunk (20 mm, tolerance 17.4); plan shoes at 1:100 to 1:500 longer than the elevation shoe (30 mm) | `extrude` of the plan core; `plan-sweep` on four shoes |
| `teenager-with-headphones` | flat plan neck at four scales (36 mm); plan head of 1:500 (46 mm); `hand-r-close-p`, a 3 mm tracer plate that stuck 62 mm out of the hand | `plan-revolve` on the necks; `extrude` on the 1:500 head; `remove` of the plate |
| `waiter-with-tray` | front necks (65 mm); right sleeve at 1:50 a 20 mm plate in front and plan (113 mm); `torso-fill-p` bound to the thorax (38 mm); flat shoes at 1:200 and 1:500 (116 mm, `ball-*` 74 mm off the foot); flat plan waistcoat at 1:200 | `front-sweep`, `thicken`, `rebind` to `pelvis`, `extrude` on four shoes, `core` |
| `wheelchair-user` | contacts `patch-rear`, `patch-front` at z 0 between the wheels (4,627 anchor findings); flat trunk at 1:500 | `anchor-split` (D45): `patch-rear-l/-r` z ±0.28, `patch-front-l/-r` z ±0.215, the contacts of poses and clips follow; `front-sweep`, `plan-prism`, `core` |
| `worker-with-pallet-jack` | flat thighs, shanks, feet at 1:100 to 1:500 (63 mm); `roller-contact` and `steer-contact` on parts the coarse bands did not draw (107 mm) | `front-sweep`, `plan-revolve`, `extrude`; `carry` and `carry-solid` (load rollers drawn and modelled at 1:200 and 1:500); `copy` with `create` (steer wheels in the 1:500 front). Third chat, 17:25: at 1:100 both arms (`sleeve-*`, `forearm-*`) and the plan jacket as lofts, after the engine began to read `lateralScale` and the plan guard of 1:100 read 34.0 mm (now 15.0 mm) |
| `worker-with-jackhammer` | flat thighs and shanks (136 mm); the left hand of 1:50 a tracer plate per view (64 mm) | `front-sweep`, `plan-revolve`; both hands of 1:50 as bodies from the volume solids (`bodies.ts`) |
| `worker-carrying-box` | flat thighs, shanks, necks (233 mm); flat plan pelvis at 1:500; flat plan trunk at 1:100; 1:500 front helmet; at 1:50 arms and hands tracer plates (98 mm) | `front-sweep`, `plan-revolve`, `plan-prism`, `core`; hands of 1:50 as bodies; third chat, 17:25: both upper arms, forearms and glove cuffs of 1:50 as lofts (front guard 16.9 → 9.4 mm, plan 16.9 → 10.9 mm) |
| `traveller-with-rolling-suitcase` | every limb flat at four scales (182 mm), flat feet, left arm and hand flat; `case-foot` one contact between two feet; plan coat a 3 mm sheet; plan `handle-grip` a plate that lay on the grip in the default pose only (357 mm) | `front-sweep`, `plan-revolve`, `extrude`, `plan-sweep`, `thicken`; `anchor-split` of `case-foot` into `case-foot-l/-r` (z ±0.165); `handle-grip` the prism of its volume capsule (1:50, 1:100) and the elevation body (1:200); the left hand of 1:50 as bodies of its 18 volume solids |
| `tennis-player` | forearms a prism in one view and a body of revolution in the others (64 mm); neck (33 mm); the plan shirt a sheet across the chest (47 mm in `idle-breathing`); left hand three different sweeps (59 mm) | `copy` for forearm and neck; third chat, 17:25: `shirt` at 1:50 and 1:500 as a loft (replaces `shirt-core`), left forearm, hand and fingers of 1:50 as lofts (fingers fitted into one loft of 7 stations) |
| `tour-guide` | right sleeve of 1:50 three tracer plates (168 mm); left sleeve and right hand in the plan | `copy` (this batch); both arms and hands of 1:50 as lofts by MX-2 at 17:13 (`🧰️matrix/MX-2/ops/tour-guide.lofts.json`), which replaced the bodies this batch drew at 11:09. `land.sh` now ends with MX-2's ops for this asset and `ops/tour-guide.bodies.json` is removed, so that the chain reproduces the file MX-2 wrote |
| `waving-person` | front necks (56 mm); left sleeve of 1:50 two different bodies of revolution (42 mm); plan sleeves and forearm of the coarse bands (239 mm); plan `sil-arm-l` at 1:500 a flat blob (338 mm) | `front-sweep`, `copy`; `sil-arm-l` a body of revolution fitted to the volume (plan IoU 0.9259, before 0.9254) |

## 4. What happened in the fourth chat (19:00 to 22:25)

- No process of the batch was left at 19:00. The files landed by the third chat at 17:25 (`tennis-player`, `worker-carrying-box`, `worker-with-pallet-jack`) were compared with its trial copies (identical) and had their proofs in logs that ended; their six test files had never run (the request of 2500 MB waited out).
- A catalog-wide pass of another lane rewrote all eleven files at 19:09:31 (343 asset files in that minute; `asset.tags` from strings to `{ id, title }`) and `waiter-with-tray` once more at 20:35. Compared as parsed documents with the proven copies of seven assets: nothing but `asset.tags` differs. The matrix of `teenager-with-headphones` that ran across the rewrite was void (252 cells "no runner produced this cell"). Every asset was therefore proven again on the present file: small proof and matrix of all eleven after 19:09 (after 20:35 for `waiter-with-tray`), the six test files once at the end.
- At 21:03:31 the lane and the waiting test request were ended from outside by SIGTERM. Both were resubmitted; the tests as a chain of single grants at MM's figures instead of one grant of 2500 MB.
- Queue: at most two requests at a time (one lane of 1000 MB that ran gate, scan, measure, check CLI and the matrices one process at a time, and one test request). A second lane started by mistake at 20:22 while the first was still alive was withdrawn within ten seconds, before it was admitted.

## 5. Open

Nothing in this batch. Not caused here and not worked on: the reported tier (`group-drift`, `range-end`) in the tables above.

## 6. Requests in `U/requests.md`

None filed in the fourth chat. Earlier, both answered by MX at 11:26 and cured since in the assets (lofts, D54): `## multiview MH-6 — evaluate: the union of a prism swept along w throws at one clip instant` and `## multiview MH-6 — tracer plates of the silhouette rounds`.

## 7. Files

Updated (repo-relative):
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<id>/🔣️.json` for the eleven ids of the table (the last write of this batch was 17:25 on the 25th; `tour-guide` last by MX-2 at 17:13; the rewrites of 19:09 and 20:35 are another lane's).
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/multiview-MH-6.md` (this report)
- `…/🧰️matrix/MH-6/land.sh` (fourth chat: lofts ops of MX-2 as the last step where this folder has none)

Created under `…/🧰️matrix/MH-6/` (kept): `fit.ts`, `bodies.ts`, `where.ts`, `edge.ts`, `gaps.ts`, `ext.ts`, `facts.py`, `part.py`, `hard.py`, `pick.py`, `cmp.py`, `lateral-list.py`, `gate-all.sh`, `try.sh`, `try-all.sh`, `var.sh`, `land.sh`, `proof.sh`, `small-proof.sh`, `mx.sh`, `mx-in.sh`, `rss.sh`, `diag.sh`, `pre.sh`, `small.sh`, `look-file.sh`, `lane.sh`, `q.sh`, `q2.sh`, `wait.sh`, `cap.sh`, `vt-all.sh`, `vt-chain.sh` (fourth chat), `report-*.txt`, `report-data.py`, `report-build.py`, and `ops/`: `<id>.json` for the eleven ids, `<id>.bodies.json` for `traveller-with-rolling-suitcase`, `waving-person`, `worker-carrying-box`, `worker-with-jackhammer`, `<id>.lofts.json` for `tennis-player`, `worker-carrying-box`, `worker-with-pallet-jack`.

Removed: `…/🧰️matrix/MH-6/ops/tour-guide.bodies.json` (superseded by MX-2's lofts); `…/🗑️generated/matrix/MH-6` (backups, trial copies, logs, sheets, matrix folders).

Manifest: not edited. Declared dimensions: none changed. Anchors renamed (for the coordinator's sync): `wheelchair-user` `patch-rear` → `patch-rear-l`, `patch-rear-r`, `patch-front` → `patch-front-l`, `patch-front-r`; `traveller-with-rolling-suitcase` `case-foot` → `case-foot-l`, `case-foot-r`. `worker-with-pallet-jack` has two more body solids (`roller-l`, `roller-r`) in `volumes.200` and `volumes.500`.
