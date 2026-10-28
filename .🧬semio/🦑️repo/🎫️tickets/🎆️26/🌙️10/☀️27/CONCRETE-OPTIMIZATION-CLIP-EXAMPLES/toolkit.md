# Toolkit: Room Widgets For The Optimization Clips

Code: `C/🎨️visuals/🐍️.py` (`ClipScene`: language, numbers, plot axes, live marker) and `C/🎨️visuals/🏠️room/🐍️.py` (`RoomVisuals`: every drawing widget). Verified with `T/toolkit-showcase.py <en|de> [board ...]`; stills live in `T/🗑️generated/toolkit/<board>-<language>.png` (both languages rendered, layout report `showcase-<language>-layout.md` is clean for all widget boards). Final signatures are in `api.md`; this page adds parameters, examples and screenshots.

## Principles

- One design, one set of `ValueTracker`s (`study_trackers`). Every widget redraws from the tracker values and from `evaluate`, so `self.play(self.set_design(trackers, design, run_time))` moves room, plan, bars, marker and every number together.
- Units are physical. `scale` is screen units per metre for `room_section`, `facade`, `plan`, `thumbnail`. The 16:9 stage between title and caption is y 2.58 … −2.60; widgets accept `point` so scenes lay out plot and room side by side.
- Strings: every visible word goes through `self.tr`; numbers through `number` rules (en `1,250.5`, de `1.250,5`), units glued with a narrow no-break space. Labels are `Text`-compatible glyph groups, so `layout_conflicts` checks them.
- Live widgets swap their children each frame: ≈ 17 ms (facade, plan), 29 ms (bars), 60 ms (section) per redraw. `FadeIn`/`FadeOut`/`self.add` work.
- Colours: heating `P_RED`, cooling `P_BLUE`, lighting `P_YELLOW`, glass `P_CYAN`, insulation and overhang chips `P_ORANGE`, sun `P_YELLOW`, best/optimum `P_GREEN`; electricity ramp dark teal → cream.

## Widgets

### `room_section(trackers, point=(0,0,0), scale=0.5, season="winter", *, labels=True, demand=True, person=True)`

Vertical section through the 6 × 3 m room, drawn like the energy clips: wall cut in `section_material` (cream), insulation layer ∝ t as a green hatched strip, glass as a filled cyan pane with a white frame, the overhang as an orange slab of depth `d` (0.3 m thick, to scale) in front of the cream roof slab, 1 m scale bar and 6 m dimension, a 1.75 m person, three ceiling lamps whose bulbs and halos follow `1 − autonomy`. `point` is the centre of the interior; the glazed wall is on the left.

Sun: the sun-path point (Zurich latitude, solstices) that meets the façade best gives the profile angle: south summer 66°, south winter 19°, east summer ≈ 33° (the model uses the same sun state, so blocking agrees with the geometry). Seven bright, thick rays cross the window height; `round(7 · shading_<season>)` rays (top first) end with a small cross on the overhang slab, the others enter as a pale light wedge and end on the sill, floor or back wall; a bright yellow sun patch fills the floor (and the back wall) where lit rays land; ray opacity follows `sun_<season>`. If the sun is behind the façade (winter, north) the sun sits above the back wall and "Sun behind" replaces the angle. Labels never cover the geometry: the chips `d`, `WWR`, `t` stand in a column left of the façade (`P_ORANGE`, `P_CYAN`, `P_GREEN`) with leader lines to dimension lines on the slab, the window and the insulation; the sun label dodges the chips. Heating (winter): a radiator glyph inside, three red arrows leaving through the back wall sized by `heating_demand`, labelled `Heating` / `Heizen` and the kWh/a value. Cooling (summer): a cooling-unit glyph under the ceiling, three blue arrows removing heat through the back wall sized by `cooling_demand`, labelled `Cooling` / `Kühlen` and the kWh/a value.

```python
trackers = self.study_trackers({"orientation": 180, "wwr": .45, "overhang": .6})
winter = self.room_section(trackers, (-2.6, .3, 0), .42, "winter")
summer = self.room_section(trackers, (3.2, .3, 0), .42, "summer", labels=False)
self.play(FadeIn(winter), FadeIn(summer))
self.play(self.set_design(trackers, {"wwr": .8, "overhang": 1.2}, 2))
```

Screenshots: `T/🗑️generated/toolkit/section-baseline-en.png` / `section-baseline-de.png` (south, 45 %, 0.6 m, winter with radiator + summer with cooler), `section-east-en.png` (east, 60 %, no overhang: deep summer patch), `section-glass-en.png` (80 %, 1.2 m: rays stopped), `section-north-en.png` (north, 30 %, 30 cm insulation: "Sun behind"), `section-big-en.png` (scale 0.66 next to the bars), German `section-glass-de.png`.

