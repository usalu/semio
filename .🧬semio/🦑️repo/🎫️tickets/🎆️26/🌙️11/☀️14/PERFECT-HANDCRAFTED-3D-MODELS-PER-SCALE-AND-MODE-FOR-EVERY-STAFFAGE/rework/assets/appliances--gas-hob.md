# appliances/gas-hob — rework report

Chisel: `rework-gas-hob-pass1.py` with `rework-gas-hob-pass1-texts.json` (hand-chosen numbers, starts from the pass-0 base kept in `🗑️generated/rework-gas-hob/base/` while the pass runs).

## Pass 1 (2026-11-22)

### Reference table (table-top single-burner gas hob, domino format)

| Feature | Reality | Model before | Model after |
|---|---|---|---|
| Top plate | 290–306 mm wide, 500–527 mm deep, 8–10 mm pressed enamelled steel, 10 mm overhang | 300 x 497 x 10 mm | unchanged |
| Housing (fascia) height | 45 mm built-in depth for a domino; 50–80 mm for a table-top cooker, because the gas tap (about 30 mm body) and a 35–45 mm knob sit on the fascia | **23.5 mm** | 49.5 mm (y 7.5–57 mm) |
| Total height to pan support | 90–120 mm | 88 mm | 114 mm |
| Knob | 35–45 mm diameter, 20–25 mm deep, skirt or bezel, grip ridge | **26 mm diameter on a 23.5 mm fascia: it stuck out below the housing and cut into the plate** | 37 mm body on a 42 mm skirt with a 3 mm neck, 9 x 36 mm ridge, 25 mm deep |
| Feet | rubber, 20 mm diameter, 8–10 mm | 20 mm, 8 mm | unchanged |
| Burner (rapid, 2.5–3 kW) | cup about 85–90 mm, crown with ports, cap 75–100 mm | cup 82/90 mm, 36 ports, cap 96 mm | unchanged |
| Ignition and flame supervision | spark electrode in a 6–7 mm ceramic and a 5–6 mm thermocouple beside the crown, tips in the flame | **missing** | electrode 7 mm x 15.5 mm, thermocouple 6 mm x 14.5 mm in the well at radius 52 mm |
| Gas connection | R 1/2 (20.9 mm thread) at the back, rail to the tap, pipe to the injector | **missing in the body; structure had only tap-to-burner** | R 1/2 nipple with 27 mm collar on the back wall under the overhang; structure: rail inlet–tap, pipe tap–injector |
| Pan support | cast iron, bars 8–12 mm, pan plane 45–50 mm above the plate, four fingers | 270 mm frame of 10 x 12 mm bars, 48 mm, four drooping fingers | unchanged |
| Flame | 24–36 blue cones 15–25 mm, leaning out | ring + 24 + 2 tongues to 22 mm | unchanged |

### Defects found

1. 1:50 body: the housing was a 23.5 mm strip, too low for any gas tap, and the knob was larger than the fascia it sat on (overlapping the plate above and hanging below the housing). The earlier report had only noted "one knob on the side of the housing".
2. 1:50 body: no gas connection, no igniter, no thermocouple.
3. Thermal, all scales: the housing, after the correction the largest visible surface, was a ghost without region ("stays a ghost" in the old decision), while the 2D housing shape was booked on the region `plate`.
4. Thermal 1:200 and 1:500: the four grate bars floated above the plate without legs.
5. Structure 1:100: the support ring was buried inside the housing block.
6. Clip `flicker`: the rigid lean of 8 degrees pushed the tongues of one side into the pan-support fingers and those of the other side into the well.
7. Coarse scales carried the 40 mm block and the 26/30 mm knob.

### Rebuilt

