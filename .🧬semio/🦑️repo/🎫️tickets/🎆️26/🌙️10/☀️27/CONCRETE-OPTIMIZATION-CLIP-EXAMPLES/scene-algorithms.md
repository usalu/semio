# Scene report: Algorithms and Evolutionary Search (`🧬️algorithms`, `Beat6_EvolutionarySearch`)

Owner file: `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🐍️.py` (class, `TITLE`, `construct()` unchanged; `present_algorithms` plus helpers added). Storyboard: `storyboard-algorithms.md`. Check script: `check-algorithms-narration.py` (run it after every model change).

## What the viewer sees

One question runs through the clip: which window share and overhang give the seminar room the least annual electricity when every design costs one simulation? The left half is the landscape (heat map and contours from `surface()`, window share 10-80 % against overhang 0-1.2 m, colour bar in kWh/a). The right half always shows the current design: room section (summer), stacked electricity bar (fixed scale 900 kWh/a), run tape and counter ("n of 50 simulations"). The map dims to a fog while a solver works; every simulated design is painted on it in the colour of its own electricity, so the viewer sees only what the solver knows.

| Beat group | Keys | Visual |
| --- | --- | --- |
| Task and landscape | `task` .. `map_3` | empty axes, room and bar; heat map sweeps in while the counter races to 1,681; optimum star with its kWh/a; fog returns |
| Gradient descent | `gradient` .. `gradient_result_2` | four probe chips with their kWh/a, slope chips with units, downhill arrow, room morphs and the bar shrinks, valley-floor line for the first overshoot, 5-run steps on the tape, zigzag trail, probe/step share on the tape |
| Pattern search | `pattern` .. `pattern_result` | compass arms and painted probes, 2 x 2 chip grid (winner ringed), step chip, "halve the step" verdict, 13 rounds as a montage |
| Genetic algorithm | `population` .. `genetic_result_2` | 4 x 2 façade cards (thumbnail in map colour, window share, overhang, kWh/a), two recorded duels, parent façades with gene chips and curved gene arrows into the child façade, dashed parent box on the map, mutation arrow, six children replace the non-elite cards, elite stars, generation counter, cloud of dots contracting, runs to within 1 % per method |
| Comparison | `compare` .. `proof` | best-so-far curves vs simulations for all three methods with live thumbnails, best design and kWh/a per method, gap to the map minimum, stop-rule marks, 50 of 1,681 sliver and the three final designs on the map |
| Close | `confirm` .. `close` | winner in room and bar with a "model result" stamp; slider / simulation / solver diagram driven by the genetic bests |

Every number on screen is read from the model (`surface`, `gradient_path`, `pattern_path`, `genetic`, `convergence`, `evaluate`). Pointer, room, bar, chips and cards share the `study_trackers`, so they cannot disagree.

## Narration

`NARRATION` is a literal list (44 rows, en/de). Typed numbers: 1,681 (map runs), 50 (budget and runs of gradient/genetic), 49 (pattern runs), 75 % / 0.1 m (the start), 8 (population), 6 (children), 7 (generations), 5 and 4 (runs per step), 16 (pattern search runs to be within 1 %), 1 % and 2 kWh/a (thresholds). `check-algorithms-narration.py` re-derives every one of them and every claim (slope signs, first step crosses the valley floor, gradient raises the electricity at some iterate, genetic improves in steps, pattern arrives first, all within 2 kWh/a, mutation helps in generation 0 and hurts in generation 1, a weak card wins a duel, no unresolved placeholders, both languages carry the same number of numbers, clauses at most 100 characters). Last run against the final model: 0 failed facts, 0 textual problems. Clauses with figures that change with the model (results, 16 runs, 2 kWh/a) are the ones to re-check after any model edit.

## Scene-specific helpers (all inside the scene file)

`_data` (flattened simulation log per method, best-so-far designs, runs to within 1 %), `_tape`/`_counter`/`_spend` (run tape, live counter), `_pointer` (ring, dot and walked path from live trackers), `_painted` (simulated point in its map colour), `_probe_grid`, `_compass`, `_result`, `_card`, `_cloud_dot`, `_duel`, `_selection`/`_breeding`/`_children`/`_generations`, `_compare`/`_stopping`/`_proof`, `_slider`/`_tools`, `_fade` (family-aware FadeOut list), `_outdent` (moves y tick labels clear of tick marks).

