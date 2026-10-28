# Scene report: Multiple Criteria and Index (`⚖️index`, `Beat4_NormalizedIndex`)

File owned: `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py` (class, `construct()` unchanged; `TITLE` de lower-cased `normierter` by the narration audit). Storyboard: `storyboard-index.md`. Audits: `audit-index.md` (fast read-only pass) and the index rows of `audit-narration.md`. Tools in `index-tools/` (`run.sh`, `part.sh`, `partial.py`, `smoke-trace.py` = smoke with the clause trace, `probe-sweep.py` = winners / switch shares / cap, `narration-patch.py` = the audit rows as an assertion-checked patch, `scene-tail-*.py` = pass-two source of the scene class), narration check `check-index-narration.py`.

## Smoke blocker: root cause and fix

`smoke.py ⚖️index … en` failed at scene time about 95 s in `screen_rulers` (`matrix` step) with `ValueError: zip() argument 2 is shorter than argument 1` from `Animation.get_all_families_zipped`. The trace (`smoke-trace.py`, prints the failing animation class and family sizes) showed two separate causes:

1. My local `live()` helper (updater that replaces `submobjects`) was copied together with its updater by every `copy()` that Manim makes of an animated mobject; the copy rebuilt itself while the original stayed frozen. Fix: `live()` now returns the toolkit's `LiveGroup` (static twins on copy, frozen while animated, optional `key` memo); `freeze()` and the `UpdateFromFunc(freeze)` step are gone. Bound methods must not be passed as builder or key (`__deepcopy__` would deep-copy the whole scene through the bound `self`: `cannot pickle '_thread.lock'`); all builders and keys are lambdas or nested functions.
2. After that, `FadeOut(panel)` of a plain `VGroup` that holds `LiveGroup`s still failed (`family 98 start 100`): `Transform.begin` aligns the live original with its copy and then `LiveGroup.__deepcopy__` refreshes (rebuilds) the original once more for `starting_mobject`, so the original's family no longer matches the copies. Fix in the scene: `Retire(group)` fades a live group out through a static twin made when the animation starts (used in `next_ruler`, the matrix fold, the range fold and the overridden `clear_stage`). See "Toolkit requests" for the proper fix.

Result: full real playback of `Beat4_NormalizedIndex` en and de, see Verification.

## What the scene shows now (36 clauses)

