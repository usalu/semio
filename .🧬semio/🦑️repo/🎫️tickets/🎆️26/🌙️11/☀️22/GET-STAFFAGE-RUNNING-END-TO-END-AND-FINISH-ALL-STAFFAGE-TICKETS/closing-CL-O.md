IN WORK (2026-11-26 14:45: four assets written and proven by gate, scan, guard, check CLI, pins and matrix; the pinned cases and the suites on the written files wait in the queue)

# 🚲️ Closing Lane CL-O: Objects and Vehicles

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-O`, `M` = `U/🧰️matrix/MR`, `GEN` = `U/🗑️generated/matrix/CL-O` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/q.sh`), at most two requests at a time. No test, fixture, rule, engine or manifest was edited.

## 1. Result

| Asset | Pin | Before | After | Catalog file | Matrix of the written file |
| --- | --- | --- | --- | --- | --- |
| `gas-hob` | `📐️plan-dimension`: drawn plan depth equals `dimensions.depthM` 0.52 | plan 1:50: value 0.519 m, span 0.5190 m (knob y −0.259, plate y 0.260) | value 0.52 m, label "0.52 m", span 0.5200 m | `b7543fcfe315fd60` → `47df97e0553eb2f9` (274,300 bytes), written 14:10 | 252 cells: 147 ok, 84 reported group drift, 21 unavailable, 0 fail (6 s, peak 216 MB) |
| `city-bicycle` | `🧊️model-skeleton`: spokes through open nipple seats | 16 failing: `Missing nipple` for the 8 spoke families at 1:200 and 1:500 | 0 failing: 36 nipples per wheel at four scales, length 0.014 m, every bore open | `05eb41c215d0d619` → `089f50b67b8193cf` (1,705,238 bytes), written 14:10 | 252 cells: 231 ok, 21 unavailable, 0 fail (127 s, peak 509 MB) |
| `cargo-bike` | same case | 0 failing (`PINS cargo-bike: OK`): the case was red through `city-bicycle` alone | not changed | `cb5eaa4508bdb5ec`, untouched | not run (file unchanged) |
| `scissor-lift` | `🧊️model-skeleton`: arm bores for the cross pins | 56 failing: 24 sleeves bore 0.020 at 1:50, 24 sleeves bore 0.015 at 1:100 (pinned 0.02025), 8 `Missing bored link` at 1:200 (the arms were plain tubes; the 24 sleeves of 1:200 were missing behind them) | 0 failing: 72 sleeves bore 0.02025 at 1:50, 1:100, 1:200; the arms of 1:200 are bored | `69f9bdcfcb67eb73` → `e9388e92a820bdae` (2,068,406 bytes), written 14:14 | 252 cells: 189 ok, 42 reported group drift, 21 unavailable, 0 fail (106 s, peak 459 MB) |
| `excavator` | `🧊️model-skeleton`: idler pin through its yoke | 3 failing: at 1:200 the 20 bore points of each of the two yokes 0.037 m from the pin radius the case reads (pinned 0.002 m), at 1:500 `Missing idler cross pin` | 0 failing: clearance 0.002 m at 1:50, 1:100, 1:200; the pin exists at 1:500 | `0dcec793a88e7197` → `5cd5e0c17b2bd8a7` (1,017,499 bytes), written 14:16 | 252 cells: 175 ok, 56 reported group drift, 21 range end, 0 fail (427 s, peak 711 MB) |

"Before" and "after" of the model pins are the counts of `W/pins.ts`, a measuring copy of the three cases that reports every failing expectation instead of stopping at the first (the cases themselves: section 4). The matrices equal the cell counts of the earlier lanes for the same assets (`gas-hob` 147 ok and 21 unavailable in `multiview-MP.md`, `excavator` 175 ok, 56 group drift, 21 range end in `multiview-MU.md`).

## 2. What was changed and why (`W/fix.ts <id>`, idempotent, the last step of each asset's chain)

### `gas-hob`: the knob stands 26 mm proud, the dimension stays 0.52 m

- Which side is right: `dimensions.depthM` 0.52 has been declared since the first version (2026-11-03, when the plate itself was 0.52 m deep), the 1:200 drawing and the volumes of 1:200 and 1:500 already reach y −0.260, and 0.52 m is the depth of a table-top domino. The 1:50 and 1:100 knob was one millimetre short (25 mm in front of the fascia at y −0.234). So the drawing was corrected, not the dimension; the manifest needs no sync (`sync-manifest.py` without `--write`: 0 changes).
- 2D, 1:50 and 1:100: `plan-knob` y −0.259 → −0.26 and h 0.025 → 0.026; the `solid` of `knob` in front and side elevation `z0` −0.259 → −0.26.
- 3D: 1:50 `knob-ridge` 10 mm deep instead of 9 (size z 0.009 → 0.01, centre −0.2545 → −0.255) with `knob-pointer` on its face (−0.2591 → −0.2601); 1:100 `knob` stub 26 mm instead of 25. The knob body, its bone and the coarse scales are unchanged.
- Texts: the provenance note said "0.497 m plate and the 25 mm knob in front of it" (0.522 m in sum); it now names what is drawn: the 0.496 m plate and the knob 26 mm proud of the fascia, 24 mm in front of the plate. The five decisions that named the plate depth as 0.497 m say 0.496 m.
- Guard (`M/measure.ts`), before → after: depth of the bounding box 0.5190 → 0.5200 at 1:50 and 1:100; 1:50 front 0.9784 / 11.3 mm → 0.9792 / 9.4 mm, plan 0.9993 → 0.9995; 1:100 front 0.9896 / 4.5 mm → 0.9891 / 5.7 mm; everything else equal.

