# Toolkit Labels: Demand Label, Dimension Labels, Plot Tick Labels

Code: `C/🎨️visuals/🏠️room/🐍️.py` (`room_section`, `systems()`, constants `DEMAND_EDGE`, `DIMENSION_SIZE`, `DEMAND_FULL_SCALE`) and `C/🎨️visuals/🐍️.py` (`plot_axes`, `Y_TICK_GAP`). Stills: `T/🗑️generated/toolkit-labels/label-<scale>-<season>-<en|de>.png` (scales 0.3, 0.45, 0.6), `axes-en.png`, `axes-left-en.png`; script `T/toolkit-labels.py <en|de> [board ...]` (layout report `labels-<lang>-layout.md`, extents `labels-<lang>-extents.md`).

## Demand label (`room_section(demand=True, labels=True)`)

The label states the conversion from useful heat or cold to electricity, both numbers from `evaluate`, the factor from the study JSON (`parameters.systems.cop` 3.5, `eer` 3), colours = the bar segment colours (heating `P_RED`, cooling `P_BLUE`):

| scale | winter (en) | summer (en) |
|---|---|---|
| ≥ 0.34 | `Heating demand 152 kWh/a heat` / `÷ COP 3.5 = 43 kWh/a electricity` | `Cooling demand 978 kWh/a heat` / `÷ EER 3.0 = 326 kWh/a electricity` |
| < 0.34 | `Heating → 43 kWh/a electricity` | `Cooling → 326 kWh/a electricity` |

German: `Heizbedarf … kWh/a Wärme` / `÷ COP 3,5 = … kWh/a Strom`, `Kühlbedarf … Wärme` / `÷ EER 3,0 = … Strom`; short `Heizen → … Strom`. The first (heat) line is the season colour muted toward grey, the second (electricity) line is the full bar colour, so the number beside the bar is always the saturated one. Dependence: design trackers, season, scale, language only (all in the `design_key` memo key; the factor is a constant of the study).

Placement: left-aligned right of the back wall above the roof (anchor 0.5 m right of the wall, 3.35 m high); when the widest line would pass the frame edge `DEMAND_EDGE` = 6.95 it is right-aligned at that edge above the roof slab instead (never off-frame). Two lines sit 0.04 apart (no self-overlap at any scale; the single short line below 0.34 fixes the old scale-0.3 overlap). The lines are added to the placed items so the sun label dodges them.

Measured extents of the whole widget (labels on, screen units relative to `point`, English):

| scale | winter x | winter y | summer x | summer y |
|---|---|---|---|---|
| 0.3 | −2.49 … 3.37 | −1.05 … 0.84 | −2.16 … 3.44 | −1.05 … 1.61 |
| 0.45 | −3.69 … 4.71 | −1.47 … 1.25 | −3.00 … 4.83 | −1.47 … 2.41 |
| 0.6 | −4.86 … 5.70 | −1.89 … 1.66 | −3.77 … 5.84 | −1.89 … 3.20 |

(left: sun and chip column; right: the label, ≈ 3 units wide at scale 0.45 — German is 5 % wider; y up to the sun in summer). Callers that need less width pass `labels=False` or a small scale. These figures replace the old "−8.5 … +7.4 m" note.

## Dimension labels

The `1 m` scale bar and `6 m` depth labels use `DIMENSION_SIZE` = 14 (≈ 14 px at 1280 width; they were `label_size − 2` = 9 px). Each label hangs 0.08 below its dimension line (top edge under the line, not centred on a fixed offset), so it stays clear of the line, the floor slab (the line is 1.3 m below the floor) and the t chip. The bottom of the section extent is now ≈ −(2.8 · scale + 0.25) relative to `point` (measured −1.47 at scale 0.45), so chips placed under the section start below that value.

## `plot_axes` tick labels

- Vertical tick numbers: right edge `Y_TICK_GAP` = 0.14 left of the axes' left edge (tick marks stick out 0.05, clear gap 0.09), fixed, no frame clamp. They need ≈ 0.14 + 0.3 free space left of the axes; axes whose left edge is ≥ −6.6 stay inside the frame (algorithms: −6.15, no conflicts).
- Horizontal tick numbers: unchanged, 0.22 below the axis.
- New keyword `titles=True`: `titles=False` draws tick numbers only. Empty `x_label`/`y_label` (`None`) already skip their own title, so existing callers (`x_label=None, y_label=None`) are unchanged; no caller needs the old outdent shift any more.

## Verification

- Stills at 0.3, 0.45, 0.6, winter and summer, English; German at 0.3 winter, 0.45 winter, 0.6 summer: no layout conflicts (`labels-en-layout.md`, `labels-de-layout.md`).
- `T/capture-labels.py 🧬️algorithms Beat6_EvolutionarySearch en` (40 beats, `T/🗑️generated/toolkit-labels/frames/`): all beats `- none`, i.e. the 148 off-frame tick conflicts are gone.
- `cd C/🧪️tests && python 🐍️.py StudyModel`: 8 tests OK.

## Notes for callers

- At scale ≥ 0.6 in summer the sun label can touch the scene title when the room sits high on the stage (observed at point y = 0.25); lower the room or use a smaller scale.
- Bars with `labels=True` need ≈ 2.6 units right of the bar; the bar at x = 4.9 in the label stills is clipped by my test layout, not by the toolkit.
