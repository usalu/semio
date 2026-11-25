IN WORK (2026-11-25, started 19:15; state lines below are updated after every asset)

# 🌿️ Multiview Fixer Report: Later Round ML-3 (Open Plants of Batch MS)

Batch `🧰️matrix/batches/ML-3.tsv`: `boxwood-ball`, `norway-spruce`, `windmill-palm`, `saucer-magnolia`, `rowan` (`saguaro-cactus` is not part of it and was not touched). Briefs `📋️multiview-brief.md`, `📋️takeover-brief.md`. Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/ML-3`, `GEN` = `U/🗑️generated/matrix/ML-3`, `C` = `S/🗂️catalog/🌳️vegetation`.

## 1. State at the start (19:15)

- No process of this batch existed; `W` and `GEN` were created by this round.
- The five catalog files carry the modification time 19:08 to 19:09 of 2026-11-25 (all 67 plants do: a catalog-wide write of another lane). Backups of the files as found: `GEN/backup/<id>.json`, sha-256 `781b4afa0f82296a` (boxwood-ball), `0137c11a68fc42f2` (norway-spruce), `8332acedd775540d` (windmill-palm), `d7109a2b16a40730` (saucer-magnolia), `56b5d3377cfaa58c` (rowan).
- The fixture already holds `derivedViewAllowlist` entries for the front of `boxwood-ball`, `norway-spruce` and `windmill-palm` without bands (merged from `🧰️matrix/MS/allowlist.json`, kept by D58). This round measures without any allowlist (`W/derived.ts`).
- `rowan` and `saucer-magnolia` were repaired by batch MV-3 after MS (`multiview-MV-3.md`: both claimed clean, logs deleted). This round measures them again and looks at the wind poses.

- Start scan `T/🔎️scan.ts --language en` over the five plants (19:33 to 19:42, one process, to its end, with the allowlist of the fixture as it stands): `[scan] 5 assets: 5 clean, 0 with a hard finding, 0 hard findings`; reported tier: `rowan` 4 (group drift, worst 221 mm `fol-7` 1:200 `gale`), `saucer-magnolia` 2 (165 mm `fol-1` 1:100 `spring-gale`).

## 2. Result per asset

### `norway-spruce` (repaired at 1:200, written 19:49; allowlist proposed for 1:500 only) — PROOFS IN WORK

- Measured without allowlist (`W/derived.ts`, front against side elevation, limit 0.03): before 1:50 0.0506, 1:100 0.0660, **1:200 0.0239**, **1:500 0.0178**; after 1:200 **0.1109**, the others unchanged.
- Cause at 1:200: the individual has two leading sprays per whorl in the sagittal plane (bones `spray-w-NN`, `spray-e-NN`); they make the tree 8.48 m wide and 7.41 m deep. The 1:100 body keeps them as capsules beyond the rim of each skirt; the 1:200 body had merged them into the skirt by stretching each skirt of revolution sideways (lateral scale 0.80 to 0.85), so the front was the side squeezed by one factor per tier and the spray bones drew nothing.
- Repair (`W/fix-norway-spruce.ts`, idempotent: second run `unchanged`, same digest): the 15 skirts of `volumes.200.body` go back to their own reach (profile radii and lateral scale of the 1:100 skirt; elevation and plan path scaled about the skirt axis, the front path stays), and the 30 leading sprays of the 1:100 band (capsule solids, thermal references, capsule shapes in the three views) are set into the 1:200 band, painted below their skirt as at 1:100; decision texts of the representation and of the volume follow (50 body solids). A spray is 1.2 to 3 mm thick and 7 to 23 mm long on paper at 1:200.
- 1:500 is not repaired in the asset: there the leading sprays reach about 1 mm beyond the rim and are 0.3 to 0.6 mm thick on paper; the crown is two solids of revolution with an elliptic section. Proposal in `W/allowlist.json`: front, bands `["500"]`, with the physical reason.
- Catalog file: `0137c11a68fc42f2` before, `fbb2bdcbdc609de7` after (byte-identical to the trial copy that was measured).
- Proofs so far (each run to its end, lines in `GEN/proof/norway-spruce.txt`): scan `1 clean, 0 hard findings`; gate over every pose, clip instant and transition `hard findings: 0`; `suit.ts` `prints`; `M/measure.ts` on the identical bytes: silhouette OK (1:200 IoU 0.9993, 0.9990, 0.9971; bounding box 8.4808 × 30.0000 × 7.4112 m against 8.483 × 30 × 7.412), `validate: 0 issues`, three-views sweep 0 findings (before: the same).
- Sheets opened: 1:200 calm before and after, 1:200 storm after, 1:100 calm and storm for comparison. After: the side elevation shows the ragged spray ends beyond every rim, the front stays the smooth narrower cone, the plan shows the four longest spray ends beyond the lowest skirt, as the 1:100 band does. In `storm` the solid plan paints the lifted windward spray of the lowest whorl over its skirt; the 1:100 band did so before (same shapes), it is not new.
- NOT YET RUN: check CLI, the six vitest files, the matrix (they wait for a 2500 MB grant).

### `rowan` (trial proven on a copy, NOT YET WRITTEN) — IN WORK

- State found: scan clean, but the wind poses show what the coordinator does not accept. `../MV-3/visible.ts rowan`: worst sheet 1:50 front `gale`, 12 pieces of depth plates beside the outlines, 5.74 % of the silhouette, 4 slivers, longest straight free edge 8.8 mm on paper (1:100 front `gale` 4.07 %, 7 slivers). At 1:200 the depth slabs of batch MS (93 front and 120 plan shapes) draw broken contour lines in the default pose and ticks, notches and nested rings in `gale` (sheets opened: 1:50, 1:100, 1:200 `gale`, 1:200 and 1:50 at rest).
- Repair on a copy (`W/search-rowan.sh`): `W/base-rowan-200.ts` sets the 1:200 front and plan back to one flat outline per part (`W/ops/rowan-200-base.json`, the 27 and 23 shapes authored before the slab redraw; elevation and body of 1:200 are unchanged since); `../MV-L1/loft.ts` (used as it is, not copied) then draws the front and plan of every foliage mass at 1:50, 1:100 and 1:200 as the loft of its own ellipsoids (83 lofts, `W/ops/rowan.json`, absolute values) and removes all 142 depth plates and sheets. The search ran on a copy without the fruit dots of 1:200, because the tool converts a bone only when it draws one closed body outline; the saved lofts are applied to the full file.
- Measured on the copy: gate over every pose, clip instant and transition `hard findings: 0`, and no reported group drift at any scale (before: 4 reported, worst 221 mm); `suit.ts` `prints`; second application byte-identical; one evaluation of front and plan 219 ms at 1:50, 99 ms at 1:100, 82 ms at 1:200. The file grows from 2.48 MB to 4.06 MB.
- Sheets of the copy opened: 1:50 at rest (no visible difference to the catalog file) and `gale`, 1:100 `gale`, 1:200 `gale`: whole lobed masses in front and plan, no cut, no sliver. Left: the tool reports holes it could not fill inside the top mass seen from above (`fol-top` plan, 14 at 1:100, 6 at 1:200; the authored top mass has four such slits at rest) and one in `fol-7` front at 1:100; they show as two dots in the middle of the 1:100 and 1:200 plan in `gale`.
- NOT YET RUN: silhouette guard on the copy (`M/measure.ts`), then write and the full proof list.

### `saucer-magnolia` (not changed) — MATRIX IN WORK

- Scan clean (2 reported: `fol-1` 165 mm in `spring-gale` at 1:50 and 1:100); front against side without allowlist 0.0814, 0.0751, 0.0603, 0.0838 (1:50 to 1:500): no finding. It carries no depth plate. Sheets opened: `spring-gale` at 1:50 and 1:200: whole masses, no cut or sliver; the four blossoms on top of the crown stand about 0.1 m above the front outline of the top mass at 1:50 (the reported drift).

### `boxwood-ball` and `windmill-palm` (not changed, allowlist proposals) — MATRIX IN WORK

- `boxwood-ball`: front against side 0.0000 at 1:500 only (one sphere of 0.8 m, 1.6 mm on paper); proposal front `["500"]`.
- `windmill-palm`: 0.0279 at 1:200 only (0.218 m on 7.81 m, carried by a dead leaf beside the stem at 5.4 m; 0.037 to 0.049 in the other bands). The crown of twenty leaves in a spiral round one stem is rotationally symmetric by nature and both views are true views of the 201 solids; no part of the palm tells its sides apart by 0.23 m, so nothing truthful can be added in the asset. Proposal front `["200"]`. Sheet 1:200 opened.
- Request filed: `requests.md`, "multiview ML-3 — `derivedViewAllowlist`: three plant entries without bands to be replaced by band-limited ones".

## 3. Open

(in work)

## 4. Files

(in work)
