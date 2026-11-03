# R2 kr: kettle beat re-timing and OI-23

Scripts: `T/🧰️kr/` (`render.sh` physics-series render, `sheet.py` frame contact sheet of the video, `cast_sheet.py` rasterises cast frames by clip time).

## Kettle beat (`🧲️fundamentals` `Beat1_Leistung`, de, 480p15)
Rendered, looked at frames before, at and after the boil (video frames 520 to 700, every 5 or 6) and at cast frames 0/100/150/170/183/186/190/199 s.
- Timing: no change needed. `KETTLE_RATE` 60, click starts the cast. Boil (clip 184 s) = 3.07 s (46 frames) after the click, the plume reaches full strength about frame 590, the clip ends at 200 s (3.33 s) and the last frame holds about 6 s while the narration runs. The narration and labels make no boil claim, so nothing to re-offset.
- Colour: light blue at the start, orange within about 1 s of the click, red from about 4 s; held red afterwards (thermal span +-25 K saturates at 47 degC, so 100 degC is not distinguishable from 60 degC). I tried `thermalSpanK` 78 (reads blue, orange, red up to 100 degC) but `staffage_only.test_thermal_span_every_clip_draws_its_thermal_casts_on_one_span` requires one span per scene, so it is back at 25.
- Steam: there was none at all (the thermal cast had no transport layer, so the evaporation-driven vent was never drawn). Added `"transport": ["pathlines"], "transportAgeS": 0.2, "transportStrokeMm": 0.1` to `physics-fundamentals-kettle` in `V/🧍️staffage/🔣️.json` (line 60). Tested ages 10, 1.5, 0.5 (swamps the figure, plume 4 to 10 kettle heights and the icon shrinks to fit) and 0.2: a few thin lines, a small wisp before the boil, a clear plume that follows the evaporation after 184 s, kettle stays full size.
- Coordinator's id merge: `physics-fundamentals-kettle` and `-vacuum` ids unchanged (checked).

## OI-23 kettle mass
`S/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json` line 51 `dimensions.massKg` 2.22 to 1.22 (dry: base 0.55 + body 0.45 + handle 0.12 + lid 0.1; the 1 kg of water is the phase-change filling). The filled 2.22 kg in the notes and the 2.2 kg support text stay correct. The manifest does not mirror the mass. The thermal solve does not use `massKg`, the boil is unchanged at 184 s (cast re-exported, same colours and plume timing).

## Checks
- Catalog check `--asset electric-kettle`: pass (1:200, 1:100, 1:50 suitable, 1:500 unavailable as before).
- Phase-change vitest (q2 alias config): 1 file, 21 of 21 pass.
- Python (`m9_tests.sh`): `🧪️tests.🐍️staffage_only` 20 of 20 OK; with `🧪️tests.🐍️` and `🧪️tests.🐍️scenes` 38 tests, the single failure was the span guard from my span 78 experiment, fixed by reverting.

## Open
- The held 100 degC is not visible in colour under the shared span 25. Fixing it needs a scene-wide span change for all `physics-fundamentals` casts.
- Generated output deleted.
