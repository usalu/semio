# 🧭 Energy clip idioms for optimization scene authors

> **Superseded (2026-10-27, `title-convention.md`).** The persistent title chip, `scene_title`, `play_scene_title` and `beat_subtitle` described below were removed on purpose by ENERGY-CLIP-STYLE (`inspection-body-title-removal.md`, 19:19). Current convention: no persistent video-name chip in the body; each beat shows one instructional heading top-left from `_fonts.section_heading(text)`; the chapter name lives in the presentation window chrome and `TITLE` stays as transcript metadata. Statements below about the title chip and the subtitle chip (table rows "Title chip" and "Beat subtitle chip", the Beat 1 recipe steps 2 and 3, and rule 9 "Title chip top-left, subtitle chip below it") are marked **[superseded]**; the mentions of the title chip as a font-size or colour example in the `P_RED` and `BODY_FONT_SIZE` rows are likewise obsolete; everything else stays valid.

Sources read: `📊️demand/🎬️clip/⚡️demand-power`, `🔥️heating/🎬️clip/☀️solar-gains`, `❄️cooling/🎬️clip/🌤️summer-strategy`, `❄️cooling/🎬️clip/📐️system-sizing`, `🔥️heating/🎬️clip/🧮️calculation` (all `🐍️.py` under `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/`), plus `🏗️authoring/🎨️visuals/🐍️.py` (`V:` below), `🏗️authoring/🔤️fonts/🐍️.py` (`F:`), `🏗️authoring/🎨️theme/🐍️.py` and the design tokens in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`.

Scene-file shorthand: `DP` = demand-power, `SG` = solar-gains, `SU` = summer-strategy, `SS` = system-sizing, `CA` = calculation.

Important caveat: no rendered energy MP4 exists in this workspace. `.🧬semio/🎓️teaching/⚡️cache/videos/` only holds the optimization videos, and no `ffmpeg` is on PATH. The frames in section 5 were therefore produced by replaying the energy scenes with skipped animations and saving the frame at each narration clause (`T/energy-frames-capture.py`, which reuses the shared `hold_for`). They show the end state of each clause, not motion in between.

---

## 1. Layout grammar

### 1.1 Stage zones (values from `V:` and `F:`)

| Zone | Where | Source |
|---|---|---|
| Frame limits | x ±7.05, y ±3.95 (off-frame check) | `V:2325-2326` |
| Title chip **[superseded]** | top-left, `to_edge(UP, UI_SPACING*2)` and `to_edge(LEFT, UI_SPACING*3)`; primary chip, BODY size; written once in Beat 1 with `play_scene_title` (0.3 s), `self.add()` in later beats | `F:357-373` |
| Beat subtitle chip **[superseded]** | under the title, `next_to(title, DOWN, UI_SPACING)`, `align_to(title, LEFT)`; LABEL size, `P_PRIMARY`; fades in with `BEAT_SUBTITLE_FADE` = 0.3 s | `F:380-392` |
| DIN footnote | top-right corner, `LABEL_FONT_SIZE - 3`, `P_PRIMARY`, opacity 0.72, `to_corner(UR, buff=0.30)` | `DP:93-98`, same in SG:128, SU:77, SS:71 |
| Note band | one line under the subtitle at `NOTE_BAND_Y = 2.30` (`NOTE_MAX_WIDTH = 11.0`); scaffolds are kept below `CONTENT_TOP = 1.98` so they clear the note | `V:1334-1336`; used at `DP:772-773` |
| Stage | between `CONTENT_TOP` (≈1.98) and the formula band; wide charts sit at `WIDE_ORIGIN = (-4.4, -0.75)`, size `8.8 × 2.75` | `DP:89-91`, `DP:144-146` |
| Safe box | `SAFE_TOP 2.58`, `SAFE_BOTTOM -2.60`, `SAFE_LEFT/RIGHT ±6.80`; `fit_band(group, top, bottom)` scales/shifts a scaffold into it | `V:168-193` |
| Formula band | math panel at `FORMULA_PANEL_EDGE_BUFF = 1.7` from the bottom (`formula_panel`, box at z=-1); band check `FORMULA_BAND = (-2.80, -1.84)`; `PANEL_MAX_WIDTH = 11.6` (shrinks the row only if too wide) | `V:1571, 1610-1622, 1959, 2355` |
| Caption band | `caption_bar` `to_edge(DOWN, buff=CAPTION_EDGE_BUFF = 0.4)`, box fill 0.88, z=10000; `CAPTION_MAX_WIDTH = 11.5`; more than `CAPTION_MAX_LINES = 2` raises `ValueError`; band check `CAPTION_BAND = (-4.00, -2.62)` | `V:2008-2063, 2355-2356` |

So the drawable area is roughly y ∈ [-1.8, 1.9] for diagrams, x ∈ [-6.8, 6.8], and the formula sits right above the caption.

### 1.2 Per-clip recipe (all five clips)

1. `apply_scene_style(self)` (sets background `P_DEEP_DARK` and the body font).
2. **[superseded]** `title = scene_title(TITLE_DE)`, `play_scene_title(self, title)` in beat 1; later beats `self.add(title)`.
3. **[superseded]** `subtitle = beat_subtitle("…", title)`, `din = _din_ref("…")`, `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`.
4. `caption = caption_bar(subtitle_text(N, "key"))`, `self.play(FadeIn(caption), run_time=0.3)`.
5. Diagram in the stage; formula in `math_panel` at the bottom; readouts at the left (x≈-2…-6) or right column (x≈2…3.4).
6. End: `_clear_stage(self, (title, subtitle, din))` (DP:367-372) then `self.wait(0.4)`.

### 1.3 Helpers: what the clips really use

Counts are literal grep hits in the five clip files (not the toolkit):

| Helper (`V:` line) | Clip uses | Note |
|---|---|---|
| `caption_bar` (2016), `swap_caption` (2066), `hold_for` (2185) | 5 / 5 / 5 files | the clip backbone |
| `fit_band` (178) | 1 (SU:556) | `fit_band(chart, top=2.35, bottom=-2.42)` |
| `check_layout` (2538) | 0 direct | runs inside every `hold_for` (V:2217); prints only when `LAYOUT_CHECK=1` (V:2329, 2540) |
| `layout_conflicts` (2442) | 0 direct | returns issues: caption > 2 lines, text in formula/caption band, off-frame, text overlap, objects under text, lines through text |
| `side_labels` (1432), `card_grid` (1402), `stat_card` (1381), `window_card` (1362), `stacked_bar` (1486), `note_line` (1339), `labeled_axes` (1519), `equation_row` (1574), `symbol_token` (1557), `paced_flow` (808), `color_ramp` (790), `recolor_meter` (722), `beam_pulses` (926), `cloud_glyph` (1222) | 0 | defined in the toolkit, not used by any energy clip. Do not assume they are proven in the reference; if you use one, check it renders |

The clips draw their own small cards and labels instead: `_card` (DP:155), `_back_chip` (SU:119, SS:162), `_step` (SU:185), `_name` and `_row` (SS:171, 177), `_fact` (DP:562), `_person` (SU:94).

### 1.4 Frame limits that bite in practice

- The formula and caption are fixed. A formula chain that runs long is shrunk to `PANEL_MAX_WIDTH`, so clips set `size=FORMULA_FONT_SIZE - 5` (DP:806, 1009) on purpose.
- Two-line captions must stay short. Three lines raise an error, so write each clause for ≤ 2 lines (the German subtitle is what is measured).

---

## 2. Value ↔ meaning linking patterns

Each entry gives the function, its signature, and a usage example copied from the energy code.

### 2.1 One tracker drives everything (ValueTracker + always_redraw)

`_ramp(tracker, to, *, rate_func=smooth)` (`DP:335-338`) moves a tracker with one animation. `linear` keeps the motion proportional to time, which is what a year sweep needs.

```python
year = ValueTracker(0.0)                                   # DP:431
def p_frac():
    return float(np.interp(year.get_value(), xs, ys))      # DP:433-434
