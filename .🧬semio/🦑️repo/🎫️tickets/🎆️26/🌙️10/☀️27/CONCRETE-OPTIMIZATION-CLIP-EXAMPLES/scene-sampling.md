# Scene Report — Sampling the Design Space (`🎲️sampling`, `Beat2_GridRandomSampling`)

File: `🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py` (only file edited). Storyboard: `storyboard-sampling.md`. Class, `TITLE`, `construct()` → `self.present("sampling")` unchanged; `NARRATION` is a literal list of 33 `[key, en, de]` rows.

## What the viewer sees

One question, one room, one currency (kWh/a). Plane: window-to-wall ratio WWR 10–80 % (x) × overhang depth d 0–1.2 m (y), physical ticks, dashed design-space box. Every point is a section pictogram (`thumbnail`): ghost (dashed grey) until simulated, then filled with the shared ramp colour. Beside the plane the same design lives in two `room_section`s (winter 19°, summer 66° sun), a stacked electricity bar on one fixed scale (surface maximum) and a live "WWR · d" chip. A scoreboard (9 budget pips of the running set, one row per set with marker shape, name, best-so-far kWh/a) persists.

Acts and keys: example (`parameters`, `parameters_2`: façade elevation, two sliders, section pictogram → plane with live marker), one simulation (`run`, `run_2`, `run_3`: both seasons, bar fills with a progress tracker, point takes colour and number), budget (`budget`), full grid (`grid` … `grid_4`: eight ghosts, runs 2–9 swept with room, rays, bar, colour, pip, best-so-far; ring on the best), growth (`growth` … `growth_3`: four factor columns incl. orientation plans and insulation layers, 108-pip block with the 9-pip budget frame, "99 over budget / 12 × budget", mᵏ), random (`random`, `random_3`, `gaps`, `gaps_2`, `gaps_3`: recorded-draw table, sweep, dashed largest empty box with limits and a "not simulated" ghost room, closest pair with Δ WWR / Δ d / Δ kWh/a and the room alternating between both), Latin hypercube (`lhs` … `evaluate_2`: nine strata, one sample each with edge ticks, vertical and horizontal strips with interval and count, 9 of 81 cells, largest empty box, ghosts stay "position only" until the sweep), reveal (`reveal`, `reveal_2`: 27 set markers filled with ramp colours, dense-grid heat map column by column with contours, counter 41 × 41 = 1,681 simulations, valley line, star and room at the optimum), compare (`compare` … `compare_3`: four tiles — best of grid, random, LHS and the dense-grid optimum — each with room, bar on the shared scale, kWh/a and Δ in kWh/a and % plus a Δ bar on one shared scale), limits (`limits` … `limits_3`: ≤ +5 % contour, "1,681 simulations → optimum" vs "9 → best of nine", closing line, model note).

The very first simulation (baseline 45 % / 0.6 m) is grid run 1 of 9 (the grid centre).

## Numbers (all from the model; current values)

Surface minimum WWR 66 %, d 0.99 m, 494.8 kWh/a. Best of nine: grid 499.2 (corner 80 %/1.2 m, +4.3 kWh/a, +0.9 %), random 561.8 (+67.0, +13.5 %), LHS 495.9 (+1.0, +0.2 %). Random: largest empty box WWR 35–70 % × d 0.15–1.20 m, closest pair #3/#4 (Δ 2 pp, 0.06 m, ≈ 6 kWh/a). LHS strata 7.8 pp × 0.13 m. Nothing in the file is typed by hand except structural words; every on-screen number comes from `samples`, `lhs_strata`, `sampling_diagnostics`, `surface`, `levels`, `evaluate` (via the toolkit) or arithmetic on them (`len`, products). Differences between the sets are legible through Δ in kWh/a and %, a Δ bar on a shared scale (random's 67 kWh/a fills 1.4 u; LHS's 1 kWh/a is a sliver) and bars on the fixed full scale.

## Narration contract

