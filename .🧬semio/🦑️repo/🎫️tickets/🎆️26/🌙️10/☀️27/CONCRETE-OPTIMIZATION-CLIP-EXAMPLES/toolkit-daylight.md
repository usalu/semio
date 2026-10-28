# Toolkit: Daylight Zone And Lamp Contrast In `room_section`

Code: `C/🎨️visuals/🏠️room/🐍️.py` (new module helpers `autonomy_range`, `daylight_threshold`, `daylit_depth`, constants `WORK`, `DAYLIGHT`; `room_section(..., daylight=True)`).

## Daylight zone

The model has only a mean daylight factor `DF̄` (`daylight` in `evaluate`), not a profile. The widget assumes the usual linear fall-off from the window to zero at the back wall: `DF(x) = 2·DF̄·(1 − x/D)`, `D` = 6 m. The zone reaches where `DF(x)` drops below the threshold `DF₀` = the model's `reference_factor` (2 %, the common "daylit" limit):

`reach = D · (1 − DF₀ / (2·DF̄))`, clamped to 0 … 6 m (`daylit_depth`).

Drawn behind everything (right after the ground, before sun wedge, walls, rays, patches, lamps, person): 16 pale-cream columns over the interior height whose opacity falls from 0.5 to 0 along the reach, plus a work-plane line at 0.85 m with an end tick at the reach. Colour `DAYLIGHT` = sun yellow mixed 55 % toward white (cream), distinct from the saturated sun patch and rays. WWR 10 % (DF̄ < 1 %) draws no zone. Examples (south, t 16 cm): WWR 30 % no overhang DF 2.4 % → 3.5 m; WWR 80 % → 5.1 m; WWR 30 % with 1.2 m overhang DF 1.4 % → 1.7 m; WWR 80 % with 1.2 m → 4.4 m (deeper everything with bigger windows, shorter with overhangs: the overhang's daylight cost is visible).

Keyword `daylight=True` (default); `labels`, `scale`, `demand` untouched. The new drawing reads only `found = result(design)` of the design trackers, so the `design_key(trackers)` memo key of the widget covers it (season and scale are fixed per widget).

## Lamp contrast

`autonomy_range()` = min/max autonomy over the south wall `surface(41)` grid and the 36 Pareto designs: 0.20 … 0.83. Lamp brightness `1 − (autonomy − min) / (max − min)` clamped; bulb opacity `0.08 + 0.92·b`, halo opacity `0.35·b`, halo radius `.4 + .6·b`. WWR 30 % (autonomy 0.63) now shows a glowing halo, WWR 80 % (0.83) a nearly dark lamp.

## Verification

- Stills (`toolkit-showcase.py en daylight-a daylight-b`): `🗑️generated/toolkit/daylight-a-en.png` (WWR 30 % winter | 80 % summer, no overhang) and `daylight-b-en.png` (same with 1.2 m overhang). Layout clean.
- Pareto capture (`capture-daylight.py 🏆️pareto Beat5_ParetoFront en third third_2 walk`): `🗑️generated/toolkit-daylight/frames/Beat5_ParetoFront/` — no layout conflicts; zone and lamps differ clearly between the 30 % and 80 % designs.
- `cd C/🧪️tests && python 🐍️.py StudyModel`: 8 tests OK.
- Note: a first capture run wrote by mistake to the shared `🗑️generated/frames/Beat5_ParetoFront` (`en-third`, `en-third_2`, `en-walk` PNGs and `en-layout.md` overwritten with the three-beat report); regenerate with `capture.py` if the full English report is needed.
