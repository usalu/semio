# Shared Interfaces

Both interfaces are contracts between the model agent, the toolkit agent and six scene agents. Change them only by editing this file and telling the coordinator.

## Model `🧮️model/🐍️.py` (stdlib only)

```python
load_study() -> dict                          # the authored 🔣️.json (fresh copy); "sun": {latitude, winter_altitude, summer_altitude,
                                              #  low_summer_altitude, winter_azimuth, summer_azimuth}, "weights": {balanced, cost_heavy}
Design = dict(orientation=180.0, wwr=0.45, overhang=0.6, insulation=0.16)   # degrees, ratio, m, m (unknown keys / impossible values -> ValueError)
evaluate(design: dict) -> dict                # missing keys fall back to study["baseline"]
# keys: window_area m², wall_area m², u_wall W/m²K,
#       heating_demand, cooling_demand          kWh/a useful heat/cold
#       heating, cooling, lighting, electricity kWh/a electricity (heating/COP, cooling/EER, lighting)
#       intensity kWh/(m²·a) = electricity / floor area
#       solar_winter, solar_summer kWh/a admitted gains
#       shading_winter, shading_summer 0..1 = beam-weighted share of the window height in the overhang's shadow,
#         per hourly solstice sun position clamp((d·tan(profile) − 0.3 m gap above the window) / window height, 0, 1),
#         window height = min(WWR·3 m, 2.65 m), head at 2.7 m (same as the toolkit room drawing); 0 when no direct sun reaches the façade
#       sun_winter, sun_summer 0..1 irradiance on this bearing relative to the best bearing of the season
#       daylight % mean daylight factor; autonomy 0..1
#       cost CHF; carbon kg CO₂e
sun_samples(orientation, season) -> list[dict]   # hourly solstice sun positions (noon ± 4 h, altitude ≥ 3°) shining on the façade:
                               #  [{"hour","bearing","altitude","across" (angle to the façade normal),"profile" (angle in section),"weight" (cos alt · cos across)}]
sun_state(orientation, season) -> dict           # the single sun-path point nearest to the façade bearing {"bearing","altitude","across","profile","front"}
                                                 #  (identical to the toolkit's sun_state)
orientation_study() -> dict    # {"design" (wwr, overhang, insulation; orientation is the variable), "range":[90,450],
                               #  "local_minima":[180,360], "local_minima_electricity", "local_maxima":[..], "local_maxima_electricity",
                               #  "global_minimum":θ, "constraint":[270,450], "feasible_minimum":θ, "reference":90, "reference_electricity",
                               #  "tariff": CHF/kWh, "samples":[[θ, electricity], ... every 1°], "savings":[[θ, kWh/a, CHF/a], ...],
                               #  "start":315, "neighbour_step":5, "descent":[{"theta","electricity","neighbours":[{theta,electricity}]}],
                               #  "sun": {winter_altitude, summer_altitude, low_summer_altitude, winter_azimuth, summer_azimuth, ...}}
surface(n=41) -> dict          # {"design", "wwr":[...], "overhang":[...], "electricity":[[...]] indexed [wwr][overhang], "minimum":{design,electricity}}
levels() -> dict               # {"wwr":[3], "overhang":[3], "orientation":[90,180,270,360], "insulation":[3]}
lhs_strata() -> dict           # {"count":9, "wwr":[10 edges], "overhang":[10 edges], "index":[[i, j], ...]}
samples(kind) -> list[dict]    # kind in grid|random|lhs (9 each); each {"wwr","overhang", **evaluate}; lhs items also {"strata_index":[i, j]}
sampling_diagnostics(kind="random") -> dict   # {"largest_empty_box":{"wwr":[lo,hi],"overhang":[lo,hi]}, "area" (share of the domain),
                                              #  "closest_pair":[i, j, distance on range-normalized axes], "closest_delta":{"wwr","overhang"}}
factorial() -> dict            # {"factors", "response":"cooling", "runs":[{"id":"r1".."r4","coded":[a,b],"design","response", **evaluate}] in the order
                               #  (A-,B-),(A-,B+),(A+,B-),(A+,B+), "effects":{"a","b","ab"} (mean high - mean low), "coefficients":[b0,b1,b2,b12] (= effects/2),
                               #  "centre":{"design","predicted"(=b0),"actual","curvature","result"}, "centre_savings":{"half","full","share"}}
fractional() -> dict           # {"generator":"C=AB", "factors":[wwr, overhang, insulation 10/30 cm], "runs":[{"coded":[a,b,c],"design","response"}] (4),
                               #  "alias_contrast", "true":{"ab","c","sum"} from the full 2³ (also "full":[8 runs])}
variants(extra=False) -> list[dict]  # [{"id","title":{"en","de"},"design", **evaluate}]; extra=True appends the dearer sixth ("extra": True)
criteria() -> list[dict]       # [{"id","key","direction","unit","title":{"en","de"}}] in the order electricity, cost, daylight, carbon
reweigh(weights, criterion, share) -> dict   # that criterion gets `share`, the others keep their old proportions
index(weights, ids=None) -> list[dict]       # [{"id","raw":{..},"normalized":{..} (0 best … 1 worst),"score" (lower is better),"rank" (1 = lowest score)}]
                                             #  weights by criterion id (study["weights"]); min–max range from the compared ids only
pareto() -> dict               # {"designs":[{"id","design","cost","electricity","heating","cooling","lighting","daylight"}], "front":[ids sorted by cost],
                               #  "dominated_example":{"winner","loser"}, "step":{"from","to","cost","electricity"},
                               #  "steps":[{"from","to","cost","electricity"(<0),"saved","ratio" CHF per kWh/a,"payback","payback_hi" years}],
                               #  "tariff","tariff_hi","service_life", "knee":{"tariff":id,"tariff_hi":id},
                               #  "refinement":{"member","design","cost","electricity","dominates":[ids]},
                               #  "third_example":{"dominated_2d","dominator"}, "front3":[ids], "picks":[{"weight","id"}]}
weighted_pick(weight) -> id    # share of electricity 0..1 in a weighted sum of min–max normalized cost and electricity over the front
gradient_path() -> list[dict]  # [{"design","electricity","gradient":{"wwr" per 1.0, "overhang" per m},"slope":{"wwr" per %-point,"overhang" per m},
                               #  "step":{"wwr","overhang"} moved to get here,"probes":[{"direction":"wwr-|wwr+|overhang-|overhang+","design","electricity"}],
                               #  "runs":5,"evaluations":cumulative}]
pattern_path() -> list[dict]   # [{"design","electricity","step":{wwr,overhang} used in this round,"probes":[{"direction","design","electricity"}] (4),
                               #  "moved":bool,"evaluations":cumulative}]
genetic() -> dict              # {"generations":[{"population":[{"id","design","electricity"}], "parents":[[id,id],...],
                               #  "tournaments":[[id,id,winner],...], "elites":[ids], "survivors":[ids], "evaluations":cumulative,
                               #  "children":[{"id","from":[id,id],"genes":{"wwr":id,"overhang":id},"mutation":{"wwr":Δ,"overhang":Δ},"design","electricity"}],
                               #  "best":{"id","design","electricity"}}], "lcg": {seed, multiplier, increment, modulus}}
convergence() -> dict          # {"budget":50, "map_minimum", "runs_map":1681, "series":{"gradient"|"pattern"|"genetic": [[runs, best_so_far], ...]}}
```

