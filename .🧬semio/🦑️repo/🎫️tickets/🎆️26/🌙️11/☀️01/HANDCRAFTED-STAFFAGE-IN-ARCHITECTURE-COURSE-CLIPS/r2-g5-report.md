# r2-g5: heat-carrying casts to thermal

## Manifest (`V/🧍️staffage/🔣️.json`)
- `optimization-room-lamp-on/-off`, `optimization-index-lamp`, `optimization-experiments-occupant`, `optimization-experiments-window`: dark-line -> thermal, environment `office-neutral`, theme dark.
- `physics-fundamentals-blind`, `heating-solar-gains-blind`: dark-line -> thermal, environment `summer-hot-sun` (lamellae warm in the sun). `cooling-summer-strategy-blind` was already thermal (clip `sun-and-shade`).

## Blind geometry (the careful point)
- Thermal SVG shapes carry a `<title>` between `<g id="shape-...">` and `<path>`, so `_SHAPE` in `V/🧍️staffage/🐍️.py:42` found no parts (0 slats). Fixed the regex to allow an optional `<title>...</title>`. Slats, box, bottom-rail, block-sill, block-head are now found in all three blind casts (17 slats).
- Verified the dark-line and thermal casts give identical geometry (slat-0, box, bottom-rail, block-sill extents equal, 20 mm/m, same origin), so no separate geometry cast is needed.
- Removed recolouring that no longer applies: `color=` on the thermal casts (room, index, extrema, experiments, heating `COLOR_BLIND`), and the `blind_group.animate.set_stroke(...)` in physics `Beat5_Strahlung` (it would have overridden thermal strokes).

## Renders (ql, 15 fps, looked at frames)
- physics `Beat5_Strahlung`: blind warm-tinted, headbox orange, ray ends meet the lamellae.
- heating `Beat5_Verschattung`: blind reads, rays stop on lamellae.
- optimization `Beat4_NormalizedIndex` (room lamps, crossfade of on over off reads as warm shade plus glow halo; index glyph lamp), `Beat1_LocalGlobalExtrema` (lamp icon in the causes row), `Beat3_FactorialExperiments`.
- Experiments: the thermal occupant at 0.55 m was too thin next to the cloud; enlarged to 0.85 m (`🧪️experiments/🐍️.py:905`), re-rendered and checked.
- Not rendered: cooling `Beat` using `cooling-summer-strategy-blind` (already thermal, no `staffage_parts`, untouched).

## Tests
- `staffage_only` guard: 5 tests, only the allowlist-staleness test fails, for the foreign entry `⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py::_plant_panel` (not touched by me; someone else's in-flight change). The other 4 pass.
- Visuals + optimization clip tests: Ran 95, 1 failure (the foreign allowlist entry above), 94 pass.

## Open
- The first experiments render died once with exit 127 (transient tool lookup), the identical rerun was fine.
- Scripts: `T/🧰️g5/` (`render.sh`, `sheet.py`, `run.sh`, `parts.py`, `cmp.py`, `chain.sh`). Generated output in `T/🗑️generated/g5/` to delete.

## Correction note (OI-127, r3-reg, 2026-11-03)
- The cast ids and counts in this report describe the state when the work was done. They are not updated here. The current source of truth is the manifest `V/🧍️staffage/🔣️.json` and the guard `V/🧪️tests/🔣️staffage_only.json`.
- This report does not list the removed cast ids by name, so no id is corrected here. The recolouring removals in the Manifest and Blind geometry sections still hold in the current code.
