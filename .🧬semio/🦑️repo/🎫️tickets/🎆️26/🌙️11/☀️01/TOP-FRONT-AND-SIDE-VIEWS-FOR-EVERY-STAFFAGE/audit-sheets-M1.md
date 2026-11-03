# Audit sheets M1 (machinery visual audit)

## Verdict

FAIL for the M1 "every scale suitable" claim; geometry and pose-following are otherwise sound where inspected.

- Two 1:500 representations are not printable under office-laser and must not be listed as available: concrete-mixer-truck (exhaust-stack 0.18 mm < 0.20 mm) and forklift (fork-tine-left/right 0.19 mm < 0.20 mm). The driver logged these renders as ok, because `runPreview` returns normally on a representation error and only sets the exit code. M1 report and QA12 both state every scale is suitable.
- Three views (side, front, plan) are present on every sheet inspected. Heights agree across side and front views, and plan widths and depths match the asset dimensions within a few centimetres. Poses and clip instants change the drawing as authored (mixer drum and chute, excavator boom/stick/bucket, forklift mast/forks, heat pump not inspected).
- Minor: expansion valve at 1:100 labels its height 0.08 m against asset heightM 0.09 (1:50 labels 0.09 m). Mixer plan label 2.70 m against depthM 2.55 (mirror arms; documented in QA12).
- Legibility: at 1:100 and 1:200, the valve pose change (closed vs half-open) and coil fin detail (cold vs hot) are not discernible; the humidifier at 1:200 is a small glyph with dimension text crowding its leader lines.
- Coverage is partial: 1:50 and 1:100 excavator, 1:50 forklift, mixer 1:50 and 1:200, heat pump, diesel, and all but the first montage of humidifier, valve and coil were not opened. Every SVG was rasterized and counted (282 views sheets plus the two explanation boxes); the unviewed montages are not verified by eye.

## concrete-mixer-truck (priority)

Viewed: `concrete-mixer-truck-100-0`, `-100-1`, `-100-2` (all 11 sheets at 1:100).

- Three views on every sheet (side, front, top). Captions and scale bars sit inside each panel; no overrun.
- Poses at 1:100: transit-agitating, parked-off, charging, discharging, chute-swing. The chute is stowed up-left in transit, angled down-left at discharging with its tip near the ground, and swung left in the top view for chute-swing. Side, front and top agree on the chute position in each pose.
- Drum: side view shows the drum; top view shows the tapered outline of the tilted drum (not a circle), matching the revolved-solid rule in QA12.
- Clip discharge-sequence at 4, 8 and 12 s: the chute raises and lowers; elevation height 4.28 m at 4 s and 3.95 m at 8 and 12 s; the front view height matches the elevation in every instant.
- Chute-swing: elevation and front both 4.33 m; top view extends the chute past the body to the left.
- Extents: elevation height 3.95 m matches heightM. Plan depth label 2.70 m vs depthM 2.55 (mirror arms included at 1:100; documented in QA12).
- Clip agitate-at-rest at 2.5, 5 and 7.5 s: the drum is static in all three instants, as expected for agitation at rest (the drum rotation is not visible in a line drawing).

### concrete-mixer-truck 1:500 (not printable)

- `concrete-mixer-truck--1-500--explanation.svg`: "Not printable at 1:500 (office-laser) Feature exhaust-stack: 0.18 mm < 0.20 mm." Offers 1:200, 1:100, 1:50.
- Driver logged all 1:500 queries as ok. The representation is not suitable under office-laser, contradicting the M1 report.

## excavator

Viewed: `excavator-200-0` (dig-cycle at 2.25, 4.5 and 6.75 s, digging) and `excavator-500-0` (same instants). `-50` and `-100` not opened.

- Three views on every sheet. Boom, stick and bucket follow the pose in side view: boom low at 2.25 s, raised at 4.5 s, raised further with bucket tipped at 6.75 s, and tucked at digging. Front view stacks the boom and cab consistently.
- Plan width 2.99 m matches depthM 2.99. Heights agree: 4.26 / 5.47 / 6.49 / 3.57 m in both side and front views.
- Stick and bucket are readable at 1:200. At 1:500 the bucket and stick detail blur, but the silhouette and extents are correct.
- No ladder artefacts (the QA12 extruded-profile regression) visible at 1:200 or 1:500.

