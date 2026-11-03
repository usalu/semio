# r2-sc: handcrafted solar thermal collector

## Done
- New asset `S/🗂️catalog/🏗️machinery/solar-thermal-collector/🔣️.json` (category machinery: building-services machine with operating states, same as the plate exchanger and sorption wheel). Generator `T/🧰️sc/solar.mjs` (+ `build.mjs`, `row.mjs`, `check.sh`, `render.sh`, `vitest.sh`, `frames.py`), built on the `🧰️g2` helpers.
  - 2.0 x 1.0 x 0.1 m flat plate, 35 degree mount, 40 kg (stand 6.4, housing 10, glass 14, absorber 8, headers 1.6); drawing 1.409 m tall, 1.696 m wide.
  - Section (elevation): aluminium tray with end walls, hatched mineral wool, glass, absorber sheet with riser strip, inlet and outlet header circles, base rail and two struts. Front and plan views at 1:50, 1:100 and 1:200; 1:500 unavailable with reason. Nine risers at 0.1 m pitch in front and plan.
  - en and de titles, summary, notes, decisions, poses, clips, assumptions.
  - Thermal: six regions, keys `flow` (absorber, -620 W when pumped) and `fluid` (headers, 8/22 W, x3 for stagnation vapour), solar from the environment (effective absorptivity 0.86 behind glass), plume emitter. Poses `night-idle`, `operating`, `stagnation`; clips `stagnation`, `start-up`, `operating` (300 s, 1 fps). Thermal cast in `summer-heatwave-calm`: operating headers 34 to 58 degC, stagnation absorber/headers 106 to 172 degC.
- Manifest row added to `S/🗂️catalog/🔣️.json` before `evaporative-humidifier` (viewpoints elevation, front, plan). The first insert by a Python script silently wrote nowhere visible (emoji path), and a concurrent manifest rewrite also dropped it; it was re-added with Edit (coordinator request: React `catalog-loading` 148 vs 147).
- Cast `cooling-summer-strategy-collector` in `V/🧍️staffage/🔣️.json` (clip `start-up`, thermal, `summer-heatwave-calm`, `thermalSpanK` 70).
- `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` Beat2: primitive glyph replaced by `fitted_staffage(...collector, rate=0.0)`, `collector.rate = COLLECTOR_RATE` (60) when the regeneration beat starts; flow line now starts at the collector's upper right corner (upper header) into the regeneration coil; tag placed under the cast; collector moved up to clear the caption bar.

## Verification
- Catalog check, full: `✓ manifest`, 148 of 148 assets, 0 failures; solar asset suitable under office-laser at 1:200, 1:100, 1:50.
- React vitest (alias config): 18 of 18 files, 117 passed, 1 skipped (includes `catalog-loading`, 148).
- Library vitest: 55 of 56 files; 1 failure `dxf-reopen` (ezdxf test timed out at 180 s on the loaded machine, unrelated).
- `staffage_only`: 22 tests OK. Cooling clip tests: 4 OK.
- Rendered Beat2 once (ql, 15 fps, jobs not an option in this manim; single process), frames via PyAV: the collector appears cold after the sun, then turns orange as it runs, the yellow flow reaches the coil, "70 degC" readout intact.

## Open
- The glass cover and air gap are not solved layers, so the absorber's open-surface losses make stagnation about 106 to 172 degC only in calm hot air.
- Heat carrier flow is not a schema transport (air and vapour only); shown by header temperatures.
- The tag "Solarkollektor" sits close to "Sorptionsrad" and "Solarwärme" labels; legible but tight.

## Label fix and 70 K check (coordinator follow-up)
- `Solarkollektor` no longer crowds `Sorptionsrad`: collector (height 0.75) and sun moved to y -1.8, sun rays shifted, and `Solarwärme → Heizregister` moved from x -1.0 to 0.0 so the two yellow tags do not touch. Re-rendered Beat2 once and read frames at 40 s and 44 s: all labels have clear space, the flow line still starts at the collector and reaches the coil.
- With all DEC casts at `thermalSpanK` 70 (tr2): heater coil reads warm (orange), humidifiers cool (pale blue), wheel still shows its gradient (warm matrix, darker hot regeneration sector). The collector reads lighter than the coil at the start (cold) and warms with the clip.