| Screen | Clauses | Content (all numbers from the model) |
| --- | --- | --- |
| A cards | `variants` … `best` | room chip "Seminar room 8 × 6 × 3 m · 48 m² · Zurich · simplified annual teaching model"; five cards open with large section pictograms and "same room 8 × 6 × 3 m", then the frames grow, the pictograms shrink and window / overhang / insulation lines and the four raw values (2 decimals for kWh/(m²·a) and daylight) appear with their unit and kind (CHF one-off, kg CO₂e embodied); green frames on each best value |
| B units | `units` … `units_3` | unit chips with a cross; naive bars of the printed numbers; kCHF / t re-sorts the ranking; heading "Still a sum of unlike units" |
| C rulers | `ruler` … `matrix` | one ruler per criterion (electricity intensity, façade cost, daylight factor mirrored, embodied carbon); focus ring and room; panel per criterion: section + kWh bars + the line `501 kWh/a ÷ 48 m² = 10.44 kWh/(m²·a)`, an own 8 × 3 m façade elevation with the price, daylight section with factor and autonomy, elevation with the carbon cloud; "0 = best of the set · 1 = worst of the set" from `shift_2` on; matrix fold into the table |
| D table | `matrix` … `sensitivity_3` | column heads with icon, name, unit and direction (percent from `weights` on, "equal weights" first); the focus row (frame) moves the room panel on the right: title, section, "window · overhang · insulation" and the four raw values; `weights_3` visits all rows; `index_2` moves the focus to the winner; winner pulse; weakest criterion named "weakest: Daylight 0.72"; cost-driven client (coin glyph): weights tween and the rows re-rank in the same clause, then dashed "was 1st" frame, swap arrow, and the room swaps |
| E sweep | `sweep`, `sweep_2` | ranks against the cost weight, winner strip with thumbnails, switch shares found by bisection (4 % and 39 %), axis title, "at 0 %" / "at 100 %", close-call legend, room panel of the live winner with its raw values |
| range | `range` … `range_3` | cost ruler with a sixth design, "1.30 > 1: beyond the old range", best frame and worst frame (moves from Shaded to Premium in `range_2`, elevation of the live focus with its price); board with cost in CHF per row, rank change `old → new` and the flip arrow |
| F record | `record` … `next` | decision record card; `limits`: façade-cost ruler in CHF with a mandatory limit of 21,000 CHF (derived: next 500 CHF above the winner's cost), Bare and Shaded struck out, the three allowed designs ranked by index with their costs, the winner's room and "20,544 CHF < 21,000 CHF"; `pointer`: cost × electricity scatter with design names and the Pareto front; `next`: the winner's room (section, kWh bars, the four raw values) with "Index 0.40 · lowest of five" |

## Construction

- Trackers: ruler `xb`, `g`, `sigma`, tick opacities; board weights, row positions, `t`, `compact`, `fill`, `mode` (unit line → percent line and weight strip), `cur` and `focus` (the focus row; `Board.cursor()` returns the id and the frame height, so the frame follows its row while rows re-rank); `slot_frame.level` for the frames on the range ruler.
- `variant_panel(rt, getter)` is the one room panel (board, sweep, limits); `elevation(rt, point)` is the own façade drawing; `Retire` and `clear_stage` fade live groups safely.
- Narration: the index rows of `audit-narration.md` are applied (design / Entwurf, electricity intensity / Strombedarf pro m², daylight factor, embodied carbon, opening question, closing answer). Decisions of the coordinator followed: ß kept (`größten` stays), Vordach, intensity naming. On screen: the ruler heads use `Electricity intensity` / `Strombedarf pro m²` and the model titles; chips are shortened to `Electricity / Strombedarf`, `Embodied CO₂e / Emissionen`.

## Audit items (audit-index.md)

Fixed: `limits` (cap, variants, room), `sensitivity` (rows re-rank, "was 1st"), `range_2` (worst frame moves to Premium), `cost` elevation (own drawing: no slab at d = 0, punched windows as four 1.5 × 1.0 m openings, ribbon as band, model shadow), `variants` (room chip, large pictograms, disclaimer, Zurich), `index` / `index_2` (focus row + room), axis wording ("best of the set"), electricity conversion line, `daylight` (autonomy defined), weight chips (spacing 1.68, unit line first, DE no longer touching), `range_3` gutter (frame height = pitch − 0.06), weakest criterion defined, `shift_2` decimals (2 digits), 0–1 legend from `shift_2`, `sweep` (switch shares, axis title, rank headers), `sweep_2` (legend), unit notation ("Electricity intensity, kWh/(m²·a)"), matrix column heads, disclaimer, units captions, weights labels ("equal weights", "weights"), `next` labels, persona icon, true "weights" strip label, secondary text raised to 15 px.

Rejected or deferred (reason):

- DE title `Normierter`: applied via the narration audit (transcript regeneration is the coordinator's).
- "Premium-Glaswand" hyphen and DE room-bar words `Licht` / `Kühlen` / `Heizen`: model title / toolkit labels, not in this file (toolkit requests).
- `1 m` dimension labels at about 9 px and the dimension of d: toolkit `room_section` with `labels=False`; the overhang depth is now written in the spec line and the elevation chip instead.
- First thumbnail of the sweep strip unlabeled (0–4 %): the left rank-1 name of the column names it; a label would collide with the Ribbon label.
- Duplicate tick `1` and value label `1.00` on the normalized ruler: kept (the tick is the scale, the label the value).
- Minus sign "-10.4": the font draws the true minus U+2212 like a hyphen; no code change possible.

## Toolkit requests

1. `facade()`: omit the overhang slab when `d < 0.01 m` and draw WWR ≤ 30 % as discrete openings (four 1.5 m wide openings, same area); this scene uses its own `elevation()` meanwhile.
2. `LiveGroup.__deepcopy__` must not refresh the original (or `Transform.begin` must not copy twice): `FadeOut(VGroup(LiveGroup, …))` raises the zip `ValueError` (family sizes differ between `begin` and `interpolate`). Workaround here: `Retire`.
3. `energy_bars`: `digits` argument for the intensity (prints `10 kWh/(m²·a)` under a bar whose total is 10.44); German segment labels as nouns (`Beleuchtung`, `Kühlung`, `Heizung`).
4. `room_section`: dimension labels `1 m` / `6 m` are about 9 px high; the `1 m` scale bar is not the overhang depth: dimension d directly.
5. `room_section` shows direct sun rays for the daylight factor (diffuse sky): a diffuse mode with 2 % / 5 % daylight isolines would match the criterion.

## Verification

See the final report in the chat (frames, smoke, tests).


## Round 2

Scene file rebuilt around one persistent room panel (sources of the build: `index-tools/r2-part1.py`, `r2-part2.py`, `r2-part3.py` and the `r2-patch*.py` steps; the scene file itself is the only product file touched). Audit: `audit-r2-index.md`. Numbers check of every printed equation: `check-r2-index-scene.py` (all six variants, both weight sets) and `index-r2-check-norm.py` (normalized values from the printed raw values). Frames: 42 keys, en and de, strict layout findings 0 in both languages (see "Layout" below).

### Changes against the audit

| ID | Fix |
| --- | --- |
| D01, D08, D21 | carbon and cost as physical build-ups: stacked bar of layers (masonry, insulation, glass, concrete overhang; fixed items, wall, insulation, windows, overhang) with area × rate = kg CO2e or CHF per layer, the layers tinted in the section (`materials_overlay`); new clauses `cost_2` and `carbon`; the carbon ruler has its own beat |
| D02, D07 | daylight: toolkit sky light and daylit-depth zone, gauge of the daylight factor against the 2 % limit, sky angle, substituted formula DF = T · A_w · θ / (A · (1 − R²)), lamps dim with the autonomy; heating drawn as winter section with red heat arrows and its own clause `ruler_4` (U values, COP); cooling `ruler_3`; lighting and sum `ruler_5`; every row is a written equation |
| D03, D09, D10 | a room panel on all 42 frames (interior 4.3 units of 14.2, 32 % with the walls), the cards became a table of five rows with a cursor that moves the room; raw values of the room in a strip under it, the active criterion framed |
| D04, D11 | parts and sums use `apportion` (largest remainder) and the digits are searched until the printed equation reproduces (725.1 kWh/a ÷ 48 m² = 15.11); electricity ruler values with three decimals so that 0.71 follows from the printed numbers |
| D05, D13 | `BUDGET_CHF = 21000` in the scene data, narrated in `limits` (client budget, mandatory limit before any weights); it removes the shaded and the bare glass wall; the allowed three are normalized again (Ribbon 0.32, Punched 0.60, Super-insulated 0.90) in `limits_2`; `CAP_STEP` and the `ceil` rule are gone |
| D06 | `index_3` writes the index of the focus row (`0.40 = 0.40 × 0.10 + 0.30 × 0.55 + 0.20 × 0.72 + 0.10 × 0.50`), `next` the full table raw value → normalized × weight = part; the printed weights get as many digits as the product needs (0.229 for the cost-driven set) |
| D12 | close-call bands from bisection on the gap 0.03 (0 – 10 % and 35.9 – 42.5 %) |
| D14, D15, D26 | scale labels under bars removed; unit "kWh/(m²·a)" and true minus everywhere; the overhang depth is dimensioned in the panel (`d 0.6 m`, "no overhang" with a leader) |
| D16, D17, D18, D19, D22, D23, D24, D25 | Pareto: dominated designs grey with "grey = dominated by Ribbon window"; Premium's 10.41 kWh/(m²·a) in the value strip at `range_3`; "weakest = highest normalized value"; limits layout (label below the zone, no tick label on the limit line); gutters; weights_3 frames labelled "strongest / weakest pull"; sensitivity arrow replaced by the dashed "was 1st" frame; secondary text at least 11 px |
| D27, D28, D29, D30 | nouns Beleuchtung, Kühlung, Heizung; "Graue Emissionen" in every DE string and chip; "Rang" left of the axis; footer "Simplified annual teaching model · Zurich-like climate · EN ISO 13790 style" (en, de) on every frame |

### Final beat list (42)

variants, variants_2, criteria, criteria_2, best, units, units_2, units_3, ruler, ruler_2, ruler_3 (cooling), ruler_4 (heating, winter), ruler_5 (lamps and sum), shift, shift_2, mapped, cost, cost_2 (price parts), daylight, carbon (material layers), matrix, weights, weights_2, weights_3, index, index_2, index_3 (weight × normalized), winner, winner_2, sensitivity, sensitivity_2, sensitivity_3, sweep, sweep_2, range, range_2, range_3, record, limits (budget), limits_2 (allowed three), pointer, next.

Shortened captions: `units`, `units_3`, `ruler_3` (no orphan words). The number of clauses grows from 36 to 42.

### Verification

- Frames en and de read one by one; strict layout block 0 findings (`capture-*-11.log`), the layout report's older check lists only false positives noted below.
- `python 🧪️tests/🐍️.py` with `SEMIO_TEACHING_OUTPUT_ROOT`: OK (all clips, narration lengths, bilingual completeness, caption fit).
- `check-r2-index-scene.py`: all equations hold as printed; the single exception is the electricity block of the sixth variant (Premium, heating 124.8 ÷ 3.5 would print "≈"), which is never shown (Premium appears only in the cost block and the value strip).

### Remaining risks

- The toolkit still draws a `1 m` scale bar left of the room (about 9 px text); it sits under the room, away from all scene content.
- `capture-until.py` does not run the strict block; only the full `capture.py` run certifies zero.
- Rendering time grows: the panel redraws a 100 – 125 ms section during every move; the whole capture takes about 7 minutes per language on the RDP host.
- DE labels in the board header are tight (Strombedarf, Tageslicht, Graue Emissionen); chips are spread 1.18 units apart and offset by up to 0.2 from their equal-weight columns, linked by lines once the weights differ.


## Round 3

Verifier report `verify-r2-index.md` (1 blocker, 4 major, 15 minor). Sources of the build: `index-tools/r2-part1..3.py` plus `r2-patch8.py`, `r2-patch9.py`; checks `check-r2-index-scene.py`, `index-tools/caption-wrap.py` (orphan check of every caption, en and de).

### Changes

| ID | Fix |
| --- | --- |
| B1 | the value strip under the room (every frame) now carries a mini stacked bar per criterion in the layer colours: kWh heating / cooling / lighting, CHF fixed / wall / insulation / windows / overhang, kg CO2e masonry / insulation / glass / overhang, daylight factor against the 2 % mark; the parts of the design on show move with the room |
| M1 | `range_2` narration is now true: "Premium: worst cost, best electricity; all other values shrink." (de: "Premium ist am teuersten und am sparsamsten; alle anderen Werte werden kleiner."); the beat first stretches the cost ruler and then draws the six-design electricity ruler (Premium 0.00 at 10.41, Shaded 0.01, Ribbon 0.11, Super-insulated 0.64, Punched 0.71, Bare 1.00) |
| M2 | winter heat loss is drawn from the façade: red arrows through the opaque wall and the glass, count by position, weight and length bound to the heating demand; `next` shows the same arrows with "winter loss 183 kWh/a" and "summer cooling 816 kWh/a" next to the teal cooling arrows |
| M3 | the budget is authored in the model: `brief.budget_chf = 21000` with a bilingual note in `🔣️.json` (Edit only), required in the schema, read by `brief()`, case `brief` in `🧪️cases/🔣️.json`, test `test_client_budget_is_authored_and_excludes_exactly_the_stated_variants`; the scene reads `_model.brief()`, the typed constant is gone |
| M4 | heating: written basis `H = U·A wall + U·A window + other = 20.87 W/K`, `× 70,000 Kh/a = 1,461 kWh/a loss − 1,107 free heat = 354 kWh/a` and the toolkit `derivation("heating")` (demand ÷ COP); footer "Simplified annual teaching model · degree-hours method · EN ISO 13790 style" (no SIA claim: the model docs name the degree-hours method and "EN ISO 13790 style" utilization) |
| m1, m2, m3 | the span prints 4.669 (endpoints rounded to the printed three decimals); minus sign for the best value; the first offset tick carries the unit (+2 kWh/(m²·a)); the unit notation is now the model's kWh/(m²·a) through the toolkit row formatter |
| m4 | the overhang dimension is a chip on the top layer (`z_index` above the rays); the table row frame stops at x = 0.1 clear of the room |
| m5, m6 | daylight frame labels "sky light · in DF" and "direct sun · not in DF" for the rays; "Daylit ≥ 2 % to 4.9 m of 6 m (linear falloff)" states the assumption |
| m7 | the rank-change frame is a pair of dashed rules plus the tag "was 1st" on row 2; the grey bracket marks and the unlabelled arrow in `range_3` are gone (the rank column "old → new (orange: changed)" carries the meaning) |
| m8 | weight chips keep their link lines (a centred chip would overlap at 20 % and 10 %); the 10 % label stays outside only where the segment is under 0.45 wide, consistently |
| m9 | all captions rebalanced: `caption-wrap.py` reports no one-word last line in en or de |
| m10 | DE terms: "Fixposten" (chip and narration), "Graue Emissionen" in `units_3`, "Tageslichtversorgt" for daylit; the toolkit's "Premium-Glaswand" hyphen is the model title |
| m11 | the sweep winner strip is coloured per design (Shaded cyan, Ribbon green, Punched purple) and the end names carry the winner colour; the first thumbnail is covered by the colour legend of its strip |
| m12, m13, m15 | index values right-aligned inside the column (no ground-line hit); record labels on one baseline; `next` prints three decimals (0.545 × 30 % = 0.164, index 0.401 reproduces) |
| m14 | table thumbnails enlarged to about 80 px |

### Verification

- Both languages captured, all 42 + 42 frames read as stacked pairs, strict layout block: 0 findings in en and de (last full capture).
- `check-r2-index-scene.py`: all equations hold as printed; only the never-shown Premium heating row differs. `check-index-narration.py`: ok.
- Full test run: 63 of 64 OK; the failure is `StudyModel.test_evaluate_matches_a_vectorized_numpy_reimplementation` (heating differs for 41 of 300 designs) and comes from the model agent's concurrent change of the shading and beam reference in `🧮️model/🐍️.py`, not from the `brief` additions (new test and schema tests pass).

### Remaining risks

- The matrix row frame still touches the thumbnail of the next row by a few px (old D24); weight chips are linked, not centred (m8).
- `derivation("heating")` writes "Heizen" in the DE head row (toolkit word); the other heating labels say "Heizung".
- Printed values come live from the model; the model agent's open beam-reference change can move heating values (the scene recomputes, the checkers must be rerun after it lands).


## Round 4

Final verifier `final-verify-experiments-index.md` (3 major, 7 minor), build step `index-tools/r2-patch10.py` plus small follow-ups on the `r2-part*.py` sources.

| ID | Fix |
| --- | --- |
| I-M1 | `ruler_4` re-laid out: heading, conductance, U values at the top; two plain-word lines (loss − used free heat = demand; demand ÷ COP = heating) replace the colliding formula line; the toolkit `derivation("heating")` runs with `words=False` below them, clear of the sun disc; no text under the sun or rays (strict check 0) |
| I-M2 | `range_2`: "Premium is dearest and most frugal: costs shrink, electricity values shift a little." (de: "Premium ist am teuersten und sparsamsten: Kosten schrumpfen, Stromwerte verschieben sich."), true for both rulers (cost values shrink, electricity 0.00 / 0.10 become 0.01 / 0.11) |
| I-M3 | the weight strip prints one decimal when the set needs it (22.9 / 60.0 / 11.4 / 5.7 %), so the index line `0.229 × 0.706 + 0.600 × 0.000 + 0.114 × 1.000 + 0.057 × 0.000 = 0.28` reproduces from the printed weights; the `next` table uses four decimals for the products (0.0416 + 0.1635 + 0.1444 + 0.0505 = 0.4000) |
| I-m1 | "winter heating demand 183 kWh/a" (de: "Winter: Heizbedarf") |
| I-m2 | carbon product printed as 0.0505 |
| I-m3 | one unit notation everywhere, the toolkit's kWh/m²a (all plain strings go through it; also W/m²K, CHF/m²/cm, kg CO₂e/m²/cm) |
| I-m4, I-m6 | derivation head words removed, the heating labels say "Heizung"; the loss appears once (1,460.7 kWh/a) |
| I-m5 | `variants_2`: "Only window share, overhang and insulation are compared." (de: "Verglichen werden nur Fensteranteil, Vordach und Dämmung.") |
| I-m7 | de "stärkster Einfluss" / "schwächster Einfluss"; the two chips are anchored at the left and right edge so they no longer overlap |

Verification: both languages re-captured (strict 0 in en and de), changed frames read as stacked pairs; 70 of 70 tests OK; `check-r2-index-scene.py` unchanged (only the never-shown Premium heating row differs). The old layout check lists "object under 11.4 %" for the narrow yellow weight segment in `sensitivity*`: the label sits on its own coloured segment by design (strict check: none).