### `city-bicycle`: the nipples of 1:200 and 1:500

- Cause: the feature rework of 2026-11-23 (`rework-fittings.md`, section 3) left the nipples out at 1:200 and 1:500 ("rim a closed 30 mm band; nipples left out"), while the case pins a bored nipple for every spoke family at every scale and the decisions of both scales still said "headed nipples sit on the actual flange and rim".
- Restored: the eight nipple solids of each scale (six rings of eight vertices, `texture`, so the feature rule admits them as it does at `cargo-bike`) from the catalog state of 2026-11-22 13:42, kept as input in `W/ops/city-bicycle.json`, placed after their spokes, with their thermal (`th-nipple-*`, region `rims-spokes`) and structure references (`st-nipple-*`). The spokes of both scales are the ones those nipples were made for (equal to that state); the heads now sit in the closed rim band.
- Texts: the two decisions that said the nipples are left out say that they keep their bored seats in the rim band.
- Guard before → after: equal at every scale (1:200 elevation 0.9483 / 51.3 mm, front 0.9468 / 15.1 mm, plan 0.9515 / 23.2 mm).

### `scissor-lift`: bushings bored for their pins, bored arms at 1:200

- Cause: "Rework 10" gave the bushings a wall of one feature (bore 0.020 with 7.5 mm at 1:50, bore 0.015 with 15 mm at 1:100) and replaced the arms of 1:200 by plain tubes without bushings. No rule asks for that: design amendment C30 of the 3D ticket keeps the real wall of hollow parts at 1:50 and 1:100 and makes a hollow part solid only where its cavity is below the feature; the 40.5 mm bore, the 47 mm arm bores and the 38 mm cavity of the arm are above the 30 mm feature of 1:200.
- 1:50, 1:100: the 48 sleeves are the pinned bushing again (bore radius 0.02025 for the 0.02 pin, outside 0.0235, which is the radius of the arm bore they sit in).
- 1:200: the eight arms are the bored box sections of 1:100 (polyhedron, three cross bores) in the place of the plain tubes, with the 24 sleeves and their thermal and structure references. 1:500 is unchanged (the 50 mm arm is below the 75 mm feature there, the case does not ask for it).
- Texts: the three "Rework 10" sentences name the real bushing section and the bored arms.
- Proven without a wall-thickness finding: `validateAsset` 0 issues, check CLI `✓`. Guard before → after: equal at every scale (1:200 elevation 0.9969 / 15.0 mm, front 0.9806, plan 1.0000).

### `excavator`: the idler cross pin

- 1:200: the pin was a closed round whose profile began on the axis (`[0, 0]`), so the case, which reads the shaft radius from the first profile point, measured the yoke bore against radius 0. The profile now begins with the shaft radius: `[[0, 0.035], [0.3, 0.035], [0.3, 0], [0, 0]]`, the same closed section.
- 1:500: the pin of 1:50 is 86 mm across its head, above the 75 mm feature, so the case asks for it; it was missing. Added as one round of that head diameter (radius 0.043, 0.305 m long, twice) through hub and yoke arms, with its thermal reference as at 1:200 and one sentence in the decisions. It lies inside hub and yoke arms, so no silhouette moves.
- Guard before → after: equal at every scale.

## 3. Proofs of the written files