curve = always_redraw(lambda: _partial(chart, ys, year.get_value()))   # DP:445
knob = always_redraw(lambda: Dot(chart["pt"](year.get_value(), p_frac()), radius=0.07, color=POWER))   # DP:446
self.play(_ramp(year, 1.0, rate_func=linear), run_time=6.0)   # DP:470
```

Rule of thumb from these lines: the same tracker feeds the curve, the marker, the readout and the reservoir. Nothing is set by hand.

### 2.2 Live readouts: `math_readout(fn, at, *, size, color, edge)` (V:1981)

`always_redraw(lambda: math_label(fn(), at(), ...))`. `fn` returns a LaTeX-subset string; `at` may be a point or a callable.

```python
p_read = math_readout(lambda: rf"P = {de_num(p_frac() * PEAK_KW, 1)}\,\mathrm{{kW}}",
                      np.array([-2.0, 1.30, 0.0]), color=POWER)          # DP:447-448
cap_read = math_readout(lambda: rf"\Phi_{{\mathrm{{Anlage}}}} = {de_num(kw_of(cap.get_value()), 1)}\,\mathrm{{kW}}",
                        lambda: chart["pt"](1.0, cap.get_value()) + UP * 0.2,
                        size=LABEL_FONT_SIZE, color=P_GREEN, edge="right")   # DP:1111-1113
