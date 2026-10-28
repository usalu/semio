# Draft (reverted from the room module on coordinator request): daylight zone, lamp contrast, demand label, y tick gap

Verified in stills `🗑️generated/toolkit/daylight-a-en.png`, `daylight-b-en.png`, `daylight-b-de.png` before the revert; the owners of tasks 3 and 4 may reuse any of it.

## Module constants and helpers (room module)

```python
WORK, LABEL_EDGE = 0.85, 6.95
DAYLIGHT = _engine.interpolate_color(_engine.ManimColor(SUN), _engine.ManimColor(INK), .7).to_hex()

@lru_cache(maxsize=1)
def autonomy_range() -> tuple[float, float]:
    found = [_evaluated(float(orientation), wwr / 20, overhang / 5 * 1.2, baseline()["insulation"])["autonomy"]
             for orientation in range(90, 451, 30) for wwr in range(2, 17) for overhang in range(6)]
    return min(found), max(found)

@lru_cache(maxsize=1)
def systems() -> dict:
    values = _model.load_study()["parameters"]["systems"]
    return {key: float(values[key]["value"]) for key in ("cop", "eer")}
```

## Daylight zone (in `build()` right after `group.add(ground)`) and lamps

```python
low_autonomy, high_autonomy = autonomy_range()
if daylight:
    reach = DEPTH * min(1.0, found["autonomy"] / high_autonomy)
    group.add(mn.VGroup(*[rect(reach * k / 16, 0, reach * (k + 1) / 16, HEIGHT).set_fill(DAYLIGHT, opacity=.4 * (1 - (k + .5) / 16) ** 1.2).set_stroke(width=0) for k in range(16)]),
              strokes([(P(0, WORK), P(reach, WORK)), (P(reach, WORK - .12), P(reach, WORK + .12))], DAYLIGHT, 3, .9))
...
brightness = 1 - min(1.0, max(0.0, (found["autonomy"] - low_autonomy) / max(high_autonomy - low_autonomy, 1e-9)))
bulb = clone(lamp["bulb"]).set_fill(opacity=0.08 + 0.92 * brightness)
halo = mn.Circle(radius=0.28 * s / 0.6 * (.4 + .6 * brightness), stroke_width=0, fill_color=LIGHT, fill_opacity=.35 * brightness)
```

Depth rationale: `autonomy` / study maximum (270 designs: WWR 0.10-0.80, overhang 0-1.2 m, orientations 90-450 deg) times the 6 m room depth; the model has no per-position daylight factor.

## Demand label (replaces the head/value pair under `if labels:` of `demand`)

```python
word_heat, word_power = self.tr("heat", "Wärme"), self.tr("electricity", "Strom")
unit_key = "heating" if heating else "cooling"
electricity = parts(found[unit_key], "kWh/a", 0, german)
heat_line = ([word_demand, *parts(found[key], "kWh/a", 0, german), word_heat], _engine.interpolate_color(_engine.ManimColor(colour), _engine.ManimColor(GRAY), .45))
power_line = (["÷", "COP" if heating else "EER", Number(number(systems()["cop" if heating else "eer"], 1, german)), "→", *electricity, word_power], colour)
short_line = ([word_demand, "→", *electricity, word_power], colour)
anchor = P(DEPTH + MASONRY + 0.5, 3.35)
lines = (heat_line, power_line) if s >= 0.34 else (short_line,)
drawn = [text(words, size=size, color=tint) for words, tint in lines]
edge, level, top = "left", anchor, anchor[1]
if anchor[0] + max(item.width for item in drawn) > LABEL_EDGE:
    edge, level, top = "right", np.array([LABEL_EDGE, 0.0, 0.0]), P(0, HEIGHT + SLAB + (0.9 if len(drawn) > 1 else 0.45))[1]
for index, item in enumerate(drawn):
    place(item, (level[0], top if index == 0 else drawn[index - 1].get_bottom()[1] - item.height / 2 - 0.04, 0), edge)
group.add(*drawn)
```
Rendered: "Heating 302 kWh/a heat" / "÷ COP 3.5 → 86 kWh/a electricity" (de: "Heizen 302 kWh/a Wärme" / "÷ COP 3,5 → 86 kWh/a Strom"); the glyphs ÷ and → exist in the font. Needs room right of the back wall (~3 units at scale 0.42), else right-align at x = 6.95 above the roof slab.

## plot_axes y tick gap (visuals module)

Old behaviour: vertical tick numbers right-aligned 0.2 left of the axes' left edge (`axes.c2p(x_range[0], value) + LEFT * .2`, then shifted by width / 2); the algorithms scene `_outdent` moved them 0.14 more, so with axes at x = -6.5 they ended at x = -7.09 (limit 7.05). A gap of 0.1 (tick marks stick out 0.05) plus a clamp of at most 0.07 toward the axis when `tick.get_left()[0] < -FRAME_X_LIMIT + .02` gave 0 conflicts on algorithms (44 frames) and sampling (32 frames).
