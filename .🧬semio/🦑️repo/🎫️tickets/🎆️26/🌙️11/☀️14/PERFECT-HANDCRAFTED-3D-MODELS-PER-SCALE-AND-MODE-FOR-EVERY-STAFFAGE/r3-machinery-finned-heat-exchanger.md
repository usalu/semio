# Round 3: machinery/finned-heat-exchanger

State: finished and spliced (2026-11-20). An earlier author had left two scripts (`-geo.py`, `-volumes.py`) for a serpentine coil that kept the old proportions; nothing of it had reached the catalog. I kept its frame helpers, replaced the design by a real two-row header coil and rebuilt the asset from the roots. Everything below describes the catalog file as it is now.

## Final state

- `volumes-check.ts machinery/finned-heat-exchanger --product`: **PASS** (36.2 s) at 1:50, 1:100, 1:200 (1:500 unavailable, as its representation).
- `r3-validate.ts machinery/finned-heat-exchanger --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- Print suitability (office laser profile): suitable at all three scales.
- Silhouette IoU 2D against 3D: 1:50 1.000 / 0.999 / 1.000 (elevation / front / plan), 1:100 1.000 / 0.998 / 0.989, 1:200 1.000 / 1.000 / 1.000.

## Scripts (ticket folder)

| Script | What it holds |
|---|---|
| `r3-finned-heat-exchanger-geo.py` | Measures of the coil, bone frames, hairpin path, header cap profile |
| `r3-finned-heat-exchanger-volumes.py` | Every solid of every scale and list, stated by hand |
| `r3-finned-heat-exchanger-roots.py` | Dimensions, material, rig, anchor, thermal regions, pose texts, notes, references, 2D of 1:50, 1:100, 1:200 |
| `r3-finned-heat-exchanger-build.py` | Writes the candidate `🗑️generated/r3-finned-heat-exchanger/🔣️.json` and the source `volumes/machinery/finned-heat-exchanger.json` |
| `r3-finned-heat-exchanger-apply.py` | Writes changed root members into the catalog file and height/width into the manifest row |
| `r3-finned-heat-exchanger-suit.ts` | Print-suitability probe |
| `r3-finned-heat-exchanger-temps.ts` | Solved region temperatures per pose |
| `r3-finned-heat-exchanger-ship.sh` | Build, validate, suitability, apply, splice, check in one run (resume with this) |

## 1. Reference

The subject is the heating or cooling coil of a small air handler or heat pump: a plate-fin and tube coil with headers.

| Quantity | Real value / practice | Source | Model |
|---|---|---|---|
| Tube | 5/8 in = 15.88 mm copper, wall about 0.4 mm | EN 12735-2; product knowledge | 15.88 mm, modelled full (bore closed by bends and headers) |
| Tube pattern | Staggered, 40 mm tube pitch, 34.64 mm row pitch (equilateral, "P40"); each row has the same number of tubes, rows offset by half a pitch | Common coil geometry for 5/8 in tube (manufacturers' catalogues, from memory, not a cited document) | 2 rows of 12, rows at x = -17.32 / +17.32 mm, tubes at 30 + 40k and 50 + 40k mm |
| Fins | Aluminium plate fins 0.1 to 0.25 mm, pitch 2 to 4 mm; fin depth = rows x row pitch, fin height = tubes x tube pitch | VDI 3803-1 (asset reference); EN 1216 terms | 0.2 mm at 3 mm, 134 fins, 69.3 mm deep, 480 mm high, finned length 400 mm |
| Circuiting | Water coils: supply and return header on one end, return bends (hairpins) on the other; supply on the air-leaving row (counterflow); supply stub low, return stub high, vent on top, drain at the bottom | Product knowledge | 12 full circuits, 12 hairpins of 20 mm bend radius (= half the 40 mm centre distance of the diagonal neighbours), two vertical headers |
| Headers and stubs | Copper pipe 22 to 35 mm for this size; threaded connections R 3/4 to R 1 1/4 | EN 1057; EN 10226-1 (R 1: 33.25 mm) | 28 mm headers with domed caps, 28 mm stubs 64 mm long ending in a 34 mm threaded sleeve with open bore, 10 mm vent and drain plugs |
| Casing | Galvanised sheet 1.25 to 2 mm: two tube sheets with flanges turned away from the pack, top and bottom plates as channels; casing deeper than the fins | EN 10346; product knowledge | 1.5 mm; tube sheets 100 x 520 mm with 25 mm flanges; channels 100 x 20 mm, 400 mm long |
| Envelope | Follows from the above | - | 0.52 m high, 0.10 m deep (air direction), 0.535 m long (hairpins -0.239 to stub ends +0.296) |
| Mass (dry) | Fins 134 x 0.0284 m2 x 0.2 mm x 2700 = 2.06 kg; tube 11 m x 0.173 kg/m = 1.9 kg; headers and stubs 1.1 m x 0.755 kg/m = 0.8 kg; casing 0.403 m2 x 1.5 mm x 7800 = 3.1 kg | Hand calculation | fins 2.1, tubes 2.7, frame 3.1, total 7.9 kg |

## 2. Audit of the state before round 3

- **Rig**: all three bones had `angleDeg` 0, so each ran 0.48 to 0.52 m horizontally ahead out of a coil that is 0.1 m deep and 0.52 m tall (visible in the first sheet: the bone lines left the model).
- **Construction**: the "headers" were two flat copper boxes of 150 x 20 x 500 mm lying on top of and under the fin pack, each with a 12 mm stub pointing ahead. No coil is built like that: nothing connected the 23 tubes to them, there were no return bends, no tube sheets and no casing; the coil stood on a copper box.
- **Tube pattern**: 12 + 11 tubes, rows 45 mm apart in a 100 mm fin (not a real fin pattern); notes said 9.5 mm tubes, the drawing had 18 mm, the 1:100 drawing 22 mm.
- **2D**: the front view drew 11 tube runs across the fin face "as if the first fin were lifted"; the fin hatch ran horizontally although fins stand vertical; the plan was one rounded rectangle; end plates were bound to the fin region.
- **Solids**: body 9 solids (block, tube rods, two boxes); qa-visual-2 grade B / B+ / B-; structure was the body (D3 defect, model-findings line 82).
- **Thermal**: `headers.areaM2` 0.36 described the two fake boxes; no casing region; with `heatW` 24 W on 0.67 m2 the solver put the tubes at 25 degC next to fins at 46 degC while the pose text said 52 degC.
- **Mass**: 5 kg, below the dry mass of the parts.

## 3. Root fixes (catalog file outside `volumes`)

| Member | Old | New |
|---|---|---|
| `dimensions` | 0.52 / 0.205 / 0.5 m, 5 kg | 0.52 / 0.1 / 0.535 m, 7.9 kg; manifest row `widthM` 0.205 -> 0.1 |
| `materials` | aluminium-fin, copper | + `galvanised-steel` (same values as the sibling plate exchanger) |
| `rig.frame` | (0, 0), 0 deg, 0.52, 1.3 kg com (0, 0.26) | (0, 0), **90 deg**, 0.52, 3.1 kg com (0.26, 0) along the bone |
| `rig.fins` | (0, 0.02), 0.48, 2.6 kg | (0.02, 0) in the turned frame = same world point, 0.48, 2.1 kg com (0.24, 0) |
| `rig.tubes` | (0, 0), 0.48, 1.1 kg | (0.01, 0) = lowest tube at 0.03 m, 0.46 to the highest tube, 2.7 kg com (0.23, 0) |
| Bone titles | Headers and end plates / fin pack / tube rows | Say what the bone is and where it runs |
| `anchors.base-contact` | footprint 0.15 x 0.4, "Base" | 0.1 x 0.45, "Tube-sheet feet and channel legs" |
| `thermal.regions.tubes` | 0.67 m2, L 0.018, 24 W | **0.56 m2**, L 0.016, **168 W** |
| `thermal.regions.headers` | 0.36 m2, L 0.15, 2 W | **0.1 m2**, L 0.028, **17 W** |
| `thermal.regions.fins` | L 0.1 | L 0.07 (fin depth); area 2 m2 (exchange) and 340 W unchanged |
| `thermal.regions.casing` | - | new passive region, galvanised steel, 0.54 m2, one 1.5 mm layer |
| `poses[].thermal.regions` | fins, tubes, headers | + casing in all three poses |
| `poses.hot/cold.description` | tubes 52, fins 45 degC; fins 5 degC | the solved values: hot 50 / 44 / 42 degC (tubes / fins / headers), cold 0 / 4 / 6 degC |
| `structure.assumptions` | stands on its bottom header | stands on tube-sheet feet and channel legs; fins hang on tubes, tubes span between tube sheets |
| `summary`, `provenance.notes` | headers above and below | the real coil, its circuiting, casing, masses, heat values |
| `provenance.references` | 4 | + EN 12735-2, EN 1057, EN 10346, EN 10226-1, EN 1216 |
| `representations.500` reason | 18 mm tubes | 0.2 mm fins, 1.5 mm sheet, 16 mm tubes |

Areas and heat are also listed in `area-review.md` (2026-11-20 entry). Two points there need the area reviewer: the header area changed by a factor 3.6 (beyond the 1.5 of C16; the old value measured boxes that no longer exist), and I raised `heatW` of tubes and headers to keep the flux densities first authored (300 and 167 W/m2), because otherwise the tubes are colder than the fins they heat.

### 2D at 1:50, 1:100, 1:200 (ruling E1)

All three views redrawn by hand from the same measures as the 3D.

- **Elevation** is now declared as section A-A (plane z = 0): one fin seen flat with the 24 cut tubes in the staggered pattern, the two channels cut at their real sheet thickness, the left tube sheet beyond. The headers stand hidden directly behind that tube sheet; because every region must be bound in the elevation, the tube-sheet shape lists `casing` and `headers` (said in `decisions.overlays`).
- **Front** (air-leaving face): fin face with a vertical hatch (fin edges; was horizontal), frame of flanges and channel legs, the twelve hairpins as one scalloped strip on the right (their 4 mm gaps do not print), the two headers, drain plug and both stubs with sleeves on the left.
- **Plan**: top channel, tube sheets as thin C outlines, hairpin U, tube ends, header circles, vent plug, stubs.
- 1:100 drops plugs and sleeves and merges the two tube ends; 1:200 is coil block, two bars, two end plates, header block, stubs.
- All eight decision texts per scale rewritten (en, de).

## 4. Model per scale (solids in the list / after repeats)

### 1:50 — body 16 / 315, thermal 14, structure 20 / 93
- **Body**: bottom and top channel (U extrudes, 1.5 mm); two tube sheets (C extrudes, web plus two 25 mm flanges); fin pack core 2 mm inside each air face with 2 x 134 real fin edges of 0.2 mm as texture; 2 x 12 tubes from the hairpin root into the header axis; 12 lofted hairpins (10 sections, 20 mm radius, tilted 30 degrees as the diagonal neighbours demand); two revolved headers with elliptical caps; two revolved stubs with sleeve and open bore; vent and drain plug.
- **Thermal**: fins = pack core; tubes = both rows and hairpins; headers = headers, stubs, plugs; casing = channels and tube sheets. One layer per region, so no shells; no emitters.
- **Structure**: two tube-sheet posts (web with flange roots as lips) and the two channels as frame; 24 tubes as members; five representative fin plates hanging on them; collar joints at every tube pass through both sheets (4 x 12), brazed joints at the header entries (2 x 12), lap joints at the four channel ends; support = the feet of the two tube sheets; masses of fin pack (centre), tube circuit (shifted 67 mm towards the headers) and the two casing halves.

### 1:100 — body 12 / 45, thermal 12, structure 14 / 17
Plain pack block, tubes at real size (15.88 > 15 mm), hairpins with 5 sections, flat-ended headers, plain stubs, no plugs; casing profiles keep 1.5 mm. Structure: posts with 15 mm lips, channels, tubes, three fin plates, one joint band per tube sheet, feet, four masses.

### 1:200 — body 8, thermal 9, structure 8 / 12
Coil block, two 30 mm bars, left end plate 30 mm, right end plate 39 mm (takes in the hairpins), header block 70 x 30 mm, two 30 mm stubs. Thermal: tube bank as one slab inside the block. Structure: bars and end plates, four 30 mm tube bars (six tubes each), one fin plate, pads, one mass.

Measured outer areas at 1:50: tubes 0.566 (0.56), headers 0.100 (0.1), casing 0.537 (0.54).

## 5. What I saw in the final renders

- **1:50 body, six-view sheet and a close-up from the header side**: reads at once as a duct coil. Front and back: fin face with fine vertical fin lines inside a sheet-metal frame. Right side: twelve hairpins, each tilted between the two rows. Left side: two headers side by side, offset by half a pitch, tube ends entering them, vent on top of the rear one, drain under the front one, stubs with sleeve and visible bore. Top: channel between the tube sheets, hairpin U, header circles, stubs. Rig: three vertical bones inside the coil.
- **1:50 thermal (cut)**: fin core, tubes running through it into the header, casing frame; nothing pokes out.
- **1:50 and 1:100 structure**: frame, tube grid, fin plates, orange collars at both sheets, masses in a row along the tube axis; reads as a load path, not as the body.
- **1:100 body**: same coil, smooth fin face, plain headers and stubs. **1:200**: framed block with header block and stubs; recognisable.
- **1:200 structure** was first one slab (a block); replaced by four tube bars and looked at again.
- **2D** at all three scales and the cold pose opened: sections, hatch and stubs are where the 3D has them. At 1:100 the 24 tube dots in A-A are dense in the line view (0.16 mm circles); I thinned their pen to 0.1 mm, they are legible but tight.
- Poses and clips do not move bones; the cold pose and a `reverse` frame at 30 s were rendered and are identical in form.

## 6. Still imperfect

- **No tube holes and no flange mounting holes** in the tube sheets: a flat 1.5 mm plate with holes fails `volumes-feature`, so each sheet is one C extrude and the tubes pass through it. Request filed in `author-requests.md`.
- **Structure tube sheets are own solids, not references** to the body sheets: the product's body mask does not rasterise the 1.5 mm flange on edge and then reports the same solid 24 mm "outside the body". Filed in `tool-requests.md`; the posts carry 7.5 / 15 mm lips instead of the 25 mm flanges.
- **Fin plates in the structure list are 7.5 / 15 mm thick** (feature rule), standing for 0.2 mm fins.
- **Fins are a core block with edge texture**, not 134 plates with collars and louvres; tubes behind the fin face are therefore not visible through the gaps.
- **Headers in section A-A** are bound through the tube sheet shape, not drawn, because they are hidden.
- **Fin exchange area** stays at the committed 2 m2 although the real fin surface is about 7.6 m2 (C19 forbids changing it from a model).
- The P40 pattern is cited from memory of manufacturers' practice, not from a document in `provenance.references`.

## Round 5 handcraft (2026-11-21)

Script: `r5-finned-heat-exchanger-build.py` (runs the round 3 build, then the 1:200 band); apply and splice with `r3-finned-heat-exchanger-apply.py` and `volumes-insert.py`.

### Reference vs model (1:50)

| quantity | reference | source | model | deviation |
|---|---|---|---|---|
| tube | 15.88 mm (5/8 in) copper | EN 12735-2 | 15.88 mm | 0 |
| tube / row pitch | 40 / 34.64 mm staggered (P40) | coil manufacturers' practice (K) | 40 / 34.64 mm | 0 |
| fin pitch / thickness | 2-4 mm / 0.1-0.25 mm | VDI 3803-1, EN 1216 | 3 mm / 0.2 mm | in band |
| fin pack | 400 x 480 x 69.3 mm (12 tubes x 40, 2 rows x 34.64) | derived | 400 x 480 x 69.3 mm | 0 |
| headers, stubs | 28 mm, R 1 thread 33.25 mm | EN 1057, EN 10226-1 | 28 mm, 34 mm sleeve | +2.3 % sleeve |
| casing sheet | 1.25-2 mm galvanised | EN 10346 | 1.5 mm | in band |
| dry mass | 7.9 kg by hand calculation | round 3 | 7.9 kg | 0 |

### Grades before -> after (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- |
| 1:100 | A- / A- / A- | A- / A- / A- |
| 1:200 | B+ / B+ / B+ | B+ / B+ / B+ |
| poses | hot (default), cold, idle and `reverse` t = 30 s: no bone moves, identical form | unchanged |

Opened: r3-sheet body, structure and thermal --cut at 1:50, body at 1:200. Rendered by `r5-m03-render.sh` but not opened this round: the 1:100 sheets, the 1:200 thermal and structure sheets, the cold and idle poses and the `reverse` t = 30 s frame (no bone moves, so their form equals the default pose).

### Changes

- `representations.200.validRange.maxDenominator` 350 -> 220: the 70 x 30 mm header block measured 0.150 mm at 1:350, 0.180 at 1:260 and 0.188 at 1:240 (visible 45 mm beside the end plate), 0.2 mm or more at 1:220.

Check: `r3-validate.ts machinery/finned-heat-exchanger --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0).

### Remaining

- Round 3 open points stay: no tube holes in the tube sheets, fin pack a core with edge texture, structure fin plates at the feature floor.