```

Numbers always come from the model: `round(_energy_at(cum, year.get_value()), -2)` (DP:498), `_cumulative` (DP:132). The readout is frozen before a morph with `_freeze(readout)` (CA:195) or `_freeze(marker, marker_lbl)` (SG:1067), or copied with `_snap(mob)` (DP:343) before it flies into the formula.

### 2.3 Energy tank: stored energy = area under the curve (`energy_tank`, V:1271)

`energy_tank(center, *, height, width, color, level)` returns `{frame, cap, level (ValueTracker 0–1), fill (always_redraw), group, center, height, width}`. The fill height is `(height-0.12) * level`.

```python
tank = _wide_tank()                                         # DP:149-152 (energy_tank, height = WIDE_H-0.2)
tank["level"].add_updater(lambda m: m.set_value(_energy_at(cum, year.get_value()) / cum[-1]))   # DP:500
self.add(area, tank["level"])                               # DP:501; area = always_redraw(_partial_area(...))
```

The tank fills in step with the area being drawn (DP:497-501). Other uses: DP:971 (`x` tracker in Beat 3), DP:1207 (Beat 4), CA:1247 (`energy_tank(..., level=0.9)` in the plant house).

### 2.4 Area under the curve: `_partial`, `_partial_area` (DP:220, 232)

`_partial(chart, ys, frac, color, width)` draws the series up to `frac`. `_partial_area(chart, ys, frac, color, opacity)` closes the same polyline to the axis. Both use the `chart` mapper from `_chart` (DP:174), which maps unit-square fractions to screen points. Beat 3 swaps the curve order with `Transform(curve, sorted_curve)` (DP:935) to show the load-duration curve; the area then sums to the annual demand.

### 2.5 Meters (V:671-731)

- `meter(label, *, length, thickness, color, vertical)` → `{track, fill, label, span, group}`.
- `set_meter(gauge, frac)` stretches the fill from its base.
- `bind_meter(gauge, tracker)` sets the fill once and adds an updater.
- `recolor_meter(gauge, color, **kw)` (V:722) only interpolates the colour; the toolkit docstring warns that `fill.animate.set_fill` pins the bar to a snapshot and fights the updater.

```python
load_bar = meter("Kühllast", length=4.5, thickness=0.5, color=P_RED, vertical=False)   # SU:180
load = ValueTracker(1.0)
bind_meter(load_bar, load)                                                            # SU:182-183
self.play(load.animate.set_value(0.55), load_bar["fill"].animate.set_fill(P_YELLOW), ...)   # SU:215-216
```

System sizing uses the same idea per factor, with the physical quantity mapped to 0–1 (SS:177-186):

```python
def _row(name, x0, y, length, color, frac):
    g = meter("·", length=length, thickness=0.26, color=color, vertical=False)
    ...
    g["fill"].add_updater(lambda m: set_meter(g, frac()))
h_row = _row(_name(r"q_{v,\mathrm{R}}", "Hygiene", AIR), x0, y_h, length, AIR,
             lambda: k_h.get_value() * Q_V_R_HYGIENE / full)                          # SS:533
```

Caveat: the summer clip recolours a bound meter with `animate.set_fill` (SU:216, 230, 247). That is the pattern V:722 warns about, and it is brief enough to look fine, but `recolor_meter` is the correct call.

### 2.6 Thermometers (V:1252-1268)

`thermometer_glyph(bottom, *, height, color, level)` → `{group, column (always_redraw), level, top}`. Map the physical value with a clipped linear function, then bind it:

```python
def _lvl(temp: float) -> float:
    return float(np.clip((temp + 15.0) / 40.0, 0.0, 1.0))                              # DP:552-554
