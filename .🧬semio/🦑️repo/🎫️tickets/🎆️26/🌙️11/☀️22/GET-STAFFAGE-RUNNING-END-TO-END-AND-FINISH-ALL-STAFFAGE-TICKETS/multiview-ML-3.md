FINAL (2026-11-25 22:05; four assets clean with all proofs, `rowan` 1:50 open for the drawing of its wind poses)

# 🌿️ Multiview Fixer Report: Later Round ML-3 (Open Plants of Batch MS)

Batch `🧰️matrix/batches/ML-3.tsv`: `boxwood-ball`, `norway-spruce`, `windmill-palm`, `saucer-magnolia`, `rowan` (`saguaro-cactus` is not part of it and was not touched). Briefs `📋️multiview-brief.md`, `📋️takeover-brief.md`. Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/ML-3`, `GEN` = `U/🗑️generated/matrix/ML-3`, `C` = `S/🗂️catalog/🌳️vegetation`.

## 1. State at the start (19:15)

- No process of this batch existed; `W` and `GEN` were created by this round.
- The five catalog files carry the modification time 19:08 to 19:09 of 2026-11-25 (all 67 plants do: a catalog-wide write of another lane). Backups of the files as found: `GEN/backup/<id>.json`, sha-256 `781b4afa0f82296a` (boxwood-ball), `0137c11a68fc42f2` (norway-spruce), `8332acedd775540d` (windmill-palm), `d7109a2b16a40730` (saucer-magnolia), `56b5d3377cfaa58c` (rowan).
- The fixture already holds `derivedViewAllowlist` entries for the front of `boxwood-ball`, `norway-spruce` and `windmill-palm` without bands (merged from `🧰️matrix/MS/allowlist.json`, kept by D58). This round measures without any allowlist (`W/derived.ts`).
- `rowan` and `saucer-magnolia` were repaired by batch MV-3 after MS (`multiview-MV-3.md`: both claimed clean, logs deleted). This round measures them again and looks at the wind poses.

- Start scan `T/🔎️scan.ts --language en` over the five plants (19:33 to 19:42, one process, to its end, with the allowlist of the fixture as it stands): `[scan] 5 assets: 5 clean, 0 with a hard finding, 0 hard findings`; reported tier: `rowan` 4 (group drift, worst 221 mm `fol-7` 1:200 `gale`), `saucer-magnolia` 2 (165 mm `fol-1` 1:100 `spring-gale`).

## 2. Result per asset

| Asset | Scan (hard) | Matrix of the asset, to its end | Front against side without allowlist | State |
| --- | --- | --- | --- | --- |
| `norway-spruce` | 0 | 252 ok, 0 fail (564 s) | 1:200 0.0239 → 0.1109; 1:500 0.0178 | repaired at 1:200, clean; allowlist proposal 1:500 |
| `boxwood-ball` | 0 | 252 ok, 0 fail (9 s) | 1:500 0.0000 | not changed; allowlist proposal 1:500 |
| `windmill-palm` | 0 | 210 ok, 42 `range-end` (reported), 0 fail (1655 s, peak 610 MB) | 1:200 0.0279 | not changed; allowlist proposal 1:200 |
| `saucer-magnolia` | 0 | 182 ok, 70 reported group drift, 0 fail (132 s) | no finding | not changed, clean |
| `rowan` | 0 | 252 ok, 0 fail (758 s) | no finding | repaired at 1:100 and 1:200, all proofs; **1:50 open** (wind-pose drawing) |

The scan and the matrices ran with the allowlist of the fixture as it stands (three entries without bands); the request below asks MX-2 to replace them. Reservations: MM's figures of 20:05 (1600 MB worker for matrix, measure and check; 2500 MB for the vitest files; measured peaks: scan 653 MB, `measure.ts` 825 MB, vitest with a name filter 1380 MB, sheet render 571 MB).

### `norway-spruce` (repaired at 1:200, written 19:49; allowlist proposed for 1:500 only) — CLEAN

- Measured without allowlist (`W/derived.ts`, front against side elevation, limit 0.03): before 1:50 0.0506, 1:100 0.0660, **1:200 0.0239**, **1:500 0.0178**; after 1:200 **0.1109**, the others unchanged.
- Cause at 1:200: the individual has two leading sprays per whorl in the sagittal plane (bones `spray-w-NN`, `spray-e-NN`); they make the tree 8.48 m wide and 7.41 m deep. The 1:100 body keeps them as capsules beyond the rim of each skirt; the 1:200 body had merged them into the skirt by stretching each skirt of revolution sideways (lateral scale 0.80 to 0.85), so the front was the side squeezed by one factor per tier and the spray bones drew nothing.
- Repair (`W/fix-norway-spruce.ts`, idempotent: second run `unchanged`, same digest): the 15 skirts of `volumes.200.body` go back to their own reach (profile radii and lateral scale of the 1:100 skirt; elevation and plan path scaled about the skirt axis, the front path stays), and the 30 leading sprays of the 1:100 band (capsule solids, thermal references, capsule shapes in the three views) are set into the 1:200 band, painted below their skirt as at 1:100; decision texts of the representation and of the volume follow (50 body solids). A spray is 1.2 to 3 mm thick and 7 to 23 mm long on paper at 1:200.
- 1:500 is not repaired in the asset: there the leading sprays reach about 1 mm beyond the rim and are 0.3 to 0.6 mm thick on paper; the crown is two solids of revolution with an elliptic section. Proposal in `W/allowlist.json`: front, bands `["500"]`, with the physical reason.
- Catalog file: `0137c11a68fc42f2` before, `fbb2bdcbdc609de7` after (byte-identical to the trial copy that was measured).
- Proofs so far (each run to its end, lines in `GEN/proof/norway-spruce.txt`): scan `1 clean, 0 hard findings`; gate over every pose, clip instant and transition `hard findings: 0`; `suit.ts` `prints`; `M/measure.ts` on the identical bytes: silhouette OK (1:200 IoU 0.9993, 0.9990, 0.9971; bounding box 8.4808 × 30.0000 × 7.4112 m against 8.483 × 30 × 7.412), `validate: 0 issues`, three-views sweep 0 findings (before: the same).
- Sheets opened: 1:200 calm before and after, 1:200 storm after, 1:100 calm and storm for comparison. After: the side elevation shows the ragged spray ends beyond every rim, the front stays the smooth narrower cone, the plan shows the four longest spray ends beyond the lowest skirt, as the 1:100 band does. In `storm` the solid plan paints the lifted windward spray of the lowest whorl over its skirt; the 1:100 band did so before (same shapes), it is not new.
- Catalog file `fbb2bdcbdc609de7`, 21:08 to 21:22: check CLI `✓ norway-spruce (1/1)`, 4 suitability lines; `M/measure.ts` on the catalog file: silhouette OK, `validate: 0 issues`, sweep 0 findings; matrix `252 cells: 252 ok, 0 unavailable, 0 fail` (564 s, to its end).
- Vitest files with the name filter (21:29 to 21:31): `🧊️catalog-silhouette` 1 passed of 385, `🧊️three-views` 2 passed, `🌡️catalog-physics` 2 passed, `🫁️breathing-catalog` 1 passed, `✅️asset-validation` 10 passed, `⚡️asset-power` no test of this name (24 skipped), as before.

### `rowan` (repaired at 1:100 and 1:200, written 21:35; 1:50 OPEN)

- State found: scan clean, but the wind poses show what the coordinator does not accept. `../MV-3/visible.ts rowan`: worst sheet 1:50 front `gale`, 12 pieces of depth plates beside the outlines, 5.74 % of the silhouette, 4 slivers, longest straight free edge 8.8 mm on paper (1:100 front `gale` 4.07 %, 7 slivers). At 1:200 the depth slabs of batch MS (93 front and 120 plan shapes) drew broken contour lines in the default pose and ticks, notches and nested rings in `gale` (`W/sheets/rowan.s200.gale.before.png`, `W/sheets/rowan.s100.gale.before.png`).
- Repair (`W/make-rowan.sh`, idempotent: second run same digest): `W/base-rowan-200.ts` sets the 1:200 front and plan back to one flat outline per part (`W/ops/rowan-200-base.json`: the 27 and 23 shapes authored before the slab redraw; elevation and body of 1:200 did not change since); `W/apply-rowan.ts --scales 100,200` removes the 64 depth plates and 36 sheet suffixes of the two bands and sets 57 lofts: the front and plan outline of every foliage mass as the loft of its own ellipsoids, found by `../MV-L1/loft.ts` (used as it is, not copied; `W/search-rowan.sh`, absolute values in `W/ops/rowan.json`; the search ran on a copy without the fruit dots of 1:200, because the tool converts a bone only when it draws one closed body outline). `../MS/make-rowan.sh` is no longer part of the chain.
- Catalog file `56b5d3377cfaa58c` → `445312f3c924a4e6` (2,478,195 → 2,981,911 bytes), byte-identical to the copy that was measured.
- Proofs (each to its end, 21:31 to 21:59): scan `1 clean, 0 hard findings`; gate over every pose, clip instant and transition `hard findings: 0`, and no reported group drift at any scale (before: 4 reported, worst 221 mm); `suit.ts` `prints`; `M/measure.ts`: silhouette OK (1:100 front 19.1 mm, plan 15.5 mm; 1:200 front 16.8 mm, plan 15.0 mm; 1:50 unchanged 7.7 and 10.6 mm), `validate: 0 issues`, sweep 0 findings; check CLI `✓ rowan (1/1)`, 4 suitability lines; matrix `252 cells: 252 ok, 0 fail`; vitest: `🧊️catalog-silhouette` 1 passed, `🧊️three-views` 2 passed, `🌡️catalog-physics` 2 passed, `🫁️breathing-catalog` 1 passed, `✅️asset-validation` 10 passed, `⚡️asset-power` 24 skipped as before.
- Sheets opened: copies with the same lofts at 1:100 and 1:200 in `gale` and at rest (whole lobed masses in front and plan, no cut, no sliver; at rest no visible difference to before at 1:50; `W/sheets/rowan.s100.gale.after.png`, `W/sheets/rowan.s200.gale.after.png` are the written file). Left in these two bands: the tool reports holes it could not fill inside the top mass seen from above (`fol-top` plan, 14 at 1:100, 6 at 1:200; the authored top mass has four such slits at rest) and one in `fol-7` front at 1:100; they show as two dots in the middle of the plan in `gale`.
- **1:50, the coordinator's question (depth plates accepted only if the wind-pose sheets show no straight cut, notch or sliver): they do show them.** `W/sheets/rowan.s50.gale.plates.png` (side, front, plan): stepped edges and slivers at the lower left of the front, double contours and crescents on the windward side of the plan. `W/sheets/rowan.s50.fresh-breeze.plates.png`: the same on the windward edge of the plan, smaller. So 1:50 is the second case: not repaired, request filed, listed under open.
- Why 1:50 is not repaired: with every mass as a loft the 1:50 plan fails the silhouette guard at rest (67.2 mm, the band allows about 15 mm; front 9.4 mm passes). Mass `fol-6` alone is 42 mm from its outline with every option of the tool (`--slack 0`, `--no-fill`, `--step 8`); with `fol-6` put back to its flat outline the plan still measures 16.0 mm, so splitting `fol-6` into two carrier masses was not tried: the rest comes from the other lofts (a bead of sections is not the ellipsoid). The gate on that copy was clean (0 hard, no reported drift), so only the guard stands in the way.

### `saucer-magnolia` (not changed) — CLEAN

- Scan clean (2 reported: `fol-1` 165 mm in `spring-gale` at 1:50 and 1:100); front against side without allowlist 0.0814, 0.0751, 0.0603, 0.0838 (1:50 to 1:500). Matrix 182 ok, 70 reported group drift, 0 fail. The file `d7109a2b16a40730` is the one batch MV-3 proved; its other tests were not run again.
- It carries no depth plate. Sheets opened: `spring-gale` at 1:50 and 1:200: whole masses, no cut or sliver; the four blossoms on top of the crown stand about 0.1 m above the front outline of the top mass at 1:50 (the reported drift).

### `boxwood-ball` and `windmill-palm` (not changed, allowlist proposals) — CLEAN ONCE THE ENTRIES ARE DECIDED

- `boxwood-ball` (`781b4afa0f82296a`): front against side 0.0000 at 1:500 only (one sphere of 0.8 m, 1.6 mm on paper); proposal front `["500"]`. Matrix 252 ok.
- `windmill-palm` (`8332acedd775540d`): 0.0279 at 1:200 only (0.218 m on 7.81 m, carried by a dead leaf beside the stem at 5.4 m; 0.037 to 0.049 in the other bands). The crown of twenty leaves in a spiral round one stem is rotationally symmetric by nature and both views are true views of the 201 solids; no part of the palm tells its sides apart by 0.23 m, so nothing truthful can be added in the asset. Proposal front `["200"]`. Sheet 1:200 opened. Matrix 0 fail; 42 cells `range-end` (reported): the band ends 1:350 and 1:150 do not print under `office-laser` (features 0.171 to 0.192 mm below 0.2 mm), not a multiview finding and not changed here.
- Without the entries (old or new) the fronts fail `derived-view-in-band`: 7 cells each for `boxwood-ball` 1:500, `norway-spruce` 1:500, `windmill-palm` 1:200 (not run without them; the count is the seven modes of one preset).

## 3. Open

1. `rowan` 1:50: depth plates of batch MV-3 stay; wind poses show slivers and stepped edges (`gale` front: 12 pieces, 5.74 % of the silhouette, 4 slivers, 8.8 mm straight edge on paper). No failing cell. Cure needs lofts that keep the outline of their ellipsoids at rest: request "multiview ML-3 — loft silhouette fol-6" in `requests.md` (67.2 mm, 42 mm for `fol-6`, 16.0 mm without it). Then `apply-rowan.ts --scales 50,100,200` with the saved ops and the proofs.
2. Allowlist: three proposals in `W/allowlist.json` (front `boxwood-ball` 500, `norway-spruce` 500, `windmill-palm` 200) and the withdrawal of the three entries without bands: request "multiview ML-3 — `derivedViewAllowlist`: three plant entries without bands to be replaced by band-limited ones". Decision and merge are MX-2's.
3. `windmill-palm`: 42 reported `range-end` cells (print profile at the band ends), outside this round.
4. Manifest: not edited; `rowan` and `norway-spruce` keep their dimensions.

## 4. Files

Catalog (repo-relative, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation`), updated: `C/norway-spruce/🔣️.json`, `C/rowan/🔣️.json`. Not changed: `C/boxwood-ball/🔣️.json`, `C/windmill-palm/🔣️.json`, `C/saucer-magnolia/🔣️.json`. No engine, schema, validator, test, fixture, README, manifest or tool of another lane was edited.

Created in `U/🧰️matrix/`: `batches/ML-3.tsv`; `ML-3/env.sh`, `q.sh`, `worker.sh`, `steps.sh`, `scan.sh`, `tool.sh` (queue and steps); `ML-3/derived.ts` (measure without allowlist); `ML-3/fix-norway-spruce.ts`; `ML-3/base-rowan-200.ts`, `ML-3/apply-rowan.ts`, `ML-3/search-rowan.sh`, `ML-3/make-rowan.sh`, `ML-3/ops/rowan-200-base.json`, `ML-3/ops/rowan.json`; `ML-3/allowlist.json`; `ML-3/sheets/` (seven sheets named above, kept as evidence).

Updated: `U/requests.md` (two sections appended), `U/multiview-ML-3.md` (this report). Removed: `U/🗑️generated/matrix/ML-3` after this report.