Narration contains only structural numbers (nine, 81, 108 = 3·3·4·3, twelve times, three/four levels) and qualitative claims that `check-sampling-narration.py` (T) verifies against the model: nine samples per set; levels 3/3/4/3 → 108 = 12 × 9; 9 strata, 81 cells; one LHS sample per stratum on both axes; no random draw with WWR ≥ 55 % and d ≥ 0.8 m and the empty box reaches the deepest overhang; random closest pair is close; best of each set stays above the dense-grid optimum; optimum interior; LHS best < random best; clause length ≤ 95 characters; every `self.step` key has a clause and vice versa. After the latest model change the script reports OK. Re-run it after every model change: `python -I T/check-sampling-narration.py`.

## Timing

33 clauses. Caption reading time (German word count / 1.6 words per second, the floor `hold_for` enforces) ≈ 204 s; real low-quality render (`T/render-sampling.py`) measured the final length (see the end of this file). Rendering takes ≈ 2.5 × real time because live widgets are rebuilt every frame.

## Implementation notes

- `Drive(update)` is a tiny `Animation` that calls a pure function of the progress; it only moves `ValueTracker`s and opacities, so the toolkit's live sections, bars and numbers follow. Every clause is exactly one `self.step(key, *animations)`, which keeps `T/capture.py` (instant states) and the real render identical in end state. Objects that must keep updating while a clause runs are added by a short `self.play` before the clause, because Manim freezes updaters of animated mobjects.
- `_fader(group)` fades static groups by scaling their registered opacities (`set_opacity` would fill unfilled outlines).
- Sweeps (`_sweep`) evaluate a list of samples: the shared design trackers move (room, rays, lamps update live), a progress tracker fills the bar segment by segment, the ghost cross-fades into the ramp-coloured thumbnail, the pip lights and the scoreboard best updates — all as one pure function of the clock.
- The scene keeps its own bar (`_bars`) because the toolkit's `energy_bars` has no progress; it matches the toolkit look and uses the same `column`/`text` helpers.

## Toolkit gaps (for the coordinator)

1. `room_section` cannot hide rays for a not-yet-simulated design or draw rays progressively; the scene shows a large ghost pictogram instead and fills the bar itself. A `rays=False`/`progress` option would let the room stay on screen.
2. `room_section` always draws the "1 m"/"6 m" dimension lines; at compare-tile scale (0.17) they touch the text below. A `dimensions=False` option is wanted.
3. `thumbnail(state="ghost")` and chips cannot be faded with `set_opacity` (outlines get filled); a `thumbnail(..., opacity=)` or a documented fade helper would remove the scene-local `_fader`.
4. `heat_map` contour labels are placed without avoiding markers (e.g. "600 kWh/a" under a set marker).
5. `energy_bars(progress=…)` (fill by a tracker) and a `total_label=False` option.

## Verification

- `T/capture-sampling.py` (copy of `capture.py` with a private media folder; parallel captures of different agents collide in `🗑️generated/frames/media`): en and de, 33 beats each, layout reports "none"; every PNG viewed (`🗑️generated/sampling/frames`).
- Narration check script: OK. Contract tests `OptimizationClipContract` and render measurement: see below.

## Polish Pass 2 (owner: "unsophisticated, nothing abstract")

Only `🎲️sampling/🐍️.py` changed (plus scripts `capture-until.py`, `check-sampling-captions.py`, `fix-captions.py` in the ticket). Every audit finding was verified on the frame first.

### What changed

