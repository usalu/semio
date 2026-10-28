# Scene Report: Design of Experiments (`🧪️experiments`, `Beat3_FactorialExperiments`)

File: `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️experiments/🐍️.py` (only file edited). Class, `TITLE`, `construct()` unchanged; `NARRATION` is a literal list of 35 `[key, en, de]` rows (one clause changed in round 2: `difference` now reads "half the difference of the two overhang effects" / "halbe Differenz der beiden Vordach-Effekte").

## Round 2

### Changes against `audit-r2-experiments.md` (43 defects)

| Audit | Fix |
| --- | --- |
| D01 sign (blocker) | One convention, stated on screen from `saving_30` to `difference_2` and printed as a chip: "Effect = high level − low level · negative = saves cooling electricity". Overhang effects are −122.3 and −278.9 kWh/a, the interaction is `½ × (−278.9 − (−122.3)) = −78.3 kWh/a`; the narration says "overhang effects". Chip, strip, plot arrows and narration agree. |
| D02, D03 rays | A ray is 0.3 m of glass and carries total sun heat: 204 kWh/a = 122 direct + 82 diffuse (legend, bound to `solar_summer`/7 and `direct_share_summer`). A crossed ray stops its direct strand with a cross at the overhang and continues as a thin pale diffuse strand, so 7 of 7 crossed rays at 70 %/1 m is consistent with 59 % of the sun heat blocked. |
| D04 chain | Exact chain per pair, every step as printed: "Direct sun stopped: 7 rays × 122.4 kWh/a × 0.977 = 837 kWh/a", "Cooling load: 1,700.0 → 863.3 kWh/a (−836.7)", "÷ EER 3.0: electricity 566.7 → 287.8 kWh/a (−278.9)" (30 %: 3 × 122.4 × 1.000 = 367; 938.4 → 571.5; 312.8 → 190.5, −122.3). Operands come from `evaluate` (`cooling_demand`, `cooling`, `shading_summer`). |
| D05 | Two-line ratio chain: rays 7 ÷ 3, direct sun stopped 837 ÷ 367, cooling electricity saved 278.9 ÷ 122.3, all = 2.3. |
| D06, D25 | `repeat_2`: no bars; four grey dots in a dashed band, labelled "schematic, no values"; icons and captions 16 px. |
| D07 | One precision rule: run values, effects and strips one decimal (means two, to close the arithmetic from the printed operands); formula and waterfall integers with largest-remainder parts (`_room.apportion`), coded levels `+1`/`−1`. |
| D08, D11, D42 | Curvature: straight dashed line labelled in a key, the simulated curve from `evaluate` at 50 % window (key "simulated, 50 % window"), value tags 339.4 and 298.3 on the plot, no illustrative curves. |
| D09, D13 | `question`/`factors`: no bar before the centre run exists; the room carries chips "overhang 0.5 m", "window 50 %", "insulation 16 cm" (words, no bare letters), the compass plan shows 8 m × 6 m and the south façade. |
| D10 | `model`: the room follows the coded levels (tour A +1 → −1 → +1, then B), waterfall and substituted rows move with it. |
| D12 | Fraction cards say "10 cm insulation"/"30 cm insulation" in the header; `purpose_4` shows the reason with model numbers: U-wall 0.29 → 0.11 W/(m²·K), cooling 285.9 → 289.4 kWh/a (70 %, 1.0 m). |
| D14 | `repeat`: bar and ghost removed from the room; pair 298.3 = 298.3 kWh/a. |
| D15–D18, D20–D23, D26, D29, D31–D34 | Labels placed by `_near` (tests every side against plotted lines, spines and frames), two-line value labels beside arrows, frames and tags separated, plot legend moved to the free top-left corner, formula strips ≥ 15 px, German placed identically. |
| D19 | `plot_axes(corner=True)`, x range from −0.2 m for the curvature plot, so spine and tick numbers coincide. |
| D24, D27 | The note is the persistent source tag top right ("Simplified annual teaching model · Zurich-like climate · south façade", 15 px, 85 %) and a flow chip "WWR · overhang → teaching model → kWh/a, not measured" in `response_3`. |
| D28 | The ghost 1.0 m outline is driven by a tracker inside the live room (no group swap), the bar and the load label stay on screen in `confirm_3`. |
| D35 | "simulation" everywhere. |
| D36, D37 | Substituted formulas with units in the toolkit's `formula` style (words above numbers): effect A and B as three rows (words, printed operands, means), interaction `½ × [effect at 70 % − effect at 30 %]`, bilinear prediction with the live coded levels, coefficient = effect ÷ 2 lines; hero room (s 0.85) in question, factors, curvature, confirm, repeat; s 0.6 beside the waterfall in `model`; the room's cooling unit label reads "Cooling load 894.9 kWh/a ÷ EER 3.0 = 298.3 kWh/a". |
| D38, D39, D40, D41, D43 | Overhang effects chip in `interaction_2`; "document it" in the confounding chip; run cards carry only the badge, the table maps the coding; ghost tops carry value and "without overhang". |
| D30 | Strict checker used: 0 findings in English and German (see below). |

