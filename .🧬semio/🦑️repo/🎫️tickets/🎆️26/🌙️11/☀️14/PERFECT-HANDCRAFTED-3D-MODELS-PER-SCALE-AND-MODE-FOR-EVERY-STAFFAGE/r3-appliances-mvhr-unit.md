# Round 3: appliances/mvhr-unit

State: finished and spliced (2026-11-20). No earlier round-3 work existed for this asset, so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts in the ticket folder:
- `r3-mvhr-unit-geo.py`: measures, frame helpers, outlines (rounded rectangle, volute, blade, rail, hook).
- `r3-mvhr-unit-volumes.py`: every solid of every scale and list, stated by hand.
- `r3-mvhr-unit-roots.py`: the root fixes and the 2D shapes of 1:50, 1:100 and 1:200.
- `r3-mvhr-unit-build.py`: builds the candidate `🗑️generated/r3-mvhr-unit/🔣️.json` and the source `volumes/appliances/mvhr-unit.json`.
- `r3-mvhr-unit-apply.py`: writes the candidate's root members into the catalog file, keeping its formatting.
- Probes: `r3-mvhr-unit-solve.ts` (support reactions per pose) and `r3-mvhr-unit-suit.ts` (print suitability per scale).

To resume: build, `r3-validate.ts --file <candidate>`, apply, then `volumes-insert.py`.

## 1. Reference

Subject: a compact wall-hung whole-house MVHR unit of the 250 m3/h class with four spigots on top.

| Quantity | Real value | Source | Model |
|---|---|---|---|
| Casing W x H x D | 550 x 550 x 285 mm (Kinetic); 560 x 635 x 260 mm (energiSava 260) | Vent-Axia Kinetic spec; EnviroVent ENV627 data sheet (web search 2026-11-19) | 0.6 x 0.6 x 0.3 m kept (asset class estimate, within about 10 %): casing 0.285 m deep plus 15 mm of wall rail |
| Mass | 14 to 15 kg for plastic/EPP units; steel-cased units are heavier | same | 24 kg kept (steel wrapper 1.37 m2 x 7.8 kg/m2 = 10.7 kg, plus core, fans, EPP and filters) |
| Spigots | four on top, 125 mm on these references, 100 mm on small units | same | DN100, 98 mm OD, 0.7 mm galvanised wall, double-lip EPDM seal of 114 mm; exhaust and supply in the back row, extract and outdoor intakes in the front row |
| Heat exchanger | composite or polystyrene counter-flow plate pack, hexagonal | Kinetic spec; my knowledge | 0.53 x 0.26 m hexagon, 0.195 m stack, 88 plates at 2.2 mm, wetted area 18.3 m2 |
| Fans | EC centrifugal, backward-curved shrouded impeller of about 190 mm, external-rotor motor in the hub | ebm-papst R3G190 class (my knowledge) | supply impeller of 190 mm: shroud with a 124 mm eye, conical back plate, 7 blades of 2.5 mm, rotor cap of 70 mm; extract fan in a scroll under the exhaust spigot |
| Casing build | 0.8 to 1 mm steel or ABS shell, 20 mm or more of EPP lining that forms the channels, removable front cover, filters changed from the front | product knowledge | 1 mm steel wrapper with 25 mm bends, back tray with folded flanges, 19 mm EPP, 30 mm cover with an 8 mm rounded edge |
| Mounting | hung on a wall bracket or rail, with anti-vibration pads at the bottom | installation manuals | folded hook strip over a galvanised rail at 0.586 m, rubber rest strip 300 x 30 x 15 mm, 15 mm off the wall |
| Window | real units have none | n/a | kept as the asset's didactic feature (224 mm polycarbonate in a 256 mm bezel) |

Material data come from EN ISO 10456 (PS 1050 kg/m3, 0.16 W/mK, 1300 J/kgK; EPDM 1150, 0.25, 1000) and from estimates for EPP (35 kg/m3, 0.039) and filter media.

## 2. Audit of the state before round 3

