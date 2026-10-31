# Content report CV-FO (furniture and objects, v1 to v2)

Assets: `S/🗂️catalog/🛋️furniture/{park-bench,cafe-chair,office-swivel-chair,height-adjustable-desk}` and `S/🗂️catalog/🪑️objects/{street-lamp,market-parasol,waste-bin,playground-swing}` (`🔣️.json`, version v2, `completion: authored`).
Scratch (idempotent, load saved v1 copies in `v1/`): `T/🧰️scripts/CV-FO/` (`lib.py`, `common.py`, one builder per asset, `validate.py`). Evidence images: `T/🗑️generated/CV-FO/<asset>-<scale>-print.png`.

## Verification status
- Official `check` and `🖨️preview` CLIs were NOT run (no `core-v2-status.md`, no bun in this session). Own checks (`validate.py`, all eight OK): ids unique, bones/regions/hatches resolve, every shape region-bound, no role without a pen, every pose binds every region/emitter/operating key, default-pose bbox of every scale within max(2 %, 15 mm) of `dimensions`, every drawn region present at every scale.
- Own SVG to PNG preview (rsvg-convert) looked at: all four scales of every asset at print size (bench 50/100/200/500, chair 50/100/500, office chair 50/100/200/500, desk all, lamp all, parasol 50/100/200, bin 50/200, swing 50/200). Not looked at: the other six views, clip instants, thermal overlays, final bench/parasol 500 after tweaks. Design targets: office-laser (stroke 0.1, gap 0.15, feature 0.2, hatch 0.4); outline strokes counted into gaps.
- No asset has an `unavailable` scale: all eight read at 1:500 as composed signs (chair 0.9 x 1.8 mm is the smallest). No `symbols`.

## Common
- v1-only fields removed (`geometry`, `lod`, `riseK`, `operatingKey` on regions, emitter `carries/temperatureC/velocityMS`); thermal v2: regions with geometry, sky view factor, layers (conductivity/cp/density after ISO 10456, heat capacity = rho*cp*min(thickness, sqrt(alpha*3600 s))), operating keys, exhaustive pose bindings, sun/shade through `sunlitFraction` tracks. No temperature authored. Contact (clothed body 30 degC, ISO 7730) only for occupied bench/chair/swing poses.
- Pens: 1:500 body 0.18; 1:200 0.25/detail 0.13; 1:100 0.35/0.18; 1:50 0.5/0.25, texture 0.13, with per-shape `penMm` for thin parts. Thin tubes, chains, ribs: single strokes at 1:500-1:100 (pen inflation stated per scale), solids at 1:50; hatching only at 1:50 (oak grain 0.6 mm bench/desk top, concrete stipple 0.8 mm lamp plinth).
- Dimensions corrected to what is drawn: cafe-chair 0.44 x 0.88; office chair 0.6 x 1.08 (was 1.10); desk 1.60 x 1.18 (monitor panel enlarged to 0.365 m so it reaches 1.18); waste-bin 0.53 x 1.03 (handle); parasol 2.6 x 2.46 (finial); swing 2.78 x 2.35 (foot plates). Manifest sync needed (coordinator).

## Per asset
- **park-bench** 50: two cast-style frames, 3 separate back slats + seat slat with chamfers, 4 bolts per slat (refinement: slat detail), grain hatch. 100: one back band with 2 joints, seat band, bolts. 200: frame contours, bands. 500: one block on two 0.16 m stems (gap seat/back merged). Regions seat/back-timber, steel-frame (500 block lists seat+back as merged).
- **cafe-chair** 50: v1 detail with ground penetration (feet 7 mm below ground) and tube tops through the seat fixed, tiny screws dropped; 100/200: seat band + leg strokes + stretcher + back post and panel; 500: seat bar + 3 strokes. Sit clip now has load tracks (`load:occupant`, `load:occupant-back`), sunlit tracks and thermal event; seated poses bind body contact on seat/back.
- **office-swivel-chair** 50: v1 minus hidden far arms/castors, tread, axle; 100: star silhouette + 3 castor discs, column, seat, shell, armrest; 200: base shape, column stroke, seat band, shell, armrest stroke; 500: base bar, column, seat bar, back stroke, castor dots. Far arms/castors coincide in elevation (hidden). Indoor, no sun.
- **height-adjustable-desk** 50: telescoping 3 stages, motor pods with vents, rail, cable tray, top (grain), keypad, monitor (stand, neck, panel, glazed screen), closed laptop; 100/200: nested tubes with light outlines, pods, monitor panel; 500: strokes + monitor block. Operating keys `lift-drive` (2 x 35 W heat while lifting, authored estimate from 60 W input / 40 % efficiency) and `monitor` (off/standby/on, 25 W). Clip raise-and-lower now 30 s (29 mm/s) with `operating:lift-drive` track.
- **street-lamp** 50: v1 detail, plinth stipple; 100/200/500: tapered column, arm stroke, head. LED housing operating (60 W input, 40 W heat at level 1, levels off/dimmed 0.5/rated; Schubert 2006 for the heat share, power authored estimate); poses day-off (sun), dusk-off, dimmed-night, lit-night, lit-in-wind; ignition clip 900 s at 2 fps so the housing heat capacity (8.8 kJ/m2K) shows; plume bound to lamp-housing.
- **market-parasol** 50: all 8 ribs, scallops at every rib tip, panels (refinement); 100: all ribs as strokes, main scallops; 200: 4 ribs; 500: two canopy lenses + mast. Regions canopy-top (sun), canopy-underside (shaded, NOT drawn: hidden in elevation), frame-alu, base-concrete.
- **waste-bin** 50: v1 detail minus 0.1-0.2 mm marks; 100: body + 3 bands + liner line + lid; 200/500 progressively merged (500 body lists bin-hdpe, bin-ribs, liner-film merged). Sun/shade bound per pose.
- **playground-swing** 50: legs, welds, clamps, brace, beam end, chain with 18 alternating links (rings and edge-on bars), seat plank; 100: chain one stroke; 200/500: A of strokes, chain hairline (8-fold inflation at 1:500, stated). Rubber seat is the sun-heated surface; child contact on seat.

## Doubts
- Underside region of the parasol has no shape (will fail a strict "every region drawn" check; remedy: bind to a future plan view).
- 1:500 minimum feature (0.2 mm) is met only for filled geometry plus outline; the waste-bin lid slab (0.04 m) and desk top line rely on the outline stroke.
- Desk motor/monitor heat, lamp electrical power, foam conductivity, cloth mass are `authored-estimate`; no manufacturer data.
- `plan` views not authored (viewpoints remain elevation); a plan would show the bench seat slats.
- Poses added: street-lamp `dimmed-night`, `day-off` (manifest pose list changes).