## Verification

- `capture.py` en (final code): 45 frames, all viewed over the last three revisions, layout report "none" for every beat. German (final code, run to completion): 45 frames, key frames viewed, layout report "none" for every beat. Both captures used the toolkit as of 09:26; the toolkit agent was still editing `🏠️room/🐍️.py` (mtime 09:27), so one more English capture is due after its heat_map / plot_axes / Indicate fixes land.
- Caption contract (`check-algorithms-captions.py`, same assertions as `OptimizationClipContract.test_all_clauses_fit_both_languages_and_audiences`, this scene only): 44 clauses x en/de x speaker/spectator, 0 failures. The other `OptimizationClipContract` tests (bilingual rows, native scene discovery, curriculum) passed (4 of 4); the full caption test of all six chapters was stopped because it ran over 25 minutes on the loaded host.
- `check-algorithms-narration.py`: 0 failed facts, 0 textual problems against the final model (minimum 494.8 kWh/a at 66 % / 0.99 m; gradient, pattern, genetic end at 496.4 / 496.7 / 498.1 kWh/a).

## Toolkit gaps and traps (not edited, reported)

1. `heat_map` builds 1,681 rectangles; every frame redraws them (about 3 s of render time per second of video with live widgets). An `ImageMobject` variant would make rendering several times faster.
2. `room_section` labels (chips, "Cooling ... kWh/a") extend farther left and right than documented, and the two-line demand label overlaps itself at scale 0.3; `facade(labels=True)` chips overflow at scales below 0.3 (I draw my own gene chips and pass `labels=False`).
3. `plot_axes` always creates the two axis titles (I remove them) and puts vertical tick labels over the tick marks.
4. `Indicate` (and any Transform) on a live holder (`_live`, `room_section`, `energy_bars`, `live_marker`) raises "zip() argument 2 is longer" when its children are rebuilt; use `Flash`, `Circumscribe` or static chips. Re-`FadeIn` of a removed `room_section` also failed once; the scene builds a fresh one.
5. `ClipScene.step` runs all top-level animations for the same `seconds`; sequences must be wrapped in `Succession`.
6. A full capture of this scene takes 5-15 minutes on this host.

## Round 2

Source: `audit-r2-algorithms.md`. Scene file only (`🧬️algorithms/🐍️.py`); toolkit API from `toolkit-r2.md` (`formula`, `live`, `electricity_formula`, `parts`/`apportion`, daylight and lamp drawing, `heat_map(avoid, ticks)`, `ramp_strip(ticks)`). Helper scripts: `algorithms-r2-probe.py`, `check-algorithms-narration.py` (updated: mid-ranked duel winner, tolerance stops, "15"). Old file: `algorithms-scene-before-r2.py.txt`.

### What changed

