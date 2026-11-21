# Round 3 — appliances/kitchen-extractor-hood

State: DONE (with the open points listed under "Still imperfect")

Resumed from the interrupted session: `r3-kitchen-extractor-hood-kit.py` (dimensions and builders), `-model.py` (volumes), `-roots.py` (root fixes and 2D). The catalog had none of it applied and carried a bulk-pass 1:500 volume built on the old wrong geometry; that one is replaced.

## Reference

Source: Bosch DWP96BC50B data sheet (90 cm wall-mounted pyramid hood), as cited in `provenance.references` (`bosch-dwp96bc50b`). The data sheet was read by the interrupted session; it was not fetched again in this one.

| Quantity | Reference | Model |
|---|---|---|
| Width / depth of canopy | 900 / 500 mm | 0.9 / 0.5 m |
| Canopy height without chimney | 246 mm | 0.246 m (rim band 45 mm + pyramid 201 mm) |
| Top plate | chimney 250 wide sits on it | 0.26 x 0.2 m |
| Chimney width / depth | 250 mm / about 190 mm | 0.25 x 0.19 m lower, 0.244 x 0.187 m upper (telescopic) |
| Total height | 799 to 975 mm | 0.915 m (lower channel 0.4 m, upper 0.254 m visible, duct stub 15 mm) |
| Hanging screws | 270 mm apart at 190 mm; safety screws near the lower edge | pin anchor at 0.19 m, roller at 0.03 m, screws at +-0.135 m |
| Outlet | centre 85 mm from the wall, 150 mm duct | duct axis at 0.085 m, radius 0.075 m |
| Net weight | 10.3 kg | 10.3 kg (7.6 canopy with fan, 1.2 lower, 1.5 upper channel; split estimated) |
| Sheet | 0.6 mm stainless (typical, estimate) | 0.6 mm shells in thermal |
| Filters / lamps | washable aluminium cassettes, two LED lamps | three cassettes 0.28 x 0.32 m, two modules 0.12 x 0.05 m |

Slopes that follow: front 34 degrees, sides 32 degrees from horizontal.

## Audit (before)

- Canopy was a wedge prism (side profile extruded over 0.88 m): no side slopes, top 0.4 m deep, 0.2 m high. A real hood of this class is a pyramid on three sides.
- Rig: `chimney` joint at y 0.2 (real seat 0.246), length 0.35 (0.4); `chimney-top` at 0.35 (0.4), length 0.365 (0.269).
- Chimney drawn 0.3 x 0.3 m and 0.28 m (real 0.25 x 0.19 m). Filter drawn as a 20 mm strip below the canopy and lamps as visible blocks; both are hidden under the canopy in all three views.
- Mass 14.5 kg (data sheet 10.3 kg). Outlet anchor at x 0.14 (0.085).
- Structure was the body shells relabelled, supports and masses invisible (qa-visual.md). 1:100 was a near copy of 1:50.
- Catalog 1:500 (bulk pass): the old wedge and 0.3 m chimney tubes, structure as `of` references to the whole body.

## Root fixes (catalog file, by `r3-kitchen-extractor-hood-roots.py --write`)

| Member | Old | New |
|---|---|---|
| `dimensions.massKg` | 14.5 | 10.3 |
| bone `canopy` mass | 8.5 kg, com (0.22, 0.1) | 7.6 kg, com (0.18, 0.1) |
| bone `chimney` | y 0.2, length 0.35, 3.2 kg | y 0.246, length 0.4, 1.2 kg, com (0.13, 0.2) |
| bone `chimney-top` | y 0.35, length 0.365, 2.8 kg | y 0.4, length 0.269, 1.5 kg, com (0.12, 0.06) |
| anchor `outlet` | (0.14, 0.365) | (0.085, 0.269) |
| anchors `wall-hook`, `wall-rest` | footprint width 0.3 | 0.28, titles name the screws and pads |
| region `canopy` | areaM2 1.14, layer 0.8 mm | 0.97, layer 0.6 mm (resistance 4e-05, capacity 2370) |
| region `filter` | areaM2 0.84, visible | 0.57 (three cassettes, both faces and edges), `hidden` |
| region `lamps` | areaM2 0.03, visible | 0.025 (two 2 mm diffusers), `hidden` |
| region `chimney` | areaM2 0.96 | unchanged; characteristic length 0.65 |
| materials | 3 | + `galvanised-steel`, `switch-plastic` |
| 2D 1:50, 1:100, 1:200 | wedge, 0.3 m chimney, filter strip | redrawn as the true views of each scale's body (E1): pyramid profile, rim edge, hips in plan, chimney channels, duct stub; 1:50 adds seam, louvres, switch panel; 1:100 merges the 3 mm telescopic step; 1:200 merges stub and step |
| `representations.500` | reason names the filter strip | reason names the 45 mm rim band (still unavailable) |
| summary, notes, reference, structure assumption, emitter title | old texts | rewritten for the pyramid hood and 10.3 kg |

