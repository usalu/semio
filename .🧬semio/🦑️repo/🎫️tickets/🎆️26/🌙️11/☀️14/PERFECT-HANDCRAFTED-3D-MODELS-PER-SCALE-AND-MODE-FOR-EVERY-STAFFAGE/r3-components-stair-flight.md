# Round 3: components/stair-flight

State: finished and spliced. `r3-validate.ts components/stair-flight --catalog-only` returns `VALID ... schema 0, validateAsset 0`. `volumes-check.ts components/stair-flight --product` returns `PASS` at every scale and for every rule. The print suitability of all four representations is `authored` with no violations (`r3-stair-flight-suit.ts`).

No earlier round-3 work existed for this asset: no `r3-stair-flight-*` scripts and no report. The source matched the catalog.

Scripts (all in the ticket folder):

- `r3-stair-flight-kit.py`: the real geometry (step rule, HEA faces, bracket, plates, anchors, folded 1:500 band), the masses and centre of mass, the world/bone frame conversions, and the solid writers.
- `r3-stair-flight-volumes.py`: every solid of every scale and list, stated part by part. It writes `volumes/components/stair-flight.json`.
- `r3-stair-flight-roots.py`: root values and the 2D drawings of all scales and of the symbol, as true views of the same geometry. Without a flag it writes a candidate to `🗑️generated/r3-stair-flight/🔣️.json`. With `--apply` it rewrites only the listed members of the catalog file, keeping its layout.
- `r3-stair-flight-suit.ts`: prints the product's print-suitability violations per scale. It is needed because `validateAsset` does not report them.

## 1. Reference

| item | real value | source |
|---|---|---|
| step rule | 2 r + g = 0.59 to 0.65 m; for buildings in general r ≤ 0.19 m, g ≥ 0.26 m | DIN 18065:2020-08 |
| chosen steps | 16 rises of 181.25 mm (2.90 m storey), going 280 mm, 2 r + g = 0.6425 m, slope 32.92° | DIN 18065 |
| open stair | nosing lap ≥ 30 mm | DIN 18065 |
| opening between treads | ≤ 0.12 m only where small children are to be expected; the 141 mm open riser here means a non-residential stair | MBO §38 and state codes |
| HEA 160 | h 152, b 160, t_w 6, t_f 9, r 15 mm; A 38.8 cm², 30.4 kg/m, I_y 1673 cm⁴, W_el 220 cm³, surface 0.906 m²/m; extreme fibre h/2 = 76 mm | EN 10365 |
| centre-stringer (Mittelholm) flight, construction | one stringer under the middle of the treads; a welded bracket per tread (fin plate on the top flange plus a horizontal carrier plate; the tread is screwed on from below); base plate with anchors on the lower floor; head plate bolted to the landing face, slotted holes giving a roller | steel stair practice (Stahlbau Atlas, Detail 2001; fabricator details) |
| treads | C24, 420 kg/m³, 40 mm. Cantilever of 0.42 m beyond a 360 mm carrier under a 0.8 kN edge load: σ 5 MPa, deflection 1 mm | EN 338, own calculation |
| loads | q_k 3.0 kN/m² over 1.2 × 4.48 m = 16.13 kN on the single stringer; person 0.8 kN | EN 1991-1-1 (authored value, as before) |
| stringer check | M ≈ 6.5 kNm under full load, σ ≈ 30 MPa ≪ 235 MPa | own calculation |

## 2. Audit findings (before this round)

- **Step proportions:** 12 rises of 241.7 mm with goings of 300 mm, 38.85°. That gives 2 r + g = 0.783 m, 13 cm above the DIN 18065 range. The rise is also 52 mm over the 0.19 m limit. Nobody walks this flight comfortably, and an architecture teacher rejects it at first sight. This was the root defect of the whole asset.
- **Load model contradicted the body:** the provenance and loads said "one stringer of a pair, 0.6 m tributary width", but the body, the 2D and the summary show a single centre stringer under 1.2 m treads. The loads carried half the real value: 1.402 kN/m and a 400 N person share.
- **Rig bone on the wrong line:** the bone ran along the tread back edges, 107 mm above the stringer axis, so the member drawn in the force views passed through the treads. The supports sat at the floor and landing-floor corners, not at the bearings. `extremeFibreM` was 0.08 instead of h/2 = 0.076; W = 1673/7.6 = 220 cm³ confirms the correct value.
- **HEA depth:** modelled and drawn 160 mm deep instead of 152 mm.
- **Stringer ends:** the stringer had no bearing hardware.
  - The foot was a bare horizontal cut on the floor.
  - The head ran 255 mm into the landing as a sharp wedge with no bearing face. The plan showed it as a stub lying on the landing.