- **Design panel (B1, B2, D2).** Two live room sections, winter (heating, radiator, loss arrows) and summer (cooling, sun, flows), both with daylight, sky light and dimming lamps from the toolkit. Heat-loss arrows through glass and wall are drawn by the scene (`_losses`: weight and length follow U · A · degree hours of the live design; the number under the room is the same quantity, 1,134 kWh/a at the start). Under the summer room the solar heat gain. Three substituted rows (`_ledger`): `Heating demand 122 kWh/a ÷ COP 3.5 = 35 kWh/a`, `Cooling demand 1,778 kWh/a ÷ EER 3.0 = 593 kWh/a`, `Lamps 0.43 kW · 1,500 h/a · 18 % = 117 kWh/a` (parts rounded with the largest remainder, they add up to the bar total; the genetic beats show `35 + 593 + 117 = 745 kWh/a` through `electricity_formula`). Season tags carry "sun altitude 66°" (N6), the design chip above the map prints window share and overhang with two decimals (N4, L16).
- **Gradient probes (B3, N9).** A ×11 lens on the map separates the four probe crosses (cyan windows, orange overhang, with signs); four façade cards show every probe design with its simulated kWh/a. The difference quotients are written with the printed values (`(748.2 − 741.2) kWh/a / 1.4 % = 5.0 kWh/a per %`, `(743.05 − 746.33) kWh/a / 0.024 m = −136 kWh/a per m`, decimals chosen so the arithmetic closes). The first step is written as arithmetic (`75.0 % − 19.7 % = 55.3 %`, `0.10 m + 0.16 m = 0.26 m`, `745 − 132 = 613 kWh/a`); the overshoot chip says 10.7 % past the valley floor; the change row is signed.
- **Pattern search (N3, N7, S4).** Four cards per round with signed gene change and "at the limit" for clamped probes (80 % and 0 m), a decision band (`best neighbour 648.8 < current 744.7 → move there`, and in the stalled round `no neighbour below 499.2 kWh/a → step ÷ 2: 20 → 10 %`), new beat `pattern_5` for the shrinking montage after the halving round.
- **Genetic algorithm (B4, S2, S3, S5, L2, L7, L11, L15, L16, N2, N8).** Cards 1.4 wide (14 px text), small winter and summer rooms below that always show the best design so far with `96 + 184 + 307 = 587 kWh/a`; crossover shows both parents as façades with gene chips, crossing arrows into one child row and the child's rooms; mutation compares the what-if (dashed ghost card, not simulated) with the simulated child (`run 9`); the hurting nudge (`mutation_2`) moved behind `survival` and shows a generation-2 child at its true run (`run 18`, counter 18 of 50) with its parents; earlier generations shrink to faint dots so the cloud contracts; history as `Generation 0..7` over `best kWh/a`; elite stars inside the card corner; `tournament_2` wording is "mid-ranked" (rank 5 beats rank 8).
- **Comparison and stopping (S1, B5, B6, L1, L14).** Right column shows the live best design of each curve tip with dashed links, `within 1 %` band drawn on the chart, reach chips in the column. `stopping` marks where the first rule fires on each curve (tolerance at 15, 21, 23) with the three rules at the right; new beat `stopping_2` shows the design each method would deliver there (513.2 / 499.2 / 548.3 kWh/a, 16.8 / 2.5 / 50.2 above the 50-run result): the genetic run would stop far too early.
- **Close (L4, B6).** The map returns with the two trails, the three solver rings and the star; both rooms at the map minimum; chips with `façade cost 21,260 CHF`; full method names in German.
- **Layout and text.** All chip, card and formula text is at least 14 px at 1280 wide (the strict check flags below 11 px); y title, legend and contour note were moved so nothing overlaps; the contour labels are replaced by one note (`contour lines 550 650 750 kWh/a`) because the toolkit's chips sit on their own lines.

### Final beat list (43 keys)

`task`, `task_2` (new: three ways to spend electricity), `task_3`, `map`, `map_2`, `map_3`, `gradient`, `gradient_2`, `gradient_3`, `gradient_4`, `overshoot`, `overshoot_2`, `gradient_result`, `gradient_result_2`, `pattern`, `pattern_2`, `pattern_3`, `pattern_4`, `pattern_5` (new), `pattern_result`, `population`, `population_2`, `population_3`, `tournament`, `tournament_2`, `crossover`, `crossover_2`, `mutation`, `children`, `survival`, `mutation_2` (moved), `generations`, `genetic_result`, `genetic_result_2`, `compare`, `compare_2`, `stopping`, `stopping_2` (new), `proof`, `confirm`, `tools`, `tools_2`, `close`.

### Verification

- `capture.py` en and de (final code): 43 frames each, every PNG of the current keys viewed in en, the changed or German-risky ones in de. Strict check: 3 findings in 2 frames per language, all justified below.
- `check-algorithms-narration.py`: 0 failed facts, 0 textual problems.
- Unit tests: see the line in the coordinator reply.

### Remaining strict findings (false positives)

`map_2` and `overshoot`: the chips `map minimum`, `valley floor`, `overshoot` sit on the map and the map's contour lines and the iterate arrow pass under their opaque (fill 0.88) frames; the check counts a stroke that enters and leaves a chip border although it is hidden behind the chip. No text is crossed.

### Remaining risks

- Heating arrows through the back wall come from the toolkit's `demand=True` and stay short (scaled to the largest demand of the study); the meaningful heat-loss arrows are the scene's `_losses` on the glass side.
- Façade pictograms in the comparison and stopping columns look alike at similar designs; the numbers beside them carry the difference.
- A full capture takes about four minutes per language; layout depends on the toolkit extents (`ROOMS_AT`, `BAR_AT`).

## Round 3