th_e = thermometer_glyph(..., level=_lvl(theta_e.get_value()))
th_e["level"].add_updater(lambda m: m.set_value(_lvl(theta_e.get_value())))          # DP:662
```

Dashed guides follow the column tops, and `dim_arrow` (V:766) measures the span between them, so Δθ is seen as a height difference (DP:666-673).

### 2.7 Stacked cascade with readout column (CA:1369-1420)

Five `ValueTracker`s feed one `always_redraw` that stacks Rectangles from `BASE` (CA:1369-1384). Each segment gets a leader line (`leaders`, CA:1412-1413) from a readout row on the right (`readouts`, CA:1410-1411; their formula strings are `formulas`, CA:1399-1408) to the segment midpoint (`mids`, CA:1381-1385), in the segment colour. The `stacked_bar` helper (V:1486) implements the same stack but is not used.

### 2.8 Sun rays, shading, light packets (V:862-930, SG:155-362)

- `sun_rays(sun_c, glass_x, ys, land_y, *, gap)` (V:862) → `{out, in, group, starts, hits, lands}`. Rays share one direction aimed through the glazing. `shine(rays)` (V:881) draws each ray from the sun through the glass.
  Usage: `rays = sun_rays(sun.get_center(), float(up_win["x"]), [...], floor1 + 0.02, gap=0.55)`, then `self.play(shine(rays), run_time=1.8)` (DP:734-742).
- `_parallel_starts(sun_c, hits, gap)` (SG:161) gives the shared direction for an arbitrary set of hits. `_soft_ray(start, end, ...)` (SG:155) fades a ray in on the sun side.
- Shading is computed, not drawn by hand: `_box_exit` (SG:304) finds where a ray leaves a shading box; `_ray_paths` (SG:320) stops each ray at the first shade it hits; `_lit_top` and `_shade_factor` (SG:255, 261) turn the elevation angle into the sunlit fraction F_sh.
- Light packets travel along the ray polylines: `pulse_flashes(paths, color, repeats, width)` (V:913) wrapped as `_pulses(paths, rt, every=1.6, lead=0.3)` (SG:170-177), used as `during=` in `hold_for`.

### 2.9 Flow animations: density, not speed (V:430-612, DP:295-332)

- `flow_guides(paths, color, *, opacity, width, tips)` (V:414): faint tipped streamlines so the route stays readable.
- `flow_animation(streams, *, waves, radius, cycles, streak)` (V:556): particles on the paths; each stream is `(paths, colour)` or `(paths, colour, colour_end)`.
- `_rate_stream(paths, level, rt, ...)` (DP:295-322) is the reference: `level()` in 0–1 decides how many particles per path are alive, and every particle moves at one speed. Its docstring says it: "a thicker stream means more energy per second, never a faster one".
- `ripples(centers, *, r_max, rings, cycles, color, down, facing)` (V:889) is the radiating heat from warm surfaces (DP:325-332, SG and CA use it too). `down=True` emits from a ceiling.

```python
self.play(_ramp(year, 0.06), _rate_stream([rate_path], p_frac, 3.0, per_path=14, speed=0.5), run_time=3.0)   # DP:491
```

### 2.10 Formula slots: copy, fly, ring (V:1571-1631, V:1992-2000)

- `math_panel(parts, *, color, size, edge_buff, buff)` → `(row, box, items)` where each part is `(key, latex, colour)` (V:1992). `key=None` is glue.
- Fly the on-stage readout into its slot: `_fill_panel(scene, panel, sources)` (DP:352-364) moves `_snap` copies with `TransformFromCopy`, then `FadeIn(rest)` and `Create(box)`. `_fly_into(scene, panel, sources)` (SG:188-205) does the same with `move_to` and `FadeTransform`.

```python
panel = math_panel([("q", "Q", ENERGY), (None, "=", P_WHITE), (None, r"\Sigma", P_WHITE), ("p", "P_{i}", POWER), ...])   # DP:527-532
_fill_panel(self, panel, {"q": q_final, "p": p_now, "dt": cell_h, "u1": cell_p, "u2": cell_h, "u3": cell_q})        # DP:533
self.play(Create(highlight_param(panel[2], "p", color=POWER)), FadeIn(tag_load), run_time=1.4)                        # DP:540
```

- `highlight_param(items, key, *, color)` (V:1625) rings one fragment; it is played when the narration names that parameter. Repeat per parameter in narration order (SS:352-360, SG:1199-1202).
- The `chain` pattern: one ValueTracker per factor, a `math_readout` whose source string is rebuilt each frame with `\textcolor{HEX}{value}`, and a bar that scales with the product (SG:1170-1205). Factor colours in the formula, the numbers and the bar are the same colour (SG:1129-1131, 1180-1184).

### 2.11 Thresholds, gaps, reserves (DP:1090-1137)

- Threshold: `cap = ValueTracker(...)`, `cap_line = always_redraw(DashedLine(pt(0, cap), pt(1, cap)))` (DP:1109).
- Deficit: `gap = Polygon(...)` between curve and threshold (DP:1115-1118).
- Reserve: `reserve = Polygon(...)` from the curve up to `big` (DP:1130-1133).
- Time-driven switching: `cycled()` uses `self.time` to switch a compressor on and off (DP:1096-1102).

### 2.12 Decisions and flows of a token (SU:500-615)

A decision chart with boxes and `_link` lines, a travelling `Dot` token parked beside the next node (`_park(node)`, SU:114), and `dim_chip(box, 0.3)` on the branch not taken (SU:582-583). Un-dimming happens before `Indicate` in a separate play (SU:591-594), with a comment explaining that `Indicate` snapshots the start state.

---

## 3. Pacing

### 3.1 Clause cycle

Every clause in every file follows one cycle:

1. `caption = swap_caption(self, caption, subtitle_text(N, "key"))`. This is a fade out then fade in, 0.35 s total (V:2066-2085). It also records `scene._vo_caption_t0`, which `hold_for` uses.
2. One to three `self.play(...)` calls. Typical lengths: FadeIn/Create 1.0–1.6 s, Indicate 1.0–1.4 s, `_ramp` 2–7 s (DP:470 6.0 s, DP:504 7.0 s).
3. `hold_for(self, N, "key", used=..., during=...)`: wait out the rest of the clause.
4. Retire the finished objects with `_hide(self, *mobs, run_time=0.6…0.8)` (DP:164-169: clears updaters first, then FadeOut). Objects that stay live are kept in `keep` and removed at the end with `_clear_stage`.

### 3.2 How `hold_for` sizes a clause (V:2185-2223)

Budget = max of three numbers: measured TTS length (from `vo_timing.json` if present), the language estimate (`narration_seconds`: German words / 2.2 or English words / 2.5), and the German reading floor (`subtitle_read_seconds`: German words / 1.6, at least 1.5 s; V:2096-2101). For German captions the reading floor is the largest, so **a clause holds about (German words / 1.6) seconds in total**, counted from when its caption appeared. Time already spent is subtracted (`charged`), so `used=` is only needed for the first caption of a beat. `during(rt)` returns animations that fill the remaining time (e.g. `_life`, `_rate_stream`, `pulse_flashes`), otherwise `scene.wait`.

Example: DP:458 `hold_for(self, N, "zahlen", used=0.3 + 1.4 + 1.4)`; DP:493 `hold_for(self, N, "rate", during=lambda rt: [_rate_stream([rate_path], p_frac, rt, per_path=14, speed=0.5)])`.

### 3.3 Beat size

- Clauses per beat: DP Beat1 10, Beat2 13, Beat3 11, Beat4 12; CA AnlagenVerluste 8; SU Beat1 4; SS Beat2 5. Keep one beat to about 10 clauses.
- Objects on screen: DP Beat1 shows at most about one chart, one tank, 2–3 readouts and one formula at a time, then `_hide`s. The file comment says it outright: "every explained object fades before the next one enters" (DP:106-107).
- Every clause has at least one motion: a ramp, a stream (`during=`), a ring, or an Indicate. Clauses are not allowed to be static for 10 seconds.

### 3.4 Reveal order

The same order recurs in all five files: name, then symbol, then drawn quantity, then readout, then formula slot, then highlight. Examples: DP:441-442 cards then `≠` then `Indicate` of both cards (DP:456-463); SG:1129-1165 tokens fly into the formula row; SS:206-360 room, supply, temperatures, Δθ, then formula, then highlighted cards in narration order.

### 3.5 Captions and VO

- `subtitle_text(N, key)` gives the German caption; the English text in the `NARRATION` tuple is the TTS source. `set_vo_language("de")` is set in every clip (DP:68, SG:80, SU:65, SS:58, CA:75).
- Captions are one or two lines: the German clause text is the screen text, so write it to fit.

---

## 4. Colour, fonts, numbers

### 4.1 Semantic palette (tokens from `🔣️.json` in the styling folder; `V:37-59`)

Resolved hex values read from `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`:

| Constant (`V:`) | Token | Hex | Used for in the energy clips |
|---|---|---|---|
| `P_RED = P_PRIMARY` | primary | `#ff344f` | heat and load power (`POWER` in DP), heat loss, warning, chrome, DIN footnote, title chip |
| `P_CYAN = P_TEAL = P_BLUE` | secondary | `#34d1bf` | energy/area (`ENERGY = P_CYAN` in DP), cooling air and ventilation (`AIR`), supply air, natural ventilation |
| `P_ORANGE` | tertiary | `#fa9500` | room temperature, person (`_person` default), transmission loss `H_T`, heat content |
| `P_YELLOW` | warning | `#fccf05` | sun and irradiance (`COLOR_G`), time (`TIME`), Δθ (`DELTA_T`), solar load (SS) |
| `P_GREEN` | success | `#7eb77f` | g-value, ρ and c_p factors, comfort goal, target line (`cap_line`) |
| `P_WHITE` | light | `#f7f3e3` | axes, neutral text, formula operators, building section |
| `P_DEEP_DARK` | dark | `#001117` | background, meter track fill |
| `CHROME_FILL` | dark-8-9 | `#07181d` | caption and formula box fills |
| `PASTEL_GREY` / `semio_color("gray")` | gray | `#7b827d` | `COLOR_FSH` (shading factor), inactive labels |
| `semio_color("gray-600")` | gray-600 | `#a2a59d` | frame and end labels (`COLOR_FRAME`) |
| `PASTEL_PURPLE` | info | `#dbbea1` | **misnamed**: it is a beige. CA uses it as "purple" (`PURPLE`, CA:439, CA:1333) |

