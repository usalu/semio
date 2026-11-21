# Round 3 — machinery/sorption-wheel

State 2026-11-20: finished, spliced, catalog valid. `volumes-check.ts machinery/sorption-wheel --product`: **PASS**. `r3-validate.ts machinery/sorption-wheel --volumes`: **VALID** (schema 0, validateAsset 0, checkVolumes 0); `--catalog-only`: VALID.

Scripts (all in the ticket folder): `r3-sorption-wheel-geo.py` (measures and outlines), `r3-sorption-wheel-texts.py` (texts, masses, areas), `r3-sorption-wheel-volumes.py` (solids), `r3-sorption-wheel-roots.py` (roots and 2D, writes the candidate), `r3-sorption-wheel-apply.py` (writes the roots into the catalog file), `r3-sorption-wheel-ship.sh` (build, validate the candidate, apply, splice, validate), `r3-sorption-wheel-crop.sh` (enlarges a region of a PNG for looking). No earlier round-3 work existed for this asset; I started from the catalog state.

## 1. Reference

| item | real value | source |
|---|---|---|
| Rotor depth | standard 50, 100, 200, 400 mm | DRI components brochure (drirotors.com); Kmaire cassette data (100, 200, 400) |
| Rotor diameter | 550 mm is a catalogue size, with 100, 200 and 400 mm depth | Rotor Source brochure (table row 550) |
| Flute geometry | wave height 1.3 to 4.0 mm, wavelength 2.5 to 6.5 mm (a 270 mm SEMCO medium: 1.5 x 4.2 mm) | US 5,683,532; US patent text on the SEMCO wheel |
| Speed | 8 to 12 rev/h typical, up to 30 in studies | web search results (manufacturer text, wheel-speed study) |
| Sectors | process 60 to 75 %, regeneration 25 to 40 % of the face | web search result; the asset's 270 / 90 degrees is in that range |
| Rotor construction | corrugated glass-fibre paper with silica gel wound on a hub, sheet rim shell, spokes on larger rotors, rotor on a fixed shaft with two bearings in the hub | my knowledge of Munters, DRI, Seibu Giken cassettes |
| Cassette | sheet-steel casing, shaft carried by a centre beam per face, sector divider bars with radial seals, perimeter seal, geared motor with belt (or chain) round the rotor rim | DRI brochure; Seibu Giken RUF cassette (geared motor, chain); my knowledge |
| Drive motor | small AC gearmotor, 70 mm frame, 6 to 25 W shaft power, about 30 to 40 W input, 1.5 to 2 kg | my knowledge of catalogue gearmotors |
| Asset's own data | casing 0.62 m, 0.22 m deep; matrix 0.57 m x 0.2 m; 12 rev/h; 90 degree sector; belt motor | `provenance.notes` |

Not found: a published cassette envelope for a 550 x 200 rotor (real square cassettes are about 0.3 m deep). The asset's round 0.62 x 0.22 m casing is kept as the declared subject, with no cited reason to change `dimensions`.

## 2. Audit against the reference (state before)

- **Rig**: joints right (rotor and sector at the wheel centre 0 / 0.31, revolute rotor about the lateral axis). The casing mass had no drive in it.
- **Dimensions**: 0.62 x 0.62 x 0.22 m kept. Mass 22 kg had no motor.
- **2D**: casing a full disc, matrix 0.57 m in front of the flange, sector 84 degrees instead of the 90 of the notes and drawn from the centre through the hub, no shaft beam, no divider bars, no drive, the stand a trapezoid block under a ring that itself touches the ground. Front and plan showed the matrix and the sector through the closed band. 1:100 had four spokes, 1:200 merged the hub into the casing circle.
- **Solids**: casing a C drum with 20 mm walls and 20 mm flange rings; matrix 0.18 m deep (notes: 0.2 m); hub a solid cylinder; spokes as 10 mm rods lying on the faces; the hot sector a closed 10 mm plate on both faces, which would block the regeneration air; nothing holds the rotor (the old text says the flanges do); no drive. Structure was the body by reference plus a cross behind the wheel. The emitters were crescents hovering at the faces.
- **Check before**: `volumes-thermal-coverage` failed at all three scales (casing 1.05 against 0.72 m2, regen 0.23 against 0.138 m2, total at 1:200 +57 %).
- **Texts**: the notes say belt motor, the thermal model put the 30 W electrical input on the hub.

## 3. Root fixes (catalog file, outside `volumes`)

