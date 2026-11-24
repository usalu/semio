# 🔭️ Multiview Batch MB-2 (2026-11-24)

18 assets whose main finding was a band copy. Tools and ops: `🧰️matrix/MB-2/` (`apply.ts` applies `ops/<id>.json`, written by `make/<id>.py`; `withdraw.py`; `allowlist.json`). Measured with `🧰️matrix/catalog-scan.ts` (the suite has no `🔎️scan.ts`) and `🔭️multiview-coverage/📊️matrix.ts --jobs 1 --assets <id>`.

## Result

| | Assets |
|---|---|
| Clean in scan and matrix (no `fail` cell) | 5: `used-extendable-dining-table`, `sorption-wheel`, `patio-heater`, `used-sandpit-digger`, `used-climbing-net-pyramid` |
| Clean except band copies that are one drawing by nature, entered in `allowlist.json` (D47), pending the coordinator's merge | 11: `bicycle-stand`, `heat-pump-outdoor-unit`, `plate-heat-exchanger`, `solar-thermal-collector`, `saguaro-cactus`, `used-coffee-table`, `used-folding-bookcase-ladder`, `used-shoe-cabinet`, `used-tv-sideboard`, `used-wall-unit`, `storage-crate-stack` (this one also has an open `language-geometry` finding, see below) |
| In-band view of a body of revolution, entered in `allowlist.json` under `derivedViewAllowlist`, pending | 2: `tealight` (also plan band copy, allowlisted), `used-bollard-light` |

All synchrony and anchor findings of the batch are gone: scan of 2026-11-24 17:25 has no `sync-*`, `sync-bone-*`, `anchor-landmark` or dimension finding in any of the 18 assets (before: 5 assets, 80 findings). The moments cue of three assets (`storage-crate-stack`, `tealight`, `used-bollard-light`) is still reported by the scan script; the matrix, which carries the shared-code repair, no longer fails on it.

## Proof

- `validateAsset`: 0 issues for all 18 (before: 0).
- Check CLI `✓` for the 15 changed or touched assets; the 3 untouched (`saguaro-cactus`, `storage-crate-stack`, `used-folding-bookcase-ladder`) were `✓` in the baseline and their files did not change.
- `🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power` with the name filter: every asset passes exactly as in the baseline run of the same morning (15 assets after the repair, 18 before; 105 and 126 result lines, none failing).
- Print: every authored scale and both ends of every range resolve as `authored` under the default print profile (`try.ts`).
- Sheets of the changed views were rendered and looked at (`look.sh`); the turned coffee table (pose `turning-key`, 1:100) reads right in the three views.

## Matrix per asset (cells of the asset: ok / group-drift / unavailable / fail)

| Asset | Before (scan) | After (matrix) | Remaining `fail` |
|---|---|---|---|
| used-coffee-table | copy front 100=200; 4 `sync-height`, 11 `sync-bone-*`, 32 anchors | 238 / 0 / 0 / 14 | plan 200=500 (allowlisted; the two plans became equal when the frame turned into one solid, the slab lies under the shelf: covered) |
| used-extendable-dining-table | copy front 50=100, plan 200=500 | 252 / 0 / 0 / 0 | none |
| used-folding-bookcase-ladder | copy front 200=500 | 210 / 28 / 0 / 14 | front 200=500 (allowlisted, asset not changed) |
| used-shoe-cabinet | copy plan 200=500; 12 anchors | 133 / 105 / 0 / 14 | plan 200=500 (allowlisted) |
| used-tv-sideboard | copy plan 200=500; 3 `sync-forward` | 42 / 196 / 0 / 14 | plan 200=500 (allowlisted) |
| used-wall-unit | copy plan 100=200=500; 4 `sync-forward`, 4 anchors | 56 / 182 / 0 / 14 | plan 200=500 (allowlisted; 100 against 200 now differs only under the cornice: covered, entered in the allowlist as well) |
| heat-pump-outdoor-unit | copy plan 50=100=200=500 | 238 / 0 / 0 / 14 | plan 200=500 (allowlisted) |
| plate-heat-exchanger | copy plan 100=200; 9 anchors | 203 / 0 / 21 / 28 | plan 100=200 (allowlisted) |
| solar-thermal-collector | copy front and plan 50=100; 12 anchors | 70 / 154 / 0 / 28 | front and plan 50=100 (allowlisted) |
| sorption-wheel | copy plan 50=100=200 | 231 / 0 / 21 / 0 | none |
| bicycle-stand | copy front and plan 50=100=200 | 210 / 0 / 0 / 42 | all six (allowlisted, asset not changed) |
| patio-heater | copy plan 100=200 | 252 / 0 / 0 / 0 | none |
| storage-crate-stack | copy front 50=100, plan 200=500; cue | 196 / 0 / 0 / 56 | 28 band copy (allowlisted, asset not changed), 28 `language-geometry` ("the de drawing has other shape geometry than the English one", poses high, half, low; not in the batch file, not repaired) |
| tealight | copy plan 50=100=200; in-band front; 6 `sync-bone`; cue | 133 / 28 / 21 / 70 | 35 band copy and 35 in-band (both allowlisted, asset not changed); the `sync-bone` findings went with the twin rule of the suite |
| used-bollard-light | copy plan 200=500; in-band front 200, 500; cue | 196 / 42 / 0 / 14 | in-band front (allowlisted) |
| used-climbing-net-pyramid | copy plan 50=100 | 252 / 0 / 0 / 0 | none |
| used-sandpit-digger | copy front 50=100 | 252 / 0 / 0 / 0 | none |
| saguaro-cactus | copy plan 200=500 | not finished: the matrix of this asset ran into the 25 minute limit | plan 200=500 by the scan (allowlisted, asset not changed) |

