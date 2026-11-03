# R2 op: optimization clip defects (sampling overlap, pareto doubled values)

Paths relative to `O = 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip`. Scripts: `🧰️op/render.sh`, `frames.py`, `crop.py`. Renders ql, 15 fps, de, speaker, one scene at a time, frames via PyAV and Read.

## 1. `Beat2_GridRandomSampling`: label overlap (real bug)

Cause: `_bars` in `O/🎲️sampling/🐍️.py` (about lines 254 and 266) left-aligned two labels at `base.x - width/2`: the empty-state "Nicht simuliert" and the partial total "229,9 kWh/a". The "alone" bar sits at `ALONE_BASE = (0.8, -2.2)`. The labels run about 1.4 units to the right, past `PANEL_X = 1.95`, so during `run_2` they printed over the first panel rows (heating and cooling equations) while the bar fills.

Fix: both labels are now centred on the bar (`np.array([0.0, ...])`, default edge). The bar column is unchanged. Frames at 26 s, 30 s and 34 s show no collision with the panel. The mid-fade ghost of a LiveGroup copy (empty and filled label at the same time while `bars_alone` fades in, 0.5 s) is unchanged and cosmetic only.

## 2. `Beat5_ParetoFront`: doubled values (data is not duplicated, no label is drawn twice)

Checked at 12, 19, 26, 40, 70, 100 and 200 s, plus a 4x crop: no ghosting, no double rendering, no crossfade artefact. The doubled impression is real redundancy of different widgets printing the same number:

- Intro stage `room_2`: the bar (`energy_bars`, `O/🏆️pareto/🐍️.py:510`) printed "11,1 kWh/m²a" under the bar, and `per_floor` (line 515, "533,1 kWh/a ÷ 48 m² Boden ≈ 11,1 kWh/m²a") printed the same result next to it.
- Main stage: the bar printed the total (533,1 kWh/a) on top of the stack and `_bar_legend` printed "Summe 533,1 kWh/a" below (needed for the Δ column, so it stays).
- Sun: `room_section` prints "Sonnenhöhe 66°" and the scene note (`sun0`, `s.sun`, line 506) added the hard-coded "66° über dem Horizont" beside it.

Fix:
- `energy_bars` in `O/🎨️visuals/🏠️room/🐍️.py:939` got the keyword `total_label=True` (docstring updated), same name and meaning as `_bars` in sampling. Default unchanged, so every other clip is unaffected.
- Pareto: `bars0` uses `intensity=False` (the derivation line carries it); `s.bars` uses `total_label=False` (the legend carries the sum); the sun note is the single line "Sommer-Mittagssonne" (the live altitude stays in the room label).

Re-render check at 12 s, 100 s and 200 s: each value appears once, no overlap with the legend or Δ column.

## Re-render list (course preview, cp3)

Only these two videos changed (de and en both, same code path): 
1. sampling `Beat2_GridRandomSampling`
2. pareto `Beat5_ParetoFront`

The other optimization scenes are not affected: `total_label` defaults to True, and `energy_bars` callers in extrema, algorithms (via `total=`) and index are unchanged.

## Tests

`m9_tests.sh` with `🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧪️tests.🐍️` plus `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only`: Ran 92 tests, OK (962 s).

## Open

Nothing from these two defects. Generated output deleted.