New in the room drawing (own `_section`, because rays and the legend must match the model): daylight zone and lamp contrast from `_room.autonomy_range`/`daylit_depth` on hero rooms, sun heat per card (`solar_summer`), sun label above the sun, overhang chip above the roof (rays cross the old column).

### Final beat list (35)

`question` hero room + compass plan · `question_2` question, A, B, result chips, sweep · `question_3` fixed sun/room/weather · `factors` A slider, window area live · `factors_2` B slider, shaded share live · `factors_3` coding formulas and corner tour · `plan` four rooms and design table · `plan_2` window and slab highlights · `response` rays, sun heat per room, ray legend · `response_2` bars, table, best and worst frames · `response_3` model flow chip, source tag pulse · `plot` two lines, tags, legend · `plot_2` badges linked to markers · `saving_30` pair 1/2, chain, convention, arrow · `saving_70` pair 3/4 · `interaction` ratios, dashed parallel line, bracket · `interaction_2` overhang effects chip · `main` effect A formula, column frames, means, arrow · `main_2` effect B · `difference` interaction formula, diagonal frames · `difference_2` meaning chip · `model` hero room + waterfall + substituted rows + slope tour · `model_2` centre run, mean · `model_3` effect ÷ 2 lines · `curvature` straight line through the two levels · `confirm` centre room, model 339.4 vs simulation 298.3 · `confirm_2` gap −41.1 kWh/a, substituted model · `confirm_3` simulated curve, 69 % of the saving · `repeat` 298.3 = 298.3 · `repeat_2` schematic scatter · `purpose` checklist and five rooms · `purpose_2` fraction cards · `purpose_3` confounded columns · `purpose_4` contrast, truth, reason · `answer` two rooms, ghost tops, savings.

### Verification

`T/capture.py 🧪️experiments Beat3_FactorialExperiments en|de` completes in both languages. `T/exp-r2-findings.py <en|de>` lists every layout and strict finding once: **0 findings in en, 0 in de** (35 keys each). All 36 en and the de frames were inspected (the de frames of the beats with identical layout were viewed for the hero, formula, plot and fraction beats). Unit tests: see the end of this file.

### Remaining risks

- The legend line (14 px) and the model-flow chip are close to the 14 px floor in German (text is shrunk to ≈ 13.6 px by the width cap, above the 11 px threshold).
- Substituted bilinear rows are exact at the corner and centre states; during the slope tour (B −1 states) the largest-remainder rounding keeps the printed parts summing to the printed total but a part can differ by 1 from the integer coefficient for a moment.
- Real render time: three live widgets (room, waterfall, formula) during the 9 s slope tour; capture uses instant states, a real render was not timed in round 2.
- The scene no longer shows a scene-title chip (the shared `ClipScene.present` stopped drawing it); `clear_stage` keeps only caption and source tag.
- Hero room labels use fixed 14 px chips; at scale 0.85 the room sits at x −3.6, leaving the left 0.5 units free for the chips.

### Tests

`SEMIO_TEACHING_OUTPUT_ROOT=<T>/generated/output python C/tests/py` (full clip suite, run on the final scene file): Ran 63 tests, OK (narration lengths for both languages and audiences, bilingual completeness, native discovery, cases).

## Round 3 (verifier `verify-r2-experiments.md`: 1 blocker, 3 major, 17 minor)

Changes (all in `🧪️experiments/🐍️.py`; the missing-title remark is ignored on purpose, the persistent title stays removed):