## What was changed

Visible redraws (the finer band shows what the coarser cannot print):

- `used-extendable-dining-table`: front 1:50 gets the chair scuffs on the end apron and the near leg and the folds of the shim (as its side elevation has them); plan 1:200 gets the broken corner of the right top (38 x 28 mm).
- `sorption-wheel`: plan and front 1:50 draw the two seams of the 16 mm belt strip of the casing band, 1:100 draws the strip as one line, 1:200 stays plain.
- `heat-pump-outdoor-unit`: plan 1:50 gets the four hinge knuckles behind the cover, 1:100 the two of the cover.
- `patio-heater`: plan 1:100 gets the stiffening rib of the dome as one circle.
- `used-bollard-light`: plan 1:500 drops the sand ring and the flange ring (0.1 and 0.08 mm on paper), which 1:200 prints.
- `used-sandpit-digger`: front 1:50 gets three drying checks in the two old courses.
- `used-climbing-net-pyramid`: plan 1:50 gets the seven frayed rope covers that its two elevations draw, at the places those give. Runs after the R2-O rope-end ops and touches none of their shapes.
- `used-coffee-table`: front 1:100 draws the four legs and four aprons apart where 1:200 keeps one portal (part of the solid repair below).

Synchrony and anchors:

- `used-coffee-table`: top, shelf and frame are solids in front and plan of all four scales (59 ops); extents agree in every pose, the foot contacts lie on the legs.
- `used-wall-unit`: every plan shape of the tilting centre cabinet is a solid (12 ops).
- `used-tv-sideboard`: the side elevation of 1:100, 1:200, 1:500 gets the far flap door (covered at rest, 0.38 m ahead when open).
- `used-shoe-cabinet`: plan 1:50 and 1:100 get the carcass block under the top (covered at rest; the tilted cabinet now has a body in plan and the foot contacts lie on it).
- `plate-heat-exchanger`: the lumped base contact becomes four contacts, one per 2 mm leg of the two channel rails (D45), with poses, clips, supports, the four support pads and the assumption text following.
- `solar-thermal-collector`: the lumped base contact becomes one per rail at z ±0.4375 (D45), likewise.

Withdrawn after the coordinator's ruling (`withdraw.py`): forced differences of the first pass in `bicycle-stand`, `tealight`, `used-tv-sideboard` (plan 1:500), `used-shoe-cabinet` (plan 1:500), `heat-pump-outdoor-unit` (plan 1:500); these views are back to their state before the batch. A wick in the plan of the tealight at 1:50 was also withdrawn because it made the band unprintable (0.17 mm against 0.2 mm).

## Open

- `allowlist.json` waits for the merge: 15 `bandCopyAllowlist` entries and 2 `derivedViewAllowlist` entries, each with its reason in English and German.
- `storage-crate-stack` `language-geometry` (28 cells) is not a finding of the batch file and was not looked into.
- `saguaro-cactus`: no matrix result.
- At 13:22:08 the catalog documents of `bicycle-stand`, `tealight`, `used-tv-sideboard` and `used-shoe-cabinet` were rewritten by something else with the content they had about twenty minutes earlier (before the withdrawal). They were restored at 17:20 and proven again; see `requests.md`.

## Files

Updated (13 catalog documents, under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/<category>/<id>/🔣️.json`): used-coffee-table, used-extendable-dining-table, used-shoe-cabinet, used-tv-sideboard, used-wall-unit, heat-pump-outdoor-unit, plate-heat-exchanger, solar-thermal-collector, sorption-wheel, patio-heater, used-bollard-light, used-climbing-net-pyramid, used-sandpit-digger. Touched and restored unchanged: bicycle-stand, tealight.

Created under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `multiview-MB-2.md`; `🧰️matrix/MB-2/` with `allowlist.json`, `apply.ts`, `app.sh`, `cells.py`, `copies.py`, `diff.py`, `dump.py`, `env.sh`, `files.ts`, `ids.py`, `lib.py`, `look.sh`, `matrix.sh`, `moves.py`, `paper.py`, `prove.sh`, `rings.ts`, `scan.sh`, `show.py`, `sol.sh`, `solids.ts`, `try.sh`, `try.ts`, `validate.ts`, `view.py`, `vt.sh`, `withdraw.py`, `x.sh`, `make/<id>.py` (13) and `ops/<id>.json` (13). Updated: `requests.md`. Removed: `🗑️generated/matrix/MB-2`.
