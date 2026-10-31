# Morph audit: excel / energy (`Beat4_Energy`)

Verdict: **FIX**

Scope: series `excel`, topic `energy`, scene `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py`.
Inputs: `morph-excel-energy.md` (planned beats), `morph-excel-energy-done.md` (built), `🗑️generated/morph/📊️excel/Beat4_Energy/{en,de}/` (frames and `index.md`). Frames are 640x360. Nothing was captured or edited.

Staleness: scene last edit 2026-10-29 16:17:41, so the cutoff is 15:17:41. Every frame in en and de is newer (earliest 16:18:48), so none is stale.

Missing from disk: `dims`, `areas`, `ua`, `ht`, `volume`, `total`, `trace` (no frames in en or de, not in either `index.md`). de also lacks `degreedays`, `mistake`, `load`.

## 1. Beat status (14 planned beats)

| Beat | Clause | en | de | Evidence |
| --- | --- | --- | --- | --- |
| M1 lit capsule proxy to big capsule and tab chip | sheet | WEAK | WEAK | `en/sheet-03-mid.png`, `de/sheet-03-mid.png`: capsule with offset ghost, "Type A" tab highlighted, but no lit tower proxy or tab chip, and the tower is not in frame |
| M2 dimension lines to B2:B4 | dims | MISSING | MISSING | no frames |
| M3 seven faces unfold to D3:D9 | areas | MISSING | MISSING | no frames |
| M4 U·A cells to heat arrows | ua | MISSING | MISSING | no frames |
| M5 arrows pile up into H_T | ht | MISSING | MISSING | no frames |
| M6 hatched shortfalls gather into Σ | degreedays | WEAK | MISSING | `en/degreedays-03-mid.png` still shows the intact hatch with no gather; `en/degreedays-04-mid.png` shows the red "Σ = 1,586 K·d" pill, identity and colour readable |
| M7 b chip to back-wall arrow and back | mistake | WEAK | MISSING | `en/mistake-03-mid.png`: b chip mid-value 0.75, arrow not visible, capsule over the chart. `en/mistake-end.png`: 3.75 W/K back-wall arrow and b = 1.00 readable (end state only). Return to 0.50 not captured |
| M8 volume box to V chip, blue H_V arrow | volume | MISSING | MISSING | no frames |
| M9a H_T and H_V chips to bar | total | MISSING | MISSING | no frames |
| M9b bar splits into W arrows | load | WEAK | MISSING | `en/load-03-mid.png` shows the intact bar. `en/load-04-mid.png` shows red/blue arrows (swarm), readable. `en/load-end.png`: arrows gone, B13 "Heating load" still empty |
| M10 hatched floor plan to floor-area tile | annual | WEAK | WEAK | `en/annual-03-mid.png`, `en/annual-04-mid.png`, `de/annual-03/04-mid.png`: tile text pale grey, grey ghost squares over tiles, capsule ghost behind |
| M11 140 dots to four count chips | energy | BUILT | BUILT | `en/energy-04-mid.png`, `de/energy-04-mid.png`: type-coloured dots flow to A, B, C, D chips. Caveat: C chip partly under dots; chip-to-tile chain not visible |
| M12 four capsules to bar chart | compare | WEAK | WEAK | `en/compare-03-mid.png` capsules, `en/compare-04-mid.png` bars already formed. No in-between frame. Same in de |
| M13 faces unfold into net | shape | BUILT | BUILT | `en/shape-04-mid.png`, `de/shape-04-mid.png`: coloured net readable, but it sits over the second capsule and not beside a floor square; grey hatch for dropped faces not visible |
| M14a window U cell to 140 dots | whatif | BUILT | BUILT | `en/whatif-04-mid.png`, `de/whatif-04-mid.png`: dotted trail from B14 to tower, dots coloured, counter 177,022 kWh/a (-2.4%) |
| M14b trace | trace | MISSING | MISSING | no frames |
| M14c legend gathers into tower | recap | WEAK | WEAK | `en/recap-03-mid.png`, `en/recap-04-mid.png`: legend text stays fully readable, tower is a pale green blob (`recap-04`). Legend symbols do not gather |

Tally by beat (M9 and M14 counted once each): BUILT 2 (M11, M13), WEAK 7 (M1, M6, M7, M9, M10, M12, M14), MISSING 5 (M2, M3, M4, M5, M8).