- **Brackets:** 60 mm blocks.
- **Screws:** countersunk screws were driven from the top through 40 mm timber into steel. Real treads are screwed from below.
- **1:100:** a stripped copy of 1:50.
- **1:500:**
  - The body was a copy of 1:200.
  - Its 75 mm bars drawn with 106 mm gaps cannot print once the rise is corrected (see 1:500 below).
- **Structure:** the supports were small blobs and the mass sat on the surface.
- **`massKg` 166.2:** this was neither the half stringer nor the whole flight.

## 3. Root fixes (catalog file, outside `volumes`)

| member | old | new |
|---|---|---|
| geometry | 12 × 241.7 / 300 mm | 16 × 181.25 / 280 mm, lap 30, tread 310 × 1200 × 40 |
| `dimensions.widthM` | 3.885 | 4.51: from the first nosing at −0.03 to the landing face at 4.48 |
| `dimensions.massKg` | 166.2 | 347.3 |
| mass breakdown | – | HEA 153.4, treads 100.0, carriers 61.5, fins 14.5, plates 17.8, anchors 0.2 kg |
| rig `stringer` | x 0, angle 38.853°, L 4.62277, mass 166.2 at (2.31, 0) | x 0.21401 (centroid axis on the floor), angle 32.915847°, L 5.08177 (to the landing face at y 2.7615), mass 347.3 at (2.52915, 0.08132); `extremeFibreM` 0.076; new title |
| rig `walker-carrier` | at (0, 0), maxM 4.62277 | at (−0.17966, 0.11629), on the tread line through world (0, 0); maxM 5.33670 |
| anchor `landing-roller` | x 4.62277 | x 5.08177; titles name the base plate (pin) and the head plate with slotted holes (roller) |
| pose `crowd-live-load` | wy −1401.9 N/m to 4.62277 | wy −3173.7 N/m to 5.08177 (16.13 kN, full width); description rewritten |
| pose `point-load-tread` | 400 N at dx 2.31139 | 800 N at dx 2.66835 |
| clip `moving-point-load` | dx 0.3 to 4.32277 | dx 0.3 to 5.0367 |
| thermal `stringer-steel` | areaM2 4.2, "Stringer and gusset steel" | 6.18 (hand calculation, factor 1.47 ≤ 1.5, C16), "Stringer, plates and brackets steel" |
| thermal `tread-top` | areaM2 4.75, L 0.33 | 5.95 (16 × 0.31 × 1.2, factor 1.25), L 0.31 |
| thermal assumption 1 | 12 boards | rewritten |
| structure assumptions | | rewritten: centre stringer, bearings, span 4.27 m, torsion note |
| `summary`, `provenance` | | rewritten; EN 10365 and EN 1991-1-1 citations corrected; `din-18065` added; notes give the geometry and masses |
| `representations` 50/100/200/500 | | all three views redrawn from the kit geometry (E1); every decision key rewritten in en and de |
| `symbols[0]` | | 16 treads; description and dimensional note updated |

Breakdown of the 6.18 m² hand calculation:

- HEA 0.906 × 5.046 m, minus the fin footprints: 4.545
- base plate: 0.075
- head plate: 0.069
- 16 carriers: 1.088
- 16 fins: 0.392
- anchors: 0.011

What the 2D drawings show now:

- **1:50 elevation:**
  - the HEA outline with steel hatch, cut horizontally on the base plate and plumb at the head plate
  - two base nuts with bolt ends, and two head nuts with bolt ends
  - 16 fins and 16 carriers
  - 16 treads with grain hatch and rounded nosing