| member | old | new |
|---|---|---|
| `summary` | wheel on spokes in a casing ring | adds the two feet and the belt gearmotor |
| `provenance.references` | 6 | + `dri-rotors-components`, `us-5683532-1997` |
| `provenance.notes` | stand, belt motor, 22 kg (6 / 14 / 2) | two feet, shaft between two beams, flat belt, 30 W gearmotor under a guard, divider bars, 24 kg (6.4 / 1.6 / 14 / 2), motor heat 28 W, standard depths, flute size, the 0.02 m honeycomb is a symbol |
| `dimensions.massKg` | 22 | 24 (gearmotor 1.6 kg and guard added) |
| `materials` | 2 | + `painted-steel` (motor housing), `epdm` (seals, belt) |
| `rig.bones.casing.mass` | 6 kg at 0 / 0.31 | 8 kg at 0.05 / 0.25 (casing 6.4 kg at the ring centre, motor 1.6 kg at 0.255 / 0.043) |
| bone titles | casing with stand; sector with seals | casing with feet and drive; sector frame with seals |
| `anchors.base-contact` | footprint 0.24 x 0.2 m | 0.34 x 0.18 m (the two feet over all) |
| `thermal.regions.casing` | 0.72 m2 | **0.93 m2** (factor 1.29, C16; measured 0.927 at 1:50; the casing gained beams, sector frames, motor plate and guard), title updated |
| `thermal.regions.hub` | "Drive hub", ratedPowerW 30 | "Hub with bearings", rated power moved to the motor; 1.5 W, area 0.064 kept (measured 0.069) |
| `thermal.regions.motor` | none | new: engine, operating on `rotation`, 28 W, rated 30 W electrical, 0.038 m2 (measured 0.0382), 3 mm housing layer |
| `thermal.assumptions` | 2 | + the motor heat sentence |
| `poses.*.thermal.regions.motor` | none | exposed 0.75 (measured 0.75: stands on the plate, gearhead against the guard) |
| `structure.assumptions` | 22 kg on a 0.24 x 0.2 m stand | 24 kg on two feet, 0.34 m over all, 0.18 m wide |
| `representations.50/100/200` | see audit | redrawn, section 4; all decision texts rewritten (en, de) |
| `representations.500` | "50 mm casing band" | "35 mm face flange", "honeycomb symbol" |

`regen` 0.138 m2 and `disc` 0.19 m2 (exchange) are unchanged; the regen model now measures 0.142 m2.

The new motor region is physical data I added, not a fit: the notes already named a belt motor, and the 30 W were on the hub. The 28 W heat and 1.6 kg are my estimates for a 70 mm gearmotor; they are for the thermal maintainer to confirm.

## 4. 2D (ruling E1)

- **Elevation**, the true face view of each scale's body: rotor (disc 0.57 m, honeycomb symbol, eight spokes, hot sector of 90 degrees from the hub to the rim, hub) drawn behind the casing, which is now an annulus with its 35 mm flange (path with a hole); the sector frame (V of the two divider bars) and the shaft beam over it; two feet; belt guard; gearmotor with end cap and terminal box. Edge strokes mark the ring over guard and feet, because the line view merges shapes of one region.
- **Front**: the band edge-on, the motor-side foot (plate and gusset edges at 1:50), guard edge, motor with end cap, gearhead joint and terminal box. The matrix and the sector are no longer drawn through the band.
- **Plan**: the band alone; everything else lies under it.
- 1:100: eight spokes (was four), block feet with the plinth, one motor block. 1:200: hub merged into the beam shape (`regions` casing and hub), no spokes, motor and guard on the ground.
- Looked at: all three scales in line and solid tone, pose `rated` and pose `off` at 1:50 (rotor turns, frame, beam and sector stay).

## 5. Model per scale (ids / expanded solids: body, thermal, structure)

### 1:50 (28 / 37, 28 / 28, 22 / 24)

- **Body**: casing ring of 1 mm sheet with 35 mm face flanges in three parts (16 mm belt window); EPDM rim seals; fixed 25 mm shaft; one 40 x 8 mm channel beam per face lapped behind the flange; two feet (8 mm plate, two gussets each), the motor side lengthened into the motor plate; rotor: hub tube 90 mm with 4 mm wall and bearing end walls, matrix ring 0.57 x 0.2 m, eight 6 mm spoke plates, 2 mm rim shell; sector frame per face (welded V of two 30 x 8 mm bars, cut along the beam flank); gearmotor (gearhead, motor, end cap, terminal box, shaft, flanged pulley); flat belt 10 x 5 mm round the rim shell and the pulley (slender); belt guard of two plates and an edge strip.
- **Thermal**: stationary zones on the seal bone (process zone 272 degrees, sector core, two 20 mm hot face skins with layer `regen-skin`), casing as four quarter shells, two flange rings and references, hub as its closed cylinder, motor by reference, two outlet patches on the matrix faces (process air near face, regeneration air far face).
- **Structure**: band, beams, shaft, feet, hub, spokes, rim shell and sector frames by reference (they are the members); two bearing joints in the hub end walls, the sector root joints, one support solid of two pads, three mass balls. No matrix, seals, motor or guard.

### 1:100 (12 / 19, 19 / 19, 16 / 19)