## Toolkit `🎨️visuals/🐍️.py` (final, implemented; drawing code lives in `🎨️visuals/🏠️room/🐍️.py`)

`ClipScene` keeps `language`, `tr`, `label`, `step`, `explain`, `clear_stage`, `present` (dispatches to `present_<topic>`), `axes`, `_curve` and gains the methods below. `⚖️decisions/` is deleted; the old topic bodies and `building` are gone. All widgets are methods of `ClipScene`, use `self.tr` for every visible word and `number` rules for every number, and read their numbers from `evaluate` with the live tracker values. Live widgets are `Text`-compatible mobjects whose children are swapped each frame (cheap: ≈ 20–60 ms per redraw), so `FadeIn`, `FadeOut`, `self.add` work normally. Units: `scale` = screen units per metre.

```python
number(value, digits=0) -> str                       # en 1,250.5 · de 1.250,5 (minus is "−")
quantity(value, unit, digits=0) -> str               # "1,250 kWh/a" with narrow no-break space (no space before "°")
study_trackers(design=None) -> dict                  # ValueTrackers orientation, wwr, overhang, insulation (baseline for missing keys)
set_design(trackers, design, run_time=1.0)           # AnimationGroup: all named trackers move together (smooth)

room_section(trackers, point=(0,0,0), scale=0.5, season="winter"|"summer", *, labels=True, demand=True, person=True)
    # All live widgets are memoized on the design trackers (LiveGroup key); see toolkit-performance.md.
    # point = centre of the 6 × 3 m interior; glazed wall on the left. Wall (insulation layer ∝ t), window ∝ WWR, overhang slab of d,
    # 1 m scale bar, 1.75 m person, ceiling lamps dimmed by (1 − autonomy), season sun at its study altitude (summer 66°, winter 19° on south;
    # real sun-path point that meets the façade otherwise, "Sun behind" when it is behind). 7 sun rays across the window: round(7 · shading_<season>)
    # of them (top first) stop on the overhang with a dot, the others enter and paint a floor/back-wall sun patch; ray opacity ∝ sun_<season>;
    # winter: radiator + red heat arrows leaving through the back wall (∝ heating_demand), label "Heating/Heizen N kWh/a"; summer: cooling unit + blue arrows
    # (∝ cooling_demand), label "Cooling/Kühlen N kWh/a"; chips d / WWR / t in a left column with leader lines to dimension lines; blocked rays end with a small cross.
    # Extent (labels on) relative to `point`: x −8.5 … +7.4 m, y −3.0 … +3.9 m (at scale 0.5: x −4.2 … +3.7, y −1.5 … +2.0 units).
facade(trackers, point=(0,0,0), scale=0.5, season="summer", *, labels=True)
    # 8 × 3 m elevation: window ribbon with mullions, height = window_area / 8 m, overhang edge, dark shadow band (model shading share of the glass
    # plus the wall strip above the window), dimensions 8 m / 3 m, chips "WWR 45 % · 10.8 m²" and "d 0.6 m" above the slab.
plan(trackers, point=(0,0,0), scale=0.28, *, ring_radius=6.0, arcs=False, sector=None, site=None, ghost=None, dimensions=False, season_labels=True)
    # fixed compass ring (N E S W, O in German, ticks every 45°) with the 8 × 6 m room turning inside; glazed façade highlighted (thickness ∝ WWR) with
    # outward arrow; centre label "180° S". arcs=True: winter and summer sun arcs (radius ring+1.7 / +2.5 m) with noon altitude labels and the two
    # live "design sun" discs and rays that hit the façade; sector=(lo, hi, colour): shaded band on the inside of the ring between two bearings with
    # edge ticks (e.g. (90, 270, P_ORANGE) for the infeasible half); site=(street_bearing, courtyard_bearing): "Street"/"Courtyard" labels outside
    # the ring; ghost=bearing: dashed room outline at a fixed bearing; dimensions=True: "8 m" / "6 m". Radius incl. arcs ≈ (ring_radius + 3.4) · scale.
energy_bars(trackers, point=(0,0,0), height=3.0, maximum=None, *, width=0.8, labels=True, compact=False, reference=None, intensity=True, size=15)
    # point = bottom centre. Heating (P_RED) / cooling (P_BLUE) / lighting (P_YELLOW) stacked, constant scale = `maximum` kWh/a ↔ `height`
    # (pass the same maximum in a whole clip; default = largest electricity of a coarse reference grid), scale rule on the left, total above the stack,
    # intensity kWh/(m²·a) below. labels: segment labels with leader lines to the right (needs ≈ 2.6 units right of the bar); compact: total only.
    # reference=design: outlined ghost bar of that design 1.6 widths to the left with its own total and the word "reference".
thumbnail(design, point=(0,0,0), scale=0.12, *, state="filled"|"ghost", ramp=None, value=None, ring=None)
    # static section pictogram on a tile 4.8 m wide × 4.0 m tall (0.58 × 0.48 units at the default scale): glass opening ∝ WWR,
    # overhang slab ∝ d, insulation layer ∝ t (×2, pictogram). ghost = grey dashed outline, not yet simulated; filled = tile takes ramp(value or the
    # design's electricity); ring = outline colour (e.g. P_GREEN for the best).
ramp(low=None, high=None) -> colour(value) -> "#hex"   # dark teal (low, good) → teal → cream (high, bad); default range = south-wall surface min/max
ramp_strip(colour, point=(0,0,0), width=3.5, height=0.16, *, unit="kWh/a", size=14)   # horizontal legend with end values and the unit
heat_map(axes, surface=None, *, colour=None, opacity=0.6, levels=0|int|list, labels=True, star=True, legend=None|True|point, legend_width=3.5, x_scale=100.0, columns=True) -> dict
    # surface = model.surface(n) (default 41); axes x in % (x_scale=100) and y in m. Raster (bilinear, 256 shades, whole-pixel registered to the axes
    # corners). Returns {"image": Group of one HeatStrip (ImageMobject) per WWR column (reveal column by column; columns=False: one strip), "contours": VGroup
    # (levels as int = nice count, or list of kWh/a; labels as chips), "star": optimum, "legend": ramp strip if legend=point, "group": Group of all,
    # "ramp": colour function}. FadeIn/FadeOut/set_opacity/set_fill(opacity=)/get_fill_opacity work on the strips; z_index orders image vs. vector parts.
    # Needs the scene camera of RoomVisuals (RasterCamera, installed by ClipScene.__init__): never pass your own camera_class.
plot_axes(*, x_range, y_range, x_label, y_label, point, width, height, x_digits=0, y_digits=0, x_ticks=None, y_ticks=None) -> (axes, labels)
    # replaces the old decisions plot helper: manim Axes with numeric tick labels in the scene language and axis titles, no TeX.
    # Fixed slots: x title below the ticks, y title above the left edge, heat_map(legend=True) strip above the right edge.
live_marker(axes, trackers, x_key, value_fn, *, x_label="", x_unit="", x_digits=0, y_label="", y_unit="", y_digits=0, color=P_WHITE, drops=True, readout=True, x_scale=1.0)
    # x_key: design key (× x_scale) or fn(design); value_fn: model key ("electricity") or fn(design); dot + ring, dashed drop lines, readout chip
    # "θ 180° · 681 kWh/a" kept inside the axes.
```

Notes for scene agents:
- Blocking follows the model: the share of rays that stop at the overhang equals `shading_<season>` (the model's shaded share of the glass), not `1 − shading`. Lit rays are traced against floor, sill, back wall; a ray that hits the sill is drawn ending there.
- `thumbnail` and `heat_map` use one shared `ramp`; give every scene of a clip the same `ramp(lo, hi)` instance so every colour means the same kWh/a.
- Not built (scene-specific, per coordinator): probes, tapes, pips, cards, sliders, solver diagrams, brackets, price chips, bearing axes, `season="both"`, `progress` tracker.
- Showcase: `T/toolkit-showcase.py <en|de> [board …]` → `T/🗑️generated/toolkit/<board>-<lang>.png` (boards: section-baseline, section-east, section-glass, section-north, section-big, facade, plan, bars, thumbnails, heatmap, orientation, orientation-moved).