`dimensions` height, width and depth are unchanged (0.915 / 0.5 / 0.9). All area changes are within the factor 1.5 of C16.

## Model per scale (solids body / thermal / structure)

- **1:50 — 26 / 19 / 17.** Body: rim band with return flange, three sloping sheets as trapezoids in their own planes (exact hips), top plate, back sheet, base panel with five openings, three filter cassettes (frame, mesh, latch), two lamp modules, switch panel with four buttons, two U-channel chimney tubes with return lips, louvres, 150 mm duct, wall bracket. Thermal: 0.6 mm canopy shell in ten sheets, three cassettes, two diffusers, both channels, outlet patch in the duct mouth. Structure: back sheet, top plate, two folded hips, rim, both channels, wall bracket, seat and telescope joints, two keyhole-screw supports and two pad supports outside the back plane, fan unit (volute, motor, outlet box) and two channel masses.
- **1:100 — 12 / 12 / 14.** Rim as one slab with the underside openings, one filter field, two lamp blocks, plain channels of one width, duct. Structure adds a rim frame so the hips end on a member.
- **1:200 — 8 / 11 / 7.** Closed rim slab, five 30 mm plates, two channels with the stub merged; supports as strips, fan as one mass.
- **1:500 — 8 / 11 / 6** (digital scale, no drawing). Closed rim slab and five 50 mm plates closing into one mass with true slopes; two closed chimney prisms. Structure: one hanger angle, two channels, two support strips, fan block.

Final check: `volumes-check.ts appliances/kitchen-extractor-hood --product` PASS; `r3-validate.ts ... --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0), also with `--catalog-only`. Silhouette IoU 0.998 to 1.0 in all views at 1:50, 1:100, 1:200. Thermal areas at 1:50: canopy 0.949 against 0.97, filter 0.574 against 0.57, lamps 0.0254 against 0.025, chimney 1.045 against 0.96.

## What I saw

- 1:50 body, six views and one from below: reads at once as a pyramid chimney hood; hips straight, telescopic step visible, louvres on both sides, duct and bracket visible from the back and above; underside shows three cassettes with latches, two lamps, the switch panel on the front rim.
- 1:50 structure (after the hip fix): rim frame, two hips ending inside the corners, back sheet, top plate with the fan unit hanging under it, both channels, orange telescope overlap, green screws and pads on the wall side.
- 1:50 thermal cut: shell, green cassettes, lamps, chimney channels. Opened before the rim band was split into four strips; the re-render after that change was not reopened.
- 1:100 body, structure, thermal cut and 2D: clean; one chimney width, duct in the open top.
- 1:200 body, structure, thermal cut and 2D: clean; reads as hood at 2.5 x 4.6 mm.
- 1:500 body and thermal (final state) and structure (opened before the last two edits, which did not touch structure): clean after two fixes found by looking (plates poking 18 mm below the rim at 75 mm thickness; outlet disc z-fighting with the prism top).
- 2D 1:50, 1:100, 1:200: true orthographic views, economical lines.

## Still imperfect

- **No C31 flood in the product.** Inward faces of the canopy shell carry `sectionFaces` as a stand-in, as other appliance authors did. Without it the canopy reads 1.77 m².
- **Chimney area.** `areaM2` 0.96 is kept; the model measures 1.045 at 1:50 because both faces of the wall-open channels count. Whether the inner faces belong in the value is a question for the area reviewers.
- **Poses and clips.** The three poses move no bone; only the default pose was rendered, no clip frame.
- **1:500 sheets** have no rig overlay: `r3-sheet.ts` throws on a scale without a drawing (entry in `tool-requests.md`); `r3-kitchen-extractor-hood-digital-sheet.ts` is the workaround.
- **Not modelled:** the fan unit is only a structural mass, not a body or thermal solid (it is enclosed); the duct belongs to no thermal region; screw heads and keyhole slots are below 7.5 mm.
- **Sheet thickness in body:** canopy sheets are 8 / 15 / 30 / 50 mm thick towards the hidden inside (feature limit); outer faces are true.

## Files

- `r3-kitchen-extractor-hood-kit.py`, `r3-kitchen-extractor-hood-model.py`, `r3-kitchen-extractor-hood-roots.py`, `r3-kitchen-extractor-hood-digital-sheet.ts`
- `volumes/appliances/kitchen-extractor-hood.json`
- Catalog: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/kitchen-extractor-hood/🔣️.json`
- Renders: `🗑️generated/r3-sheet/appliances/kitchen-extractor-hood.*`, `🗑️generated/r3-2d/appliances/kitchen-extractor-hood.*`, `🗑️generated/r3-kitchen-extractor-hood/`