Notes:

- `P_BLUE` is the same teal as `P_CYAN`. DP sets `H_V = P_BLUE` and `ENERGY = P_CYAN`, so ventilation and the energy tank are the same colour (DP:76, 79).
- In CA, `QH_C = PASTEL_RED` and `CE_C = PASTEL_PINK` resolve to the same primary red (`V:50, 58`), so two series of the cascade are indistinguishable (CA:1333).
- Module colour names (`POWER`, `ENERGY`, `TIME`, `H_T`, `H_V`, `DELTA_T`; `COLOR_G/A/FF/GVAL/FSH/HEAT/MASS/C`) are the per-file meaning map. Keep one meaning per colour inside a clip.
- Manim named colours are used in CA (`GREY_A`, `GREY_B`, `GREY_C`, `PURPLE` default) and SG (`GREY_C`). Prefer semio tokens.

### 4.2 Font sizes (`F:341-346`, computed as token × 1.5 in `🎨️theme/🐍️.py:37-40`)

| Constant | Value (pt) | Token | Use |
|---|---|---|---|
| `TITLE_FONT_SIZE` | 32.4 | `text2xlPx` 21.6 | (not used by the energy clips) |
| `SUBTITLE_FONT_SIZE` | 21.6 | `textBasePx` 14.4 | (not used) |
| `BODY_FONT_SIZE` | 21.6 | `textBasePx` 14.4 | card names, title chip, readouts with units (`P = 6,3 kW`), formula parts in summer and sizing |
| `LABEL_FONT_SIZE` | 19.2 | `textSmPx` 12.8 | axis labels, beat subtitle, DIN footnote (−3 → 16.2), chips, tags |
| `FORMULA_FONT_SIZE` | 32.4 | `text2xlPx` 21.6 | single symbols in the chain (G, A, F_f…), the solar chain `chain_src` (uses `BODY_FONT_SIZE + 4`) |
| `CAPTION_FONT_SIZE` | 24.0 | `textLgPx` 16 | captions (`caption_bar`), never shrunk |