## forklift

Viewed: `forklift-200-0` (approach, drive-with-load at 1.88, 3.75, 5.63 s) and `forklift-200-1` (lift-pallet at 2.5, 5, 7.5 s, lifted-load). `-50` and `-100` not opened. No 1:500 montage exists.

- Three views on every sheet. Mast tilt and forks follow the pose; the pallet rises with the forks; lifted-load reaches 3.53 m.
- Plan width 1.20 m matches depthM 1.20; heights agree across side and front (2.26 m; 3.30 m at lift 7.5 s; 3.53 m lifted-load).
- 1:500 not printable: `forklift--1-500--explanation.svg`: fork-tine-left and fork-tine-right at 0.19 mm < 0.20 mm. Driver logged as ok.

## evaporative-humidifier

Viewed: `evaporative-humidifier-50-0` (drip 0.3, 0.6, 0.9 s, dry), `-100-0` (same), `-200-0` (same). `-50-1`, `-50-2`, `-100-1/2`, `-200-1/2` not opened. No 1:500 (unavailable by design).

- Three views on every sheet. Height 0.60 m (side and front), depth 0.40 m (plan), consistent with asset dimensions.
- Drip instants show the pad wetting only faintly (dots within the pad outline); dry vs drip is not distinguishable in the top view at any scale.
- 1:200: the unit is a small glyph; the 0.60 m dimension text overlaps its own leader line in side and front views. Legible, but crowded.

## expansion-valve

Viewed: `expansion-valve-50-0` (closed, half-open, modulate 0.75 and 1.5 s) and `-100-0` (same). `-50-1/2`, `-100-1/2` not opened. No 1:200 or 1:500 (unavailable).

- Three views on every sheet. Height 0.09 m at 1:50, 0.08 m at 1:100 (asset heightM 0.09); depth 0.06 m in plan.
- Closed vs half-open vs modulated: the stem and body shape change is not discernible at 1:50 or 1:100. The three poses look the same at the printed size.
- Finding: at 1:100 the drawn height reads 0.08 m, not 0.09 m as the asset states. Check whether the top of the stem is dropped at this scale.

## finned-heat-exchanger (coil)

Viewed: `finned-heat-exchanger-50-0` (cold, hot, idle, reverse at 15 s) and `-200-0` (same). `-100-0` not opened. No 1:500 (unavailable).

- Three views on every sheet at 1:50: fin stack visible in front view, tube bank in side view, plan shows the 0.5 m depth and 0.2 m width. Heights 0.52 m (side, front) match.
- At 1:200 the coil is a small glyph; fin detail is not visible and cold/hot/idle are indistinguishable. The fin stack is a solid block in front view.
- Captions and scale bars sit inside each panel.

## heat-pump-outdoor-unit, diesel-generator

Not opened in this pass (montages exist: heat-pump 50/100/200/500, diesel 50/100/200/500). Recommend a follow-up pass before sign-off.

## Driver note (method)

- Rendering: one bun process (`R-M1/driver.ts`) rendered 304 pose/clip queries at all authored scales as SVG views sheets (`--no-png`), then one python process rasterized 282 SVGs at 150 dpi (pymupdf), then 81 montages (4 per image) were built.
- Explanation boxes (2 files) were excluded from montages.
- R-M1 and its scripts were deleted after the cited PNGs were copied to `T/🗑️generated/R-M1-cited`. The "Viewed" montage names above refer to the deleted R-M1 folder; the cited PNGs there are:
  - `concrete-mixer-truck-100-1.png` (chute-swing and discharge sequence, 1:100)
  - `concrete-mixer-truck--1-500--explanation.png` (1:500 not printable)
  - `excavator-200-0.png` (dig cycle, 1:200)
  - `forklift-200-1.png` (lift pallet, 1:200)
  - `expansion-valve-100-0.png` (closed, half-open, modulate, 1:100)