- **1:50 front:**
  - treads behind the stringer band: in true projection the stringer is nearer than every tread at that height, so it now lies over the treads
  - carriers beside the stringer, base plate with nuts, and the head plate in front at the top
- **1:50 plan:** 16 treads with a nosing-edge line each and the walking line. The hidden stringer is drawn under the treads.
- **1:100:** stringer, plates, fins and plain treads. **1:200:** stringer bar, 30 mm plates, fins and treads. **1:500:** see below.

The pose `point-load-tread` was checked with `r3-2d --pose point-load-tread --views forces,moments` at 1:100. Weight is 3.41 kN, reactions are 2.16 and 2.05 kN, the residual is 0, and M_max is 2.69 kNm (hand: 0.85 + 1.82 = 2.67).

## 4. What is modelled

Expanded solid counts:

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 69 | 69 | 54 |
| 1:100 | 37 | 37 | 38 |
| 1:200 | 35 | 35 | 36 |
| 1:500 | 2 | 2 | 5 |

**1:50**
- **Body:**
  - HEA 160 as top flange, web and bottom flange. The web is 7.5 mm for the real 6 mm; the root radii are left out.
  - The stringer is cut horizontally on a 300 × 260 × 15 base plate and plumb against a 280 × 260 × 15 head plate.
  - 4 + 4 M12 hexagon nuts (AF 19) with bolt ends.
  - 16 carriers, 170 × 360 × 8.
  - 16 fins of 10 mm. Fin 1 runs down onto the base plate.
  - 16 treads with 8 mm rounded nosing.
- **Thermal:** every steel solid by reference. Carrier tops, the base-plate underside and the head-plate back are marked `sectionFaces` under C25 b. The tread reference has only the walking surface counted, as ruled in area-decisions.
- **Structure:**
  - HEA as the member.
  - Treads as members.
  - Carriers and fins as joints.
  - Base plate as the pin support and head plate as the roller support.
  - A 70 mm mass sphere whose top point is the true centre of mass. The centre of mass lies 81 mm above the axis, in the open zone over the flange, so a sphere centred on it would sit in air.

**1:100**
- **Body:** I-section with flanges and web at 15 mm, true 15 mm plates, 15 mm fins up to the tread (the carrier is merged into them), plain treads. No anchors and no nosing radius.

**1:200**
- **Body:** one 152 × 160 stringer bar, base and head plate at 30 mm, 30 mm fins, 40 mm treads.

**1:500**
- **Body:** a stringer bar on the floor up to the landing face, plus one folded 75 mm band carrying the 16 steps as a saw-tooth.
- **Why the risers are closed:** the open risers drawn between 75 mm bars leave 0.08 mm of white between the pens, below the 0.15 mm printable gap. The product marked the representation `unsuitable` (gap violations step-01/02 …). So the risers are closed in the 2D and in the 3D alike, and the decisions say so.
- **Structure supports:** the 75 mm end zones of the bar.

Final check line: `PASS (12.5 s)`.

- **Silhouette IoU** (elevation / front / plan): 1:50 1.000 / 1.000 / 1.000; 1:100 1.000 / 1.000 / 1.000; 1:200 1.000 / 0.974 / 0.999; 1:500 1.000 / 1.000 / 1.000.
- **Bounds:** deviation below 0.04 mm everywhere.
- **Area deviation:**
  - 1:50: stringer-steel +2.2 %, tread-top +1.5 %.
  - 1:100: +15.4 % and 0.0 %.
  - Total: 20.1 % at 1:200 and 28.6 % at 1:500.

## 5. What I saw in the final renders

- **`r3-sheet` 1:50 body, six panels:**
  - **Front (from the landing):** the stringer band rises behind the head plate, and the 16 treads cross it at their true heights.
  - **Right and left side:** a sloping I-beam whose top flange runs 50 mm under the tread backs. A triangular fin and a flat carrier sit under every tread. The base plate sits under the horizontal foot cut and the head plate stands plumb at the top with its nuts.
  - **Back:** the stringer with fins in line under the treads.
  - **Top:** 16 lapping treads.
  - **Three-quarter:** reads at once as a centre-stringer steel stair with timber treads.
