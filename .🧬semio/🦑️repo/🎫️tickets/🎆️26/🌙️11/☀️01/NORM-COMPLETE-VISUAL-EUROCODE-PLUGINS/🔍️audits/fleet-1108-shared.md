# Fleet 1108 lane shared

## Gates
- `@semio-tech/norm-artifact-contract-rs:build` run 1 (`shared/1.txt`): exit 1, build:graph-plan SIGKILL (host OOM), no rustc errors.
- Run 2 (`shared/2.txt`): reached rustc phase after about 3500 s in the capacity queue; killed by Claude Code memory-pressure reaper, no exit line. Not restarted (instructed not to restart without being asked).
- Contract `:test`, mechanics build/test: not run.

## Changes
- mechanics `slope::slices()`: ponded water above ground adds gamma_w*(gwl-yg)+*b to slice weight; base u = gamma_w*(gwl-yb)+*b.
- mechanics oracle `🔮️oracles/🌍️geotechnics/🐍️.py` mirrors it and emits `slope.janbu.ponded` = 1.79266 (fixture en1997.G.slope-janbu-ponded: 1.79303, 0.02%).
- New Rust test `ponded_water_above_the_toe_loads_the_slice_and_the_base_like_the_independent_solution`.

## Findings
- Audit item #1 (diagram_kit mount) is a false positive: mounted in structural-scene foundation.
- visual-chrome E0425 (3): unverified, locations elided.
- Scope: structural-scene -> shared-scene, app-surface -> shared-surface.