| Proof | `gas-hob` | `city-bicycle` | `scissor-lift` | `excavator` |
| --- | --- | --- | --- | --- |
| Light proof on the trial copy BEFORE the write (gate over every subject, gate over the poses, guard, validator, sweep, check CLI `--file`, pins) | PASS, copy `47df97e0553eb2f9` (14:04) | PASS, copy `089f50b67b8193cf` (14:06) | PASS, copy `e9388e92a820bdae` (14:07) | PASS, copy `5cd5e0c17b2bd8a7` (14:04) |
| `fix.ts --write` twice | 24 changes, second run 0 | 52 changes, second run 0 | 134 changes, second run 0 | 5 changes, second run 0 |
| Catalog file against the trial copy | byte for byte | byte for byte | byte for byte | byte for byte |
| Gate `M/gate.ts --soft` | exit 0, hard findings: 0 | exit 0, hard findings: 0 | exit 0, hard findings: 0 | exit 0, hard findings: 0 |
| Authors' scan `T/🔎️scan.ts` | 1 clean, 0 hard findings | 1 clean, 0 hard findings | 1 clean, 0 hard findings | 1 clean, 0 hard findings |
| Prints (`../MC/prints.ts`) | every authored scale prints | every authored scale prints | every authored scale prints | 1 refusal at the range end 1:20 (`mirror-arm-right-2` 0.193 mm), as before this lane (`multiview-MU.md`); no 2D shape of this asset was touched |
| Guard, `validateAsset`, sweep (`M/measure.ts`) | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings |
| Check CLI `--asset` | `✓`, 3 suitability lines | `✓`, 4 suitability lines | `✓`, 3 suitability lines | `✓`, 4 suitability lines |
| Pins (`W/pins.ts`, `W/depth.ts`) | `DEPTH gas-hob: OK` | `PINS city-bicycle: OK` | `PINS scissor-lift: OK` | `PINS excavator: OK` |
| Matrix `T/📊️matrix.ts --jobs 1 --fresh --assets <id>`, to its end | 252 cells, 0 fail | 252 cells, 0 fail | 252 cells, 0 fail | 252 cells, 0 fail |
| `🧊️catalog-silhouette` (name filter) | 1 passed, 384 skipped | 1 passed, 384 skipped | 1 passed, 384 skipped | 1 passed, 384 skipped |
| `🧊️three-views` | 2 passed, 773 skipped | 2 passed, 773 skipped | 2 passed, 773 skipped | 2 passed, 773 skipped |
| `🌡️catalog-physics` | 2 passed, 838 skipped | 2 passed, 838 skipped | 2 passed, 838 skipped | 2 passed, 838 skipped |
| `✅️asset-validation` | 10 passed, 75 skipped | 10 passed, 75 skipped | 10 passed, 75 skipped | 10 passed, 75 skipped |
| `🧊️model-catalog` with the asset as the only selected one (`SEMIO_STAFFAGE_MODEL_ASSETS`): valid volume at every authored scale, watertight scenes with falling budgets, valid file set in every pose, clip, scale and mode | 1 passed, 2 skipped | 1 passed, 2 skipped | 1 passed, 2 skipped | 1 passed, 2 skipped |

The suites ran from 14:51 to 14:57 on the digests named in section 1 (`GEN/proof/<id>-suites.txt`).

## 4. The pinned cases on the written catalog

REDCASES

## 5. Stated plainly

- `cargo-bike` needed nothing; the request listed it because the case names both bicycles and stops at the first missing nipple.
- No sheet was opened. The only 2D change is the knob of `gas-hob`, 1 mm in the model, 0.02 mm on paper at 1:50; the 3D changes of the other three are inside or flush with what was there (guard numbers equal before and after). The 3D views of `scissor-lift` 1:200 (bored arms with bushings) and of the nipples at 1:200 and 1:500 were not rendered.
- `city-bicycle` 1:200 and 1:500 carry nipples that are below the feature of those scales (declared `texture`, as at `cargo-bike`); that is what the pin asks, against the decision of the rework of 2026-11-23.
- The decisions of `scissor-lift` 1:200 still describe the arms as "bars 110 mm deep" in their first sentence, and those of `excavator` 1:500 say the pin "passes through two pierced yoke plates" although the yoke arms of 1:500 have no bore (the case does not ask for one there: the 30 mm plates are below the feature). Both texts are older than this lane and were not rewritten.
- The 1:500 pin of `excavator` is a round of the head diameter (86 mm), 16 mm thicker than the 70 mm shaft: the price of the 75 mm feature.

## 6. Requests and notes in `requests.md`

- `CL-O rewrote gas-hob at 2026-11-26 14:10` under `## closing — gas-hob drawn plan depth …`.
- `CL-O rewrote city-bicycle at 2026-11-26 14:10`, `… scissor-lift at 2026-11-26 14:14`, `… excavator at 2026-11-26 14:16` and the note that `cargo-bike` was not rewritten, under `## closing — nine assets against the detail pins …`.
- Queue figures measured here (`/usr/bin/time -l`, largest process): light proof of four rigid assets 549 MB; write chain with the four matrices 711 MB (the matrix of `excavator`).

## 7. Files

Updated in the catalog (repo-relative): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json`, `…/🚜️vehicles/city-bicycle/🔣️.json`, `…/🏗️machinery/scissor-lift/🔣️.json`, `…/🏗️machinery/excavator/🔣️.json`.

Created in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `closing-CL-O.md`; in `🧰️matrix/CL-O/`: `env.sh`, `q.sh`, `fix.ts` (the repair), `ops/city-bicycle.json` (its input), `pins.ts` and `depth.ts` (measuring copies of the pinned cases), `light.sh`, `first.sh`, `prove.sh`, `after.sh`, `tell.py`, `red.sh`, `suites.sh`, `more.sh`, `final.sh`, `backup-sha256.txt` (digests of the five files before this lane).

Updated: `requests.md` (five lines appended). Removed: `🗑️generated/matrix/CL-O` (tool output, trial copies and backups) after this report was written.
