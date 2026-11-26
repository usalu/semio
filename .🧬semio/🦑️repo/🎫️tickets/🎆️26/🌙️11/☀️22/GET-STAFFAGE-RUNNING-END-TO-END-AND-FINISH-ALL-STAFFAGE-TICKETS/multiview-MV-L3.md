FINAL (2026-11-26 09:35: `field-maple` and `black-alder` clean with all proofs, their depth plates gone, 0 failing matrix cells; `used-bentwood-dining-chairs` written with MU's 1:500 cure, 21 of its 22 failing cells cured, 1 `language-geometry` cell open as before)

# 🌳️ Multiview Fixer Report: Later Round MV-L3 (`black-alder`, `field-maple`, and `used-bentwood-dining-chairs` at 1:500)

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/MV-L3`, `M` = `U/🧰️matrix/MR`, `GEN` = `U/🗑️generated/matrix/MV-L3` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every run went through `repo capacity run` (`W/q.sh`), at most two requests at a time.

## 1. Result

| Asset | Before | Now | Matrix of the file in the catalog | State |
| --- | --- | --- | --- | --- |
| `field-maple` | MV-1 state `bff851e37a2c48a0`, 170 depth plates | `0bc30c2d3c219902` (1,460,564 bytes): no plate, 33 rounded bodies below the authored outlines (`../MV-3/body.ts`, `W/caps.ts`) | 252 cells: 210 ok, 42 reported group drift, 0 fail (350 s) | clean, all proofs |
| `black-alder` | MV-1 state `ee9dcf31fb5b2ca4`, 294 depth plates | `b269763bda84b8f2` (9,686,213 bytes): no plate; 114 lofts of the ellipsoids (`../MV-L1/loft.ts`: every mass in the three views at 1:100 and 1:200, the plan of every mass and front and elevation of `fol-1` to `fol-7` and `fol-top` at 1:50) and 4 rounded bodies (front of `fol-8`, `fol-9`, `fol-10`, `fol-12` at 1:50) | 252 cells: 147 ok, 105 reported group drift, 0 fail (749 s; the MV-1 state: 147 ok, 105 reported, 1063 s) | clean, all proofs |
| `used-bentwood-dining-chairs` | `3ca04a5f576c8296`, 22 failing cells (21 at 1:500 by geometry, 1 `language-geometry`) | `33acb9b3d6bb5c87` (1,106,051 bytes): the 1:500 band of lane MU's `--with-500` | 252 cells: 251 ok, 1 fail (62 s): the `language-geometry` cell at `500:band-max`, side elevation, mode `forces`, pose `in-use` | the 21 geometry cells cured; 1 cell open as before (section 5) |

The request `## multiview MV-L1 — silhouette guard reads 1207 mm …` had no answer at 05:20; MX-3 answered at 05:55 (D70) and landed the pocket rule at 08:34:13. What was written before that (`field-maple`, 06:21) passed the guard of that moment and needs nothing of the rule; `black-alder` and the chairs were measured and written on the live rule.

## 2. `field-maple`: bodies (MV-3's method), written 06:21

- `../MV-3/body.ts` on the backup: 33 bodies `mvb-<bone>` in the place of the 170 plates (front and plan at 1:50, 1:100, 1:200; 5 and 6 parts per scale), none left out; then `W/caps.ts --last 0.01` (rounded shoulders and ends). Ops `W/body/field-maple.json`, applied by `W/make.sh` (method `body`). No anchor had to move (`../MV-3/anchors.ts`: 0).
- **A defect of the rounding step, found on this tree and cured in `W/caps.ts`:** `body.ts` gives a plan section its depth as `rx` and its width as `ry`; `../MV-L2/caps.ts` opens `rx` with the large circle in both views, so in the plan it rounded the depth and left the shelves of the width. In `storm` three such faces stood as straight edges in the hatched see-throughs of the 1:50 plan (`W/pieces.ts --above`: 29.7 mm² on paper with a straight edge of 7.1 mm for `mvb-fol-limb-c3`). With the radii exchanged for plan bodies the faces are gone (sheet opened before and after). Filed for MV-L2 in `requests.md` (their 1:50 plan bodies of `norway-maple`, `rowan`, `multi-stem-birch` were made with the other copy).
- What the bodies show beside the outlines in the wind, measured on the written bytes: worst sheet 1:50 front `storm` 0.16 % of the silhouette (the plates: 0.69 %, straight edges up to 7.8 mm); no island; the longest straight free edge 3.2 mm at 1:50, 2.5 mm at 1:100, 1.1 mm at 1:200 on paper; at rest 0.0 mm beside the outlines at every scale.
- Guard unchanged against the MV-1 state: 1:50 11.4, 11.9, 9.4 mm (limit 15); 1:100 23.7, 18.8, 13.5 mm (30); 1:200 27.0, 15.0, 15.0 mm (60); 1:500 26.5, 41.9, 26.5 mm.
- Sheets opened: `storm` 1:50 front, plan and elevation, 1:100 plan, the plates before (front and plan), `summer-calm` front and plan. The straight cuts at the right flank and the top of the front and the jagged notches of the plan are gone; the openings keep their scalloped edges; a body shows as a smooth rounded edge beside a foreshortened outline. Stated plainly: slivers of that kind remain (7 pieces at 1:50 front, thickness 0.2 to 1.2 mm on paper), and at 1:100 plan one of 0.7 mm at the upper end of the hatched crown.
- Not used for this tree: the lofts. On the live rule the whole-tree loft copy was not measured again; with MX-2's trial module at 06:54 it read 1:50 elevation 19.9 mm (limit 15), and its 1:50 plan draws the hatched crown opening with chords of about 23 mm on paper (a core ellipsoid of 5 m with 32 points per ring).

## 3. `black-alder`: lofts where the guard carries them, bodies for four parts, written 09:05

- Bodies alone (MV-3's method on the whole tree) passed the light proof at 05:57, 06:14 and 06:35, but the sheets were not kept: narrow spurs of a mass got the full depth of the mass and hung as tongues below it in `gale` (front 1:50); with a depth limit (`W/caps.ts --slim 1.5`) the tongues went, yet the lowest mass showed a level edge of about 30 mm on paper and the plan stepped notches.
- The lofts of MV-L1 read as whole lobed masses in every pose opened. Their guard on the live rule, measured per mass and jointly at 1:50 (`W/bisect.sh`, `W/guard50.sh`, 08:39 to 08:53):

  | 1:50, lofts of | elevation | front | plan (limit 15 mm) |
  | --- | --- | --- | --- |
  | no mass (MV-1 state) | 13.1 | 13.1 | 9.4 |
  | each of `fol-8` to `fol-12` alone | 13.1 | 13.1 | 9.4 |
  | `fol-13` alone | 13.1 | 13.3 | 9.4 |
  | every mass but `fol-8` to `fol-13` | 15.0 | 15.0 | 12.0 |
  | every mass but `fol-8` to `fol-12` | 15.0 | **18.8** | 12.0 |
  | every mass | **52.8** | **30.9** | 12.0 |

  So at 1:50 the upper masses pass one by one and fail together (before the rule: `fol-8` 16.9, `fol-9` 18.8, `fol-10` 32.7, `fol-11` 32.7, `fol-12` 48.7, `fol-13` 36.8 mm each alone). 1:100 passes whole (23.7, 23.7, 13.5 of 30 mm); 1:200 passes whole since the rule (front 1207.5 → 43.7 of 60 mm).
- Written: `W/flags/black-alder.lofts` = `50/plan:* 50/front:-fol-8,…,fol-13 50/elevation:-fol-8,…,fol-13 100:* 200:*` picks 114 of the 126 lofts of the search (`W/pick.py` → `W/loft/black-alder.json`; the guard reads every view by itself, so the plan of all fourteen masses is a loft at 1:50). The six upper masses keep their authored front and elevation outline at 1:50; the plates of four of them (`fol-8`, `fol-9`, `fol-10`, `fol-12` front) became bodies (`W/body/black-alder.json`, `--last 0.01 --slim 1.5`), `fol-11` and `fol-13` had none. No anchor moved. Guard of the written file: 1:50 15.0, 15.0, 12.0 mm (limit 15: at the limit, not beyond it; the suite decides, section 6); 1:100 23.7, 23.7, 13.5; 1:200 22.5, 43.7, 15.0; 1:500 as before.
- What the four bodies show: worst sheet 1:50 front `gale`, 2 pieces, 0.09 % of the silhouette, longest straight free edge 1.3 mm on paper (the plates: 27 pieces, 2.65 %, 16.0 mm).
- Sheets opened (the trial copy, equal to the file): `gale` 1:50 front, plan, elevation; `early-wet-snow` 1:50 front; `gale` 1:100 front and plan, 1:200 front; `summer-calm` 1:50 front; the plates before. The vertical ticks, cut lobes and boxed corners of the plates are gone. The lower eight masses are whole lobes; the upper six are their authored outlines, foreshortened, with the authored diamond openings turned. Stated plainly: in the 1:50 `gale` front three ticks of 2 to 3 mm stay at the upper masses where a foreshortened outline meets the mass behind it, and the 1:50 plan keeps the four authored openings of `fol-top` and two pen-width dots MV-L1 named.
- The file grew from 3,427,862 to 9,686,213 bytes (114 lofts, up to about 1,000 sections each); the matrix of the tree took 749 s (1063 s with the plates on the engine of 2026-11-25).

## 4. `used-bentwood-dining-chairs` 1:500 (lane MU's prepared cure), written 08:58

- `W/mu-ops.py` ran MU's `make/used-bentwood-dining-chairs.py --with-500` into `W/ops/` (16 ops, 2 solidify rules, 74 after-ops); `../MU/trial.ts --spec` made the copy; light proof on the copy at 08:43 on the live rule: gate 0 hard at the four scales (103, 103, 102, 103 subjects), guard OK (1:500 elevation 0.9381 / 23.8 mm, **front 0.9471 / 77.7 mm**, plan 0.9954 / 18.6 mm; limits 0.92 and 150 mm; the old guard read the front at 0.905), validator 0, sweep 0, check `✓`. PASS.
- Landing: `W/make.sh` (method `mu`) made `W/ops/used-bentwood-dining-chairs.json` the spec `../MU/ops/used-bentwood-dining-chairs.json` of that lane's chain (the only file of another lane this round changed; the former spec is what MU's make script writes without the flag) and ran `../MU/fix.ts --write`; second run: digest unchanged; file equal to the trial copy.
- Sheets opened at 1:500: `stored` front before and after, `in-use` front and plan. The symbols are 2 mm on paper and read as before: two stacked pairs when stored, four chairs with their bows beside the table square in use.

## 5. Open, stated plainly

- `used-bentwood-dining-chairs`: 1 cell `language-geometry` (`500:band-max`, side elevation, mode `forces`, pose `in-use`: the German drawing has other shape geometry than the English one). It was there before, lies outside the geometry this cure touches and is MU's request 2 (`## multiview MU — two residual matrix findings …`, second item); not cured here.
- `black-alder` 1:50: six upper masses are not lofts in front and elevation because the guard reads the joint drawing 18.8 to 52.8 mm from its reference (table above); which gap of the reference they cover together was not located (`../MX-3/guard-where.ts` at 1 cm names two reference holes of 3 cells at x ±2.05, y 15.29 in the elevation and none in the front; a run at 4 mm did not end in 9 minutes and was stopped). The ops of all 126 lofts are one search away (`W/loft-trial.sh`), the choice is one line (`W/flags/black-alder.lofts`).
- `field-maple` keeps its authored outlines over bodies (no lofts): see section 2.
- The 1:50 guard of `black-alder` stands exactly at its limit (15.0 of 15 mm in elevation and front).

## 6. Proofs

Lines of `GEN/light/<id>.txt` (the trial copy, before the write) and `GEN/proof/<id>.txt`, `GEN/proof/<id>-suites.txt` (the catalog file):

| Proof | `field-maple` (06:10 to 06:34) | `black-alder` (09:00 to 09:31) | `used-bentwood-dining-chairs` (08:43 to 09:06) |
| --- | --- | --- | --- |
| Light proof on the copy BEFORE the write (gate over every subject, gate over the poses, guard, validator, sweep, check CLI `--file`) | PASS, copy `0bc30c2d3c219902` | PASS, copy `b269763bda84b8f2` | PASS, copy `33acb9b3d6bb5c87` |
| `make.sh` twice | `bff851e37a2c48a0` → `0bc30c2d3c219902`, second run unchanged | `ee9dcf31fb5b2ca4` → `b269763bda84b8f2`, second run unchanged | `3ca04a5f576c8296` → `33acb9b3d6bb5c87`, second run unchanged |
| Catalog file against the trial copy (`../MV-3/same.py`) | representations equal, anchors equal | equal, equal | equal, equal |
| Fixer's gate `M/gate.ts --soft` | exit 0, hard findings: 0 | exit 0, hard findings: 0 | exit 0, hard findings: 0 |
| Authors' gate `T/🔎️scan.ts` | 1 clean, 0 hard findings | 1 clean, 0 hard findings | 1 clean, 0 hard findings |
| Prints (`../MC/prints.ts`, key and both band ends) | every authored scale prints | every authored scale prints | every authored scale prints |
| Guard, `validateAsset`, sweep (`M/measure.ts`) | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings |
| Check CLI `--asset` | `✓`, 4 suitability lines | `✓`, 4 suitability lines | `✓`, 4 suitability lines |
| Matrix `T/📊️matrix.ts --jobs 1 --fresh --assets <id>`, to its end | 252 cells: 210 ok, 42 reported, 0 fail (350 s) | 252 cells: 147 ok, 105 reported, 0 fail (749 s) | 252 cells: 251 ok, 1 fail (62 s) |
| `🧊️catalog-silhouette` | 1 passed, 384 skipped | 1 passed, 384 skipped | 1 passed, 384 skipped |
| `🧊️three-views` | 2 passed, 773 skipped | 2 passed, 773 skipped | 2 passed, 773 skipped |
| `🌡️catalog-physics` | 2 passed, 838 skipped | 2 passed, 838 skipped | 2 passed, 838 skipped |
| `🫁️breathing-catalog` | 1 passed, 289 skipped | 1 passed, 289 skipped | no test of this name (290 skipped) |
| `✅️asset-validation` | 10 passed, 75 skipped | 10 passed, 75 skipped | 10 passed, 75 skipped |
| `⚡️asset-power` | no test of this name (24 skipped) | same | same |

"Reported" is the soft tier `group-drift`. The proofs of `field-maple` ran on the guard of before 08:34 (its numbers are far inside the limits and the pocket rule only fills holes both masks agree on; it was not run again on the live rule: not run). The proofs of the other two ran on the live rule.

## 7. Requests and notes in `requests.md`

- `## multiview MV-L3 — for MV-L2: caps.ts rounds the DEPTH of plan bodies …` (06:40, a note with its cure).
- Queue figures measured here: body search with gate and guard of a tree 1233 MB; loft search 1127 MB; `../MX-2/pocket/measure.ts` 1833 MB (both modules); six suites of a tree 2741 MB, of the chairs 2157 MB; a panel of `../MV-L1/zoom.ts` at `DENSITY=500` up to 890 MB.

## 8. Files

Updated in the catalog (repo-relative): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/field-maple/🔣️.json`, `…/🌳️vegetation/black-alder/🔣️.json`, `…/🛋️furniture/used-bentwood-dining-chairs/🔣️.json`. No engine, schema, validator, test, fixture, README or manifest was edited.

Updated in another lane's folder: `🧰️matrix/MU/ops/used-bentwood-dining-chairs.json` (the `--with-500` spec, section 4).

Created in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `multiview-MV-L3.md`; in `🧰️matrix/MV-L3/`:

- chain: `env.sh`, `q.sh`, `make.sh` (methods `body`, `loft`, `mu` by `flags/<id>.method`), `prove.sh`, `suites.sh`, `light.sh`, `trial.sh`, `mixed.sh`, `loft-trial.sh`, `pick.py`, `caps.ts` (copy of `../MV-L2/caps.ts` with the plan rounding corrected and the flags `--along`, `--last`, `--slim`), `rest.ts` (copy of `../MV-L2/rest.ts`), `mu-ops.py`, `chairs-trial.sh`;
- inputs: `body/field-maple.json`, `body/black-alder.json`, `loft/black-alder.json`, `ops/used-bentwood-dining-chairs.json`, `flags/` (`field-maple.caps`, `field-maple.method`, `black-alder.caps`, `black-alder.lofts`, `black-alder.method`, `used-bentwood-dining-chairs.method`), `backup/sha256.txt`;
- measuring and looking: `pieces.ts`, `probe.sh`, `bisect.sh`, `guard50.sh`, `where.sh`, `light-loft.sh`, `pocket-trial.sh`, `sheet.sh`, `zoom.sh`, `looks.sh`, `looks2.sh`, `looks3.sh`, `looks4.sh`, `both.sh`, `alder-body.sh`, `alder-loft.sh` (the last two: trials that were not the way taken).

Updated: `requests.md` (one section appended). Removed: `🗑️generated/matrix/MV-L3` (tool output, trial copies and backups) after this report was written.

The chain of a tree: `../MV-1/make.sh <id>`, then `W/make.sh <id>`; of the chairs: `../MU/fix.ts` with the spec named above. The digests of the three files of before this round: `bff851e37a2c48a0` (`field-maple`), `ee9dcf31fb5b2ca4` (`black-alder`; both in `W/backup/sha256.txt`), `3ca04a5f576c8296` (the chairs); the backups themselves went with `GEN`.