- **Rig.** `casing` lay along +x from the lower centre with length 0.6 m, so half the bone was outside the unit. `spigots` was a 65 mm horizontal stub on the top. `impeller` sat at z 0 while its solids were at z -0.13. qa-visual graded body B, thermal C and structure C at every scale.
- **Anchors.**
  - Hook and rest were at z 0 (mid depth), not at the wall.
  - The rest's roller normal was +x (lateral). The solver therefore gave a pure 235 N hook reaction and no overturning moment, which contradicts the asset's own assumption text.
- **2D.**
  - The side face and the top face were drawn with an inset "cover seam" rectangle that does not exist on the real unit.
  - The casing was rounded at all four corners in the front and plan views, although the back is square to the wall.
  - Nine filled crescent blades were drawn as free-standing over the window (no shroud).
  - There was no wall fixing at all.
- **Body.** A 30 mm solid front slab and a solid wrapper, with no door, edge, chamber or fixing. The grip was a capsule.
- **Thermal.**
  - The casing was one 20 mm shell, although the layer data name steel plus EPP. The steel layer's heat capacity of 3000 J/m2K is below 1 mm of steel alone (3822).
  - The core, filters and extract fan were absent (empty box).
  - The ducts were closed cylinders that failed at 0.167 m2 against 0.1 m2.
- **Structure.** The casing was a filled shell block plus a box back. The support bars ended at mid-depth. The 1:200 thermal list was a copy of 1:100.

## 3. Root fixes (catalog file outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.casing` | angle 0, com (0, 0.3) | angle 90 (vertical centre line, rest to hook), com (0.3, 0) |
| `rig.spigots` | (0, 0.6), horizontal | (0.6, 0) in casing frame, vertical; com (0.03, 0) |
| `rig.impeller` | (0, 0.3), z 0, angle 0 | (0.3, 0), z -0.13, angle -90 (world orientation, poses and clips unchanged) |
| `anchors.wall-hook` | (0, 0.57), z 0, normal 0 | (0.586, 0), z 0.145, normal 90; title "Hook strip on the wall rail" |
| `anchors.wall-rest` | (0, 0.03), z 0, normal +x | (0.03, 0), z 0.15, tiltDeg -90 (wall pushes along -z) |
| `anchors.supply-spigot`, `exhaust-spigot` | (0.075 / -0.225, 0.065) | (0.065, -0.075 / 0.225), z 0.07 (the back-row spigot axes) |
| `materials` | 4 | + `epp-foam`, `polystyrene-plate`, `filter-media`, `epdm-seal` |
| `provenance` | 4 refs | + `iso-10456-2007`, `vent-axia-kinetic`, `envirovent-energisava-260`, `ebm-papst-r3g190`; notes rewritten (en, de) |
| `summary` | window plus 4 spigots | adds the rail, the shrouded impeller, the plate exchanger and the filters |
| `thermal.regions.casing` | 1 layer `sheet` (20 mm, R 0.5, C 3000); area 1.16 | layers `liner` EPP 19 mm (R 0.487, C 1197) and `sheet` steel 1 mm (R 2e-5, C 3822); C 5019; area 1.37 (factor 1.18) |
| `thermal.regions.impeller` | area 0.06 | `areaBasis: exchange` (blade fins); 0.06 kept; 0.11 proposed in `area-review.md` |
| `thermal.regions` | 4 | + hidden passive `core` (18.3 m2, exchange), `extract-fan` (0.078), `filters` (0.12) |
| `thermal.assumptions` | 2 | + hidden regions and casing area |
| `poses[*].thermal.regions` | 4 | + the three hidden regions (exposed 1, sunlit 0) |
| `structure.assumptions` | CoM 0.12 m, hook and rest at mid depth | rail at 0.586 m, CoM 0.15 m before the wall, about 63 N pull and push |
| `representations[50/100/200]` | 40 / 26 / 19 shapes | 46 / 25 / 17 shapes redrawn from the 3D measures (E1); decisions rewritten |

After the fix the solver gives, in every pose:
- hook: 235.4 N up and 64.6 N towards the wall;
- rest: 64.6 N away from the wall;
- residual 6e-14.

Dimensions, masses, poses, clips, emitters and the catalog index (height, width, 75 W) are unchanged.