- **Rig**: `burner` and `trivet` joints at the new plate top (y 0.066), `knob` joint at the middle of the fascia (0.105, 0.031), length 18 mm; `hob` centre of mass at y 0.033. `dimensions.heightM` 0.114.
- **Body 1:50** (29 solids): housing 7.5–57 mm, top 56–66 mm, well, beads and vent slots (14 mm louvres, nine on the fascia and eleven per side) moved; knob re-turned (skirt, neck, body, chamfer) with ridge and pointer; `gas-inlet`, `igniter`, `thermocouple` new.
- **Structure 1:50** (26): housing tray to the new height, `member-gas-rail` inlet–tap, `member-gas-pipe` tap–injector under the well, `member-gas-inlet`, housing mass at the new centre.
- **Thermal**: new passive region `housing` (0.211 m2, enamelled steel) with pose entries in all three poses; `th-housing` at every scale (1:50 the tray, coarser the lower part of the block with its top face marked). 1:200/1:500 grate bars now stand on the posts of the scale. Knob region 0.0046 m2.
- **1:100 / 1:200 / 1:500**: block 66 mm, knob stub 40 mm, plate slab, tray ring, masses re-placed; 1:100 tray lifted to 8 mm so the support ring shows.
- **2D** 1:50, 1:100, 1:200: housing, plate, slots, bead, knob (42 mm circle with ridge, pointer and body circle), plan knob and the 1:200 knob steps redrawn; every front-view shape above the housing raised 26 mm (front shapes carry absolute rest heights).
- **Clip** `flicker`: leans -5 / 4 / -3 degrees, stretches 4 / -3 / 3 mm.
- **Texts**: notes, 2D silhouette sizes and the volume decisions (en, de) of all scales follow the new form.

### Validation

`r3-validate.ts appliances/gas-hob --catalog-only --volumes` after the last publish: `VALID appliances/gas-hob (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. Catalog `volumes` equal the source `volumes/appliances/gas-hob.json` (29/16/26, 8/10/16, 7/10/13, 6/10/13 solids for body/thermal/structure at 1:50, 1:100, 1:200, 1:500).

`r3-project2d --check` silhouettes after the redraw: 1:100 elevation 0.961, front 0.949, plan 0.996; 1:200 elevation 0.958, front 0.939, plan 0.968 (parts of a few features). The 1:50 summary line was not captured.

Rendered and read after the change that affected them: 1:50 body six views, right-side close-up, rear three-quarter, poses `off` and `simmer`, `flicker` at 0.5 s (before and after the amplitude change), `light-and-burn` at 2.6 s, 1:50 thermal cut and structure (sheet and cut three-quarter), 1:100/1:200/1:500 body, 1:100 thermal cut and structure, 1:200 thermal and structure cut, 1:500 thermal and structure, 2D 1:50/1:100/1:200. Rendered but not read after the last publish: `flicker` at 1 s, `light-and-burn` at 1 s and 150 s, the repeated full sheets.

### What remains imperfect

- 1:50 body, pan support: legs are plain 10 mm square posts without cast taper or rubber pads, the frame is a plain 10 x 12 mm bar at finger height; not re-modelled in this pass.
- 1:50 body, flame: untouched. `flicker` is still a rigid lean of the whole crown (now 5 degrees); at the lean the low side comes down to plate level over the well. `simmer` and `off` sink the tongues through the burner body.
- 1:200 body: the flame crown is a dome with a hole that hides the burner puck; 1:500 flame is a cone frustum. Not re-modelled.
- Thermal: `housing` 0.211 m2 is computed by hand (walls 0.076 + underside 0.135) and not compared with a measured number; at 1:50 it references the solid tray, not a shell; at 1:200/1:500 the grate area is larger than the real grate by the body posts it now stands on.
- 2D: gas inlet, igniter and thermocouple are not drawn (7 mm, back side); the 2D drawings were looked at only in the default pose; the 1:50 knob detail path uses arc commands. The 1:500 representation stays `unavailable` with its old reason text.
- Structure: at 1:100 and coarser the gas rail is merged; the 1:500 support slab stays inside the block.
- The catalog file was re-serialised with the `dump` formatter of `volumes-insert.py` (590 lines added, 222 removed in `git diff --stat`), which is not byte-identical to the formatter that wrote the root members before.

### Fundamental issues (outside this asset)

- `r3-sheet.ts` has no `--file`: a rig change can only be looked at after the catalog file is written, so "validate the candidate, look, then publish" is not possible for root fixes.
- `volumes-slender` (length at least 20 times the thickness) leaves no way to give a thermal list at 1:200/1:500 the real short legs of a grate: they are neither slender nor a feature of the scale, so the real-area bars either float or stand on oversized body posts.
- Front-view 2D shapes carry absolute rest heights although they are bound to a bone: moving a bone's rest joint does not move them, which `r3-tools.md` states only indirectly.
- `rework-gas-hob-pass1.py` needs its pass-0 base; the base was deleted with the generated folder after publishing, and the script now refuses to run on the published state.
