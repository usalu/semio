# DSL Assets Patch (W3-DSL-ASSETS)

Status: dry run only. No DSL asset was modified. Nothing applied.

## Dry run
- `🛠️asset-laws/patch.py` (no `--apply`), via `🗑️generated/build-runner/heavy.sh`, 2026-11-03 ~15:04.
- Output: `🗑️generated/dsl-assets/dryrun-1.txt` (441 lines, errors 0).
- Freeze at start: `build-en1996` (graph-plan). Cleared later; `freeze.txt` absent at the last check.
- 29 assets report CHANGED. The trace-and-tools report listed 14 (en1991, en1993, en1994, en1997, en1998). The extra changes come from en1992, en1996 and en1999, which are now also affected.

## Classification
The brief expects add-only changes. The dry run is not add-only:

| Asset | Dry-run change | Verdict |
|---|---|---|
| en1991 multi-fail-noncompliant | ADD empty collection x5 (footbridges, rail-dynamics, load-dispersals, bulk-solids, rotating-machines) | safe to apply |
| en1991 de-office-compliant | ADD empty collection x5 (same five) | safe to apply |
| en1991 bridge-showcase | `tables not in model dropped: ['fatiguemodels']` | skip, value loss |
| en1992 liquid-retaining-fem-anchor, failing-prestressed-beam, compliant-prestressed-beam, failing-under-reinforced, rc-building-showcase, compliant-office-frame | DROP stale column `bearing-width`; stale nested keys dropped from PunchingSpec | skip, value loss |
| en1993 heb240-compliant, steel-showcase, high-strength-connection | DROP stale column `beam-web-fy` | skip, value loss |
| en1994 composite-floor-beam, composite-floor-beam-failing, composite-bridge-girder, composite-showcase | `tables not in model dropped: ['studs']` | skip, value loss (studs becomes a collection via convert-field-to-collection, not add-only) |
| en1996 multi-fail-masonry, loadbearing-wall | DROP stale column `web-spacing-m`; `1 trailing tokens dropped (partial row)` | skip, value loss |
| en1996 masonry-showcase | DROP `web-spacing-m`; stale nested keys in WallPier | skip, value loss |
| en1997 demo, noncompliant, compliant | text changes with no reported add, drop or warning | skip, unexplained change not reviewed |
| en1998 seismic-multipart-fail, seismic-rc-frame, seismic-rc-frame-fail, seismic-showcase, seismic-multipart | DROP stale column `intermediate-steel-area-m2` | skip, value loss |
| en1999 noncompliant-multi-fail, aluminium-roof-purlin | DROP `load-eccentricity`; stale nested keys in WeldGroup | skip, value loss |
| en1999 aluminium-showcase | DROP `load-eccentricity` | skip, value loss |

Warnings "`X` is not a variant of Y" (e.g. `0.015m` for UnitGroup, `false` for TightnessClass) leave the cell unchanged. They point to values the spelling conversion cannot map and need a human decision, not a patch.

## Not done
- No apply. The dry run was not re-run after apply, because nothing was applied.
- A per-asset unified diff was attempted (`🗑️generated/dsl-assets/diffdrv.py`) but never ran: the heavy-slot queue held it and the relaunch did not start. The en1997 changes are unexplained for that reason.
- `patch.py` applies per artifact, so `--apply` for en1991 would also rewrite bridge-showcase (which must be skipped). Applying the two safe en1991 assets needs a per-file apply.
- Build marks not written: "ready for rebuild: dsl assets" was not appended to `build-en1991/latest.md`, because no file changed.

## Pending (frozen or blocked, owners to decide)
- Every "skip" row above, pending a decision on DROP (drop stale columns or keep them) and on the unmapped values.
- en1991 safe pair: apply the 5 empty collections each, then re-run the dry run.

## Correction (after report): en1997 diffs
The original diff driver finished later and wrote 23 per-asset diffs to `🗑️generated/dsl-assets/diffs/`. The earlier note "diff never ran" is wrong.
- en1997 demo, noncompliant, compliant: header-only type-tag changes. Column types TEXT → ENUM for `soil-type`, `pile-type`, `earth-pressure-mode`, `wall-movement`. No value changes. Still skipped: the brief asks for no spelling or type changes, and the enum choice needs an owner decision.
- en1999 aluminium-showcase and the other en1997/en1999 entries were all confirmed as DROP or type changes in the same diff set. No new asset is safe.
- Diffs are not reviewed line by line for the rest. The classification above stands.