2D (E1):
- **Elevation (front face):** casing rect r25, the 8 mm cover edge line, bezel, window, impeller (back plate, shroud ring with eye, 7 blade-edge strokes, hub), light, grip, spigots, seal rings.
- **Front view (unit side):** casing outline with 8 mm front corners and a square back, cover joint, hook strip, rail, rest strip, two spigots with seals.
- **Plan:** outline, cover joint, rail, hook, four spigots with seal rings and bores.
- **1:100:** window disc, shroud, eye and hub, spigots with 15 mm seal bands (elevation only), and the bracket.
- **1:200:** window and impeller discs, with spigots generalised to 90 mm. At 98 mm the office-laser profile rejected 1:200 for 0.08 mm gaps; `r3-mvhr-unit-suit.ts` now reports every scale suitable.

## 4. What was modelled

Counts are solids (with repeats).

### 1:50: body 19 (31), thermal 29 (39), structure 15 (23)
- **Body:**
  - Casing: 1 mm wrapper extrude; cover rim as a superelliptic loft with the 8 mm rounded front edge; flat cover face with the bezel opening; bezel ring; polycarbonate glass; black EPP fan chamber cup.
  - Impeller: shroud with inlet lip, conical back plate, rotor cap, 7 backward-curved blades (texture, 2.5 mm).
  - Front details: status-light lens and moulded pull grip.
  - Spigots: 4 galvanised tubes of 0.7 mm wall (2 x repeat) and 4 double-lip EPDM seals.
  - Fixing: galvanised wall rail (true Z profile), folded hook strip and rubber rest strip.
- **Thermal:**
  - Steel shell: rim skin, cover face, four wrapper panels, back, bezel, grip, light. The inward faces are marked `sectionFaces` because the product has no C31 cavity flood.
  - EPP liner shell nested inside it (wrap, front with window bore, back, fan chamber).
  - Glass (back face marked) and the impeller parts.
  - Four closed spigots with duct-section top and floor, plus the seals.
  - Hidden internals: hexagonal plate pack, extract-fan volute rising to the exhaust spigot, two filter cassettes.
  - Supply and exhaust outlet patches in the spigot bores.
- **Structure:** 1 mm wrapper ring, back tray with 17 mm folded flanges, hook strip, 4 spigot tubes, impeller back plate on a 12 mm shaft, 4 collars, rail (pin) and rest strip (roller) supports, and masses at the casing CoM, inside each spigot tube and in the hub.

### 1:100: body 9 (11), thermal 13 (15), structure 14 (18)
- **Body:** square-edged cover slab with the window opening, casing, one glass disc, a 15 mm chamber cup, the impeller as one revolve with eye and hub, spigots with a 15 mm seal band, a 15 mm bracket and the rest strip.
- **Thermal:**
  - Casing, cover and chamber are surface references, with the layers merged.
  - Glass disc (back face marked), impeller, closed ducts with bands, hexagonal pack, extract-fan drum, two cassettes, 15 mm outlets.
- **Structure:** wrapper, back tray, own tubes, impeller, 15 mm shaft and collars, bracket and rest supports, three masses.

### 1:200: body 6 (8), thermal 11 (13), structure 11 (13)
- **Body:** a casing block to the wall, the cover slab with its opening, glass, a 30 mm impeller disc, and 90 mm spigot tubes.
- **Thermal:** surfaces, glass, impeller, plain ducts, pack, extract-fan block, one filter slab, 30 mm outlets.
- **Structure:** wrapper and 30 mm tray to the wall, 90 mm tubes in 30 mm collars, impeller on a 30 mm shaft, 30 mm hook and rest bars, the casing mass only.

1:500 stays unavailable, mirroring its representation (1.2 mm on paper, 42 mm gaps). Decisions for each scale are three bilingual sentences (body, thermal, structure).

## 5. Checks (final, on the catalog asset)

- `$BUN volumes-check.ts appliances/mvhr-unit --product`: **PASS** (5.5 s).
  - Silhouette IoU (elevation / front / plan): 1:50 0.999 / 0.996 / 0.994; 1:100 1.000 / 0.998 / 1.000; 1:200 1.000 / 1.000 / 1.000.
  - Bounds within 6.5 mm at 1:50.
  - Area deviation at 1:50: casing 0.1 %, window 1.1 %, ducts 2.5 %, extract-fan 0.2 %, filters 0.5 %. At 1:100 every region is under 30 %. The 1:200 total is under 30 %.