Smaller clip-local sizes are computed from these (`LABEL_FONT_SIZE - 2`, `FORMULA_FONT_SIZE - 5`, `BODY_FONT_SIZE + 4`). **Calculation (CA) breaks the scale**: 56 literal `size=` values (15, 19, 20, 22, 24, 26, 28, 32, 36…). DP has 4 literal values, SG 0, SU 1, SS 0.

### 4.3 Number formatting (`V:1962-1964`)

```python
def de_num(value: float, digits: int = 0) -> str:
    return f"{value:,.{digits}f}".replace(",", r"\,").replace(".", "{,}")
```

So `de_num(24600)` → `24\,600` (thin-space thousands) and `de_num(6.3, 1)` → `6{,}3`. These are LaTeX-subset strings for `math_text`/`math_label`/`math_readout`; never put them in a plain `Text`. Signs: `_signed(value, digits)` (DP:557-559) uses U+2212. Units inside the string: `\,\mathrm{kW}`, `\,\mathrm{kWh/a}`, `\,\mathrm{°C}` (German decimal comma only; the energy clips are German-only on screen).

### 4.4 Typesetting and text

- Text: `body_text(text, font_size, color)` (F:275) is the only sanctioned text constructor. Counted bare `Text(` calls: SG 22, SU 16, SS 9, DP 0, CA 0. Those three files break the rule; use `body_text`.
- Math: `math_text`/`math_label`/`math_row`/`math_panel` (V:1926-2000) support sub/superscripts, `\frac`, `\text{}`, `\textcolor{HEX}{}`, `\mathrm{}`, spaces, and a few symbols. There is no TeX dependency.
- Lines: `centered_body_text(text, font_size, color, line_buff, max_width)` (F:309) makes multi-line captions and chips, centered.