### `facade(trackers, point=(0,0,0), scale=0.5, season="summer", *, labels=True)`

8 × 3 m elevation: window ribbon with mullions every 2 m, overhang slab edge on top, dark shadow band from the model's shaded share of the glass (`shading_<season>` × window height plus the wall strip above the window) with a dashed `P_ORANGE` limit, dimension lines 8 m / 3 m, chips `WWR 45 % · 10.8 m²` and `d 0.6 m`.

```python
self.add(self.facade(trackers, (-3.2, .2, 0), .6, "summer"))
```

Screenshot: `facade-en.png` (baseline left, 80 % / 1.2 m right).

### `plan(trackers, point=(0,0,0), scale=0.28, *, ring_radius=6.0, arcs=False, sector=None, site=None, ghost=None, dimensions=False, season_labels=True)`

Fixed compass ring with N / E / S / W (O in German) and 45° ticks; the 8 × 6 m room turns inside it. Glazed façade is a thick `P_CYAN` line (stroke grows with WWR) with an outward arrow; the centre reads `180° S`.

- `arcs=True`: winter and summer sun arcs outside the ring (radius ring + 1.7 / + 2.5 m, dashed) with noon altitudes; two live "design sun" discs sit on the arcs at the sun-path point that meets the façade, joined to the façade by a line whose opacity follows `sun_<season>`; `season_labels=False` hides the arc labels.
- `sector=(lo, hi, colour)`: shaded band on the inside of the ring between two bearings with edge ticks (infeasible bearings, e.g. `(90, 270, P_ORANGE)`).
- `site=(street_bearing, courtyard_bearing)`: "Street" / "Courtyard" ("Straße" / "Innenhof") outside the ring.
- `ghost=bearing`: dashed outline of a second room at a fixed bearing (east reference).
- `dimensions=True`: "8 m" and "6 m".

```python
self.add(self.plan(trackers, (-4.9, 1.1, 0), .15, arcs=True, sector=(90, 270, P_ORANGE), site=(180, 360), ghost=90))
```

Screenshots: `plan-en.png` / `plan-de.png` (south; east with arcs; 315° with sector, site, ghost and dimensions), `orientation-en.png` (plan inside a study layout).

### `energy_bars(trackers, point=(0,0,0), height=3.0, maximum=None, *, width=0.8, labels=True, compact=False, reference=None, intensity=True, size=15)`

`point` is the bottom centre of the bar. Heating, cooling and lighting electricity are stacked in their colours; `maximum` kWh/a corresponds to `height` (pass one value for a whole clip so shrinking is honest; the default is the largest electricity of a coarse reference grid). A thin scale rule on the left shows the full height; the live total sits above the stack, the intensity in kWh/(m²·a) below. `labels` adds segment labels with leader lines to the right (reserve ≈ 2.6 units); `compact=True` keeps the total only. `reference=design` draws an outlined ghost bar of that design 1.6 widths to the left with its total and "reference" (savings bracket is scene-specific).

```python
bars = self.energy_bars(trackers, (4.3, -2.1, 0), 3.6, maximum=1200)
ghost = self.energy_bars(trackers, (0, -2.1, 0), 3.6, 1200, reference={"orientation": 90})
```

Screenshots: `bars-en.png` (full labels, with reference, compact), `section-big-en.png` (next to a section).

### `thumbnail(design, point=(0,0,0), scale=0.12, *, state="filled"|"ghost", ramp=None, value=None, ring=None)`

Static section pictogram on a 4.8 × 4.0 m tile (green insulation, orange overhang, cream wall and slab, cyan glass) (0.58 × 0.48 units at the default scale; use 0.5 for large cards): masonry + insulation wall (insulation drawn ×2 thickness), window opening ∝ WWR under the roof slab, overhang slab of depth `d`, floor. `state="ghost"`: grey dashed outline, grey glyph, no colour (not yet simulated). `state="filled"`: the tile takes `ramp(value)` where `value` defaults to the design's electricity; `ring` outlines it (e.g. `P_GREEN` for the best).

```python
colour = self.ramp(low, high)
ghost = self.thumbnail({"wwr": .8, "overhang": 1.2}, axes.c2p(80, 1.2), .12, state="ghost")
self.play(Transform(ghost, self.thumbnail({"wwr": .8, "overhang": 1.2}, axes.c2p(80, 1.2), .12, ramp=colour)))
```

