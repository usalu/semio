# Verification V2

E = `🎓️teaching/🏛️architecture/⚡️energy`. Scenes rendered one at a time, quality l, 15 fps, script `T/v2_render.sh` (copy of the G2 renderer, output in `T/🗑️generated/v2`, deleted at the end).

## Scenes looked at

1. Cooling `🌤️summer-strategy` Beat2, regeneration beat (28 to 44 s): heater (finned coil) turns from light grey to orange when the sun heat arrives, sits in the exhaust lane between wheel and plate exchanger. Problem: the dashed solar duct ended on the first letter of the label "Plattentauscher". Fix: label moved from x -0.45 to -0.2 (`E/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`). Re-rendered and zoomed: duct ends clear of the label, label stays clear of "Verdunstungskühlung". The heater stays a slim column (true proportions), legible.
2. Physics `🧲️fundamentals` clock scenes: Beat2 (clock with "1 Stunde", clock with `t = n min`, 10 to 90 s) and Beat6 (vacuum clock "approx 0.2 h / 2.7 h Staubsaugen", 31 to 34 s zoom). Round face, ticks and hand turning, labels clear. No change needed.
3. Physics Beat5 blind (outer and inner): the full-height blind read as a thin zigzag. `_blind` now draws only the upper `BLIND_PORTION` = 0.65 of the cast (true proportions, cut at a lamella boundary, lamellae and `_blind_hit` rays filtered to the drawn part), so the scale is 1/0.65 times larger; about eight tilted lamellae, headbox and rails are visible, rays end on the lamella faces, warm recolour and ripples work (outer 64 to 66 s, inner 73 s). A 0.4 portion was tried first and rejected (headbox dominates, only four lamellae). File: `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (`BLIND_PORTION`, `_blind`).
4. Demand `🧪️practice` Beat2 (HydraulischerAbgleich, full render, 88 s): pump at the left, four thermostatic valves at the riser tops (closing to 0.3, 0.45, 0.8, 1.0, pipes meet them), radiators with ripples, thermometers, flow bars and the closing JAZ bar. Legible, no overlaps; valves are small by room scale but read. No change.
5. Full catalog check (`🔨️modules/🗂️catalog/🩺️check/🟦️.ts`, no arguments, via `repo tools exec bun`): `✓ manifest` and 144 of 144 assets, 0 failures. The older G2 output was incomplete (stopped at 51/144), this run is the finished tally.

## Tests

- `🐍️`, `staffage_only` (guard), `icons`, `scenes`, `sections`, `section_consumers`, `transforms`: 51 tests OK.
- `integration`: 9 tests OK (324 s).
- Run via `T/m9_tests.sh` with module paths `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️<module>` and `PYTHONPATH` = repo root.

## Files

Edited: `E/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`, `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`. New in T: `v2_render.sh`, this report.