---

## 5. Extracted frames

No MP4 existed, so these are stills from the energy scenes replayed with skipped animations (`T/energy-frames-capture.py`; log `T/🗑️generated/energy-frames/capture.log`). Each frame is the state when the named clause's caption is on screen, at 1280×720. Folder: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/energy-frames/`

| # | File (in the folder above) | What it shows |
|---|---|---|
| 1 | `Beat1_LeistungUndEnergie-jahr.png` | DP Beat 1, clause "jahr": the year power curve is drawn in full on the wide chart with kW ticks and months; the knob sits at December; no readouts yet. |
| 2 | `Beat1_LeistungUndEnergie-summe.png` | DP Beat 1, "summe": the area under the curve is filled, the energy tank on the left is full, readout Q ≈ 24 600 kWh. |
| 3 | `Beat1_LeistungUndEnergie-trennung.png` | DP Beat 1, "trennung": formula panel Q = Σ P_i · Δt with 1 kW · 1 h = 1 kWh; rings on P_i (red) and Q (teal); tags "Last" and "Bedarf". |
| 4 | `Beat1_VerlustZuGewinn-gain.png` | SG Beat 1, "gain": house section with sun rays hitting roof, walls and glazing; loss paths gone; label Φ_solar. |
| 5 | `Beat5_Verschattung-raffstore.png` | SG Beat 5, "raffstore": slats down, direct rays blocked; F_sh scale marker at 0,00 with end labels "ganz verschattet" / "ohne Verschattung". |
| 6 | `Beat8_Hauptgleichung-chain.png` | SG Beat 8, "chain": Φ_solar = G·A·F_f·g·F_sh with colour-matched factors; numeric chain 400·1,92·0,70·0,60·0,80 ≈ 258 W; bar 258 W of 768 W (G·A). |
| 7 | `Beat1_LastZuerstSenken-envelope.png` | SU Beat 1, "envelope": house with envelope outline and louvres; Kühllast meter at about 0.55, yellow; step 1 visible. |
| 8 | `Beat1_LastZuerstSenken-natural.png` | SU Beat 1, "natural": meter about 0.4, teal; cyan streams through the windows; step 2 active; the meter carries no number or unit. |
| 9 | `Beat1_LastZuerstSenken-reserve.png` | SU Beat 1, "reserve": meter about 0.25, orange; step 3 "Hauptweg"; red crosses on both windows, loss streams faded. |
| 10 | `Beat2_VolumeFlowEquation-formula.png` | SS Beat 2, "formula": room section with ventilation grille, θ_Zu 18 °C → θ_Raum 25 °C, card "Temperaturhub 25 − 18 = 7 K", formula Q̇_V = ρ_a · c_p,a · Δθ · q_v,R [W] in the panel. |
| 11 | `Beat3_IsolateAirflow-compare.png` | SS Beat 3, "compare": two horizontal meters q_v,R Hygiene 0,061 m³/s and Solarlast 0,126 m³/s; ratio chip ≈ 2,1; formula fraction giving 0,126 m³/s. |
| 12 | `Beat3_IsolateAirflow-duct.png` | SS Beat 3, "duct": two discs whose areas are the duct cross-sections A = q_v,R / v_m (0,0244 m² and 0,0505 m²), drawn at true relative area; formula A ≈ 0,0505 m². |
| 13 | `AnlagenVerluste-chain.png` | CA, clause "chain" (start): heated house with cellar, store, generator and pipes; stacked column holds only Q_h ≈ 11 500 kWh/a at this point. |
| 14 | `AnlagenVerluste-d.png` | CA, clause "d": the stack gains the distribution segment (orange, Q_h,d = 900 kWh/a) and leader lines to readouts in segment colours; "30 m Rohr" label collides with "Wärmeerzeuger". |
| 15 | `AnlagenVerluste-sum.png` | CA, clause "sum": all five losses stacked (beige = generation, yellow = storage, orange = distribution, red = transfer and Q_h), final formula Q_E ≈ 14 000 kWh/a with the factors coloured to match. |

