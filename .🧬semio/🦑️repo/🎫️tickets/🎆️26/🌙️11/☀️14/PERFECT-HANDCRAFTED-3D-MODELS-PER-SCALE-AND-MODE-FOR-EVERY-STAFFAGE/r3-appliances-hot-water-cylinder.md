# Round 3: appliances/hot-water-cylinder

State on 2026-11-20: finished and spliced. No earlier round-3 work existed for this asset (only the round-1 script `author-12-hot-water-cylinder.py`), so I started from scratch. At the start `r3-validate.ts` was VALID and the catalog `volumes` equalled the source `volumes/appliances/hot-water-cylinder.json`. The catalog file now holds the corrected roots, the redrawn 1:50 drawings and new volumes at all four scales.

Final state:
- `r3-validate.ts appliances/hot-water-cylinder --catalog-only --volumes`: `VALID (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- `volumes-check.ts appliances/hot-water-cylinder --product`: `PASS` at 1:50, 1:100, 1:200 and 1:500.

Scripts in the ticket folder:
- `r3-hot-water-cylinder-kit.py`: the measures and the writers for the solids. Measures are the jacket, the vessel, the DIN 28011 Kloepper head geometry, the feet, the stubs and the cover ellipse.
- `r3-hot-water-cylinder-model.py`: states every solid of every scale and list by hand. It writes `volumes/appliances/hot-water-cylinder.json`.
- `r3-hot-water-cylinder-texts.json`: the volume decisions, each in en and de.
- `r3-hot-water-cylinder-roots.py`: builds the candidate document and runs `--file` validation on it. With `--apply` it writes only the changed members into the catalog file, in the catalog's style. It is idempotent.

## 1. Reference

| Quantity | Real subject | Source | Model |
|---|---|---|---|
| Class | Floor-standing, indirectly heated, unvented DHW cylinder of 200 l. Enamelled steel tank with one smooth-tube coil, rigid PU foam jacket with a sheet or foil skin, plastic top cover, adjustable feet, inspection and cleaning opening. | EN 12897:2016 (`en-12897-2016`); Viessmann Vitocell 100-V data sheets, read 2026-11-20 (new reference `viessmann-vitocell-100-v`) | As in the rows below |
| Overall size | Vitocell 100-V CVAA 200 l: 581 mm diameter, 605 mm over the connections, 1409 mm high. CVAB-A 200 l: 634 to 637 mm by 1349 mm. Makers' range 0.55 to 0.64 m by 1.3 to 1.53 m. | Viessmann data sheets (CVAA and CVAB-A tables) | 0.60 m jacket, 0.66 m over the stubs, 1.48 m high. `dimensions` unchanged because they are inside the class. |
| Pressure vessel | 480 mm outside diameter, 3 mm wall. At 10 bar, t = p D / 2 sigma = 1.0 MPa x 0.48 / 240 MPa = 2 mm, plus allowance for corrosion and forming. Torispherical heads with crown radius D and knuckle radius 0.1 D. | DIN 28011 (new reference `din-28011`) | Cylindrical shell from 0.20 to 1.20 m. Head depth 93 mm, apices at 0.107 and 1.293 m. Inner volume 0.1765 + 2 x 0.0107 m3 = 198 l. |
| Insulation | 50 to 60 mm rigid PU foam bonded to the tank, a skin with one vertical lock seam, bottom insulation over the plinth, top insulation under the cover | Asset layer `jacket`: 0.06 m, R 1.7; ISO 10456 | Foam from r 0.240 to 0.300 m. 43 mm under the bottom head, 87 mm over the top head. |
| Feet | Three adjustable feet (screw and pad) at 120 degrees | Asset structure assumption ("a third foot lies behind them"); maker installation manuals | Pitch circle 0.25 m. One foot at x -0.25. Two at x +0.125, z ±0.2165, one behind the other in the side elevation. 50 mm pad, M16 screw, 36 mm socket. |
| Connections | Cold inlet low, coil flow and return, circulation, sensor well, hot outlet at the top. R 3/4 to R 1 nipples with union nuts. | Vitocell 100-V connection table; EN 12897 | 24 mm stubs with 34 mm hex nuts at the drawn heights, and a 30 mm hot-water outlet on the cover |
| Mass | Empty 200 l cylinder about 70 to 90 kg | Data-sheet class | 80 kg (`massKg` unchanged) |

## 2. Audit of the state before round 3

**Rig**
- The bones are a didactic idealisation: four 0.33 m thermal quarters, the cover, the base, the connections and the needle. They are consistent with the asset's purpose, so they are kept.
- Defect: the `connections` mass sat on the axis at comY 0.74. The actual stubs have their centre at about x -0.02, y 0.84.

**Anchors and structure**
- The two support anchors at x ±0.215 described a symmetric two-foot stand. The 3D model had four feet (2 x 2 boxes at z ±0.12), but the structure assumption speaks of three feet.
- With three feet at 120 degrees and one foot alone in the side elevation, the planar statics are exact only if the anchors sit at x -0.25 (one foot, 1/3 of the load) and x +0.125 (the pair, 2/3).

**Dimensions**: inside the class (see the reference table). Unchanged.

**2D at 1:50**
- The feet were 70 x 60 mm blocks: no pad, screw or socket. The front view had only two feet.
- The zip was a 0.27 m stroke on one quarter only, which matches no real jacket.
- No inspection cover was drawn.
- The dial bezel was missing from the front and plan views.
- 1:100 to 1:500 draw the feet as a base band and the stubs at the feature floor. Those drawings are consistent generalisations and stay.

**Solids** (`qa-visual.md` grades: body B/B/B, thermal B/B/B, structure C/C/C)
- Body: 24-gon fluting, four boxes as feet, no inspection cover, the dial was a texture sheet.
- Thermal:
  - The four zones were solid cylinders with no layer shells.
  - Their interfaces counted as outer area: 1.18 m2 against 0.62 m2, and FAIL at every scale (`check-before`, 12 measurements).
  - The plume was a 0.4 m disc floating above the dome (violates D2).
  - At 1:200 and 1:500 the thermal lofts had no caps (`kernel-traps.md`).
- Structure:
  - The tank was four stacked 8 mm tubes with girth rings at the didactic quarter joints, which are not welds.
  - The heads were a 0.8-scaled dome copy.
  - There were no legs, so the pads were not connected to the tank.
  - The cover floated.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `anchors.foot-left` | x -0.215, footprint 0.07 x 0.07, "Left foot" | x -0.25, footprint 0.05 x 0.05 (the pad) |
| `anchors.foot-right` | x 0.215, "Right foot" | x 0.125, footprint 0.05 x 0.05, "Right pair of feet" / "Rechtes Fußpaar" |
| `rig.connections.mass` | comX 0, comY 0.74 | comX -0.02, comY 0.84 (the centre of the six stubs) |
| `materials` | (none) | added `enamelled-steel`: emissivity 0.9, 7850 kg/m3, 50 W/mK, 460 J/kgK, E 210 GPa, strength 235 MPa |
| `thermal.regions.zone-1..4.layers` | `jacket` only | added the innermost layer `tank-wall`: metal, 3 mm, R 6e-5, 10830 J/m2K. The solver sums only resistances, so the change is negligible. |
| `structure.assumptions[0]` | "two feet in the plane of the view (a third foot lies behind them)" | three feet at 120 degrees on a 0.25 m circle, carrying 1/3 and 2/3 (en, de) |
| `provenance.references` | 4 | added `viessmann-vitocell-100-v` and `din-28011` |
| `representations.50` (E1) | see §2 | listed below |

Changes to `representations.50`:
- Feet:
  - The elevation `foot-l` and `foot-r` are now the true side outline of a levelling foot (pad, chamfer, screw, socket) at x -0.25 and +0.125.
  - In the front view, `foot-l` is the rear foot on the axis.
  - The front `foot-r` is one path with two subpaths, the pair at ±0.2165.
- Seam: the one-quarter `zip` is replaced by the lock seam `seam-4`..`seam-1` over the full height, in the elevation (x 0.22) and the front (x -0.204).
- Inspection cover: new `inspection-cover`, a filled detail circle of r 0.08 on zone-4 in the elevation, and edge-on in the front and plan views.
- Dial: `dial-bezel` is now also drawn edge-on in the front and plan views.
- Decisions: `silhouette`, `retained` and `merged` (en, de), plus the "zip" wording in `thinElements`.

The forces view now shows reactions of 264 N (left) and 520 N (right pair), with residual 0. Thermal areas were not changed: every region is within tolerance at 1:50 (C29).

## 4. Volumes per scale (expanded solid ids)

**1:50: body 32, thermal 21, structure 30**
- Body:
  - Three levelling feet (`foot-l`, and `foot-r` repeated twice), each one revolve with pad, chamfer, M16 screw and socket. Plinth `base-ring`.
  - Four jacket quarters. `zone-4` has a 10 mm rolled bottom edge.
  - `cap`: a 3:1 half ellipsoid with 16 steps, with `cap-seam` lying on the surface normal.
  - Five brass nipples `stub-0..4`, starting inside the jacket, with 34 mm hex union nuts `nut-0..4`. `hot-out` on the cover.
  - Thermometer: `dial-bezel`, 8 mm proud, with a chamfer. Glass, three ticks and the needle are texture.
  - Type plate `label` (texture). `inspection-cover`, a plastic cap 10 to 21 mm proud. The full-height lock seam `seam-4..1` (texture).
- Thermal:
  - Per quarter, nested layers, innermost first: `th-steel-k`, the 3 mm vessel wall as a closed loop (Kloepper bottom head in zone-4, top head in zone-1); and `th-jacket-k` = `{ of }` the quarter, with cuts `-v`/`+v`.
  - `th-cap` = `{ of cap }`, with `-v`.
  - `th-stub-k` are only the exposed 30 mm of the nipples, with the jacket face marked `±u` (C25 b). The nuts are references. `th-hot-out` is the exposed part above the cover.
  - The plume outlet `th-top-plume` is an 8 mm skin on the crown of the cover within r 0.2 (D2).
- Structure:
  - `shell-4..1`: the 3 mm vessel courses with Kloepper heads.
  - Joints: girth welds `weld-bottom` and `weld-top` at the real head seams. `leg-welds`. `weld-pipe-0..4` and `weld-pipe-hot` where the nipples enter the shell. `needle-pivot`.
  - Members: three welded `legs` (24 mm), three `screws`, nipple tubes `pipe-0..4` (24 x 3.5 mm), `pipe-hot`, `cover-skin` (3 mm ABS), and `cover-collar`.
  - `cover-collar` is the rigid foam ring bonded to the upper course. It is the only path for the cover's weight, so the cover no longer floats.
  - Supports: `support-foot-left`, and `support-foot-right` repeated twice.
  - Masses: `mass-4..1` at the centre of each quarter.

**1:100: body 13, thermal 12, structure 21**
- Body:
  - Feet and plinth are merged into a 0.5 m base drum, as the 1:100 drawing shows.
  - The quarters keep a 15 mm bottom edge. Cover with 8 steps.
  - The nipples are 24 x 40 mm oval stubs (revolve `lateralScale` 1.667), with the nut merged; front and plan draw them 40 mm wide. The outlet is 30 x 40 mm.
  - The dial is a shallow dome.
- Thermal: quarters as `{ of }` with cuts, cover, five stubs, outlet, 16 mm plume patch.
- Structure:
  - Shells with coarser heads, cover skin and collar, 15 mm girth beads.
  - Legs as one bar per foot, two pads, nipples as bars, four masses.

**1:200: body 7, thermal 9, structure 16**
- Body: base drum, lower and upper jacket halves (as drawn), cover with 6 steps, outlet, one 50 mm nipple per side.
- Thermal: one closed cylinder slice per quarter (layers merged), cover, three nipples, 32 mm plume patch.
- Structure: shells with three-facet heads, cover skin and collar, three legs, two 60 mm pads, three bars, four masses. No welds.

**1:500: body 7, thermal 9, structure 14**
- Body: 0.12 m base as drawn, jacket halves, cover with 4 steps, outlet and side nipples as 110 mm blocks.
- Thermal: slices, cover, the three blocks, 76 mm plume patch.
- Structure: shells with single-facet heads, cover skin and collar, three legs of 76 mm on two 75 mm pads, one outlet bar, four masses.

**Final `volumes-check.ts --product`**: PASS, 41 rule lines.

Silhouette IoU:

| Scale | Elevation | Front | Plan |
|---|---|---|---|
| 1:50 | 0.996 | 0.998 | 0.995 |
| 1:100 | 0.999 | 0.998 | 0.993 |
| 1:200 | 0.999 | 1.000 | 0.994 |
| 1:500 | 0.999 | 0.999 | 1.000 |

Thermal outer area in m2:

| Region | Declared | 1:50 | 1:100 |
|---|---|---|---|
| zone-1..3 | 0.62 | 0.620 | 0.620 |
| zone-4 | 0.62 | 0.616 | 0.613 |
| top-cap | 0.31 | 0.339 | 0.338 |
| connections | 0.03 | 0.027 | 0.036 |

At 1:200 and 1:500 the totals compared under C15 deviate by 1.2 % and 0.9 %.

## 5. What I saw in the final renders

All renders were opened with the Read tool.

**r3-sheet 1:50 body, six panels**
- Side elevation (-z):
  - The quarters stand on the plinth and three feet; the pair overlaps exactly.
  - The dial has a bezel and needle. In the `cold` pose the needle turns left by 70 degrees.
  - Also visible: the type plate, the round inspection cover, the lock seam at the right and the nipples with nuts on both sides.
- Front (+x): three feet (rear one in the middle). The two right-side nipples are seen end-on as hexagons. The dial and the cover show as small bumps on the left edge, as the redrawn front drawing shows them.
- Back: three feet and the hexagons of the left nipples.
- Top: the cover with its seam ring, the outlet, the nipples, and the thin edges of dial and cover.
- Three-quarter (turn 300, tilt 15): the dial and cover read as real parts, the nuts as hexagons, the feet as levelling feet.
- Overlays (`volumes-render`): green everywhere in all three views at every scale; only a few pixels of blue at the 1:100 dial, under one feature.

**1:50 structure**
- Front, right, back and left: a real pressure vessel. Kloepper heads, two girth welds, three splayed legs on pads, nipples welded into the shell, and the hot-water pipe up through the cover.
- Top: the cover skin.
- Three-quarter: the vessel inside the ghost jacket.

**1:500 structure**: the legs now connect the vessel to the pads.

**1:50 thermal**
- Uncut: colours per quarter, the cover in teal with the ring of the plume patch on its crown, the connections in magenta.
- Cut (`--cut`) and the `volumes-render` cutaway: the steel vessel with both heads is nested inside each quarter's jacket colour, and the cuts between the quarters are hatched.
- 1:100 to 1:500 cutaways: hatched slices with layers merged, as declared.

**2D (`r3-2d`, 1:50, light-line and forces)**: the three feet, the seam over the full height, and the cover circle. Forces: left foot 264 N and right pair 520 N, residual 0.

**1:100 body sheet**: base drum, oval stubs, dial dome.

## 6. Still imperfect (stated plainly)

- Small round parts are visibly faceted. The dial bezel, inspection cover and feet tessellate as 12- to 16-gons at 1:50, and the cylinder shows faint flutes. The kernel picks the segment count; a solid cannot set it.
- The top silhouette of the cover shows a faint jagged dark edge in the side elevation. It is a tessellation artefact of the revolve and was there before this round.
- The thermal outer layer `jacket` is the closed quarter, not the 60 mm foam annulus. The product's `outerArea` has no C31 cavity flood: the annulus measured 1.117 m2 against 0.62. This is logged in `tool-requests.md` as the fourth case. The steel layer is still a real nested 3 mm shell.
- The internal coil, the magnesium anode and the sensor wells are not modelled. The asset has no region for them (hidden regions are not declared) and they are not load-bearing for the stand. Adding a hidden water or coil region would change the thermal model and is a question for the thermal maintainer.
- The four jacket quarters show seams and tone steps in the body. That is the asset's didactic division, which the 2D also draws; a real jacket is continuous.
- Masses sit inside the opaque vessel shells, as their real centres of mass do.

Tool output was kept only under `🗑️generated/r3-hot-water-cylinder/` (deleted at the end), `r3-sheet`, `r3-2d` and `volumes-tools`.