Screenshot: `thumbnails-en.png` (eleven filled at 0.12, three ghosts, three at 0.5 with green ring, ramp strip).

### `ramp(low=None, high=None)` and `ramp_strip(colour, point, width=3.5, height=0.16, *, unit="kWh/a", size=14)`

`ramp` returns a colour function (`colour.low`, `colour.high`) from dark teal (low, good) via teal to cream (high, bad); the default range is the min/max of the 41 × 41 south-wall surface. `ramp_strip` draws the horizontal legend with end values and the unit. Use one ramp instance per clip.

### `heat_map(axes, surface=None, *, colour=None, opacity=0.6, levels=0, labels=True, star=True, legend=None, legend_width=3.5, x_scale=100.0)`

Electricity of `model.surface(41)` over WWR (x, in % with `x_scale=100`) × overhang (y, m) as coloured cells inside `axes` (cells are clamped to the axes; `ImageMobject` is unusable with the installed Pillow, so cells are rectangles). Returns a dict: `image` (one VGroup per WWR column, so `LaggedStart(*[FadeIn(c) for c in heat["image"]])` reveals column by column), `contours` (marching squares in pure Python; `levels` = count of nice levels or an explicit list; each level gets one label chip such as `650 kWh/a`), `star` (optimum, `P_GREEN`), `legend` (ramp strip: `legend=True` puts it in the fixed slot above the right edge of the axes, 45 % of the axes width at most; or pass a point), `group`, `ramp`. Construction takes ≈ 1 s.

```python
axes, titles = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), x_label=..., y_label=..., point=(-3.2, .1, 0), width=6.2, height=3.4, y_digits=1)
heat = self.heat_map(axes, colour=colour, levels=4, legend=(-3.2, 2.25, 0))
self.add(axes, titles, heat["image"], heat["contours"], heat["star"], heat["legend"])
```

Screenshot: `heatmap-en.png` / `heatmap-de.png` (map with contours, star, ramp strip, live marker, section and bar at the same design).

### `plot_axes(...)`, `live_marker(...)`

`plot_axes(x_range=(lo, hi, step), y_range=..., x_label, y_label, point, width, height, x_digits, y_digits, x_ticks, y_ticks) -> (axes, labels)`: Manim `Axes` plus tick numbers and axis titles in the scene language (replaces the removed decisions helper). Fixed slots: x title below the tick row, y title above the left edge (left-aligned, at most half the axes width), legend slot above the right edge, so the three never collide.

`live_marker(axes, trackers, x_key, value_fn, *, x_label, x_unit, x_digits, y_label, y_unit, y_digits, color, drops, readout, x_scale)`: dot with ring following the trackers, dashed drop lines to both axes, readout chip kept inside the axes. `x_key` is a design key (times `x_scale`) or a function of the design; `value_fn` is a model key such as `"electricity"` or a function of the design.

```python
marker = self.live_marker(axes, trackers, "orientation", "electricity", x_label="θ", x_unit="°", y_unit="kWh/a")
self.play(self.set_design(trackers, {"orientation": 360}, 2))
```

Screenshots: `orientation-en.png` (marker at 180° with plan, section and bar) and `orientation-moved-en.png` (after `set_design` to 360°).

## Notes and limits

- Rays are model-driven (shaded share of the glass from `shading_<season>`); the model now uses the same window placement and sun state, so the stopped rays agree with the geometry. Lit rays are plain 2D tracing against sill, floor and back wall; the drawn ray angle is capped so lit rays never cross the slab.
- Per-redraw cost scales with labels; use `labels=False` for small secondary sections.
- Layout: with labels on, `room_section` extents (screen units relative to `point`) are measured in `toolkit-labels.md` (scale 0.45: x −3.7 … +4.8, y −1.5 … +2.4); the left part holds the sun and the chip column, the right-hand two-line demand label ("Cooling demand 978 kWh/a heat" / "÷ EER 3.0 = 326 kWh/a electricity", right-aligned at x = 6.95 when it would overflow) is the widest item, pass `labels=False` for compact secondary sections. Dimension labels "1 m" / "6 m" are 14 px; `plot_axes(titles=False)` omits axis titles, y tick numbers sit 0.14 left of the axes (`toolkit-labels.md`).
- Deleted: `⚖️decisions/🐍️.py`, `ClipScene.building`, `present_extrema/sampling/experiments`, `_sampling_plane`. Scene agents keep `present(topic)` and implement `present_<topic>` in their own scene class.