- **1:50 from below (turn 215, tilt −25):**
  - The web and both flanges of the HEA are distinct.
  - The head plate shows its four bolts, and the base plate shows under the foot.
  - Carrier plates and fins appear at every tread. Nothing floats and there are no gaps.
- **1:50 structure:** the blue HEA and treads, orange brackets, green base and head plates, and the magenta mass just above the top flange at mid-flight.
- **1:50 thermal (`--cut`):** a red steel region and green tread tops. Nothing pokes out of the body.
- **1:100 and 1:500 `volumes-render` sheets:** every overlay is all green (IoU 1.000).
  - The 1:500 saw-tooth reads as a stair at a glance.
- **1:200 from below:** the stringer bar with plates and the fin rhythm.
- **`r3-2d` light-line and light-solid at 1:50:** clean outlines, hatches and the plan nosing lines.

## 6. Still imperfect, stated plainly

- **Web thickness:** the 1:50 web is 7.5 mm instead of 6 mm. `volumes-feature` asks for that minimum, and no 2D view can draw the web because it is hidden in every view. The HEA root radii (r 15) are not modelled.
- **Structure looks like the body:** the structure list is the body minus the anchors. For a steel stair every remaining part carries load (tread, bracket, stringer, bearing), so this is the load path, not a copy of cladding (D3).
- **Torsion not evaluated:** an HEA is torsionally soft for a centre stringer that gets one-sided loads. This is stated in the structure assumptions. A hollow section would be the better design, but HEA 160 is the declared section and I kept it.
- **Tread-top area convention:** `tread-top` counts the walking surfaces only, as previously ruled in area-decisions. Under C25 the free undersides would be exposed area, so this convention is for the area reviewers to revisit.
- **Plan convention:** the plan draws the hidden stringer under the treads to keep the region present in that view. It is fully covered, so it does not change what is seen.

## Round 5 handcraft

Agent C03, 2026-11-21. Opened: `r3-sheet` body, structure and cut thermal at 1:50, 1:100, 1:200 and 1:500, the poses `self-weight`, `crowd-live-load`, `point-load-tread` and clip `moving-point-load` t 2 at 1:50, `r3-2d` at every scale (`🗑️generated/r5-stair-flight/before`), and a 1:50 view from below (turn 215, tilt -20).

### Reference and model (1:50)

| quantity | model | reference | deviation |
|---|---|---|---|
| rise / going / lap | 181.25 / 280 / 30 mm | DIN 18065: r ≤ 190, g ≥ 260, lap ≥ 30 mm | within |
| 2 r + g | 0.6425 m | 0.59 to 0.65 m (DIN 18065) | within |
| pitch | 32.92° | ≤ 45° for building stairs | within |
| HEA 160 h / b / t_f | 152 / 160 / 9 mm | EN 10365 | 0 % |
| HEA 160 web | 7.5 mm | 6 mm | +25 %: 1:50 feature floor (hidden in every 2D view) |
| tread | 310 x 1200 x 40 mm C24 | same | 0 % |
| base / head plate | 300 x 260 x 15, 280 x 260 x 15 mm | same | 0 % |
| flight mass | 347.3 kg | hand calculation 347.3 kg | 0 % |

### Grades (body / thermal / structure)

| scale | before = after |
|---|---|
| 1:50 | A- / A- / A- |
| 1:100 | A- / A- / A- |
| 1:200 | B+ / B+ / B+ |
| 1:500 | B+ / B+ / B+ (folded 75 mm band with closed risers: the open risers cannot print at 1:500) |

No remodelling: every quantity is within 3 % of DIN 18065 and EN 10365 except the web at the feature floor, the sections and bearings are the real construction, poses and clip frames stay closed. `r3-validate.ts components/stair-flight` VALID; `volumes-check.ts --product` PASS (2026-11-21).

Still imperfect: no guard or handrail. MBO § 38 and DIN 18065 require a handrail and a guard on free sides; this component models the flight as the structural element, and the guard belongs to the separate balustrade component of the catalog (`components/glass-balustrade`) or to the stairwell wall. A guard modelled into this asset would change dimensions, thermal regions and the 2D of every scale; it is listed as an open point, not done in this round. The HEA root radii are not modelled; the 1:500 band closes the open risers.