- **B1 blocker:** `difference` now computes row by row as printed: `difference = effect at 70 % − effect at 30 %`, `−278.9 kWh/a − (−122.3 kWh/a) = −156.6 kWh/a`, `A·B = ½ × (−156.6 kWh/a) = −78.3 kWh/a` (en and de; no nested brackets, the bundled font draws "(" as "[").
- **M1:** `curvature`, `confirm*` show a hero room per plotted endpoint: two section rooms (0.0 m and 1.0 m overhang at 50 % window) above the plot, each captioned with depth and its simulated kWh/a (439.7 / 235.8); the 235.8 end is tagged on the plot, the straight line is labelled "straight line (model)" with its 239.1 end (m16); the "?" hint is gone (m8).
- **M2:** every act has a `_fonts.section_heading` chip (`_head`, swapped per act: Question, Factors, Plan, Simulate, Overhang saves, Effects, Bilinear model, Confirm the middle, Repeat, Purpose, Fractional plan, Answer; en/de). Main substituted formulas are 20 px (effects A/B, difference, interaction, bilinear rows); the model beat shows the bilinear prediction in two 20 px rows across the stage; colours match the drawn parts (A teal, B orange, A·B tan). Leader lines from terms to drawn parts were not added (risk below).
- **M3:** layout reports are re-read by `exp-r2-findings.py`; strict list is 0 in en and de. The seeded-collision self-test of the checker is a toolkit task and was not done here; instead every visible touch from the verifier was fixed by hand.
- **m1/m2:** bar moved right of the slab (`BIG_BAR` x 0.1) and the answer bars 0.15 right of the wall. **m3:** mean labels placed with `_near`; 287.8 clear of the 239.15 line. **m4:** German `confirm` caption reworded ("Fensteranteil 50 %, Vordach 0,5 m"), non-breaking spaces between numbers and units in the German narration. **m5:** waterfall pitch 0.9. **m6:** ratio line uses signed values (−278.9 ÷ (−122.3) = 2.3). **m7:** mean chip says "mean of 4 runs, unrounded 339.4". **m9:** the chain prints the shading factor ("7 × 122.4 kWh/a × 0.977 shaded = 837"). **m10:** table rule 0.05 lower, note "cards: A grows to the right, B grows upwards". **m11:** `repeat_2` keeps the simulated 298.3 kWh/a room and bar beside the schematic spread, labelled "assumed spread (schematic)". **m12:** question title is "Cooling electricity = f(A, B)", the caption stays the narration. **m13:** façade icon with arrow on the south side. **m14:** `purpose` mini rooms 0.34. **m15:** reason line says "at this design (70 %, 1.0 m)" and the 5.7 is labelled "mean effects". **m17:** counts corrected to 35 here and in the audit.

Final beat list: unchanged (35 keys), headings per act as above.

Verification: `capture.py` en and de complete; `exp-r2-findings.py` 0 findings in both; all 70 PNGs (35 en, 35 de) read. Tests: see below.

Remaining risks: no leader lines from formula terms to room parts (M2 asked for them); `m9` shows the factor in the chain but all seven direct strands are still drawn fully crossed; the toolkit checker still cannot see label-on-fill touches (M3), so zero findings is necessary but not sufficient.

Tests (round 3, final scene file): Ran 64 tests, OK.

## Round 4 (`final-verify-experiments-index.md`: E-M1, E-m1, E-m2, E-m3)

- **E-M1 (terms tied to drawn parts):** every coloured formula term now has a same-colour marker with a short connector on the drawing. Room beats: the room chips carry the formula's letters and colours with their leader lines to the glass and the slab ("A · window 70 %" teal to the window, "B · overhang 1.0 m" orange to the overhang slab) in `question` … `factors`, `model*`, `curvature`, `confirm*`, `repeat*`. Card beats: `_column_tags` places coloured tags with connectors on the frames the term averages: `main` "mean at 30 %" (teal, left column) and "mean at 70 %" (yellow, right column), `main_2` "mean with 1.0 m" (green, top row) and "mean without" (orange, bottom row), `difference` "effect at 30 %" / "effect at 70 %" (tan, both columns). No elbow lines from the formula text itself: the formula block is live and its columns move with the coded levels, so a connector would cross the waterfall and the plot.
- **E-m1:** `saving_30/70` print the stopped sun and the cooling-load change at the same precision: 367 ↔ 938 → 571 (−367) and 837 ↔ 1,700 → 863 (−837); the electricity step stays at one decimal (566.7 → 287.8, −278.9), a different quantity.
- **E-m2:** `purpose_4` panel: the reason is three short lines (heading, U-wall, cooling) in a 5.7-unit column, about 40 px clear of the right edge and clear of the caption band; the chips above moved up by 0.1 and use the same 5.7 width.
- **E-m3:** `verify-check-experiments.py` tests the three-row form (−278.9 − (−122.3) = −156.6, ½ × (−156.6) = −78.3): 0 of 29 checks mismatch.
- Capture en + de complete, 0 layout findings in both; all 70 PNGs open with PIL (1280 × 720, `exp-r4-validate.py`) and the changed ones were viewed (en: main, main_2, difference, saving_70, purpose_4, model, question, confirm_3; de: purpose_4, main_2, saving_30, model_3).
Tests (round 4, final scene file): Ran 70 tests, OK.