- `$BUN r3-validate.ts appliances/mvhr-unit --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. The catalog `volumes` equal `volumes/appliances/mvhr-unit.json`.

## 6. What the renders show (opened with the Read tool)

Seen:
- `r3-sheet` body at 1:50, 1:100 and 1:200 (six views each);
- 1:50 right and three-quarter views without the glass (a debug copy of the source, restored afterwards);
- 1:50 `boost-spin` at 2 s, right view;
- thermal cut at 1:50 and 1:100 (six views each);
- structure at 1:50 and 1:200 (six views each);
- `r3-2d` at 1:50, 1:100, 1:200 and 1:50 `boost-spin` at 2 s, plus the candidate 2D at 1:50.

Rendered but not opened: thermal cut at 1:200, structure at 1:100.

What the views show:
- **Elevation (front face):**
  - A square steel box with 25 mm bends and four seal-ringed spigots on top.
  - The bezel with the window, a light at lower right and the grip at the bottom.
  - Through the window: a black chamber, the light shroud ring, and in its eye the blade edges, back plate and dark rotor cap. It reads as a radial fan.
- **Side views:**
  - The 8 mm rounded cover edge and the cover joint.
  - At the back, the rail, hook strip and rest strip in the 15 mm gap to the wall.
  - Spigots in two staggered rows.
- **Back:** the rail across the top, the rest strip at the bottom, and the plain back panel.
- **Top:** the four spigot rings in a zigzag, the cover joint and the rail.
- **Three-quarter:** reads at once as a wall MVHR unit.
- **Thermal cut:**
  - Red EPP liner inside the pink steel skin.
  - A cyan hexagonal plate pack filling the middle, the magenta volute at upper left rising to the exhaust spigot, and green filter cassettes under the front spigots.
  - The purple impeller behind the green glass, and yellow ducts.
- **Structure:**
  - A thin blue sheet box open at the front, showing the back tray from inside.
  - The impeller disc on its shaft, orange collars at the spigot roots, green rail and rest strip, and the magenta CoM ball.
  - At 1:200 the same in coarser form.
- **2D:** clean at every scale; the clip frame turns the blade-edge strokes.

## 7. Still imperfect

- **Inspection window.** It is not a real MVHR feature; I kept it because the asset's summary, the window region and the spin clips rest on it. The shrouded impeller is now realistic behind it.
- **Glass in the renders.** The glass is opaque in `r3-sheet` (the product shows glazing at 45 % opacity). The impeller was therefore checked with the glass removed from a debug copy.
- **Shading streaks.** Large flat faces show diagonal shading streaks in `r3-sheet` (the old model showed them too). They come from the renderer's interpolation over long fan triangles, not from the geometry.
- **Hidden thermal regions.** The solver treats them as passive at room air: the temperatures of the core between the two streams are not solved (written in the thermal assumptions).
- **Impeller area.** `impeller` keeps 0.06 m2 under `areaBasis: exchange`; the honest wetted area is about 0.11 m2 (more than factor 1.5, so it is listed in `area-review.md`).
- **`sectionFaces` stand-in.** Inward faces of the steel shell and of the glass carry `sectionFaces`, standing in for the missing C31 flood (`tool-requests.md`, 2026-11-20, heat pump). They can go once the product floods cavities.
- **Not modelled:**
  - condensate drain, power cable, screws, labels, bypass flap and electronics box;
  - the extract fan's impeller inside its scroll;
  - EPP channel ribs between spigots and filters.
- **Spigots at 1:200.** They are 90 mm in 2D and 3D (printability), not the real 98 mm.
- **Mass.** 24 kg is above the 14 to 15 kg of the plastic references; it was kept because no cited steel-cased reference contradicts it.
- **Test suites.** The repo test suites were not run; only the checks of the brief.

## Round 5 handcraft

Process (round 5, 2026-11-21): rendered and opened with the Read tool, before and after: `r3-sheet` body, structure and `--list thermal --cut` at 1:50, 1:100, 1:200 and 1:500, every pose at 1:50 and one clip frame, the `r3-2d` drawings of every drawn scale (1:500 has no drawing: its representation is unavailable, the 3D 1:500 is a digital scale). Coarse sheets were opened as 2 x 2 contact sheets (`r5-grid.ts`, half size). Measurements with `r5-e01-measure.ts` (world extents per solid in the default pose). Root edits through `r5-e01-roots.py` (candidate validated with `r3-validate.ts --file --volumes` before writing), volumes through the asset's `r5-<slug>-model.py` (which re-uses the hand-stated round-3 lists it keeps and states every new solid by hand), splice with `volumes-insert.py`. The round-3 reference table above stays the reference; the table below measures the model against it.

### Model against reference (Vent-Axia Kinetic 550 x 550 x 285; EnviroVent energiSava 260 560 x 635 x 260; DN100 / 125 spigots)

| Quantity | Reference | Model 1:50 | Deviation |
|---|---|---|---|
| Casing width x height | 550 to 560 x 550 to 635 mm | 600 x 600 mm (asset class estimate kept in round 3) | +7 % on the width of the references, inside the class height |
| Depth incl. wall rail | 260 to 285 mm + rail | 285 mm + 15 mm rail = 300 mm | inside the class |
| Overall height with spigots | 0.665 m (`dimensions`) | 0.662 m | -0.5 % |
| Spigots | DN100 (98 mm OD) to 125 mm | 98 mm, 0.7 mm wall, 114 mm seals | 0 % against DN100 |
| Supply impeller | about 190 mm (R3G190 class) | 190 mm shroud, 124 mm eye | 0 % |
| 1:100 / 1:200 / 1:500 overall | 0.60 x 0.665 x 0.30 m | 0.60 x 0.662 x 0.302 / 0.302 / 0.301 m | within one feature |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged, re-read: wrapper with bends, rounded cover, bezel and window, light, grip, spigots with seals, rail, hook and rest; steel and EPP shells, plate pack, extract scroll, filters; wrapper, tray, tubes, collars, impeller on its shaft) |
| 1:100 | A- / A- / A- | A- / A- / A- |
| 1:200 | A- / B+ (extract fan a 144 mm cube beside round parts) / A- | A- / A- / A- |
| 1:500 | B (inherited: one generic decision sentence, 2 mm window and 0.7 mm hollow spigots copied from 1:200) / B / B+ | A- / A- / A- |
| Poses off, normal, boost, clip boost-spin t 2 s | A- | A- |
| 2D 1:50 / 1:100 / 1:200 | A- | A- |

Catalog before round 5: INVALID (`PRINT range-end-gap` of the 1:200 spigots, 0.041 mm at 1:350).

### Changes

- Root: `representations.200.validRange.maxDenominator` 350 -> 210: the four 90 mm spigots stand 60 mm apart; with the pen the gap is 0.15 mm on paper only up to 1:214 (the spigots cannot be thinner without losing the DN100 reading).
- 1:200 thermal: the extract fan is its 144 mm scroll drum as at 1:100.
- 1:500 authored anew (`r5-mvhr-unit-model.py`): crisp casing block, flush window, impeller disc, the four spigots as solid 90 mm drums; thermal casing, window face, impeller, closed duct drums, plate pack, extract drum, filter slab, thin outlet patches on the spigot tops; structure wrapper ring, back plate, spigot tubes on collars, impeller shaft, hook, rest, mass. Three bilingual decisions.
- Final: `r3-validate.ts --volumes` VALID, `volumes-check.ts --product` PASS.

### Remaining issues

- The inspection window is the asset's didactic feature; kept.
- Casing 600 mm square is about 7 % wider than the cited 550 to 560 mm units (asset class estimate, not changed without a cited 600 mm reference).
- At 1:100 and 1:200 the thermal cut shows hatch noise on the side faces where the casing and cover references meet in one plane (render artefact of coplanar faces); the casing region is grey in merged-layer scales (tool request of curtain-wall-bay applies).