Source: `verify-r2-algorithms.md` (1 blocker, 6 major, 13 minor). Coordinator precision rule applied everywhere. Check scripts: `check-r2-algorithms.py` (three stale assertions updated: later child at run 18, one-decimal segments that add up exactly; 0 mismatches), `check-algorithms-narration.py` (0 failed facts), `algorithms-r3-arithmetic.py` (new: every ledger row over all 88 designs the clip shows).

- **V1 ledger.** Four rows at their own precision: `Loss 2,054.9 − useful gains 1,932.8 = heat 122.1 kWh/a` (V5 bridge: transmission loss from U·A·degree hours, gains = loss − demand), `Heat 122.1 ÷ COP 3.5 = 34.9`, `Cold 1,777.7 ÷ EER 3.0 = 592.6`, `Lamps 0.432 kW · 1,500 h/a · 18.090 % = 117.2`; results are one-decimal largest-remainder parts, so the sum `34.9 + 592.6 + 117.2 = 744.7` (bar parts, tools, population and later beats) holds exactly. A row prints `≈` instead of `=` when the apportioned last digit differs from the derivation (63 of 264 rows over all designs, e.g. `139.6 ÷ 3.5 ≈ 47.1`): the arithmetic on screen is never claimed wrong.
- **V2.** Slope rows print the probes with the digits at which the printed quotient reproduces the model slope: `(748.2 − 741.2) kWh/a ÷ 1.4 % = 5.0`, `(743.053 − 746.327) ÷ 0.024 m = −136`. Step rows use one decimal (`744.7 − 132.0 = 612.7`).
- **V3.** Crossover shows the child only as a dashed what-if ring on the map and chips; bar, ledger and rooms stay on the last simulated design (parent A) with "not simulated yet: the counter still reads 8"; the simulated child (`Mutation … run 9`) enters in `mutation`. Child chips use two decimals (V8: 30.27 → 31.13 gives +0.9).
- **V4, V15.** Compare: the three façade cards follow the curve tips live (run counter 24 in `compare`, dashed links), 1 % band shaded, `stopping` draws the tolerance window (5 runs) as a bracket on each curve; `stopping_2` units and designs at the stop.
- **V6.** Symbolic rule chips at 18 px: `step = −η · slope`, `try the four neighbours: ± step`, `best neighbour < now → move there`, `step ÷ 2: 20 → 10 % · 0.30 → 0.15 m`, `parent = lower electricity of two random cards`, `child = (windows of B, overhang of A)`, `mutate: windows ±10 %, overhang ±0.2 m, p = 0.5` (values read from the study JSON); ledger and slope formulas 16 to 18 px.
- **V7.** `pattern_5` narration: "until no neighbour is better"; chip "still 1.9 kWh/a above the map minimum 494.8" counts down live. `mutation_2` caption names the baseline ("worse than the crossover alone").
- **Minors.** V9 chips moved clear of the dashed line; V10 result chips merged into two non-overlapping chips with the start value; V11 first-generation map dots are dashed until counted; V12 German "noch kein Bestwert", "Mutation", typographic minus in rule chips (signed values come from `quantity`); V13 "simulations" after the run counts; V16 cards sorted by kWh/a every generation (ranking visible), stars on the inside corner; V17 "limit: no move" note; V18 "arrow length ∝ cooling demand" legend; V19 close chip "full map minimum" with the map and trails.
- **Not done (minor, reported):** V16 thumbnail scale stays at 0.13 (the card text carries window share and overhang in numbers); V20 no start room next to the result room (the result chip prints start → result and gap instead).
- **Verification:** `capture.py` en + de 43 frames each, every English frame read after the final layout change, German key frames read (task_2, gradient_3, gradient_4, gradient_result, pattern_4, pattern_5, crossover, mutation_2); strict check 3 findings in 2 frames per language, same justified false positives as round 2 (map_2 and overshoot chips on map lines, hidden behind opaque chip frames). Unit tests: 70 tests OK (`🧪️tests/🐍️.py`, 824 s).

## Round 4

Source: `final-verify-pareto-algorithms.md` (1 blocker, 10 majors, 14 minors). Patch scripts: `algorithms-patch-r4.py`, `algorithms-r4-wording.py`, `algorithms-r4-layout*.py`; checks: `algorithms-r3-arithmetic.py` (rewritten, `PYTHONUTF8=1`: 218 designs, 0 problems: every ledger row, the loss bridge and the sum close from the printed numbers, and every printed part is within 0.06 and the total within 0.16 kWh/a of the model), `algorithms-r4-wrap.py` (caption wraps), `check-algorithms-narration.py` (0 failed facts).