Layout findings from the capture's `layout-conflicts.md` (`T/🗑️generated/energy-frames/layout-conflicts.md`, produced by `layout_conflicts` on each frame) and confirmed by eye:

- DP Beat 1 (`jahr`, `summe`, `trennung`): the "P" axis name overlaps the "10" tick label by 0.21 units.
- SG Beat 5 (`raffstore`): "ohne Verschattung" spans x to 7.08, past `FRAME_X_LIMIT` 7.05.
- SS Beat 2 (`formula`): the live readout "25 − 18 = … K" under the Temperaturhub card title sits about 0.01 units into the title, because the static Δθ line kept at opacity 0 still takes up layout space. Remove hidden placeholders from the scene instead of fading them to 0.
- CA (`chain`, `d`, `sum`): "beheizt, 20 °C" runs into the occupant's silhouette; "30 m Rohr" overlaps "Wärmeerzeuger" (`d`, `sum`).
- Not flagged by the checker, but visible: SU `natural` and `reserve` show the Kühllast meter with no number; CA `sum` colours G (generation) beige, not a semio hue.

---

## 6. Ten rules for optimization scene authors

1. **One tracker, many views.** Drive the plotted curve, the marker, the live readout and the physical reservoir from the same `ValueTracker` (DP:431, 445-447, 500). Never update a number or a shape by hand.
2. **Show the quantity as area or height, not only as a number.** Energy is `_partial_area` under the curve and a `energy_tank` filled from the cumulative sum (DP:232, 149-152, 500). A value with no drawn height is not allowed.
3. **Encode rate by density.** Use `_rate_stream`-style particle density for power and keep particle speed constant (DP:295-300). Do not speed particles up to mean more power.
4. **Every number is generated and has a unit.** Use `math_readout` with a model function and `de_num` (German decimal comma, thin-space thousands). Do not type `"5 kW"` in a `Text`. Keep the unit (`\mathrm{kW}`, `\mathrm{kWh/a}`, `\%`, `\mathrm{°C}`) on every readout.
5. **Freeze, copy, fly, then ring.** Before a readout morphs into the formula, `_freeze` or `_snap` it, then `_fill_panel` or `_fly_into` (DP:352-364, SG:188-205). Ring each parameter with `highlight_param` in the order the narration names it.
6. **Colour-match the formula to the drawing.** The same colour token marks the object, its readout, its formula fragment and its bar (SG:1129-1184, DP:527-541). Give each meaning one colour in the clip. Do not reuse a colour for two series (CA:1333 is the counter-example).
7. **One clause, one motion, one hold.** Each clause is `swap_caption` + at least one animation + `hold_for(during=…)` (V:2066, 2185). Keep captions to two lines (`CAPTION_MAX_LINES`). Clear finished objects with `_hide` before the next idea (DP:106-107). Keep ≤ 10 clauses per beat and ≤ 3–4 live objects in view.
8. **Fonts only from the scale.** `body_text`, `math_label`, `math_readout`; sizes only `BODY_FONT_SIZE`, `LABEL_FONT_SIZE`, `FORMULA_FONT_SIZE`, `CAPTION_FONT_SIZE`, optionally `± small offset`. Do not use bare `Text(` or literal `size=28` (SG, SU, SS and CA break this today).
9. **Use the sanctioned zones.** **[superseded: top-left is one `section_heading` per beat, no title chip]** Title chip top-left, subtitle chip below it, DIN footnote top-right, note band at `NOTE_BAND_Y`, formula via `math_panel` at the bottom, caption via `caption_bar`. Keep diagrams between `CONTENT_TOP` (1.98) and the formula band (−1.84). Check `x` against `FRAME_X_LIMIT = 7.05`: in the solar Beat 5 frame `raffstore` the label "ohne Verschattung" runs a few pixels past that limit at the right edge, so run `LAYOUT_CHECK=1` on every new scene.
10. **Do not animate a bound meter's colour with `animate.set_fill`.** Use `recolor_meter` (V:722), which interpolates only the colour, instead of `bind_meter`-driven fills being pinned by a snapshot (SU:216, 230, 247). Also do not draw a meter without a numeric readout: the summer "Kühllast" bar in the `natural` frame shows a fill of about 0.4 with no number and no unit, which the quality bar (every value shown carries its meaning and units) does not allow.

---

Files written by this exploration: this report and `T/energy-frames-capture.py` (replay script), and frames plus the capture log and `layout-conflicts.md` in `T/🗑️generated/energy-frames/`. No source file was modified and no git command was run.
