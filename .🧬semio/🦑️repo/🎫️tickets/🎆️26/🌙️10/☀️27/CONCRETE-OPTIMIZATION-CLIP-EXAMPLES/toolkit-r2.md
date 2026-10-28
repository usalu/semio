# Toolkit Round 2: Parts Rounding, Daylight Drawing, Substituted Formulas, Label Defects

Code: `C/🎨️visuals/🐍️.py` (`ClipScene`) and `C/🎨️visuals/🏠️room/🐍️.py` (`RoomVisuals`, module functions, imported by scenes as `_room`). Additive only: every round-1 name and call form still works. Stills: `T/🗑️generated/toolkit-r2/<board>-<en|de>.png`, script `T/toolkit-showcase.py <en|de> [board ...]` (new boards: `daylight-a`, `daylight-b`, `daylight-mid`, `daylight-deep`, `daylight-small`, `formula`, `formula-daylight`, `sum-bars`, `axes-corner`; the report now also lists the strict findings of `layout_strict.py`; unit check `T/toolkit-r2-check.py`, redraw cost `T/toolkit-r2-bench.py`: section 103 → 124 ms with the daylight drawing, formulas 26–29 ms).

## 1. Parts that add up to the total

```python
self.parts(values, digits=0, total=None) -> list[float]   # ClipScene; largest-remainder rounding
_room.apportion(values, digits=0, total=None) -> list[float]   # same function for module-level code
```

`values` are the unrounded parts, `total` defaults to their sum. The returned parts are rounded to `digits` decimals and add up exactly to the rounded total (`round(total, digits)`, the same rounding `number()` prints). Print every part of a stacked bar, a sum row or a table with these, and the total as `self.number(sum(parts), digits)` (or `self.number(total, digits)`, identical by construction):

```python
heating, cooling, lighting = self.parts([found["heating"], found["cooling"], found["lighting"]])   # 34.41 / 424.24 / 139.89 -> 35, 424, 140
total = sum((heating, cooling, lighting))                                                           # 599, the printed total
```

Used inside the toolkit by: `energy_bars` (segment labels, total above the stack, the ghost bar of `reference=`), `electricity_formula` (below). The old failure case (76 % / 0.6 m: 34 + 424 + 140 printed next to 599) now prints 35 + 424 + 140 = 599. Scenes that format parts themselves (own tables, chips, formulas) must call `self.parts`; unit tests of the helper: `T/toolkit-r2-check.py`.

## 2. Daylight and lighting are drawn

`room_section(..., daylight=True, light_label=False)` (new keyword `light_label`; `daylight=True` is still the default, so every existing section gains the drawing automatically):

| Element | Meaning | Bound to |
| --- | --- | --- |
| cream arrows from four sky elevations (15°, 35°, 55°, 75°) | diffuse sky light entering at window mid height, arrow weight ∝ CIE overcast luminance `(1 + 2 sin e)/3` (high sky is brightest), opacity ∝ daylight factor | window size (mid height), overhang depth |
| arrows that stop at the overhang with a small cross | sky light the overhang takes away (blocked when `(3 m − window mid)/tan e ≤ d`) | overhang depth and window height |
| fading stub behind the glass | light that reached the room | same |
| cream gradient strip on the floor, 6 m deep | daylight factor `DF(x) = 2·DF̄·(1 − x/D)`, opacity ∝ `DF/8 %` | `daylight` (mean DF̄) of the live design |
| cream columns over the room height + work-plane line (0.85 m) with end tick | daylit depth, where DF ≥ 2 % | same |
| ceiling lamps: bulb opacity, halo, and a yellow light cone down to the work plane, all ∝ `1 − autonomy` (normalised over the study range 0.20 … 0.83) | lighting electricity (219 kWh/a at the baseline) | `autonomy` |