- Right panel redone: design chip moved to the top right (no longer on the 6 m dimension line), season captions, sections, bar and board on separate rows; board shifted down (Run row, pips, header `Best so far in kWh/a`, rows). Bar fixed scale, total label left-aligned beside the bar (no crowding), cost line `Cost 131 CHF/a` from the model tariff (0.25 CHF/kWh, shown once in the note under the intro).
- Intro: the façade elevation now sits next to a live summer section (person, overhang, rays) at the same trackers; slider ends carry units; `parameters_2` shows the section in colour with the design chip under it (replaces the grey ghost C-shape). `run_2` flashes the winter caption (red), summer caption (blue) and the lamp row (yellow) in step with the bar segments.
- Grid: 0.19-scale tiles, tags `522` plus `kWh/a` (right of the tile, left for 80 %), ramp strip one row higher, `grid_4` adds an outlined ghost bar of the worst of nine (846 kWh/a) on the same scale next to the best one.
- Growth: factor pictograms are real coloured sections (cyan glass, orange overhang, green insulation) at 0.15; orientation glyphs rotated correctly with a north tick, labels `90° E`; the letter formula is gone (narration now `Every extra level multiplies the runs`).
- Random: recorded-draw table has a section thumbnail per column and unit headers (`WWR in %`, `d in m`). `gaps`: the room moves to the empty box centre with an empty dashed bar `? not simulated` and a `?` ghost on the plane (instead of the grey ghost beside the room); `gaps_2` brings the bar back; `gaps_3` chip `best of nine draws 561.8 kWh/a`; rings r 0.25.
- LHS: no more stale room; `lhs`, `marginals`, `cells`, `evaluate` keep the room on the unsimulated design with an empty bar; strata lines, strips and cells below the thumbnails (z, 0.22 opacity); `cells` labels the dashed box `largest empty box`.
- Reveal: contour labels placed by the scene at free spots (markers, star and each other avoided); while the dense grid is uncovered the room and bar sweep through the designs (WWR 10 to 80 %, overhang 0 to 1.2 m); works with the new raster `heat_map` (`_fader` fades `ImageMobject` strips by pixel alpha); valley line removed (unlabelled, not in the caption).
- Compare: tile rooms without dimension lines (`_section` uses the toolkit's `dimensions=` if it appears, otherwise drops the four trailing dimension children), subtitle in INK 14, shared colour key, deltas from the displayed 1-decimal values (4.4 / 67.0 / 1.1 kWh/a), whole 2 x 2 spacing re-measured.
- Limits: the optimum room returns, chip `≤ +5 % · ≤ 519.6 kWh/a`, the best-of-nine card carries a value (495.9 kWh/a), closing line and note moved apart.
- Naming: `Latin hypercube` (en) / `Latin-Hypercube` (de) everywhere; `Heizen / Kühlen / Licht` words in the DE caption match the bar.
- Captions: `run_2`, `lhs`, `evaluate_2` (en and de), `parameters`, `grid_2`, `random`, `gaps`, `reveal`, `limits`, `growth_3` re-worded; no one-word last lines, all two lines at most in both languages and audiences.

### Audit items rejected or deferred

- Orientation glyph "rotated by 180°": the cause analysis was right, fixed (`rotate(-bearing)`).
- "Gaps caption says big windows but the box starts at 35 %": kept the claim (the box range is shown by its chip), reworded to `wide window with a deep overhang`.
- Layout checker misses label-on-label collisions (audit last row): true, unchanged here (toolkit/ticket-level); every frame was viewed instead.
- `evaluate_2` 598 vs 599 rounding: no longer present (totals come from the live bar sum).
- Dimension labels about 9 to 10 px (`1 m`, `6 m`) remain on the two main sections: they are drawn by the toolkit (`room_section`), the scene only removes them on the compare tiles.

### Verification

- `capture-all.sh Beat2_GridRandomSampling`: 32 beats en and de, layout reports "none" for every clause; all English frames and the German frames with new text viewed.
- `smoke-all.sh Beat2_GridRandomSampling`: en ok, scene time 242.0 s (4:02); de ok, 226.3 s (3:46).
- `python -I check-sampling-narration.py`: OK; `check-sampling-captions.py`: 32 clauses, 0 failures, 0 short last lines.
- `🧪️tests/🐍️.py SurfaceAndSampling`: 7 tests OK.

### Narration pass (audit-narration.md) and consistency audit (audit-consistency.md)

Applied after checking each row against the model and the picture: concrete opening question (`parameters`: "Which window share and overhang depth suit this south façade?"), simulation/simulate instead of run/Lauf/rechnen, design variables / Entwurfsvariablen, Ausrichtung, point sets / Punktmengen, dense reference grid, Heizstrom / Kühlstrom / Beleuchtungsstrom (the on-screen DE bar and key words follow: Heizstrom, Kühlstrom, Beleuchtung; German scene note `Vereinfachtes Jahresmodell`), kWh/a without "per year", ß kept. Lines over 95 characters shortened (DE `run_2`, `limits_4`), and several lines re-balanced for the two-line caption (no one-word last lines). New closing: `limits_3` keeps the validation remark, new `limits_4` ("Nine simulations found a good south façade design, no proven optimum.") returns the room to the best of nine (68 % / 1.00 m, 495.9 kWh/a, +1.1 kWh/a above the 494.8 kWh/a optimum) with the live bar and a chip on the plane.

Rejected or adjusted: audit's `run_2` en wording kept (heating electricity / cooling / lighting) but the German line shortened without articles; the consistency audit asks to close on the 66 % / 0.99 m optimum room, rejected because the claim is about what nine simulations found (LHS best of nine), the optimum stays visible as the star and in the `reveal_2` and `limits` rooms; subtitle chip / DIN footnote and formula band are cross-clip style items outside the scene file; the rows on grid_4 value boxes, bar total over "Lighting" (column moved right, total 16 px), and the 9 px dimension labels are fixed (the latter by the toolkit).

Final verification after the toolkit change (daylight zone, 14 px dimension labels): `capture-all.sh` 33 beats en and de, layout reports "none" for every clause, final frames viewed; `smoke-all.sh` en ok 254.0 s (4:14), de ok 240.7 s (4:01); `check-sampling-narration.py` OK; `check-sampling-captions.py` 33 clauses, 0 failures, 0 short last lines. The sampling scene uses `labels=False, demand=False` sections, so the new two-line demand label does not appear; the narration says "heating electricity" and "cooling" consistently with the bar labels (Heating/Cooling/Lighting electricity in kWh/a).

## Round 2 (audit `audit-r2-sampling.md`; owner: every plotted point shown by its room, nothing abstract)

Only `🎲️sampling/🐍️.py` changed (plus ticket scripts `sampling-r2-probe.py`, `smoke-sampling-r2.py`; `check-sampling-narration.py` extended). Toolkit used: `room_section` (daylight sky arrows, daylit zone, lamp cones), `formula`, `live`, `apportion`, `constant`, `muted` from `toolkit-r2.md`.

### Changes

- Daylight and lighting (D01, D11): sections now show the toolkit daylight drawing; live gauges under the sections ("Glass in shade 0 % / 89 %", shading_<season>) and a daylight gauge ("Daylight autonomy 66 % → lamps on 34 %"); bar footer "Daylight autonomy"; cards show autonomy, window area, shade.
- Equations (D10): `_panel` in `run_2`/`run_3` (and again `reveal_2`, `limits_4`): `204 kWh/a heat ÷ COP 3.5 = 58 kWh/a`, `736 kWh/a cold ÷ EER 3.0 = 245`, `Lamps 0.43 kW · 1,500 h/a · 34 % on = 219`, `58 + 245 + 219 = 522 kWh/a`, `Bill 522 kWh/a × 0.25 CHF/kWh = 131 CHF/a`; every number live from the model, "≈" where rounding does not close, constants from the study JSON.
- Parts add up (D02): bar labels, cards, formula via `apportion`; evaluate_2 prints 35 + 424 + 140 = 599.
- Cost (D03): "Electricity bill / Stromrechnung 131 CHF/a" (tariff once in the note and in the bill row); the façade cost is not part of this story.
- D04/D21: limits cards "Dense grid · 1,681 simulations → 494.8" and "Latin hypercube · 9 simulations → 495.9"; limits_4 chip names the set. D05: DE table headers via `tr`. D06/D19/D18/D28: captions of `gaps`, `gaps_3`, `run`, `run_2` (DE) reworded and checked against the model.
- Every plotted point has its room: `_visit` moves the shared room through each design in `grid` (8 ghosts), `random` (9 recorded draws with a table column highlight), `lhs`, `evaluate`; `gaps`/`cells` probe the empty-box centre (cross marker); `gaps_2` and `grid_4` show two cards (room, parts, daylight autonomy, window, shade, bar on one scale) for pair #3/#4 and worst vs best of nine; `reveal` has a live ring on the plane that carries the room.
- Smaller: x axis from 0 (10 % column off the axis, D23), plane floor lowered so the bottom tiles clear the axis, pair ring instead of two overlapping rings (D22), one tick legend (D24), ≤ +5 % region filled and threshold from the printed optimum 519.5 (D14, D25), Δ tracks (D26), contour values read on the ramp strip ticks (500, 600, 700, 800 kWh/a) instead of chips sitting on the lines (strict check), empty-box chips with numbers (D13), "1 m" removed by toolkit (D17), growth matrix labelled by orientation and insulation (nine-simulation icon instead of a block).

### Final beat list (33 clauses, unchanged keys)

parameters, parameters_2, run, run_2, run_3, budget, grid, grid_2, grid_3, grid_4 (cards), growth, growth_2, growth_3, random (room tour + table), random_3, gaps, gaps_2 (cards), gaps_3, lhs, marginals, marginals_2, cells, evaluate, evaluate_2, reveal (live ring), reveal_2 (equations of the optimum), compare, compare_2, compare_3, limits, limits_2, limits_3, limits_4 (equations of the Latin hypercube best).

### Verification

- `capture.py` en and de, every PNG viewed; strict layout block brought to zero or justified below. `smoke-sampling-r2.py`: en ok, scene 262 s (4:22). `check-sampling-narration.py` OK; `check-sampling-captions.py` 33 clauses, 0 failures, 0 short last lines.

### Remaining risks

- The equation panel is dense (six rows right of the bar); only on four beats.
- Growth matrix labels insulation 10/20/30 cm while the study used 16 cm; the nine simulations are shown as a separate icon, not as a matrix cell.
- Integer kWh/a on plane tags and bars, one decimal on scoreboard, cards and tiles (stated rule), so 522 and 522.1 can appear on different widgets.
- Scene title is no longer drawn by `ClipScene.present` (shared change), layout assumes free top-left.

### Strict layout (coordinator request)

`capture.py` strict block: en 0 and de 0 findings in 33 frames each (final captures); `OptimizationClipContract` and all 63 tests of `🧪️tests/🐍️.py` OK (narration lengths, bilingual completeness, caption fit). Fixes made for it: contour chips removed (values on the ramp strip), strata lines faded while the `cells` chips are shown, column chips raised above the best-tile ring, y-axis title raised, scene note out of the caption band, card bar scale line removed. No finding is justified as a false positive.

## Round 3 (verifier `verify-r2-sampling.md`: 1 blocker, 4 major, 15 minor)

- Precision rule (B1, M4, m1, m4): one decimal for every kWh/a total and part on all widgets (plane tags, bars, cards, board, tiles, chips, equations); the autonomy and lamp share carry two decimals everywhere; the bill is whole CHF/a computed from the printed total. The panel prints derivations that close: `135.0 kWh/a heat ÷ COP 3.5 = 38.6`, `841.8 ÷ EER 3.0 = 280.6`, `Lamps 0.432 kW · 1,500 h/a · 27.27 % = 176.7`, `38.6 + 280.6 + 176.7 = 495.9`, `Bill 495.9 × 0.25 = 124 CHF/a`. The "≈" is kept only as a guard.
- M1: value tags sit inside their own tile (bottom right), no gutter tags. M2: the tick explanation is part of the strata chip, nothing below the x title. M3: nine numbered badges on the random thumbnails, the table row numbers match; the column highlight and room tour stay. Daylight in the room (m10): note `Daylit ≥ 2 % to 3.8 m` under each room naming the work-plane line; gauges got physical ends (0 % / 100 %) (m11). Bill row appears on `run_3` only, not on `run_2` (m12).
- Minors: bill from the printed total (m1); Δ in the pair callout from the printed values with one decimal (5.9) and a "full length = 67.0 kWh/a" note for the Δ bars (m2); threshold from the exact optimum 519.6 (m3); overhang one decimal where the toolkit chips print one, two decimals everywhere else, gaps caption 0.15 to 1.20 m (m5); probe centre rounded half up (m6); gaps caption "edges excluded", box edge under the thumbnails (m7); cells numbers moved into the top chip, box label only inside (m8); optimum and "LHS best" labelled with leaders in free space, pair ring smaller and removed at `gaps_3` (m9); table pitch widened (m13); German chip capitalised (m14); horizontal strip behind the ghosts, fill 0.10 (m15).
- Strict layout: en 0 and de 0 findings in 33 frames each; narration check OK; smoke render en ok (263 s); `OptimizationClipContract` and `SurfaceAndSampling` OK (narration, bilingual, caption fit). The full-file run had one failure, `StudyModel.test_evaluate_matches_a_vectorized_numpy_reimplementation`, caused by the model agent editing `🧮️model` and the tests while the run was in progress (model files changed 00:24 to 00:28); not a scene issue, my on-screen numbers are unchanged.
- Remaining risks: the Δ bars for Grid and Latin hypercube stay slivers (they are 4.4 and 1.1 of 67.0 kWh/a), now anchored by outlines and the scale note; the thumbnail inside a tile is partly covered by its value chip; motion (tour, sweeps) is verified by the smoke render of round 2 only, not re-rendered.

## Round 4 (`final-verify-extrema-sampling.md`: 2 major, 17 minor)

- S-M1: `limits_2` now reads "A finite sample cannot prove that it found the global optimum." / "Eine endliche Stichprobe kann nicht beweisen, dass sie das globale Optimum gefunden hat."; the closing chip says "Coverage informs the search; it cannot prove an optimum." / "Abdeckung leitet die Suche, ersetzt keinen Beweis." Captions re-broken (`parameters`, `gaps_2`, `gaps_3`, `reveal_2` DE, no orphan lines).
- S-M2: every room label and meter end is size 17 (measured about 14.8 px cap height at 1280 px): "Winter/Summer sun", "Daylit to 3.8 m", "Shaded 89 %" with 0 % / 100 % ends, the daylight gauge, plane ticks and axis titles, the model note (white, moved above the plane on the closing beats). Texts were shortened instead of shrunk ("Daylit to 3.8 m" / "Hell bis 3,8 m", "Shaded", "Daylight 66.26 % → lamps on 33.74 %"). The parameters section runs at scale 0.48 so the German chips no longer overlap.
- Minors: threshold 519.5 from the printed 494.8 (S-m1); badges placed outside all thumbnails (S-m3, S-m4); optimum and best LHS labelled with leaders (S-m5); gaps probe at two decimals (S-m6), box edge under thumbnails (S-m7), cells chip above the box and the label only in the plane, 9 × 9 grid stays visible (S-m8); bar total clear of the meter (S-m9); the plane marker prints `d 0.60 m` once the room appears (S-m10); compare shows "Bars: full height = 846.4 kWh/a" then the Δ scale note (S-m11); the bill moved to `run_3` only (S-m12); DE terms "Ziehung", "Nicht simuliert", "Bester von neun", "Kosten" (panel) and "Stromkosten" (bar), "Bester Latin-Hypercube" (S-m13 to S-m15); growth matrix cells are now drawn dimmed from the start and light up (S-m16). S-m17 (values on the best points) not done: the colour ramp plus the chips of `reveal_2` carry them.
- Strict layout: en 0, de 0 in 33 frames each; narration check OK; full `🧪️tests/🐍️.py` run OK.