## 2. Remaining fade-swaps (concrete to abstract with no identity carried)

- `recap` (en/de `recap-03`): FadeOut x6 on the legend, FadeIn on the tower. Planned gather is not there.
- `compare` (en/de `compare-03`): FadeOut x2 on the capsules, FadeIn on the chart. Planned morph is a crossfade.
- `shape` (en/de `shape-03`): FadeOut x3, FadeIn. Swarm only starts at `shape-04`.
- `load` (en `load-03`): FadeOut x4 on the bar, FadeIn on the arrows. Bar slices only appear in the swarm.
- `annual` (en/de `annual-03`): FadeIn of the "degree days" and floor-area tiles. Morph only appears in `annual-04`.
- `energy` (en/de `energy-03`): FadeOut/FadeIn on the Q/A and hours-per-day tiles.
- `degreedays` (en `degreedays-03`): Glide plus FadeIn x7 on the chart and tiles.
- `mistake` (en `mistake-03`): UpdateFromAlphaFunc, FadeIn/FadeOut pairs on the b chip and arrow.

## 3. Direction and chains

- Concrete to abstract is seen in `degreedays-04`, `energy-04`, `shape-04`, `annual-04` (partially).
- Abstract to concrete is seen in `mistake-end`, `load-04`, `whatif-04`.
- Both directions occur. The return of `mistake` (b back to 0.50) is not captured.
- Chains: the Σ chain reaches the annual tile (`annual-03`) only as a fade; the teal capsule chain (sheet to dims to areas to ua to ht) is broken after `sheet` because four links are missing; the window chain (uvalues, trace, whatif) is broken before `whatif`.

## 4. Collisions and clutter

- `mistake-03` (en): capsule covers the bar chart and clips the Σ pill; the "Type A · F9" formula overlays the top right.
- `annual-03/04` (en/de): capsule ghost and grey squares sit under the tiles; "Q = ... ÷ 1000" overlaps.
- `energy-03` (en/de): tower overlaps the faded tiles.
- `energy-04` (en/de): dots cover the C chip.
- `shape-04` (en/de): net overlaps the second capsule's face.
- `recap-03/04` (en/de): legend sits over spreadsheet ghost grid lines; the tower labels overlap the dots.
- `whatif-04` (en/de): "dependent cells" label clipped by the tower.
- `mistake-end` (en): the "3.75 W/K" label overlaps the "b = 1.00" chip.

## 5. Discrepancies to fix in the plan and done files

- `morph-excel-energy.md` M12 says "only C stands higher". The frames and the scene caption show all four at 148 kWh/(m²a), so their heat loss is equal. Fix the plan narration.
- `morph-excel-energy-done.md` claims frames read for `dims`, `ua`, `ht`, `areas`, `trace`, and de `sheet` to `recap` as captured. Those frames are not in `Beat4_Energy/` (see Missing from disk above). Correct the done file or re-capture.

## 6. Fix list

1. Capture or drop the missing clauses: `dims`, `areas`, `ua`, `ht`, `volume`, `total`, `trace` (en and de). Capture de `degreedays`, `mistake`, `load`. Update both `index.md` files.
2. `annual`: fade tile fills to full opacity before the Morph, remove grey ghost squares and the capsule ghost behind the tiles.
3. `compare`: morph each capsule silhouette into its bar top (part-morph target = bar), not FadeOut/FadeIn; move the item tokens above the chart.
4. `recap`: morph the legend symbols into the tower proxy (not FadeOut); use the tower silhouette as sink, not a full-tower blob; move tower labels off the dots.
5. `mistake`: move the capsule off the degree-day chart and the Σ pill; keep the back-wall arrow visible in the mid frame; capture the reverse morph to 0.50.
6. `load`: land the W arrows into B13 "Heating load" (value token) instead of leaving the cell empty; morph bar slices, not FadeOut.
7. `shape`: place the net beside the floor square (per plan) and off the second capsule's face; show the grey hatch for dropped faces.
8. `energy`: keep the C chip above the dots; hide the tower's ghost tiles during the swarm.
9. Fix M12 plan narration (all four bars equal at 148 kWh/(m²a)).
10. Reconcile `morph-excel-energy-done.md` with the frames on disk.