- **A-B1 / A-M1 (before and after).** Pattern beats print the move explicitly: `before 497.8 kWh/a → after 496.7 kWh/a (moved to the best neighbour)`. Chip, bar, rooms and ledger are the "after" design in every moved round (pattern_3 `before 744.7 → after 648.8`), and the remaining-distance chip reads "after: still 1.9 kWh/a above the map minimum".
- **A-M11 / A-M3 (one precision, derived rows).** The scene draws its own stacked bar (`_bar`, the toolkit's `energy_bars` has no `digits`) whose total is the printed sum of the ledger at one decimal. Every ledger row is derived from the numbers printed in it (`_figures`: loss − gains = heat demand; heat demand ÷ COP; cold demand ÷ EER; 0.432 kW · 1,500 h/a · share; the three parts add up to the total), so gradient_4 prints `165.1 ÷ 3.5 = 47.2` and the total follows (`744.7 − 131.9 = 612.8`). Cards, chips and the step rows use the same printed totals; the "≈" of round 3 is gone.
- **A-M2.** Gradient probe cards print the values the slope uses (748.2 / 741.2; 746.327 / 743.053 at three decimals), so `(743.053 − 746.327) kWh/a ÷ 0.024 m = −136 kWh/a per m` can be recomputed from the cards; the rule chip defines "η: one fixed step factor".
- **A-M4, A-M5, A-M6.** Crossover bar labelled "bar: parent A"; the stale design chip is removed when the population starts; the façade cost is introduced as "one-off façade cost 21,260 CHF" in the map-minimum chip of `close`.
- **A-M7, A-M8, A-M9.** "zoom × 11" in a chip; the lamp factor reads "on 18.090 %"; probes at a bound say "at limit 80 %" / "at limit 0.00 m", "limit: no move" only when the displacement is zero.
- **A-M10.** Captions reworded so no number is split from its unit at the line break (`algorithms-r4-wrap.py`: remaining breaks are after commas); the shared wrapper still splits U+00A0, which is why non-breaking spaces were not used.
- **Minors.** A-m1 ring and chip "ring: within 1 % at 16" for the 16-run point in `pattern_result`; A-m2 results read "if stopped after N simulations" and the counter is replaced by a bracket legend in `stopping`; A-m3 what-if cards say "what-if, not simulated"; A-m4 the band label prints `494.84 × 1.01 = 499.8`; A-m5 no spaces before commas in the rule chips; A-m6 mutate chip clear of the cards; A-m7 survival legend names the yellow and the green frame; A-m8, A-m9 cards print one decimal like the design chip; A-m10 result chips print one decimal; A-m11 star legend "map minimum" (German "Minimum") beside the map; A-m12 change chip says "change to the previous iterate"; A-m13 the bar appears together with its parts in `task_2`; A-m14 counter replaced in `stopping`, brackets explained.
- **Verification.** `capture.py` en + de, 43 frames each; strict check en 6 findings in 2 frames and de 2 findings in 1 frame, the same three false positives as before (chips on map lines in `map_2` and `overshoot`, hidden behind the opaque chip frames). All English frames of the changed beats and the German key frames were read. Tests: 70 OK (`🧪️tests/🐍️.py`, 368 s).

### Round 4b (cleanup)

The scene-owned `_bar` is gone: the bar is `energy_bars(self.focus, BAR_AT, BAR_H, 900, compact=True, intensity=False, digits=1, size=13, rule=False, total=...)`. The toolkit gained two compatible keywords with Edit: `total=fn(design, found)` (the scene prints the sum of its own printed ledger parts, which can differ by 0.1 from the apportioned model total) and `rule=False` (omits the vertical scale rule that crossed the total label); `digits` came from the pareto owner. German what-if cards read "nur Kreuzung / nicht simuliert" and the German band chip "1 % darüber: 494,84 × 1,01 = 499,8" (both read in frame); the `Simulationen bis 1 %` header and the stopping rule panel moved right so they no longer touch the map legend or the budget line. Final: en and de 43 frames, strict check 2 findings in 1 frame per language (the `overshoot` chip hidden behind its opaque frame), `algorithms-r3-arithmetic.py` 218 designs 0 problems, 70 tests OK.