One C drum, 15 mm bar beams and V frames flush with the faces (matrix 0.19 m deep), matrix ring with merged shell, closed hub, eight spokes, block feet, one motor block, solid guard slab. Structure adds its own rim shell, bearings and root joints.

### 1:200 (10 / 10, 19 / 19, 14 / 18)

Solid casing ring from the flange edge to the band, one rotor disc, 30 mm beams and V frames, block feet, motor and guard blocks on the ground. Structure is hub, four 30 mm spoke plates and a rim ring, not the disc; joints are below the feature and left out.

### 1:500

Unavailable, as the representation.

### Final check lines

- 1:50: silhouette IoU 0.998 / 1.000 / 1.000 (elevation / front / plan), p95 1.9 / 0 / 0 mm; areas casing +0.3 %, regen +2.8 %, hub +7.3 %, motor +0.6 %.
- 1:100: IoU 0.998 / 0.999 / 1.000; areas casing −6.4 %, regen −3.9 %, hub +2.9 %, motor −6.3 %.
- 1:200: IoU 0.998 / 1.000 / 1.000; area total +3.5 %.
- Bounds, feature, monotonic, cross-scale, structure coverage, bones, slender (belt 365), copied: ok at every scale.

## 6. What I saw in the final renders

- **1:50 body, six-view sheet and three-quarters from both sides** (`r3-sheet`): front (+x) the band with the belt strip, the guard edge and the motor low on the near side; right side (face) the flange ring, the beam, the V of the divider bars pointing at the hub, eight spokes at the rated angle, feet, guard and motor with its round end cap and terminal box; back the plain band with the far foot; left side the far face with the guard plate; top the band. Three-quarter views read as a small desiccant cassette with its drive. The matrix face is plain: the flutes are too fine to model.
- **1:50 thermal** (sheet, cut sheet, `volumes-render`): green process zone, purple hot sector on both faces, yellow hub cylinder inside, red casing, cyan motor. In the cut view the coincident inner faces of sector core, skins and outlet patches flicker; that is the uncapped cut, not a gap.
- **1:50 structure**: ring, beams, spokes, rim shell, hub, V frames in blue with the belt window visible in the band, orange root joint at the centre, green pads under the feet, three magenta balls; motor and guard only as ghost.
- **1:100 and 1:200 body and structure sheets**: same machine at a glance. The first 1:200 draft had the divider bars floating off the beam; they are now one V welded to the beam flank at every scale.
- **Clip `spin` at 100 s and pose `off`**: the rotor turns, everything else stays.
- **Overlays** (`volumes-render` 1:50): all green.

## 7. Still imperfect

- The casing is only 10 mm deeper than the rotor per face, so the shaft beams are 8 mm channels where a real cassette has 20 to 40 mm sections and is about 0.3 m deep. I had no cited envelope to change `depthM`.
- The radial seal lips on the divider bars (2 mm) and all bolts are below the feature and absent. The shaft ends are hidden behind the beam webs; there is no visible fixing.
- The guard plates are 7.5 mm (the feature) instead of 1 mm sheet.
- The thermal band ignores the 16 mm belt window, and its inner faces are marked by quarter (stand-in for the missing C31 flood, `tool-requests.md`).
- The bearing joints sit inside the hub end walls and are hidden by the hub in the structure view from most directions.
- Emitter anchors and directions are unchanged: the 2D plumes still leave in the paper plane, while the real air flows along the wheel axis. That is a limit of the face-on elevation.
- Rotor mass 14 kg and the silica-gel bulk density of 700 kg/m3 do not agree (0.049 m3 would be 34 kg); I left both.
- `r3-sheet.ts` shades the far caps of concave extrudes unevenly (logged in `tool-requests.md`).

## Round 5 handcraft

No geometry change. Every scale and list rendered and opened (body 1:50 to 1:500, structure at every scale, thermal cut at every scale, poses `off`, `slow`, `rated` and clip `spin` t 100 at 1:50, 2D 1:50 to 1:200). Reference as in section 1 (rotor 550 x 200 mm, casing 0.62 x 0.22 m, 12 rev/h, 90 degree sector); measured 1:50 extents agree to 0 % (bounds check 0 mm at every scale).

| scale | before = after (body / thermal / structure) |
|---|---|
| 1:50 | A- / A- / A- |
| 1:100 | A- / A- / A- |
| 1:200 | A- / A- / A- |
| 1:500 (model only) | B+ / B+ / A- (the 30 mm beams and V frame read right; the casing ring is faceted by the coarse tessellation, and 1:500 is close to 1:200 in content) |

`volumes-check.ts machinery/sorption-wheel --product`: PASS.

Remaining: casing only 10 mm deeper than the rotor per face (no cited cassette envelope to change `depthM`); seal lips and bolts below the feature.

The paired foot outlines now use a 0.20 mm pen in the 1:100 print representation. The drawing passed five checks from 1:55 to 1:150.
