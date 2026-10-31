# Morph storyboard: excel / energy (`Beat4_Energy`)

Scene file: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py`. Binding: `morph-plan.md`, `morph-brief.md`. Clause count stays 24 (keys unchanged, every clause en + de, captions at most two lines). Numbers always come from the model/engine (`_num`, `vals_*`, `_nakagin.slots()`), never typed here; the figures in this file are only for orientation (type A 36 capsules, 140 in total, H_T about 30.9 W/K, H_V about 3.1 W/K, HDD about 1,586 K·d).

Morph toolkit used (section 7 of `api.md`, to be confirmed when `Status: ready`): `morph`, `scatter`, `gather`, `split`, `token`, `tether`, `unfold`, `retext`, `travel`, `proxy`, `silhouette`. The tower always enters a morph through `proxy()` or a per-capsule `silhouette()`.

## Chains (the same object keeps its identity across clauses)

- **Teal capsule chain**: lit type A capsule on the tower (hook) → big capsule (sheet) → three dimension labels → cells B2:B4 (dims) → seven tinted faces → cells D3:D9 (areas) → seven U·A cells → seven arrows (ua) → H_T cell (ht).
- **Degree-day chain**: hatched monthly shortfalls (degreedays) → one token Σ K·d → the "degree days" tile (annual) → ×24 h/d → Q tile → count chips → tinted tower (energy).
- **Window chain**: window U cell E8 (uvalues) → blue window ring → Window-U input cell (trace) → chain tokens → 140 window dots on the tower (whatif) → typed constant (frozen) → formula again (repaired).

## Morph beats (14)

Legend: C→A concrete to abstract, A→C abstract to concrete, A→A abstract to abstract.

| # | Clause | Dir. | Concrete object (drawing) | Abstract object (tool) | Carries over | s | Narration while it happens |
| --- | --- | --- | --- | --- | --- | --- | --- |
| M1 | hook → sheet | C→C→A | one lit type A capsule `proxy` on the tower (`kinds[key]=="A"`, first one in tree order) | the large capsule drawing beside the window and the tab chip "Type A / Typ A" | teal type colour, silhouette; a ghost stays on the tower | 1.6 | "Every capsule type has its own worksheet." (en) / "Jeder Kapseltyp hat ein eigenes Arbeitsblatt." |
| M2 | dims | C→A | the three dimension lines of the capsule (width, length, height) with their labels | cells B2, B3, B4 (value tokens in the cell font) | label text → cell string (`retext`), colour DATA, order top to bottom = width, length, height; tether stays while the frame shows | 1.4 | "Three constants describe the shell. On the capsule they are the three dimension lines." |
| M3 | areas | C→A | the seven faces of the capsule (roof, floor, left, right, front, window, back) `unfold` into a flat row of tinted rectangles | the seven area cells D3:D9 and their reference frames | face tint = frame colour `ROW_COLOR`, rectangle width ∝ area (engine value), order = row order 3..9 | 2.0 | "Each surface area is a formula from these dimensions." |
| M4 | ua | A→C | the seven U·A cells F3:F9 (value tokens) | the seven heat arrows on the capsule; length and width ∝ U·A | cell value text → arrow label, colour `SERV`, order; hidden faces start on their dashed lead | 1.8 | "Area times U-value is U·A. The larger the U·A, the longer and thicker its arrow." |
| M5 | ht | C→A | the seven arrows on the capsule | one H_T token (`xl_display`, W/K) at cell B11 and the SUMPRODUCT card | arrow colour → token colour, lengths add up (arrows slide end to end into one bar, then the bar becomes the token) | 1.6 | "H_T sums all U·A: the arrows pile up into one number." |
| M6 | degreedays | C→A | twelve hatched monthly shortfalls (area between the mean bar and the 18 °C line) | one token "Σ = HDD" (Climate!B5) | hatch colour `SERV`, count 12 → 1 (`gather`), sum from the engine | 1.6 | "Heating degree days are an area: the twelve shortfalls add up to one number." |
| M7 | mistake | A→C | the cell value b (Climate!B6 token, 0.50 → 1.00) | the back-wall arrow (thickens) and the bolted back face | grey `MUTED` of the b token → arrow; value drives width live | 1.4 | "With b at one, the back-wall arrow grows and H_T grows with it." |
| M8 | volume | C→A→C | the translucent interior volume box of the capsule | cell B10 (V) → formula tokens `air_heat_capacity`, `air_change`, V → the blue H_V air arrow | blue `BLUE`; box faces collapse into the V token, the token multiplies into the arrow (`travel` along a tether) | 2.0 | "Air leaks too: volume times air change times heat capacity gives H_V." |
| M9 | total / load | A→C | H_T and H_V tokens, then the single H token | stacked bar (total), then the arrows in W (load) | colours `SERV` and `BLUE` stay as the two bar segments; H splits (`split`) into the same arrows now labelled in W (× Δθ) | 1.8 | "H_T and H_V stack into one loss per kelvin; times the temperature difference they become watts." |
| M10 | annual | C→A | the hatched floor plan of the capsule (8.7 m² from `program`/B9) | the "floor area" tile B9 | hatch → tile fill `GOOD`, value text via `retext` | 1.4 | "Divided by floor area: kWh per square metre." |
| M11 | annual → energy | A→C→A | the "degree days" tile and the Q tile | the count chips of the Program sheet and the 140 capsule proxies on the tower | the Σ token from M6 enters the tile (chain); the 36/35/34/35 capsules of each type `gather` into their chip, type colours `type_color` | 2.0 | "The Energy sheet multiplies each type's demand by its capsule count." |
| M12 | compare | C→A | the four capsule silhouettes of the types row | the four bars of the chart (F4:F7) | type colour, left-to-right order A B C D; bar height from the engine value | 1.6 | "The four types share one shell, so their bars stand at the same height (148 kWh/(m²·a), engine value) and the value tokens land on the bar tops." |
| M13 | shape | C→A | the exposed faces of the capsule `unfold` into a net beside the floor square; with roof and floor shared two faces drop out (grey hatch) | the ratio token "m²/m²" and the demand token | face colours, area proportion; removing two faces shortens the net and the ratio token via `retext` | 2.0 | "Five faces and a window wrap a small room; heated neighbours would remove two of them." |
| M14 | trace → whatif → recap | A→C | the window-U input cell B14, its dependent chain tokens (Type A H_T, Type A demand, building total D8) | the 140 window dots on the tower and the tinted tower | blue `BLUE` of the window ring (uvalues) → chain tokens → window dots on the tower; the counter and the dots turn from red to green together; at recap the legend symbols `gather` back into the tower proxy | 2.2 | "Enter a better window in this one cell, and every dependent cell, arrow and total follows." |

Counts: C→A beats M2, M3, M5, M6, M10, M12, M13 (7). A→C beats M4, M7, M9, M14 (4), plus the A→C halves of M8 and M11 (6 in all). Chains: teal capsule (M1 → M2 → M3 → M4 → M5), degree day (M6 → M11 → energy tower), window U (uvalues → M14).

## Replaced fade-swaps

| Where in the scene | Today | After |
| --- | --- | --- |
| `sheet`: `FadeOut(tower)` + `FadeIn(capsule)` | cut | M1: lit capsule proxy grows into the big capsule, ghost on the tower |
| `dims`: `select("B2:B4")` plus a frame | cells highlighted, lines unchanged | M2: label strings fly into the cells |
| `ua`: `ramp(cs.grow)` | arrows appear on their own | M4: cell value tokens grow into the arrows |
| `ht`: card fades in | card appears | M5: arrows pile up into H_T |
| `degreedays`: `FadeOut(names)` + `FadeIn(hdd)` | chart appears, Σ readout fades | M6: shortfalls gather into Σ |
| `total`/`load`: `FadeOut(bar)`/`FadeIn(temps)` | bar fades | M9: tokens, bar and arrows morph |
| `annual`: `FadeOut(units)` etc. | tiles fade | M10 + chain from M6 |
| `energy`/`compare`: `FadeOut(tower)`, `FadeIn(chart)` | cuts | M11, M12 |
| `shape`: `FadeOut(chart)`, `FadeIn(shape)` | cut | M13 |
| `trace`/`whatif`: `FadeIn(chain)` and cell updates only | tower shown as counter | M14: window dots on the tower change colour |
| `recap`: all `FadeOut`, `FadeIn(legend)` | cut | symbols gather into the tower proxy |

## Implementation notes (Phase B)

- Re-read the scene file immediately before each edit; small Edit-tool diffs; the Nakagin model and drawings are foreign (counts are on disk: take them from `_nakagin.slots()`, `capsule_types()` and the engine, not from this file).
- Sources come from the live drawings: `capsule.parts.faces[name]`, `.dims[name]`, `tower.parts.capsules[key]` (`silhouette()` each), window cells via `win.rect(a1)` / `win.cell(a1)`. Targets come from `win.rect`, the chart `rows`, and `_arrow` geometry. Source and target are matched in a fixed order so that colour, count and order carry over; every morph has a `tether` back to the origin for the duration of its frame.
- All new numbers are `_num`/`xl_display`; no typed values; every new label exists in en and de (`self.tr`).
- Blocking: `api.md` has no section 7 and `🎓️teaching/🏛️architecture/🔀️morph/` does not exist yet, so Phase B waits for `Status: ready`.