## Round 5 handcraft

Scripts: `r5-kitchen-extractor-hood-model.py` (keeps the hand-stated lists of the round-3 model, restates the back sheet at 1:100 to 1:500, the coarse rim slab, the 1:200 and 1:500 canopy load path and the coarse thermal markers), `r5-kitchen-extractor-hood-texts.json` (1:200 and 1:500 structure decisions). Renders in `🗑️generated/r5-kitchen-extractor-hood/{before,after}/` (opened: body, structure and thermal cut at every scale, poses `off`, `low`, clips `switch-on-warm-up` t 300 and `step-down-and-off` t 150 at 1:50, close-ups of the 1:200 and 1:500 wall side).

### Reference and measurement (1:50)

| Quantity | Reference (Bosch DWP96BC50B) | Model | Deviation |
|---|---|---|---|
| Canopy W x D | 900 x 500 mm | 900 x 500 | 0 % |
| Canopy height | 246 mm | 246 | 0 % |
| Chimney W x D | 250 x 190 mm | 250 x 190 (upper 244 x 187) | 0 % |
| Total height | 799-975 mm | 915 | in range |
| Outlet | 150 mm at 85 mm from the wall | 150 at 85 | 0 % |
| Hanging screws | 270 mm apart at 190 mm | 270 at 190 | 0 % |
| Net weight | 10.3 kg | 10.3 | 0 % |
| Slopes | follow from the above | front 34 deg, sides 32 deg | exact |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | unchanged (re-opened) |
| 1:100 | A- (faint stripes along the wall in side view) / A- / A- | A- / A- / A- |
| 1:200 | B (stripe band along the wall in side and plan: the back sheet's edges coplanar with the slopes; spots on the wall face) / A- / B (no rim frame, no hips: only back sheet, top plate, channels) | A- / A- / A- |
| 1:500 | B (same stripes, larger) / B+ / C+ (hanger, channels and fan only; the canopy carried nothing) | A- / A- / A- |
| Poses/clips | A- (no bone moves) | unchanged |

### Changes

- Back sheet at 1:100, 1:200, 1:500 set 2 mm inside the side slopes and under the top face of the top plate, its wall face 0.5 mm proud of the slope ends; the closed rim slab of 1:200 and 1:500 likewise 0.5 mm proud: no coplanar faces left (side, plan and wall views clean).
- Structure 1:200 and 1:500: rim band as a closed frame (30 / 50 mm) and the two folded hips from the top plate corners down to it.
- Thermal 1:100 to 1:500: the canopy shell is closed at these scales, so the inward `sectionFaces` stand-ins are removed (C31 g); the flood measures canopy at -0.8 % (1:100) and the totals at 6 % / 1 %. At 1:50 they stay: removing them reads 1.73 m2 against 0.97, because the open duct lets air into the shell through the top plate opening (real: extract air does pass through); this is the area reviewers' question whether `canopy` counts the inside of the extract path.
- `volumes-check.ts --product` PASS, `r3-validate.ts --volumes` VALID; spliced.

### Remaining

- 1:50 canopy inner faces still carry `sectionFaces` (see above).
- Poses move no bone (fan speed only in thermal).
- Fan unit only as a structural mass.

### Enclosure and blower working dimensions, 21 November

The released model uses 0.6 mm stainless sheet, a pierced top plate, an open telescopic chimney, a horizontal blower shaft and 24 curved vanes. A four-section transition joins the rectangular blower discharge to the round outlet. Two semicircular backdraft leaves rotate on their shared spindle at 0°, 46°, 67° and 80°. The inspection clip turns the impeller through a full revolution. The fixture measured the independent clearance of the wheel, case, motor, outlet and flaps at all four digital scales. Schema, asset and all measured digital geometry rules passed.

The manufacturer reports a 900 × 500 mm footprint, a 246 mm canopy, a 150 mm outlet, 626 m³/h at full extraction, 220 W connected load and two 1.5 W LED lamps. The material gauges, internal wheel dimensions, intermediate airflow settings and partition of the remaining 217 W between shell and exhaust are authored engineering estimates. At full load the model delivers 3 W at the lamps, 21.7 W to the shell and 195.3 W to the exhaust. [Bosch product data sheet](https://media3.bosch-home.com/Documents/specsheet/en-SG/DWP96BC50B.pdf)

An independent reader opened all twelve shipped GLB scenes in 1,590,348 bytes. All 39 sampled print sizes passed across the three available paper bands. The 1:500 digital scenes remain available.
