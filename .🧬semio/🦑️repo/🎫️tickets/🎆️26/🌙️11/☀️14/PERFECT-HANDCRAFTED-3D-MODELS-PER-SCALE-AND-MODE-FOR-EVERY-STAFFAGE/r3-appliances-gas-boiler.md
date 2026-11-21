# Round 3: appliances/gas-boiler

State on 2026-11-20: finished and spliced. No earlier round-3 work existed for this asset (only the round-1 script `author-12-gas-boiler.py`). At the start `r3-validate.ts` was VALID, the catalog `volumes` equalled the source, and `volumes-check.ts --product` FAILED thermal coverage at all three scales (casing 1.22 against 0.9 m2, sight glass 0.008 against 0.0042, flue collar −17 %, 1:200 total 31 %).

Final state:
- `r3-validate.ts appliances/gas-boiler --catalog-only --volumes`: `VALID (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- `volumes-check.ts appliances/gas-boiler --product`: `PASS` at 1:50, 1:100 and 1:200 (1:500 stays unavailable, as its representation).

Scripts in the ticket folder (inputs, kept):
- `r3-gas-boiler-kit.py`: the measures of the boiler and the solid writers.
- `r3-gas-boiler-model.py`: states every solid of every scale and list by hand; writes `volumes/appliances/gas-boiler.json`.
- `r3-gas-boiler-texts.json`: the volume decisions, en and de.
- `r3-gas-boiler-roots.py`: the root fixes (height, rig, anchors, 2D of all scales, materials, hidden regions, texts, manifest height). Builds and validates the candidate; `--apply` writes the changed members. It refuses to run a second time on an already fixed catalog.
- `r3-gas-boiler-rest.py`: the second support `wall-rest` (anchor, supports, contacts, statics text).
- `r3-gas-boiler-areas.py`: sets `areaM2` of casing and hidden regions in place.

## 1. Reference

| Quantity | Real subject | Source | Model |
|---|---|---|---|
| Class | Wall-hung, room-sealed, premix condensing gas boiler of 20 kW: pressed white steel front hood over a galvanised chassis (back sheet with folded edges) that hangs on a wall rail; stainless heat exchanger with the burner in its combustion chamber; premix fan; expansion vessel; hydraulic group (pump, gas valve, plate heat exchanger for hot water, diverter, siphon) at the bottom; controls fascia at the lower front; concentric flue adapter on top; connections below. | EN 15502-1 (`en-15502-1-2015`); maker manuals (Vaillant, Viessmann, Remeha) | As in the rows below |
| Casing | Vaillant ecoTEC plus: 720 x 440 x 335 mm, low-lift weight from 36.8 kg; Remeha Calenta Ace 28c: 690 x 450 x 450 mm; class 690 to 770 mm high, 400 to 450 mm wide | Vaillant UK brochure (web search 2026-11-20, new reference `vaillant-ecotec-plus`); retailer data for Remeha | 720 x 440 x 330 mm, 350 mm over the knobs |
| Flue adapter | Concentric 60/100 or 80/125 mm, about 50 to 100 mm above the casing | Vaillant data | 80/125: Ø125 turret 30 mm with seal seat, Ø80 x 3 mm PP spigot with a 64 mm lip seal, top at 0.82 m |
| Connections | Five: flow, hot water, gas, cold water, return; 15 to 22 mm with union nuts, about 50 to 60 mm below the casing | Maker installation manuals | Five Ø22 x 1.5 mm brass tubes, 80 mm pitch, hex nuts 31 mm across flats |
| Hood sheet | 0.7 to 1 mm painted steel, sharp pressed front edges, side edges rounded | Asset layer `sheet` 0.8 mm | Hood with 14 mm rounded side-to-top edges, crisp front edges |
| Heat exchanger | Stainless coil pack about Ø 0.2 to 0.25 m, 0.15 to 0.2 m deep, burner inside, burner door with sight glass | Vaillant/Viessmann radial stainless exchangers | Coil ring Ø 0.13 / 0.23 m, 0.17 m deep, 10 mm door and back plate, flue collector Ø80 to the collar |
| Expansion vessel | Flat, 6 to 10 l, Ø 0.3 to 0.4 m | Maker data (Vaillant VC 206/5-5: 10 l) | Ø 0.32 m lens of two dished shells, 80 mm deep, flange |
| Mass | 30 to 40 kg | Vaillant (36.8 kg low lift) | 29.75 kg unchanged (rig masses) |

## 2. Audit of the state before round 3

**Dimensions**: the casing was 0.62 m high (total 0.72 m with collar and stubs). The class is 0.69 to 0.77 m for the casing alone: the boiler was 14 % too squat (casing ratio 1.41 against 1.64 for the 440 mm wide Vaillant).

**Rig and anchors**: bones are right in kind (casing, connections, collar, knob, two flame slides). Wrong only through the height: collar at 0.67, flames at 0.441, bracket at 0.64. The anchors had no depth: the wall bracket and the flue outlet sat on the mid-plane z 0, 0.175 m in front of the wall and 0.04 m behind the collar axis. With one pin support at the wall the overturning moment of the weight could not be carried.

**2D** (all scales): consistent with each other, but the front and plan outlines drew r 14 to 20 mm corners on every edge (also at the wall), which neither the pressed hood nor the chassis have; the front view drew a door line on the outline itself; the plan "top edge" was an inset rectangle that matches no edge.

**Solids** (`qa-visual.md`: body B/B/B, thermal C/C/C, structure B/B/B):
- Body: the casing was three stacked slabs in depth (box, chamber frame, front plate), so a seam ran round the casing 25 mm behind the front; the collar base was an open revolve filled through the axis (kernel trap), faceted; knobs were flat discs.
- Thermal: an almost empty cavity with a floating burner bar and flames; no heat exchanger, fan, pump, valve, vessel (no hidden regions existed); several lofts without caps (kernel traps); flue-gas emitter a disc inside the collar.
- Structure: a hollow box plus an 8 mm back plate, the mass clipped against the back wall, the support a 0.2 x 0.175 m block.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `dimensions.heightM` | 0.72 | 0.82 (casing 0.72 + collar 0.05 + stubs 0.052); manifest `🗂️catalog/🔣️.json` heightM 0.72 → 0.82 |
| `provenance.references` | 4 | + `vaillant-ecotec-plus` |
| `provenance.notes` (en, de) | casing 0.44 x 0.62 x 0.35, total 0.72 | casing 0.44 x 0.72 x 0.33 (Vaillant 720 x 440 x 335), 0.35 over the knobs, total 0.82 |
| `materials` | 6 | + `galvanised-steel` (chassis), `stainless-steel` (heat exchanger), `die-cast-aluminium` (fan) |
| `rig.casing` | lengthM 0.67, comY 0.4 | 0.77, 0.42 (weighted from heat exchanger, vessel, hydraulics, chassis, hood) |
| `rig.collar`, `rig.flame`, `rig.flame-tall` | y 0.67, 0.441, 0.441 | 0.77, 0.541, 0.541 |
| `anchors.wall-bracket` | y 0.64, z 0 | y 0.74, z 0.165 (the rail in the pocket of the back sheet) |
| `anchors.flue-outlet` | z 0 | z 0.04 (on the collar axis) |
| `anchors.wall-rest` | — | new: roller, y 0.075, z 0.165, normal away from the wall (tiltDeg −90), 0.3 x 0.03 m, μ 0.3 |
| `structure.supports` | wall-bracket | + wall-rest; assumption rewritten: 292 N at the rail, ±73 N couple (292 x 0.165 / 0.665); second assumption: the chassis load path |
| poses (3), clips (3) | contact wall-bracket | + wall-rest; poses + the four hidden regions (exposed 1, sunlit 0) |
| `thermal.regions.casing` | areaM2 0.9, char. length 0.7 | 1.35 (C16 limit; honest 1.40, listed in `area-review.md`), 0.72 |
| `thermal.regions.controls` | areaM2 0.03 | 0.025 (display 2 x 0.0066 + knobs 0.0058 + 0.0026 + gauge 0.0018 = 0.024) |
| `thermal.regions` | 6 | + hidden passive `heat-exchanger` 0.34, `fan` 0.04, `expansion-vessel` 0.18, `hydraulics` 0.21 (hand calculation = modelled parts), one layer each |
| `thermal.assumptions` | 2 | + hidden regions sentence |
| `representations.50/100/200` | see §2 | everything above y 0.30 lifted by 0.10 (casing outline, window, chamber, burner, bezel, door top, slits, logo, collar in the front view); front and plan casing and panel corner radii removed (E1: crisp pressed edges); 1:50 plan `top-edge` redrawn as the two tangent lines of the rounded side edges; 1:50 front `door` (lay on the outline) removed; new `rear-seam` in 1:50 front and plan (hood edge 20 mm before the wall); decisions numbers (16.4, 8.2, 4.1 mm; 0.72 m casing) and seam wording, 1:500 reason 1.64 mm |

Forces view (1:50, hot-water): reaction at the rail 301 N (292 N up, 72.4 N towards the wall), at the rest 72.4 N; residual ΣF 0.006 N, ΣM 0.015 N·m (before the rest: ΣMx −48.1 N·m).

## 4. Volumes per scale (own solids / expanded)

**1:50: body 35/46, thermal 33/33, structure 20/30**
- Body: `casing` = the hood as one extrude of the elevation outline (14 mm rounded side edges, crisp front edges) with the real Ø73 window opening, 310 mm deep to the rear seam; `chassis` = the last 20 mm to the wall, 1.5 mm inset; porthole: `bezel` revolve 8 mm proud with chamfer and a lip into the opening, `sight-glass` 4 mm (glazing), `chamber` dark stainless cup 120 mm deep; `burner-tube` 52 x 11 x 46 mm; flames as three-tongue crowns (`flame-tall`, `-l`, `-r` 24/32 mm deep; `flame`, `-l`, `-r` nearer the glass); `panel` fascia 410 x 145 x 10 mm r 8; `knob` Ø52 with skirt and chamfer; `set-knob` Ø36 x 5; `gauge` domed glass; `collar-base` turret with seal seat, `collar-pipe` spigot with lip seal, `flue-bore` dark; `stubs` x5 tubes, `nuts` x5 hex. Texture: door seam ring, 4 slits, 2 logo bars, LED, display, 2 text lines, 3 ticks, pointer, notch, needle.
- Thermal: casing = closed hood + `{of}` chassis, panel, bezel; controls = display plate 2 mm + knob, set knob, gauge; sight glass = own disc with rim and back as section faces; burner = bar + 6 flame tongues; flue collar = turret + spigot, `th-flue-gas` Ø60 outlet patch flush in the bore (D2); connections = stubs + nuts. Hidden: `th-hx` (coil ring, burner door with sight hole, back plate, chamber inside), `th-hx-collector`; `th-fan` + `th-fan-duct` to the burner door; `th-vessel` lens with flange; `th-pump`, `th-gas-valve`, `th-plate-hx`, `th-manifold`, `th-siphon`, `th-flow-pipe`, `th-return-pipe`.
- Structure: `member-chassis` (back sheet with folded side edges, one U), `member-chassis-top` and `-bottom` (20 mm angles), `member-hx-brackets-low` and `-high` (2 x 2 angles, touching the coil at x ±0.08), `member-hx`, `member-flue-collector`, `member-manifold` + `member-manifold-bracket`, `member-collar-spigot`, `member-collar-turret`, `joint-collar-flange`, `member-connections-pipes` x5, `joint-connections-bulkheads` x5, `joint-mode-knob` shaft, `joint-flame`, `joint-flame-tall` slides, `support-wall-bracket` (hook rail profile), `support-wall-rest` (rest strip), `mass-casing` at the rig centre of mass (0, 0.42, 0).

**1:100: body 11/15, thermal 18/22, structure 14/20** — hood with Ø78 window and rear-seam chassis band, 50 mm dark chamber, burner bar, two single crowns, plain 10 mm band, Ø56 knob, two solid collar steps, Ø26 stubs; thermal with hood, fascia, display, knob, glass, burner, flames, collar with outlet, open stub tubes, heat exchanger + collector, fan drum, vessel lens, one hydraulic block and a riser; structure with chassis U, both edge angles as one repeated solid, one bracket pair, solid heat-exchanger drum, collector, stepped collar, flange, manifold with bracket merged, 5 stub tubes, knob shaft, one flame slide, rail, rest, mass.

**1:200: body 4, thermal 12, structure 9** — hood block, 15 mm control band, Ø125 collar drum, 0.34 m connection bar; thermal hood, band, glass disc, burner block, collar, outlet, bar, heat-exchanger drum + collector, fan, vessel and hydraulic blocks; structure chassis U, one bracket shelf, heat-exchanger drum, collector, collar, connection bar, rail, rest, mass.

**Final `volumes-check.ts --product`: PASS.** Silhouette IoU elevation / front / plan: 1:50 0.999 / 0.998 / 0.998, 1:100 1.000 / 1.000 / 1.000, 1:200 1.000 / 1.000 / 1.000; p95 ≤ 1.9 mm. Thermal outer area at 1:50 (declared): casing 1.404 (1.35), controls 0.024 (0.025), sight glass 0.0040 (0.0042), burner 0.0092 (0.01), flue collar 0.048 (0.05), connections 0.037 (0.04), heat exchanger 0.344 (0.34), fan 0.040 (0.04), vessel 0.180 (0.18), hydraulics 0.211 (0.21); all within 15 % at 1:50 and 30 % at 1:100; 1:200 total 6.5 %.

## 5. What I saw in the final renders (all opened with the Read tool)

- 2D candidate and final (`r3-2d` 1:50, 1:100, 1:200; light-line, light-solid, thermal, forces): the boiler now has the slender proportion of a 20 kW wall boiler; window in the upper third, fascia low, collar and stubs unchanged; front and plan with crisp corners and the rear seam; forces balanced with rail and rest.
- `r3-sheet` 1:50 body, six panels: front (+x) shows the hood side face with the chassis band at the wall, fascia, knobs and bezel standing out; elevation shows door seam, slits, logo, porthole, LED, fascia with display, three knobs, five stubs with nuts; back and left side closed (galvanised back); top shows collar with seal seat and bore and the chassis band; three-quarter reads as a real boiler.
- Porthole (render with the glass left out, crops at the window, poses hot-water, heating, off): dark chamber, burner bar, outer and inner flame crowns at full output; only the inner crown when heating; burner only when off (flames sink into the hood solid). The glass is translucent in the product; `r3-sheet` draws it opaque.
- 1:50 thermal cut: heat exchanger ring with burner and flames inside, fan with duct, vessel lens behind, pipes down to pump, valve, plate exchanger, manifold, siphon; collector up into the collar.
- 1:50 structure, front, side and from behind: chassis U with edge angles, brackets under and over the coil, collector into the spigot, flange ring, manifold on its angle, stubs through bulkheads, rail at the top and rest strip at the bottom of the back, mass in the casing.
- 1:100 and 1:200 body, thermal cut and structure sheets: generalised as described, recognisable; the 1:100 see-through window found in the first render (hole through to the back) was fixed with the chassis band.

## 6. Still imperfect (stated plainly)

- `r3-sheet` shades large flat faces with visible fan-shaped gradients (per-vertex lighting over long cap triangles of the extrudes); the faces are flat and the effect is not in the geometry. Revolves (bezel, knobs, collar) show 12- to 16-gon facets: the kernel chooses the segment count.
- The hood is modelled as the solid volume it encloses with a window tunnel; the dark chamber cup lines the tunnel. A real hood is a 0.8 mm shell over an air box, which a body list cannot show without hiding the burner.
- The front edges of the hood are crisp (a pressed edge radius of about 3 mm is below the 7.5 mm feature); some makers round them more.
- The casing `areaM2` is 1.35 against the honest 1.40 (C16 cap); listed for the reviewers in `area-review.md` with the wall contact fraction 0.23 it implies (the poses keep 0.15).
- The rig carries no depth for centres of mass, so the statics take the weight on the mid-plane, 0.165 m in front of the rail; the real centre lies about 0.14 m in front of the wall. The mass solid sits at the rig's centre for consistency.
- Mass stays 29.75 kg (a 20 kW system boiler; combi boilers weigh up to about 37 kg).
- Condensate drain and pressure-relief outlet below the casing are not modelled: the 2D draws five connections only.
- The four hidden regions are passive at room air; heat between water and flue gas is not solved (stated in the thermal assumptions).
- Not run: the staffage test suite (`caption-fit` uses gas-boiler at 1:200; the drawing is now 0.1 m taller).

Tool output: `🗑️generated/r3-gas-boiler/` deleted; my renders in `🗑️generated/r3-sheet/` and `🗑️generated/r3-2d/` deleted.

## Round 5 handcraft

Process (round 5, 2026-11-21): rendered and opened with the Read tool, before and after: `r3-sheet` body, structure and `--list thermal --cut` at 1:50, 1:100, 1:200 and 1:500, every pose at 1:50 and one clip frame, the `r3-2d` drawings of every drawn scale (1:500 has no drawing: its representation is unavailable, the 3D 1:500 is a digital scale). Coarse sheets were opened as 2 x 2 contact sheets (`r5-grid.ts`, half size). Measurements with `r5-e01-measure.ts` (world extents per solid in the default pose). Root edits through `r5-e01-roots.py` (candidate validated with `r3-validate.ts --file --volumes` before writing), volumes through the asset's `r5-<slug>-model.py` (which re-uses the hand-stated round-3 lists it keeps and states every new solid by hand), splice with `volumes-insert.py`. The round-3 reference table above stays the reference; the table below measures the model against it.

### Model against reference (Vaillant ecoTEC plus 720 x 440 x 335 mm, 80/125 flue, 22 mm connections)

| Quantity | Reference | Model 1:50 | Deviation |
|---|---|---|---|
| Casing height | 720 mm | 720 mm (y 0.05 to 0.77) | 0 % |
| Casing width | 440 mm | 440 mm | 0 % |
| Casing depth (hood + chassis) | 335 mm | 330 mm (310 + 20) | -1.5 % |
| Over the knobs | about 350 mm | 351 mm | 0 % |
| Flue turret / spigot | 125 / 80 mm | 125 / 80 mm | 0 % |
| Overall height with flue and stubs | 0.82 m (`dimensions`) | 0.820 m | 0 % |
| Connections | 5 x 22 mm, 80 mm pitch | 5 x 22 mm, 80 mm pitch, nuts 31 mm AF | 0 % |
| 1:100 / 1:200 / 1:500 overall | 0.44 x 0.82 x 0.35 m | 0.44 x 0.82 x 0.350 / 0.340 / 0.340 m | within one feature |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged, re-read: hood, porthole, flame crowns, fascia, flue adapter, stubs with nuts; nested internals; chassis load path) |
| 1:100 | A- / A- / A- | A- / A- / A- (unchanged) |
| 1:200 | B+ / B (expansion vessel a square 0.31 m slab, fan a cube) / A- | A- / A- / A- |
| 1:500 | C+ (inherited digital model: asymmetric 13/35 mm chamfers on the front edges, a 9 mm hollow ring as flue collar below the 75 mm feature, one generic decision sentence) / B / B | A- / A- / A- |
| Poses off, heating, hot-water, clip heating-cycle t 2 s | A- | A- (flame crowns sink into the hood as designed; nothing torn) |
| 2D 1:50 / 1:100 / 1:200 | A- | A- |

Catalog before round 5: INVALID (`PRINT range-end-feature` of the 1:200 control band at 1:350; `connections` outer area 0.034 against 0.04 m2, 15 %).

### Changes

- 1:200 body: the control band is a 30 mm strip that stands 15 mm proud of the hood and runs 15 mm into it; the 1:200 front and plan drawings draw it 30 mm deep (root edit `representations.200.geometry.front/plan.panel`), so the band prints to the end of the range 1:350 (the 15 mm band printed 0.173 mm there).
- 1:200 structure: because the thinnest 1:200 casing drawing is now 30 mm, the chassis is generalised to a channel 30 mm deep and the rail and rest strip to 30 mm bars at their anchors, offset 0.5 mm off the chassis back face (the first render showed z-fighting marks there).
- 1:200 thermal: the expansion vessel is its real 0.32 m lens (revolve of two dished shells), the premix fan its drum.
- 1:500 authored anew (`r5-gas-boiler-model.py`): crisp hood block 0.44 x 0.72 x 0.325 m, control strip, solid 125 mm flue drum, connection bar; thermal with flush window disc, burner block, outlet patch on the flue drum, exchanger drum, fan drum, vessel lens, hydraulic block; structure chassis channel, bracket shelf, heat-exchanger drum, collector and adapter as one stepped tube, connection bar, rail, rest, mass. Three bilingual decisions.
- Root: `thermal.regions.connections.areaM2` 0.04 -> 0.035 (factor 1.14; measured 1:50 outer area of the five tubes and nuts 0.034 m2).
- Final: `r3-validate.ts appliances/gas-boiler --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts --product` PASS.

### Remaining issues

- The porthole with flames is the asset's didactic feature (real condensing boilers have no window); kept because rig, poses and clips are built on it.
- Revolves are 12- to 16-gons at the product tessellation (knobs, bezel, collar); flat faces show the renderer's fan shading. Neither is in the geometry.
- Condensate drain and pressure-relief outlet still not modelled (the 2D draws five connections).