`light_label=True` adds two labels (needs about 0.3 units above the roof and 0.5 under the floor; off by default so existing layouts keep their extents): `Lamps on 34 % of hours` (`Lampen an 34 % der Zeit`, yellow, above the roof, dodges the demand label) and a dimension line under the floor with `Daylit ≥ 2 % to 3.8 m` (`Tageslicht ≥ 2 % bis 3,8 m`). The daylight drawing needs no extra space: the sky arrows reach only 0.75 m out from the window (shorter than the gap to the chip column at 0.8 m, found by the strict check: longer arrows ran through the WWR chip), the strip and cones are inside the room. With `light_label=True` the `6 m` dimension moves down so that the daylit label never touches it.

Sun colour stays saturated yellow (direct sun), daylight is the cream `DAYLIGHT`, heating red, cooling teal; the "1 m" scale bar under the section was removed (round 2 audit D17: unexplained dimension); the `6 m` depth dimension stays.

Stills: `toolkit-r2/daylight-a-en.png` (WWR 30 % winter | 80 % summer, no overhang), `daylight-b-en.png` (with 1.2 m overhang), `daylight-mid/deep/small-en.png` (large section scale 0.75 with the electricity formula below: baseline, 80 % + 1.2 m, 10 % windows).

## 3. Live substituted formula

```python
self.electricity_formula(trackers, point=(0,0,0), *, size=20, words=True, frame=None, align="center", width=11.6)
    # "Heating + Cooling + Lighting = Total" above "34 + 424 + 140 = 598 kWh/a"; numbers in the bar colours (red / teal / yellow, total white),
    # largest-remainder parts, localised number format; words above their numbers in shared columns.
self.derivation(trackers, part, point=(0,0,0), *, size=20, words=True, frame=None, align="center", width=11.6)
    # part = "heating":  Heating demand ÷ COP ≈ Heating       121 kWh/a ÷ 3.5 ≈ 34 kWh/a
    #        "cooling":  Cooling demand ÷ EER = Cooling       1,271 kWh/a ÷ 3.0 = 424 kWh/a
    #        "lighting": Installed power × Hours × Lamps on ≈ Lighting     0.43 kW × 1,500 h/a × 22 % ≈ 140 kWh/a
    #        "bill":     Electricity × Tariff = Energy cost    598 kWh/a × 0.25 CHF/kWh = 150 CHF/a
    #        "daylight": Glass transmittance × Window area × Sky angle ÷ Room surfaces = Daylight factor    0.60 × 7.2 m² × 75° ÷ 135 m² = 2.4 %
    #                    (the model's BRE average daylight factor; sky angle from the overhang, room surfaces = 180 m² × (1 − 0.5²))
    # every constant (COP, EER, power density, hours, tariff) is read from the study JSON; "=" turns into "≈" automatically when the rounded inputs
    # do not reproduce the rounded result; the bill is computed from the printed kWh/a so the printed arithmetic closes (audit D19).
self.formula(trackers, rows, point=(0,0,0), *, size=20, align="center", columns=True, frame=None, width=11.6, gap=0.12)
    # generic: rows = list of rows, or fn(design, found) -> rows. A row is a list of items: "string" (ink), ("string", colour), self.live(...).
    # With equally long rows, item i of every row shares column i (words above numbers). frame = outline colour or None; align = left | center | right of `point`.
self.live(source, color=INK, digits=0, unit="", *, scale=1.0, parts_of=())
    # source: model key ("heating", "daylight", "autonomy" ...), a constant, or fn(design, found) -> float; scale 100 turns a share into %;
    # parts_of=("heating","cooling","lighting") prints the largest-remainder rounding of that key within the group.
```

All are `LiveGroup` widgets memoised on the four design trackers, so `self.play(self.set_design(trackers, {...}, 2))` moves the numbers with the room and the bar. Example:

```python
self.add(self.electricity_formula(trackers, (0, -1.9, 0), size=22, frame=P_PRIMARY))
self.add(self.formula(trackers, [[("Window", GLASS), "×", ("Overhang", WARN)], [self.live(lambda d, f: d["wwr"], GLASS, 0, "%", scale=100), "×", self.live("overhang", WARN, 1, "m")]], (0, 0, 0)))
```

Place the formula where the energy clips put theirs: under the drawing, above the caption band (y ≈ −1.5 … −2.2); a two-row formula at size 20 is 0.65 units tall and ≈ 6.8 units wide (the word row is wider than the numbers; German 5 % wider).

## 4. Other label defects handled

- **The layout checker was blind to every toolkit label** (root cause of "none" in all layout reports, audits sampling D29, extrema D28, algorithms L13, experiments D30, pareto D12): `LiveText` (the glyph-cached label behind chips, bar labels, section labels, tags) had fill opacity 0, and `art.layout_conflicts` skips texts below 0.05 opacity. `LiveText` now has fill opacity 1 (its drawn child is unchanged), so text-on-text, text-on-line, off-frame and zone checks now see **all** labels. Expect new findings in the scene reports; they are real. Known false positives: a filled sloped polygon (sun wedge, lamp cone) is tested by its bounding box, so a label next to it reads "object under"; the sun wedge opacity was lowered to ≤ 0.24 (below the checker's 0.25 fill threshold) to keep those out of the way.
- `d` chip of `room_section` now steps left with the slab tip (right edge `min(column, x_tip − 0.35 m)`), so the overhang dimension line no longer runs through the chip when `d > 0.8 m` (found by the repaired checker).
- `plan(arcs=True)`: the `winter 19°` / `summer 66°` arc labels hang outward from the arc end (right-aligned on the west side), 14 px; they no longer cross the compass ring.
- `energy_bars` intensity unit is `INTENSITY_UNIT = "kWh/m²a"` (the bundled font draws "(" and ")" as square brackets, so "kWh/(m²·a)" read "kWh/[m²·a]", audits experiments D15, extrema D18, index D15); scenes that print the unit themselves should use `_room.INTENSITY_UNIT`.
- `plot_axes(..., corner=True)`: both axis lines meet in the lower-left corner even when a range runs through zero (`CornerAxes`, `_origin_shift` override; `c2p` unchanged). Default stays `False` (a plot of a signed quantity wants its zero line). Experiments D19: y axis at x = 0 inside the plot next to tick numbers at the left edge, caused by `x_range` starting at −0.3: pass `corner=True`.
- `live_marker(..., avoid=[curve])`: the readout chip picks the position (above, below, beside) that covers the fewest points of the given mobjects or point arrays, so it stays off the plotted line.
- `label_size(scale)`: now 14 … 17 (was 11 … 16), so chips, dimension labels and demand lines are never smaller than 14 px at 1280 wide (audits: algorithms L10, extrema D17). Chip text is `label_size − 1`, i.e. ≥ 13.
- `plot_axes` tick numbers 16 px (were 15).
- `ramp_strip(..., size=15, ticks=())`: unit "kWh/a" is white (was grey 14 px); `ticks=[600, 700, 800]` draws marks with numbers under the strip and moves the unit to the right of the strip (algorithms L9). `heat_map(..., ticks=...)` passes it on.
- `heat_map(..., avoid=())`: contour label chips (now 13 px, boxes measured) stay inside the axes, never overlap each other and keep clear of the star and of every point in `avoid` (pass marker positions `axes.c2p(...)`; the readout chip of a live marker is about 2.2 × 0.3 units above the dot, so add `c2p(...) + UP * .6` too) (sampling D25, algorithms L8).
- Demand label of `room_section` unchanged (see `toolkit-labels.md`); `demand=True` draws the heating arrows through the wall and the radiator; scenes that call `demand=False` show no heat flow (algorithms B1/B2 came from that).

## 5. Not toolkit-level (left to the scene owners)

Scene tables with hard-coded strings (sampling D05), 10 % column vs y axis (D23), ring and marker clusters (D22), probes and slopes on the map (algorithms B3), the layout checker (`layout_conflicts` lives in the shared energy authoring toolkit, not in this clip).

## 6. Round 3 (pareto M1, extrema B1 and M3)

- **Chips out of the ray corridor (pareto M1).** `room_section` collects every sun ray and sky arrow outside the façade as a corridor and places the `d`, `WWR` and `t` chips at the first free spot of a fixed candidate list (preferred spot, then ±0.5 … 2 m up or down, then 0.6 / 1.2 m further left). A spot is scored first by overlaps with already placed chips, then by the number of ray points inside the chip box; the ground hatch band is excluded. The leader line still ends at its dimension, and each chip now has an opaque dark backing above the rays as a fallback when no free spot exists. Effect: chips jump to the side of the ray bundle (winter sun low: WWR chip drops below the bundle; summer sun steep: chips stay left). Extents can grow up to 1.2 m (0.5 units at scale 0.42) further left than in round 2.
- **Legend title.** `ramp_strip(..., title=None)` and `heat_map(..., legend_title=None)` draw a title above the strip, right-aligned to the strip, on an opaque dark backing, so a dashed guide or the y axis never touches it (the `legend=True` slot sits 0.5 units above the axes' top edge, right half only). Scene-drawn legend titles need the same: keep them ≥ 0.4 units off axes and guides or use this parameter.
- **Direct sun follows the hours the label uses (extrema B1).** `room_section` counts `hours = len(model.sun_samples(orientation, season))` (the number "direct sun N h" prints). With 0 h there is no sun disc, no direct rays, no wedge and no sun patch, only the cream sky-light arrows, and the label reads "Diffuse light only" / "Nur diffuses Licht" (above the roof, left, dodging other labels). With hours > 0 the ray and disc brightness scale with the share `hours / max_sun_hours(season)` (`_room.max_sun_hours`: most hours any bearing gets, 9 in summer); the ray count still shows the overhang's shaded share.
- **What "strength" means (extrema M3).** New `self.sun_note(trackers, season, point, *, size=14, color=SUN, align="left", width=3.2)`: two live lines, "direct sun 7 h" and "44 % of the sunniest façade" (`Stärke` replaced by "der sonnigsten Fassade"), where 44 % is the model's `sun_<season>` (seasonal irradiance relative to the best bearing). Scenes that print their own "strength N %" should switch to it or drop the number.

## 7. Round 3b (index verifier: heat losses, diffuse sky, unit glyphs)

- **Transmission losses through the façade.** Winter `room_section(demand=True)` no longer draws arrows through the rear wall. Two red arrows leave through the glazing (at window mid height) and through the opaque façade wall (below the sill, or above the head when the sill is below 0.3 m); stroke weight and length ∝ U·A of each (`_room.transmission(found)` = `{"window": u_window · A_w, "wall": u_wall · A_wall}` in W/K, e.g. WWR 30 %: 6.5 vs 3.2 W/K, WWR 80 %: 17.3 vs 0.9). The rear and side walls (adiabatic in the study) carry no arrows; the radiator stays as the heat source. The arrows are part of the chip corridor, so the d / WWR / t chips avoid them. Summer cooling arrows (heat removed through the rear wall by the cooling unit) are unchanged.
- **`room_section(..., sky="direct" | "diffuse")`.** `sky="diffuse"` draws no sun disc, direct rays, wedge or sun patch at all, only the sky-light arrows, and the note reads "Diffuse sky light" / "Diffuses Himmelslicht". Use it for scenes about the daylight factor.
- **Daylit depth is a rule of thumb, and says so.** The model has only the mean daylight factor `daylight` (no profile function), so the depth comes from `daylit_depth` (factor falling linearly to zero at the back wall, threshold = the model's `reference_factor` 2 %). `light_label=True` now adds the grey line "rule of thumb: linear fall-off" / "Faustregel: linear abfallend" under the "Daylit ≥ 2 % to 3.8 m" line (the `6 m` dimension moves down accordingly).
- **Unit glyphs.** The bundled font draws "(" and ")" as brackets, so every toolkit label (`row`, `text`, `chip`, `parts`, `unit_text`, `ClipScene.quantity`) maps "kWh/(m²·a)" to `INTENSITY_UNIT` = "kWh/m²a" automatically; scenes that pass the long form through toolkit labels need no change (strings drawn with `ClipScene.label` / plain `Text` are not remapped: pass `_room.INTENSITY_UNIT`).
- Verification: showcase boards `losses`, `section-diffuse`; captures of 📉️extrema, ⚖️index, 🏆️pareto (en+de): `### strict` 0 findings in all 6 runs.

## 8. Round 3c

- **`d` chip precision.** The `d` chip of `room_section` and `facade` prints the fewest decimals (1 or 2) that show the overhang exactly to 0.01 m (`_room.decimals`): 0.6 → "d 0.6 m", 0.99 → "d 0.99 m", 1.0 → "d 1.0 m". WWR (whole %) and t (whole cm) already printed whole numbers. The chip width follows the live value.
- **Not fixed here (shared energy toolkit, not edited).** `wrap_body_lines` in `⚡️energy/🏗️authoring/🔤️fonts/🐍️.py` calls `para.split()`, which also splits on U+00A0, so the non-breaking spaces scenes put between a number and its unit are lost and captions can wrap between "65" and "kWh/a" (verify-r2-pareto m6). The fix there is to split on ASCII space and tab only; every clip is affected. Toolkit labels (`row`, `text`, `chip`) are not affected: they never wrap.

## 9. Round 4: derivations read the model's balance terms

`derivation(part)` now reads `evaluate` keys directly and re-derives nothing: `heating_losses`, `heating_gains`, `heating_eta`, `heating_demand`, `cop`, `heating`; `cooling_gains`, `cooling_removable`, `cooling_eta`, `cooling_demand`, `eer`, `cooling`; `lighting_power`, `lighting_hours`, `lamps_on`, `lighting`; `window_area`, `daylight`; `electricity`. New keyword `digits=1` (compatible: appended).

| part | printed lines (words above numbers) |
| --- | --- |
| `heating` | `Heat losses − Utilisation × Gains = Heating demand` → `2,066.8 kWh/a − 0.5723 × 3,400.8 kWh/a ≈ 120.6 kWh/a`; then `Heating demand ÷ COP = Heating` → `120.6 kWh/a ÷ 3.5 = 34.5 kWh/a` |
| `cooling` | `Heat gains − Utilisation × Removable heat = Cooling load`, then `÷ EER = Cooling` |
| `lighting` | `Installed power × Hours × Lamps on = Lighting` → `0.432 kW × 1,500 h/a × 21.6 % = 140.0 kWh/a` |
| `bill` | `598.1 kWh/a × 0.25 CHF/kWh = 149.5 CHF/a` |
| `daylight` | unchanged (transmittance × window area × sky angle ÷ room surfaces): the model returns only the factor, so the terms still come from the study coefficients (documented in the docstring) |

Precision rule (CONCRETE-BIM-CLIP-EXAMPLES/fleet-r2.md): kWh/a with one decimal everywhere (`digits`); the electricity results are the largest-remainder parts of the sum formula, so `derivation(..., digits=1)` and `electricity_formula(..., digits=1)` agree digit by digit (pass `digits=1` to the sum formula next to a derivation; its default stays 0 for scenes that print integers); utilisation, share and COP/EER use the fewest decimals that make the line hold exactly as printed, and "=" becomes "≈" only when no precision does (the example heating balance: 0.5723 × 3,400.8 = 1,946.27, so 2,066.8 − 1,946.27 = 120.53, not the true 120.6).

`formula` aligns equal-length rows in blocks now: consecutive rows of equal length share columns, each block is centred (or left / right aligned) on its own, so one widget can hold several lines of different token counts (heating = two blocks of two rows, 7 and 5 tokens). A heating or cooling derivation is four rows (0.45 units each at size 20); pass `words=False` for the two number rows.
